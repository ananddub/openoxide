/*
** 2026-08-31
**
** The author disclaims copyright to this source code.  In place of
** a legal notice, here is a blessing:
**
**    May you do good and not evil.
**    May you find forgiveness for yourself and forgive others.
**    May you share freely, never taking more than you give.
**
******************************************************************************
**
** SQLite Realtime / Subscription C Extension.
**
** Implements a low-overhead, transaction-aware Change Data Capture (CDC) engine
** with Linux eventfd integration for non-blocking epoll reactors.
**
** DESIGN NOTES:
**
**   - Per-connection context is stored in a linked list keyed by db pointer.
**     No global singleton; multiple connections are fully supported.
**
**   - The update_hook appends change events to a connection-local "pending"
**     list. No locks are taken in the update_hook path because SQLite
**     guarantees that a single connection is only used from one thread at
**     a time (serialized mode aside, but hooks execute on the calling thread).
**
**   - The commit_hook atomically moves pending events into a "committed"
**     queue under a mutex and signals eventfd only when the queue
**     transitions from empty to non-empty (edge-triggered wakeup).
**
**   - The rollback_hook discards the entire pending list to prevent
**     false events from rolled-back transactions.
**
**   - The drain function atomically swaps out the committed list and
**     processes events outside the lock. The eventfd is read (drained)
**     outside the lock to minimize lock hold time.
**
**   - Autocommit failure handling: update_hook fires during autocommit
**     statements, but if the statement fails, neither commit_hook nor
**     rollback_hook fires. To handle this, we track whether we are inside
**     an explicit transaction. For autocommit, pending events are published
**     directly from commit_hook (which fires for autocommit too). If an
**     autocommit statement errors, SQLite does not call commit_hook, and
**     pending events are cleared at the start of the next hook invocation.
*/

#if !defined(SQLITE_CORE) || defined(SQLITE_ENABLE_REALTIME)

#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <pthread.h>
#include <errno.h>
#include <stdatomic.h>

#ifdef __linux__
#include <sys/eventfd.h>
#else
#include <fcntl.h>
#endif

#ifndef SQLITE_CORE
  #include "sqlite3ext.h"
  SQLITE_EXTENSION_INIT1
#else
  #include "sqlite3.h"
#endif

#include "realtime.h"

/* =========================================================================
** Data Structures
** ========================================================================= */

/*
** Individual change event node.
*/
typedef struct RealtimeEvent RealtimeEvent;
struct RealtimeEvent {
  int op;                 /* SQLITE_INSERT, SQLITE_UPDATE, SQLITE_DELETE */
  char *zDb;              /* Database name (e.g. "main") */
  char *zTable;           /* Table name */
  sqlite3_int64 iRowid;   /* Row ID */
  RealtimeEvent *pNext;   /* Linked list pointer */
};

/*
** Connection context for Realtime tracking.
*/
typedef struct RealtimeContext RealtimeContext;
struct RealtimeContext {
  sqlite3 *db;

  /* Event notification descriptor */
  int eventFd;

  /* Pending list: populated during transaction by update_hook.
  ** Only accessed from the thread that calls into SQLite on this connection,
  ** so no lock is needed for pending list operations. */
  RealtimeEvent *pPendingHead;
  RealtimeEvent *pPendingTail;
  int nPending;

  /* Committed list: populated on commit_hook, drained by reactor/reader.
  ** Protected by commitLock because the writer thread (commit_hook) and the
  ** reactor thread (drain) access this concurrently. */
  pthread_mutex_t commitLock;
  RealtimeEvent *pCommittedHead;
  RealtimeEvent *pCommittedTail;
  int nCommitted;

  /* Linked list of all contexts for multi-connection support */
  RealtimeContext *pNext;
};

/* =========================================================================
** Per-Connection Context Registry
** ========================================================================= */

static pthread_mutex_t g_registryLock = PTHREAD_MUTEX_INITIALIZER;
static RealtimeContext *g_pContextList = NULL;

/*
** Find the RealtimeContext for a given db connection.
** Caller must hold g_registryLock or guarantee single-threaded access.
*/
static RealtimeContext *realtime_find_ctx(sqlite3 *db){
  RealtimeContext *p;
  for( p = g_pContextList; p; p = p->pNext ){
    if( p->db == db ) return p;
  }
  return NULL;
}

/*
** Find context with lock. For use from public API functions.
*/
static RealtimeContext *realtime_find_ctx_safe(sqlite3 *db){
  RealtimeContext *p;
  pthread_mutex_lock(&g_registryLock);
  p = realtime_find_ctx(db);
  pthread_mutex_unlock(&g_registryLock);
  return p;
}

/* =========================================================================
** Internal Helpers
** ========================================================================= */

static void free_event_list(RealtimeEvent *pList){
  while( pList ){
    RealtimeEvent *pNext = pList->pNext;
    if( pList->zDb ) sqlite3_free(pList->zDb);
    if( pList->zTable ) sqlite3_free(pList->zTable);
    sqlite3_free(pList);
    pList = pNext;
  }
}

/*
** Discard all pending events. Used by rollback_hook and to clean up
** orphaned pending events from failed autocommit statements.
*/
static void realtime_discard_pending(RealtimeContext *pCtx){
  if( pCtx->pPendingHead ){
    free_event_list(pCtx->pPendingHead);
    pCtx->pPendingHead = NULL;
    pCtx->pPendingTail = NULL;
    pCtx->nPending = 0;
  }
}

/* =========================================================================
** SQLite Hook Callbacks
** ========================================================================= */

/*
** Update hook callback.
** Invoked by SQLite on every INSERT, UPDATE, DELETE within a transaction.
** Performs O(1) in-memory allocation and append to the connection's pending list.
** No locks taken here — the pending list is connection-local and SQLite
** serializes calls on a single connection.
*/
static void realtime_update_hook(
  void *pArg,
  int op,
  const char *zDb,
  const char *zTable,
  sqlite3_int64 iRowid
){
  RealtimeContext *pCtx = (RealtimeContext *)pArg;
  RealtimeEvent *pEv;
  char *zDbCopy;
  char *zTableCopy;

  if( !pCtx ) return;

  /*
  ** Autocommit orphan cleanup: If we are in autocommit mode (no explicit
  ** transaction) and the pending list is non-empty, it means the previous
  ** autocommit statement triggered update_hook but then failed — neither
  ** commit_hook nor rollback_hook fired. Discard those orphaned events
  ** before processing this new mutation.
  */
  if( pCtx->nPending > 0 && sqlite3_get_autocommit(pCtx->db) ){
    realtime_discard_pending(pCtx);
  }

  /* Allocate event and string copies. If any allocation fails,
  ** free everything and silently drop this event. */
  pEv = (RealtimeEvent *)sqlite3_malloc(sizeof(RealtimeEvent));
  if( !pEv ) return;

  zDbCopy = sqlite3_mprintf("%s", zDb);
  if( !zDbCopy ){
    sqlite3_free(pEv);
    return;
  }

  zTableCopy = sqlite3_mprintf("%s", zTable);
  if( !zTableCopy ){
    sqlite3_free(zDbCopy);
    sqlite3_free(pEv);
    return;
  }

  pEv->op = op;
  pEv->zDb = zDbCopy;
  pEv->zTable = zTableCopy;
  pEv->iRowid = iRowid;
  pEv->pNext = NULL;

  /* Append to pending transaction queue */
  if( pCtx->pPendingTail ){
    pCtx->pPendingTail->pNext = pEv;
    pCtx->pPendingTail = pEv;
  } else {
    pCtx->pPendingHead = pEv;
    pCtx->pPendingTail = pEv;
  }
  pCtx->nPending++;
}

/*
** Commit hook callback.
** Invoked by SQLite when a transaction (explicit or autocommit) is about
** to commit. Atomically moves pending events into the committed queue
** and signals eventfd if the queue was previously empty.
**
** Returns 0 to allow the COMMIT to proceed.
*/
static int realtime_commit_hook(void *pArg){
  RealtimeContext *pCtx = (RealtimeContext *)pArg;
  int wasEmpty;

  if( !pCtx || !pCtx->pPendingHead ) return 0;

  pthread_mutex_lock(&pCtx->commitLock);

  wasEmpty = (pCtx->pCommittedHead == NULL);

  /* Splice entire pending list onto committed list */
  if( pCtx->pCommittedTail ){
    pCtx->pCommittedTail->pNext = pCtx->pPendingHead;
  } else {
    pCtx->pCommittedHead = pCtx->pPendingHead;
  }
  pCtx->pCommittedTail = pCtx->pPendingTail;
  pCtx->nCommitted += pCtx->nPending;

  pthread_mutex_unlock(&pCtx->commitLock);

  /* Clear pending list (no lock needed — connection-local) */
  pCtx->pPendingHead = NULL;
  pCtx->pPendingTail = NULL;
  pCtx->nPending = 0;

  /*
  ** Edge-triggered wakeup: Only write to eventfd if queue was previously
  ** empty. This avoids redundant syscalls during high-frequency commits.
  **
  ** Note: the write happens outside the lock. In the worst case this
  ** causes a spurious wakeup (drain returns 0), which is harmless.
  ** Missed wakeups cannot occur because: if another commit races and
  ** sees wasEmpty=false, it means our events are already in the queue
  ** and either our write or a future drain will pick them up.
  */
  if( wasEmpty && pCtx->eventFd >= 0 ){
    uint64_t val = 1;
    ssize_t ret;
    do {
      ret = write(pCtx->eventFd, &val, sizeof(val));
    } while( ret < 0 && errno == EINTR );
  }

  return 0; /* Allow COMMIT to proceed */
}

/*
** Rollback hook callback.
** Invoked by SQLite when an explicit transaction is rolled back.
** Discards all pending events so no false events are ever published.
*/
static void realtime_rollback_hook(void *pArg){
  RealtimeContext *pCtx = (RealtimeContext *)pArg;
  if( !pCtx ) return;
  realtime_discard_pending(pCtx);
}

/* =========================================================================
** Public API
** ========================================================================= */

/*
** Initialize realtime tracking on a database connection.
*/
int sqlite3_realtime_init(sqlite3 *db, int *pOutEventFd){
  RealtimeContext *pCtx;
  RealtimeContext *pExisting;

  if( !db ) return SQLITE_MISUSE;

  /* Check for double-init on the same connection */
  pthread_mutex_lock(&g_registryLock);
  pExisting = realtime_find_ctx(db);
  pthread_mutex_unlock(&g_registryLock);

  if( pExisting ) return SQLITE_MISUSE;

  pCtx = (RealtimeContext *)sqlite3_malloc(sizeof(RealtimeContext));
  if( !pCtx ) return SQLITE_NOMEM;

  memset(pCtx, 0, sizeof(RealtimeContext));
  pCtx->db = db;
  pCtx->eventFd = -1;

#ifdef __linux__
  pCtx->eventFd = eventfd(0, EFD_NONBLOCK | EFD_CLOEXEC);
  if( pCtx->eventFd < 0 ){
    sqlite3_free(pCtx);
    return SQLITE_ERROR;
  }
#endif

  if( pthread_mutex_init(&pCtx->commitLock, NULL) != 0 ){
    if( pCtx->eventFd >= 0 ) close(pCtx->eventFd);
    sqlite3_free(pCtx);
    return SQLITE_ERROR;
  }

  /* Register SQLite hooks */
  sqlite3_update_hook(db, realtime_update_hook, pCtx);
  sqlite3_commit_hook(db, realtime_commit_hook, pCtx);
  sqlite3_rollback_hook(db, realtime_rollback_hook, pCtx);

  /* Add to global registry */
  pthread_mutex_lock(&g_registryLock);
  pCtx->pNext = g_pContextList;
  g_pContextList = pCtx;
  pthread_mutex_unlock(&g_registryLock);

  if( pOutEventFd ){
    *pOutEventFd = pCtx->eventFd;
  }

  return SQLITE_OK;
}

/*
** Get the eventfd descriptor for epoll registration.
*/
int sqlite3_realtime_get_fd(sqlite3 *db){
  RealtimeContext *pCtx = realtime_find_ctx_safe(db);
  return pCtx ? pCtx->eventFd : -1;
}

/*
** Drain and process all committed change events.
** Returns the number of events processed.
*/
int sqlite3_realtime_drain(
  sqlite3 *db,
  sqlite3_realtime_callback xCallback,
  void *pUserData
){
  RealtimeContext *pCtx = realtime_find_ctx_safe(db);
  RealtimeEvent *pToDrain;
  RealtimeEvent *pCurr;
  int count = 0;

  if( !pCtx ) return 0;

  /* Lock committed queue and atomically swap the list out */
  pthread_mutex_lock(&pCtx->commitLock);
  pToDrain = pCtx->pCommittedHead;
  pCtx->pCommittedHead = NULL;
  pCtx->pCommittedTail = NULL;
  pCtx->nCommitted = 0;
  pthread_mutex_unlock(&pCtx->commitLock);

  /* Drain eventfd counter outside the lock to reduce lock hold time.
  ** This is safe because eventfd is a simple counter: reading resets it
  ** to zero, and any concurrent write will re-increment it. */
  if( pCtx->eventFd >= 0 ){
    uint64_t val;
    ssize_t ret;
    do {
      ret = read(pCtx->eventFd, &val, sizeof(val));
    } while( ret < 0 && errno == EINTR );
  }

  /* Process stolen events outside the lock */
  pCurr = pToDrain;
  while( pCurr ){
    RealtimeEvent *pNext = pCurr->pNext;
    count++;
    if( xCallback ){
      xCallback(pCurr->op, pCurr->zDb, pCurr->zTable, pCurr->iRowid, pUserData);
    }
    sqlite3_free(pCurr->zDb);
    sqlite3_free(pCurr->zTable);
    sqlite3_free(pCurr);
    pCurr = pNext;
  }

  return count;
}

/*
** Cleanup resources for a database connection.
*/
int sqlite3_realtime_cleanup(sqlite3 *db){
  RealtimeContext *pCtx;
  RealtimeContext **pp;

  if( !db ) return SQLITE_MISUSE;

  /* Remove from global registry */
  pthread_mutex_lock(&g_registryLock);
  for( pp = &g_pContextList; *pp; pp = &(*pp)->pNext ){
    if( (*pp)->db == db ){
      pCtx = *pp;
      *pp = pCtx->pNext;
      pthread_mutex_unlock(&g_registryLock);
      goto found;
    }
  }
  pthread_mutex_unlock(&g_registryLock);
  return SQLITE_OK; /* Not initialized, nothing to do */

found:
  /* Unregister hooks */
  sqlite3_update_hook(db, NULL, NULL);
  sqlite3_commit_hook(db, NULL, NULL);
  sqlite3_rollback_hook(db, NULL, NULL);

  if( pCtx->eventFd >= 0 ){
    close(pCtx->eventFd);
    pCtx->eventFd = -1;
  }

  free_event_list(pCtx->pPendingHead);

  pthread_mutex_lock(&pCtx->commitLock);
  free_event_list(pCtx->pCommittedHead);
  pCtx->pCommittedHead = NULL;
  pCtx->pCommittedTail = NULL;
  pthread_mutex_unlock(&pCtx->commitLock);

  pthread_mutex_destroy(&pCtx->commitLock);
  sqlite3_free(pCtx);

  return SQLITE_OK;
}

/* =========================================================================
** SQL User-Defined Functions (UDF) & Extension Entrypoint
** ========================================================================= */

static void realtimeInitFunc(
  sqlite3_context *context,
  int argc,
  sqlite3_value **argv
){
  sqlite3 *db = sqlite3_context_db_handle(context);
  int eventFd = -1;
  int rc;
  (void)argc; (void)argv;

  rc = sqlite3_realtime_init(db, &eventFd);
  if( rc == SQLITE_MISUSE ){
    sqlite3_result_error(context, "realtime already initialized on this connection", -1);
    return;
  }
  if( rc != SQLITE_OK ){
    sqlite3_result_error(context, "failed to initialize realtime extension", -1);
    return;
  }

  sqlite3_result_int(context, eventFd);
}

static void realtimeFdFunc(
  sqlite3_context *context,
  int argc,
  sqlite3_value **argv
){
  sqlite3 *db = sqlite3_context_db_handle(context);
  (void)argc; (void)argv;
  sqlite3_result_int(context, sqlite3_realtime_get_fd(db));
}

/* =========================================================================
** Virtual Table: realtime_events
** Allows querying and draining committed realtime events via pure SQL:
**   SELECT op, db_name, table_name, rowid_val FROM realtime_events;
** ========================================================================= */

typedef struct RealtimeEventsVtab RealtimeEventsVtab;
struct RealtimeEventsVtab {
  sqlite3_vtab base;
  sqlite3 *db;
};

typedef struct RealtimeEventsCursor RealtimeEventsCursor;
struct RealtimeEventsCursor {
  sqlite3_vtab_cursor base;
  RealtimeEvent *pHead;
  RealtimeEvent *pCurrent;
  sqlite3_int64 iRow;
};

static char *realtime_fetch_row_json(sqlite3 *db, const char *zTable, sqlite3_int64 rowid){
  sqlite3_stmt *pStmt = NULL;
  char *zSql = sqlite3_mprintf("SELECT * FROM \"%w\" WHERE rowid = %lld", zTable, (long long)rowid);
  int rc;
  char *zJson = NULL;

  if( !zSql ) return NULL;
  rc = sqlite3_prepare_v2(db, zSql, -1, &pStmt, NULL);
  sqlite3_free(zSql);
  if( rc != SQLITE_OK ) return NULL;

  if( sqlite3_step(pStmt) == SQLITE_ROW ){
    int nCols = sqlite3_column_count(pStmt);
    sqlite3_str *pStr = sqlite3_str_new(db);
    sqlite3_str_appendchar(pStr, 1, '{');
    for(int i = 0; i < nCols; i++){
      const char *zColName = sqlite3_column_name(pStmt, i);
      int type = sqlite3_column_type(pStmt, i);
      if( i > 0 ) sqlite3_str_appendall(pStr, ", ");
      sqlite3_str_appendf(pStr, "\"%w\": ", zColName);
      if( type == SQLITE_INTEGER ){
        sqlite3_str_appendf(pStr, "%lld", (long long)sqlite3_column_int64(pStmt, i));
      } else if( type == SQLITE_FLOAT ){
        sqlite3_str_appendf(pStr, "%f", sqlite3_column_double(pStmt, i));
      } else if( type == SQLITE_NULL ){
        sqlite3_str_appendall(pStr, "null");
      } else {
        const char *zVal = (const char *)sqlite3_column_text(pStmt, i);
        sqlite3_str_appendf(pStr, "\"%w\"", zVal ? zVal : "");
      }
    }
    sqlite3_str_appendchar(pStr, 1, '}');
    zJson = sqlite3_str_finish(pStr);
  }
  sqlite3_finalize(pStmt);
  return zJson;
}

static int realtimeEventsConnect(
  sqlite3 *db,
  void *pAux,
  int argc, const char *const*argv,
  sqlite3_vtab **ppVtab,
  char **pzErr
){
  RealtimeEventsVtab *pNew;
  int rc;
  (void)pAux; (void)argc; (void)argv; (void)pzErr;

  rc = sqlite3_declare_vtab(db, "CREATE TABLE x(op TEXT, db_name TEXT, table_name TEXT, rowid_val INT, data TEXT)");
  if( rc == SQLITE_OK ){
    pNew = (RealtimeEventsVtab *)sqlite3_malloc64(sizeof(*pNew));
    if( !pNew ) return SQLITE_NOMEM;
    memset(pNew, 0, sizeof(*pNew));
    pNew->db = db;
    *ppVtab = (sqlite3_vtab *)pNew;
  }
  return rc;
}

static int realtimeEventsDisconnect(sqlite3_vtab *pVtab){
  sqlite3_free(pVtab);
  return SQLITE_OK;
}

static int realtimeEventsOpen(sqlite3_vtab *pVtab, sqlite3_vtab_cursor **ppCursor){
  RealtimeEventsCursor *pCur;
  (void)pVtab;
  pCur = (RealtimeEventsCursor *)sqlite3_malloc64(sizeof(*pCur));
  if( !pCur ) return SQLITE_NOMEM;
  memset(pCur, 0, sizeof(*pCur));
  *ppCursor = (sqlite3_vtab_cursor *)pCur;
  return SQLITE_OK;
}

static int realtimeEventsClose(sqlite3_vtab_cursor *cur){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  if( pCur->pHead ){
    free_event_list(pCur->pHead);
    pCur->pHead = NULL;
  }
  sqlite3_free(pCur);
  return SQLITE_OK;
}

static int realtimeEventsBestIndex(sqlite3_vtab *pVtab, sqlite3_index_info *pIdxInfo){
  (void)pVtab;
  pIdxInfo->estimatedCost = 1.0;
  pIdxInfo->estimatedRows = 10;
  return SQLITE_OK;
}

static int realtimeEventsFilter(
  sqlite3_vtab_cursor *cur,
  int idxNum, const char *idxStr,
  int argc, sqlite3_value **argv
){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  RealtimeEventsVtab *pVtab = (RealtimeEventsVtab *)cur->pVtab;
  RealtimeContext *pCtx = realtime_find_ctx_safe(pVtab->db);
  (void)idxNum; (void)idxStr; (void)argc; (void)argv;

  if( pCur->pHead ){
    free_event_list(pCur->pHead);
    pCur->pHead = NULL;
  }

  if( pCtx ){
    pthread_mutex_lock(&pCtx->commitLock);
    pCur->pHead = pCtx->pCommittedHead;
    pCtx->pCommittedHead = NULL;
    pCtx->pCommittedTail = NULL;
    pCtx->nCommitted = 0;
    pthread_mutex_unlock(&pCtx->commitLock);

    if( pCtx->eventFd >= 0 ){
      uint64_t val;
      ssize_t ret;
      do {
        ret = read(pCtx->eventFd, &val, sizeof(val));
      } while( ret < 0 && errno == EINTR );
    }
  }

  pCur->pCurrent = pCur->pHead;
  pCur->iRow = 1;
  return SQLITE_OK;
}

static int realtimeEventsNext(sqlite3_vtab_cursor *cur){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  if( pCur->pCurrent ){
    pCur->pCurrent = pCur->pCurrent->pNext;
    pCur->iRow++;
  }
  return SQLITE_OK;
}

static int realtimeEventsEof(sqlite3_vtab_cursor *cur){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  return pCur->pCurrent == NULL;
}

static int realtimeEventsColumn(sqlite3_vtab_cursor *cur, sqlite3_context *ctx, int i){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  RealtimeEventsVtab *pVtab = (RealtimeEventsVtab *)cur->pVtab;
  if( !pCur->pCurrent ) return SQLITE_OK;

  switch( i ){
    case 0: {
      const char *zOp = "UNKNOWN";
      if( pCur->pCurrent->op == SQLITE_INSERT ) zOp = "INSERT";
      else if( pCur->pCurrent->op == SQLITE_UPDATE ) zOp = "UPDATE";
      else if( pCur->pCurrent->op == SQLITE_DELETE ) zOp = "DELETE";
      sqlite3_result_text(ctx, zOp, -1, SQLITE_STATIC);
      break;
    }
    case 1:
      sqlite3_result_text(ctx, pCur->pCurrent->zDb ? pCur->pCurrent->zDb : "", -1, SQLITE_TRANSIENT);
      break;
    case 2:
      sqlite3_result_text(ctx, pCur->pCurrent->zTable ? pCur->pCurrent->zTable : "", -1, SQLITE_TRANSIENT);
      break;
    case 3:
      sqlite3_result_int64(ctx, pCur->pCurrent->iRowid);
      break;
    case 4: {
      if( pCur->pCurrent->op == SQLITE_DELETE ){
        sqlite3_result_null(ctx);
      } else {
        char *zJson = realtime_fetch_row_json(pVtab->db, pCur->pCurrent->zTable, pCur->pCurrent->iRowid);
        if( zJson ){
          sqlite3_result_text(ctx, zJson, -1, SQLITE_TRANSIENT);
          sqlite3_free(zJson);
        } else {
          sqlite3_result_null(ctx);
        }
      }
      break;
    }
    default:
      break;
  }
  return SQLITE_OK;
}

static int realtimeEventsRowid(sqlite3_vtab_cursor *cur, sqlite_int64 *pRowid){
  RealtimeEventsCursor *pCur = (RealtimeEventsCursor *)cur;
  *pRowid = pCur->iRow;
  return SQLITE_OK;
}

static sqlite3_module realtimeEventsModule = {
  0,                         /* iVersion */
  realtimeEventsConnect,     /* xCreate */
  realtimeEventsConnect,     /* xConnect */
  realtimeEventsBestIndex,   /* xBestIndex */
  realtimeEventsDisconnect,  /* xDisconnect */
  realtimeEventsDisconnect,  /* xDestroy */
  realtimeEventsOpen,        /* xOpen */
  realtimeEventsClose,       /* xClose */
  realtimeEventsFilter,      /* xFilter */
  realtimeEventsNext,        /* xNext */
  realtimeEventsEof,         /* xEof */
  realtimeEventsColumn,      /* xColumn */
  realtimeEventsRowid,       /* xRowid */
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0 /* unused */
};

#ifdef _WIN32
__declspec(dllexport)
#endif
int sqlite3_realtime_init_extension(
  sqlite3 *db,
  char **pzErrMsg,
  const sqlite3_api_routines *pApi
){
  int rc = SQLITE_OK;
#ifndef SQLITE_CORE
  SQLITE_EXTENSION_INIT2(pApi);
#endif
  (void)pzErrMsg;

  rc = sqlite3_create_function(
      db, "realtime_init", 0, SQLITE_UTF8, NULL,
      realtimeInitFunc, NULL, NULL
  );
  if( rc != SQLITE_OK ) return rc;

  rc = sqlite3_create_function(
      db, "realtime_fd", 0, SQLITE_UTF8, NULL,
      realtimeFdFunc, NULL, NULL
  );
  if( rc != SQLITE_OK ) return rc;

  rc = sqlite3_create_module(
      db, "realtime_events", &realtimeEventsModule, NULL
  );

  return rc;
}

/*
** Analyze an arbitrary SQL query using SQLite's EXPLAIN QUERY PLAN engine.
** Invokes xCallback for each table dependency discovered by SQLite's optimizer.
*/
/*
** Internal structure mapping a B-tree rootpage to its parent table name.
*/
typedef struct TableRootpageMap TableRootpageMap;
struct TableRootpageMap {
  int rootpage;
  char zTableName[128];
};

/*
** Analyze an arbitrary SQL query using SQLite's VDBE bytecode engine and column metadata.
** Accurately discovers all underlying table dependencies even when aliases (e.g. "users u", "orders o")
** or complex subqueries/JOINs are used.
*/
int sqlite3_realtime_analyze_query(
  sqlite3 *db,
  const char *zSql,
  void (*xCallback)(const char *zTable, void *pUser),
  void *pUser
){
  sqlite3_stmt *pSchemaStmt = NULL;
  sqlite3_stmt *pVdbeStmt = NULL;
  sqlite3_stmt *pDirectStmt = NULL;
  char *zExplainSql = NULL;
  TableRootpageMap map[128];
  int nMap = 0;
  int rc;

  if( !db || !zSql || !xCallback ) return SQLITE_MISUSE;

  /* 1. Build rootpage -> tbl_name map from sqlite_master (covers both tables and indexes) */
  rc = sqlite3_prepare_v2(db, "SELECT tbl_name, rootpage FROM sqlite_master WHERE rootpage > 0", -1, &pSchemaStmt, NULL);
  if( rc == SQLITE_OK ){
    while( sqlite3_step(pSchemaStmt) == SQLITE_ROW && nMap < 128 ){
      const char *zTbl = (const char *)sqlite3_column_text(pSchemaStmt, 0);
      int rootpage = sqlite3_column_int(pSchemaStmt, 1);
      if( zTbl && rootpage > 0 ){
        map[nMap].rootpage = rootpage;
        strncpy(map[nMap].zTableName, zTbl, sizeof(map[nMap].zTableName) - 1);
        map[nMap].zTableName[sizeof(map[nMap].zTableName) - 1] = '\0';
        nMap++;
      }
    }
    sqlite3_finalize(pSchemaStmt);
  }

  /* 2. Check direct column table names via SQLite column metadata */
  rc = sqlite3_prepare_v2(db, zSql, -1, &pDirectStmt, NULL);
  if( rc == SQLITE_OK ){
    int nCols = sqlite3_column_count(pDirectStmt);
    for( int i = 0; i < nCols; i++ ){
      const char *zTbl = sqlite3_column_table_name(pDirectStmt, i);
      if( zTbl && *zTbl ){
        xCallback(zTbl, pUser);
      }
    }
    sqlite3_finalize(pDirectStmt);
  }

  /* 3. Run EXPLAIN <zSql> to inspect VDBE opcodes for table/index scans (OpenRead/OpenWrite) */
  zExplainSql = sqlite3_mprintf("EXPLAIN %s", zSql);
  if( !zExplainSql ) return SQLITE_NOMEM;

  rc = sqlite3_prepare_v2(db, zExplainSql, -1, &pVdbeStmt, NULL);
  sqlite3_free(zExplainSql);
  if( rc != SQLITE_OK ) return rc;

  while( sqlite3_step(pVdbeStmt) == SQLITE_ROW ){
    const char *zOp = (const char *)sqlite3_column_text(pVdbeStmt, 1);
    if( zOp && (strncmp(zOp, "OpenRead", 8) == 0 || strncmp(zOp, "OpenWrite", 9) == 0 || strncmp(zOp, "ReopenIdx", 9) == 0) ){
      int rootpage = sqlite3_column_int(pVdbeStmt, 2); /* P2 is rootpage */
      if( rootpage > 0 ){
        for( int m = 0; m < nMap; m++ ){
          if( map[m].rootpage == rootpage ){
            xCallback(map[m].zTableName, pUser);
            break;
          }
        }
      }
    }
  }

  sqlite3_finalize(pVdbeStmt);
  return SQLITE_OK;
}

#ifndef SQLITE_CORE
#ifdef _WIN32
__declspec(dllexport)
#endif
int sqlite3_extension_init(
  sqlite3 *db,
  char **pzErrMsg,
  const sqlite3_api_routines *pApi
){
  return sqlite3_realtime_init_extension(db, pzErrMsg, pApi);
}
#endif

#endif /* !defined(SQLITE_CORE) || defined(SQLITE_ENABLE_REALTIME) */

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
int sqlite3_realtime_attach_if_needed(sqlite3 *db, int *pOutEventFd, int *pAlreadyAttached){
  RealtimeContext *pExisting;

  if( !db || !pOutEventFd || !pAlreadyAttached ) return SQLITE_MISUSE;

  *pAlreadyAttached = 0;

  pthread_mutex_lock(&g_registryLock);
  pExisting = realtime_find_ctx(db);
  pthread_mutex_unlock(&g_registryLock);

  if( pExisting ){
    /* Already carrying hooks — report the existing eventfd and do nothing.
    ** Re-running init would hit the double-init guard (SQLITE_MISUSE). */
    *pAlreadyAttached = 1;
    *pOutEventFd = pExisting->eventFd;
    return SQLITE_OK;
  }

  return sqlite3_realtime_init(db, pOutEventFd);
}

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


#endif /* !defined(SQLITE_CORE) || defined(SQLITE_ENABLE_REALTIME) */

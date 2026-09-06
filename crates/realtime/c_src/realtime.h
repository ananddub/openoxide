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
** Header file for SQLite Realtime / Subscription Extension.
** Provides transactional change capture with Linux eventfd/epoll integration.
*/

#ifndef SQLITE_REALTIME_H
#define SQLITE_REALTIME_H

#include "sqlite3.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Change notification callback signature */
typedef void (*sqlite3_realtime_callback)(
    int op,                /* SQLITE_INSERT, SQLITE_UPDATE, SQLITE_DELETE */
    const char *zDb,       /* Database name, e.g. "main" */
    const char *zTable,    /* Table name */
    sqlite3_int64 iRowid,  /* Row ID */
    void *pUserData        /* User context pointer */
);

/*
** Initialize realtime tracking on a database connection.
** Returns SQLITE_OK on success and sets *pOutEventFd to the eventfd descriptor.
** Returns SQLITE_MISUSE if called twice on the same connection without cleanup.
** Returns SQLITE_ERROR if eventfd creation fails (Linux only).
*/
int sqlite3_realtime_init(sqlite3 *db, int *pOutEventFd);

/*
** Get the eventfd descriptor associated with the database connection.
** Returns -1 if not initialized.
*/
int sqlite3_realtime_get_fd(sqlite3 *db);

/*
** Drain and process all committed change events.
** Calls xCallback for each change event and resets eventfd.
** Returns the number of events processed.
*/
int sqlite3_realtime_drain(
    sqlite3 *db,
    sqlite3_realtime_callback xCallback,
    void *pUserData
);

/*
** Cleanup realtime tracking resources for a database connection.
** Safe to call even if not initialized (returns SQLITE_OK).
*/
int sqlite3_realtime_cleanup(sqlite3 *db);

/*
** Idempotent attach: returns SQLITE_OK with *pAlreadyAttached set to 1 when
** realtime is already attached to this connection; otherwise behaves exactly
** like sqlite3_realtime_init (and sets *pAlreadyAttached to 0). Used by pool
** layers that re-open connections — every pooled connection must carry the
** change hooks.
*/
int sqlite3_realtime_attach_if_needed(sqlite3 *db, int *pOutEventFd, int *pAlreadyAttached);

/*
** Query dependency analyzer callback signature.
** Called once for each table discovered in the query plan.
*/
typedef void (*sqlite3_realtime_table_callback)(
    const char *zTable,    /* Table name (unquoted, without schema prefix) */
    void *pUser            /* User context pointer */
);

/*
** Analyze an arbitrary SQL query using SQLite's EXPLAIN QUERY PLAN engine.
** Invokes xCallback once for each dependent table discovered by the optimizer.
**
** For a JOIN query like "SELECT * FROM users u JOIN orders o ON ...",
** xCallback will be called twice: once with "users" and once with "orders".
**
** Handles quoted identifiers ("order details"), schema prefixes (main.users),
** and skips non-table entries (SUBQUERY, CONSTANT, TEMP B-TREE).
**
** Returns SQLITE_OK on success, SQLITE_MISUSE if arguments are NULL,
** or the error code from sqlite3_prepare_v2 if the SQL is invalid.
*/
int sqlite3_realtime_analyze_query(
    sqlite3 *db,
    const char *zSql,
    sqlite3_realtime_table_callback xCallback,
    void *pUser
);

#ifdef __cplusplus
}
#endif

#endif /* SQLITE_REALTIME_H */

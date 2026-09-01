/*
** 2026-08-31
**
** Test & Verification runner for SQLite Realtime / Subscription extension.
** Demonstrates:
**   1. Transactional change capture (Commit vs Rollback).
**   2. Linux epoll reactor thread waking up via eventfd.
**   3. Draining committed changes without blocking the writer connection.
**   4. Double-init guard.
**   5. Multi-connection support.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/epoll.h>
#include <string.h>
#include <stdatomic.h>
#include <assert.h>

#include "sqlite3.h"
#include "realtime.h"

static atomic_int g_stopReactor = 0;
static atomic_int g_eventCount = 0;
static int g_testsPassed = 0;
static int g_testsFailed = 0;

#define TEST_ASSERT(cond, msg) do { \
  if( cond ){ \
    printf("  ✅ PASS: %s\n", msg); \
    g_testsPassed++; \
  } else { \
    printf("  ❌ FAIL: %s\n", msg); \
    g_testsFailed++; \
  } \
} while(0)

/*
** Callback invoked for each drained event on the reactor thread.
*/
static void on_realtime_event(
    int op,
    const char *zDb,
    const char *zTable,
    sqlite3_int64 iRowid,
    void *pUserData
){
    (void)pUserData;
    const char *opStr = "UNKNOWN";
    if( op == SQLITE_INSERT ) opStr = "INSERT";
    else if( op == SQLITE_UPDATE ) opStr = "UPDATE";
    else if( op == SQLITE_DELETE ) opStr = "DELETE";

    printf("    [Event] Op: %-6s | DB: %-4s | Table: %-10s | RowID: %lld\n",
           opStr, zDb, zTable, (long long)iRowid);
    atomic_fetch_add(&g_eventCount, 1);
}

/*
** Background Reactor Thread running epoll_wait.
*/
static void *reactor_thread_main(void *pArg){
    sqlite3 *db = (sqlite3 *)pArg;
    int eventFd = sqlite3_realtime_get_fd(db);
    int epollFd = epoll_create1(0);
    struct epoll_event ev, events[8];

    if( epollFd < 0 || eventFd < 0 ){
        perror("epoll_create1 or invalid eventFd");
        return NULL;
    }

    ev.events = EPOLLIN;
    ev.data.fd = eventFd;
    if( epoll_ctl(epollFd, EPOLL_CTL_ADD, eventFd, &ev) < 0 ){
        perror("epoll_ctl ADD");
        close(epollFd);
        return NULL;
    }

    printf("[Reactor] epoll loop started (eventfd=%d)\n", eventFd);

    while( !atomic_load(&g_stopReactor) ){
        int nfds = epoll_wait(epollFd, events, 8, 200 /* 200ms timeout */);
        for( int i = 0; i < nfds; i++ ){
            if( events[i].data.fd == eventFd ){
                int drained = sqlite3_realtime_drain(db, on_realtime_event, NULL);
                printf("    [Reactor] Drained %d event(s)\n", drained);
            }
        }
    }

    close(epollFd);
    printf("[Reactor] stopped\n");
    return NULL;
}

int main(void){
    sqlite3 *db = NULL;
    pthread_t reactorThread;
    int eventFd = -1;
    int rc;

    printf("====================================================\n");
    printf("   SQLite Realtime Extension — Test Suite\n");
    printf("====================================================\n\n");

    /* 1. Open Database */
    rc = sqlite3_open(":memory:", &db);
    if( rc != SQLITE_OK ){
        fprintf(stderr, "Failed to open db: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    /* ── Test: Initialize ── */
    printf("--- Test: Init ---\n");
    rc = sqlite3_realtime_init(db, &eventFd);
    TEST_ASSERT(rc == SQLITE_OK, "sqlite3_realtime_init returns SQLITE_OK");
    TEST_ASSERT(eventFd >= 0, "eventfd is a valid descriptor");

    /* ── Test: Double-init guard ── */
    printf("\n--- Test: Double-init guard ---\n");
    rc = sqlite3_realtime_init(db, NULL);
    TEST_ASSERT(rc == SQLITE_MISUSE, "Double init returns SQLITE_MISUSE");

    /* ── Test: get_fd returns correct fd ── */
    printf("\n--- Test: get_fd ---\n");
    TEST_ASSERT(sqlite3_realtime_get_fd(db) == eventFd, "get_fd matches init fd");

    /* Start reactor thread */
    if( pthread_create(&reactorThread, NULL, reactor_thread_main, db) != 0 ){
        perror("pthread_create");
        return 1;
    }
    usleep(50000);

    /* Create tables */
    sqlite3_exec(db, "CREATE TABLE users(id INTEGER PRIMARY KEY, name TEXT, age INT);", 0, 0, 0);
    sqlite3_exec(db, "CREATE TABLE orders(id INTEGER PRIMARY KEY, user_id INT, amount REAL);", 0, 0, 0);

    /* ── Test: Single INSERT (autocommit) ── */
    printf("\n--- Test: Single INSERT (autocommit) ---\n");
    int before = atomic_load(&g_eventCount);
    sqlite3_exec(db, "INSERT INTO users VALUES(1, 'Alice', 25);", 0, 0, 0);
    usleep(150000);
    TEST_ASSERT(atomic_load(&g_eventCount) == before + 1, "1 INSERT event received");

    /* ── Test: Batch Transaction ── */
    printf("\n--- Test: Batch transaction (3 writes, 1 commit) ---\n");
    before = atomic_load(&g_eventCount);
    sqlite3_exec(db, "BEGIN;", 0, 0, 0);
    sqlite3_exec(db, "INSERT INTO users VALUES(2, 'Bob', 30);", 0, 0, 0);
    sqlite3_exec(db, "INSERT INTO users VALUES(3, 'Charlie', 35);", 0, 0, 0);
    sqlite3_exec(db, "INSERT INTO orders VALUES(101, 1, 99.99);", 0, 0, 0);
    sqlite3_exec(db, "COMMIT;", 0, 0, 0);
    usleep(150000);
    TEST_ASSERT(atomic_load(&g_eventCount) == before + 3, "3 batch events received");

    /* ── Test: Rollback discards events ── */
    printf("\n--- Test: Rollback discards all pending events ---\n");
    before = atomic_load(&g_eventCount);
    sqlite3_exec(db, "BEGIN;", 0, 0, 0);
    sqlite3_exec(db, "INSERT INTO users VALUES(99, 'Ghost', 0);", 0, 0, 0);
    sqlite3_exec(db, "DELETE FROM users WHERE id = 1;", 0, 0, 0);
    sqlite3_exec(db, "ROLLBACK;", 0, 0, 0);
    usleep(150000);
    TEST_ASSERT(atomic_load(&g_eventCount) == before, "Rollback: zero false events");

    /* ── Test: UPDATE and DELETE ── */
    printf("\n--- Test: UPDATE and DELETE ---\n");
    before = atomic_load(&g_eventCount);
    sqlite3_exec(db, "UPDATE users SET age = 26 WHERE id = 1;", 0, 0, 0);
    sqlite3_exec(db, "DELETE FROM orders WHERE id = 101;", 0, 0, 0);
    usleep(150000);
    TEST_ASSERT(atomic_load(&g_eventCount) == before + 2, "UPDATE + DELETE events received");

    /* ── Test: Multi-connection support ── */
    printf("\n--- Test: Multi-connection support ---\n");
    {
        sqlite3 *db2 = NULL;
        int eventFd2 = -1;
        sqlite3_open(":memory:", &db2);
        rc = sqlite3_realtime_init(db2, &eventFd2);
        TEST_ASSERT(rc == SQLITE_OK, "Second connection init succeeds");
        TEST_ASSERT(eventFd2 >= 0, "Second connection has valid eventfd");
        TEST_ASSERT(eventFd2 != eventFd, "Second connection has different eventfd");
        TEST_ASSERT(sqlite3_realtime_get_fd(db2) == eventFd2, "get_fd works for second connection");
        TEST_ASSERT(sqlite3_realtime_get_fd(db) == eventFd, "First connection fd unchanged");
        sqlite3_realtime_cleanup(db2);
        sqlite3_close(db2);
        TEST_ASSERT(sqlite3_realtime_get_fd(db) == eventFd, "First connection still works after second cleanup");
    }

    /* ── Test: Cleanup idempotency ── */
    printf("\n--- Test: Cleanup ---\n");
    {
        sqlite3 *db3 = NULL;
        sqlite3_open(":memory:", &db3);
        rc = sqlite3_realtime_cleanup(db3); /* Never initialized */
        TEST_ASSERT(rc == SQLITE_OK, "Cleanup on un-initialized connection is safe");
        sqlite3_close(db3);
    }

    /* Shutdown */
    atomic_store(&g_stopReactor, 1);
    pthread_join(reactorThread, NULL);
    sqlite3_realtime_cleanup(db);
    sqlite3_close(db);

    printf("\n====================================================\n");
    printf("   Results: %d passed, %d failed\n", g_testsPassed, g_testsFailed);
    printf("   Total events processed: %d\n", atomic_load(&g_eventCount));
    printf("====================================================\n");

    return g_testsFailed > 0 ? 1 : 0;
}

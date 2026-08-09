# Rustploy Background Queue & Scheduler Architecture

This document details the Background Queue Engine and Cron Scheduler built into Rustploy.

---

## 1. Architecture Overview

```
                      ┌───────────────────────────────┐
                      │    Background Scheduler Loop  │
                      │    (Tokio Async Sleep Loop)   │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │      ScheduleRepository       │
                      │   (Fetch Due Cron Tasks)      │
                      └───────────────┬───────────────┘
                                      │
      ┌──────────────────┬────────────┴─────────────┬──────────────────┐
      ▼                  ▼                          ▼                  ▼
┌──────────────┐ ┌──────────────┐          ┌──────────────┐   ┌──────────────┐
│ Missed Run   │ │ Concurrency  │          │ Cron Expression│  │ Async Task   │
│ Policy Guard │ │ Policy Guard │          │ Evaluator    │   │ Execution    │
│(SKIP/RUN_ONCE) │(FORBID/ALLOW)│          │ (5-field)    │   │ (Worker)     │
└──────────────┘ └──────────────┘          └──────────────┘   └──────────────┘
```

---

## 2. Queue & Concurrency Policies

### 2.1 Missed Run Policy (`MissedRunPolicy`)
When the server restarts or recovers from downtime:
- **`SKIP`**: Ignores all missed executions during the downtime and calculates the next future cron timestamp.
- **`RUN_ONCE`**: Executes one immediate catch-up run before returning to the normal schedule.

### 2.2 Concurrency Policy (`ConcurrencyPolicy`)
When a scheduled job's previous execution is still running:
- **`FORBID`**: Skips the new execution to prevent thread/resource exhaustion.
- **`ALLOW`**: Runs the new task concurrently alongside the active task.
- **`REPLACE`**: Cancels the running task via Tokio cancellation token and spawns the new task.

---

## 3. Supported Task Triggers (`ScheduleTriggerKind`)

1. **`BACKUP`**: Automated scheduled database and volume backups.
2. **`DEPLOYMENT`**: Scheduled application re-deployments and image pulls.
3. **`COMMAND`**: Scheduled remote shell command / maintenance script executions.

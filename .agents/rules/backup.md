# Rustploy Backup & Disaster Recovery Architecture

The **Backup Service** in Rustploy provides automated scheduled database dumps (`pg_dump`, `mysqldump`, `mongodump`), volume archive snapshots, S3 / S3-compatible cloud storage uploads, and one-click database/volume restores.

---

## 1. Architecture Overview

```
                        ┌───────────────────────────────┐
                        │       BackupController        │
                        │     (/api/backups/...)        │
                        └───────────────┬───────────────┘
                                        │
                                        ▼
                        ┌───────────────────────────────┐
                        │         BackupService         │
                        └───────────────┬───────────────┘
                                        │
        ┌───────────────────────────────┼───────────────────────────────┐
        ▼                               ▼                               ▼
┌───────────────────────┐   ┌───────────────────────┐   ┌───────────────────────┐
│ BackupExecutions Repo │   │   Database / Volume   │   │ S3 Storage Engine     │
│ (Audit Execution)     │   │     Dump Engine       │   │ (AWS S3/MinIO/Cloud)  │
└───────────────────────┘   └───────────┬───────────┘   └───────────────────────┘
                                        │
                                        ▼
                            ┌───────────────────────┐
                            │ Backup Retention &    │
                            │ Pruning Service       │
                            └───────────────────────┘
```

---

## 2. Detailed Internal Working Mechanism

The Backup Engine executes through a 5-phase pipeline:

```
[1. Trigger Init]   ──► Scheduled Cron or API Request ──► Execution Entry ('RUNNING')
                             │
[2. Dump Generation]──► Execute `pg_dump`/`mysqldump`/`tar.gz` volume snapshot
                             │
[3. Compress & Encrypt]► Gzip Compression + SHA-256 Checksum Calculation
                             │
[4. S3 Upload]      ──► Stream archive to AWS S3 / MinIO / DigitalOcean Spaces
                             │
[5. Prune & Audit]  ──► Retention Policy Enforcement ──► Status Updated ('COMPLETED')
```

### Phase 1: Trigger & Execution Entry (`backup_executions.rs`)
1. Initiated via cron schedule (`ScheduleService`) or manual API trigger.
2. Creates record in `backup_executions` table with `status = 'RUNNING'`.

### Phase 2: Engine-Specific Dump Generation (`panel.rs` & `compose_config.rs`)
Invokes native database dump commands inside target container / remote server node:
- **Postgres**: Executes `pg_dump -U <user> -d <dbname> -F c`.
- **MySQL / MariaDB**: Executes `mysqldump --single-transaction --quick -u <user> -p<pass> <dbname>`.
- **Mongo**: Executes `mongodump --archive --gzip`.
- **Redis**: Triggers `BGSAVE` or dumps `dump.rdb`.
- **Volume Backup**: Creates gzipped tarball archive (`tar -czf backup.tar.gz /var/lib/docker/volumes/<vol>/_data`).

### Phase 3: Compression & Integrity Verification
1. Compresses raw dump using Gzip.
2. Computes file size and SHA-256 checksum hash for integrity verification during restores.

### Phase 4: S3 Cloud Upload (`destination.rs`)
1. Resolves active S3 destination credentials (AWS S3, MinIO, Cloudflare R2, DigitalOcean Spaces, Wasabi).
2. Uploads compressed backup archive to S3 bucket path (`backups/<org_id>/<db_kind>/<timestamp>.sql.gz`).

### Phase 5: Retention Enforcement & Audit (`service.rs`)
1. Checks retention policy (e.g. keep last 14 daily backups).
2. Automatically deletes older remote S3 objects exceeding the retention count.
3. Updates `backup_executions` status to `'COMPLETED'` (or `'FAILED'` with error logs if an error occurred).

---

## 3. Supported Backup Target Types (`BackupKind`)

- **`Database`**: Database dumps for Postgres, MySQL, MariaDB, Mongo, Redis, LibSQL.
- **`Volume`**: Persistent Docker Volume tarball snapshots.
- **`Panel`**: Complete Rustploy Panel SQLite database & system config backups.
- **`ComposeConfig`**: Multi-container Compose YAML stack definitions & environment backups.

---

## 4. Database Schema & Models

### 4.1 `backup_executions` Table
- `id` (INTEGER PRIMARY KEY)
- `backup_kind` (TEXT NOT NULL): `'DATABASE'`, `'VOLUME'`, `'PANEL'`, `'COMPOSE_CONFIG'`.
- `operation` (TEXT NOT NULL): `'BACKUP'`, `'RESTORE'`.
- `status` (TEXT NOT NULL): `'RUNNING'`, `'COMPLETED'`, `'FAILED'`.
- `size_bytes` (INTEGER NULLABLE).
- `checksum` (TEXT NULLABLE).
- `s3_key` (TEXT NULLABLE).
- `organization_id` (INTEGER REFERENCES organization(id)).
- `created_at` (INTEGER DEFAULT (unixepoch())).
- `finished_at` (INTEGER NULLABLE).

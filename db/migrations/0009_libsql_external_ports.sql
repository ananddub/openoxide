-- Migration 0009: Add LibSQL gRPC/admin external port columns.
ALTER TABLE libsql_dbs ADD COLUMN external_grpc_port INTEGER;
ALTER TABLE libsql_dbs ADD COLUMN external_admin_port INTEGER;

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/backup.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use backupKindDescriptor instead')
const BackupKind$json = {
  '1': 'BackupKind',
  '2': [
    {'1': 'BACKUP_KIND_UNSPECIFIED', '2': 0},
    {'1': 'BACKUP_KIND_DATABASE', '2': 1},
    {'1': 'BACKUP_KIND_VOLUME', '2': 2},
    {'1': 'BACKUP_KIND_PANEL', '2': 3},
    {'1': 'BACKUP_KIND_COMPOSE_CONFIG', '2': 4},
  ],
};

/// Descriptor for `BackupKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List backupKindDescriptor = $convert.base64Decode(
    'CgpCYWNrdXBLaW5kEhsKF0JBQ0tVUF9LSU5EX1VOU1BFQ0lGSUVEEAASGAoUQkFDS1VQX0tJTk'
    'RfREFUQUJBU0UQARIWChJCQUNLVVBfS0lORF9WT0xVTUUQAhIVChFCQUNLVVBfS0lORF9QQU5F'
    'TBADEh4KGkJBQ0tVUF9LSU5EX0NPTVBPU0VfQ09ORklHEAQ=');

@$core.Deprecated('Use backupOperationDescriptor instead')
const BackupOperation$json = {
  '1': 'BackupOperation',
  '2': [
    {'1': 'BACKUP_OPERATION_UNSPECIFIED', '2': 0},
    {'1': 'BACKUP_OPERATION_BACKUP', '2': 1},
    {'1': 'BACKUP_OPERATION_RESTORE', '2': 2},
  ],
};

/// Descriptor for `BackupOperation`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List backupOperationDescriptor = $convert.base64Decode(
    'Cg9CYWNrdXBPcGVyYXRpb24SIAocQkFDS1VQX09QRVJBVElPTl9VTlNQRUNJRklFRBAAEhsKF0'
    'JBQ0tVUF9PUEVSQVRJT05fQkFDS1VQEAESHAoYQkFDS1VQX09QRVJBVElPTl9SRVNUT1JFEAI=');

@$core.Deprecated('Use backupExecutionStatusDescriptor instead')
const BackupExecutionStatus$json = {
  '1': 'BackupExecutionStatus',
  '2': [
    {'1': 'BACKUP_EXECUTION_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'BACKUP_EXECUTION_STATUS_RUNNING', '2': 1},
    {'1': 'BACKUP_EXECUTION_STATUS_SUCCEEDED', '2': 2},
    {'1': 'BACKUP_EXECUTION_STATUS_FAILED', '2': 3},
  ],
};

/// Descriptor for `BackupExecutionStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List backupExecutionStatusDescriptor = $convert.base64Decode(
    'ChVCYWNrdXBFeGVjdXRpb25TdGF0dXMSJwojQkFDS1VQX0VYRUNVVElPTl9TVEFUVVNfVU5TUE'
    'VDSUZJRUQQABIjCh9CQUNLVVBfRVhFQ1VUSU9OX1NUQVRVU19SVU5OSU5HEAESJQohQkFDS1VQ'
    'X0VYRUNVVElPTl9TVEFUVVNfU1VDQ0VFREVEEAISIgoeQkFDS1VQX0VYRUNVVElPTl9TVEFUVV'
    'NfRkFJTEVEEAM=');

@$core.Deprecated('Use backupDatabaseDescriptor instead')
const BackupDatabase$json = {
  '1': 'BackupDatabase',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'app_name', '3': 2, '4': 1, '5': 9, '10': 'appName'},
    {'1': 'schedule', '3': 3, '4': 1, '5': 9, '10': 'schedule'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'database_name', '3': 5, '4': 1, '5': 9, '10': 'databaseName'},
    {'1': 'prefix', '3': 6, '4': 1, '5': 9, '10': 'prefix'},
    {
      '1': 'service_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'keepLatestCount',
      '17': true
    },
    {'1': 'backup_type', '3': 9, '4': 1, '5': 9, '10': 'backupType'},
    {
      '1': 'database_type',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'databaseType',
      '17': true
    },
    {
      '1': 'metadata',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'service_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'serviceId',
      '17': true
    },
    {'1': 'destination_id', '3': 13, '4': 1, '5': 3, '10': 'destinationId'},
    {'1': 'organization_id', '3': 14, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 15, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 16, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_database_type'},
    {'1': '_metadata'},
    {'1': '_service_id'},
  ],
};

/// Descriptor for `BackupDatabase`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDatabaseDescriptor = $convert.base64Decode(
    'Cg5CYWNrdXBEYXRhYmFzZRIOCgJpZBgBIAEoA1ICaWQSGQoIYXBwX25hbWUYAiABKAlSB2FwcE'
    '5hbWUSGgoIc2NoZWR1bGUYAyABKAlSCHNjaGVkdWxlEhgKB2VuYWJsZWQYBCABKAhSB2VuYWJs'
    'ZWQSIwoNZGF0YWJhc2VfbmFtZRgFIAEoCVIMZGF0YWJhc2VOYW1lEhYKBnByZWZpeBgGIAEoCV'
    'IGcHJlZml4EiYKDHNlcnZpY2VfbmFtZRgHIAEoCUgAUgtzZXJ2aWNlTmFtZYgBARIvChFrZWVw'
    'X2xhdGVzdF9jb3VudBgIIAEoA0gBUg9rZWVwTGF0ZXN0Q291bnSIAQESHwoLYmFja3VwX3R5cG'
    'UYCSABKAlSCmJhY2t1cFR5cGUSKAoNZGF0YWJhc2VfdHlwZRgKIAEoCUgCUgxkYXRhYmFzZVR5'
    'cGWIAQESHwoIbWV0YWRhdGEYCyABKAlIA1IIbWV0YWRhdGGIAQESIgoKc2VydmljZV9pZBgMIA'
    'EoA0gEUglzZXJ2aWNlSWSIAQESJQoOZGVzdGluYXRpb25faWQYDSABKANSDWRlc3RpbmF0aW9u'
    'SWQSJwoPb3JnYW5pemF0aW9uX2lkGA4gASgDUg5vcmdhbml6YXRpb25JZBIdCgpjcmVhdGVkX2'
    'F0GA8gASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgQIAEoA1IJdXBkYXRlZEF0Qg8KDV9z'
    'ZXJ2aWNlX25hbWVCFAoSX2tlZXBfbGF0ZXN0X2NvdW50QhAKDl9kYXRhYmFzZV90eXBlQgsKCV'
    '9tZXRhZGF0YUINCgtfc2VydmljZV9pZA==');

@$core.Deprecated('Use backupVolumeDescriptor instead')
const BackupVolume$json = {
  '1': 'BackupVolume',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'volume_name', '3': 3, '4': 1, '5': 9, '10': 'volumeName'},
    {'1': 'prefix', '3': 4, '4': 1, '5': 9, '10': 'prefix'},
    {'1': 'service_type', '3': 5, '4': 1, '5': 9, '10': 'serviceType'},
    {'1': 'app_name', '3': 6, '4': 1, '5': 9, '10': 'appName'},
    {
      '1': 'service_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'serviceName',
      '17': true
    },
    {'1': 'turn_off', '3': 8, '4': 1, '5': 8, '10': 'turnOff'},
    {'1': 'cron_expression', '3': 9, '4': 1, '5': 9, '10': 'cronExpression'},
    {
      '1': 'keep_latest_count',
      '3': 10,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'keepLatestCount',
      '17': true
    },
    {'1': 'enabled', '3': 11, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'destination_id', '3': 12, '4': 1, '5': 3, '10': 'destinationId'},
    {'1': 'organization_id', '3': 13, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'service_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'postgres_id',
      '3': 17,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'postgresId',
      '17': true
    },
    {
      '1': 'mysql_id',
      '3': 18,
      '4': 1,
      '5': 3,
      '9': 6,
      '10': 'mysqlId',
      '17': true
    },
    {
      '1': 'mariadb_id',
      '3': 19,
      '4': 1,
      '5': 3,
      '9': 7,
      '10': 'mariadbId',
      '17': true
    },
    {
      '1': 'mongo_id',
      '3': 20,
      '4': 1,
      '5': 3,
      '9': 8,
      '10': 'mongoId',
      '17': true
    },
    {
      '1': 'redis_id',
      '3': 21,
      '4': 1,
      '5': 3,
      '9': 9,
      '10': 'redisId',
      '17': true
    },
    {
      '1': 'libsql_id',
      '3': 22,
      '4': 1,
      '5': 3,
      '9': 10,
      '10': 'libsqlId',
      '17': true
    },
    {'1': 'created_at', '3': 23, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 24, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_service_id'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_postgres_id'},
    {'1': '_mysql_id'},
    {'1': '_mariadb_id'},
    {'1': '_mongo_id'},
    {'1': '_redis_id'},
    {'1': '_libsql_id'},
  ],
};

/// Descriptor for `BackupVolume`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupVolumeDescriptor = $convert.base64Decode(
    'CgxCYWNrdXBWb2x1bWUSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSHwoLdm'
    '9sdW1lX25hbWUYAyABKAlSCnZvbHVtZU5hbWUSFgoGcHJlZml4GAQgASgJUgZwcmVmaXgSIQoM'
    'c2VydmljZV90eXBlGAUgASgJUgtzZXJ2aWNlVHlwZRIZCghhcHBfbmFtZRgGIAEoCVIHYXBwTm'
    'FtZRImCgxzZXJ2aWNlX25hbWUYByABKAlIAFILc2VydmljZU5hbWWIAQESGQoIdHVybl9vZmYY'
    'CCABKAhSB3R1cm5PZmYSJwoPY3Jvbl9leHByZXNzaW9uGAkgASgJUg5jcm9uRXhwcmVzc2lvbh'
    'IvChFrZWVwX2xhdGVzdF9jb3VudBgKIAEoA0gBUg9rZWVwTGF0ZXN0Q291bnSIAQESGAoHZW5h'
    'YmxlZBgLIAEoCFIHZW5hYmxlZBIlCg5kZXN0aW5hdGlvbl9pZBgMIAEoA1INZGVzdGluYXRpb2'
    '5JZBInCg9vcmdhbml6YXRpb25faWQYDSABKANSDm9yZ2FuaXphdGlvbklkEiIKCnNlcnZpY2Vf'
    'aWQYDiABKANIAlIJc2VydmljZUlkiAEBEioKDmFwcGxpY2F0aW9uX2lkGA8gASgDSANSDWFwcG'
    'xpY2F0aW9uSWSIAQESIgoKY29tcG9zZV9pZBgQIAEoA0gEUgljb21wb3NlSWSIAQESJAoLcG9z'
    'dGdyZXNfaWQYESABKANIBVIKcG9zdGdyZXNJZIgBARIeCghteXNxbF9pZBgSIAEoA0gGUgdteX'
    'NxbElkiAEBEiIKCm1hcmlhZGJfaWQYEyABKANIB1IJbWFyaWFkYklkiAEBEh4KCG1vbmdvX2lk'
    'GBQgASgDSAhSB21vbmdvSWSIAQESHgoIcmVkaXNfaWQYFSABKANICVIHcmVkaXNJZIgBARIgCg'
    'lsaWJzcWxfaWQYFiABKANIClIIbGlic3FsSWSIAQESHQoKY3JlYXRlZF9hdBgXIAEoA1IJY3Jl'
    'YXRlZEF0Eh0KCnVwZGF0ZWRfYXQYGCABKANSCXVwZGF0ZWRBdEIPCg1fc2VydmljZV9uYW1lQh'
    'QKEl9rZWVwX2xhdGVzdF9jb3VudEINCgtfc2VydmljZV9pZEIRCg9fYXBwbGljYXRpb25faWRC'
    'DQoLX2NvbXBvc2VfaWRCDgoMX3Bvc3RncmVzX2lkQgsKCV9teXNxbF9pZEINCgtfbWFyaWFkYl'
    '9pZEILCglfbW9uZ29faWRCCwoJX3JlZGlzX2lkQgwKCl9saWJzcWxfaWQ=');

@$core.Deprecated('Use backupExecutionDescriptor instead')
const BackupExecution$json = {
  '1': 'BackupExecution',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'backup_kind',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.BackupKind',
      '10': 'backupKind'
    },
    {
      '1': 'operation',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.BackupOperation',
      '10': 'operation'
    },
    {
      '1': 'backup_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'backupId',
      '17': true
    },
    {
      '1': 'status',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.BackupExecutionStatus',
      '10': 'status'
    },
    {
      '1': 'object_key',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'objectKey',
      '17': true
    },
    {
      '1': 'checksum_sha256',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'checksumSha256',
      '17': true
    },
    {
      '1': 'size_bytes',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'sizeBytes',
      '17': true
    },
    {'1': 'attempt', '3': 9, '4': 1, '5': 3, '10': 'attempt'},
    {'1': 'error', '3': 10, '4': 1, '5': 9, '9': 4, '10': 'error', '17': true},
    {'1': 'started_at', '3': 11, '4': 1, '5': 3, '10': 'startedAt'},
    {
      '1': 'finished_at',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'finishedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_backup_id'},
    {'1': '_object_key'},
    {'1': '_checksum_sha256'},
    {'1': '_size_bytes'},
    {'1': '_error'},
    {'1': '_finished_at'},
  ],
};

/// Descriptor for `BackupExecution`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupExecutionDescriptor = $convert.base64Decode(
    'Cg9CYWNrdXBFeGVjdXRpb24SDgoCaWQYASABKANSAmlkEj0KC2JhY2t1cF9raW5kGAIgASgOMh'
    'wub3Blbm94aWRlLmFwaS52MS5CYWNrdXBLaW5kUgpiYWNrdXBLaW5kEj8KCW9wZXJhdGlvbhgD'
    'IAEoDjIhLm9wZW5veGlkZS5hcGkudjEuQmFja3VwT3BlcmF0aW9uUglvcGVyYXRpb24SIAoJYm'
    'Fja3VwX2lkGAQgASgDSABSCGJhY2t1cElkiAEBEj8KBnN0YXR1cxgFIAEoDjInLm9wZW5veGlk'
    'ZS5hcGkudjEuQmFja3VwRXhlY3V0aW9uU3RhdHVzUgZzdGF0dXMSIgoKb2JqZWN0X2tleRgGIA'
    'EoCUgBUglvYmplY3RLZXmIAQESLAoPY2hlY2tzdW1fc2hhMjU2GAcgASgJSAJSDmNoZWNrc3Vt'
    'U2hhMjU2iAEBEiIKCnNpemVfYnl0ZXMYCCABKANIA1IJc2l6ZUJ5dGVziAEBEhgKB2F0dGVtcH'
    'QYCSABKANSB2F0dGVtcHQSGQoFZXJyb3IYCiABKAlIBFIFZXJyb3KIAQESHQoKc3RhcnRlZF9h'
    'dBgLIAEoA1IJc3RhcnRlZEF0EiQKC2ZpbmlzaGVkX2F0GAwgASgDSAVSCmZpbmlzaGVkQXSIAQ'
    'FCDAoKX2JhY2t1cF9pZEINCgtfb2JqZWN0X2tleUISChBfY2hlY2tzdW1fc2hhMjU2Qg0KC19z'
    'aXplX2J5dGVzQggKBl9lcnJvckIOCgxfZmluaXNoZWRfYXQ=');

@$core.Deprecated('Use backupFileDescriptor instead')
const BackupFile$json = {
  '1': 'BackupFile',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'path', '3': 2, '4': 1, '5': 9, '10': 'path'},
    {'1': 'size_bytes', '3': 3, '4': 1, '5': 3, '10': 'sizeBytes'},
    {'1': 'modified_at', '3': 4, '4': 1, '5': 9, '10': 'modifiedAt'},
  ],
};

/// Descriptor for `BackupFile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupFileDescriptor = $convert.base64Decode(
    'CgpCYWNrdXBGaWxlEhIKBG5hbWUYASABKAlSBG5hbWUSEgoEcGF0aBgCIAEoCVIEcGF0aBIdCg'
    'pzaXplX2J5dGVzGAMgASgDUglzaXplQnl0ZXMSHwoLbW9kaWZpZWRfYXQYBCABKAlSCm1vZGlm'
    'aWVkQXQ=');

@$core.Deprecated('Use backupArchiveDescriptor instead')
const BackupArchive$json = {
  '1': 'BackupArchive',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'checksum_sha256', '3': 2, '4': 1, '5': 9, '10': 'checksumSha256'},
    {'1': 'size_bytes', '3': 3, '4': 1, '5': 3, '10': 'sizeBytes'},
    {'1': 'created_at', '3': 4, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `BackupArchive`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupArchiveDescriptor = $convert.base64Decode(
    'Cg1CYWNrdXBBcmNoaXZlEhIKBHBhdGgYASABKAlSBHBhdGgSJwoPY2hlY2tzdW1fc2hhMjU2GA'
    'IgASgJUg5jaGVja3N1bVNoYTI1NhIdCgpzaXplX2J5dGVzGAMgASgDUglzaXplQnl0ZXMSHQoK'
    'Y3JlYXRlZF9hdBgEIAEoA1IJY3JlYXRlZEF0');

@$core.Deprecated('Use backupStagedRestoreDescriptor instead')
const BackupStagedRestore$json = {
  '1': 'BackupStagedRestore',
  '2': [
    {'1': 'restore_id', '3': 1, '4': 1, '5': 9, '10': 'restoreId'},
    {'1': 'checksum_sha256', '3': 2, '4': 1, '5': 9, '10': 'checksumSha256'},
    {'1': 'restart_required', '3': 3, '4': 1, '5': 8, '10': 'restartRequired'},
    {'1': 'pending_marker', '3': 4, '4': 1, '5': 9, '10': 'pendingMarker'},
  ],
};

/// Descriptor for `BackupStagedRestore`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupStagedRestoreDescriptor = $convert.base64Decode(
    'ChNCYWNrdXBTdGFnZWRSZXN0b3JlEh0KCnJlc3RvcmVfaWQYASABKAlSCXJlc3RvcmVJZBInCg'
    '9jaGVja3N1bV9zaGEyNTYYAiABKAlSDmNoZWNrc3VtU2hhMjU2EikKEHJlc3RhcnRfcmVxdWly'
    'ZWQYAyABKAhSD3Jlc3RhcnRSZXF1aXJlZBIlCg5wZW5kaW5nX21hcmtlchgEIAEoCVINcGVuZG'
    'luZ01hcmtlcg==');

@$core.Deprecated('Use backupRestoreStatusDescriptor instead')
const BackupRestoreStatus$json = {
  '1': 'BackupRestoreStatus',
  '2': [
    {'1': 'restore_id', '3': 1, '4': 1, '5': 9, '10': 'restoreId'},
    {'1': 'status', '3': 2, '4': 1, '5': 9, '10': 'status'},
    {'1': 'message', '3': 3, '4': 1, '5': 9, '10': 'message'},
    {'1': 'updated_at', '3': 4, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `BackupRestoreStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRestoreStatusDescriptor = $convert.base64Decode(
    'ChNCYWNrdXBSZXN0b3JlU3RhdHVzEh0KCnJlc3RvcmVfaWQYASABKAlSCXJlc3RvcmVJZBIWCg'
    'ZzdGF0dXMYAiABKAlSBnN0YXR1cxIYCgdtZXNzYWdlGAMgASgJUgdtZXNzYWdlEh0KCnVwZGF0'
    'ZWRfYXQYBCABKANSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use backupRetentionPreviewDescriptor instead')
const BackupRetentionPreview$json = {
  '1': 'BackupRetentionPreview',
  '2': [
    {
      '1': 'keep',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupFile',
      '10': 'keep'
    },
    {
      '1': 'delete',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupFile',
      '10': 'delete'
    },
  ],
};

/// Descriptor for `BackupRetentionPreview`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRetentionPreviewDescriptor = $convert.base64Decode(
    'ChZCYWNrdXBSZXRlbnRpb25QcmV2aWV3EjAKBGtlZXAYASADKAsyHC5vcGVub3hpZGUuYXBpLn'
    'YxLkJhY2t1cEZpbGVSBGtlZXASNAoGZGVsZXRlGAIgAygLMhwub3Blbm94aWRlLmFwaS52MS5C'
    'YWNrdXBGaWxlUgZkZWxldGU=');

@$core.Deprecated('Use backupIntegrityDescriptor instead')
const BackupIntegrity$json = {
  '1': 'BackupIntegrity',
  '2': [
    {'1': 'valid', '3': 1, '4': 1, '5': 8, '10': 'valid'},
    {'1': 'expected_sha256', '3': 2, '4': 1, '5': 9, '10': 'expectedSha256'},
    {'1': 'actual_sha256', '3': 3, '4': 1, '5': 9, '10': 'actualSha256'},
    {'1': 'size_bytes', '3': 4, '4': 1, '5': 3, '10': 'sizeBytes'},
  ],
};

/// Descriptor for `BackupIntegrity`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupIntegrityDescriptor = $convert.base64Decode(
    'Cg9CYWNrdXBJbnRlZ3JpdHkSFAoFdmFsaWQYASABKAhSBXZhbGlkEicKD2V4cGVjdGVkX3NoYT'
    'I1NhgCIAEoCVIOZXhwZWN0ZWRTaGEyNTYSIwoNYWN0dWFsX3NoYTI1NhgDIAEoCVIMYWN0dWFs'
    'U2hhMjU2Eh0KCnNpemVfYnl0ZXMYBCABKANSCXNpemVCeXRlcw==');

@$core.Deprecated('Use backupListBackupsRequestDescriptor instead')
const BackupListBackupsRequest$json = {
  '1': 'BackupListBackupsRequest',
};

/// Descriptor for `BackupListBackupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListBackupsRequestDescriptor =
    $convert.base64Decode('ChhCYWNrdXBMaXN0QmFja3Vwc1JlcXVlc3Q=');

@$core.Deprecated('Use backupListBackupsResponseDescriptor instead')
const BackupListBackupsResponse$json = {
  '1': 'BackupListBackupsResponse',
  '2': [
    {
      '1': 'backups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backups'
    },
  ],
};

/// Descriptor for `BackupListBackupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListBackupsResponseDescriptor =
    $convert.base64Decode(
        'ChlCYWNrdXBMaXN0QmFja3Vwc1Jlc3BvbnNlEjoKB2JhY2t1cHMYASADKAsyIC5vcGVub3hpZG'
        'UuYXBpLnYxLkJhY2t1cERhdGFiYXNlUgdiYWNrdXBz');

@$core.Deprecated('Use backupGetBackupRequestDescriptor instead')
const BackupGetBackupRequest$json = {
  '1': 'BackupGetBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupGetBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetBackupRequestDescriptor =
    $convert.base64Decode(
        'ChZCYWNrdXBHZXRCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use backupGetBackupResponseDescriptor instead')
const BackupGetBackupResponse$json = {
  '1': 'BackupGetBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupGetBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetBackupResponseDescriptor =
    $convert.base64Decode(
        'ChdCYWNrdXBHZXRCYWNrdXBSZXNwb25zZRI4CgZiYWNrdXAYASABKAsyIC5vcGVub3hpZGUuYX'
        'BpLnYxLkJhY2t1cERhdGFiYXNlUgZiYWNrdXA=');

@$core.Deprecated('Use backupCreateBackupRequestDescriptor instead')
const BackupCreateBackupRequest$json = {
  '1': 'BackupCreateBackupRequest',
  '2': [
    {'1': 'app_name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'appName'},
    {'1': 'schedule', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'schedule'},
    {
      '1': 'database_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'databaseName'
    },
    {'1': 'prefix', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'prefix'},
    {
      '1': 'service_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 6,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'keepLatestCount',
      '17': true
    },
    {'1': 'backup_type', '3': 7, '4': 1, '5': 9, '8': {}, '10': 'backupType'},
    {
      '1': 'database_type',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'databaseType'
    },
    {
      '1': 'metadata',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'service_id',
      '3': 10,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {
      '1': 'organization_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
  '8': [
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_metadata'},
    {'1': '_service_id'},
  ],
};

/// Descriptor for `BackupCreateBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateBackupRequestDescriptor = $convert.base64Decode(
    'ChlCYWNrdXBDcmVhdGVCYWNrdXBSZXF1ZXN0EiUKCGFwcF9uYW1lGAEgASgJQgq6SAdyBRABGP'
    '8BUgdhcHBOYW1lEiMKCHNjaGVkdWxlGAIgASgJQge6SARyAhABUghzY2hlZHVsZRIsCg1kYXRh'
    'YmFzZV9uYW1lGAMgASgJQge6SARyAhABUgxkYXRhYmFzZU5hbWUSHwoGcHJlZml4GAQgASgJQg'
    'e6SARyAhABUgZwcmVmaXgSMAoMc2VydmljZV9uYW1lGAUgASgJQgi6SAVyAxj/AUgAUgtzZXJ2'
    'aWNlTmFtZYgBARI4ChFrZWVwX2xhdGVzdF9jb3VudBgGIAEoA0IHukgEIgIoAEgBUg9rZWVwTG'
    'F0ZXN0Q291bnSIAQESKAoLYmFja3VwX3R5cGUYByABKAlCB7pIBHICEAFSCmJhY2t1cFR5cGUS'
    'LAoNZGF0YWJhc2VfdHlwZRgIIAEoCUIHukgEcgIQAVIMZGF0YWJhc2VUeXBlEioKCG1ldGFkYX'
    'RhGAkgASgJQgm6SAZyBBiAgEBIAlIIbWV0YWRhdGGIAQESKwoKc2VydmljZV9pZBgKIAEoA0IH'
    'ukgEIgIgAEgDUglzZXJ2aWNlSWSIAQESLgoOZGVzdGluYXRpb25faWQYCyABKANCB7pIBCICIA'
    'BSDWRlc3RpbmF0aW9uSWQSMAoPb3JnYW5pemF0aW9uX2lkGAwgASgDQge6SAQiAiAAUg5vcmdh'
    'bml6YXRpb25JZEIPCg1fc2VydmljZV9uYW1lQhQKEl9rZWVwX2xhdGVzdF9jb3VudEILCglfbW'
    'V0YWRhdGFCDQoLX3NlcnZpY2VfaWQ=');

@$core.Deprecated('Use backupCreateBackupResponseDescriptor instead')
const BackupCreateBackupResponse$json = {
  '1': 'BackupCreateBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupCreateBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateBackupResponseDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBDcmVhdGVCYWNrdXBSZXNwb25zZRI4CgZiYWNrdXAYASABKAsyIC5vcGVub3hpZG'
        'UuYXBpLnYxLkJhY2t1cERhdGFiYXNlUgZiYWNrdXA=');

@$core.Deprecated('Use backupUpdateBackupRequestDescriptor instead')
const BackupUpdateBackupRequest$json = {
  '1': 'BackupUpdateBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'app_name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'appName',
      '17': true
    },
    {
      '1': 'schedule',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'schedule',
      '17': true
    },
    {
      '1': 'database_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'databaseName',
      '17': true
    },
    {'1': 'prefix', '3': 5, '4': 1, '5': 9, '9': 3, '10': 'prefix', '17': true},
    {
      '1': 'service_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'keepLatestCount',
      '17': true
    },
    {
      '1': 'backup_type',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'backupType',
      '17': true
    },
    {
      '1': 'database_type',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'databaseType',
      '17': true
    },
    {
      '1': 'metadata',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 8,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 9,
      '10': 'destinationId',
      '17': true
    },
    {
      '1': 'enabled',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 10,
      '10': 'enabled',
      '17': true
    },
  ],
  '8': [
    {'1': '_app_name'},
    {'1': '_schedule'},
    {'1': '_database_name'},
    {'1': '_prefix'},
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_backup_type'},
    {'1': '_database_type'},
    {'1': '_metadata'},
    {'1': '_destination_id'},
    {'1': '_enabled'},
  ],
};

/// Descriptor for `BackupUpdateBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateBackupRequestDescriptor = $convert.base64Decode(
    'ChlCYWNrdXBVcGRhdGVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIoCg'
    'hhcHBfbmFtZRgCIAEoCUIIukgFcgMY/wFIAFIHYXBwTmFtZYgBARIfCghzY2hlZHVsZRgDIAEo'
    'CUgBUghzY2hlZHVsZYgBARIoCg1kYXRhYmFzZV9uYW1lGAQgASgJSAJSDGRhdGFiYXNlTmFtZY'
    'gBARIbCgZwcmVmaXgYBSABKAlIA1IGcHJlZml4iAEBEjAKDHNlcnZpY2VfbmFtZRgGIAEoCUII'
    'ukgFcgMY/wFIBFILc2VydmljZU5hbWWIAQESOAoRa2VlcF9sYXRlc3RfY291bnQYByABKANCB7'
    'pIBCICKABIBVIPa2VlcExhdGVzdENvdW50iAEBEiQKC2JhY2t1cF90eXBlGAggASgJSAZSCmJh'
    'Y2t1cFR5cGWIAQESKAoNZGF0YWJhc2VfdHlwZRgJIAEoCUgHUgxkYXRhYmFzZVR5cGWIAQESKg'
    'oIbWV0YWRhdGEYCiABKAlCCbpIBnIEGICAQEgIUghtZXRhZGF0YYgBARIzCg5kZXN0aW5hdGlv'
    'bl9pZBgLIAEoA0IHukgEIgIgAEgJUg1kZXN0aW5hdGlvbklkiAEBEh0KB2VuYWJsZWQYDCABKA'
    'hIClIHZW5hYmxlZIgBAUILCglfYXBwX25hbWVCCwoJX3NjaGVkdWxlQhAKDl9kYXRhYmFzZV9u'
    'YW1lQgkKB19wcmVmaXhCDwoNX3NlcnZpY2VfbmFtZUIUChJfa2VlcF9sYXRlc3RfY291bnRCDg'
    'oMX2JhY2t1cF90eXBlQhAKDl9kYXRhYmFzZV90eXBlQgsKCV9tZXRhZGF0YUIRCg9fZGVzdGlu'
    'YXRpb25faWRCCgoIX2VuYWJsZWQ=');

@$core.Deprecated('Use backupUpdateBackupResponseDescriptor instead')
const BackupUpdateBackupResponse$json = {
  '1': 'BackupUpdateBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupUpdateBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateBackupResponseDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBVcGRhdGVCYWNrdXBSZXNwb25zZRI4CgZiYWNrdXAYASABKAsyIC5vcGVub3hpZG'
        'UuYXBpLnYxLkJhY2t1cERhdGFiYXNlUgZiYWNrdXA=');

@$core.Deprecated('Use backupDeleteBackupRequestDescriptor instead')
const BackupDeleteBackupRequest$json = {
  '1': 'BackupDeleteBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupDeleteBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDeleteBackupRequestDescriptor =
    $convert.base64Decode(
        'ChlCYWNrdXBEZWxldGVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use backupRunBackupRequestDescriptor instead')
const BackupRunBackupRequest$json = {
  '1': 'BackupRunBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupRunBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunBackupRequestDescriptor =
    $convert.base64Decode(
        'ChZCYWNrdXBSdW5CYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use backupRestoreBackupRequestDescriptor instead')
const BackupRestoreBackupRequest$json = {
  '1': 'BackupRestoreBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'backup_file', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'backupFile'},
  ],
};

/// Descriptor for `BackupRestoreBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRestoreBackupRequestDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBSZXN0b3JlQmFja3VwUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSKA'
        'oLYmFja3VwX2ZpbGUYAiABKAlCB7pIBHICEAFSCmJhY2t1cEZpbGU=');

@$core.Deprecated('Use backupValidateDestinationRequestDescriptor instead')
const BackupValidateDestinationRequest$json = {
  '1': 'BackupValidateDestinationRequest',
  '2': [
    {
      '1': 'destination_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
  ],
};

/// Descriptor for `BackupValidateDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupValidateDestinationRequestDescriptor =
    $convert.base64Decode(
        'CiBCYWNrdXBWYWxpZGF0ZURlc3RpbmF0aW9uUmVxdWVzdBIuCg5kZXN0aW5hdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINZGVzdGluYXRpb25JZA==');

@$core.Deprecated('Use backupValidateDestinationResponseDescriptor instead')
const BackupValidateDestinationResponse$json = {
  '1': 'BackupValidateDestinationResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `BackupValidateDestinationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupValidateDestinationResponseDescriptor =
    $convert.base64Decode(
        'CiFCYWNrdXBWYWxpZGF0ZURlc3RpbmF0aW9uUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3'
        'VjY2VzcxIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdl');

@$core.Deprecated('Use backupRunComposeBackupRequestDescriptor instead')
const BackupRunComposeBackupRequest$json = {
  '1': 'BackupRunComposeBackupRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {'1': 'include_secrets', '3': 2, '4': 1, '5': 8, '10': 'includeSecrets'},
  ],
};

/// Descriptor for `BackupRunComposeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunComposeBackupRequestDescriptor =
    $convert.base64Decode(
        'Ch1CYWNrdXBSdW5Db21wb3NlQmFja3VwUmVxdWVzdBImCgpjb21wb3NlX2lkGAEgASgDQge6SA'
        'QiAiAAUgljb21wb3NlSWQSJwoPaW5jbHVkZV9zZWNyZXRzGAIgASgIUg5pbmNsdWRlU2VjcmV0'
        'cw==');

@$core.Deprecated('Use backupRunComposeBackupResponseDescriptor instead')
const BackupRunComposeBackupResponse$json = {
  '1': 'BackupRunComposeBackupResponse',
  '2': [
    {
      '1': 'archive',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupArchive',
      '10': 'archive'
    },
  ],
};

/// Descriptor for `BackupRunComposeBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunComposeBackupResponseDescriptor =
    $convert.base64Decode(
        'Ch5CYWNrdXBSdW5Db21wb3NlQmFja3VwUmVzcG9uc2USOQoHYXJjaGl2ZRgBIAEoCzIfLm9wZW'
        '5veGlkZS5hcGkudjEuQmFja3VwQXJjaGl2ZVIHYXJjaGl2ZQ==');

@$core.Deprecated('Use backupRunPanelBackupRequestDescriptor instead')
const BackupRunPanelBackupRequest$json = {
  '1': 'BackupRunPanelBackupRequest',
};

/// Descriptor for `BackupRunPanelBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunPanelBackupRequestDescriptor =
    $convert.base64Decode('ChtCYWNrdXBSdW5QYW5lbEJhY2t1cFJlcXVlc3Q=');

@$core.Deprecated('Use backupRunPanelBackupResponseDescriptor instead')
const BackupRunPanelBackupResponse$json = {
  '1': 'BackupRunPanelBackupResponse',
  '2': [
    {
      '1': 'archive',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupArchive',
      '10': 'archive'
    },
  ],
};

/// Descriptor for `BackupRunPanelBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunPanelBackupResponseDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBSdW5QYW5lbEJhY2t1cFJlc3BvbnNlEjkKB2FyY2hpdmUYASABKAsyHy5vcGVub3'
        'hpZGUuYXBpLnYxLkJhY2t1cEFyY2hpdmVSB2FyY2hpdmU=');

@$core.Deprecated('Use backupStageRestoreRequestDescriptor instead')
const BackupStageRestoreRequest$json = {
  '1': 'BackupStageRestoreRequest',
  '2': [
    {'1': 'archive', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'archive'},
    {
      '1': 'checksum_sha256',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'checksumSha256',
      '17': true
    },
  ],
  '8': [
    {'1': '_checksum_sha256'},
  ],
};

/// Descriptor for `BackupStageRestoreRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupStageRestoreRequestDescriptor = $convert.base64Decode(
    'ChlCYWNrdXBTdGFnZVJlc3RvcmVSZXF1ZXN0EiEKB2FyY2hpdmUYASABKAlCB7pIBHICEAFSB2'
    'FyY2hpdmUSLAoPY2hlY2tzdW1fc2hhMjU2GAIgASgJSABSDmNoZWNrc3VtU2hhMjU2iAEBQhIK'
    'EF9jaGVja3N1bV9zaGEyNTY=');

@$core.Deprecated('Use backupStageRestoreResponseDescriptor instead')
const BackupStageRestoreResponse$json = {
  '1': 'BackupStageRestoreResponse',
  '2': [
    {
      '1': 'staged',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupStagedRestore',
      '10': 'staged'
    },
  ],
};

/// Descriptor for `BackupStageRestoreResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupStageRestoreResponseDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBTdGFnZVJlc3RvcmVSZXNwb25zZRI9CgZzdGFnZWQYASABKAsyJS5vcGVub3hpZG'
        'UuYXBpLnYxLkJhY2t1cFN0YWdlZFJlc3RvcmVSBnN0YWdlZA==');

@$core.Deprecated('Use backupGetRestoreStatusRequestDescriptor instead')
const BackupGetRestoreStatusRequest$json = {
  '1': 'BackupGetRestoreStatusRequest',
  '2': [
    {'1': 'restore_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'restoreId'},
  ],
};

/// Descriptor for `BackupGetRestoreStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetRestoreStatusRequestDescriptor =
    $convert.base64Decode(
        'Ch1CYWNrdXBHZXRSZXN0b3JlU3RhdHVzUmVxdWVzdBImCgpyZXN0b3JlX2lkGAEgASgJQge6SA'
        'RyAhABUglyZXN0b3JlSWQ=');

@$core.Deprecated('Use backupGetRestoreStatusResponseDescriptor instead')
const BackupGetRestoreStatusResponse$json = {
  '1': 'BackupGetRestoreStatusResponse',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupRestoreStatus',
      '10': 'status'
    },
  ],
};

/// Descriptor for `BackupGetRestoreStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetRestoreStatusResponseDescriptor =
    $convert.base64Decode(
        'Ch5CYWNrdXBHZXRSZXN0b3JlU3RhdHVzUmVzcG9uc2USPQoGc3RhdHVzGAEgASgLMiUub3Blbm'
        '94aWRlLmFwaS52MS5CYWNrdXBSZXN0b3JlU3RhdHVzUgZzdGF0dXM=');

@$core.Deprecated('Use backupRollbackRestoreRequestDescriptor instead')
const BackupRollbackRestoreRequest$json = {
  '1': 'BackupRollbackRestoreRequest',
  '2': [
    {
      '1': 'recovery_database',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'recoveryDatabase'
    },
  ],
};

/// Descriptor for `BackupRollbackRestoreRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRollbackRestoreRequestDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBSb2xsYmFja1Jlc3RvcmVSZXF1ZXN0EjQKEXJlY292ZXJ5X2RhdGFiYXNlGAEgAS'
        'gJQge6SARyAhABUhByZWNvdmVyeURhdGFiYXNl');

@$core.Deprecated('Use backupRollbackRestoreResponseDescriptor instead')
const BackupRollbackRestoreResponse$json = {
  '1': 'BackupRollbackRestoreResponse',
  '2': [
    {
      '1': 'staged',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupStagedRestore',
      '10': 'staged'
    },
  ],
};

/// Descriptor for `BackupRollbackRestoreResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRollbackRestoreResponseDescriptor =
    $convert.base64Decode(
        'Ch1CYWNrdXBSb2xsYmFja1Jlc3RvcmVSZXNwb25zZRI9CgZzdGFnZWQYASABKAsyJS5vcGVub3'
        'hpZGUuYXBpLnYxLkJhY2t1cFN0YWdlZFJlc3RvcmVSBnN0YWdlZA==');

@$core.Deprecated('Use backupListFilesRequestDescriptor instead')
const BackupListFilesRequest$json = {
  '1': 'BackupListFilesRequest',
  '2': [
    {
      '1': 'destination_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {
      '1': 'prefix',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'prefix',
      '17': true
    },
  ],
  '8': [
    {'1': '_prefix'},
  ],
};

/// Descriptor for `BackupListFilesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListFilesRequestDescriptor = $convert.base64Decode(
    'ChZCYWNrdXBMaXN0RmlsZXNSZXF1ZXN0Ei4KDmRlc3RpbmF0aW9uX2lkGAEgASgDQge6SAQiAi'
    'AAUg1kZXN0aW5hdGlvbklkEiUKBnByZWZpeBgCIAEoCUIIukgFcgMYgAhIAFIGcHJlZml4iAEB'
    'QgkKB19wcmVmaXg=');

@$core.Deprecated('Use backupListFilesResponseDescriptor instead')
const BackupListFilesResponse$json = {
  '1': 'BackupListFilesResponse',
  '2': [
    {
      '1': 'files',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupFile',
      '10': 'files'
    },
  ],
};

/// Descriptor for `BackupListFilesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListFilesResponseDescriptor =
    $convert.base64Decode(
        'ChdCYWNrdXBMaXN0RmlsZXNSZXNwb25zZRIyCgVmaWxlcxgBIAMoCzIcLm9wZW5veGlkZS5hcG'
        'kudjEuQmFja3VwRmlsZVIFZmlsZXM=');

@$core.Deprecated('Use backupDownloadFileRequestDescriptor instead')
const BackupDownloadFileRequest$json = {
  '1': 'BackupDownloadFileRequest',
  '2': [
    {
      '1': 'destination_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'objectKey'},
  ],
};

/// Descriptor for `BackupDownloadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDownloadFileRequestDescriptor = $convert.base64Decode(
    'ChlCYWNrdXBEb3dubG9hZEZpbGVSZXF1ZXN0Ei4KDmRlc3RpbmF0aW9uX2lkGAEgASgDQge6SA'
    'QiAiAAUg1kZXN0aW5hdGlvbklkEiYKCm9iamVjdF9rZXkYAiABKAlCB7pIBHICEAFSCW9iamVj'
    'dEtleQ==');

@$core.Deprecated('Use backupDownloadFileResponseDescriptor instead')
const BackupDownloadFileResponse$json = {
  '1': 'BackupDownloadFileResponse',
  '2': [
    {'1': 'size_bytes', '3': 1, '4': 1, '5': 3, '10': 'sizeBytes'},
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '10': 'objectKey'},
  ],
};

/// Descriptor for `BackupDownloadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDownloadFileResponseDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBEb3dubG9hZEZpbGVSZXNwb25zZRIdCgpzaXplX2J5dGVzGAEgASgDUglzaXplQn'
        'l0ZXMSHQoKb2JqZWN0X2tleRgCIAEoCVIJb2JqZWN0S2V5');

@$core.Deprecated('Use backupPreviewRetentionRequestDescriptor instead')
const BackupPreviewRetentionRequest$json = {
  '1': 'BackupPreviewRetentionRequest',
  '2': [
    {
      '1': 'destination_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {
      '1': 'prefix',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'prefix',
      '17': true
    },
    {'1': 'keep_latest', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'keepLatest'},
  ],
  '8': [
    {'1': '_prefix'},
  ],
};

/// Descriptor for `BackupPreviewRetentionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupPreviewRetentionRequestDescriptor = $convert.base64Decode(
    'Ch1CYWNrdXBQcmV2aWV3UmV0ZW50aW9uUmVxdWVzdBIuCg5kZXN0aW5hdGlvbl9pZBgBIAEoA0'
    'IHukgEIgIgAFINZGVzdGluYXRpb25JZBIlCgZwcmVmaXgYAiABKAlCCLpIBXIDGIAISABSBnBy'
    'ZWZpeIgBARIoCgtrZWVwX2xhdGVzdBgDIAEoA0IHukgEIgIoAVIKa2VlcExhdGVzdEIJCgdfcH'
    'JlZml4');

@$core.Deprecated('Use backupPreviewRetentionResponseDescriptor instead')
const BackupPreviewRetentionResponse$json = {
  '1': 'BackupPreviewRetentionResponse',
  '2': [
    {
      '1': 'preview',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupRetentionPreview',
      '10': 'preview'
    },
  ],
};

/// Descriptor for `BackupPreviewRetentionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupPreviewRetentionResponseDescriptor =
    $convert.base64Decode(
        'Ch5CYWNrdXBQcmV2aWV3UmV0ZW50aW9uUmVzcG9uc2USQgoHcHJldmlldxgBIAEoCzIoLm9wZW'
        '5veGlkZS5hcGkudjEuQmFja3VwUmV0ZW50aW9uUHJldmlld1IHcHJldmlldw==');

@$core.Deprecated('Use backupVerifyFilesRequestDescriptor instead')
const BackupVerifyFilesRequest$json = {
  '1': 'BackupVerifyFilesRequest',
  '2': [
    {
      '1': 'destination_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'objectKey'},
    {
      '1': 'checksum_sha256',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'checksumSha256'
    },
  ],
};

/// Descriptor for `BackupVerifyFilesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupVerifyFilesRequestDescriptor = $convert.base64Decode(
    'ChhCYWNrdXBWZXJpZnlGaWxlc1JlcXVlc3QSLgoOZGVzdGluYXRpb25faWQYASABKANCB7pIBC'
    'ICIABSDWRlc3RpbmF0aW9uSWQSJgoKb2JqZWN0X2tleRgCIAEoCUIHukgEcgIQAVIJb2JqZWN0'
    'S2V5EjAKD2NoZWNrc3VtX3NoYTI1NhgDIAEoCUIHukgEcgIQAVIOY2hlY2tzdW1TaGEyNTY=');

@$core.Deprecated('Use backupVerifyFilesResponseDescriptor instead')
const BackupVerifyFilesResponse$json = {
  '1': 'BackupVerifyFilesResponse',
  '2': [
    {
      '1': 'integrity',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupIntegrity',
      '10': 'integrity'
    },
  ],
};

/// Descriptor for `BackupVerifyFilesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupVerifyFilesResponseDescriptor =
    $convert.base64Decode(
        'ChlCYWNrdXBWZXJpZnlGaWxlc1Jlc3BvbnNlEj8KCWludGVncml0eRgBIAEoCzIhLm9wZW5veG'
        'lkZS5hcGkudjEuQmFja3VwSW50ZWdyaXR5UglpbnRlZ3JpdHk=');

@$core.Deprecated('Use backupListExecutionsRequestDescriptor instead')
const BackupListExecutionsRequest$json = {
  '1': 'BackupListExecutionsRequest',
  '2': [
    {
      '1': 'limit',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'limit',
      '17': true
    },
  ],
  '8': [
    {'1': '_limit'},
  ],
};

/// Descriptor for `BackupListExecutionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListExecutionsRequestDescriptor =
    $convert.base64Decode(
        'ChtCYWNrdXBMaXN0RXhlY3V0aW9uc1JlcXVlc3QSJQoFbGltaXQYASABKANCCrpIByIFGPQDKA'
        'FIAFIFbGltaXSIAQFCCAoGX2xpbWl0');

@$core.Deprecated('Use backupListExecutionsResponseDescriptor instead')
const BackupListExecutionsResponse$json = {
  '1': 'BackupListExecutionsResponse',
  '2': [
    {
      '1': 'executions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupExecution',
      '10': 'executions'
    },
  ],
};

/// Descriptor for `BackupListExecutionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListExecutionsResponseDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBMaXN0RXhlY3V0aW9uc1Jlc3BvbnNlEkEKCmV4ZWN1dGlvbnMYASADKAsyIS5vcG'
        'Vub3hpZGUuYXBpLnYxLkJhY2t1cEV4ZWN1dGlvblIKZXhlY3V0aW9ucw==');

@$core.Deprecated('Use backupRetryExecutionRequestDescriptor instead')
const BackupRetryExecutionRequest$json = {
  '1': 'BackupRetryExecutionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupRetryExecutionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRetryExecutionRequestDescriptor =
    $convert.base64Decode(
        'ChtCYWNrdXBSZXRyeUV4ZWN1dGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use backupListDatabaseBackupsRequestDescriptor instead')
const BackupListDatabaseBackupsRequest$json = {
  '1': 'BackupListDatabaseBackupsRequest',
};

/// Descriptor for `BackupListDatabaseBackupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListDatabaseBackupsRequestDescriptor =
    $convert.base64Decode('CiBCYWNrdXBMaXN0RGF0YWJhc2VCYWNrdXBzUmVxdWVzdA==');

@$core.Deprecated('Use backupListDatabaseBackupsResponseDescriptor instead')
const BackupListDatabaseBackupsResponse$json = {
  '1': 'BackupListDatabaseBackupsResponse',
  '2': [
    {
      '1': 'backups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backups'
    },
  ],
};

/// Descriptor for `BackupListDatabaseBackupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListDatabaseBackupsResponseDescriptor =
    $convert.base64Decode(
        'CiFCYWNrdXBMaXN0RGF0YWJhc2VCYWNrdXBzUmVzcG9uc2USOgoHYmFja3VwcxgBIAMoCzIgLm'
        '9wZW5veGlkZS5hcGkudjEuQmFja3VwRGF0YWJhc2VSB2JhY2t1cHM=');

@$core.Deprecated('Use backupGetDatabaseBackupRequestDescriptor instead')
const BackupGetDatabaseBackupRequest$json = {
  '1': 'BackupGetDatabaseBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupGetDatabaseBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetDatabaseBackupRequestDescriptor =
    $convert.base64Decode(
        'Ch5CYWNrdXBHZXREYXRhYmFzZUJhY2t1cFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
        'lk');

@$core.Deprecated('Use backupGetDatabaseBackupResponseDescriptor instead')
const BackupGetDatabaseBackupResponse$json = {
  '1': 'BackupGetDatabaseBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupGetDatabaseBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetDatabaseBackupResponseDescriptor =
    $convert.base64Decode(
        'Ch9CYWNrdXBHZXREYXRhYmFzZUJhY2t1cFJlc3BvbnNlEjgKBmJhY2t1cBgBIAEoCzIgLm9wZW'
        '5veGlkZS5hcGkudjEuQmFja3VwRGF0YWJhc2VSBmJhY2t1cA==');

@$core.Deprecated('Use backupCreateDatabaseBackupRequestDescriptor instead')
const BackupCreateDatabaseBackupRequest$json = {
  '1': 'BackupCreateDatabaseBackupRequest',
  '2': [
    {'1': 'app_name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'appName'},
    {'1': 'schedule', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'schedule'},
    {
      '1': 'database_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'databaseName'
    },
    {'1': 'prefix', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'prefix'},
    {
      '1': 'service_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 6,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'keepLatestCount',
      '17': true
    },
    {'1': 'backup_type', '3': 7, '4': 1, '5': 9, '8': {}, '10': 'backupType'},
    {
      '1': 'database_type',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'databaseType'
    },
    {
      '1': 'metadata',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'service_id',
      '3': 10,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'destinationId'
    },
    {
      '1': 'organization_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
  '8': [
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_metadata'},
    {'1': '_service_id'},
  ],
};

/// Descriptor for `BackupCreateDatabaseBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateDatabaseBackupRequestDescriptor = $convert.base64Decode(
    'CiFCYWNrdXBDcmVhdGVEYXRhYmFzZUJhY2t1cFJlcXVlc3QSJQoIYXBwX25hbWUYASABKAlCCr'
    'pIB3IFEAEY/wFSB2FwcE5hbWUSIwoIc2NoZWR1bGUYAiABKAlCB7pIBHICEAFSCHNjaGVkdWxl'
    'EiwKDWRhdGFiYXNlX25hbWUYAyABKAlCB7pIBHICEAFSDGRhdGFiYXNlTmFtZRIfCgZwcmVmaX'
    'gYBCABKAlCB7pIBHICEAFSBnByZWZpeBIwCgxzZXJ2aWNlX25hbWUYBSABKAlCCLpIBXIDGP8B'
    'SABSC3NlcnZpY2VOYW1liAEBEjgKEWtlZXBfbGF0ZXN0X2NvdW50GAYgASgDQge6SAQiAigASA'
    'FSD2tlZXBMYXRlc3RDb3VudIgBARIoCgtiYWNrdXBfdHlwZRgHIAEoCUIHukgEcgIQAVIKYmFj'
    'a3VwVHlwZRIsCg1kYXRhYmFzZV90eXBlGAggASgJQge6SARyAhABUgxkYXRhYmFzZVR5cGUSKg'
    'oIbWV0YWRhdGEYCSABKAlCCbpIBnIEGICAQEgCUghtZXRhZGF0YYgBARIrCgpzZXJ2aWNlX2lk'
    'GAogASgDQge6SAQiAiAASANSCXNlcnZpY2VJZIgBARIuCg5kZXN0aW5hdGlvbl9pZBgLIAEoA0'
    'IHukgEIgIgAFINZGVzdGluYXRpb25JZBIwCg9vcmdhbml6YXRpb25faWQYDCABKANCB7pIBCIC'
    'IABSDm9yZ2FuaXphdGlvbklkQg8KDV9zZXJ2aWNlX25hbWVCFAoSX2tlZXBfbGF0ZXN0X2NvdW'
    '50QgsKCV9tZXRhZGF0YUINCgtfc2VydmljZV9pZA==');

@$core.Deprecated('Use backupCreateDatabaseBackupResponseDescriptor instead')
const BackupCreateDatabaseBackupResponse$json = {
  '1': 'BackupCreateDatabaseBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupCreateDatabaseBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateDatabaseBackupResponseDescriptor =
    $convert.base64Decode(
        'CiJCYWNrdXBDcmVhdGVEYXRhYmFzZUJhY2t1cFJlc3BvbnNlEjgKBmJhY2t1cBgBIAEoCzIgLm'
        '9wZW5veGlkZS5hcGkudjEuQmFja3VwRGF0YWJhc2VSBmJhY2t1cA==');

@$core.Deprecated('Use backupUpdateDatabaseBackupRequestDescriptor instead')
const BackupUpdateDatabaseBackupRequest$json = {
  '1': 'BackupUpdateDatabaseBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'app_name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'appName',
      '17': true
    },
    {
      '1': 'schedule',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'schedule',
      '17': true
    },
    {
      '1': 'database_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'databaseName',
      '17': true
    },
    {'1': 'prefix', '3': 5, '4': 1, '5': 9, '9': 3, '10': 'prefix', '17': true},
    {
      '1': 'service_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'keepLatestCount',
      '17': true
    },
    {
      '1': 'backup_type',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'backupType',
      '17': true
    },
    {
      '1': 'database_type',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'databaseType',
      '17': true
    },
    {
      '1': 'metadata',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 8,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 9,
      '10': 'destinationId',
      '17': true
    },
    {
      '1': 'enabled',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 10,
      '10': 'enabled',
      '17': true
    },
  ],
  '8': [
    {'1': '_app_name'},
    {'1': '_schedule'},
    {'1': '_database_name'},
    {'1': '_prefix'},
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_backup_type'},
    {'1': '_database_type'},
    {'1': '_metadata'},
    {'1': '_destination_id'},
    {'1': '_enabled'},
  ],
};

/// Descriptor for `BackupUpdateDatabaseBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateDatabaseBackupRequestDescriptor = $convert.base64Decode(
    'CiFCYWNrdXBVcGRhdGVEYXRhYmFzZUJhY2t1cFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
    'BSAmlkEigKCGFwcF9uYW1lGAIgASgJQgi6SAVyAxj/AUgAUgdhcHBOYW1liAEBEh8KCHNjaGVk'
    'dWxlGAMgASgJSAFSCHNjaGVkdWxliAEBEigKDWRhdGFiYXNlX25hbWUYBCABKAlIAlIMZGF0YW'
    'Jhc2VOYW1liAEBEhsKBnByZWZpeBgFIAEoCUgDUgZwcmVmaXiIAQESMAoMc2VydmljZV9uYW1l'
    'GAYgASgJQgi6SAVyAxj/AUgEUgtzZXJ2aWNlTmFtZYgBARI4ChFrZWVwX2xhdGVzdF9jb3VudB'
    'gHIAEoA0IHukgEIgIoAEgFUg9rZWVwTGF0ZXN0Q291bnSIAQESJAoLYmFja3VwX3R5cGUYCCAB'
    'KAlIBlIKYmFja3VwVHlwZYgBARIoCg1kYXRhYmFzZV90eXBlGAkgASgJSAdSDGRhdGFiYXNlVH'
    'lwZYgBARIqCghtZXRhZGF0YRgKIAEoCUIJukgGcgQYgIBASAhSCG1ldGFkYXRhiAEBEjMKDmRl'
    'c3RpbmF0aW9uX2lkGAsgASgDQge6SAQiAiAASAlSDWRlc3RpbmF0aW9uSWSIAQESHQoHZW5hYm'
    'xlZBgMIAEoCEgKUgdlbmFibGVkiAEBQgsKCV9hcHBfbmFtZUILCglfc2NoZWR1bGVCEAoOX2Rh'
    'dGFiYXNlX25hbWVCCQoHX3ByZWZpeEIPCg1fc2VydmljZV9uYW1lQhQKEl9rZWVwX2xhdGVzdF'
    '9jb3VudEIOCgxfYmFja3VwX3R5cGVCEAoOX2RhdGFiYXNlX3R5cGVCCwoJX21ldGFkYXRhQhEK'
    'D19kZXN0aW5hdGlvbl9pZEIKCghfZW5hYmxlZA==');

@$core.Deprecated('Use backupUpdateDatabaseBackupResponseDescriptor instead')
const BackupUpdateDatabaseBackupResponse$json = {
  '1': 'BackupUpdateDatabaseBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupDatabase',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupUpdateDatabaseBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateDatabaseBackupResponseDescriptor =
    $convert.base64Decode(
        'CiJCYWNrdXBVcGRhdGVEYXRhYmFzZUJhY2t1cFJlc3BvbnNlEjgKBmJhY2t1cBgBIAEoCzIgLm'
        '9wZW5veGlkZS5hcGkudjEuQmFja3VwRGF0YWJhc2VSBmJhY2t1cA==');

@$core.Deprecated('Use backupDeleteDatabaseBackupRequestDescriptor instead')
const BackupDeleteDatabaseBackupRequest$json = {
  '1': 'BackupDeleteDatabaseBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupDeleteDatabaseBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDeleteDatabaseBackupRequestDescriptor =
    $convert.base64Decode(
        'CiFCYWNrdXBEZWxldGVEYXRhYmFzZUJhY2t1cFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use backupRunDatabaseBackupRequestDescriptor instead')
const BackupRunDatabaseBackupRequest$json = {
  '1': 'BackupRunDatabaseBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupRunDatabaseBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunDatabaseBackupRequestDescriptor =
    $convert.base64Decode(
        'Ch5CYWNrdXBSdW5EYXRhYmFzZUJhY2t1cFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
        'lk');

@$core.Deprecated('Use backupListVolumeBackupsRequestDescriptor instead')
const BackupListVolumeBackupsRequest$json = {
  '1': 'BackupListVolumeBackupsRequest',
};

/// Descriptor for `BackupListVolumeBackupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListVolumeBackupsRequestDescriptor =
    $convert.base64Decode('Ch5CYWNrdXBMaXN0Vm9sdW1lQmFja3Vwc1JlcXVlc3Q=');

@$core.Deprecated('Use backupListVolumeBackupsResponseDescriptor instead')
const BackupListVolumeBackupsResponse$json = {
  '1': 'BackupListVolumeBackupsResponse',
  '2': [
    {
      '1': 'backups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.BackupVolume',
      '10': 'backups'
    },
  ],
};

/// Descriptor for `BackupListVolumeBackupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupListVolumeBackupsResponseDescriptor =
    $convert.base64Decode(
        'Ch9CYWNrdXBMaXN0Vm9sdW1lQmFja3Vwc1Jlc3BvbnNlEjgKB2JhY2t1cHMYASADKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLkJhY2t1cFZvbHVtZVIHYmFja3Vwcw==');

@$core.Deprecated('Use backupGetVolumeBackupRequestDescriptor instead')
const BackupGetVolumeBackupRequest$json = {
  '1': 'BackupGetVolumeBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupGetVolumeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetVolumeBackupRequestDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBHZXRWb2x1bWVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use backupGetVolumeBackupResponseDescriptor instead')
const BackupGetVolumeBackupResponse$json = {
  '1': 'BackupGetVolumeBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupVolume',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupGetVolumeBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupGetVolumeBackupResponseDescriptor =
    $convert.base64Decode(
        'Ch1CYWNrdXBHZXRWb2x1bWVCYWNrdXBSZXNwb25zZRI2CgZiYWNrdXAYASABKAsyHi5vcGVub3'
        'hpZGUuYXBpLnYxLkJhY2t1cFZvbHVtZVIGYmFja3Vw');

@$core.Deprecated('Use backupCreateVolumeBackupRequestDescriptor instead')
const BackupCreateVolumeBackupRequest$json = {
  '1': 'BackupCreateVolumeBackupRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'volume_name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'volumeName'},
    {'1': 'prefix', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'prefix'},
    {'1': 'service_type', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'serviceType'},
    {'1': 'app_name', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'appName'},
    {
      '1': 'service_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'serviceName',
      '17': true
    },
    {'1': 'turn_off', '3': 7, '4': 1, '5': 8, '10': 'turnOff'},
    {
      '1': 'cron_expression',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'cronExpression'
    },
    {
      '1': 'keep_latest_count',
      '3': 9,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'keepLatestCount',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 10,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'destinationId',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'service_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 4,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 6,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'database_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 7,
      '10': 'databaseId',
      '17': true
    },
    {
      '1': 'postgres_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 8,
      '10': 'postgresId',
      '17': true
    },
    {
      '1': 'mysql_id',
      '3': 17,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 9,
      '10': 'mysqlId',
      '17': true
    },
    {
      '1': 'mariadb_id',
      '3': 18,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 10,
      '10': 'mariadbId',
      '17': true
    },
    {
      '1': 'mongo_id',
      '3': 19,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 11,
      '10': 'mongoId',
      '17': true
    },
    {
      '1': 'redis_id',
      '3': 20,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 12,
      '10': 'redisId',
      '17': true
    },
    {
      '1': 'libsql_id',
      '3': 21,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 13,
      '10': 'libsqlId',
      '17': true
    },
  ],
  '8': [
    {'1': '_service_name'},
    {'1': '_keep_latest_count'},
    {'1': '_destination_id'},
    {'1': '_organization_id'},
    {'1': '_service_id'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_database_id'},
    {'1': '_postgres_id'},
    {'1': '_mysql_id'},
    {'1': '_mariadb_id'},
    {'1': '_mongo_id'},
    {'1': '_redis_id'},
    {'1': '_libsql_id'},
  ],
};

/// Descriptor for `BackupCreateVolumeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateVolumeBackupRequestDescriptor = $convert.base64Decode(
    'Ch9CYWNrdXBDcmVhdGVWb2x1bWVCYWNrdXBSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEA'
    'EY/wFSBG5hbWUSKAoLdm9sdW1lX25hbWUYAiABKAlCB7pIBHICEAFSCnZvbHVtZU5hbWUSHwoG'
    'cHJlZml4GAMgASgJQge6SARyAhABUgZwcmVmaXgSKgoMc2VydmljZV90eXBlGAQgASgJQge6SA'
    'RyAhABUgtzZXJ2aWNlVHlwZRIlCghhcHBfbmFtZRgFIAEoCUIKukgHcgUQARj/AVIHYXBwTmFt'
    'ZRIwCgxzZXJ2aWNlX25hbWUYBiABKAlCCLpIBXIDGP8BSABSC3NlcnZpY2VOYW1liAEBEhkKCH'
    'R1cm5fb2ZmGAcgASgIUgd0dXJuT2ZmEjAKD2Nyb25fZXhwcmVzc2lvbhgIIAEoCUIHukgEcgIQ'
    'AVIOY3JvbkV4cHJlc3Npb24SOAoRa2VlcF9sYXRlc3RfY291bnQYCSABKANCB7pIBCICKABIAV'
    'IPa2VlcExhdGVzdENvdW50iAEBEjMKDmRlc3RpbmF0aW9uX2lkGAogASgDQge6SAQiAiAASAJS'
    'DWRlc3RpbmF0aW9uSWSIAQESNQoPb3JnYW5pemF0aW9uX2lkGAsgASgDQge6SAQiAiAASANSDm'
    '9yZ2FuaXphdGlvbklkiAEBEisKCnNlcnZpY2VfaWQYDCABKANCB7pIBCICIABIBFIJc2Vydmlj'
    'ZUlkiAEBEjMKDmFwcGxpY2F0aW9uX2lkGA0gASgDQge6SAQiAiAASAVSDWFwcGxpY2F0aW9uSW'
    'SIAQESKwoKY29tcG9zZV9pZBgOIAEoA0IHukgEIgIgAEgGUgljb21wb3NlSWSIAQESLQoLZGF0'
    'YWJhc2VfaWQYDyABKANCB7pIBCICIABIB1IKZGF0YWJhc2VJZIgBARItCgtwb3N0Z3Jlc19pZB'
    'gQIAEoA0IHukgEIgIgAEgIUgpwb3N0Z3Jlc0lkiAEBEicKCG15c3FsX2lkGBEgASgDQge6SAQi'
    'AiAASAlSB215c3FsSWSIAQESKwoKbWFyaWFkYl9pZBgSIAEoA0IHukgEIgIgAEgKUgltYXJpYW'
    'RiSWSIAQESJwoIbW9uZ29faWQYEyABKANCB7pIBCICIABIC1IHbW9uZ29JZIgBARInCghyZWRp'
    'c19pZBgUIAEoA0IHukgEIgIgAEgMUgdyZWRpc0lkiAEBEikKCWxpYnNxbF9pZBgVIAEoA0IHuk'
    'gEIgIgAEgNUghsaWJzcWxJZIgBAUIPCg1fc2VydmljZV9uYW1lQhQKEl9rZWVwX2xhdGVzdF9j'
    'b3VudEIRCg9fZGVzdGluYXRpb25faWRCEgoQX29yZ2FuaXphdGlvbl9pZEINCgtfc2VydmljZV'
    '9pZEIRCg9fYXBwbGljYXRpb25faWRCDQoLX2NvbXBvc2VfaWRCDgoMX2RhdGFiYXNlX2lkQg4K'
    'DF9wb3N0Z3Jlc19pZEILCglfbXlzcWxfaWRCDQoLX21hcmlhZGJfaWRCCwoJX21vbmdvX2lkQg'
    'sKCV9yZWRpc19pZEIMCgpfbGlic3FsX2lk');

@$core.Deprecated('Use backupCreateVolumeBackupResponseDescriptor instead')
const BackupCreateVolumeBackupResponse$json = {
  '1': 'BackupCreateVolumeBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupVolume',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupCreateVolumeBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupCreateVolumeBackupResponseDescriptor =
    $convert.base64Decode(
        'CiBCYWNrdXBDcmVhdGVWb2x1bWVCYWNrdXBSZXNwb25zZRI2CgZiYWNrdXAYASABKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLkJhY2t1cFZvbHVtZVIGYmFja3Vw');

@$core.Deprecated('Use backupUpdateVolumeBackupRequestDescriptor instead')
const BackupUpdateVolumeBackupRequest$json = {
  '1': 'BackupUpdateVolumeBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'name',
      '17': true
    },
    {
      '1': 'volume_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'volumeName',
      '17': true
    },
    {'1': 'prefix', '3': 4, '4': 1, '5': 9, '9': 2, '10': 'prefix', '17': true},
    {
      '1': 'service_type',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'serviceType',
      '17': true
    },
    {
      '1': 'app_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'appName',
      '17': true
    },
    {
      '1': 'service_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'turn_off',
      '3': 8,
      '4': 1,
      '5': 8,
      '9': 6,
      '10': 'turnOff',
      '17': true
    },
    {
      '1': 'cron_expression',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'cronExpression',
      '17': true
    },
    {
      '1': 'keep_latest_count',
      '3': 10,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 8,
      '10': 'keepLatestCount',
      '17': true
    },
    {
      '1': 'destination_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 9,
      '10': 'destinationId',
      '17': true
    },
    {
      '1': 'enabled',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 10,
      '10': 'enabled',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_volume_name'},
    {'1': '_prefix'},
    {'1': '_service_type'},
    {'1': '_app_name'},
    {'1': '_service_name'},
    {'1': '_turn_off'},
    {'1': '_cron_expression'},
    {'1': '_keep_latest_count'},
    {'1': '_destination_id'},
    {'1': '_enabled'},
  ],
};

/// Descriptor for `BackupUpdateVolumeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateVolumeBackupRequestDescriptor = $convert.base64Decode(
    'Ch9CYWNrdXBVcGRhdGVWb2x1bWVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUg'
    'JpZBIhCgRuYW1lGAIgASgJQgi6SAVyAxj/AUgAUgRuYW1liAEBEiQKC3ZvbHVtZV9uYW1lGAMg'
    'ASgJSAFSCnZvbHVtZU5hbWWIAQESGwoGcHJlZml4GAQgASgJSAJSBnByZWZpeIgBARImCgxzZX'
    'J2aWNlX3R5cGUYBSABKAlIA1ILc2VydmljZVR5cGWIAQESKAoIYXBwX25hbWUYBiABKAlCCLpI'
    'BXIDGP8BSARSB2FwcE5hbWWIAQESMAoMc2VydmljZV9uYW1lGAcgASgJQgi6SAVyAxj/AUgFUg'
    'tzZXJ2aWNlTmFtZYgBARIeCgh0dXJuX29mZhgIIAEoCEgGUgd0dXJuT2ZmiAEBEiwKD2Nyb25f'
    'ZXhwcmVzc2lvbhgJIAEoCUgHUg5jcm9uRXhwcmVzc2lvbogBARI4ChFrZWVwX2xhdGVzdF9jb3'
    'VudBgKIAEoA0IHukgEIgIoAEgIUg9rZWVwTGF0ZXN0Q291bnSIAQESMwoOZGVzdGluYXRpb25f'
    'aWQYCyABKANCB7pIBCICIABICVINZGVzdGluYXRpb25JZIgBARIdCgdlbmFibGVkGAwgASgISA'
    'pSB2VuYWJsZWSIAQFCBwoFX25hbWVCDgoMX3ZvbHVtZV9uYW1lQgkKB19wcmVmaXhCDwoNX3Nl'
    'cnZpY2VfdHlwZUILCglfYXBwX25hbWVCDwoNX3NlcnZpY2VfbmFtZUILCglfdHVybl9vZmZCEg'
    'oQX2Nyb25fZXhwcmVzc2lvbkIUChJfa2VlcF9sYXRlc3RfY291bnRCEQoPX2Rlc3RpbmF0aW9u'
    'X2lkQgoKCF9lbmFibGVk');

@$core.Deprecated('Use backupUpdateVolumeBackupResponseDescriptor instead')
const BackupUpdateVolumeBackupResponse$json = {
  '1': 'BackupUpdateVolumeBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.BackupVolume',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `BackupUpdateVolumeBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupUpdateVolumeBackupResponseDescriptor =
    $convert.base64Decode(
        'CiBCYWNrdXBVcGRhdGVWb2x1bWVCYWNrdXBSZXNwb25zZRI2CgZiYWNrdXAYASABKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLkJhY2t1cFZvbHVtZVIGYmFja3Vw');

@$core.Deprecated('Use backupDeleteVolumeBackupRequestDescriptor instead')
const BackupDeleteVolumeBackupRequest$json = {
  '1': 'BackupDeleteVolumeBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupDeleteVolumeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupDeleteVolumeBackupRequestDescriptor =
    $convert.base64Decode(
        'Ch9CYWNrdXBEZWxldGVWb2x1bWVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUg'
        'JpZA==');

@$core.Deprecated('Use backupRunVolumeBackupRequestDescriptor instead')
const BackupRunVolumeBackupRequest$json = {
  '1': 'BackupRunVolumeBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `BackupRunVolumeBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRunVolumeBackupRequestDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBSdW5Wb2x1bWVCYWNrdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use backupRestoreDatabaseRequestDescriptor instead')
const BackupRestoreDatabaseRequest$json = {
  '1': 'BackupRestoreDatabaseRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'backup_file', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'backupFile'},
  ],
};

/// Descriptor for `BackupRestoreDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRestoreDatabaseRequestDescriptor =
    $convert.base64Decode(
        'ChxCYWNrdXBSZXN0b3JlRGF0YWJhc2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZB'
        'IoCgtiYWNrdXBfZmlsZRgCIAEoCUIHukgEcgIQAVIKYmFja3VwRmlsZQ==');

@$core.Deprecated('Use backupRestoreVolumeRequestDescriptor instead')
const BackupRestoreVolumeRequest$json = {
  '1': 'BackupRestoreVolumeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'backup_file', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'backupFile'},
  ],
};

/// Descriptor for `BackupRestoreVolumeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupRestoreVolumeRequestDescriptor =
    $convert.base64Decode(
        'ChpCYWNrdXBSZXN0b3JlVm9sdW1lUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSKA'
        'oLYmFja3VwX2ZpbGUYAiABKAlCB7pIBHICEAFSCmJhY2t1cEZpbGU=');

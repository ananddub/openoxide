// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/remote_server.proto.

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

@$core.Deprecated('Use remoteServerTypeDescriptor instead')
const RemoteServerType$json = {
  '1': 'RemoteServerType',
  '2': [
    {'1': 'REMOTE_SERVER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_TYPE_DEPLOY', '2': 1},
    {'1': 'REMOTE_SERVER_TYPE_BUILD', '2': 2},
  ],
};

/// Descriptor for `RemoteServerType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerTypeDescriptor = $convert.base64Decode(
    'ChBSZW1vdGVTZXJ2ZXJUeXBlEiIKHlJFTU9URV9TRVJWRVJfVFlQRV9VTlNQRUNJRklFRBAAEh'
    '0KGVJFTU9URV9TRVJWRVJfVFlQRV9ERVBMT1kQARIcChhSRU1PVEVfU0VSVkVSX1RZUEVfQlVJ'
    'TEQQAg==');

@$core.Deprecated('Use remoteServerStatusDescriptor instead')
const RemoteServerStatus$json = {
  '1': 'RemoteServerStatus',
  '2': [
    {'1': 'REMOTE_SERVER_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_STATUS_ACTIVE', '2': 1},
    {'1': 'REMOTE_SERVER_STATUS_INACTIVE', '2': 2},
  ],
};

/// Descriptor for `RemoteServerStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerStatusDescriptor = $convert.base64Decode(
    'ChJSZW1vdGVTZXJ2ZXJTdGF0dXMSJAogUkVNT1RFX1NFUlZFUl9TVEFUVVNfVU5TUEVDSUZJRU'
    'QQABIfChtSRU1PVEVfU0VSVkVSX1NUQVRVU19BQ1RJVkUQARIhCh1SRU1PVEVfU0VSVkVSX1NU'
    'QVRVU19JTkFDVElWRRAC');

@$core.Deprecated('Use remoteServerActionDescriptor instead')
const RemoteServerAction$json = {
  '1': 'RemoteServerAction',
  '2': [
    {'1': 'REMOTE_SERVER_ACTION_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_ACTION_ACTIVATE', '2': 1},
    {'1': 'REMOTE_SERVER_ACTION_DEACTIVATE', '2': 2},
    {'1': 'REMOTE_SERVER_ACTION_TEST_CONNECTION', '2': 3},
  ],
};

/// Descriptor for `RemoteServerAction`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerActionDescriptor = $convert.base64Decode(
    'ChJSZW1vdGVTZXJ2ZXJBY3Rpb24SJAogUkVNT1RFX1NFUlZFUl9BQ1RJT05fVU5TUEVDSUZJRU'
    'QQABIhCh1SRU1PVEVfU0VSVkVSX0FDVElPTl9BQ1RJVkFURRABEiMKH1JFTU9URV9TRVJWRVJf'
    'QUNUSU9OX0RFQUNUSVZBVEUQAhIoCiRSRU1PVEVfU0VSVkVSX0FDVElPTl9URVNUX0NPTk5FQ1'
    'RJT04QAw==');

@$core.Deprecated('Use remoteServerDependencyMigrationStatusDescriptor instead')
const RemoteServerDependencyMigrationStatus$json = {
  '1': 'RemoteServerDependencyMigrationStatus',
  '2': [
    {'1': 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_RUNNING', '2': 1},
    {'1': 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_SUCCEEDED', '2': 2},
    {'1': 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_FAILED', '2': 3},
    {'1': 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_ROLLED_BACK', '2': 4},
  ],
};

/// Descriptor for `RemoteServerDependencyMigrationStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerDependencyMigrationStatusDescriptor = $convert.base64Decode(
    'CiVSZW1vdGVTZXJ2ZXJEZXBlbmRlbmN5TWlncmF0aW9uU3RhdHVzEjkKNVJFTU9URV9TRVJWRV'
    'JfREVQRU5ERU5DWV9NSUdSQVRJT05fU1RBVFVTX1VOU1BFQ0lGSUVEEAASNQoxUkVNT1RFX1NF'
    'UlZFUl9ERVBFTkRFTkNZX01JR1JBVElPTl9TVEFUVVNfUlVOTklORxABEjcKM1JFTU9URV9TRV'
    'JWRVJfREVQRU5ERU5DWV9NSUdSQVRJT05fU1RBVFVTX1NVQ0NFRURFRBACEjQKMFJFTU9URV9T'
    'RVJWRVJfREVQRU5ERU5DWV9NSUdSQVRJT05fU1RBVFVTX0ZBSUxFRBADEjkKNVJFTU9URV9TRV'
    'JWRVJfREVQRU5ERU5DWV9NSUdSQVRJT05fU1RBVFVTX1JPTExFRF9CQUNLEAQ=');

@$core
    .Deprecated('Use remoteServerApplicationMigrationStatusDescriptor instead')
const RemoteServerApplicationMigrationStatus$json = {
  '1': 'RemoteServerApplicationMigrationStatus',
  '2': [
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_RUNNING', '2': 1},
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_APPLYING', '2': 2},
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_SUCCEEDED', '2': 3},
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_FAILED', '2': 4},
    {'1': 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_CANCELLED', '2': 5},
  ],
};

/// Descriptor for `RemoteServerApplicationMigrationStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerApplicationMigrationStatusDescriptor = $convert.base64Decode(
    'CiZSZW1vdGVTZXJ2ZXJBcHBsaWNhdGlvbk1pZ3JhdGlvblN0YXR1cxI6CjZSRU1PVEVfU0VSVk'
    'VSX0FQUExJQ0FUSU9OX01JR1JBVElPTl9TVEFUVVNfVU5TUEVDSUZJRUQQABI2CjJSRU1PVEVf'
    'U0VSVkVSX0FQUExJQ0FUSU9OX01JR1JBVElPTl9TVEFUVVNfUlVOTklORxABEjcKM1JFTU9URV'
    '9TRVJWRVJfQVBQTElDQVRJT05fTUlHUkFUSU9OX1NUQVRVU19BUFBMWUlORxACEjgKNFJFTU9U'
    'RV9TRVJWRVJfQVBQTElDQVRJT05fTUlHUkFUSU9OX1NUQVRVU19TVUNDRUVERUQQAxI1CjFSRU'
    '1PVEVfU0VSVkVSX0FQUExJQ0FUSU9OX01JR1JBVElPTl9TVEFUVVNfRkFJTEVEEAQSOAo0UkVN'
    'T1RFX1NFUlZFUl9BUFBMSUNBVElPTl9NSUdSQVRJT05fU1RBVFVTX0NBTkNFTExFRBAF');

@$core.Deprecated('Use remoteServerMigrationPhaseDescriptor instead')
const RemoteServerMigrationPhase$json = {
  '1': 'RemoteServerMigrationPhase',
  '2': [
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_PREFLIGHT', '2': 1},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_QUIESCE', '2': 2},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_SYNC', '2': 3},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_VERIFY', '2': 4},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_APPLY', '2': 5},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_REPOINT', '2': 6},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_REDEPLOY', '2': 7},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_HEALTH', '2': 8},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_SUPERSEDED', '2': 9},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_FAILED', '2': 10},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_ROLLED_BACK', '2': 11},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_CANCELLED', '2': 12},
    {'1': 'REMOTE_SERVER_MIGRATION_PHASE_DONE', '2': 13},
  ],
};

/// Descriptor for `RemoteServerMigrationPhase`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerMigrationPhaseDescriptor = $convert.base64Decode(
    'ChpSZW1vdGVTZXJ2ZXJNaWdyYXRpb25QaGFzZRItCilSRU1PVEVfU0VSVkVSX01JR1JBVElPTl'
    '9QSEFTRV9VTlNQRUNJRklFRBAAEisKJ1JFTU9URV9TRVJWRVJfTUlHUkFUSU9OX1BIQVNFX1BS'
    'RUZMSUdIVBABEikKJVJFTU9URV9TRVJWRVJfTUlHUkFUSU9OX1BIQVNFX1FVSUVTQ0UQAhImCi'
    'JSRU1PVEVfU0VSVkVSX01JR1JBVElPTl9QSEFTRV9TWU5DEAMSKAokUkVNT1RFX1NFUlZFUl9N'
    'SUdSQVRJT05fUEhBU0VfVkVSSUZZEAQSJwojUkVNT1RFX1NFUlZFUl9NSUdSQVRJT05fUEhBU0'
    'VfQVBQTFkQBRIpCiVSRU1PVEVfU0VSVkVSX01JR1JBVElPTl9QSEFTRV9SRVBPSU5UEAYSKgom'
    'UkVNT1RFX1NFUlZFUl9NSUdSQVRJT05fUEhBU0VfUkVERVBMT1kQBxIoCiRSRU1PVEVfU0VSVk'
    'VSX01JR1JBVElPTl9QSEFTRV9IRUFMVEgQCBIsCihSRU1PVEVfU0VSVkVSX01JR1JBVElPTl9Q'
    'SEFTRV9TVVBFUlNFREVEEAkSKAokUkVNT1RFX1NFUlZFUl9NSUdSQVRJT05fUEhBU0VfRkFJTE'
    'VEEAoSLQopUkVNT1RFX1NFUlZFUl9NSUdSQVRJT05fUEhBU0VfUk9MTEVEX0JBQ0sQCxIrCidS'
    'RU1PVEVfU0VSVkVSX01JR1JBVElPTl9QSEFTRV9DQU5DRUxMRUQQDBImCiJSRU1PVEVfU0VSVk'
    'VSX01JR1JBVElPTl9QSEFTRV9ET05FEA0=');

@$core.Deprecated('Use remoteServerMigrationObjectKindDescriptor instead')
const RemoteServerMigrationObjectKind$json = {
  '1': 'RemoteServerMigrationObjectKind',
  '2': [
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_UNSPECIFIED', '2': 0},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_VOLUME', '2': 1},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_BIND', '2': 2},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_DBDATA', '2': 3},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_DUMP', '2': 4},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_SRC', '2': 5},
    {'1': 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_CERT', '2': 6},
  ],
};

/// Descriptor for `RemoteServerMigrationObjectKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List remoteServerMigrationObjectKindDescriptor = $convert.base64Decode(
    'Ch9SZW1vdGVTZXJ2ZXJNaWdyYXRpb25PYmplY3RLaW5kEjMKL1JFTU9URV9TRVJWRVJfTUlHUk'
    'FUSU9OX09CSkVDVF9LSU5EX1VOU1BFQ0lGSUVEEAASLgoqUkVNT1RFX1NFUlZFUl9NSUdSQVRJ'
    'T05fT0JKRUNUX0tJTkRfVk9MVU1FEAESLAooUkVNT1RFX1NFUlZFUl9NSUdSQVRJT05fT0JKRU'
    'NUX0tJTkRfQklORBACEi4KKlJFTU9URV9TRVJWRVJfTUlHUkFUSU9OX09CSkVDVF9LSU5EX0RC'
    'REFUQRADEiwKKFJFTU9URV9TRVJWRVJfTUlHUkFUSU9OX09CSkVDVF9LSU5EX0RVTVAQBBIrCi'
    'dSRU1PVEVfU0VSVkVSX01JR1JBVElPTl9PQkpFQ1RfS0lORF9TUkMQBRIsCihSRU1PVEVfU0VS'
    'VkVSX01JR1JBVElPTl9PQkpFQ1RfS0lORF9DRVJUEAY=');

@$core.Deprecated('Use remoteServerDescriptor instead')
const RemoteServer$json = {
  '1': 'RemoteServer',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'ip_address', '3': 4, '4': 1, '5': 9, '10': 'ipAddress'},
    {'1': 'port', '3': 5, '4': 1, '5': 3, '10': 'port'},
    {'1': 'username', '3': 6, '4': 1, '5': 9, '10': 'username'},
    {'1': 'app_name', '3': 7, '4': 1, '5': 9, '10': 'appName'},
    {
      '1': 'server_status',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerStatus',
      '10': 'serverStatus'
    },
    {
      '1': 'server_type',
      '3': 9,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerType',
      '10': 'serverType'
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 10,
      '4': 1,
      '5': 8,
      '10': 'enableDockerCleanup'
    },
    {
      '1': 'log_cleanup_cron',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'logCleanupCron',
      '17': true
    },
    {'1': 'command', '3': 12, '4': 1, '5': 9, '10': 'command'},
    {'1': 'metrics_config', '3': 13, '4': 1, '5': 9, '10': 'metricsConfig'},
    {
      '1': 'ssh_key_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'sshKeyId',
      '17': true
    },
    {'1': 'created_at', '3': 15, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 16, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'build_memory_limit',
      '3': 17,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_log_cleanup_cron'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `RemoteServer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerDescriptor = $convert.base64Decode(
    'CgxSZW1vdGVTZXJ2ZXISDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSJQoLZG'
    'VzY3JpcHRpb24YAyABKAlIAFILZGVzY3JpcHRpb26IAQESHQoKaXBfYWRkcmVzcxgEIAEoCVIJ'
    'aXBBZGRyZXNzEhIKBHBvcnQYBSABKANSBHBvcnQSGgoIdXNlcm5hbWUYBiABKAlSCHVzZXJuYW'
    '1lEhkKCGFwcF9uYW1lGAcgASgJUgdhcHBOYW1lEkkKDXNlcnZlcl9zdGF0dXMYCCABKA4yJC5v'
    'cGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlclN0YXR1c1IMc2VydmVyU3RhdHVzEkMKC3Nlcn'
    'Zlcl90eXBlGAkgASgOMiIub3Blbm94aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJUeXBlUgpzZXJ2'
    'ZXJUeXBlEjIKFWVuYWJsZV9kb2NrZXJfY2xlYW51cBgKIAEoCFITZW5hYmxlRG9ja2VyQ2xlYW'
    '51cBItChBsb2dfY2xlYW51cF9jcm9uGAsgASgJSAFSDmxvZ0NsZWFudXBDcm9uiAEBEhgKB2Nv'
    'bW1hbmQYDCABKAlSB2NvbW1hbmQSJQoObWV0cmljc19jb25maWcYDSABKAlSDW1ldHJpY3NDb2'
    '5maWcSIQoKc3NoX2tleV9pZBgOIAEoA0gCUghzc2hLZXlJZIgBARIdCgpjcmVhdGVkX2F0GA8g'
    'ASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgQIAEoA1IJdXBkYXRlZEF0EjEKEmJ1aWxkX2'
    '1lbW9yeV9saW1pdBgRIAEoCUgDUhBidWlsZE1lbW9yeUxpbWl0iAEBEisKD2J1aWxkX2NwdV9s'
    'aW1pdBgSIAEoCUgEUg1idWlsZENwdUxpbWl0iAEBQg4KDF9kZXNjcmlwdGlvbkITChFfbG9nX2'
    'NsZWFudXBfY3JvbkINCgtfc3NoX2tleV9pZEIVChNfYnVpbGRfbWVtb3J5X2xpbWl0QhIKEF9i'
    'dWlsZF9jcHVfbGltaXQ=');

@$core.Deprecated('Use remoteServerDependencyMigrationDescriptor instead')
const RemoteServerDependencyMigration$json = {
  '1': 'RemoteServerDependencyMigration',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '10': 'migrationId'},
    {
      '1': 'status',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerDependencyMigrationStatus',
      '10': 'status'
    },
    {'1': 'source_server_id', '3': 3, '4': 1, '5': 3, '10': 'sourceServerId'},
    {'1': 'target_server_id', '3': 4, '4': 1, '5': 3, '10': 'targetServerId'},
    {'1': 'applications', '3': 5, '4': 1, '5': 3, '10': 'applications'},
    {
      '1': 'build_assignments',
      '3': 6,
      '4': 1,
      '5': 3,
      '10': 'buildAssignments'
    },
    {'1': 'compose_projects', '3': 7, '4': 1, '5': 3, '10': 'composeProjects'},
    {'1': 'databases', '3': 8, '4': 1, '5': 3, '10': 'databases'},
    {'1': 'certificates', '3': 9, '4': 1, '5': 3, '10': 'certificates'},
    {'1': 'schedules', '3': 10, '4': 1, '5': 3, '10': 'schedules'},
    {
      '1': 'queued_applications',
      '3': 11,
      '4': 1,
      '5': 3,
      '10': 'queuedApplications'
    },
    {
      '1': 'queued_compose_projects',
      '3': 12,
      '4': 1,
      '5': 3,
      '10': 'queuedComposeProjects'
    },
    {'1': 'error', '3': 13, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_error'},
  ],
};

/// Descriptor for `RemoteServerDependencyMigration`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerDependencyMigrationDescriptor = $convert.base64Decode(
    'Ch9SZW1vdGVTZXJ2ZXJEZXBlbmRlbmN5TWlncmF0aW9uEiEKDG1pZ3JhdGlvbl9pZBgBIAEoCV'
    'ILbWlncmF0aW9uSWQSTwoGc3RhdHVzGAIgASgOMjcub3Blbm94aWRlLmFwaS52MS5SZW1vdGVT'
    'ZXJ2ZXJEZXBlbmRlbmN5TWlncmF0aW9uU3RhdHVzUgZzdGF0dXMSKAoQc291cmNlX3NlcnZlcl'
    '9pZBgDIAEoA1IOc291cmNlU2VydmVySWQSKAoQdGFyZ2V0X3NlcnZlcl9pZBgEIAEoA1IOdGFy'
    'Z2V0U2VydmVySWQSIgoMYXBwbGljYXRpb25zGAUgASgDUgxhcHBsaWNhdGlvbnMSKwoRYnVpbG'
    'RfYXNzaWdubWVudHMYBiABKANSEGJ1aWxkQXNzaWdubWVudHMSKQoQY29tcG9zZV9wcm9qZWN0'
    'cxgHIAEoA1IPY29tcG9zZVByb2plY3RzEhwKCWRhdGFiYXNlcxgIIAEoA1IJZGF0YWJhc2VzEi'
    'IKDGNlcnRpZmljYXRlcxgJIAEoA1IMY2VydGlmaWNhdGVzEhwKCXNjaGVkdWxlcxgKIAEoA1IJ'
    'c2NoZWR1bGVzEi8KE3F1ZXVlZF9hcHBsaWNhdGlvbnMYCyABKANSEnF1ZXVlZEFwcGxpY2F0aW'
    '9ucxI2ChdxdWV1ZWRfY29tcG9zZV9wcm9qZWN0cxgMIAEoA1IVcXVldWVkQ29tcG9zZVByb2pl'
    'Y3RzEhkKBWVycm9yGA0gASgJSABSBWVycm9yiAEBQggKBl9lcnJvcg==');

@$core.Deprecated('Use remoteServerApplicationMigrationDescriptor instead')
const RemoteServerApplicationMigration$json = {
  '1': 'RemoteServerApplicationMigration',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'service_id', '3': 3, '4': 1, '5': 3, '10': 'serviceId'},
    {'1': 'source_server_id', '3': 4, '4': 1, '5': 3, '10': 'sourceServerId'},
    {'1': 'target_server_id', '3': 5, '4': 1, '5': 3, '10': 'targetServerId'},
    {
      '1': 'status',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerApplicationMigrationStatus',
      '10': 'status'
    },
    {
      '1': 'phase',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerMigrationPhase',
      '10': 'phase'
    },
    {'1': 'progress', '3': 8, '4': 1, '5': 3, '10': 'progress'},
    {'1': 'cancel_requested', '3': 9, '4': 1, '5': 8, '10': 'cancelRequested'},
    {'1': 'error', '3': 10, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
    {'1': 'log_path', '3': 11, '4': 1, '5': 9, '10': 'logPath'},
    {'1': 'created_at', '3': 12, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 13, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_error'},
  ],
};

/// Descriptor for `RemoteServerApplicationMigration`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerApplicationMigrationDescriptor = $convert.base64Decode(
    'CiBSZW1vdGVTZXJ2ZXJBcHBsaWNhdGlvbk1pZ3JhdGlvbhIOCgJpZBgBIAEoCVICaWQSJwoPb3'
    'JnYW5pemF0aW9uX2lkGAIgASgDUg5vcmdhbml6YXRpb25JZBIdCgpzZXJ2aWNlX2lkGAMgASgD'
    'UglzZXJ2aWNlSWQSKAoQc291cmNlX3NlcnZlcl9pZBgEIAEoA1IOc291cmNlU2VydmVySWQSKA'
    'oQdGFyZ2V0X3NlcnZlcl9pZBgFIAEoA1IOdGFyZ2V0U2VydmVySWQSUAoGc3RhdHVzGAYgASgO'
    'Mjgub3Blbm94aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJBcHBsaWNhdGlvbk1pZ3JhdGlvblN0YX'
    'R1c1IGc3RhdHVzEkIKBXBoYXNlGAcgASgOMiwub3Blbm94aWRlLmFwaS52MS5SZW1vdGVTZXJ2'
    'ZXJNaWdyYXRpb25QaGFzZVIFcGhhc2USGgoIcHJvZ3Jlc3MYCCABKANSCHByb2dyZXNzEikKEG'
    'NhbmNlbF9yZXF1ZXN0ZWQYCSABKAhSD2NhbmNlbFJlcXVlc3RlZBIZCgVlcnJvchgKIAEoCUgA'
    'UgVlcnJvcogBARIZCghsb2dfcGF0aBgLIAEoCVIHbG9nUGF0aBIdCgpjcmVhdGVkX2F0GAwgAS'
    'gDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgNIAEoA1IJdXBkYXRlZEF0QggKBl9lcnJvcg==');

@$core.Deprecated('Use remoteServerMigrationObjectDescriptor instead')
const RemoteServerMigrationObject$json = {
  '1': 'RemoteServerMigrationObject',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '10': 'migrationId'},
    {'1': 'obj_key', '3': 2, '4': 1, '5': 9, '10': 'objKey'},
    {
      '1': 'kind',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerMigrationObjectKind',
      '10': 'kind'
    },
    {'1': 'source_ref', '3': 4, '4': 1, '5': 9, '10': 'sourceRef'},
    {'1': 'dest_ref', '3': 5, '4': 1, '5': 9, '10': 'destRef'},
    {'1': 'status', '3': 6, '4': 1, '5': 9, '10': 'status'},
    {'1': 'bytes', '3': 7, '4': 1, '5': 3, '10': 'bytes'},
    {'1': 'error', '3': 8, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_error'},
  ],
};

/// Descriptor for `RemoteServerMigrationObject`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerMigrationObjectDescriptor = $convert.base64Decode(
    'ChtSZW1vdGVTZXJ2ZXJNaWdyYXRpb25PYmplY3QSIQoMbWlncmF0aW9uX2lkGAEgASgJUgttaW'
    'dyYXRpb25JZBIXCgdvYmpfa2V5GAIgASgJUgZvYmpLZXkSRQoEa2luZBgDIAEoDjIxLm9wZW5v'
    'eGlkZS5hcGkudjEuUmVtb3RlU2VydmVyTWlncmF0aW9uT2JqZWN0S2luZFIEa2luZBIdCgpzb3'
    'VyY2VfcmVmGAQgASgJUglzb3VyY2VSZWYSGQoIZGVzdF9yZWYYBSABKAlSB2Rlc3RSZWYSFgoG'
    'c3RhdHVzGAYgASgJUgZzdGF0dXMSFAoFYnl0ZXMYByABKANSBWJ5dGVzEhkKBWVycm9yGAggAS'
    'gJSABSBWVycm9yiAEBEh0KCmNyZWF0ZWRfYXQYCSABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVk'
    'X2F0GAogASgDUgl1cGRhdGVkQXRCCAoGX2Vycm9y');

@$core.Deprecated('Use remoteServerListRequestDescriptor instead')
const RemoteServerListRequest$json = {
  '1': 'RemoteServerListRequest',
};

/// Descriptor for `RemoteServerListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListRequestDescriptor =
    $convert.base64Decode('ChdSZW1vdGVTZXJ2ZXJMaXN0UmVxdWVzdA==');

@$core.Deprecated('Use remoteServerListResponseDescriptor instead')
const RemoteServerListResponse$json = {
  '1': 'RemoteServerListResponse',
  '2': [
    {
      '1': 'servers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'servers'
    },
  ],
};

/// Descriptor for `RemoteServerListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListResponseDescriptor =
    $convert.base64Decode(
        'ChhSZW1vdGVTZXJ2ZXJMaXN0UmVzcG9uc2USOAoHc2VydmVycxgBIAMoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuUmVtb3RlU2VydmVyUgdzZXJ2ZXJz');

@$core.Deprecated('Use remoteServerGetRequestDescriptor instead')
const RemoteServerGetRequest$json = {
  '1': 'RemoteServerGetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoteServerGetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerGetRequestDescriptor =
    $convert.base64Decode(
        'ChZSZW1vdGVTZXJ2ZXJHZXRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use remoteServerGetResponseDescriptor instead')
const RemoteServerGetResponse$json = {
  '1': 'RemoteServerGetResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
  ],
};

/// Descriptor for `RemoteServerGetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerGetResponseDescriptor =
    $convert.base64Decode(
        'ChdSZW1vdGVTZXJ2ZXJHZXRSZXNwb25zZRI2CgZzZXJ2ZXIYASABKAsyHi5vcGVub3hpZGUuYX'
        'BpLnYxLlJlbW90ZVNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use remoteServerCreateRequestDescriptor instead')
const RemoteServerCreateRequest$json = {
  '1': 'RemoteServerCreateRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'description',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'ip_address', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'ipAddress'},
    {'1': 'port', '3': 4, '4': 1, '5': 3, '10': 'port'},
    {'1': 'username', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {
      '1': 'server_type',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerType',
      '8': {},
      '10': 'serverType'
    },
    {
      '1': 'ssh_key_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'sshKeyId',
      '17': true
    },
    {
      '1': 'build_memory_limit',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `RemoteServerCreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerCreateRequestDescriptor = $convert.base64Decode(
    'ChlSZW1vdGVTZXJ2ZXJDcmVhdGVSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG'
    '5hbWUSLwoLZGVzY3JpcHRpb24YAiABKAlCCLpIBXIDGOgHSABSC2Rlc2NyaXB0aW9uiAEBEikK'
    'CmlwX2FkZHJlc3MYAyABKAlCCrpIB3IFEAEY/wFSCWlwQWRkcmVzcxISCgRwb3J0GAQgASgDUg'
    'Rwb3J0EiQKCHVzZXJuYW1lGAUgASgJQgi6SAVyAxj/AVIIdXNlcm5hbWUSTwoLc2VydmVyX3R5'
    'cGUYBiABKA4yIi5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlclR5cGVCCrpIB4IBBBABIA'
    'BSCnNlcnZlclR5cGUSKgoKc3NoX2tleV9pZBgHIAEoA0IHukgEIgIgAEgBUghzc2hLZXlJZIgB'
    'ARI7ChJidWlsZF9tZW1vcnlfbGltaXQYCCABKAlCCLpIBXIDGP8BSAJSEGJ1aWxkTWVtb3J5TG'
    'ltaXSIAQESNQoPYnVpbGRfY3B1X2xpbWl0GAkgASgJQgi6SAVyAxj/AUgDUg1idWlsZENwdUxp'
    'bWl0iAEBQg4KDF9kZXNjcmlwdGlvbkINCgtfc3NoX2tleV9pZEIVChNfYnVpbGRfbWVtb3J5X2'
    'xpbWl0QhIKEF9idWlsZF9jcHVfbGltaXQ=');

@$core.Deprecated('Use remoteServerCreateResponseDescriptor instead')
const RemoteServerCreateResponse$json = {
  '1': 'RemoteServerCreateResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
  ],
};

/// Descriptor for `RemoteServerCreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerCreateResponseDescriptor =
    $convert.base64Decode(
        'ChpSZW1vdGVTZXJ2ZXJDcmVhdGVSZXNwb25zZRI2CgZzZXJ2ZXIYASABKAsyHi5vcGVub3hpZG'
        'UuYXBpLnYxLlJlbW90ZVNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use remoteServerUpdateRequestDescriptor instead')
const RemoteServerUpdateRequest$json = {
  '1': 'RemoteServerUpdateRequest',
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
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'description',
      '17': true
    },
    {
      '1': 'ip_address',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'ipAddress',
      '17': true
    },
    {'1': 'port', '3': 5, '4': 1, '5': 3, '9': 3, '10': 'port', '17': true},
    {
      '1': 'username',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'username',
      '17': true
    },
    {
      '1': 'server_status',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerStatus',
      '8': {},
      '9': 5,
      '10': 'serverStatus',
      '17': true
    },
    {
      '1': 'server_type',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerType',
      '8': {},
      '9': 6,
      '10': 'serverType',
      '17': true
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 9,
      '4': 1,
      '5': 8,
      '9': 7,
      '10': 'enableDockerCleanup',
      '17': true
    },
    {
      '1': 'log_cleanup_cron',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'logCleanupCron',
      '17': true
    },
    {
      '1': 'command',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'command',
      '17': true
    },
    {
      '1': 'metrics_config',
      '3': 12,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 10,
      '10': 'metricsConfig',
      '17': true
    },
    {
      '1': 'ssh_key_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 11,
      '10': 'sshKeyId',
      '17': true
    },
    {
      '1': 'build_memory_limit',
      '3': 14,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 12,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 15,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 13,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_ip_address'},
    {'1': '_port'},
    {'1': '_username'},
    {'1': '_server_status'},
    {'1': '_server_type'},
    {'1': '_enable_docker_cleanup'},
    {'1': '_log_cleanup_cron'},
    {'1': '_command'},
    {'1': '_metrics_config'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `RemoteServerUpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerUpdateRequestDescriptor = $convert.base64Decode(
    'ChlSZW1vdGVTZXJ2ZXJVcGRhdGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIjCg'
    'RuYW1lGAIgASgJQgq6SAdyBRABGP8BSABSBG5hbWWIAQESLwoLZGVzY3JpcHRpb24YAyABKAlC'
    'CLpIBXIDGOgHSAFSC2Rlc2NyaXB0aW9uiAEBEiwKCmlwX2FkZHJlc3MYBCABKAlCCLpIBXIDGP'
    '8BSAJSCWlwQWRkcmVzc4gBARIXCgRwb3J0GAUgASgDSANSBHBvcnSIAQESKQoIdXNlcm5hbWUY'
    'BiABKAlCCLpIBXIDGP8BSARSCHVzZXJuYW1liAEBEloKDXNlcnZlcl9zdGF0dXMYByABKA4yJC'
    '5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlclN0YXR1c0IKukgHggEEEAEgAEgFUgxzZXJ2'
    'ZXJTdGF0dXOIAQESVAoLc2VydmVyX3R5cGUYCCABKA4yIi5vcGVub3hpZGUuYXBpLnYxLlJlbW'
    '90ZVNlcnZlclR5cGVCCrpIB4IBBBABIABIBlIKc2VydmVyVHlwZYgBARI3ChVlbmFibGVfZG9j'
    'a2VyX2NsZWFudXAYCSABKAhIB1ITZW5hYmxlRG9ja2VyQ2xlYW51cIgBARItChBsb2dfY2xlYW'
    '51cF9jcm9uGAogASgJSAhSDmxvZ0NsZWFudXBDcm9uiAEBEh0KB2NvbW1hbmQYCyABKAlICVIH'
    'Y29tbWFuZIgBARI1Cg5tZXRyaWNzX2NvbmZpZxgMIAEoCUIJukgGcgQYgIBASApSDW1ldHJpY3'
    'NDb25maWeIAQESKgoKc3NoX2tleV9pZBgNIAEoA0IHukgEIgIgAEgLUghzc2hLZXlJZIgBARI7'
    'ChJidWlsZF9tZW1vcnlfbGltaXQYDiABKAlCCLpIBXIDGP8BSAxSEGJ1aWxkTWVtb3J5TGltaX'
    'SIAQESNQoPYnVpbGRfY3B1X2xpbWl0GA8gASgJQgi6SAVyAxj/AUgNUg1idWlsZENwdUxpbWl0'
    'iAEBQgcKBV9uYW1lQg4KDF9kZXNjcmlwdGlvbkINCgtfaXBfYWRkcmVzc0IHCgVfcG9ydEILCg'
    'lfdXNlcm5hbWVCEAoOX3NlcnZlcl9zdGF0dXNCDgoMX3NlcnZlcl90eXBlQhgKFl9lbmFibGVf'
    'ZG9ja2VyX2NsZWFudXBCEwoRX2xvZ19jbGVhbnVwX2Nyb25CCgoIX2NvbW1hbmRCEQoPX21ldH'
    'JpY3NfY29uZmlnQg0KC19zc2hfa2V5X2lkQhUKE19idWlsZF9tZW1vcnlfbGltaXRCEgoQX2J1'
    'aWxkX2NwdV9saW1pdA==');

@$core.Deprecated('Use remoteServerUpdateResponseDescriptor instead')
const RemoteServerUpdateResponse$json = {
  '1': 'RemoteServerUpdateResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
  ],
};

/// Descriptor for `RemoteServerUpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerUpdateResponseDescriptor =
    $convert.base64Decode(
        'ChpSZW1vdGVTZXJ2ZXJVcGRhdGVSZXNwb25zZRI2CgZzZXJ2ZXIYASABKAsyHi5vcGVub3hpZG'
        'UuYXBpLnYxLlJlbW90ZVNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use remoteServerActivateRequestDescriptor instead')
const RemoteServerActivateRequest$json = {
  '1': 'RemoteServerActivateRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoteServerActivateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerActivateRequestDescriptor =
    $convert.base64Decode(
        'ChtSZW1vdGVTZXJ2ZXJBY3RpdmF0ZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use remoteServerActivateResponseDescriptor instead')
const RemoteServerActivateResponse$json = {
  '1': 'RemoteServerActivateResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
    {
      '1': 'action',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `RemoteServerActivateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerActivateResponseDescriptor =
    $convert.base64Decode(
        'ChxSZW1vdGVTZXJ2ZXJBY3RpdmF0ZVJlc3BvbnNlEjYKBnNlcnZlchgBIAEoCzIeLm9wZW5veG'
        'lkZS5hcGkudjEuUmVtb3RlU2VydmVyUgZzZXJ2ZXISPAoGYWN0aW9uGAIgASgOMiQub3Blbm94'
        'aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJBY3Rpb25SBmFjdGlvbg==');

@$core.Deprecated('Use remoteServerDeactivateRequestDescriptor instead')
const RemoteServerDeactivateRequest$json = {
  '1': 'RemoteServerDeactivateRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoteServerDeactivateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerDeactivateRequestDescriptor =
    $convert.base64Decode(
        'Ch1SZW1vdGVTZXJ2ZXJEZWFjdGl2YXRlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use remoteServerDeactivateResponseDescriptor instead')
const RemoteServerDeactivateResponse$json = {
  '1': 'RemoteServerDeactivateResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
    {
      '1': 'action',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `RemoteServerDeactivateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerDeactivateResponseDescriptor =
    $convert.base64Decode(
        'Ch5SZW1vdGVTZXJ2ZXJEZWFjdGl2YXRlUmVzcG9uc2USNgoGc2VydmVyGAEgASgLMh4ub3Blbm'
        '94aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJSBnNlcnZlchI8CgZhY3Rpb24YAiABKA4yJC5vcGVu'
        'b3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlckFjdGlvblIGYWN0aW9u');

@$core.Deprecated('Use remoteServerTestConnectionRequestDescriptor instead')
const RemoteServerTestConnectionRequest$json = {
  '1': 'RemoteServerTestConnectionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoteServerTestConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerTestConnectionRequestDescriptor =
    $convert.base64Decode(
        'CiFSZW1vdGVTZXJ2ZXJUZXN0Q29ubmVjdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use remoteServerTestConnectionResponseDescriptor instead')
const RemoteServerTestConnectionResponse$json = {
  '1': 'RemoteServerTestConnectionResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServer',
      '10': 'server'
    },
    {
      '1': 'action',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.RemoteServerAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `RemoteServerTestConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerTestConnectionResponseDescriptor =
    $convert.base64Decode(
        'CiJSZW1vdGVTZXJ2ZXJUZXN0Q29ubmVjdGlvblJlc3BvbnNlEjYKBnNlcnZlchgBIAEoCzIeLm'
        '9wZW5veGlkZS5hcGkudjEuUmVtb3RlU2VydmVyUgZzZXJ2ZXISPAoGYWN0aW9uGAIgASgOMiQu'
        'b3Blbm94aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJBY3Rpb25SBmFjdGlvbg==');

@$core.Deprecated('Use remoteServerDeleteRequestDescriptor instead')
const RemoteServerDeleteRequest$json = {
  '1': 'RemoteServerDeleteRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoteServerDeleteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerDeleteRequestDescriptor =
    $convert.base64Decode(
        'ChlSZW1vdGVTZXJ2ZXJEZWxldGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core
    .Deprecated('Use remoteServerMigrateDependenciesRequestDescriptor instead')
const RemoteServerMigrateDependenciesRequest$json = {
  '1': 'RemoteServerMigrateDependenciesRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'target_server_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'targetServerId'
    },
  ],
};

/// Descriptor for `RemoteServerMigrateDependenciesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerMigrateDependenciesRequestDescriptor =
    $convert.base64Decode(
        'CiZSZW1vdGVTZXJ2ZXJNaWdyYXRlRGVwZW5kZW5jaWVzUmVxdWVzdBIXCgJpZBgBIAEoA0IHuk'
        'gEIgIgAFICaWQSMQoQdGFyZ2V0X3NlcnZlcl9pZBgCIAEoA0IHukgEIgIgAFIOdGFyZ2V0U2Vy'
        'dmVySWQ=');

@$core
    .Deprecated('Use remoteServerMigrateDependenciesResponseDescriptor instead')
const RemoteServerMigrateDependenciesResponse$json = {
  '1': 'RemoteServerMigrateDependenciesResponse',
  '2': [
    {
      '1': 'migration',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerDependencyMigration',
      '10': 'migration'
    },
  ],
};

/// Descriptor for `RemoteServerMigrateDependenciesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerMigrateDependenciesResponseDescriptor =
    $convert.base64Decode(
        'CidSZW1vdGVTZXJ2ZXJNaWdyYXRlRGVwZW5kZW5jaWVzUmVzcG9uc2USTwoJbWlncmF0aW9uGA'
        'EgASgLMjEub3Blbm94aWRlLmFwaS52MS5SZW1vdGVTZXJ2ZXJEZXBlbmRlbmN5TWlncmF0aW9u'
        'UgltaWdyYXRpb24=');

@$core.Deprecated('Use remoteServerGetMigrationRequestDescriptor instead')
const RemoteServerGetMigrationRequest$json = {
  '1': 'RemoteServerGetMigrationRequest',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'migrationId'},
  ],
};

/// Descriptor for `RemoteServerGetMigrationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerGetMigrationRequestDescriptor =
    $convert.base64Decode(
        'Ch9SZW1vdGVTZXJ2ZXJHZXRNaWdyYXRpb25SZXF1ZXN0EioKDG1pZ3JhdGlvbl9pZBgBIAEoCU'
        'IHukgEcgIQAVILbWlncmF0aW9uSWQ=');

@$core.Deprecated('Use remoteServerGetMigrationResponseDescriptor instead')
const RemoteServerGetMigrationResponse$json = {
  '1': 'RemoteServerGetMigrationResponse',
  '2': [
    {
      '1': 'migration',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerDependencyMigration',
      '10': 'migration'
    },
  ],
};

/// Descriptor for `RemoteServerGetMigrationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerGetMigrationResponseDescriptor =
    $convert.base64Decode(
        'CiBSZW1vdGVTZXJ2ZXJHZXRNaWdyYXRpb25SZXNwb25zZRJPCgltaWdyYXRpb24YASABKAsyMS'
        '5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlckRlcGVuZGVuY3lNaWdyYXRpb25SCW1pZ3Jh'
        'dGlvbg==');

@$core.Deprecated('Use remoteServerRollbackMigrationRequestDescriptor instead')
const RemoteServerRollbackMigrationRequest$json = {
  '1': 'RemoteServerRollbackMigrationRequest',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'migrationId'},
  ],
};

/// Descriptor for `RemoteServerRollbackMigrationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerRollbackMigrationRequestDescriptor =
    $convert.base64Decode(
        'CiRSZW1vdGVTZXJ2ZXJSb2xsYmFja01pZ3JhdGlvblJlcXVlc3QSKgoMbWlncmF0aW9uX2lkGA'
        'EgASgJQge6SARyAhABUgttaWdyYXRpb25JZA==');

@$core.Deprecated('Use remoteServerRollbackMigrationResponseDescriptor instead')
const RemoteServerRollbackMigrationResponse$json = {
  '1': 'RemoteServerRollbackMigrationResponse',
  '2': [
    {
      '1': 'migration',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerDependencyMigration',
      '10': 'migration'
    },
  ],
};

/// Descriptor for `RemoteServerRollbackMigrationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerRollbackMigrationResponseDescriptor =
    $convert.base64Decode(
        'CiVSZW1vdGVTZXJ2ZXJSb2xsYmFja01pZ3JhdGlvblJlc3BvbnNlEk8KCW1pZ3JhdGlvbhgBIA'
        'EoCzIxLm9wZW5veGlkZS5hcGkudjEuUmVtb3RlU2VydmVyRGVwZW5kZW5jeU1pZ3JhdGlvblIJ'
        'bWlncmF0aW9u');

@$core.Deprecated('Use remoteServerCancelMigrationRequestDescriptor instead')
const RemoteServerCancelMigrationRequest$json = {
  '1': 'RemoteServerCancelMigrationRequest',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'migrationId'},
  ],
};

/// Descriptor for `RemoteServerCancelMigrationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerCancelMigrationRequestDescriptor =
    $convert.base64Decode(
        'CiJSZW1vdGVTZXJ2ZXJDYW5jZWxNaWdyYXRpb25SZXF1ZXN0EioKDG1pZ3JhdGlvbl9pZBgBIA'
        'EoCUIHukgEcgIQAVILbWlncmF0aW9uSWQ=');

@$core.Deprecated('Use remoteServerCancelMigrationResponseDescriptor instead')
const RemoteServerCancelMigrationResponse$json = {
  '1': 'RemoteServerCancelMigrationResponse',
  '2': [
    {
      '1': 'migration',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerApplicationMigration',
      '10': 'migration'
    },
  ],
};

/// Descriptor for `RemoteServerCancelMigrationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerCancelMigrationResponseDescriptor =
    $convert.base64Decode(
        'CiNSZW1vdGVTZXJ2ZXJDYW5jZWxNaWdyYXRpb25SZXNwb25zZRJQCgltaWdyYXRpb24YASABKA'
        'syMi5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlckFwcGxpY2F0aW9uTWlncmF0aW9uUglt'
        'aWdyYXRpb24=');

@$core.Deprecated('Use remoteServerMigrateServiceRequestDescriptor instead')
const RemoteServerMigrateServiceRequest$json = {
  '1': 'RemoteServerMigrateServiceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'target_server_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'targetServerId'
    },
  ],
};

/// Descriptor for `RemoteServerMigrateServiceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerMigrateServiceRequestDescriptor =
    $convert.base64Decode(
        'CiFSZW1vdGVTZXJ2ZXJNaWdyYXRlU2VydmljZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlkEjEKEHRhcmdldF9zZXJ2ZXJfaWQYAiABKANCB7pIBCICIABSDnRhcmdldFNlcnZlcklk');

@$core.Deprecated('Use remoteServerMigrateServiceResponseDescriptor instead')
const RemoteServerMigrateServiceResponse$json = {
  '1': 'RemoteServerMigrateServiceResponse',
  '2': [
    {
      '1': 'migration',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerApplicationMigration',
      '10': 'migration'
    },
  ],
};

/// Descriptor for `RemoteServerMigrateServiceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerMigrateServiceResponseDescriptor =
    $convert.base64Decode(
        'CiJSZW1vdGVTZXJ2ZXJNaWdyYXRlU2VydmljZVJlc3BvbnNlElAKCW1pZ3JhdGlvbhgBIAEoCz'
        'IyLm9wZW5veGlkZS5hcGkudjEuUmVtb3RlU2VydmVyQXBwbGljYXRpb25NaWdyYXRpb25SCW1p'
        'Z3JhdGlvbg==');

@$core.Deprecated('Use remoteServerListMigrationsRequestDescriptor instead')
const RemoteServerListMigrationsRequest$json = {
  '1': 'RemoteServerListMigrationsRequest',
  '2': [
    {'1': 'service_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serviceId'},
  ],
};

/// Descriptor for `RemoteServerListMigrationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListMigrationsRequestDescriptor =
    $convert.base64Decode(
        'CiFSZW1vdGVTZXJ2ZXJMaXN0TWlncmF0aW9uc1JlcXVlc3QSJgoKc2VydmljZV9pZBgBIAEoA0'
        'IHukgEIgIgAFIJc2VydmljZUlk');

@$core.Deprecated('Use remoteServerListMigrationsResponseDescriptor instead')
const RemoteServerListMigrationsResponse$json = {
  '1': 'RemoteServerListMigrationsResponse',
  '2': [
    {
      '1': 'migrations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerApplicationMigration',
      '10': 'migrations'
    },
  ],
};

/// Descriptor for `RemoteServerListMigrationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListMigrationsResponseDescriptor =
    $convert.base64Decode(
        'CiJSZW1vdGVTZXJ2ZXJMaXN0TWlncmF0aW9uc1Jlc3BvbnNlElIKCm1pZ3JhdGlvbnMYASADKA'
        'syMi5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlckFwcGxpY2F0aW9uTWlncmF0aW9uUgpt'
        'aWdyYXRpb25z');

@$core
    .Deprecated('Use remoteServerListMigrationObjectsRequestDescriptor instead')
const RemoteServerListMigrationObjectsRequest$json = {
  '1': 'RemoteServerListMigrationObjectsRequest',
  '2': [
    {'1': 'migration_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'migrationId'},
  ],
};

/// Descriptor for `RemoteServerListMigrationObjectsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListMigrationObjectsRequestDescriptor =
    $convert.base64Decode(
        'CidSZW1vdGVTZXJ2ZXJMaXN0TWlncmF0aW9uT2JqZWN0c1JlcXVlc3QSKgoMbWlncmF0aW9uX2'
        'lkGAEgASgJQge6SARyAhABUgttaWdyYXRpb25JZA==');

@$core.Deprecated(
    'Use remoteServerListMigrationObjectsResponseDescriptor instead')
const RemoteServerListMigrationObjectsResponse$json = {
  '1': 'RemoteServerListMigrationObjectsResponse',
  '2': [
    {
      '1': 'objects',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.RemoteServerMigrationObject',
      '10': 'objects'
    },
  ],
};

/// Descriptor for `RemoteServerListMigrationObjectsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List remoteServerListMigrationObjectsResponseDescriptor =
    $convert.base64Decode(
        'CihSZW1vdGVTZXJ2ZXJMaXN0TWlncmF0aW9uT2JqZWN0c1Jlc3BvbnNlEkcKB29iamVjdHMYAS'
        'ADKAsyLS5vcGVub3hpZGUuYXBpLnYxLlJlbW90ZVNlcnZlck1pZ3JhdGlvbk9iamVjdFIHb2Jq'
        'ZWN0cw==');

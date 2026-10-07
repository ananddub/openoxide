// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_management.proto.

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

@$core.Deprecated('Use serverCleanupStatusDescriptor instead')
const ServerCleanupStatus$json = {
  '1': 'ServerCleanupStatus',
  '2': [
    {'1': 'SERVER_CLEANUP_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_CLEANUP_STATUS_SUCCEEDED', '2': 1},
    {'1': 'SERVER_CLEANUP_STATUS_FAILED', '2': 2},
  ],
};

/// Descriptor for `ServerCleanupStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverCleanupStatusDescriptor = $convert.base64Decode(
    'ChNTZXJ2ZXJDbGVhbnVwU3RhdHVzEiUKIVNFUlZFUl9DTEVBTlVQX1NUQVRVU19VTlNQRUNJRk'
    'lFRBAAEiMKH1NFUlZFUl9DTEVBTlVQX1NUQVRVU19TVUNDRUVERUQQARIgChxTRVJWRVJfQ0xF'
    'QU5VUF9TVEFUVVNfRkFJTEVEEAI=');

@$core.Deprecated('Use serverCleanupPolicyDescriptor instead')
const ServerCleanupPolicy$json = {
  '1': 'ServerCleanupPolicy',
  '2': [
    {'1': 'containers', '3': 1, '4': 1, '5': 8, '10': 'containers'},
    {'1': 'images', '3': 2, '4': 1, '5': 8, '10': 'images'},
    {'1': 'networks', '3': 3, '4': 1, '5': 8, '10': 'networks'},
    {'1': 'volumes', '3': 4, '4': 1, '5': 8, '10': 'volumes'},
    {'1': 'packages', '3': 5, '4': 1, '5': 8, '10': 'packages'},
  ],
};

/// Descriptor for `ServerCleanupPolicy`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCleanupPolicyDescriptor = $convert.base64Decode(
    'ChNTZXJ2ZXJDbGVhbnVwUG9saWN5Eh4KCmNvbnRhaW5lcnMYASABKAhSCmNvbnRhaW5lcnMSFg'
    'oGaW1hZ2VzGAIgASgIUgZpbWFnZXMSGgoIbmV0d29ya3MYAyABKAhSCG5ldHdvcmtzEhgKB3Zv'
    'bHVtZXMYBCABKAhSB3ZvbHVtZXMSGgoIcGFja2FnZXMYBSABKAhSCHBhY2thZ2Vz');

@$core.Deprecated('Use serverManagementDescriptor instead')
const ServerManagement$json = {
  '1': 'ServerManagement',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '10': 'serverId'},
    {'1': 'maintenance_mode', '3': 2, '4': 1, '5': 8, '10': 'maintenanceMode'},
    {
      '1': 'maintenance_message',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'maintenanceMessage',
      '17': true
    },
    {
      '1': 'labels',
      '3': 4,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerManagement.LabelsEntry',
      '10': 'labels'
    },
    {
      '1': 'cleanup_policy',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerCleanupPolicy',
      '10': 'cleanupPolicy'
    },
    {'1': 'gpu_enabled', '3': 6, '4': 1, '5': 8, '10': 'gpuEnabled'},
    {'1': 'updated_at', '3': 7, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '3': [ServerManagement_LabelsEntry$json],
  '8': [
    {'1': '_maintenance_message'},
  ],
};

@$core.Deprecated('Use serverManagementDescriptor instead')
const ServerManagement_LabelsEntry$json = {
  '1': 'LabelsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `ServerManagement`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementDescriptor = $convert.base64Decode(
    'ChBTZXJ2ZXJNYW5hZ2VtZW50EhsKCXNlcnZlcl9pZBgBIAEoA1IIc2VydmVySWQSKQoQbWFpbn'
    'RlbmFuY2VfbW9kZRgCIAEoCFIPbWFpbnRlbmFuY2VNb2RlEjQKE21haW50ZW5hbmNlX21lc3Nh'
    'Z2UYAyABKAlIAFISbWFpbnRlbmFuY2VNZXNzYWdliAEBEkYKBmxhYmVscxgEIAMoCzIuLm9wZW'
    '5veGlkZS5hcGkudjEuU2VydmVyTWFuYWdlbWVudC5MYWJlbHNFbnRyeVIGbGFiZWxzEkwKDmNs'
    'ZWFudXBfcG9saWN5GAUgASgLMiUub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJDbGVhbnVwUG9saW'
    'N5Ug1jbGVhbnVwUG9saWN5Eh8KC2dwdV9lbmFibGVkGAYgASgIUgpncHVFbmFibGVkEh0KCnVw'
    'ZGF0ZWRfYXQYByABKANSCXVwZGF0ZWRBdBo5CgtMYWJlbHNFbnRyeRIQCgNrZXkYASABKAlSA2'
    'tleRIUCgV2YWx1ZRgCIAEoCVIFdmFsdWU6AjgBQhYKFF9tYWludGVuYW5jZV9tZXNzYWdl');

@$core.Deprecated('Use serverActionResultDescriptor instead')
const ServerActionResult$json = {
  '1': 'ServerActionResult',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'stdout', '3': 2, '4': 1, '5': 9, '10': 'stdout'},
    {'1': 'stderr', '3': 3, '4': 1, '5': 9, '10': 'stderr'},
  ],
};

/// Descriptor for `ServerActionResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverActionResultDescriptor = $convert.base64Decode(
    'ChJTZXJ2ZXJBY3Rpb25SZXN1bHQSGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2VzcxIWCgZzdGRvdX'
    'QYAiABKAlSBnN0ZG91dBIWCgZzdGRlcnIYAyABKAlSBnN0ZGVycg==');

@$core.Deprecated('Use serverBackupDescriptor instead')
const ServerBackup$json = {
  '1': 'ServerBackup',
  '2': [
    {'1': 'remote_path', '3': 1, '4': 1, '5': 9, '10': 'remotePath'},
    {'1': 'created_at', '3': 2, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `ServerBackup`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverBackupDescriptor = $convert.base64Decode(
    'CgxTZXJ2ZXJCYWNrdXASHwoLcmVtb3RlX3BhdGgYASABKAlSCnJlbW90ZVBhdGgSHQoKY3JlYX'
    'RlZF9hdBgCIAEoA1IJY3JlYXRlZEF0');

@$core.Deprecated('Use serverCleanupExecutionDescriptor instead')
const ServerCleanupExecution$json = {
  '1': 'ServerCleanupExecution',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'server_id', '3': 2, '4': 1, '5': 3, '10': 'serverId'},
    {
      '1': 'status',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerCleanupStatus',
      '10': 'status'
    },
    {'1': 'policy', '3': 4, '4': 1, '5': 9, '10': 'policy'},
    {'1': 'stdout', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'stdout', '17': true},
    {'1': 'stderr', '3': 6, '4': 1, '5': 9, '9': 1, '10': 'stderr', '17': true},
    {'1': 'started_at', '3': 7, '4': 1, '5': 3, '10': 'startedAt'},
    {
      '1': 'finished_at',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'finishedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_stdout'},
    {'1': '_stderr'},
    {'1': '_finished_at'},
  ],
};

/// Descriptor for `ServerCleanupExecution`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCleanupExecutionDescriptor = $convert.base64Decode(
    'ChZTZXJ2ZXJDbGVhbnVwRXhlY3V0aW9uEg4KAmlkGAEgASgDUgJpZBIbCglzZXJ2ZXJfaWQYAi'
    'ABKANSCHNlcnZlcklkEj0KBnN0YXR1cxgDIAEoDjIlLm9wZW5veGlkZS5hcGkudjEuU2VydmVy'
    'Q2xlYW51cFN0YXR1c1IGc3RhdHVzEhYKBnBvbGljeRgEIAEoCVIGcG9saWN5EhsKBnN0ZG91dB'
    'gFIAEoCUgAUgZzdGRvdXSIAQESGwoGc3RkZXJyGAYgASgJSAFSBnN0ZGVycogBARIdCgpzdGFy'
    'dGVkX2F0GAcgASgDUglzdGFydGVkQXQSJAoLZmluaXNoZWRfYXQYCCABKANIAlIKZmluaXNoZW'
    'RBdIgBAUIJCgdfc3Rkb3V0QgkKB19zdGRlcnJCDgoMX2ZpbmlzaGVkX2F0');

@$core.Deprecated('Use serverManagementGetRequestDescriptor instead')
const ServerManagementGetRequest$json = {
  '1': 'ServerManagementGetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementGetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementGetRequestDescriptor =
    $convert.base64Decode(
        'ChpTZXJ2ZXJNYW5hZ2VtZW50R2V0UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use serverManagementGetResponseDescriptor instead')
const ServerManagementGetResponse$json = {
  '1': 'ServerManagementGetResponse',
  '2': [
    {
      '1': 'management',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerManagement',
      '10': 'management'
    },
  ],
};

/// Descriptor for `ServerManagementGetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementGetResponseDescriptor =
    $convert.base64Decode(
        'ChtTZXJ2ZXJNYW5hZ2VtZW50R2V0UmVzcG9uc2USQgoKbWFuYWdlbWVudBgBIAEoCzIiLm9wZW'
        '5veGlkZS5hcGkudjEuU2VydmVyTWFuYWdlbWVudFIKbWFuYWdlbWVudA==');

@$core.Deprecated('Use serverManagementUpdateRequestDescriptor instead')
const ServerManagementUpdateRequest$json = {
  '1': 'ServerManagementUpdateRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'maintenance_mode', '3': 2, '4': 1, '5': 8, '10': 'maintenanceMode'},
    {
      '1': 'maintenance_message',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'maintenanceMessage',
      '17': true
    },
    {
      '1': 'labels',
      '3': 4,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerManagementUpdateRequest.LabelsEntry',
      '8': {},
      '10': 'labels'
    },
    {
      '1': 'cleanup_policy',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerCleanupPolicy',
      '10': 'cleanupPolicy'
    },
    {'1': 'gpu_enabled', '3': 6, '4': 1, '5': 8, '10': 'gpuEnabled'},
  ],
  '3': [ServerManagementUpdateRequest_LabelsEntry$json],
  '8': [
    {'1': '_maintenance_message'},
  ],
};

@$core.Deprecated('Use serverManagementUpdateRequestDescriptor instead')
const ServerManagementUpdateRequest_LabelsEntry$json = {
  '1': 'LabelsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `ServerManagementUpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementUpdateRequestDescriptor = $convert.base64Decode(
    'Ch1TZXJ2ZXJNYW5hZ2VtZW50VXBkYXRlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
    'QSKQoQbWFpbnRlbmFuY2VfbW9kZRgCIAEoCFIPbWFpbnRlbmFuY2VNb2RlEj4KE21haW50ZW5h'
    'bmNlX21lc3NhZ2UYAyABKAlCCLpIBXIDGOgHSABSEm1haW50ZW5hbmNlTWVzc2FnZYgBARJdCg'
    'ZsYWJlbHMYBCADKAsyOy5vcGVub3hpZGUuYXBpLnYxLlNlcnZlck1hbmFnZW1lbnRVcGRhdGVS'
    'ZXF1ZXN0LkxhYmVsc0VudHJ5Qgi6SAWaAQIQQFIGbGFiZWxzEkwKDmNsZWFudXBfcG9saWN5GA'
    'UgASgLMiUub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJDbGVhbnVwUG9saWN5Ug1jbGVhbnVwUG9s'
    'aWN5Eh8KC2dwdV9lbmFibGVkGAYgASgIUgpncHVFbmFibGVkGjkKC0xhYmVsc0VudHJ5EhAKA2'
    'tleRgBIAEoCVIDa2V5EhQKBXZhbHVlGAIgASgJUgV2YWx1ZToCOAFCFgoUX21haW50ZW5hbmNl'
    'X21lc3NhZ2U=');

@$core.Deprecated('Use serverManagementUpdateResponseDescriptor instead')
const ServerManagementUpdateResponse$json = {
  '1': 'ServerManagementUpdateResponse',
  '2': [
    {
      '1': 'management',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerManagement',
      '10': 'management'
    },
  ],
};

/// Descriptor for `ServerManagementUpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementUpdateResponseDescriptor =
    $convert.base64Decode(
        'Ch5TZXJ2ZXJNYW5hZ2VtZW50VXBkYXRlUmVzcG9uc2USQgoKbWFuYWdlbWVudBgBIAEoCzIiLm'
        '9wZW5veGlkZS5hcGkudjEuU2VydmVyTWFuYWdlbWVudFIKbWFuYWdlbWVudA==');

@$core.Deprecated('Use serverManagementRepairAuditRequestDescriptor instead')
const ServerManagementRepairAuditRequest$json = {
  '1': 'ServerManagementRepairAuditRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementRepairAuditRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementRepairAuditRequestDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJNYW5hZ2VtZW50UmVwYWlyQXVkaXRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
        'AAUgJpZA==');

@$core.Deprecated('Use serverManagementRepairAuditResponseDescriptor instead')
const ServerManagementRepairAuditResponse$json = {
  '1': 'ServerManagementRepairAuditResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerActionResult',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ServerManagementRepairAuditResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementRepairAuditResponseDescriptor =
    $convert.base64Decode(
        'CiNTZXJ2ZXJNYW5hZ2VtZW50UmVwYWlyQXVkaXRSZXNwb25zZRI8CgZyZXN1bHQYASABKAsyJC'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlckFjdGlvblJlc3VsdFIGcmVzdWx0');

@$core.Deprecated('Use serverManagementConfigureGpuRequestDescriptor instead')
const ServerManagementConfigureGpuRequest$json = {
  '1': 'ServerManagementConfigureGpuRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementConfigureGpuRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementConfigureGpuRequestDescriptor =
    $convert.base64Decode(
        'CiNTZXJ2ZXJNYW5hZ2VtZW50Q29uZmlndXJlR3B1UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIg'
        'IgAFICaWQ=');

@$core.Deprecated('Use serverManagementConfigureGpuResponseDescriptor instead')
const ServerManagementConfigureGpuResponse$json = {
  '1': 'ServerManagementConfigureGpuResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerActionResult',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ServerManagementConfigureGpuResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementConfigureGpuResponseDescriptor =
    $convert.base64Decode(
        'CiRTZXJ2ZXJNYW5hZ2VtZW50Q29uZmlndXJlR3B1UmVzcG9uc2USPAoGcmVzdWx0GAEgASgLMi'
        'Qub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJBY3Rpb25SZXN1bHRSBnJlc3VsdA==');

@$core.Deprecated('Use serverManagementRunCleanupRequestDescriptor instead')
const ServerManagementRunCleanupRequest$json = {
  '1': 'ServerManagementRunCleanupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementRunCleanupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementRunCleanupRequestDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJNYW5hZ2VtZW50UnVuQ2xlYW51cFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use serverManagementRunCleanupResponseDescriptor instead')
const ServerManagementRunCleanupResponse$json = {
  '1': 'ServerManagementRunCleanupResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerActionResult',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ServerManagementRunCleanupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementRunCleanupResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJNYW5hZ2VtZW50UnVuQ2xlYW51cFJlc3BvbnNlEjwKBnJlc3VsdBgBIAEoCzIkLm'
        '9wZW5veGlkZS5hcGkudjEuU2VydmVyQWN0aW9uUmVzdWx0UgZyZXN1bHQ=');

@$core.Deprecated(
    'Use serverManagementListCleanupHistoryRequestDescriptor instead')
const ServerManagementListCleanupHistoryRequest$json = {
  '1': 'ServerManagementListCleanupHistoryRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `ServerManagementListCleanupHistoryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    serverManagementListCleanupHistoryRequestDescriptor = $convert.base64Decode(
        'CilTZXJ2ZXJNYW5hZ2VtZW50TGlzdENsZWFudXBIaXN0b3J5UmVxdWVzdBIkCglzZXJ2ZXJfaW'
        'QYASABKANCB7pIBCICIABSCHNlcnZlcklk');

@$core.Deprecated(
    'Use serverManagementListCleanupHistoryResponseDescriptor instead')
const ServerManagementListCleanupHistoryResponse$json = {
  '1': 'ServerManagementListCleanupHistoryResponse',
  '2': [
    {
      '1': 'executions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerCleanupExecution',
      '10': 'executions'
    },
  ],
};

/// Descriptor for `ServerManagementListCleanupHistoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    serverManagementListCleanupHistoryResponseDescriptor =
    $convert.base64Decode(
        'CipTZXJ2ZXJNYW5hZ2VtZW50TGlzdENsZWFudXBIaXN0b3J5UmVzcG9uc2USSAoKZXhlY3V0aW'
        '9ucxgBIAMoCzIoLm9wZW5veGlkZS5hcGkudjEuU2VydmVyQ2xlYW51cEV4ZWN1dGlvblIKZXhl'
        'Y3V0aW9ucw==');

@$core.Deprecated('Use serverManagementUpgradeRequestDescriptor instead')
const ServerManagementUpgradeRequest$json = {
  '1': 'ServerManagementUpgradeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementUpgradeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementUpgradeRequestDescriptor =
    $convert.base64Decode(
        'Ch5TZXJ2ZXJNYW5hZ2VtZW50VXBncmFkZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
        'lk');

@$core.Deprecated('Use serverManagementUpgradeResponseDescriptor instead')
const ServerManagementUpgradeResponse$json = {
  '1': 'ServerManagementUpgradeResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerActionResult',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ServerManagementUpgradeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementUpgradeResponseDescriptor =
    $convert.base64Decode(
        'Ch9TZXJ2ZXJNYW5hZ2VtZW50VXBncmFkZVJlc3BvbnNlEjwKBnJlc3VsdBgBIAEoCzIkLm9wZW'
        '5veGlkZS5hcGkudjEuU2VydmVyQWN0aW9uUmVzdWx0UgZyZXN1bHQ=');

@$core.Deprecated('Use serverManagementBackupRequestDescriptor instead')
const ServerManagementBackupRequest$json = {
  '1': 'ServerManagementBackupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementBackupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementBackupRequestDescriptor =
    $convert.base64Decode(
        'Ch1TZXJ2ZXJNYW5hZ2VtZW50QmFja3VwUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use serverManagementBackupResponseDescriptor instead')
const ServerManagementBackupResponse$json = {
  '1': 'ServerManagementBackupResponse',
  '2': [
    {
      '1': 'backup',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerBackup',
      '10': 'backup'
    },
  ],
};

/// Descriptor for `ServerManagementBackupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementBackupResponseDescriptor =
    $convert.base64Decode(
        'Ch5TZXJ2ZXJNYW5hZ2VtZW50QmFja3VwUmVzcG9uc2USNgoGYmFja3VwGAEgASgLMh4ub3Blbm'
        '94aWRlLmFwaS52MS5TZXJ2ZXJCYWNrdXBSBmJhY2t1cA==');

@$core.Deprecated('Use serverManagementDiagnosticsRequestDescriptor instead')
const ServerManagementDiagnosticsRequest$json = {
  '1': 'ServerManagementDiagnosticsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerManagementDiagnosticsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementDiagnosticsRequestDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJNYW5hZ2VtZW50RGlhZ25vc3RpY3NSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
        'AAUgJpZA==');

@$core.Deprecated('Use serverManagementDiagnosticsResponseDescriptor instead')
const ServerManagementDiagnosticsResponse$json = {
  '1': 'ServerManagementDiagnosticsResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerActionResult',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ServerManagementDiagnosticsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverManagementDiagnosticsResponseDescriptor =
    $convert.base64Decode(
        'CiNTZXJ2ZXJNYW5hZ2VtZW50RGlhZ25vc3RpY3NSZXNwb25zZRI8CgZyZXN1bHQYASABKAsyJC'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlckFjdGlvblJlc3VsdFIGcmVzdWx0');

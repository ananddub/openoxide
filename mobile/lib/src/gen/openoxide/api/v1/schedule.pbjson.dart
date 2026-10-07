// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/schedule.proto.

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

@$core.Deprecated('Use scheduleShellTypeDescriptor instead')
const ScheduleShellType$json = {
  '1': 'ScheduleShellType',
  '2': [
    {'1': 'SCHEDULE_SHELL_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_SHELL_TYPE_BASH', '2': 1},
    {'1': 'SCHEDULE_SHELL_TYPE_SH', '2': 2},
  ],
};

/// Descriptor for `ScheduleShellType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleShellTypeDescriptor = $convert.base64Decode(
    'ChFTY2hlZHVsZVNoZWxsVHlwZRIjCh9TQ0hFRFVMRV9TSEVMTF9UWVBFX1VOU1BFQ0lGSUVEEA'
    'ASHAoYU0NIRURVTEVfU0hFTExfVFlQRV9CQVNIEAESGgoWU0NIRURVTEVfU0hFTExfVFlQRV9T'
    'SBAC');

@$core.Deprecated('Use scheduleTypeDescriptor instead')
const ScheduleType$json = {
  '1': 'ScheduleType',
  '2': [
    {'1': 'SCHEDULE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_TYPE_APPLICATION', '2': 1},
    {'1': 'SCHEDULE_TYPE_COMPOSE', '2': 2},
    {'1': 'SCHEDULE_TYPE_DATABASE', '2': 3},
    {'1': 'SCHEDULE_TYPE_SERVER', '2': 4},
    {'1': 'SCHEDULE_TYPE_DOKPANEL_SERVER', '2': 5},
  ],
};

/// Descriptor for `ScheduleType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleTypeDescriptor = $convert.base64Decode(
    'CgxTY2hlZHVsZVR5cGUSHQoZU0NIRURVTEVfVFlQRV9VTlNQRUNJRklFRBAAEh0KGVNDSEVEVU'
    'xFX1RZUEVfQVBQTElDQVRJT04QARIZChVTQ0hFRFVMRV9UWVBFX0NPTVBPU0UQAhIaChZTQ0hF'
    'RFVMRV9UWVBFX0RBVEFCQVNFEAMSGAoUU0NIRURVTEVfVFlQRV9TRVJWRVIQBBIhCh1TQ0hFRF'
    'VMRV9UWVBFX0RPS1BBTkVMX1NFUlZFUhAF');

@$core.Deprecated('Use scheduleActionDescriptor instead')
const ScheduleAction$json = {
  '1': 'ScheduleAction',
  '2': [
    {'1': 'SCHEDULE_ACTION_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_ACTION_EXEC', '2': 1},
    {'1': 'SCHEDULE_ACTION_DEPLOY', '2': 2},
    {'1': 'SCHEDULE_ACTION_REDEPLOY', '2': 3},
    {'1': 'SCHEDULE_ACTION_REBUILD', '2': 4},
    {'1': 'SCHEDULE_ACTION_RELOAD', '2': 5},
    {'1': 'SCHEDULE_ACTION_START', '2': 6},
    {'1': 'SCHEDULE_ACTION_STOP', '2': 7},
  ],
};

/// Descriptor for `ScheduleAction`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleActionDescriptor = $convert.base64Decode(
    'Cg5TY2hlZHVsZUFjdGlvbhIfChtTQ0hFRFVMRV9BQ1RJT05fVU5TUEVDSUZJRUQQABIYChRTQ0'
    'hFRFVMRV9BQ1RJT05fRVhFQxABEhoKFlNDSEVEVUxFX0FDVElPTl9ERVBMT1kQAhIcChhTQ0hF'
    'RFVMRV9BQ1RJT05fUkVERVBMT1kQAxIbChdTQ0hFRFVMRV9BQ1RJT05fUkVCVUlMRBAEEhoKFl'
    'NDSEVEVUxFX0FDVElPTl9SRUxPQUQQBRIZChVTQ0hFRFVMRV9BQ1RJT05fU1RBUlQQBhIYChRT'
    'Q0hFRFVMRV9BQ1RJT05fU1RPUBAH');

@$core.Deprecated('Use scheduleTriggerKindDescriptor instead')
const ScheduleTriggerKind$json = {
  '1': 'ScheduleTriggerKind',
  '2': [
    {'1': 'SCHEDULE_TRIGGER_KIND_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_TRIGGER_KIND_CRON', '2': 1},
    {'1': 'SCHEDULE_TRIGGER_KIND_MANUAL', '2': 2},
    {'1': 'SCHEDULE_TRIGGER_KIND_MISSED', '2': 3},
    {'1': 'SCHEDULE_TRIGGER_KIND_RETRY', '2': 4},
  ],
};

/// Descriptor for `ScheduleTriggerKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleTriggerKindDescriptor = $convert.base64Decode(
    'ChNTY2hlZHVsZVRyaWdnZXJLaW5kEiUKIVNDSEVEVUxFX1RSSUdHRVJfS0lORF9VTlNQRUNJRk'
    'lFRBAAEh4KGlNDSEVEVUxFX1RSSUdHRVJfS0lORF9DUk9OEAESIAocU0NIRURVTEVfVFJJR0dF'
    'Ul9LSU5EX01BTlVBTBACEiAKHFNDSEVEVUxFX1RSSUdHRVJfS0lORF9NSVNTRUQQAxIfChtTQ0'
    'hFRFVMRV9UUklHR0VSX0tJTkRfUkVUUlkQBA==');

@$core.Deprecated('Use scheduleExecutionStatusDescriptor instead')
const ScheduleExecutionStatus$json = {
  '1': 'ScheduleExecutionStatus',
  '2': [
    {'1': 'SCHEDULE_EXECUTION_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_EXECUTION_STATUS_RUNNING', '2': 1},
    {'1': 'SCHEDULE_EXECUTION_STATUS_SUCCEEDED', '2': 2},
    {'1': 'SCHEDULE_EXECUTION_STATUS_FAILED', '2': 3},
    {'1': 'SCHEDULE_EXECUTION_STATUS_SKIPPED', '2': 4},
  ],
};

/// Descriptor for `ScheduleExecutionStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleExecutionStatusDescriptor = $convert.base64Decode(
    'ChdTY2hlZHVsZUV4ZWN1dGlvblN0YXR1cxIpCiVTQ0hFRFVMRV9FWEVDVVRJT05fU1RBVFVTX1'
    'VOU1BFQ0lGSUVEEAASJQohU0NIRURVTEVfRVhFQ1VUSU9OX1NUQVRVU19SVU5OSU5HEAESJwoj'
    'U0NIRURVTEVfRVhFQ1VUSU9OX1NUQVRVU19TVUNDRUVERUQQAhIkCiBTQ0hFRFVMRV9FWEVDVV'
    'RJT05fU1RBVFVTX0ZBSUxFRBADEiUKIVNDSEVEVUxFX0VYRUNVVElPTl9TVEFUVVNfU0tJUFBF'
    'RBAE');

@$core.Deprecated('Use scheduleMissedRunPolicyDescriptor instead')
const ScheduleMissedRunPolicy$json = {
  '1': 'ScheduleMissedRunPolicy',
  '2': [
    {'1': 'SCHEDULE_MISSED_RUN_POLICY_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_MISSED_RUN_POLICY_SKIP', '2': 1},
    {'1': 'SCHEDULE_MISSED_RUN_POLICY_RUN_ONCE', '2': 2},
  ],
};

/// Descriptor for `ScheduleMissedRunPolicy`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleMissedRunPolicyDescriptor = $convert.base64Decode(
    'ChdTY2hlZHVsZU1pc3NlZFJ1blBvbGljeRIqCiZTQ0hFRFVMRV9NSVNTRURfUlVOX1BPTElDWV'
    '9VTlNQRUNJRklFRBAAEiMKH1NDSEVEVUxFX01JU1NFRF9SVU5fUE9MSUNZX1NLSVAQARInCiNT'
    'Q0hFRFVMRV9NSVNTRURfUlVOX1BPTElDWV9SVU5fT05DRRAC');

@$core.Deprecated('Use scheduleConcurrencyPolicyDescriptor instead')
const ScheduleConcurrencyPolicy$json = {
  '1': 'ScheduleConcurrencyPolicy',
  '2': [
    {'1': 'SCHEDULE_CONCURRENCY_POLICY_UNSPECIFIED', '2': 0},
    {'1': 'SCHEDULE_CONCURRENCY_POLICY_SKIP', '2': 1},
    {'1': 'SCHEDULE_CONCURRENCY_POLICY_QUEUE', '2': 2},
    {'1': 'SCHEDULE_CONCURRENCY_POLICY_ALLOW', '2': 3},
  ],
};

/// Descriptor for `ScheduleConcurrencyPolicy`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List scheduleConcurrencyPolicyDescriptor = $convert.base64Decode(
    'ChlTY2hlZHVsZUNvbmN1cnJlbmN5UG9saWN5EisKJ1NDSEVEVUxFX0NPTkNVUlJFTkNZX1BPTE'
    'lDWV9VTlNQRUNJRklFRBAAEiQKIFNDSEVEVUxFX0NPTkNVUlJFTkNZX1BPTElDWV9TS0lQEAES'
    'JQohU0NIRURVTEVfQ09OQ1VSUkVOQ1lfUE9MSUNZX1FVRVVFEAISJQohU0NIRURVTEVfQ09OQ1'
    'VSUkVOQ1lfUE9MSUNZX0FMTE9XEAM=');

@$core.Deprecated('Use scheduleDescriptor instead')
const Schedule$json = {
  '1': 'Schedule',
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
    {'1': 'cron_expression', '3': 4, '4': 1, '5': 9, '10': 'cronExpression'},
    {'1': 'app_name', '3': 5, '4': 1, '5': 9, '10': 'appName'},
    {
      '1': 'service_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'shell_type',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleShellType',
      '10': 'shellType'
    },
    {
      '1': 'schedule_type',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleType',
      '10': 'scheduleType'
    },
    {
      '1': 'schedule_action',
      '3': 9,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleAction',
      '10': 'scheduleAction'
    },
    {'1': 'command', '3': 10, '4': 1, '5': 9, '10': 'command'},
    {
      '1': 'script',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'script',
      '17': true
    },
    {
      '1': 'timezone',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'timezone',
      '17': true
    },
    {'1': 'enabled', '3': 13, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'service_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '9': 6,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'database_id',
      '3': 17,
      '4': 1,
      '5': 3,
      '9': 7,
      '10': 'databaseId',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 18,
      '4': 1,
      '5': 3,
      '9': 8,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 19,
      '4': 1,
      '5': 3,
      '9': 9,
      '10': 'organizationId',
      '17': true
    },
    {'1': 'created_at', '3': 20, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 21, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_description'},
    {'1': '_service_name'},
    {'1': '_script'},
    {'1': '_timezone'},
    {'1': '_service_id'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_database_id'},
    {'1': '_server_id'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `Schedule`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleDescriptor = $convert.base64Decode(
    'CghTY2hlZHVsZRIOCgJpZBgBIAEoA1ICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIlCgtkZXNjcm'
    'lwdGlvbhgDIAEoCUgAUgtkZXNjcmlwdGlvbogBARInCg9jcm9uX2V4cHJlc3Npb24YBCABKAlS'
    'DmNyb25FeHByZXNzaW9uEhkKCGFwcF9uYW1lGAUgASgJUgdhcHBOYW1lEiYKDHNlcnZpY2Vfbm'
    'FtZRgGIAEoCUgBUgtzZXJ2aWNlTmFtZYgBARJCCgpzaGVsbF90eXBlGAcgASgOMiMub3Blbm94'
    'aWRlLmFwaS52MS5TY2hlZHVsZVNoZWxsVHlwZVIJc2hlbGxUeXBlEkMKDXNjaGVkdWxlX3R5cG'
    'UYCCABKA4yHi5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlVHlwZVIMc2NoZWR1bGVUeXBlEkkK'
    'D3NjaGVkdWxlX2FjdGlvbhgJIAEoDjIgLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVBY3Rpb2'
    '5SDnNjaGVkdWxlQWN0aW9uEhgKB2NvbW1hbmQYCiABKAlSB2NvbW1hbmQSGwoGc2NyaXB0GAsg'
    'ASgJSAJSBnNjcmlwdIgBARIfCgh0aW1lem9uZRgMIAEoCUgDUgh0aW1lem9uZYgBARIYCgdlbm'
    'FibGVkGA0gASgIUgdlbmFibGVkEiIKCnNlcnZpY2VfaWQYDiABKANIBFIJc2VydmljZUlkiAEB'
    'EioKDmFwcGxpY2F0aW9uX2lkGA8gASgDSAVSDWFwcGxpY2F0aW9uSWSIAQESIgoKY29tcG9zZV'
    '9pZBgQIAEoA0gGUgljb21wb3NlSWSIAQESJAoLZGF0YWJhc2VfaWQYESABKANIB1IKZGF0YWJh'
    'c2VJZIgBARIgCglzZXJ2ZXJfaWQYEiABKANICFIIc2VydmVySWSIAQESLAoPb3JnYW5pemF0aW'
    '9uX2lkGBMgASgDSAlSDm9yZ2FuaXphdGlvbklkiAEBEh0KCmNyZWF0ZWRfYXQYFCABKANSCWNy'
    'ZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GBUgASgDUgl1cGRhdGVkQXRCDgoMX2Rlc2NyaXB0aW9uQg'
    '8KDV9zZXJ2aWNlX25hbWVCCQoHX3NjcmlwdEILCglfdGltZXpvbmVCDQoLX3NlcnZpY2VfaWRC'
    'EQoPX2FwcGxpY2F0aW9uX2lkQg0KC19jb21wb3NlX2lkQg4KDF9kYXRhYmFzZV9pZEIMCgpfc2'
    'VydmVyX2lkQhIKEF9vcmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use scheduleRunDescriptor instead')
const ScheduleRun$json = {
  '1': 'ScheduleRun',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
    {'1': 'action', '3': 2, '4': 1, '5': 9, '10': 'action'},
    {
      '1': 'deployment_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'deploymentId',
      '17': true
    },
    {'1': 'message', '3': 4, '4': 1, '5': 9, '10': 'message'},
    {'1': 'stdout', '3': 5, '4': 1, '5': 9, '9': 1, '10': 'stdout', '17': true},
    {'1': 'stderr', '3': 6, '4': 1, '5': 9, '9': 2, '10': 'stderr', '17': true},
  ],
  '8': [
    {'1': '_deployment_id'},
    {'1': '_stdout'},
    {'1': '_stderr'},
  ],
};

/// Descriptor for `ScheduleRun`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleRunDescriptor = $convert.base64Decode(
    'CgtTY2hlZHVsZVJ1bhI2CghzY2hlZHVsZRgBIAEoCzIaLm9wZW5veGlkZS5hcGkudjEuU2NoZW'
    'R1bGVSCHNjaGVkdWxlEhYKBmFjdGlvbhgCIAEoCVIGYWN0aW9uEigKDWRlcGxveW1lbnRfaWQY'
    'AyABKANIAFIMZGVwbG95bWVudElkiAEBEhgKB21lc3NhZ2UYBCABKAlSB21lc3NhZ2USGwoGc3'
    'Rkb3V0GAUgASgJSAFSBnN0ZG91dIgBARIbCgZzdGRlcnIYBiABKAlIAlIGc3RkZXJyiAEBQhAK'
    'Dl9kZXBsb3ltZW50X2lkQgkKB19zdGRvdXRCCQoHX3N0ZGVycg==');

@$core.Deprecated('Use scheduleExecutionDescriptor instead')
const ScheduleExecution$json = {
  '1': 'ScheduleExecution',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'schedule_id', '3': 2, '4': 1, '5': 3, '10': 'scheduleId'},
    {
      '1': 'organization_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'trigger_kind',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleTriggerKind',
      '10': 'triggerKind'
    },
    {
      '1': 'status',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleExecutionStatus',
      '10': 'status'
    },
    {'1': 'attempt', '3': 6, '4': 1, '5': 3, '10': 'attempt'},
    {'1': 'scheduled_at', '3': 7, '4': 1, '5': 3, '10': 'scheduledAt'},
    {'1': 'started_at', '3': 8, '4': 1, '5': 3, '10': 'startedAt'},
    {
      '1': 'finished_at',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'finishedAt',
      '17': true
    },
    {
      '1': 'message',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'message',
      '17': true
    },
    {
      '1': 'stdout',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'stdout',
      '17': true
    },
    {
      '1': 'stderr',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'stderr',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_finished_at'},
    {'1': '_message'},
    {'1': '_stdout'},
    {'1': '_stderr'},
  ],
};

/// Descriptor for `ScheduleExecution`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleExecutionDescriptor = $convert.base64Decode(
    'ChFTY2hlZHVsZUV4ZWN1dGlvbhIOCgJpZBgBIAEoA1ICaWQSHwoLc2NoZWR1bGVfaWQYAiABKA'
    'NSCnNjaGVkdWxlSWQSLAoPb3JnYW5pemF0aW9uX2lkGAMgASgDSABSDm9yZ2FuaXphdGlvbklk'
    'iAEBEkgKDHRyaWdnZXJfa2luZBgEIAEoDjIlLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVUcm'
    'lnZ2VyS2luZFILdHJpZ2dlcktpbmQSQQoGc3RhdHVzGAUgASgOMikub3Blbm94aWRlLmFwaS52'
    'MS5TY2hlZHVsZUV4ZWN1dGlvblN0YXR1c1IGc3RhdHVzEhgKB2F0dGVtcHQYBiABKANSB2F0dG'
    'VtcHQSIQoMc2NoZWR1bGVkX2F0GAcgASgDUgtzY2hlZHVsZWRBdBIdCgpzdGFydGVkX2F0GAgg'
    'ASgDUglzdGFydGVkQXQSJAoLZmluaXNoZWRfYXQYCSABKANIAVIKZmluaXNoZWRBdIgBARIdCg'
    'dtZXNzYWdlGAogASgJSAJSB21lc3NhZ2WIAQESGwoGc3Rkb3V0GAsgASgJSANSBnN0ZG91dIgB'
    'ARIbCgZzdGRlcnIYDCABKAlIBFIGc3RkZXJyiAEBQhIKEF9vcmdhbml6YXRpb25faWRCDgoMX2'
    'ZpbmlzaGVkX2F0QgoKCF9tZXNzYWdlQgkKB19zdGRvdXRCCQoHX3N0ZGVycg==');

@$core.Deprecated('Use scheduleRuntimePolicyDescriptor instead')
const ScheduleRuntimePolicy$json = {
  '1': 'ScheduleRuntimePolicy',
  '2': [
    {'1': 'schedule_id', '3': 1, '4': 1, '5': 3, '10': 'scheduleId'},
    {'1': 'retry_count', '3': 2, '4': 1, '5': 3, '10': 'retryCount'},
    {
      '1': 'retry_delay_seconds',
      '3': 3,
      '4': 1,
      '5': 3,
      '10': 'retryDelaySeconds'
    },
    {
      '1': 'missed_run_policy',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleMissedRunPolicy',
      '10': 'missedRunPolicy'
    },
    {
      '1': 'concurrency_policy',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleConcurrencyPolicy',
      '10': 'concurrencyPolicy'
    },
    {'1': 'lease_seconds', '3': 6, '4': 1, '5': 3, '10': 'leaseSeconds'},
    {'1': 'notify_on_success', '3': 7, '4': 1, '5': 8, '10': 'notifyOnSuccess'},
    {'1': 'notify_on_failure', '3': 8, '4': 1, '5': 8, '10': 'notifyOnFailure'},
    {
      '1': 'last_scheduled_at',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'lastScheduledAt',
      '17': true
    },
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_last_scheduled_at'},
  ],
};

/// Descriptor for `ScheduleRuntimePolicy`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleRuntimePolicyDescriptor = $convert.base64Decode(
    'ChVTY2hlZHVsZVJ1bnRpbWVQb2xpY3kSHwoLc2NoZWR1bGVfaWQYASABKANSCnNjaGVkdWxlSW'
    'QSHwoLcmV0cnlfY291bnQYAiABKANSCnJldHJ5Q291bnQSLgoTcmV0cnlfZGVsYXlfc2Vjb25k'
    'cxgDIAEoA1IRcmV0cnlEZWxheVNlY29uZHMSVQoRbWlzc2VkX3J1bl9wb2xpY3kYBCABKA4yKS'
    '5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlTWlzc2VkUnVuUG9saWN5Ug9taXNzZWRSdW5Qb2xp'
    'Y3kSWgoSY29uY3VycmVuY3lfcG9saWN5GAUgASgOMisub3Blbm94aWRlLmFwaS52MS5TY2hlZH'
    'VsZUNvbmN1cnJlbmN5UG9saWN5UhFjb25jdXJyZW5jeVBvbGljeRIjCg1sZWFzZV9zZWNvbmRz'
    'GAYgASgDUgxsZWFzZVNlY29uZHMSKgoRbm90aWZ5X29uX3N1Y2Nlc3MYByABKAhSD25vdGlmeU'
    '9uU3VjY2VzcxIqChFub3RpZnlfb25fZmFpbHVyZRgIIAEoCFIPbm90aWZ5T25GYWlsdXJlEi8K'
    'EWxhc3Rfc2NoZWR1bGVkX2F0GAkgASgDSABSD2xhc3RTY2hlZHVsZWRBdIgBARIdCgp1cGRhdG'
    'VkX2F0GAogASgDUgl1cGRhdGVkQXRCFAoSX2xhc3Rfc2NoZWR1bGVkX2F0');

@$core.Deprecated('Use scheduleLogDescriptor instead')
const ScheduleLog$json = {
  '1': 'ScheduleLog',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `ScheduleLog`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleLogDescriptor = $convert
    .base64Decode('CgtTY2hlZHVsZUxvZxIYCgdjb250ZW50GAEgASgJUgdjb250ZW50');

@$core.Deprecated('Use scheduleListSchedulesRequestDescriptor instead')
const ScheduleListSchedulesRequest$json = {
  '1': 'ScheduleListSchedulesRequest',
};

/// Descriptor for `ScheduleListSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListSchedulesRequestDescriptor =
    $convert.base64Decode('ChxTY2hlZHVsZUxpc3RTY2hlZHVsZXNSZXF1ZXN0');

@$core.Deprecated('Use scheduleListSchedulesResponseDescriptor instead')
const ScheduleListSchedulesResponse$json = {
  '1': 'ScheduleListSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListSchedulesResponseDescriptor =
    $convert.base64Decode(
        'Ch1TY2hlZHVsZUxpc3RTY2hlZHVsZXNSZXNwb25zZRI4CglzY2hlZHVsZXMYASADKAsyGi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNjaGVkdWxlUglzY2hlZHVsZXM=');

@$core.Deprecated('Use scheduleGetScheduleRequestDescriptor instead')
const ScheduleGetScheduleRequest$json = {
  '1': 'ScheduleGetScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleGetScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetScheduleRequestDescriptor =
    $convert.base64Decode(
        'ChpTY2hlZHVsZUdldFNjaGVkdWxlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use scheduleGetScheduleResponseDescriptor instead')
const ScheduleGetScheduleResponse$json = {
  '1': 'ScheduleGetScheduleResponse',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
  ],
};

/// Descriptor for `ScheduleGetScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetScheduleResponseDescriptor =
    $convert.base64Decode(
        'ChtTY2hlZHVsZUdldFNjaGVkdWxlUmVzcG9uc2USNgoIc2NoZWR1bGUYASABKAsyGi5vcGVub3'
        'hpZGUuYXBpLnYxLlNjaGVkdWxlUghzY2hlZHVsZQ==');

@$core.Deprecated('Use scheduleCreateScheduleRequestDescriptor instead')
const ScheduleCreateScheduleRequest$json = {
  '1': 'ScheduleCreateScheduleRequest',
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
    {
      '1': 'cron_expression',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'cronExpression'
    },
    {
      '1': 'app_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'appName',
      '17': true
    },
    {
      '1': 'service_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'shell_type',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleShellType',
      '8': {},
      '9': 3,
      '10': 'shellType',
      '17': true
    },
    {
      '1': 'schedule_type',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleType',
      '8': {},
      '9': 4,
      '10': 'scheduleType',
      '17': true
    },
    {
      '1': 'schedule_action',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleAction',
      '8': {},
      '9': 5,
      '10': 'scheduleAction',
      '17': true
    },
    {'1': 'command', '3': 9, '4': 1, '5': 9, '8': {}, '10': 'command'},
    {
      '1': 'script',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'script',
      '17': true
    },
    {
      '1': 'timezone',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'timezone',
      '17': true
    },
    {
      '1': 'enabled',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 8,
      '10': 'enabled',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 9,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 10,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 11,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 12,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_app_name'},
    {'1': '_service_name'},
    {'1': '_shell_type'},
    {'1': '_schedule_type'},
    {'1': '_schedule_action'},
    {'1': '_script'},
    {'1': '_timezone'},
    {'1': '_enabled'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_server_id'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `ScheduleCreateScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleCreateScheduleRequestDescriptor = $convert.base64Decode(
    'Ch1TY2hlZHVsZUNyZWF0ZVNjaGVkdWxlUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP'
    '8BUgRuYW1lEi8KC2Rlc2NyaXB0aW9uGAIgASgJQgi6SAVyAxjoB0gAUgtkZXNjcmlwdGlvbogB'
    'ARIwCg9jcm9uX2V4cHJlc3Npb24YAyABKAlCB7pIBHICEAFSDmNyb25FeHByZXNzaW9uEigKCG'
    'FwcF9uYW1lGAQgASgJQgi6SAVyAxj/AUgBUgdhcHBOYW1liAEBEjAKDHNlcnZpY2VfbmFtZRgF'
    'IAEoCUIIukgFcgMY/wFIAlILc2VydmljZU5hbWWIAQESUwoKc2hlbGxfdHlwZRgGIAEoDjIjLm'
    '9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVTaGVsbFR5cGVCCrpIB4IBBBABIABIA1IJc2hlbGxU'
    'eXBliAEBElQKDXNjaGVkdWxlX3R5cGUYByABKA4yHi5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdW'
    'xlVHlwZUIKukgHggEEEAEgAEgEUgxzY2hlZHVsZVR5cGWIAQESWgoPc2NoZWR1bGVfYWN0aW9u'
    'GAggASgOMiAub3Blbm94aWRlLmFwaS52MS5TY2hlZHVsZUFjdGlvbkIKukgHggEEEAEgAEgFUg'
    '5zY2hlZHVsZUFjdGlvbogBARIhCgdjb21tYW5kGAkgASgJQge6SARyAhABUgdjb21tYW5kEiYK'
    'BnNjcmlwdBgKIAEoCUIJukgGcgQYgIBASAZSBnNjcmlwdIgBARIfCgh0aW1lem9uZRgLIAEoCU'
    'gHUgh0aW1lem9uZYgBARIdCgdlbmFibGVkGAwgASgISAhSB2VuYWJsZWSIAQESMwoOYXBwbGlj'
    'YXRpb25faWQYDSABKANCB7pIBCICIABICVINYXBwbGljYXRpb25JZIgBARIrCgpjb21wb3NlX2'
    'lkGA4gASgDQge6SAQiAiAASApSCWNvbXBvc2VJZIgBARIpCglzZXJ2ZXJfaWQYDyABKANCB7pI'
    'BCICIABIC1IIc2VydmVySWSIAQESNQoPb3JnYW5pemF0aW9uX2lkGBAgASgDQge6SAQiAiAASA'
    'xSDm9yZ2FuaXphdGlvbklkiAEBQg4KDF9kZXNjcmlwdGlvbkILCglfYXBwX25hbWVCDwoNX3Nl'
    'cnZpY2VfbmFtZUINCgtfc2hlbGxfdHlwZUIQCg5fc2NoZWR1bGVfdHlwZUISChBfc2NoZWR1bG'
    'VfYWN0aW9uQgkKB19zY3JpcHRCCwoJX3RpbWV6b25lQgoKCF9lbmFibGVkQhEKD19hcHBsaWNh'
    'dGlvbl9pZEINCgtfY29tcG9zZV9pZEIMCgpfc2VydmVyX2lkQhIKEF9vcmdhbml6YXRpb25faW'
    'Q=');

@$core.Deprecated('Use scheduleCreateScheduleResponseDescriptor instead')
const ScheduleCreateScheduleResponse$json = {
  '1': 'ScheduleCreateScheduleResponse',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
  ],
};

/// Descriptor for `ScheduleCreateScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleCreateScheduleResponseDescriptor =
    $convert.base64Decode(
        'Ch5TY2hlZHVsZUNyZWF0ZVNjaGVkdWxlUmVzcG9uc2USNgoIc2NoZWR1bGUYASABKAsyGi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNjaGVkdWxlUghzY2hlZHVsZQ==');

@$core.Deprecated('Use scheduleUpdateScheduleRequestDescriptor instead')
const ScheduleUpdateScheduleRequest$json = {
  '1': 'ScheduleUpdateScheduleRequest',
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
      '1': 'cron_expression',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'cronExpression',
      '17': true
    },
    {
      '1': 'app_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'appName',
      '17': true
    },
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
      '1': 'shell_type',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleShellType',
      '8': {},
      '9': 5,
      '10': 'shellType',
      '17': true
    },
    {
      '1': 'schedule_type',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleType',
      '8': {},
      '9': 6,
      '10': 'scheduleType',
      '17': true
    },
    {
      '1': 'schedule_action',
      '3': 9,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleAction',
      '8': {},
      '9': 7,
      '10': 'scheduleAction',
      '17': true
    },
    {
      '1': 'command',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'command',
      '17': true
    },
    {
      '1': 'script',
      '3': 11,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 9,
      '10': 'script',
      '17': true
    },
    {
      '1': 'timezone',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'timezone',
      '17': true
    },
    {
      '1': 'enabled',
      '3': 13,
      '4': 1,
      '5': 8,
      '9': 11,
      '10': 'enabled',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 12,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 13,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 14,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 17,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 15,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_cron_expression'},
    {'1': '_app_name'},
    {'1': '_service_name'},
    {'1': '_shell_type'},
    {'1': '_schedule_type'},
    {'1': '_schedule_action'},
    {'1': '_command'},
    {'1': '_script'},
    {'1': '_timezone'},
    {'1': '_enabled'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_server_id'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `ScheduleUpdateScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleUpdateScheduleRequestDescriptor = $convert.base64Decode(
    'Ch1TY2hlZHVsZVVwZGF0ZVNjaGVkdWxlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
    'QSIQoEbmFtZRgCIAEoCUIIukgFcgMY/wFIAFIEbmFtZYgBARIvCgtkZXNjcmlwdGlvbhgDIAEo'
    'CUIIukgFcgMY6AdIAVILZGVzY3JpcHRpb26IAQESLAoPY3Jvbl9leHByZXNzaW9uGAQgASgJSA'
    'JSDmNyb25FeHByZXNzaW9uiAEBEigKCGFwcF9uYW1lGAUgASgJQgi6SAVyAxj/AUgDUgdhcHBO'
    'YW1liAEBEjAKDHNlcnZpY2VfbmFtZRgGIAEoCUIIukgFcgMY/wFIBFILc2VydmljZU5hbWWIAQ'
    'ESUwoKc2hlbGxfdHlwZRgHIAEoDjIjLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVTaGVsbFR5'
    'cGVCCrpIB4IBBBABIABIBVIJc2hlbGxUeXBliAEBElQKDXNjaGVkdWxlX3R5cGUYCCABKA4yHi'
    '5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlVHlwZUIKukgHggEEEAEgAEgGUgxzY2hlZHVsZVR5'
    'cGWIAQESWgoPc2NoZWR1bGVfYWN0aW9uGAkgASgOMiAub3Blbm94aWRlLmFwaS52MS5TY2hlZH'
    'VsZUFjdGlvbkIKukgHggEEEAEgAEgHUg5zY2hlZHVsZUFjdGlvbogBARIdCgdjb21tYW5kGAog'
    'ASgJSAhSB2NvbW1hbmSIAQESJgoGc2NyaXB0GAsgASgJQgm6SAZyBBiAgEBICVIGc2NyaXB0iA'
    'EBEh8KCHRpbWV6b25lGAwgASgJSApSCHRpbWV6b25liAEBEh0KB2VuYWJsZWQYDSABKAhIC1IH'
    'ZW5hYmxlZIgBARIzCg5hcHBsaWNhdGlvbl9pZBgOIAEoA0IHukgEIgIgAEgMUg1hcHBsaWNhdG'
    'lvbklkiAEBEisKCmNvbXBvc2VfaWQYDyABKANCB7pIBCICIABIDVIJY29tcG9zZUlkiAEBEikK'
    'CXNlcnZlcl9pZBgQIAEoA0IHukgEIgIgAEgOUghzZXJ2ZXJJZIgBARI1Cg9vcmdhbml6YXRpb2'
    '5faWQYESABKANCB7pIBCICIABID1IOb3JnYW5pemF0aW9uSWSIAQFCBwoFX25hbWVCDgoMX2Rl'
    'c2NyaXB0aW9uQhIKEF9jcm9uX2V4cHJlc3Npb25CCwoJX2FwcF9uYW1lQg8KDV9zZXJ2aWNlX2'
    '5hbWVCDQoLX3NoZWxsX3R5cGVCEAoOX3NjaGVkdWxlX3R5cGVCEgoQX3NjaGVkdWxlX2FjdGlv'
    'bkIKCghfY29tbWFuZEIJCgdfc2NyaXB0QgsKCV90aW1lem9uZUIKCghfZW5hYmxlZEIRCg9fYX'
    'BwbGljYXRpb25faWRCDQoLX2NvbXBvc2VfaWRCDAoKX3NlcnZlcl9pZEISChBfb3JnYW5pemF0'
    'aW9uX2lk');

@$core.Deprecated('Use scheduleUpdateScheduleResponseDescriptor instead')
const ScheduleUpdateScheduleResponse$json = {
  '1': 'ScheduleUpdateScheduleResponse',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
  ],
};

/// Descriptor for `ScheduleUpdateScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleUpdateScheduleResponseDescriptor =
    $convert.base64Decode(
        'Ch5TY2hlZHVsZVVwZGF0ZVNjaGVkdWxlUmVzcG9uc2USNgoIc2NoZWR1bGUYASABKAsyGi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNjaGVkdWxlUghzY2hlZHVsZQ==');

@$core.Deprecated('Use scheduleDeleteScheduleRequestDescriptor instead')
const ScheduleDeleteScheduleRequest$json = {
  '1': 'ScheduleDeleteScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleDeleteScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleDeleteScheduleRequestDescriptor =
    $convert.base64Decode(
        'Ch1TY2hlZHVsZURlbGV0ZVNjaGVkdWxlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use scheduleRunScheduleRequestDescriptor instead')
const ScheduleRunScheduleRequest$json = {
  '1': 'ScheduleRunScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleRunScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleRunScheduleRequestDescriptor =
    $convert.base64Decode(
        'ChpTY2hlZHVsZVJ1blNjaGVkdWxlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use scheduleRunScheduleResponseDescriptor instead')
const ScheduleRunScheduleResponse$json = {
  '1': 'ScheduleRunScheduleResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleRun',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ScheduleRunScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleRunScheduleResponseDescriptor =
    $convert.base64Decode(
        'ChtTY2hlZHVsZVJ1blNjaGVkdWxlUmVzcG9uc2USNQoGcmVzdWx0GAEgASgLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5TY2hlZHVsZVJ1blIGcmVzdWx0');

@$core.Deprecated('Use schedulePauseScheduleRequestDescriptor instead')
const SchedulePauseScheduleRequest$json = {
  '1': 'SchedulePauseScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SchedulePauseScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schedulePauseScheduleRequestDescriptor =
    $convert.base64Decode(
        'ChxTY2hlZHVsZVBhdXNlU2NoZWR1bGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use schedulePauseScheduleResponseDescriptor instead')
const SchedulePauseScheduleResponse$json = {
  '1': 'SchedulePauseScheduleResponse',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
  ],
};

/// Descriptor for `SchedulePauseScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schedulePauseScheduleResponseDescriptor =
    $convert.base64Decode(
        'Ch1TY2hlZHVsZVBhdXNlU2NoZWR1bGVSZXNwb25zZRI2CghzY2hlZHVsZRgBIAEoCzIaLm9wZW'
        '5veGlkZS5hcGkudjEuU2NoZWR1bGVSCHNjaGVkdWxl');

@$core.Deprecated('Use scheduleResumeScheduleRequestDescriptor instead')
const ScheduleResumeScheduleRequest$json = {
  '1': 'ScheduleResumeScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleResumeScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleResumeScheduleRequestDescriptor =
    $convert.base64Decode(
        'Ch1TY2hlZHVsZVJlc3VtZVNjaGVkdWxlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use scheduleResumeScheduleResponseDescriptor instead')
const ScheduleResumeScheduleResponse$json = {
  '1': 'ScheduleResumeScheduleResponse',
  '2': [
    {
      '1': 'schedule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedule'
    },
  ],
};

/// Descriptor for `ScheduleResumeScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleResumeScheduleResponseDescriptor =
    $convert.base64Decode(
        'Ch5TY2hlZHVsZVJlc3VtZVNjaGVkdWxlUmVzcG9uc2USNgoIc2NoZWR1bGUYASABKAsyGi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNjaGVkdWxlUghzY2hlZHVsZQ==');

@$core
    .Deprecated('Use scheduleListApplicationSchedulesRequestDescriptor instead')
const ScheduleListApplicationSchedulesRequest$json = {
  '1': 'ScheduleListApplicationSchedulesRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
  ],
};

/// Descriptor for `ScheduleListApplicationSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListApplicationSchedulesRequestDescriptor =
    $convert.base64Decode(
        'CidTY2hlZHVsZUxpc3RBcHBsaWNhdGlvblNjaGVkdWxlc1JlcXVlc3QSLgoOYXBwbGljYXRpb2'
        '5faWQYASABKANCB7pIBCICIABSDWFwcGxpY2F0aW9uSWQ=');

@$core.Deprecated(
    'Use scheduleListApplicationSchedulesResponseDescriptor instead')
const ScheduleListApplicationSchedulesResponse$json = {
  '1': 'ScheduleListApplicationSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListApplicationSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListApplicationSchedulesResponseDescriptor =
    $convert.base64Decode(
        'CihTY2hlZHVsZUxpc3RBcHBsaWNhdGlvblNjaGVkdWxlc1Jlc3BvbnNlEjgKCXNjaGVkdWxlcx'
        'gBIAMoCzIaLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVSCXNjaGVkdWxlcw==');

@$core.Deprecated('Use scheduleListComposeSchedulesRequestDescriptor instead')
const ScheduleListComposeSchedulesRequest$json = {
  '1': 'ScheduleListComposeSchedulesRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
  ],
};

/// Descriptor for `ScheduleListComposeSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListComposeSchedulesRequestDescriptor =
    $convert.base64Decode(
        'CiNTY2hlZHVsZUxpc3RDb21wb3NlU2NoZWR1bGVzUmVxdWVzdBImCgpjb21wb3NlX2lkGAEgAS'
        'gDQge6SAQiAiAAUgljb21wb3NlSWQ=');

@$core.Deprecated('Use scheduleListComposeSchedulesResponseDescriptor instead')
const ScheduleListComposeSchedulesResponse$json = {
  '1': 'ScheduleListComposeSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListComposeSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListComposeSchedulesResponseDescriptor =
    $convert.base64Decode(
        'CiRTY2hlZHVsZUxpc3RDb21wb3NlU2NoZWR1bGVzUmVzcG9uc2USOAoJc2NoZWR1bGVzGAEgAy'
        'gLMhoub3Blbm94aWRlLmFwaS52MS5TY2hlZHVsZVIJc2NoZWR1bGVz');

@$core.Deprecated('Use scheduleListDatabaseSchedulesRequestDescriptor instead')
const ScheduleListDatabaseSchedulesRequest$json = {
  '1': 'ScheduleListDatabaseSchedulesRequest',
  '2': [
    {'1': 'database_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'databaseId'},
  ],
};

/// Descriptor for `ScheduleListDatabaseSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListDatabaseSchedulesRequestDescriptor =
    $convert.base64Decode(
        'CiRTY2hlZHVsZUxpc3REYXRhYmFzZVNjaGVkdWxlc1JlcXVlc3QSKAoLZGF0YWJhc2VfaWQYAS'
        'ABKANCB7pIBCICIABSCmRhdGFiYXNlSWQ=');

@$core.Deprecated('Use scheduleListDatabaseSchedulesResponseDescriptor instead')
const ScheduleListDatabaseSchedulesResponse$json = {
  '1': 'ScheduleListDatabaseSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListDatabaseSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListDatabaseSchedulesResponseDescriptor =
    $convert.base64Decode(
        'CiVTY2hlZHVsZUxpc3REYXRhYmFzZVNjaGVkdWxlc1Jlc3BvbnNlEjgKCXNjaGVkdWxlcxgBIA'
        'MoCzIaLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVSCXNjaGVkdWxlcw==');

@$core.Deprecated('Use scheduleListServerSchedulesRequestDescriptor instead')
const ScheduleListServerSchedulesRequest$json = {
  '1': 'ScheduleListServerSchedulesRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `ScheduleListServerSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListServerSchedulesRequestDescriptor =
    $convert.base64Decode(
        'CiJTY2hlZHVsZUxpc3RTZXJ2ZXJTY2hlZHVsZXNSZXF1ZXN0EiQKCXNlcnZlcl9pZBgBIAEoA0'
        'IHukgEIgIgAFIIc2VydmVySWQ=');

@$core.Deprecated('Use scheduleListServerSchedulesResponseDescriptor instead')
const ScheduleListServerSchedulesResponse$json = {
  '1': 'ScheduleListServerSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListServerSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListServerSchedulesResponseDescriptor =
    $convert.base64Decode(
        'CiNTY2hlZHVsZUxpc3RTZXJ2ZXJTY2hlZHVsZXNSZXNwb25zZRI4CglzY2hlZHVsZXMYASADKA'
        'syGi5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlUglzY2hlZHVsZXM=');

@$core.Deprecated(
    'Use scheduleListOrganizationSchedulesRequestDescriptor instead')
const ScheduleListOrganizationSchedulesRequest$json = {
  '1': 'ScheduleListOrganizationSchedulesRequest',
  '2': [
    {
      '1': 'organization_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
};

/// Descriptor for `ScheduleListOrganizationSchedulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListOrganizationSchedulesRequestDescriptor =
    $convert.base64Decode(
        'CihTY2hlZHVsZUxpc3RPcmdhbml6YXRpb25TY2hlZHVsZXNSZXF1ZXN0EjAKD29yZ2FuaXphdG'
        'lvbl9pZBgBIAEoA0IHukgEIgIgAFIOb3JnYW5pemF0aW9uSWQ=');

@$core.Deprecated(
    'Use scheduleListOrganizationSchedulesResponseDescriptor instead')
const ScheduleListOrganizationSchedulesResponse$json = {
  '1': 'ScheduleListOrganizationSchedulesResponse',
  '2': [
    {
      '1': 'schedules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Schedule',
      '10': 'schedules'
    },
  ],
};

/// Descriptor for `ScheduleListOrganizationSchedulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    scheduleListOrganizationSchedulesResponseDescriptor = $convert.base64Decode(
        'CilTY2hlZHVsZUxpc3RPcmdhbml6YXRpb25TY2hlZHVsZXNSZXNwb25zZRI4CglzY2hlZHVsZX'
        'MYASADKAsyGi5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlUglzY2hlZHVsZXM=');

@$core.Deprecated('Use scheduleTriggerScheduleRequestDescriptor instead')
const ScheduleTriggerScheduleRequest$json = {
  '1': 'ScheduleTriggerScheduleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleTriggerScheduleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleTriggerScheduleRequestDescriptor =
    $convert.base64Decode(
        'Ch5TY2hlZHVsZVRyaWdnZXJTY2hlZHVsZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
        'lk');

@$core.Deprecated('Use scheduleTriggerScheduleResponseDescriptor instead')
const ScheduleTriggerScheduleResponse$json = {
  '1': 'ScheduleTriggerScheduleResponse',
  '2': [
    {
      '1': 'result',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleRun',
      '10': 'result'
    },
  ],
};

/// Descriptor for `ScheduleTriggerScheduleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleTriggerScheduleResponseDescriptor =
    $convert.base64Decode(
        'Ch9TY2hlZHVsZVRyaWdnZXJTY2hlZHVsZVJlc3BvbnNlEjUKBnJlc3VsdBgBIAEoCzIdLm9wZW'
        '5veGlkZS5hcGkudjEuU2NoZWR1bGVSdW5SBnJlc3VsdA==');

@$core.Deprecated('Use scheduleGetRuntimePolicyRequestDescriptor instead')
const ScheduleGetRuntimePolicyRequest$json = {
  '1': 'ScheduleGetRuntimePolicyRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleGetRuntimePolicyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetRuntimePolicyRequestDescriptor =
    $convert.base64Decode(
        'Ch9TY2hlZHVsZUdldFJ1bnRpbWVQb2xpY3lSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUg'
        'JpZA==');

@$core.Deprecated('Use scheduleGetRuntimePolicyResponseDescriptor instead')
const ScheduleGetRuntimePolicyResponse$json = {
  '1': 'ScheduleGetRuntimePolicyResponse',
  '2': [
    {
      '1': 'policy',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleRuntimePolicy',
      '10': 'policy'
    },
  ],
};

/// Descriptor for `ScheduleGetRuntimePolicyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetRuntimePolicyResponseDescriptor =
    $convert.base64Decode(
        'CiBTY2hlZHVsZUdldFJ1bnRpbWVQb2xpY3lSZXNwb25zZRI/CgZwb2xpY3kYASABKAsyJy5vcG'
        'Vub3hpZGUuYXBpLnYxLlNjaGVkdWxlUnVudGltZVBvbGljeVIGcG9saWN5');

@$core.Deprecated('Use scheduleUpdateRuntimePolicyRequestDescriptor instead')
const ScheduleUpdateRuntimePolicyRequest$json = {
  '1': 'ScheduleUpdateRuntimePolicyRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'retry_count', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'retryCount'},
    {
      '1': 'retry_delay_seconds',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'retryDelaySeconds'
    },
    {
      '1': 'missed_run_policy',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleMissedRunPolicy',
      '8': {},
      '10': 'missedRunPolicy'
    },
    {
      '1': 'concurrency_policy',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ScheduleConcurrencyPolicy',
      '8': {},
      '10': 'concurrencyPolicy'
    },
    {
      '1': 'lease_seconds',
      '3': 6,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'leaseSeconds'
    },
    {'1': 'notify_on_success', '3': 7, '4': 1, '5': 8, '10': 'notifyOnSuccess'},
    {'1': 'notify_on_failure', '3': 8, '4': 1, '5': 8, '10': 'notifyOnFailure'},
  ],
};

/// Descriptor for `ScheduleUpdateRuntimePolicyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleUpdateRuntimePolicyRequestDescriptor = $convert.base64Decode(
    'CiJTY2hlZHVsZVVwZGF0ZVJ1bnRpbWVQb2xpY3lSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
    'AAUgJpZBIqCgtyZXRyeV9jb3VudBgCIAEoA0IJukgGIgQYCigAUgpyZXRyeUNvdW50EjsKE3Jl'
    'dHJ5X2RlbGF5X3NlY29uZHMYAyABKANCC7pICCIGGICjBSgBUhFyZXRyeURlbGF5U2Vjb25kcx'
    'JhChFtaXNzZWRfcnVuX3BvbGljeRgEIAEoDjIpLm9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVN'
    'aXNzZWRSdW5Qb2xpY3lCCrpIB4IBBBABIABSD21pc3NlZFJ1blBvbGljeRJmChJjb25jdXJyZW'
    '5jeV9wb2xpY3kYBSABKA4yKy5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlQ29uY3VycmVuY3lQ'
    'b2xpY3lCCrpIB4IBBBABIABSEWNvbmN1cnJlbmN5UG9saWN5EjAKDWxlYXNlX3NlY29uZHMYBi'
    'ABKANCC7pICCIGGICjBSgeUgxsZWFzZVNlY29uZHMSKgoRbm90aWZ5X29uX3N1Y2Nlc3MYByAB'
    'KAhSD25vdGlmeU9uU3VjY2VzcxIqChFub3RpZnlfb25fZmFpbHVyZRgIIAEoCFIPbm90aWZ5T2'
    '5GYWlsdXJl');

@$core.Deprecated('Use scheduleUpdateRuntimePolicyResponseDescriptor instead')
const ScheduleUpdateRuntimePolicyResponse$json = {
  '1': 'ScheduleUpdateRuntimePolicyResponse',
  '2': [
    {
      '1': 'policy',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleRuntimePolicy',
      '10': 'policy'
    },
  ],
};

/// Descriptor for `ScheduleUpdateRuntimePolicyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleUpdateRuntimePolicyResponseDescriptor =
    $convert.base64Decode(
        'CiNTY2hlZHVsZVVwZGF0ZVJ1bnRpbWVQb2xpY3lSZXNwb25zZRI/CgZwb2xpY3kYASABKAsyJy'
        '5vcGVub3hpZGUuYXBpLnYxLlNjaGVkdWxlUnVudGltZVBvbGljeVIGcG9saWN5');

@$core.Deprecated('Use scheduleListExecutionsRequestDescriptor instead')
const ScheduleListExecutionsRequest$json = {
  '1': 'ScheduleListExecutionsRequest',
  '2': [
    {'1': 'schedule_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'scheduleId'},
  ],
};

/// Descriptor for `ScheduleListExecutionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListExecutionsRequestDescriptor =
    $convert.base64Decode(
        'Ch1TY2hlZHVsZUxpc3RFeGVjdXRpb25zUmVxdWVzdBIoCgtzY2hlZHVsZV9pZBgBIAEoA0IHuk'
        'gEIgIgAFIKc2NoZWR1bGVJZA==');

@$core.Deprecated('Use scheduleListExecutionsResponseDescriptor instead')
const ScheduleListExecutionsResponse$json = {
  '1': 'ScheduleListExecutionsResponse',
  '2': [
    {
      '1': 'executions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleExecution',
      '10': 'executions'
    },
  ],
};

/// Descriptor for `ScheduleListExecutionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleListExecutionsResponseDescriptor =
    $convert.base64Decode(
        'Ch5TY2hlZHVsZUxpc3RFeGVjdXRpb25zUmVzcG9uc2USQwoKZXhlY3V0aW9ucxgBIAMoCzIjLm'
        '9wZW5veGlkZS5hcGkudjEuU2NoZWR1bGVFeGVjdXRpb25SCmV4ZWN1dGlvbnM=');

@$core.Deprecated('Use scheduleGetLogsRequestDescriptor instead')
const ScheduleGetLogsRequest$json = {
  '1': 'ScheduleGetLogsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ScheduleGetLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetLogsRequestDescriptor =
    $convert.base64Decode(
        'ChZTY2hlZHVsZUdldExvZ3NSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use scheduleGetLogsResponseDescriptor instead')
const ScheduleGetLogsResponse$json = {
  '1': 'ScheduleGetLogsResponse',
  '2': [
    {
      '1': 'log',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleLog',
      '10': 'log'
    },
  ],
};

/// Descriptor for `ScheduleGetLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetLogsResponseDescriptor =
    $convert.base64Decode(
        'ChdTY2hlZHVsZUdldExvZ3NSZXNwb25zZRIvCgNsb2cYASABKAsyHS5vcGVub3hpZGUuYXBpLn'
        'YxLlNjaGVkdWxlTG9nUgNsb2c=');

@$core.Deprecated('Use scheduleGetExecutionLogsRequestDescriptor instead')
const ScheduleGetExecutionLogsRequest$json = {
  '1': 'ScheduleGetExecutionLogsRequest',
  '2': [
    {'1': 'schedule_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'scheduleId'},
    {'1': 'execution_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'executionId'},
  ],
};

/// Descriptor for `ScheduleGetExecutionLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetExecutionLogsRequestDescriptor =
    $convert.base64Decode(
        'Ch9TY2hlZHVsZUdldEV4ZWN1dGlvbkxvZ3NSZXF1ZXN0EigKC3NjaGVkdWxlX2lkGAEgASgDQg'
        'e6SAQiAiAAUgpzY2hlZHVsZUlkEioKDGV4ZWN1dGlvbl9pZBgCIAEoA0IHukgEIgIgAFILZXhl'
        'Y3V0aW9uSWQ=');

@$core.Deprecated('Use scheduleGetExecutionLogsResponseDescriptor instead')
const ScheduleGetExecutionLogsResponse$json = {
  '1': 'ScheduleGetExecutionLogsResponse',
  '2': [
    {
      '1': 'log',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ScheduleLog',
      '10': 'log'
    },
  ],
};

/// Descriptor for `ScheduleGetExecutionLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List scheduleGetExecutionLogsResponseDescriptor =
    $convert.base64Decode(
        'CiBTY2hlZHVsZUdldEV4ZWN1dGlvbkxvZ3NSZXNwb25zZRIvCgNsb2cYASABKAsyHS5vcGVub3'
        'hpZGUuYXBpLnYxLlNjaGVkdWxlTG9nUgNsb2c=');

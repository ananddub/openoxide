// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/monitoring.proto.

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

@$core.Deprecated('Use monitoringAgentStateDescriptor instead')
const MonitoringAgentState$json = {
  '1': 'MonitoringAgentState',
  '2': [
    {'1': 'MONITORING_AGENT_STATE_UNSPECIFIED', '2': 0},
    {'1': 'MONITORING_AGENT_STATE_ONLINE', '2': 1},
    {'1': 'MONITORING_AGENT_STATE_OFFLINE', '2': 2},
    {'1': 'MONITORING_AGENT_STATE_NEVER_SEEN', '2': 3},
  ],
};

/// Descriptor for `MonitoringAgentState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List monitoringAgentStateDescriptor = $convert.base64Decode(
    'ChRNb25pdG9yaW5nQWdlbnRTdGF0ZRImCiJNT05JVE9SSU5HX0FHRU5UX1NUQVRFX1VOU1BFQ0'
    'lGSUVEEAASIQodTU9OSVRPUklOR19BR0VOVF9TVEFURV9PTkxJTkUQARIiCh5NT05JVE9SSU5H'
    'X0FHRU5UX1NUQVRFX09GRkxJTkUQAhIlCiFNT05JVE9SSU5HX0FHRU5UX1NUQVRFX05FVkVSX1'
    'NFRU4QAw==');

@$core.Deprecated('Use monitoringServerMetricDescriptor instead')
const MonitoringServerMetric$json = {
  '1': 'MonitoringServerMetric',
  '2': [
    {
      '1': 'timestamp',
      '3': 1,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'timestamp',
      '17': true
    },
    {'1': 'server_id', '3': 2, '4': 1, '5': 3, '10': 'serverId'},
    {'1': 'cpu', '3': 3, '4': 1, '5': 1, '10': 'cpu'},
    {'1': 'cpu_model', '3': 4, '4': 1, '5': 9, '10': 'cpuModel'},
    {'1': 'cpu_cores', '3': 5, '4': 1, '5': 3, '10': 'cpuCores'},
    {
      '1': 'cpu_physical_cores',
      '3': 6,
      '4': 1,
      '5': 3,
      '10': 'cpuPhysicalCores'
    },
    {'1': 'cpu_speed', '3': 7, '4': 1, '5': 1, '10': 'cpuSpeed'},
    {'1': 'os', '3': 8, '4': 1, '5': 9, '10': 'os'},
    {'1': 'distro', '3': 9, '4': 1, '5': 9, '10': 'distro'},
    {'1': 'kernel', '3': 10, '4': 1, '5': 9, '10': 'kernel'},
    {'1': 'arch', '3': 11, '4': 1, '5': 9, '10': 'arch'},
    {'1': 'mem_used', '3': 12, '4': 1, '5': 1, '10': 'memUsed'},
    {'1': 'mem_used_gb', '3': 13, '4': 1, '5': 1, '10': 'memUsedGb'},
    {'1': 'mem_total', '3': 14, '4': 1, '5': 1, '10': 'memTotal'},
    {'1': 'uptime', '3': 15, '4': 1, '5': 3, '10': 'uptime'},
    {'1': 'disk_used', '3': 16, '4': 1, '5': 1, '10': 'diskUsed'},
    {'1': 'total_disk', '3': 17, '4': 1, '5': 1, '10': 'totalDisk'},
    {'1': 'network_in', '3': 18, '4': 1, '5': 1, '10': 'networkIn'},
    {'1': 'network_out', '3': 19, '4': 1, '5': 1, '10': 'networkOut'},
  ],
  '8': [
    {'1': '_timestamp'},
  ],
};

/// Descriptor for `MonitoringServerMetric`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringServerMetricDescriptor = $convert.base64Decode(
    'ChZNb25pdG9yaW5nU2VydmVyTWV0cmljEiEKCXRpbWVzdGFtcBgBIAEoA0gAUgl0aW1lc3RhbX'
    'CIAQESGwoJc2VydmVyX2lkGAIgASgDUghzZXJ2ZXJJZBIQCgNjcHUYAyABKAFSA2NwdRIbCglj'
    'cHVfbW9kZWwYBCABKAlSCGNwdU1vZGVsEhsKCWNwdV9jb3JlcxgFIAEoA1IIY3B1Q29yZXMSLA'
    'oSY3B1X3BoeXNpY2FsX2NvcmVzGAYgASgDUhBjcHVQaHlzaWNhbENvcmVzEhsKCWNwdV9zcGVl'
    'ZBgHIAEoAVIIY3B1U3BlZWQSDgoCb3MYCCABKAlSAm9zEhYKBmRpc3RybxgJIAEoCVIGZGlzdH'
    'JvEhYKBmtlcm5lbBgKIAEoCVIGa2VybmVsEhIKBGFyY2gYCyABKAlSBGFyY2gSGQoIbWVtX3Vz'
    'ZWQYDCABKAFSB21lbVVzZWQSHgoLbWVtX3VzZWRfZ2IYDSABKAFSCW1lbVVzZWRHYhIbCgltZW'
    '1fdG90YWwYDiABKAFSCG1lbVRvdGFsEhYKBnVwdGltZRgPIAEoA1IGdXB0aW1lEhsKCWRpc2tf'
    'dXNlZBgQIAEoAVIIZGlza1VzZWQSHQoKdG90YWxfZGlzaxgRIAEoAVIJdG90YWxEaXNrEh0KCm'
    '5ldHdvcmtfaW4YEiABKAFSCW5ldHdvcmtJbhIfCgtuZXR3b3JrX291dBgTIAEoAVIKbmV0d29y'
    'a091dEIMCgpfdGltZXN0YW1w');

@$core.Deprecated('Use monitoringContainerStatsDescriptor instead')
const MonitoringContainerStats$json = {
  '1': 'MonitoringContainerStats',
  '2': [
    {'1': 'cpu_percent', '3': 1, '4': 1, '5': 1, '10': 'cpuPercent'},
    {'1': 'memory_percent', '3': 2, '4': 1, '5': 1, '10': 'memoryPercent'},
    {'1': 'memory_used_mb', '3': 3, '4': 1, '5': 1, '10': 'memoryUsedMb'},
    {'1': 'memory_limit_mb', '3': 4, '4': 1, '5': 1, '10': 'memoryLimitMb'},
    {'1': 'net_rx_kbps', '3': 5, '4': 1, '5': 1, '10': 'netRxKbps'},
    {'1': 'net_tx_kbps', '3': 6, '4': 1, '5': 1, '10': 'netTxKbps'},
    {'1': 'block_read_mb', '3': 7, '4': 1, '5': 1, '10': 'blockReadMb'},
    {'1': 'block_write_mb', '3': 8, '4': 1, '5': 1, '10': 'blockWriteMb'},
  ],
};

/// Descriptor for `MonitoringContainerStats`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringContainerStatsDescriptor = $convert.base64Decode(
    'ChhNb25pdG9yaW5nQ29udGFpbmVyU3RhdHMSHwoLY3B1X3BlcmNlbnQYASABKAFSCmNwdVBlcm'
    'NlbnQSJQoObWVtb3J5X3BlcmNlbnQYAiABKAFSDW1lbW9yeVBlcmNlbnQSJAoObWVtb3J5X3Vz'
    'ZWRfbWIYAyABKAFSDG1lbW9yeVVzZWRNYhImCg9tZW1vcnlfbGltaXRfbWIYBCABKAFSDW1lbW'
    '9yeUxpbWl0TWISHgoLbmV0X3J4X2ticHMYBSABKAFSCW5ldFJ4S2JwcxIeCgtuZXRfdHhfa2Jw'
    'cxgGIAEoAVIJbmV0VHhLYnBzEiIKDWJsb2NrX3JlYWRfbWIYByABKAFSC2Jsb2NrUmVhZE1iEi'
    'QKDmJsb2NrX3dyaXRlX21iGAggASgBUgxibG9ja1dyaXRlTWI=');

@$core.Deprecated('Use monitoringContainerMetricDescriptor instead')
const MonitoringContainerMetric$json = {
  '1': 'MonitoringContainerMetric',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '9': 0, '10': 'id', '17': true},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 3, '10': 'timestamp'},
    {'1': 'container_id', '3': 3, '4': 1, '5': 9, '10': 'containerId'},
    {'1': 'container_name', '3': 4, '4': 1, '5': 9, '10': 'containerName'},
    {
      '1': 'stats',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringContainerStats',
      '10': 'stats'
    },
    {'1': 'server_id', '3': 6, '4': 1, '5': 3, '10': 'serverId'},
    {
      '1': 'application_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'composeId',
      '17': true
    },
  ],
  '8': [
    {'1': '_id'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
  ],
};

/// Descriptor for `MonitoringContainerMetric`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringContainerMetricDescriptor = $convert.base64Decode(
    'ChlNb25pdG9yaW5nQ29udGFpbmVyTWV0cmljEhMKAmlkGAEgASgDSABSAmlkiAEBEhwKCXRpbW'
    'VzdGFtcBgCIAEoA1IJdGltZXN0YW1wEiEKDGNvbnRhaW5lcl9pZBgDIAEoCVILY29udGFpbmVy'
    'SWQSJQoOY29udGFpbmVyX25hbWUYBCABKAlSDWNvbnRhaW5lck5hbWUSQAoFc3RhdHMYBSABKA'
    'syKi5vcGVub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdDb250YWluZXJTdGF0c1IFc3RhdHMSGwoJ'
    'c2VydmVyX2lkGAYgASgDUghzZXJ2ZXJJZBIqCg5hcHBsaWNhdGlvbl9pZBgHIAEoA0gBUg1hcH'
    'BsaWNhdGlvbklkiAEBEiIKCmNvbXBvc2VfaWQYCCABKANIAlIJY29tcG9zZUlkiAEBQgUKA19p'
    'ZEIRCg9fYXBwbGljYXRpb25faWRCDQoLX2NvbXBvc2VfaWQ=');

@$core.Deprecated('Use monitoringContainerStateDescriptor instead')
const MonitoringContainerState$json = {
  '1': 'MonitoringContainerState',
  '2': [
    {'1': 'container_id', '3': 1, '4': 1, '5': 9, '10': 'containerId'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'state', '3': 3, '4': 1, '5': 9, '10': 'state'},
    {'1': 'status', '3': 4, '4': 1, '5': 9, '10': 'status'},
    {'1': 'application_id', '3': 5, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'compose_id', '3': 6, '4': 1, '5': 3, '10': 'composeId'},
    {'1': 'compose_project', '3': 7, '4': 1, '5': 9, '10': 'composeProject'},
    {'1': 'compose_service', '3': 8, '4': 1, '5': 9, '10': 'composeService'},
  ],
};

/// Descriptor for `MonitoringContainerState`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringContainerStateDescriptor = $convert.base64Decode(
    'ChhNb25pdG9yaW5nQ29udGFpbmVyU3RhdGUSIQoMY29udGFpbmVyX2lkGAEgASgJUgtjb250YW'
    'luZXJJZBISCgRuYW1lGAIgASgJUgRuYW1lEhQKBXN0YXRlGAMgASgJUgVzdGF0ZRIWCgZzdGF0'
    'dXMYBCABKAlSBnN0YXR1cxIlCg5hcHBsaWNhdGlvbl9pZBgFIAEoA1INYXBwbGljYXRpb25JZB'
    'IdCgpjb21wb3NlX2lkGAYgASgDUgljb21wb3NlSWQSJwoPY29tcG9zZV9wcm9qZWN0GAcgASgJ'
    'Ug5jb21wb3NlUHJvamVjdBInCg9jb21wb3NlX3NlcnZpY2UYCCABKAlSDmNvbXBvc2VTZXJ2aW'
    'Nl');

@$core.Deprecated('Use monitoringAgentStatusDescriptor instead')
const MonitoringAgentStatus$json = {
  '1': 'MonitoringAgentStatus',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '10': 'serverId'},
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'last_seen_at',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'lastSeenAt',
      '17': true
    },
    {
      '1': 'agent_version',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'agentVersion',
      '17': true
    },
    {
      '1': 'state',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.MonitoringAgentState',
      '10': 'state'
    },
  ],
  '8': [
    {'1': '_last_seen_at'},
    {'1': '_agent_version'},
  ],
};

/// Descriptor for `MonitoringAgentStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringAgentStatusDescriptor = $convert.base64Decode(
    'ChVNb25pdG9yaW5nQWdlbnRTdGF0dXMSGwoJc2VydmVyX2lkGAEgASgDUghzZXJ2ZXJJZBInCg'
    '9vcmdhbml6YXRpb25faWQYAiABKANSDm9yZ2FuaXphdGlvbklkEiUKDGxhc3Rfc2Vlbl9hdBgD'
    'IAEoA0gAUgpsYXN0U2VlbkF0iAEBEigKDWFnZW50X3ZlcnNpb24YBCABKAlIAVIMYWdlbnRWZX'
    'JzaW9uiAEBEjwKBXN0YXRlGAUgASgOMiYub3Blbm94aWRlLmFwaS52MS5Nb25pdG9yaW5nQWdl'
    'bnRTdGF0ZVIFc3RhdGVCDwoNX2xhc3Rfc2Vlbl9hdEIQCg5fYWdlbnRfdmVyc2lvbg==');

@$core.Deprecated('Use monitoringAgentActionDescriptor instead')
const MonitoringAgentAction$json = {
  '1': 'MonitoringAgentAction',
  '2': [
    {'1': 'action', '3': 1, '4': 1, '5': 9, '10': 'action'},
    {'1': 'success', '3': 2, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 3, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `MonitoringAgentAction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringAgentActionDescriptor = $convert.base64Decode(
    'ChVNb25pdG9yaW5nQWdlbnRBY3Rpb24SFgoGYWN0aW9uGAEgASgJUgZhY3Rpb24SGAoHc3VjY2'
    'VzcxgCIAEoCFIHc3VjY2VzcxIYCgdtZXNzYWdlGAMgASgJUgdtZXNzYWdl');

@$core.Deprecated('Use monitoringMaintenanceWindowDescriptor instead')
const MonitoringMaintenanceWindow$json = {
  '1': 'MonitoringMaintenanceWindow',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'server_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'starts_at', '3': 4, '4': 1, '5': 3, '10': 'startsAt'},
    {'1': 'ends_at', '3': 5, '4': 1, '5': 3, '10': 'endsAt'},
    {'1': 'reason', '3': 6, '4': 1, '5': 9, '10': 'reason'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `MonitoringMaintenanceWindow`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringMaintenanceWindowDescriptor = $convert.base64Decode(
    'ChtNb25pdG9yaW5nTWFpbnRlbmFuY2VXaW5kb3cSDgoCaWQYASABKANSAmlkEicKD29yZ2FuaX'
    'phdGlvbl9pZBgCIAEoA1IOb3JnYW5pemF0aW9uSWQSIAoJc2VydmVyX2lkGAMgASgDSABSCHNl'
    'cnZlcklkiAEBEhsKCXN0YXJ0c19hdBgEIAEoA1IIc3RhcnRzQXQSFwoHZW5kc19hdBgFIAEoA1'
    'IGZW5kc0F0EhYKBnJlYXNvbhgGIAEoCVIGcmVhc29uEh0KCmNyZWF0ZWRfYXQYByABKANSCWNy'
    'ZWF0ZWRBdEIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use monitoringGetOverviewRequestDescriptor instead')
const MonitoringGetOverviewRequest$json = {
  '1': 'MonitoringGetOverviewRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `MonitoringGetOverviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetOverviewRequestDescriptor =
    $convert.base64Decode(
        'ChxNb25pdG9yaW5nR2V0T3ZlcnZpZXdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use monitoringGetMetricsRequestDescriptor instead')
const MonitoringGetMetricsRequest$json = {
  '1': 'MonitoringGetMetricsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `MonitoringGetMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetMetricsRequestDescriptor =
    $convert.base64Decode(
        'ChtNb25pdG9yaW5nR2V0TWV0cmljc1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use monitoringGetContainerStateRequestDescriptor instead')
const MonitoringGetContainerStateRequest$json = {
  '1': 'MonitoringGetContainerStateRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `MonitoringGetContainerStateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetContainerStateRequestDescriptor =
    $convert.base64Decode(
        'CiJNb25pdG9yaW5nR2V0Q29udGFpbmVyU3RhdGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
        'AAUgJpZA==');

@$core.Deprecated('Use monitoringGetLocalMetricsRequestDescriptor instead')
const MonitoringGetLocalMetricsRequest$json = {
  '1': 'MonitoringGetLocalMetricsRequest',
};

/// Descriptor for `MonitoringGetLocalMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetLocalMetricsRequestDescriptor =
    $convert.base64Decode('CiBNb25pdG9yaW5nR2V0TG9jYWxNZXRyaWNzUmVxdWVzdA==');

@$core.Deprecated('Use monitoringGetLocalMetricsResponseDescriptor instead')
const MonitoringGetLocalMetricsResponse$json = {
  '1': 'MonitoringGetLocalMetricsResponse',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 9, '10': 'status'},
    {'1': 'service', '3': 2, '4': 1, '5': 9, '10': 'service'},
    {'1': 'endpoints', '3': 3, '4': 3, '5': 9, '10': 'endpoints'},
  ],
};

/// Descriptor for `MonitoringGetLocalMetricsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetLocalMetricsResponseDescriptor =
    $convert.base64Decode(
        'CiFNb25pdG9yaW5nR2V0TG9jYWxNZXRyaWNzUmVzcG9uc2USFgoGc3RhdHVzGAEgASgJUgZzdG'
        'F0dXMSGAoHc2VydmljZRgCIAEoCVIHc2VydmljZRIcCgllbmRwb2ludHMYAyADKAlSCWVuZHBv'
        'aW50cw==');

@$core.Deprecated('Use monitoringGetServerMetricsRequestDescriptor instead')
const MonitoringGetServerMetricsRequest$json = {
  '1': 'MonitoringGetServerMetricsRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringGetServerMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetServerMetricsRequestDescriptor =
    $convert.base64Decode(
        'CiFNb25pdG9yaW5nR2V0U2VydmVyTWV0cmljc1JlcXVlc3QSJAoJc2VydmVyX2lkGAEgASgDQg'
        'e6SAQiAiAAUghzZXJ2ZXJJZA==');

@$core.Deprecated('Use monitoringGetServerMetricsResponseDescriptor instead')
const MonitoringGetServerMetricsResponse$json = {
  '1': 'MonitoringGetServerMetricsResponse',
  '2': [
    {
      '1': 'metrics',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringServerMetric',
      '10': 'metrics'
    },
  ],
};

/// Descriptor for `MonitoringGetServerMetricsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetServerMetricsResponseDescriptor =
    $convert.base64Decode(
        'CiJNb25pdG9yaW5nR2V0U2VydmVyTWV0cmljc1Jlc3BvbnNlEkIKB21ldHJpY3MYASADKAsyKC'
        '5vcGVub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdTZXJ2ZXJNZXRyaWNSB21ldHJpY3M=');

@$core.Deprecated('Use monitoringGetContainerMetricsRequestDescriptor instead')
const MonitoringGetContainerMetricsRequest$json = {
  '1': 'MonitoringGetContainerMetricsRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
    {
      '1': 'app_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'appName',
      '17': true
    },
    {
      '1': 'limit',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'limit',
      '17': true
    },
  ],
  '8': [
    {'1': '_app_name'},
    {'1': '_limit'},
  ],
};

/// Descriptor for `MonitoringGetContainerMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetContainerMetricsRequestDescriptor =
    $convert.base64Decode(
        'CiRNb25pdG9yaW5nR2V0Q29udGFpbmVyTWV0cmljc1JlcXVlc3QSJAoJc2VydmVyX2lkGAEgAS'
        'gDQge6SAQiAiAAUghzZXJ2ZXJJZBIwCg9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABS'
        'Dm9yZ2FuaXphdGlvbklkEigKCGFwcF9uYW1lGAMgASgJQgi6SAVyAxj/AUgAUgdhcHBOYW1liA'
        'EBEiUKBWxpbWl0GAQgASgDQgq6SAciBRjoBygBSAFSBWxpbWl0iAEBQgsKCV9hcHBfbmFtZUII'
        'CgZfbGltaXQ=');

@$core.Deprecated('Use monitoringGetContainerMetricsResponseDescriptor instead')
const MonitoringGetContainerMetricsResponse$json = {
  '1': 'MonitoringGetContainerMetricsResponse',
  '2': [
    {
      '1': 'metrics',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringContainerMetric',
      '10': 'metrics'
    },
  ],
};

/// Descriptor for `MonitoringGetContainerMetricsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetContainerMetricsResponseDescriptor =
    $convert.base64Decode(
        'CiVNb25pdG9yaW5nR2V0Q29udGFpbmVyTWV0cmljc1Jlc3BvbnNlEkUKB21ldHJpY3MYASADKA'
        'syKy5vcGVub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdDb250YWluZXJNZXRyaWNSB21ldHJpY3M=');

@$core.Deprecated('Use monitoringGetContainerStatesRequestDescriptor instead')
const MonitoringGetContainerStatesRequest$json = {
  '1': 'MonitoringGetContainerStatesRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringGetContainerStatesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetContainerStatesRequestDescriptor =
    $convert.base64Decode(
        'CiNNb25pdG9yaW5nR2V0Q29udGFpbmVyU3RhdGVzUmVxdWVzdBIkCglzZXJ2ZXJfaWQYASABKA'
        'NCB7pIBCICIABSCHNlcnZlcklk');

@$core.Deprecated('Use monitoringGetContainerStatesResponseDescriptor instead')
const MonitoringGetContainerStatesResponse$json = {
  '1': 'MonitoringGetContainerStatesResponse',
  '2': [
    {
      '1': 'states',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringContainerState',
      '10': 'states'
    },
  ],
};

/// Descriptor for `MonitoringGetContainerStatesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetContainerStatesResponseDescriptor =
    $convert.base64Decode(
        'CiRNb25pdG9yaW5nR2V0Q29udGFpbmVyU3RhdGVzUmVzcG9uc2USQgoGc3RhdGVzGAEgAygLMi'
        'oub3Blbm94aWRlLmFwaS52MS5Nb25pdG9yaW5nQ29udGFpbmVyU3RhdGVSBnN0YXRlcw==');

@$core.Deprecated('Use monitoringCreateAgentTokenRequestDescriptor instead')
const MonitoringCreateAgentTokenRequest$json = {
  '1': 'MonitoringCreateAgentTokenRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
};

/// Descriptor for `MonitoringCreateAgentTokenRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringCreateAgentTokenRequestDescriptor =
    $convert.base64Decode(
        'CiFNb25pdG9yaW5nQ3JlYXRlQWdlbnRUb2tlblJlcXVlc3QSJAoJc2VydmVyX2lkGAEgASgDQg'
        'e6SAQiAiAAUghzZXJ2ZXJJZBIwCg9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABSDm9y'
        'Z2FuaXphdGlvbklk');

@$core.Deprecated('Use monitoringCreateAgentTokenResponseDescriptor instead')
const MonitoringCreateAgentTokenResponse$json = {
  '1': 'MonitoringCreateAgentTokenResponse',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '10': 'serverId'},
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'token', '3': 3, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `MonitoringCreateAgentTokenResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringCreateAgentTokenResponseDescriptor =
    $convert.base64Decode(
        'CiJNb25pdG9yaW5nQ3JlYXRlQWdlbnRUb2tlblJlc3BvbnNlEhsKCXNlcnZlcl9pZBgBIAEoA1'
        'IIc2VydmVySWQSJwoPb3JnYW5pemF0aW9uX2lkGAIgASgDUg5vcmdhbml6YXRpb25JZBIUCgV0'
        'b2tlbhgDIAEoCVIFdG9rZW4=');

@$core.Deprecated('Use monitoringGetAgentStatusRequestDescriptor instead')
const MonitoringGetAgentStatusRequest$json = {
  '1': 'MonitoringGetAgentStatusRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringGetAgentStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetAgentStatusRequestDescriptor =
    $convert.base64Decode(
        'Ch9Nb25pdG9yaW5nR2V0QWdlbnRTdGF0dXNSZXF1ZXN0EiQKCXNlcnZlcl9pZBgBIAEoA0IHuk'
        'gEIgIgAFIIc2VydmVySWQ=');

@$core.Deprecated('Use monitoringGetAgentStatusResponseDescriptor instead')
const MonitoringGetAgentStatusResponse$json = {
  '1': 'MonitoringGetAgentStatusResponse',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringAgentStatus',
      '10': 'status'
    },
  ],
};

/// Descriptor for `MonitoringGetAgentStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetAgentStatusResponseDescriptor =
    $convert.base64Decode(
        'CiBNb25pdG9yaW5nR2V0QWdlbnRTdGF0dXNSZXNwb25zZRI/CgZzdGF0dXMYASABKAsyJy5vcG'
        'Vub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdBZ2VudFN0YXR1c1IGc3RhdHVz');

@$core.Deprecated('Use monitoringStartAgentRequestDescriptor instead')
const MonitoringStartAgentRequest$json = {
  '1': 'MonitoringStartAgentRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringStartAgentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringStartAgentRequestDescriptor =
    $convert.base64Decode(
        'ChtNb25pdG9yaW5nU3RhcnRBZ2VudFJlcXVlc3QSJAoJc2VydmVyX2lkGAEgASgDQge6SAQiAi'
        'AAUghzZXJ2ZXJJZA==');

@$core.Deprecated('Use monitoringStartAgentResponseDescriptor instead')
const MonitoringStartAgentResponse$json = {
  '1': 'MonitoringStartAgentResponse',
  '2': [
    {
      '1': 'action',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringAgentAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `MonitoringStartAgentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringStartAgentResponseDescriptor =
    $convert.base64Decode(
        'ChxNb25pdG9yaW5nU3RhcnRBZ2VudFJlc3BvbnNlEj8KBmFjdGlvbhgBIAEoCzInLm9wZW5veG'
        'lkZS5hcGkudjEuTW9uaXRvcmluZ0FnZW50QWN0aW9uUgZhY3Rpb24=');

@$core.Deprecated('Use monitoringStopAgentRequestDescriptor instead')
const MonitoringStopAgentRequest$json = {
  '1': 'MonitoringStopAgentRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringStopAgentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringStopAgentRequestDescriptor =
    $convert.base64Decode(
        'ChpNb25pdG9yaW5nU3RvcEFnZW50UmVxdWVzdBIkCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIA'
        'BSCHNlcnZlcklk');

@$core.Deprecated('Use monitoringStopAgentResponseDescriptor instead')
const MonitoringStopAgentResponse$json = {
  '1': 'MonitoringStopAgentResponse',
  '2': [
    {
      '1': 'action',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringAgentAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `MonitoringStopAgentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringStopAgentResponseDescriptor =
    $convert.base64Decode(
        'ChtNb25pdG9yaW5nU3RvcEFnZW50UmVzcG9uc2USPwoGYWN0aW9uGAEgASgLMicub3Blbm94aW'
        'RlLmFwaS52MS5Nb25pdG9yaW5nQWdlbnRBY3Rpb25SBmFjdGlvbg==');

@$core.Deprecated('Use monitoringRestartAgentRequestDescriptor instead')
const MonitoringRestartAgentRequest$json = {
  '1': 'MonitoringRestartAgentRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringRestartAgentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringRestartAgentRequestDescriptor =
    $convert.base64Decode(
        'Ch1Nb25pdG9yaW5nUmVzdGFydEFnZW50UmVxdWVzdBIkCglzZXJ2ZXJfaWQYASABKANCB7pIBC'
        'ICIABSCHNlcnZlcklk');

@$core.Deprecated('Use monitoringRestartAgentResponseDescriptor instead')
const MonitoringRestartAgentResponse$json = {
  '1': 'MonitoringRestartAgentResponse',
  '2': [
    {
      '1': 'action',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringAgentAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `MonitoringRestartAgentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringRestartAgentResponseDescriptor =
    $convert.base64Decode(
        'Ch5Nb25pdG9yaW5nUmVzdGFydEFnZW50UmVzcG9uc2USPwoGYWN0aW9uGAEgASgLMicub3Blbm'
        '94aWRlLmFwaS52MS5Nb25pdG9yaW5nQWdlbnRBY3Rpb25SBmFjdGlvbg==');

@$core.Deprecated('Use monitoringReinstallAgentRequestDescriptor instead')
const MonitoringReinstallAgentRequest$json = {
  '1': 'MonitoringReinstallAgentRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringReinstallAgentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringReinstallAgentRequestDescriptor =
    $convert.base64Decode(
        'Ch9Nb25pdG9yaW5nUmVpbnN0YWxsQWdlbnRSZXF1ZXN0EiQKCXNlcnZlcl9pZBgBIAEoA0IHuk'
        'gEIgIgAFIIc2VydmVySWQ=');

@$core.Deprecated('Use monitoringReinstallAgentResponseDescriptor instead')
const MonitoringReinstallAgentResponse$json = {
  '1': 'MonitoringReinstallAgentResponse',
  '2': [
    {
      '1': 'action',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringAgentAction',
      '10': 'action'
    },
  ],
};

/// Descriptor for `MonitoringReinstallAgentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringReinstallAgentResponseDescriptor =
    $convert.base64Decode(
        'CiBNb25pdG9yaW5nUmVpbnN0YWxsQWdlbnRSZXNwb25zZRI/CgZhY3Rpb24YASABKAsyJy5vcG'
        'Vub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdBZ2VudEFjdGlvblIGYWN0aW9u');

@$core.Deprecated('Use monitoringExportServerMetricsRequestDescriptor instead')
const MonitoringExportServerMetricsRequest$json = {
  '1': 'MonitoringExportServerMetricsRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringExportServerMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringExportServerMetricsRequestDescriptor =
    $convert.base64Decode(
        'CiRNb25pdG9yaW5nRXhwb3J0U2VydmVyTWV0cmljc1JlcXVlc3QSJAoJc2VydmVyX2lkGAEgAS'
        'gDQge6SAQiAiAAUghzZXJ2ZXJJZA==');

@$core.Deprecated('Use monitoringExportServerMetricsResponseDescriptor instead')
const MonitoringExportServerMetricsResponse$json = {
  '1': 'MonitoringExportServerMetricsResponse',
  '2': [
    {'1': 'csv', '3': 1, '4': 1, '5': 9, '10': 'csv'},
  ],
};

/// Descriptor for `MonitoringExportServerMetricsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringExportServerMetricsResponseDescriptor =
    $convert.base64Decode(
        'CiVNb25pdG9yaW5nRXhwb3J0U2VydmVyTWV0cmljc1Jlc3BvbnNlEhAKA2NzdhgBIAEoCVIDY3'
        'N2');

@$core.Deprecated('Use monitoringGetPrometheusMetricsRequestDescriptor instead')
const MonitoringGetPrometheusMetricsRequest$json = {
  '1': 'MonitoringGetPrometheusMetricsRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `MonitoringGetPrometheusMetricsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetPrometheusMetricsRequestDescriptor =
    $convert.base64Decode(
        'CiVNb25pdG9yaW5nR2V0UHJvbWV0aGV1c01ldHJpY3NSZXF1ZXN0EiQKCXNlcnZlcl9pZBgBIA'
        'EoA0IHukgEIgIgAFIIc2VydmVySWQ=');

@$core
    .Deprecated('Use monitoringGetPrometheusMetricsResponseDescriptor instead')
const MonitoringGetPrometheusMetricsResponse$json = {
  '1': 'MonitoringGetPrometheusMetricsResponse',
  '2': [
    {'1': 'prometheus_text', '3': 1, '4': 1, '5': 9, '10': 'prometheusText'},
  ],
};

/// Descriptor for `MonitoringGetPrometheusMetricsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringGetPrometheusMetricsResponseDescriptor =
    $convert.base64Decode(
        'CiZNb25pdG9yaW5nR2V0UHJvbWV0aGV1c01ldHJpY3NSZXNwb25zZRInCg9wcm9tZXRoZXVzX3'
        'RleHQYASABKAlSDnByb21ldGhldXNUZXh0');

@$core.Deprecated(
    'Use monitoringCreateMaintenanceWindowRequestDescriptor instead')
const MonitoringCreateMaintenanceWindowRequest$json = {
  '1': 'MonitoringCreateMaintenanceWindowRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'starts_at', '3': 2, '4': 1, '5': 3, '10': 'startsAt'},
    {'1': 'ends_at', '3': 3, '4': 1, '5': 3, '10': 'endsAt'},
    {'1': 'reason', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'reason'},
    {
      '1': 'organization_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `MonitoringCreateMaintenanceWindowRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringCreateMaintenanceWindowRequestDescriptor =
    $convert.base64Decode(
        'CihNb25pdG9yaW5nQ3JlYXRlTWFpbnRlbmFuY2VXaW5kb3dSZXF1ZXN0EikKCXNlcnZlcl9pZB'
        'gBIAEoA0IHukgEIgIgAEgAUghzZXJ2ZXJJZIgBARIbCglzdGFydHNfYXQYAiABKANSCHN0YXJ0'
        'c0F0EhcKB2VuZHNfYXQYAyABKANSBmVuZHNBdBIiCgZyZWFzb24YBCABKAlCCrpIB3IFEAEY6A'
        'dSBnJlYXNvbhI1Cg9vcmdhbml6YXRpb25faWQYBSABKANCB7pIBCICIABIAVIOb3JnYW5pemF0'
        'aW9uSWSIAQFCDAoKX3NlcnZlcl9pZEISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated(
    'Use monitoringCreateMaintenanceWindowResponseDescriptor instead')
const MonitoringCreateMaintenanceWindowResponse$json = {
  '1': 'MonitoringCreateMaintenanceWindowResponse',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `MonitoringCreateMaintenanceWindowResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    monitoringCreateMaintenanceWindowResponseDescriptor = $convert.base64Decode(
        'CilNb25pdG9yaW5nQ3JlYXRlTWFpbnRlbmFuY2VXaW5kb3dSZXNwb25zZRIOCgJpZBgBIAEoA1'
        'ICaWQ=');

@$core
    .Deprecated('Use monitoringListMaintenanceWindowsRequestDescriptor instead')
const MonitoringListMaintenanceWindowsRequest$json = {
  '1': 'MonitoringListMaintenanceWindowsRequest',
  '2': [
    {
      '1': 'organization_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'limit',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'limit',
      '17': true
    },
    {
      '1': 'offset',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'offset',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `MonitoringListMaintenanceWindowsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringListMaintenanceWindowsRequestDescriptor =
    $convert.base64Decode(
        'CidNb25pdG9yaW5nTGlzdE1haW50ZW5hbmNlV2luZG93c1JlcXVlc3QSNQoPb3JnYW5pemF0aW'
        '9uX2lkGAEgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBEiUKBWxpbWl0GAIgASgD'
        'Qgq6SAciBRjoBygBSAFSBWxpbWl0iAEBEiQKBm9mZnNldBgDIAEoA0IHukgEIgIoAEgCUgZvZm'
        'ZzZXSIAQFCEgoQX29yZ2FuaXphdGlvbl9pZEIICgZfbGltaXRCCQoHX29mZnNldA==');

@$core.Deprecated(
    'Use monitoringListMaintenanceWindowsResponseDescriptor instead')
const MonitoringListMaintenanceWindowsResponse$json = {
  '1': 'MonitoringListMaintenanceWindowsResponse',
  '2': [
    {
      '1': 'windows',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.MonitoringMaintenanceWindow',
      '10': 'windows'
    },
  ],
};

/// Descriptor for `MonitoringListMaintenanceWindowsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringListMaintenanceWindowsResponseDescriptor =
    $convert.base64Decode(
        'CihNb25pdG9yaW5nTGlzdE1haW50ZW5hbmNlV2luZG93c1Jlc3BvbnNlEkcKB3dpbmRvd3MYAS'
        'ADKAsyLS5vcGVub3hpZGUuYXBpLnYxLk1vbml0b3JpbmdNYWludGVuYW5jZVdpbmRvd1IHd2lu'
        'ZG93cw==');

@$core.Deprecated(
    'Use monitoringDeleteMaintenanceWindowRequestDescriptor instead')
const MonitoringDeleteMaintenanceWindowRequest$json = {
  '1': 'MonitoringDeleteMaintenanceWindowRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `MonitoringDeleteMaintenanceWindowRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List monitoringDeleteMaintenanceWindowRequestDescriptor =
    $convert.base64Decode(
        'CihNb25pdG9yaW5nRGVsZXRlTWFpbnRlbmFuY2VXaW5kb3dSZXF1ZXN0EhcKAmlkGAEgASgDQg'
        'e6SAQiAiAAUgJpZBI1Cg9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5p'
        'emF0aW9uSWSIAQFCEgoQX29yZ2FuaXphdGlvbl9pZA==');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/swarm.proto.

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

@$core.Deprecated('Use swarmNodeRoleDescriptor instead')
const SwarmNodeRole$json = {
  '1': 'SwarmNodeRole',
  '2': [
    {'1': 'SWARM_NODE_ROLE_UNSPECIFIED', '2': 0},
    {'1': 'SWARM_NODE_ROLE_WORKER', '2': 1},
    {'1': 'SWARM_NODE_ROLE_MANAGER', '2': 2},
  ],
};

/// Descriptor for `SwarmNodeRole`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List swarmNodeRoleDescriptor = $convert.base64Decode(
    'Cg1Td2FybU5vZGVSb2xlEh8KG1NXQVJNX05PREVfUk9MRV9VTlNQRUNJRklFRBAAEhoKFlNXQV'
    'JNX05PREVfUk9MRV9XT1JLRVIQARIbChdTV0FSTV9OT0RFX1JPTEVfTUFOQUdFUhAC');

@$core.Deprecated('Use swarmRoleDescriptor instead')
const SwarmRole$json = {
  '1': 'SwarmRole',
  '2': [
    {'1': 'SWARM_ROLE_UNSPECIFIED', '2': 0},
    {'1': 'SWARM_ROLE_WORKER', '2': 1},
    {'1': 'SWARM_ROLE_MANAGER', '2': 2},
  ],
};

/// Descriptor for `SwarmRole`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List swarmRoleDescriptor = $convert.base64Decode(
    'CglTd2FybVJvbGUSGgoWU1dBUk1fUk9MRV9VTlNQRUNJRklFRBAAEhUKEVNXQVJNX1JPTEVfV0'
    '9SS0VSEAESFgoSU1dBUk1fUk9MRV9NQU5BR0VSEAI=');

@$core.Deprecated('Use swarmNodeAvailabilityDescriptor instead')
const SwarmNodeAvailability$json = {
  '1': 'SwarmNodeAvailability',
  '2': [
    {'1': 'SWARM_NODE_AVAILABILITY_UNSPECIFIED', '2': 0},
    {'1': 'SWARM_NODE_AVAILABILITY_ACTIVE', '2': 1},
    {'1': 'SWARM_NODE_AVAILABILITY_PAUSE', '2': 2},
    {'1': 'SWARM_NODE_AVAILABILITY_DRAIN', '2': 3},
  ],
};

/// Descriptor for `SwarmNodeAvailability`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List swarmNodeAvailabilityDescriptor = $convert.base64Decode(
    'ChVTd2FybU5vZGVBdmFpbGFiaWxpdHkSJwojU1dBUk1fTk9ERV9BVkFJTEFCSUxJVFlfVU5TUE'
    'VDSUZJRUQQABIiCh5TV0FSTV9OT0RFX0FWQUlMQUJJTElUWV9BQ1RJVkUQARIhCh1TV0FSTV9O'
    'T0RFX0FWQUlMQUJJTElUWV9QQVVTRRACEiEKHVNXQVJNX05PREVfQVZBSUxBQklMSVRZX0RSQU'
    'lOEAM=');

@$core.Deprecated('Use swarmInfoDescriptor instead')
const SwarmInfo$json = {
  '1': 'SwarmInfo',
  '2': [
    {'1': 'node_id', '3': 1, '4': 1, '5': 9, '10': 'nodeId'},
    {'1': 'node_addr', '3': 2, '4': 1, '5': 9, '10': 'nodeAddr'},
    {'1': 'local_node_state', '3': 3, '4': 1, '5': 9, '10': 'localNodeState'},
    {
      '1': 'control_available',
      '3': 4,
      '4': 1,
      '5': 8,
      '10': 'controlAvailable'
    },
    {'1': 'nodes', '3': 5, '4': 1, '5': 3, '10': 'nodes'},
    {'1': 'managers', '3': 6, '4': 1, '5': 3, '10': 'managers'},
  ],
};

/// Descriptor for `SwarmInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmInfoDescriptor = $convert.base64Decode(
    'CglTd2FybUluZm8SFwoHbm9kZV9pZBgBIAEoCVIGbm9kZUlkEhsKCW5vZGVfYWRkchgCIAEoCV'
    'IIbm9kZUFkZHISKAoQbG9jYWxfbm9kZV9zdGF0ZRgDIAEoCVIObG9jYWxOb2RlU3RhdGUSKwoR'
    'Y29udHJvbF9hdmFpbGFibGUYBCABKAhSEGNvbnRyb2xBdmFpbGFibGUSFAoFbm9kZXMYBSABKA'
    'NSBW5vZGVzEhoKCG1hbmFnZXJzGAYgASgDUghtYW5hZ2Vycw==');

@$core.Deprecated('Use swarmJoinTokensDescriptor instead')
const SwarmJoinTokens$json = {
  '1': 'SwarmJoinTokens',
  '2': [
    {'1': 'worker', '3': 1, '4': 1, '5': 9, '10': 'worker'},
    {'1': 'manager', '3': 2, '4': 1, '5': 9, '10': 'manager'},
  ],
};

/// Descriptor for `SwarmJoinTokens`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmJoinTokensDescriptor = $convert.base64Decode(
    'Cg9Td2FybUpvaW5Ub2tlbnMSFgoGd29ya2VyGAEgASgJUgZ3b3JrZXISGAoHbWFuYWdlchgCIA'
    'EoCVIHbWFuYWdlcg==');

@$core.Deprecated('Use swarmNodeDescriptor instead')
const SwarmNode$json = {
  '1': 'SwarmNode',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'hostname', '3': 2, '4': 1, '5': 9, '10': 'hostname'},
    {'1': 'status', '3': 3, '4': 1, '5': 9, '10': 'status'},
    {'1': 'availability', '3': 4, '4': 1, '5': 9, '10': 'availability'},
    {
      '1': 'role',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SwarmNodeRole',
      '10': 'role'
    },
    {'1': 'is_leader', '3': 6, '4': 1, '5': 8, '10': 'isLeader'},
    {'1': 'reachability', '3': 7, '4': 1, '5': 9, '10': 'reachability'},
    {'1': 'ip_address', '3': 8, '4': 1, '5': 9, '10': 'ipAddress'},
    {'1': 'engine_version', '3': 9, '4': 1, '5': 9, '10': 'engineVersion'},
  ],
};

/// Descriptor for `SwarmNode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmNodeDescriptor = $convert.base64Decode(
    'CglTd2FybU5vZGUSDgoCaWQYASABKAlSAmlkEhoKCGhvc3RuYW1lGAIgASgJUghob3N0bmFtZR'
    'IWCgZzdGF0dXMYAyABKAlSBnN0YXR1cxIiCgxhdmFpbGFiaWxpdHkYBCABKAlSDGF2YWlsYWJp'
    'bGl0eRIzCgRyb2xlGAUgASgOMh8ub3Blbm94aWRlLmFwaS52MS5Td2FybU5vZGVSb2xlUgRyb2'
    'xlEhsKCWlzX2xlYWRlchgGIAEoCFIIaXNMZWFkZXISIgoMcmVhY2hhYmlsaXR5GAcgASgJUgxy'
    'ZWFjaGFiaWxpdHkSHQoKaXBfYWRkcmVzcxgIIAEoCVIJaXBBZGRyZXNzEiUKDmVuZ2luZV92ZX'
    'JzaW9uGAkgASgJUg1lbmdpbmVWZXJzaW9u');

@$core.Deprecated('Use swarmSettingsDocumentDescriptor instead')
const SwarmSettingsDocument$json = {
  '1': 'SwarmSettingsDocument',
  '2': [
    {'1': 'settings_json', '3': 1, '4': 1, '5': 9, '10': 'settingsJson'},
  ],
};

/// Descriptor for `SwarmSettingsDocument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmSettingsDocumentDescriptor = $convert.base64Decode(
    'ChVTd2FybVNldHRpbmdzRG9jdW1lbnQSIwoNc2V0dGluZ3NfanNvbhgBIAEoCVIMc2V0dGluZ3'
    'NKc29u');

@$core.Deprecated('Use swarmNetworkDescriptor instead')
const SwarmNetwork$json = {
  '1': 'SwarmNetwork',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'network_json', '3': 3, '4': 1, '5': 9, '10': 'networkJson'},
  ],
};

/// Descriptor for `SwarmNetwork`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmNetworkDescriptor = $convert.base64Decode(
    'CgxTd2FybU5ldHdvcmsSDgoCaWQYASABKAlSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSIQoMbm'
    'V0d29ya19qc29uGAMgASgJUgtuZXR3b3JrSnNvbg==');

@$core.Deprecated('Use swarmServiceInfoDescriptor instead')
const SwarmServiceInfo$json = {
  '1': 'SwarmServiceInfo',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'service_json', '3': 3, '4': 1, '5': 9, '10': 'serviceJson'},
  ],
};

/// Descriptor for `SwarmServiceInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmServiceInfoDescriptor = $convert.base64Decode(
    'ChBTd2FybVNlcnZpY2VJbmZvEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEi'
    'EKDHNlcnZpY2VfanNvbhgDIAEoCVILc2VydmljZUpzb24=');

@$core.Deprecated('Use swarmGetSettingsRequestDescriptor instead')
const SwarmGetSettingsRequest$json = {
  '1': 'SwarmGetSettingsRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmGetSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetSettingsRequestDescriptor =
    $convert.base64Decode(
        'ChdTd2FybUdldFNldHRpbmdzUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIABIAF'
        'IIc2VydmVySWSIAQFCDAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use swarmGetSettingsResponseDescriptor instead')
const SwarmGetSettingsResponse$json = {
  '1': 'SwarmGetSettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmSettingsDocument',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `SwarmGetSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChhTd2FybUdldFNldHRpbmdzUmVzcG9uc2USQwoIc2V0dGluZ3MYASABKAsyJy5vcGVub3hpZG'
        'UuYXBpLnYxLlN3YXJtU2V0dGluZ3NEb2N1bWVudFIIc2V0dGluZ3M=');

@$core.Deprecated('Use swarmUpdateSettingsRequestDescriptor instead')
const SwarmUpdateSettingsRequest$json = {
  '1': 'SwarmUpdateSettingsRequest',
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
    {
      '1': 'settings',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmSettingsDocument',
      '8': {},
      '10': 'settings'
    },
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmUpdateSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateSettingsRequestDescriptor =
    $convert.base64Decode(
        'ChpTd2FybVVwZGF0ZVNldHRpbmdzUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIA'
        'BIAFIIc2VydmVySWSIAQESSwoIc2V0dGluZ3MYAiABKAsyJy5vcGVub3hpZGUuYXBpLnYxLlN3'
        'YXJtU2V0dGluZ3NEb2N1bWVudEIGukgDyAEBUghzZXR0aW5nc0IMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use swarmUpdateSettingsResponseDescriptor instead')
const SwarmUpdateSettingsResponse$json = {
  '1': 'SwarmUpdateSettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmSettingsDocument',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `SwarmUpdateSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChtTd2FybVVwZGF0ZVNldHRpbmdzUmVzcG9uc2USQwoIc2V0dGluZ3MYASABKAsyJy5vcGVub3'
        'hpZGUuYXBpLnYxLlN3YXJtU2V0dGluZ3NEb2N1bWVudFIIc2V0dGluZ3M=');

@$core.Deprecated('Use swarmListNetworksRequestDescriptor instead')
const SwarmListNetworksRequest$json = {
  '1': 'SwarmListNetworksRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmListNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListNetworksRequestDescriptor =
    $convert.base64Decode(
        'ChhTd2FybUxpc3ROZXR3b3Jrc1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAiAASA'
        'BSCHNlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use swarmListNetworksResponseDescriptor instead')
const SwarmListNetworksResponse$json = {
  '1': 'SwarmListNetworksResponse',
  '2': [
    {
      '1': 'networks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmNetwork',
      '10': 'networks'
    },
  ],
};

/// Descriptor for `SwarmListNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListNetworksResponseDescriptor =
    $convert.base64Decode(
        'ChlTd2FybUxpc3ROZXR3b3Jrc1Jlc3BvbnNlEjoKCG5ldHdvcmtzGAEgAygLMh4ub3Blbm94aW'
        'RlLmFwaS52MS5Td2FybU5ldHdvcmtSCG5ldHdvcmtz');

@$core.Deprecated('Use swarmCreateNetworkRequestDescriptor instead')
const SwarmCreateNetworkRequest$json = {
  '1': 'SwarmCreateNetworkRequest',
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
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'config_json',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'configJson',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
    {'1': '_config_json'},
  ],
};

/// Descriptor for `SwarmCreateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmCreateNetworkRequestDescriptor = $convert.base64Decode(
    'ChlTd2FybUNyZWF0ZU5ldHdvcmtSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARIeCgRuYW1lGAIgASgJQgq6SAdyBRABGP8BUgRuYW1lEi8KC2NvbmZp'
    'Z19qc29uGAMgASgJQgm6SAZyBBiAgEBIAVIKY29uZmlnSnNvbogBAUIMCgpfc2VydmVyX2lkQg'
    '4KDF9jb25maWdfanNvbg==');

@$core.Deprecated('Use swarmCreateNetworkResponseDescriptor instead')
const SwarmCreateNetworkResponse$json = {
  '1': 'SwarmCreateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `SwarmCreateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmCreateNetworkResponseDescriptor =
    $convert.base64Decode(
        'ChpTd2FybUNyZWF0ZU5ldHdvcmtSZXNwb25zZRI4CgduZXR3b3JrGAEgASgLMh4ub3Blbm94aW'
        'RlLmFwaS52MS5Td2FybU5ldHdvcmtSB25ldHdvcms=');

@$core.Deprecated('Use swarmUpdateNetworkRequestDescriptor instead')
const SwarmUpdateNetworkRequest$json = {
  '1': 'SwarmUpdateNetworkRequest',
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
    {'1': 'network_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'networkId'},
    {'1': 'config_json', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'configJson'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmUpdateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateNetworkRequestDescriptor = $convert.base64Decode(
    'ChlTd2FybVVwZGF0ZU5ldHdvcmtSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARImCgpuZXR3b3JrX2lkGAIgASgJQge6SARyAhABUgluZXR3b3JrSWQS'
    'KgoLY29uZmlnX2pzb24YAyABKAlCCbpIBnIEGICAQFIKY29uZmlnSnNvbkIMCgpfc2VydmVyX2'
    'lk');

@$core.Deprecated('Use swarmUpdateNetworkResponseDescriptor instead')
const SwarmUpdateNetworkResponse$json = {
  '1': 'SwarmUpdateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `SwarmUpdateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateNetworkResponseDescriptor =
    $convert.base64Decode(
        'ChpTd2FybVVwZGF0ZU5ldHdvcmtSZXNwb25zZRI4CgduZXR3b3JrGAEgASgLMh4ub3Blbm94aW'
        'RlLmFwaS52MS5Td2FybU5ldHdvcmtSB25ldHdvcms=');

@$core.Deprecated('Use swarmDeleteNetworkRequestDescriptor instead')
const SwarmDeleteNetworkRequest$json = {
  '1': 'SwarmDeleteNetworkRequest',
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
    {'1': 'network_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'networkId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmDeleteNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmDeleteNetworkRequestDescriptor = $convert.base64Decode(
    'ChlTd2FybURlbGV0ZU5ldHdvcmtSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARImCgpuZXR3b3JrX2lkGAIgASgJQge6SARyAhABUgluZXR3b3JrSWRC'
    'DAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use swarmListServicesRequestDescriptor instead')
const SwarmListServicesRequest$json = {
  '1': 'SwarmListServicesRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmListServicesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListServicesRequestDescriptor =
    $convert.base64Decode(
        'ChhTd2FybUxpc3RTZXJ2aWNlc1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAiAASA'
        'BSCHNlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use swarmListServicesResponseDescriptor instead')
const SwarmListServicesResponse$json = {
  '1': 'SwarmListServicesResponse',
  '2': [
    {
      '1': 'services',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmServiceInfo',
      '10': 'services'
    },
  ],
};

/// Descriptor for `SwarmListServicesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListServicesResponseDescriptor =
    $convert.base64Decode(
        'ChlTd2FybUxpc3RTZXJ2aWNlc1Jlc3BvbnNlEj4KCHNlcnZpY2VzGAEgAygLMiIub3Blbm94aW'
        'RlLmFwaS52MS5Td2FybVNlcnZpY2VJbmZvUghzZXJ2aWNlcw==');

@$core.Deprecated('Use swarmGetServiceRequestDescriptor instead')
const SwarmGetServiceRequest$json = {
  '1': 'SwarmGetServiceRequest',
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
    {'1': 'service_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'serviceId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmGetServiceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetServiceRequestDescriptor = $convert.base64Decode(
    'ChZTd2FybUdldFNlcnZpY2VSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUg'
    'hzZXJ2ZXJJZIgBARImCgpzZXJ2aWNlX2lkGAIgASgJQge6SARyAhABUglzZXJ2aWNlSWRCDAoK'
    'X3NlcnZlcl9pZA==');

@$core.Deprecated('Use swarmGetServiceResponseDescriptor instead')
const SwarmGetServiceResponse$json = {
  '1': 'SwarmGetServiceResponse',
  '2': [
    {
      '1': 'service',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmServiceInfo',
      '10': 'service'
    },
  ],
};

/// Descriptor for `SwarmGetServiceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetServiceResponseDescriptor =
    $convert.base64Decode(
        'ChdTd2FybUdldFNlcnZpY2VSZXNwb25zZRI8CgdzZXJ2aWNlGAEgASgLMiIub3Blbm94aWRlLm'
        'FwaS52MS5Td2FybVNlcnZpY2VJbmZvUgdzZXJ2aWNl');

@$core.Deprecated('Use swarmUpdateServiceRequestDescriptor instead')
const SwarmUpdateServiceRequest$json = {
  '1': 'SwarmUpdateServiceRequest',
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
    {'1': 'service_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'serviceId'},
    {'1': 'spec_json', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'specJson'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmUpdateServiceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateServiceRequestDescriptor = $convert.base64Decode(
    'ChlTd2FybVVwZGF0ZVNlcnZpY2VSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARImCgpzZXJ2aWNlX2lkGAIgASgJQge6SARyAhABUglzZXJ2aWNlSWQS'
    'JgoJc3BlY19qc29uGAMgASgJQgm6SAZyBBiAgEBSCHNwZWNKc29uQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use swarmUpdateServiceResponseDescriptor instead')
const SwarmUpdateServiceResponse$json = {
  '1': 'SwarmUpdateServiceResponse',
  '2': [
    {
      '1': 'service',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmServiceInfo',
      '10': 'service'
    },
  ],
};

/// Descriptor for `SwarmUpdateServiceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateServiceResponseDescriptor =
    $convert.base64Decode(
        'ChpTd2FybVVwZGF0ZVNlcnZpY2VSZXNwb25zZRI8CgdzZXJ2aWNlGAEgASgLMiIub3Blbm94aW'
        'RlLmFwaS52MS5Td2FybVNlcnZpY2VJbmZvUgdzZXJ2aWNl');

@$core.Deprecated('Use swarmInfoRequestDescriptor instead')
const SwarmInfoRequest$json = {
  '1': 'SwarmInfoRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmInfoRequestDescriptor = $convert.base64Decode(
    'ChBTd2FybUluZm9SZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUghzZXJ2ZX'
    'JJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use swarmInfoResponseDescriptor instead')
const SwarmInfoResponse$json = {
  '1': 'SwarmInfoResponse',
  '2': [
    {
      '1': 'info',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmInfo',
      '10': 'info'
    },
  ],
};

/// Descriptor for `SwarmInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmInfoResponseDescriptor = $convert.base64Decode(
    'ChFTd2FybUluZm9SZXNwb25zZRIvCgRpbmZvGAEgASgLMhsub3Blbm94aWRlLmFwaS52MS5Td2'
    'FybUluZm9SBGluZm8=');

@$core.Deprecated('Use swarmGetJoinTokensRequestDescriptor instead')
const SwarmGetJoinTokensRequest$json = {
  '1': 'SwarmGetJoinTokensRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmGetJoinTokensRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetJoinTokensRequestDescriptor =
    $convert.base64Decode(
        'ChlTd2FybUdldEpvaW5Ub2tlbnNSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
        'gAUghzZXJ2ZXJJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use swarmGetJoinTokensResponseDescriptor instead')
const SwarmGetJoinTokensResponse$json = {
  '1': 'SwarmGetJoinTokensResponse',
  '2': [
    {
      '1': 'tokens',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmJoinTokens',
      '10': 'tokens'
    },
  ],
};

/// Descriptor for `SwarmGetJoinTokensResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmGetJoinTokensResponseDescriptor =
    $convert.base64Decode(
        'ChpTd2FybUdldEpvaW5Ub2tlbnNSZXNwb25zZRI5CgZ0b2tlbnMYASABKAsyIS5vcGVub3hpZG'
        'UuYXBpLnYxLlN3YXJtSm9pblRva2Vuc1IGdG9rZW5z');

@$core.Deprecated('Use swarmListNodesRequestDescriptor instead')
const SwarmListNodesRequest$json = {
  '1': 'SwarmListNodesRequest',
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
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmListNodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListNodesRequestDescriptor = $convert.base64Decode(
    'ChVTd2FybUxpc3ROb2Rlc1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAiAASABSCH'
    'NlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use swarmListNodesResponseDescriptor instead')
const SwarmListNodesResponse$json = {
  '1': 'SwarmListNodesResponse',
  '2': [
    {
      '1': 'nodes',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmNode',
      '10': 'nodes'
    },
  ],
};

/// Descriptor for `SwarmListNodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmListNodesResponseDescriptor =
    $convert.base64Decode(
        'ChZTd2FybUxpc3ROb2Rlc1Jlc3BvbnNlEjEKBW5vZGVzGAEgAygLMhsub3Blbm94aWRlLmFwaS'
        '52MS5Td2FybU5vZGVSBW5vZGVz');

@$core.Deprecated('Use swarmPromoteNodeRequestDescriptor instead')
const SwarmPromoteNodeRequest$json = {
  '1': 'SwarmPromoteNodeRequest',
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
    {'1': 'node_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'nodeId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmPromoteNodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmPromoteNodeRequestDescriptor = $convert.base64Decode(
    'ChdTd2FybVByb21vdGVOb2RlUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIABIAF'
    'IIc2VydmVySWSIAQESIAoHbm9kZV9pZBgCIAEoCUIHukgEcgIQAVIGbm9kZUlkQgwKCl9zZXJ2'
    'ZXJfaWQ=');

@$core.Deprecated('Use swarmDemoteNodeRequestDescriptor instead')
const SwarmDemoteNodeRequest$json = {
  '1': 'SwarmDemoteNodeRequest',
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
    {'1': 'node_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'nodeId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmDemoteNodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmDemoteNodeRequestDescriptor = $convert.base64Decode(
    'ChZTd2FybURlbW90ZU5vZGVSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUg'
    'hzZXJ2ZXJJZIgBARIgCgdub2RlX2lkGAIgASgJQge6SARyAhABUgZub2RlSWRCDAoKX3NlcnZl'
    'cl9pZA==');

@$core.Deprecated('Use swarmUpdateNodeAvailabilityRequestDescriptor instead')
const SwarmUpdateNodeAvailabilityRequest$json = {
  '1': 'SwarmUpdateNodeAvailabilityRequest',
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
    {'1': 'node_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'nodeId'},
    {
      '1': 'availability',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SwarmNodeAvailability',
      '8': {},
      '10': 'availability'
    },
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmUpdateNodeAvailabilityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmUpdateNodeAvailabilityRequestDescriptor =
    $convert.base64Decode(
        'CiJTd2FybVVwZGF0ZU5vZGVBdmFpbGFiaWxpdHlSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0'
        'IHukgEIgIgAEgAUghzZXJ2ZXJJZIgBARIgCgdub2RlX2lkGAIgASgJQge6SARyAhABUgZub2Rl'
        'SWQSVwoMYXZhaWxhYmlsaXR5GAMgASgOMicub3Blbm94aWRlLmFwaS52MS5Td2FybU5vZGVBdm'
        'FpbGFiaWxpdHlCCrpIB4IBBBABIABSDGF2YWlsYWJpbGl0eUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use swarmRemoveNodeRequestDescriptor instead')
const SwarmRemoveNodeRequest$json = {
  '1': 'SwarmRemoveNodeRequest',
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
    {'1': 'node_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'nodeId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmRemoveNodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmRemoveNodeRequestDescriptor = $convert.base64Decode(
    'ChZTd2FybVJlbW92ZU5vZGVSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUg'
    'hzZXJ2ZXJJZIgBARIgCgdub2RlX2lkGAIgASgJQge6SARyAhABUgZub2RlSWRCDAoKX3NlcnZl'
    'cl9pZA==');

@$core.Deprecated('Use swarmLeaveRequestDescriptor instead')
const SwarmLeaveRequest$json = {
  '1': 'SwarmLeaveRequest',
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
    {'1': 'force', '3': 2, '4': 1, '5': 8, '10': 'force'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `SwarmLeaveRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmLeaveRequestDescriptor = $convert.base64Decode(
    'ChFTd2FybUxlYXZlUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIABIAFIIc2Vydm'
    'VySWSIAQESFAoFZm9yY2UYAiABKAhSBWZvcmNlQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use swarmJoinRequestDescriptor instead')
const SwarmJoinRequest$json = {
  '1': 'SwarmJoinRequest',
  '2': [
    {
      '1': 'target_server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'targetServerId'
    },
    {
      '1': 'manager_server_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'managerServerId',
      '17': true
    },
    {
      '1': 'role',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SwarmRole',
      '8': {},
      '10': 'role'
    },
  ],
  '8': [
    {'1': '_manager_server_id'},
  ],
};

/// Descriptor for `SwarmJoinRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmJoinRequestDescriptor = $convert.base64Decode(
    'ChBTd2FybUpvaW5SZXF1ZXN0EjEKEHRhcmdldF9zZXJ2ZXJfaWQYASABKANCB7pIBCICIABSDn'
    'RhcmdldFNlcnZlcklkEjgKEW1hbmFnZXJfc2VydmVyX2lkGAIgASgDQge6SAQiAiAASABSD21h'
    'bmFnZXJTZXJ2ZXJJZIgBARI7CgRyb2xlGAMgASgOMhsub3Blbm94aWRlLmFwaS52MS5Td2FybV'
    'JvbGVCCrpIB4IBBBABIABSBHJvbGVCFAoSX21hbmFnZXJfc2VydmVyX2lk');

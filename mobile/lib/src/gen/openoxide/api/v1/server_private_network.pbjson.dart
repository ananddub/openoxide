// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_private_network.proto.

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

@$core.Deprecated('Use serverConnectionModeDescriptor instead')
const ServerConnectionMode$json = {
  '1': 'ServerConnectionMode',
  '2': [
    {'1': 'SERVER_CONNECTION_MODE_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_CONNECTION_MODE_DIRECT_SSH', '2': 1},
    {'1': 'SERVER_CONNECTION_MODE_MANAGED_WIREGUARD', '2': 2},
    {'1': 'SERVER_CONNECTION_MODE_EXTERNAL_PRIVATE_NETWORK', '2': 3},
  ],
};

/// Descriptor for `ServerConnectionMode`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverConnectionModeDescriptor = $convert.base64Decode(
    'ChRTZXJ2ZXJDb25uZWN0aW9uTW9kZRImCiJTRVJWRVJfQ09OTkVDVElPTl9NT0RFX1VOU1BFQ0'
    'lGSUVEEAASJQohU0VSVkVSX0NPTk5FQ1RJT05fTU9ERV9ESVJFQ1RfU1NIEAESLAooU0VSVkVS'
    'X0NPTk5FQ1RJT05fTU9ERV9NQU5BR0VEX1dJUkVHVUFSRBACEjMKL1NFUlZFUl9DT05ORUNUSU'
    '9OX01PREVfRVhURVJOQUxfUFJJVkFURV9ORVRXT1JLEAM=');

@$core.Deprecated('Use serverPrivateNetworkProviderDescriptor instead')
const ServerPrivateNetworkProvider$json = {
  '1': 'ServerPrivateNetworkProvider',
  '2': [
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_WIREGUARD', '2': 1},
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_TAILSCALE', '2': 2},
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_ZEROTIER', '2': 3},
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_NETBIRD', '2': 4},
    {'1': 'SERVER_PRIVATE_NETWORK_PROVIDER_CUSTOM', '2': 5},
  ],
};

/// Descriptor for `ServerPrivateNetworkProvider`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkProviderDescriptor = $convert.base64Decode(
    'ChxTZXJ2ZXJQcml2YXRlTmV0d29ya1Byb3ZpZGVyEi8KK1NFUlZFUl9QUklWQVRFX05FVFdPUk'
    'tfUFJPVklERVJfVU5TUEVDSUZJRUQQABItCilTRVJWRVJfUFJJVkFURV9ORVRXT1JLX1BST1ZJ'
    'REVSX1dJUkVHVUFSRBABEi0KKVNFUlZFUl9QUklWQVRFX05FVFdPUktfUFJPVklERVJfVEFJTF'
    'NDQUxFEAISLAooU0VSVkVSX1BSSVZBVEVfTkVUV09SS19QUk9WSURFUl9aRVJPVElFUhADEisK'
    'J1NFUlZFUl9QUklWQVRFX05FVFdPUktfUFJPVklERVJfTkVUQklSRBAEEioKJlNFUlZFUl9QUk'
    'lWQVRFX05FVFdPUktfUFJPVklERVJfQ1VTVE9NEAU=');

@$core.Deprecated('Use serverPrivateNetworkStatusDescriptor instead')
const ServerPrivateNetworkStatus$json = {
  '1': 'ServerPrivateNetworkStatus',
  '2': [
    {'1': 'SERVER_PRIVATE_NETWORK_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_PRIVATE_NETWORK_STATUS_DISABLED', '2': 1},
    {'1': 'SERVER_PRIVATE_NETWORK_STATUS_CONFIGURING', '2': 2},
    {'1': 'SERVER_PRIVATE_NETWORK_STATUS_ACTIVE', '2': 3},
    {'1': 'SERVER_PRIVATE_NETWORK_STATUS_FAILED', '2': 4},
  ],
};

/// Descriptor for `ServerPrivateNetworkStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkStatusDescriptor = $convert.base64Decode(
    'ChpTZXJ2ZXJQcml2YXRlTmV0d29ya1N0YXR1cxItCilTRVJWRVJfUFJJVkFURV9ORVRXT1JLX1'
    'NUQVRVU19VTlNQRUNJRklFRBAAEioKJlNFUlZFUl9QUklWQVRFX05FVFdPUktfU1RBVFVTX0RJ'
    'U0FCTEVEEAESLQopU0VSVkVSX1BSSVZBVEVfTkVUV09SS19TVEFUVVNfQ09ORklHVVJJTkcQAh'
    'IoCiRTRVJWRVJfUFJJVkFURV9ORVRXT1JLX1NUQVRVU19BQ1RJVkUQAxIoCiRTRVJWRVJfUFJJ'
    'VkFURV9ORVRXT1JLX1NUQVRVU19GQUlMRUQQBA==');

@$core.Deprecated('Use serverPrivateNetworkHealthStatusDescriptor instead')
const ServerPrivateNetworkHealthStatus$json = {
  '1': 'ServerPrivateNetworkHealthStatus',
  '2': [
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNKNOWN', '2': 1},
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_HEALTHY', '2': 2},
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_DEGRADED', '2': 3},
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNREACHABLE', '2': 4},
    {'1': 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_CONFIG_DRIFT', '2': 5},
  ],
};

/// Descriptor for `ServerPrivateNetworkHealthStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkHealthStatusDescriptor = $convert.base64Decode(
    'CiBTZXJ2ZXJQcml2YXRlTmV0d29ya0hlYWx0aFN0YXR1cxI0CjBTRVJWRVJfUFJJVkFURV9ORV'
    'RXT1JLX0hFQUxUSF9TVEFUVVNfVU5TUEVDSUZJRUQQABIwCixTRVJWRVJfUFJJVkFURV9ORVRX'
    'T1JLX0hFQUxUSF9TVEFUVVNfVU5LTk9XThABEjAKLFNFUlZFUl9QUklWQVRFX05FVFdPUktfSE'
    'VBTFRIX1NUQVRVU19IRUFMVEhZEAISMQotU0VSVkVSX1BSSVZBVEVfTkVUV09SS19IRUFMVEhf'
    'U1RBVFVTX0RFR1JBREVEEAMSNAowU0VSVkVSX1BSSVZBVEVfTkVUV09SS19IRUFMVEhfU1RBVF'
    'VTX1VOUkVBQ0hBQkxFEAQSNQoxU0VSVkVSX1BSSVZBVEVfTkVUV09SS19IRUFMVEhfU1RBVFVT'
    'X0NPTkZJR19EUklGVBAF');

@$core.Deprecated('Use serverPrivateNetworkRotationStateDescriptor instead')
const ServerPrivateNetworkRotationState$json = {
  '1': 'ServerPrivateNetworkRotationState',
  '2': [
    {'1': 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_IDLE', '2': 1},
    {'1': 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROTATING', '2': 2},
    {'1': 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROLLING_BACK', '2': 3},
    {'1': 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_FAILED', '2': 4},
  ],
};

/// Descriptor for `ServerPrivateNetworkRotationState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkRotationStateDescriptor = $convert.base64Decode(
    'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya1JvdGF0aW9uU3RhdGUSNQoxU0VSVkVSX1BSSVZBVEVfTk'
    'VUV09SS19ST1RBVElPTl9TVEFURV9VTlNQRUNJRklFRBAAEi4KKlNFUlZFUl9QUklWQVRFX05F'
    'VFdPUktfUk9UQVRJT05fU1RBVEVfSURMRRABEjIKLlNFUlZFUl9QUklWQVRFX05FVFdPUktfUk'
    '9UQVRJT05fU1RBVEVfUk9UQVRJTkcQAhI2CjJTRVJWRVJfUFJJVkFURV9ORVRXT1JLX1JPVEFU'
    'SU9OX1NUQVRFX1JPTExJTkdfQkFDSxADEjAKLFNFUlZFUl9QUklWQVRFX05FVFdPUktfUk9UQV'
    'RJT05fU1RBVEVfRkFJTEVEEAQ=');

@$core.Deprecated('Use serverPrivateNetworkDescriptor instead')
const ServerPrivateNetwork$json = {
  '1': 'ServerPrivateNetwork',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '10': 'serverId'},
    {
      '1': 'connection_mode',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerConnectionMode',
      '10': 'connectionMode'
    },
    {
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkProvider',
      '9': 0,
      '10': 'provider',
      '17': true
    },
    {
      '1': 'private_host',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'privateHost',
      '17': true
    },
    {
      '1': 'tunnel_address',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'tunnelAddress',
      '17': true
    },
    {
      '1': 'public_key',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'publicKey',
      '17': true
    },
    {
      '1': 'endpoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'endpoint',
      '17': true
    },
    {
      '1': 'listen_port',
      '3': 8,
      '4': 1,
      '5': 13,
      '9': 5,
      '10': 'listenPort',
      '17': true
    },
    {
      '1': 'persistent_keepalive',
      '3': 9,
      '4': 1,
      '5': 13,
      '9': 6,
      '10': 'persistentKeepalive',
      '17': true
    },
    {
      '1': 'status',
      '3': 10,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkStatus',
      '10': 'status'
    },
    {
      '1': 'last_handshake_at',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 7,
      '10': 'lastHandshakeAt',
      '17': true
    },
    {'1': 'config_version', '3': 12, '4': 1, '5': 3, '10': 'configVersion'},
    {
      '1': 'dns_name',
      '3': 13,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'dnsName',
      '17': true
    },
    {'1': 'routes', '3': 14, '4': 3, '5': 9, '10': 'routes'},
    {
      '1': 'health_status',
      '3': 15,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkHealthStatus',
      '10': 'healthStatus'
    },
    {
      '1': 'health_error',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'healthError',
      '17': true
    },
    {
      '1': 'last_health_check_at',
      '3': 17,
      '4': 1,
      '5': 3,
      '9': 10,
      '10': 'lastHealthCheckAt',
      '17': true
    },
    {
      '1': 'consecutive_failures',
      '3': 18,
      '4': 1,
      '5': 3,
      '10': 'consecutiveFailures'
    },
    {
      '1': 'rotation_state',
      '3': 19,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkRotationState',
      '10': 'rotationState'
    },
    {'1': 'updated_at', '3': 20, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_provider'},
    {'1': '_private_host'},
    {'1': '_tunnel_address'},
    {'1': '_public_key'},
    {'1': '_endpoint'},
    {'1': '_listen_port'},
    {'1': '_persistent_keepalive'},
    {'1': '_last_handshake_at'},
    {'1': '_dns_name'},
    {'1': '_health_error'},
    {'1': '_last_health_check_at'},
  ],
};

/// Descriptor for `ServerPrivateNetwork`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkDescriptor = $convert.base64Decode(
    'ChRTZXJ2ZXJQcml2YXRlTmV0d29yaxIbCglzZXJ2ZXJfaWQYASABKANSCHNlcnZlcklkEk8KD2'
    'Nvbm5lY3Rpb25fbW9kZRgCIAEoDjImLm9wZW5veGlkZS5hcGkudjEuU2VydmVyQ29ubmVjdGlv'
    'bk1vZGVSDmNvbm5lY3Rpb25Nb2RlEk8KCHByb3ZpZGVyGAMgASgOMi4ub3Blbm94aWRlLmFwaS'
    '52MS5TZXJ2ZXJQcml2YXRlTmV0d29ya1Byb3ZpZGVySABSCHByb3ZpZGVyiAEBEiYKDHByaXZh'
    'dGVfaG9zdBgEIAEoCUgBUgtwcml2YXRlSG9zdIgBARIqCg50dW5uZWxfYWRkcmVzcxgFIAEoCU'
    'gCUg10dW5uZWxBZGRyZXNziAEBEiIKCnB1YmxpY19rZXkYBiABKAlIA1IJcHVibGljS2V5iAEB'
    'Eh8KCGVuZHBvaW50GAcgASgJSARSCGVuZHBvaW50iAEBEiQKC2xpc3Rlbl9wb3J0GAggASgNSA'
    'VSCmxpc3RlblBvcnSIAQESNgoUcGVyc2lzdGVudF9rZWVwYWxpdmUYCSABKA1IBlITcGVyc2lz'
    'dGVudEtlZXBhbGl2ZYgBARJECgZzdGF0dXMYCiABKA4yLC5vcGVub3hpZGUuYXBpLnYxLlNlcn'
    'ZlclByaXZhdGVOZXR3b3JrU3RhdHVzUgZzdGF0dXMSLwoRbGFzdF9oYW5kc2hha2VfYXQYCyAB'
    'KANIB1IPbGFzdEhhbmRzaGFrZUF0iAEBEiUKDmNvbmZpZ192ZXJzaW9uGAwgASgDUg1jb25maW'
    'dWZXJzaW9uEh4KCGRuc19uYW1lGA0gASgJSAhSB2Ruc05hbWWIAQESFgoGcm91dGVzGA4gAygJ'
    'UgZyb3V0ZXMSVwoNaGVhbHRoX3N0YXR1cxgPIAEoDjIyLm9wZW5veGlkZS5hcGkudjEuU2Vydm'
    'VyUHJpdmF0ZU5ldHdvcmtIZWFsdGhTdGF0dXNSDGhlYWx0aFN0YXR1cxImCgxoZWFsdGhfZXJy'
    'b3IYECABKAlICVILaGVhbHRoRXJyb3KIAQESNAoUbGFzdF9oZWFsdGhfY2hlY2tfYXQYESABKA'
    'NIClIRbGFzdEhlYWx0aENoZWNrQXSIAQESMQoUY29uc2VjdXRpdmVfZmFpbHVyZXMYEiABKANS'
    'E2NvbnNlY3V0aXZlRmFpbHVyZXMSWgoOcm90YXRpb25fc3RhdGUYEyABKA4yMy5vcGVub3hpZG'
    'UuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUm90YXRpb25TdGF0ZVINcm90YXRpb25TdGF0'
    'ZRIdCgp1cGRhdGVkX2F0GBQgASgDUgl1cGRhdGVkQXRCCwoJX3Byb3ZpZGVyQg8KDV9wcml2YX'
    'RlX2hvc3RCEQoPX3R1bm5lbF9hZGRyZXNzQg0KC19wdWJsaWNfa2V5QgsKCV9lbmRwb2ludEIO'
    'CgxfbGlzdGVuX3BvcnRCFwoVX3BlcnNpc3RlbnRfa2VlcGFsaXZlQhQKEl9sYXN0X2hhbmRzaG'
    'FrZV9hdEILCglfZG5zX25hbWVCDwoNX2hlYWx0aF9lcnJvckIXChVfbGFzdF9oZWFsdGhfY2hl'
    'Y2tfYXQ=');

@$core.Deprecated('Use serverPrivateNetworkHealthDescriptor instead')
const ServerPrivateNetworkHealth$json = {
  '1': 'ServerPrivateNetworkHealth',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkHealthStatus',
      '10': 'status'
    },
    {'1': 'interface_exists', '3': 2, '4': 1, '5': 8, '10': 'interfaceExists'},
    {
      '1': 'private_ssh_reachable',
      '3': 3,
      '4': 1,
      '5': 8,
      '10': 'privateSshReachable'
    },
    {
      '1': 'latest_handshake_at',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'latestHandshakeAt',
      '17': true
    },
    {'1': 'checked_at', '3': 5, '4': 1, '5': 3, '10': 'checkedAt'},
    {'1': 'error', '3': 6, '4': 1, '5': 9, '9': 1, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_latest_handshake_at'},
    {'1': '_error'},
  ],
};

/// Descriptor for `ServerPrivateNetworkHealth`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkHealthDescriptor = $convert.base64Decode(
    'ChpTZXJ2ZXJQcml2YXRlTmV0d29ya0hlYWx0aBJKCgZzdGF0dXMYASABKA4yMi5vcGVub3hpZG'
    'UuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrSGVhbHRoU3RhdHVzUgZzdGF0dXMSKQoQaW50'
    'ZXJmYWNlX2V4aXN0cxgCIAEoCFIPaW50ZXJmYWNlRXhpc3RzEjIKFXByaXZhdGVfc3NoX3JlYW'
    'NoYWJsZRgDIAEoCFITcHJpdmF0ZVNzaFJlYWNoYWJsZRIzChNsYXRlc3RfaGFuZHNoYWtlX2F0'
    'GAQgASgDSABSEWxhdGVzdEhhbmRzaGFrZUF0iAEBEh0KCmNoZWNrZWRfYXQYBSABKANSCWNoZW'
    'NrZWRBdBIZCgVlcnJvchgGIAEoCUgBUgVlcnJvcogBAUIWChRfbGF0ZXN0X2hhbmRzaGFrZV9h'
    'dEIICgZfZXJyb3I=');

@$core.Deprecated('Use serverPrivateNetworkGetRequestDescriptor instead')
const ServerPrivateNetworkGetRequest$json = {
  '1': 'ServerPrivateNetworkGetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkGetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkGetRequestDescriptor =
    $convert.base64Decode(
        'Ch5TZXJ2ZXJQcml2YXRlTmV0d29ya0dldFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
        'lk');

@$core.Deprecated('Use serverPrivateNetworkGetResponseDescriptor instead')
const ServerPrivateNetworkGetResponse$json = {
  '1': 'ServerPrivateNetworkGetResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkGetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkGetResponseDescriptor =
    $convert.base64Decode(
        'Ch9TZXJ2ZXJQcml2YXRlTmV0d29ya0dldFJlc3BvbnNlEkAKB25ldHdvcmsYASABKAsyJi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use serverPrivateNetworkUpdateRequestDescriptor instead')
const ServerPrivateNetworkUpdateRequest$json = {
  '1': 'ServerPrivateNetworkUpdateRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'connection_mode',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerConnectionMode',
      '8': {},
      '10': 'connectionMode'
    },
    {
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkProvider',
      '8': {},
      '9': 0,
      '10': 'provider',
      '17': true
    },
    {
      '1': 'private_host',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'privateHost',
      '17': true
    },
    {
      '1': 'tunnel_address',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'tunnelAddress',
      '17': true
    },
    {
      '1': 'public_key',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'publicKey',
      '17': true
    },
    {
      '1': 'endpoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'endpoint',
      '17': true
    },
    {
      '1': 'listen_port',
      '3': 8,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 5,
      '10': 'listenPort',
      '17': true
    },
    {
      '1': 'persistent_keepalive',
      '3': 9,
      '4': 1,
      '5': 13,
      '9': 6,
      '10': 'persistentKeepalive',
      '17': true
    },
    {
      '1': 'dns_name',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'dnsName',
      '17': true
    },
    {'1': 'routes', '3': 11, '4': 3, '5': 9, '8': {}, '10': 'routes'},
  ],
  '8': [
    {'1': '_provider'},
    {'1': '_private_host'},
    {'1': '_tunnel_address'},
    {'1': '_public_key'},
    {'1': '_endpoint'},
    {'1': '_listen_port'},
    {'1': '_persistent_keepalive'},
    {'1': '_dns_name'},
  ],
};

/// Descriptor for `ServerPrivateNetworkUpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkUpdateRequestDescriptor = $convert.base64Decode(
    'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya1VwZGF0ZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
    'BSAmlkElsKD2Nvbm5lY3Rpb25fbW9kZRgCIAEoDjImLm9wZW5veGlkZS5hcGkudjEuU2VydmVy'
    'Q29ubmVjdGlvbk1vZGVCCrpIB4IBBBABIABSDmNvbm5lY3Rpb25Nb2RlElkKCHByb3ZpZGVyGA'
    'MgASgOMi4ub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJQcml2YXRlTmV0d29ya1Byb3ZpZGVyQgi6'
    'SAWCAQIQAUgAUghwcm92aWRlcogBARIwCgxwcml2YXRlX2hvc3QYBCABKAlCCLpIBXIDGP8BSA'
    'FSC3ByaXZhdGVIb3N0iAEBEjQKDnR1bm5lbF9hZGRyZXNzGAUgASgJQgi6SAVyAxj/AUgCUg10'
    'dW5uZWxBZGRyZXNziAEBEiwKCnB1YmxpY19rZXkYBiABKAlCCLpIBXIDGP8BSANSCXB1YmxpY0'
    'tleYgBARIpCghlbmRwb2ludBgHIAEoCUIIukgFcgMY/wFIBFIIZW5kcG9pbnSIAQESLwoLbGlz'
    'dGVuX3BvcnQYCCABKA1CCbpIBioEGP//A0gFUgpsaXN0ZW5Qb3J0iAEBEjYKFHBlcnNpc3Rlbn'
    'Rfa2VlcGFsaXZlGAkgASgNSAZSE3BlcnNpc3RlbnRLZWVwYWxpdmWIAQESKAoIZG5zX25hbWUY'
    'CiABKAlCCLpIBXIDGP8BSAdSB2Ruc05hbWWIAQESIAoGcm91dGVzGAsgAygJQgi6SAWSAQIQQF'
    'IGcm91dGVzQgsKCV9wcm92aWRlckIPCg1fcHJpdmF0ZV9ob3N0QhEKD190dW5uZWxfYWRkcmVz'
    'c0INCgtfcHVibGljX2tleUILCglfZW5kcG9pbnRCDgoMX2xpc3Rlbl9wb3J0QhcKFV9wZXJzaX'
    'N0ZW50X2tlZXBhbGl2ZUILCglfZG5zX25hbWU=');

@$core.Deprecated('Use serverPrivateNetworkUpdateResponseDescriptor instead')
const ServerPrivateNetworkUpdateResponse$json = {
  '1': 'ServerPrivateNetworkUpdateResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkUpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkUpdateResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJQcml2YXRlTmV0d29ya1VwZGF0ZVJlc3BvbnNlEkAKB25ldHdvcmsYASABKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use serverPrivateNetworkDeleteRequestDescriptor instead')
const ServerPrivateNetworkDeleteRequest$json = {
  '1': 'ServerPrivateNetworkDeleteRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkDeleteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkDeleteRequestDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya0RlbGV0ZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use serverPrivateNetworkSetupRequestDescriptor instead')
const ServerPrivateNetworkSetupRequest$json = {
  '1': 'ServerPrivateNetworkSetupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkSetupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkSetupRequestDescriptor =
    $convert.base64Decode(
        'CiBTZXJ2ZXJQcml2YXRlTmV0d29ya1NldHVwUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAF'
        'ICaWQ=');

@$core.Deprecated('Use serverPrivateNetworkSetupResponseDescriptor instead')
const ServerPrivateNetworkSetupResponse$json = {
  '1': 'ServerPrivateNetworkSetupResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkSetupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkSetupResponseDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya1NldHVwUmVzcG9uc2USQAoHbmV0d29yaxgBIAEoCzImLm'
        '9wZW5veGlkZS5hcGkudjEuU2VydmVyUHJpdmF0ZU5ldHdvcmtSB25ldHdvcms=');

@$core.Deprecated('Use serverPrivateNetworkHealthRequestDescriptor instead')
const ServerPrivateNetworkHealthRequest$json = {
  '1': 'ServerPrivateNetworkHealthRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkHealthRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkHealthRequestDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya0hlYWx0aFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use serverPrivateNetworkHealthResponseDescriptor instead')
const ServerPrivateNetworkHealthResponse$json = {
  '1': 'ServerPrivateNetworkHealthResponse',
  '2': [
    {
      '1': 'health',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetworkHealth',
      '10': 'health'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkHealthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkHealthResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJQcml2YXRlTmV0d29ya0hlYWx0aFJlc3BvbnNlEkQKBmhlYWx0aBgBIAEoCzIsLm'
        '9wZW5veGlkZS5hcGkudjEuU2VydmVyUHJpdmF0ZU5ldHdvcmtIZWFsdGhSBmhlYWx0aA==');

@$core.Deprecated('Use serverPrivateNetworkRepairRequestDescriptor instead')
const ServerPrivateNetworkRepairRequest$json = {
  '1': 'ServerPrivateNetworkRepairRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkRepairRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkRepairRequestDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJQcml2YXRlTmV0d29ya1JlcGFpclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use serverPrivateNetworkRepairResponseDescriptor instead')
const ServerPrivateNetworkRepairResponse$json = {
  '1': 'ServerPrivateNetworkRepairResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkRepairResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkRepairResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJQcml2YXRlTmV0d29ya1JlcGFpclJlc3BvbnNlEkAKB25ldHdvcmsYASABKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use serverPrivateNetworkReSetupRequestDescriptor instead')
const ServerPrivateNetworkReSetupRequest$json = {
  '1': 'ServerPrivateNetworkReSetupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkReSetupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkReSetupRequestDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJQcml2YXRlTmV0d29ya1JlU2V0dXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
        'AAUgJpZA==');

@$core.Deprecated('Use serverPrivateNetworkReSetupResponseDescriptor instead')
const ServerPrivateNetworkReSetupResponse$json = {
  '1': 'ServerPrivateNetworkReSetupResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkReSetupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkReSetupResponseDescriptor =
    $convert.base64Decode(
        'CiNTZXJ2ZXJQcml2YXRlTmV0d29ya1JlU2V0dXBSZXNwb25zZRJACgduZXR3b3JrGAEgASgLMi'
        'Yub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJQcml2YXRlTmV0d29ya1IHbmV0d29yaw==');

@$core.Deprecated('Use serverPrivateNetworkRotateKeysRequestDescriptor instead')
const ServerPrivateNetworkRotateKeysRequest$json = {
  '1': 'ServerPrivateNetworkRotateKeysRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkRotateKeysRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkRotateKeysRequestDescriptor =
    $convert.base64Decode(
        'CiVTZXJ2ZXJQcml2YXRlTmV0d29ya1JvdGF0ZUtleXNSZXF1ZXN0EhcKAmlkGAEgASgDQge6SA'
        'QiAiAAUgJpZA==');

@$core
    .Deprecated('Use serverPrivateNetworkRotateKeysResponseDescriptor instead')
const ServerPrivateNetworkRotateKeysResponse$json = {
  '1': 'ServerPrivateNetworkRotateKeysResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerPrivateNetworkRotateKeysResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkRotateKeysResponseDescriptor =
    $convert.base64Decode(
        'CiZTZXJ2ZXJQcml2YXRlTmV0d29ya1JvdGF0ZUtleXNSZXNwb25zZRJACgduZXR3b3JrGAEgAS'
        'gLMiYub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJQcml2YXRlTmV0d29ya1IHbmV0d29yaw==');

@$core.Deprecated('Use serverPrivateNetworkTeardownRequestDescriptor instead')
const ServerPrivateNetworkTeardownRequest$json = {
  '1': 'ServerPrivateNetworkTeardownRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerPrivateNetworkTeardownRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPrivateNetworkTeardownRequestDescriptor =
    $convert.base64Decode(
        'CiNTZXJ2ZXJQcml2YXRlTmV0d29ya1RlYXJkb3duUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIg'
        'IgAFICaWQ=');

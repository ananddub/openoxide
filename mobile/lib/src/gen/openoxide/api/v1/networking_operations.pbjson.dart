// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/networking_operations.proto.

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

@$core.Deprecated('Use networkingCdnProviderDescriptor instead')
const NetworkingCdnProvider$json = {
  '1': 'NetworkingCdnProvider',
  '2': [
    {'1': 'NETWORKING_CDN_PROVIDER_UNSPECIFIED', '2': 0},
    {'1': 'NETWORKING_CDN_PROVIDER_CLOUDFLARE', '2': 1},
    {'1': 'NETWORKING_CDN_PROVIDER_FASTLY', '2': 2},
    {'1': 'NETWORKING_CDN_PROVIDER_BUNNY', '2': 3},
  ],
};

/// Descriptor for `NetworkingCdnProvider`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List networkingCdnProviderDescriptor = $convert.base64Decode(
    'ChVOZXR3b3JraW5nQ2RuUHJvdmlkZXISJwojTkVUV09SS0lOR19DRE5fUFJPVklERVJfVU5TUE'
    'VDSUZJRUQQABImCiJORVRXT1JLSU5HX0NETl9QUk9WSURFUl9DTE9VREZMQVJFEAESIgoeTkVU'
    'V09SS0lOR19DRE5fUFJPVklERVJfRkFTVExZEAISIQodTkVUV09SS0lOR19DRE5fUFJPVklERV'
    'JfQlVOTlkQAw==');

@$core.Deprecated('Use networkingDomainDiagnosticDescriptor instead')
const NetworkingDomainDiagnostic$json = {
  '1': 'NetworkingDomainDiagnostic',
  '2': [
    {'1': 'host', '3': 1, '4': 1, '5': 9, '10': 'host'},
    {
      '1': 'resolved_addresses',
      '3': 2,
      '4': 3,
      '5': 9,
      '10': 'resolvedAddresses'
    },
    {'1': 'dns_ok', '3': 3, '4': 1, '5': 8, '10': 'dnsOk'},
    {'1': 'tcp_ok', '3': 4, '4': 1, '5': 8, '10': 'tcpOk'},
    {'1': 'http_ok', '3': 5, '4': 1, '5': 8, '10': 'httpOk'},
    {
      '1': 'http_status',
      '3': 6,
      '4': 1,
      '5': 13,
      '9': 0,
      '10': 'httpStatus',
      '17': true
    },
    {'1': 'error', '3': 7, '4': 1, '5': 9, '9': 1, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_http_status'},
    {'1': '_error'},
  ],
};

/// Descriptor for `NetworkingDomainDiagnostic`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingDomainDiagnosticDescriptor = $convert.base64Decode(
    'ChpOZXR3b3JraW5nRG9tYWluRGlhZ25vc3RpYxISCgRob3N0GAEgASgJUgRob3N0Ei0KEnJlc2'
    '9sdmVkX2FkZHJlc3NlcxgCIAMoCVIRcmVzb2x2ZWRBZGRyZXNzZXMSFQoGZG5zX29rGAMgASgI'
    'UgVkbnNPaxIVCgZ0Y3Bfb2sYBCABKAhSBXRjcE9rEhcKB2h0dHBfb2sYBSABKAhSBmh0dHBPax'
    'IkCgtodHRwX3N0YXR1cxgGIAEoDUgAUgpodHRwU3RhdHVziAEBEhkKBWVycm9yGAcgASgJSAFS'
    'BWVycm9yiAEBQg4KDF9odHRwX3N0YXR1c0IICgZfZXJyb3I=');

@$core.Deprecated('Use networkingRootNetworkStatusDescriptor instead')
const NetworkingRootNetworkStatus$json = {
  '1': 'NetworkingRootNetworkStatus',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'exists', '3': 2, '4': 1, '5': 8, '10': 'exists'},
    {'1': 'healthy', '3': 3, '4': 1, '5': 8, '10': 'healthy'},
    {'1': 'repaired', '3': 4, '4': 1, '5': 8, '10': 'repaired'},
    {'1': 'driver', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'driver', '17': true},
    {'1': 'scope', '3': 6, '4': 1, '5': 9, '9': 1, '10': 'scope', '17': true},
    {
      '1': 'attachable',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 2,
      '10': 'attachable',
      '17': true
    },
    {
      '1': 'connected_resources',
      '3': 8,
      '4': 1,
      '5': 3,
      '10': 'connectedResources'
    },
    {'1': 'issue', '3': 9, '4': 1, '5': 9, '9': 3, '10': 'issue', '17': true},
  ],
  '8': [
    {'1': '_driver'},
    {'1': '_scope'},
    {'1': '_attachable'},
    {'1': '_issue'},
  ],
};

/// Descriptor for `NetworkingRootNetworkStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingRootNetworkStatusDescriptor = $convert.base64Decode(
    'ChtOZXR3b3JraW5nUm9vdE5ldHdvcmtTdGF0dXMSEgoEbmFtZRgBIAEoCVIEbmFtZRIWCgZleG'
    'lzdHMYAiABKAhSBmV4aXN0cxIYCgdoZWFsdGh5GAMgASgIUgdoZWFsdGh5EhoKCHJlcGFpcmVk'
    'GAQgASgIUghyZXBhaXJlZBIbCgZkcml2ZXIYBSABKAlIAFIGZHJpdmVyiAEBEhkKBXNjb3BlGA'
    'YgASgJSAFSBXNjb3BliAEBEiMKCmF0dGFjaGFibGUYByABKAhIAlIKYXR0YWNoYWJsZYgBARIv'
    'ChNjb25uZWN0ZWRfcmVzb3VyY2VzGAggASgDUhJjb25uZWN0ZWRSZXNvdXJjZXMSGQoFaXNzdW'
    'UYCSABKAlIA1IFaXNzdWWIAQFCCQoHX2RyaXZlckIICgZfc2NvcGVCDQoLX2F0dGFjaGFibGVC'
    'CAoGX2lzc3Vl');

@$core.Deprecated('Use networkingPurgeCdnRequestDescriptor instead')
const NetworkingPurgeCdnRequest$json = {
  '1': 'NetworkingPurgeCdnRequest',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.NetworkingCdnProvider',
      '8': {},
      '10': 'provider'
    },
    {'1': 'api_token', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'apiToken'},
    {
      '1': 'zone_id',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'zoneId',
      '17': true
    },
    {
      '1': 'service_id',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'pull_zone_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'pullZoneId',
      '17': true
    },
    {'1': 'purge_all', '3': 6, '4': 1, '5': 8, '10': 'purgeAll'},
    {'1': 'urls', '3': 7, '4': 3, '5': 9, '8': {}, '10': 'urls'},
  ],
  '8': [
    {'1': '_zone_id'},
    {'1': '_service_id'},
    {'1': '_pull_zone_id'},
  ],
};

/// Descriptor for `NetworkingPurgeCdnRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingPurgeCdnRequestDescriptor = $convert.base64Decode(
    'ChlOZXR3b3JraW5nUHVyZ2VDZG5SZXF1ZXN0Ek8KCHByb3ZpZGVyGAEgASgOMicub3Blbm94aW'
    'RlLmFwaS52MS5OZXR3b3JraW5nQ2RuUHJvdmlkZXJCCrpIB4IBBBABIABSCHByb3ZpZGVyEiQK'
    'CWFwaV90b2tlbhgCIAEoCUIHukgEcgIQAVIIYXBpVG9rZW4SJgoHem9uZV9pZBgDIAEoCUIIuk'
    'gFcgMY/wFIAFIGem9uZUlkiAEBEiwKCnNlcnZpY2VfaWQYBCABKAlCCLpIBXIDGP8BSAFSCXNl'
    'cnZpY2VJZIgBARIlCgxwdWxsX3pvbmVfaWQYBSABKANIAlIKcHVsbFpvbmVJZIgBARIbCglwdX'
    'JnZV9hbGwYBiABKAhSCHB1cmdlQWxsEh0KBHVybHMYByADKAlCCbpIBpIBAxDoB1IEdXJsc0IK'
    'Cghfem9uZV9pZEINCgtfc2VydmljZV9pZEIPCg1fcHVsbF96b25lX2lk');

@$core.Deprecated('Use networkingPurgeCdnResponseDescriptor instead')
const NetworkingPurgeCdnResponse$json = {
  '1': 'NetworkingPurgeCdnResponse',
  '2': [
    {'1': 'provider', '3': 1, '4': 1, '5': 9, '10': 'provider'},
    {'1': 'success', '3': 2, '4': 1, '5': 8, '10': 'success'},
    {'1': 'status', '3': 3, '4': 1, '5': 13, '10': 'status'},
    {
      '1': 'request_id',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'requestId',
      '17': true
    },
    {'1': 'message', '3': 5, '4': 1, '5': 9, '10': 'message'},
  ],
  '8': [
    {'1': '_request_id'},
  ],
};

/// Descriptor for `NetworkingPurgeCdnResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingPurgeCdnResponseDescriptor = $convert.base64Decode(
    'ChpOZXR3b3JraW5nUHVyZ2VDZG5SZXNwb25zZRIaCghwcm92aWRlchgBIAEoCVIIcHJvdmlkZX'
    'ISGAoHc3VjY2VzcxgCIAEoCFIHc3VjY2VzcxIWCgZzdGF0dXMYAyABKA1SBnN0YXR1cxIiCgpy'
    'ZXF1ZXN0X2lkGAQgASgJSABSCXJlcXVlc3RJZIgBARIYCgdtZXNzYWdlGAUgASgJUgdtZXNzYW'
    'dlQg0KC19yZXF1ZXN0X2lk');

@$core.Deprecated('Use networkingDiagnoseDomainsRequestDescriptor instead')
const NetworkingDiagnoseDomainsRequest$json = {
  '1': 'NetworkingDiagnoseDomainsRequest',
  '2': [
    {'1': 'host', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'host'},
    {
      '1': 'port',
      '3': 2,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 0,
      '10': 'port',
      '17': true
    },
    {'1': 'https', '3': 3, '4': 1, '5': 8, '10': 'https'},
    {
      '1': 'path',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'path',
      '17': true
    },
  ],
  '8': [
    {'1': '_port'},
    {'1': '_path'},
  ],
};

/// Descriptor for `NetworkingDiagnoseDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingDiagnoseDomainsRequestDescriptor =
    $convert.base64Decode(
        'CiBOZXR3b3JraW5nRGlhZ25vc2VEb21haW5zUmVxdWVzdBIeCgRob3N0GAEgASgJQgq6SAdyBR'
        'ABGP8BUgRob3N0EiIKBHBvcnQYAiABKA1CCbpIBioEGP//A0gAUgRwb3J0iAEBEhQKBWh0dHBz'
        'GAMgASgIUgVodHRwcxIhCgRwYXRoGAQgASgJQgi6SAVyAxjQD0gBUgRwYXRoiAEBQgcKBV9wb3'
        'J0QgcKBV9wYXRo');

@$core.Deprecated('Use networkingDiagnoseDomainsResponseDescriptor instead')
const NetworkingDiagnoseDomainsResponse$json = {
  '1': 'NetworkingDiagnoseDomainsResponse',
  '2': [
    {
      '1': 'diagnostic',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NetworkingDomainDiagnostic',
      '10': 'diagnostic'
    },
  ],
};

/// Descriptor for `NetworkingDiagnoseDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingDiagnoseDomainsResponseDescriptor =
    $convert.base64Decode(
        'CiFOZXR3b3JraW5nRGlhZ25vc2VEb21haW5zUmVzcG9uc2USTAoKZGlhZ25vc3RpYxgBIAEoCz'
        'IsLm9wZW5veGlkZS5hcGkudjEuTmV0d29ya2luZ0RvbWFpbkRpYWdub3N0aWNSCmRpYWdub3N0'
        'aWM=');

@$core.Deprecated('Use networkingDiagnoseRootNetworkRequestDescriptor instead')
const NetworkingDiagnoseRootNetworkRequest$json = {
  '1': 'NetworkingDiagnoseRootNetworkRequest',
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

/// Descriptor for `NetworkingDiagnoseRootNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingDiagnoseRootNetworkRequestDescriptor =
    $convert.base64Decode(
        'CiROZXR3b3JraW5nRGlhZ25vc2VSb290TmV0d29ya1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgAS'
        'gDQge6SAQiAiAASABSCHNlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use networkingDiagnoseRootNetworkResponseDescriptor instead')
const NetworkingDiagnoseRootNetworkResponse$json = {
  '1': 'NetworkingDiagnoseRootNetworkResponse',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NetworkingRootNetworkStatus',
      '10': 'status'
    },
  ],
};

/// Descriptor for `NetworkingDiagnoseRootNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingDiagnoseRootNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiVOZXR3b3JraW5nRGlhZ25vc2VSb290TmV0d29ya1Jlc3BvbnNlEkUKBnN0YXR1cxgBIAEoCz'
        'ItLm9wZW5veGlkZS5hcGkudjEuTmV0d29ya2luZ1Jvb3ROZXR3b3JrU3RhdHVzUgZzdGF0dXM=');

@$core.Deprecated('Use networkingRepairRootNetworkRequestDescriptor instead')
const NetworkingRepairRootNetworkRequest$json = {
  '1': 'NetworkingRepairRootNetworkRequest',
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

/// Descriptor for `NetworkingRepairRootNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingRepairRootNetworkRequestDescriptor =
    $convert.base64Decode(
        'CiJOZXR3b3JraW5nUmVwYWlyUm9vdE5ldHdvcmtSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0'
        'IHukgEIgIgAEgAUghzZXJ2ZXJJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use networkingRepairRootNetworkResponseDescriptor instead')
const NetworkingRepairRootNetworkResponse$json = {
  '1': 'NetworkingRepairRootNetworkResponse',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NetworkingRootNetworkStatus',
      '10': 'status'
    },
  ],
};

/// Descriptor for `NetworkingRepairRootNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List networkingRepairRootNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiNOZXR3b3JraW5nUmVwYWlyUm9vdE5ldHdvcmtSZXNwb25zZRJFCgZzdGF0dXMYASABKAsyLS'
        '5vcGVub3hpZGUuYXBpLnYxLk5ldHdvcmtpbmdSb290TmV0d29ya1N0YXR1c1IGc3RhdHVz');

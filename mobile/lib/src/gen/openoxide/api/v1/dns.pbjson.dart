// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/dns.proto.

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

@$core.Deprecated('Use dnsProviderTypeDescriptor instead')
const DnsProviderType$json = {
  '1': 'DnsProviderType',
  '2': [
    {'1': 'DNS_PROVIDER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'DNS_PROVIDER_TYPE_CLOUDFLARE', '2': 1},
    {'1': 'DNS_PROVIDER_TYPE_ROUTE53', '2': 2},
    {'1': 'DNS_PROVIDER_TYPE_PORKBUN', '2': 6},
    {'1': 'DNS_PROVIDER_TYPE_INFOMANIAK', '2': 7},
    {'1': 'DNS_PROVIDER_TYPE_OVH', '2': 8},
    {'1': 'DNS_PROVIDER_TYPE_SPACESHIP', '2': 16},
    {'1': 'DNS_PROVIDER_TYPE_HOSTINGER', '2': 38},
  ],
  '4': [
    {'1': 3, '2': 5},
    {'1': 9, '2': 15},
    {'1': 17, '2': 37},
    {'1': 39, '2': 69},
  ],
};

/// Descriptor for `DnsProviderType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List dnsProviderTypeDescriptor = $convert.base64Decode(
    'Cg9EbnNQcm92aWRlclR5cGUSIQodRE5TX1BST1ZJREVSX1RZUEVfVU5TUEVDSUZJRUQQABIgCh'
    'xETlNfUFJPVklERVJfVFlQRV9DTE9VREZMQVJFEAESHQoZRE5TX1BST1ZJREVSX1RZUEVfUk9V'
    'VEU1MxACEh0KGUROU19QUk9WSURFUl9UWVBFX1BPUktCVU4QBhIgChxETlNfUFJPVklERVJfVF'
    'lQRV9JTkZPTUFOSUFLEAcSGQoVRE5TX1BST1ZJREVSX1RZUEVfT1ZIEAgSHwobRE5TX1BST1ZJ'
    'REVSX1RZUEVfU1BBQ0VTSElQEBASHwobRE5TX1BST1ZJREVSX1RZUEVfSE9TVElOR0VSECYiBA'
    'gDEAUiBAgJEA8iBAgRECUiBAgnEEU=');

@$core.Deprecated('Use dnsProviderDescriptor instead')
const DnsProvider$json = {
  '1': 'DnsProvider',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'provider_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DnsProviderType',
      '10': 'providerType'
    },
    {'1': 'credentials_json', '3': 4, '4': 1, '5': 9, '10': 'credentialsJson'},
    {'1': 'organization_id', '3': 5, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 7, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `DnsProvider`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dnsProviderDescriptor = $convert.base64Decode(
    'CgtEbnNQcm92aWRlchIOCgJpZBgBIAEoA1ICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRJGCg1wcm'
    '92aWRlcl90eXBlGAMgASgOMiEub3Blbm94aWRlLmFwaS52MS5EbnNQcm92aWRlclR5cGVSDHBy'
    'b3ZpZGVyVHlwZRIpChBjcmVkZW50aWFsc19qc29uGAQgASgJUg9jcmVkZW50aWFsc0pzb24SJw'
    'oPb3JnYW5pemF0aW9uX2lkGAUgASgDUg5vcmdhbml6YXRpb25JZBIdCgpjcmVhdGVkX2F0GAYg'
    'ASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgHIAEoA1IJdXBkYXRlZEF0');

@$core.Deprecated('Use dnsZoneDescriptor instead')
const DnsZone$json = {
  '1': 'DnsZone',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'status', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'status', '17': true},
  ],
  '8': [
    {'1': '_status'},
  ],
};

/// Descriptor for `DnsZone`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dnsZoneDescriptor = $convert.base64Decode(
    'CgdEbnNab25lEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEhsKBnN0YXR1cx'
    'gDIAEoCUgAUgZzdGF0dXOIAQFCCQoHX3N0YXR1cw==');

@$core.Deprecated('Use dnsRecordDescriptor instead')
const DnsRecord$json = {
  '1': 'DnsRecord',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'zone_id', '3': 2, '4': 1, '5': 9, '10': 'zoneId'},
    {'1': 'record_type', '3': 3, '4': 1, '5': 9, '10': 'recordType'},
    {'1': 'name', '3': 4, '4': 1, '5': 9, '10': 'name'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '10': 'content'},
    {'1': 'ttl', '3': 6, '4': 1, '5': 13, '9': 0, '10': 'ttl', '17': true},
    {
      '1': 'proxied',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 1,
      '10': 'proxied',
      '17': true
    },
  ],
  '8': [
    {'1': '_ttl'},
    {'1': '_proxied'},
  ],
};

/// Descriptor for `DnsRecord`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dnsRecordDescriptor = $convert.base64Decode(
    'CglEbnNSZWNvcmQSDgoCaWQYASABKAlSAmlkEhcKB3pvbmVfaWQYAiABKAlSBnpvbmVJZBIfCg'
    'tyZWNvcmRfdHlwZRgDIAEoCVIKcmVjb3JkVHlwZRISCgRuYW1lGAQgASgJUgRuYW1lEhgKB2Nv'
    'bnRlbnQYBSABKAlSB2NvbnRlbnQSFQoDdHRsGAYgASgNSABSA3R0bIgBARIdCgdwcm94aWVkGA'
    'cgASgISAFSB3Byb3hpZWSIAQFCBgoEX3R0bEIKCghfcHJveGllZA==');

@$core.Deprecated('Use listDnsProvidersRequestDescriptor instead')
const ListDnsProvidersRequest$json = {
  '1': 'ListDnsProvidersRequest',
};

/// Descriptor for `ListDnsProvidersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsProvidersRequestDescriptor =
    $convert.base64Decode('ChdMaXN0RG5zUHJvdmlkZXJzUmVxdWVzdA==');

@$core.Deprecated('Use listDnsProvidersResponseDescriptor instead')
const ListDnsProvidersResponse$json = {
  '1': 'ListDnsProvidersResponse',
  '2': [
    {
      '1': 'providers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.DnsProvider',
      '10': 'providers'
    },
  ],
};

/// Descriptor for `ListDnsProvidersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsProvidersResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0RG5zUHJvdmlkZXJzUmVzcG9uc2USOwoJcHJvdmlkZXJzGAEgAygLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5EbnNQcm92aWRlclIJcHJvdmlkZXJz');

@$core.Deprecated('Use getDnsProviderRequestDescriptor instead')
const GetDnsProviderRequest$json = {
  '1': 'GetDnsProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetDnsProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDnsProviderRequestDescriptor =
    $convert.base64Decode(
        'ChVHZXREbnNQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use getDnsProviderResponseDescriptor instead')
const GetDnsProviderResponse$json = {
  '1': 'GetDnsProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DnsProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `GetDnsProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDnsProviderResponseDescriptor =
    $convert.base64Decode(
        'ChZHZXREbnNQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm94aWRlLm'
        'FwaS52MS5EbnNQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use createDnsProviderRequestDescriptor instead')
const CreateDnsProviderRequest$json = {
  '1': 'CreateDnsProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'provider_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DnsProviderType',
      '8': {},
      '10': 'providerType'
    },
    {
      '1': 'credentials_json',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'credentialsJson'
    },
  ],
};

/// Descriptor for `CreateDnsProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDnsProviderRequestDescriptor = $convert.base64Decode(
    'ChhDcmVhdGVEbnNQcm92aWRlclJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AVIEbm'
    'FtZRJSCg1wcm92aWRlcl90eXBlGAIgASgOMiEub3Blbm94aWRlLmFwaS52MS5EbnNQcm92aWRl'
    'clR5cGVCCrpIB4IBBBABIABSDHByb3ZpZGVyVHlwZRI2ChBjcmVkZW50aWFsc19qc29uGAMgAS'
    'gJQgu6SAhyBhABGICAQFIPY3JlZGVudGlhbHNKc29u');

@$core.Deprecated('Use createDnsProviderResponseDescriptor instead')
const CreateDnsProviderResponse$json = {
  '1': 'CreateDnsProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DnsProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `CreateDnsProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDnsProviderResponseDescriptor =
    $convert.base64Decode(
        'ChlDcmVhdGVEbnNQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5EbnNQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use updateDnsProviderRequestDescriptor instead')
const UpdateDnsProviderRequest$json = {
  '1': 'UpdateDnsProviderRequest',
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
      '1': 'provider_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DnsProviderType',
      '8': {},
      '9': 1,
      '10': 'providerType',
      '17': true
    },
    {
      '1': 'credentials_json',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'credentialsJson',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_provider_type'},
    {'1': '_credentials_json'},
  ],
};

/// Descriptor for `UpdateDnsProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDnsProviderRequestDescriptor = $convert.base64Decode(
    'ChhVcGRhdGVEbnNQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEiEKBG'
    '5hbWUYAiABKAlCCLpIBXIDGP8BSABSBG5hbWWIAQESVwoNcHJvdmlkZXJfdHlwZRgDIAEoDjIh'
    'Lm9wZW5veGlkZS5hcGkudjEuRG5zUHJvdmlkZXJUeXBlQgq6SAeCAQQQASAASAFSDHByb3ZpZG'
    'VyVHlwZYgBARI5ChBjcmVkZW50aWFsc19qc29uGAQgASgJQgm6SAZyBBiAgEBIAlIPY3JlZGVu'
    'dGlhbHNKc29uiAEBQgcKBV9uYW1lQhAKDl9wcm92aWRlcl90eXBlQhMKEV9jcmVkZW50aWFsc1'
    '9qc29u');

@$core.Deprecated('Use updateDnsProviderResponseDescriptor instead')
const UpdateDnsProviderResponse$json = {
  '1': 'UpdateDnsProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DnsProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateDnsProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDnsProviderResponseDescriptor =
    $convert.base64Decode(
        'ChlVcGRhdGVEbnNQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5EbnNQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use deleteDnsProviderRequestDescriptor instead')
const DeleteDnsProviderRequest$json = {
  '1': 'DeleteDnsProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteDnsProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteDnsProviderRequestDescriptor =
    $convert.base64Decode(
        'ChhEZWxldGVEbnNQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use testDnsProviderRequestDescriptor instead')
const TestDnsProviderRequest$json = {
  '1': 'TestDnsProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `TestDnsProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDnsProviderRequestDescriptor =
    $convert.base64Decode(
        'ChZUZXN0RG5zUHJvdmlkZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use testDnsProviderResponseDescriptor instead')
const TestDnsProviderResponse$json = {
  '1': 'TestDnsProviderResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestDnsProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDnsProviderResponseDescriptor =
    $convert.base64Decode(
        'ChdUZXN0RG5zUHJvdmlkZXJSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEhgKB2'
        '1lc3NhZ2UYAiABKAlSB21lc3NhZ2U=');

@$core.Deprecated('Use testDnsConnectionRequestDescriptor instead')
const TestDnsConnectionRequest$json = {
  '1': 'TestDnsConnectionRequest',
  '2': [
    {
      '1': 'provider_type',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DnsProviderType',
      '8': {},
      '10': 'providerType'
    },
    {
      '1': 'credentials_json',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'credentialsJson'
    },
  ],
};

/// Descriptor for `TestDnsConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDnsConnectionRequestDescriptor = $convert.base64Decode(
    'ChhUZXN0RG5zQ29ubmVjdGlvblJlcXVlc3QSUgoNcHJvdmlkZXJfdHlwZRgBIAEoDjIhLm9wZW'
    '5veGlkZS5hcGkudjEuRG5zUHJvdmlkZXJUeXBlQgq6SAeCAQQQASAAUgxwcm92aWRlclR5cGUS'
    'NgoQY3JlZGVudGlhbHNfanNvbhgCIAEoCUILukgIcgYQARiAgEBSD2NyZWRlbnRpYWxzSnNvbg'
    '==');

@$core.Deprecated('Use testDnsConnectionResponseDescriptor instead')
const TestDnsConnectionResponse$json = {
  '1': 'TestDnsConnectionResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestDnsConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDnsConnectionResponseDescriptor =
    $convert.base64Decode(
        'ChlUZXN0RG5zQ29ubmVjdGlvblJlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSGA'
        'oHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');

@$core.Deprecated('Use listDnsZonesRequestDescriptor instead')
const ListDnsZonesRequest$json = {
  '1': 'ListDnsZonesRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `ListDnsZonesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsZonesRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0RG5zWm9uZXNSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAiAAUgpwcm'
    '92aWRlcklk');

@$core.Deprecated('Use listDnsZonesResponseDescriptor instead')
const ListDnsZonesResponse$json = {
  '1': 'ListDnsZonesResponse',
  '2': [
    {
      '1': 'zones',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.DnsZone',
      '10': 'zones'
    },
  ],
};

/// Descriptor for `ListDnsZonesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsZonesResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0RG5zWm9uZXNSZXNwb25zZRIvCgV6b25lcxgBIAMoCzIZLm9wZW5veGlkZS5hcGkudj'
    'EuRG5zWm9uZVIFem9uZXM=');

@$core.Deprecated('Use listDnsRecordsRequestDescriptor instead')
const ListDnsRecordsRequest$json = {
  '1': 'ListDnsRecordsRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'zone_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'zoneId'},
  ],
};

/// Descriptor for `ListDnsRecordsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsRecordsRequestDescriptor = $convert.base64Decode(
    'ChVMaXN0RG5zUmVjb3Jkc1JlcXVlc3QSKAoLcHJvdmlkZXJfaWQYASABKANCB7pIBCICIABSCn'
    'Byb3ZpZGVySWQSIAoHem9uZV9pZBgCIAEoCUIHukgEcgIQAVIGem9uZUlk');

@$core.Deprecated('Use listDnsRecordsResponseDescriptor instead')
const ListDnsRecordsResponse$json = {
  '1': 'ListDnsRecordsResponse',
  '2': [
    {
      '1': 'records',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.DnsRecord',
      '10': 'records'
    },
  ],
};

/// Descriptor for `ListDnsRecordsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDnsRecordsResponseDescriptor =
    $convert.base64Decode(
        'ChZMaXN0RG5zUmVjb3Jkc1Jlc3BvbnNlEjUKB3JlY29yZHMYASADKAsyGy5vcGVub3hpZGUuYX'
        'BpLnYxLkRuc1JlY29yZFIHcmVjb3Jkcw==');

@$core.Deprecated('Use createDnsRecordRequestDescriptor instead')
const CreateDnsRecordRequest$json = {
  '1': 'CreateDnsRecordRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'zone_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'zoneId'},
    {'1': 'record_type', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'recordType'},
    {'1': 'name', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'content'},
    {'1': 'ttl', '3': 6, '4': 1, '5': 13, '9': 0, '10': 'ttl', '17': true},
    {
      '1': 'proxied',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 1,
      '10': 'proxied',
      '17': true
    },
  ],
  '8': [
    {'1': '_ttl'},
    {'1': '_proxied'},
  ],
};

/// Descriptor for `CreateDnsRecordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDnsRecordRequestDescriptor = $convert.base64Decode(
    'ChZDcmVhdGVEbnNSZWNvcmRSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAiAAUg'
    'pwcm92aWRlcklkEiAKB3pvbmVfaWQYAiABKAlCB7pIBHICEAFSBnpvbmVJZBIoCgtyZWNvcmRf'
    'dHlwZRgDIAEoCUIHukgEcgIQAVIKcmVjb3JkVHlwZRIbCgRuYW1lGAQgASgJQge6SARyAhABUg'
    'RuYW1lEiEKB2NvbnRlbnQYBSABKAlCB7pIBHICEAFSB2NvbnRlbnQSFQoDdHRsGAYgASgNSABS'
    'A3R0bIgBARIdCgdwcm94aWVkGAcgASgISAFSB3Byb3hpZWSIAQFCBgoEX3R0bEIKCghfcHJveG'
    'llZA==');

@$core.Deprecated('Use createDnsRecordResponseDescriptor instead')
const CreateDnsRecordResponse$json = {
  '1': 'CreateDnsRecordResponse',
  '2': [
    {
      '1': 'record',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DnsRecord',
      '10': 'record'
    },
  ],
};

/// Descriptor for `CreateDnsRecordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDnsRecordResponseDescriptor =
    $convert.base64Decode(
        'ChdDcmVhdGVEbnNSZWNvcmRSZXNwb25zZRIzCgZyZWNvcmQYASABKAsyGy5vcGVub3hpZGUuYX'
        'BpLnYxLkRuc1JlY29yZFIGcmVjb3Jk');

@$core.Deprecated('Use deleteDnsRecordRequestDescriptor instead')
const DeleteDnsRecordRequest$json = {
  '1': 'DeleteDnsRecordRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'zone_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'zoneId'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'id', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'id', '17': true},
    {
      '1': 'record_type',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'recordType',
      '17': true
    },
    {
      '1': 'content',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'content',
      '17': true
    },
  ],
  '8': [
    {'1': '_id'},
    {'1': '_record_type'},
    {'1': '_content'},
  ],
};

/// Descriptor for `DeleteDnsRecordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteDnsRecordRequestDescriptor = $convert.base64Decode(
    'ChZEZWxldGVEbnNSZWNvcmRSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAiAAUg'
    'pwcm92aWRlcklkEiAKB3pvbmVfaWQYAiABKAlCB7pIBHICEAFSBnpvbmVJZBIbCgRuYW1lGAMg'
    'ASgJQge6SARyAhABUgRuYW1lEhMKAmlkGAQgASgJSABSAmlkiAEBEiQKC3JlY29yZF90eXBlGA'
    'UgASgJSAFSCnJlY29yZFR5cGWIAQESHQoHY29udGVudBgGIAEoCUgCUgdjb250ZW50iAEBQgUK'
    'A19pZEIOCgxfcmVjb3JkX3R5cGVCCgoIX2NvbnRlbnQ=');

@$core.Deprecated('Use updateDnsRecordRequestDescriptor instead')
const UpdateDnsRecordRequest$json = {
  '1': 'UpdateDnsRecordRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'zone_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'zoneId'},
    {'1': 'id', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'id', '17': true},
    {'1': 'old_name', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'oldName'},
    {
      '1': 'old_record_type',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'oldRecordType',
      '17': true
    },
    {
      '1': 'old_content',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'oldContent',
      '17': true
    },
    {'1': 'record_type', '3': 7, '4': 1, '5': 9, '8': {}, '10': 'recordType'},
    {'1': 'name', '3': 8, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'content', '3': 9, '4': 1, '5': 9, '8': {}, '10': 'content'},
    {'1': 'ttl', '3': 10, '4': 1, '5': 13, '9': 3, '10': 'ttl', '17': true},
    {
      '1': 'proxied',
      '3': 11,
      '4': 1,
      '5': 8,
      '9': 4,
      '10': 'proxied',
      '17': true
    },
  ],
  '8': [
    {'1': '_id'},
    {'1': '_old_record_type'},
    {'1': '_old_content'},
    {'1': '_ttl'},
    {'1': '_proxied'},
  ],
};

/// Descriptor for `UpdateDnsRecordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDnsRecordRequestDescriptor = $convert.base64Decode(
    'ChZVcGRhdGVEbnNSZWNvcmRSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAiAAUg'
    'pwcm92aWRlcklkEiAKB3pvbmVfaWQYAiABKAlCB7pIBHICEAFSBnpvbmVJZBITCgJpZBgDIAEo'
    'CUgAUgJpZIgBARIiCghvbGRfbmFtZRgEIAEoCUIHukgEcgIQAVIHb2xkTmFtZRIrCg9vbGRfcm'
    'Vjb3JkX3R5cGUYBSABKAlIAVINb2xkUmVjb3JkVHlwZYgBARIkCgtvbGRfY29udGVudBgGIAEo'
    'CUgCUgpvbGRDb250ZW50iAEBEigKC3JlY29yZF90eXBlGAcgASgJQge6SARyAhABUgpyZWNvcm'
    'RUeXBlEhsKBG5hbWUYCCABKAlCB7pIBHICEAFSBG5hbWUSIQoHY29udGVudBgJIAEoCUIHukgE'
    'cgIQAVIHY29udGVudBIVCgN0dGwYCiABKA1IA1IDdHRsiAEBEh0KB3Byb3hpZWQYCyABKAhIBF'
    'IHcHJveGllZIgBAUIFCgNfaWRCEgoQX29sZF9yZWNvcmRfdHlwZUIOCgxfb2xkX2NvbnRlbnRC'
    'BgoEX3R0bEIKCghfcHJveGllZA==');

@$core.Deprecated('Use updateDnsRecordResponseDescriptor instead')
const UpdateDnsRecordResponse$json = {
  '1': 'UpdateDnsRecordResponse',
  '2': [
    {
      '1': 'record',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DnsRecord',
      '10': 'record'
    },
  ],
};

/// Descriptor for `UpdateDnsRecordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDnsRecordResponseDescriptor =
    $convert.base64Decode(
        'ChdVcGRhdGVEbnNSZWNvcmRSZXNwb25zZRIzCgZyZWNvcmQYASABKAsyGy5vcGVub3hpZGUuYX'
        'BpLnYxLkRuc1JlY29yZFIGcmVjb3Jk');

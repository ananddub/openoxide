// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/domain.proto.

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

@$core.Deprecated('Use domainTypeDescriptor instead')
const DomainType$json = {
  '1': 'DomainType',
  '2': [
    {'1': 'DOMAIN_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'DOMAIN_TYPE_APPLICATION', '2': 1},
    {'1': 'DOMAIN_TYPE_COMPOSE', '2': 2},
  ],
};

/// Descriptor for `DomainType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List domainTypeDescriptor = $convert.base64Decode(
    'CgpEb21haW5UeXBlEhsKF0RPTUFJTl9UWVBFX1VOU1BFQ0lGSUVEEAASGwoXRE9NQUlOX1RZUE'
    'VfQVBQTElDQVRJT04QARIXChNET01BSU5fVFlQRV9DT01QT1NFEAI=');

@$core.Deprecated('Use domainCertificateTypeDescriptor instead')
const DomainCertificateType$json = {
  '1': 'DomainCertificateType',
  '2': [
    {'1': 'DOMAIN_CERTIFICATE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'DOMAIN_CERTIFICATE_TYPE_LETSENCRYPT', '2': 1},
    {'1': 'DOMAIN_CERTIFICATE_TYPE_CUSTOM', '2': 2},
    {'1': 'DOMAIN_CERTIFICATE_TYPE_NONE', '2': 3},
  ],
};

/// Descriptor for `DomainCertificateType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List domainCertificateTypeDescriptor = $convert.base64Decode(
    'ChVEb21haW5DZXJ0aWZpY2F0ZVR5cGUSJwojRE9NQUlOX0NFUlRJRklDQVRFX1RZUEVfVU5TUE'
    'VDSUZJRUQQABInCiNET01BSU5fQ0VSVElGSUNBVEVfVFlQRV9MRVRTRU5DUllQVBABEiIKHkRP'
    'TUFJTl9DRVJUSUZJQ0FURV9UWVBFX0NVU1RPTRACEiAKHERPTUFJTl9DRVJUSUZJQ0FURV9UWV'
    'BFX05PTkUQAw==');

@$core.Deprecated('Use domainDescriptor instead')
const Domain$json = {
  '1': 'Domain',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'host', '3': 2, '4': 1, '5': 9, '10': 'host'},
    {'1': 'https', '3': 3, '4': 1, '5': 8, '10': 'https'},
    {'1': 'port', '3': 4, '4': 1, '5': 3, '9': 0, '10': 'port', '17': true},
    {'1': 'path', '3': 5, '4': 1, '5': 9, '9': 1, '10': 'path', '17': true},
    {
      '1': 'internal_path',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'internalPath',
      '17': true
    },
    {
      '1': 'custom_entrypoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'customEntrypoint',
      '17': true
    },
    {
      '1': 'service_name',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'custom_cert_resolver',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'strip_path', '3': 10, '4': 1, '5': 8, '10': 'stripPath'},
    {'1': 'middlewares', '3': 11, '4': 1, '5': 9, '10': 'middlewares'},
    {
      '1': 'domain_type',
      '3': 12,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DomainType',
      '10': 'domainType'
    },
    {
      '1': 'certificate_type',
      '3': 13,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DomainCertificateType',
      '10': 'certificateType'
    },
    {
      '1': 'service_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 6,
      '10': 'serviceId',
      '17': true
    },
    {'1': 'created_at', '3': 15, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 16, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_port'},
    {'1': '_path'},
    {'1': '_internal_path'},
    {'1': '_custom_entrypoint'},
    {'1': '_service_name'},
    {'1': '_custom_cert_resolver'},
    {'1': '_service_id'},
  ],
};

/// Descriptor for `Domain`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List domainDescriptor = $convert.base64Decode(
    'CgZEb21haW4SDgoCaWQYASABKANSAmlkEhIKBGhvc3QYAiABKAlSBGhvc3QSFAoFaHR0cHMYAy'
    'ABKAhSBWh0dHBzEhcKBHBvcnQYBCABKANIAFIEcG9ydIgBARIXCgRwYXRoGAUgASgJSAFSBHBh'
    'dGiIAQESKAoNaW50ZXJuYWxfcGF0aBgGIAEoCUgCUgxpbnRlcm5hbFBhdGiIAQESMAoRY3VzdG'
    '9tX2VudHJ5cG9pbnQYByABKAlIA1IQY3VzdG9tRW50cnlwb2ludIgBARImCgxzZXJ2aWNlX25h'
    'bWUYCCABKAlIBFILc2VydmljZU5hbWWIAQESNQoUY3VzdG9tX2NlcnRfcmVzb2x2ZXIYCSABKA'
    'lIBVISY3VzdG9tQ2VydFJlc29sdmVyiAEBEh0KCnN0cmlwX3BhdGgYCiABKAhSCXN0cmlwUGF0'
    'aBIgCgttaWRkbGV3YXJlcxgLIAEoCVILbWlkZGxld2FyZXMSPQoLZG9tYWluX3R5cGUYDCABKA'
    '4yHC5vcGVub3hpZGUuYXBpLnYxLkRvbWFpblR5cGVSCmRvbWFpblR5cGUSUgoQY2VydGlmaWNh'
    'dGVfdHlwZRgNIAEoDjInLm9wZW5veGlkZS5hcGkudjEuRG9tYWluQ2VydGlmaWNhdGVUeXBlUg'
    '9jZXJ0aWZpY2F0ZVR5cGUSIgoKc2VydmljZV9pZBgOIAEoA0gGUglzZXJ2aWNlSWSIAQESHQoK'
    'Y3JlYXRlZF9hdBgPIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYECABKANSCXVwZGF0ZW'
    'RBdEIHCgVfcG9ydEIHCgVfcGF0aEIQCg5faW50ZXJuYWxfcGF0aEIUChJfY3VzdG9tX2VudHJ5'
    'cG9pbnRCDwoNX3NlcnZpY2VfbmFtZUIXChVfY3VzdG9tX2NlcnRfcmVzb2x2ZXJCDQoLX3Nlcn'
    'ZpY2VfaWQ=');

@$core.Deprecated('Use listDomainsRequestDescriptor instead')
const ListDomainsRequest$json = {
  '1': 'ListDomainsRequest',
};

/// Descriptor for `ListDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDomainsRequestDescriptor =
    $convert.base64Decode('ChJMaXN0RG9tYWluc1JlcXVlc3Q=');

@$core.Deprecated('Use listDomainsResponseDescriptor instead')
const ListDomainsResponse$json = {
  '1': 'ListDomainsResponse',
  '2': [
    {
      '1': 'domains',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domains'
    },
  ],
};

/// Descriptor for `ListDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDomainsResponseDescriptor = $convert.base64Decode(
    'ChNMaXN0RG9tYWluc1Jlc3BvbnNlEjIKB2RvbWFpbnMYASADKAsyGC5vcGVub3hpZGUuYXBpLn'
    'YxLkRvbWFpblIHZG9tYWlucw==');

@$core.Deprecated('Use getDomainRequestDescriptor instead')
const GetDomainRequest$json = {
  '1': 'GetDomainRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetDomainRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDomainRequestDescriptor = $convert.base64Decode(
    'ChBHZXREb21haW5SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use getDomainResponseDescriptor instead')
const GetDomainResponse$json = {
  '1': 'GetDomainResponse',
  '2': [
    {
      '1': 'domain',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domain'
    },
  ],
};

/// Descriptor for `GetDomainResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDomainResponseDescriptor = $convert.base64Decode(
    'ChFHZXREb21haW5SZXNwb25zZRIwCgZkb21haW4YASABKAsyGC5vcGVub3hpZGUuYXBpLnYxLk'
    'RvbWFpblIGZG9tYWlu');

@$core.Deprecated('Use listApplicationDomainsRequestDescriptor instead')
const ListApplicationDomainsRequest$json = {
  '1': 'ListApplicationDomainsRequest',
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

/// Descriptor for `ListApplicationDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listApplicationDomainsRequestDescriptor =
    $convert.base64Decode(
        'Ch1MaXN0QXBwbGljYXRpb25Eb21haW5zUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
        'IHukgEIgIgAFINYXBwbGljYXRpb25JZA==');

@$core.Deprecated('Use listApplicationDomainsResponseDescriptor instead')
const ListApplicationDomainsResponse$json = {
  '1': 'ListApplicationDomainsResponse',
  '2': [
    {
      '1': 'domains',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domains'
    },
  ],
};

/// Descriptor for `ListApplicationDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listApplicationDomainsResponseDescriptor =
    $convert.base64Decode(
        'Ch5MaXN0QXBwbGljYXRpb25Eb21haW5zUmVzcG9uc2USMgoHZG9tYWlucxgBIAMoCzIYLm9wZW'
        '5veGlkZS5hcGkudjEuRG9tYWluUgdkb21haW5z');

@$core.Deprecated('Use listComposeDomainsRequestDescriptor instead')
const ListComposeDomainsRequest$json = {
  '1': 'ListComposeDomainsRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
  ],
};

/// Descriptor for `ListComposeDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listComposeDomainsRequestDescriptor =
    $convert.base64Decode(
        'ChlMaXN0Q29tcG9zZURvbWFpbnNSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
        'BSCWNvbXBvc2VJZA==');

@$core.Deprecated('Use listComposeDomainsResponseDescriptor instead')
const ListComposeDomainsResponse$json = {
  '1': 'ListComposeDomainsResponse',
  '2': [
    {
      '1': 'domains',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domains'
    },
  ],
};

/// Descriptor for `ListComposeDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listComposeDomainsResponseDescriptor =
    $convert.base64Decode(
        'ChpMaXN0Q29tcG9zZURvbWFpbnNSZXNwb25zZRIyCgdkb21haW5zGAEgAygLMhgub3Blbm94aW'
        'RlLmFwaS52MS5Eb21haW5SB2RvbWFpbnM=');

@$core.Deprecated('Use createDomainRequestDescriptor instead')
const CreateDomainRequest$json = {
  '1': 'CreateDomainRequest',
  '2': [
    {'1': 'host', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'host'},
    {'1': 'https', '3': 2, '4': 1, '5': 8, '10': 'https'},
    {'1': 'port', '3': 3, '4': 1, '5': 3, '9': 0, '10': 'port', '17': true},
    {'1': 'path', '3': 4, '4': 1, '5': 9, '9': 1, '10': 'path', '17': true},
    {
      '1': 'internal_path',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'internalPath',
      '17': true
    },
    {
      '1': 'custom_entrypoint',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'customEntrypoint',
      '17': true
    },
    {
      '1': 'service_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'custom_cert_resolver',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'strip_path', '3': 9, '4': 1, '5': 8, '10': 'stripPath'},
    {
      '1': 'middlewares',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'middlewares',
      '17': true
    },
    {
      '1': 'domain_type',
      '3': 11,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DomainType',
      '8': {},
      '9': 7,
      '10': 'domainType',
      '17': true
    },
    {
      '1': 'certificate_type',
      '3': 12,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DomainCertificateType',
      '8': {},
      '9': 8,
      '10': 'certificateType',
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
  ],
  '8': [
    {'1': '_port'},
    {'1': '_path'},
    {'1': '_internal_path'},
    {'1': '_custom_entrypoint'},
    {'1': '_service_name'},
    {'1': '_custom_cert_resolver'},
    {'1': '_middlewares'},
    {'1': '_domain_type'},
    {'1': '_certificate_type'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
  ],
};

/// Descriptor for `CreateDomainRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDomainRequestDescriptor = $convert.base64Decode(
    'ChNDcmVhdGVEb21haW5SZXF1ZXN0Eh4KBGhvc3QYASABKAlCCrpIB3IFEAEY/wFSBGhvc3QSFA'
    'oFaHR0cHMYAiABKAhSBWh0dHBzEhcKBHBvcnQYAyABKANIAFIEcG9ydIgBARIXCgRwYXRoGAQg'
    'ASgJSAFSBHBhdGiIAQESKAoNaW50ZXJuYWxfcGF0aBgFIAEoCUgCUgxpbnRlcm5hbFBhdGiIAQ'
    'ESMAoRY3VzdG9tX2VudHJ5cG9pbnQYBiABKAlIA1IQY3VzdG9tRW50cnlwb2ludIgBARImCgxz'
    'ZXJ2aWNlX25hbWUYByABKAlIBFILc2VydmljZU5hbWWIAQESNQoUY3VzdG9tX2NlcnRfcmVzb2'
    'x2ZXIYCCABKAlIBVISY3VzdG9tQ2VydFJlc29sdmVyiAEBEh0KCnN0cmlwX3BhdGgYCSABKAhS'
    'CXN0cmlwUGF0aBIlCgttaWRkbGV3YXJlcxgKIAEoCUgGUgttaWRkbGV3YXJlc4gBARJOCgtkb2'
    '1haW5fdHlwZRgLIAEoDjIcLm9wZW5veGlkZS5hcGkudjEuRG9tYWluVHlwZUIKukgHggEEEAEg'
    'AEgHUgpkb21haW5UeXBliAEBEmMKEGNlcnRpZmljYXRlX3R5cGUYDCABKA4yJy5vcGVub3hpZG'
    'UuYXBpLnYxLkRvbWFpbkNlcnRpZmljYXRlVHlwZUIKukgHggEEEAEgAEgIUg9jZXJ0aWZpY2F0'
    'ZVR5cGWIAQESMwoOYXBwbGljYXRpb25faWQYDSABKANCB7pIBCICIABICVINYXBwbGljYXRpb2'
    '5JZIgBARIrCgpjb21wb3NlX2lkGA4gASgDQge6SAQiAiAASApSCWNvbXBvc2VJZIgBAUIHCgVf'
    'cG9ydEIHCgVfcGF0aEIQCg5faW50ZXJuYWxfcGF0aEIUChJfY3VzdG9tX2VudHJ5cG9pbnRCDw'
    'oNX3NlcnZpY2VfbmFtZUIXChVfY3VzdG9tX2NlcnRfcmVzb2x2ZXJCDgoMX21pZGRsZXdhcmVz'
    'Qg4KDF9kb21haW5fdHlwZUITChFfY2VydGlmaWNhdGVfdHlwZUIRCg9fYXBwbGljYXRpb25faW'
    'RCDQoLX2NvbXBvc2VfaWQ=');

@$core.Deprecated('Use createDomainResponseDescriptor instead')
const CreateDomainResponse$json = {
  '1': 'CreateDomainResponse',
  '2': [
    {
      '1': 'domain',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domain'
    },
  ],
};

/// Descriptor for `CreateDomainResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDomainResponseDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVEb21haW5SZXNwb25zZRIwCgZkb21haW4YASABKAsyGC5vcGVub3hpZGUuYXBpLn'
    'YxLkRvbWFpblIGZG9tYWlu');

@$core.Deprecated('Use updateDomainRequestDescriptor instead')
const UpdateDomainRequest$json = {
  '1': 'UpdateDomainRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'host',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'host',
      '17': true
    },
    {'1': 'https', '3': 3, '4': 1, '5': 8, '9': 1, '10': 'https', '17': true},
    {'1': 'port', '3': 4, '4': 1, '5': 3, '9': 2, '10': 'port', '17': true},
    {'1': 'path', '3': 5, '4': 1, '5': 9, '9': 3, '10': 'path', '17': true},
    {
      '1': 'internal_path',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'internalPath',
      '17': true
    },
    {
      '1': 'custom_entrypoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'customEntrypoint',
      '17': true
    },
    {
      '1': 'service_name',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'serviceName',
      '17': true
    },
    {
      '1': 'custom_cert_resolver',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'customCertResolver',
      '17': true
    },
    {
      '1': 'strip_path',
      '3': 10,
      '4': 1,
      '5': 8,
      '9': 8,
      '10': 'stripPath',
      '17': true
    },
    {
      '1': 'middlewares',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'middlewares',
      '17': true
    },
    {
      '1': 'certificate_type',
      '3': 12,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DomainCertificateType',
      '8': {},
      '9': 10,
      '10': 'certificateType',
      '17': true
    },
  ],
  '8': [
    {'1': '_host'},
    {'1': '_https'},
    {'1': '_port'},
    {'1': '_path'},
    {'1': '_internal_path'},
    {'1': '_custom_entrypoint'},
    {'1': '_service_name'},
    {'1': '_custom_cert_resolver'},
    {'1': '_strip_path'},
    {'1': '_middlewares'},
    {'1': '_certificate_type'},
  ],
};

/// Descriptor for `UpdateDomainRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDomainRequestDescriptor = $convert.base64Decode(
    'ChNVcGRhdGVEb21haW5SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIhCgRob3N0GA'
    'IgASgJQgi6SAVyAxj/AUgAUgRob3N0iAEBEhkKBWh0dHBzGAMgASgISAFSBWh0dHBziAEBEhcK'
    'BHBvcnQYBCABKANIAlIEcG9ydIgBARIXCgRwYXRoGAUgASgJSANSBHBhdGiIAQESKAoNaW50ZX'
    'JuYWxfcGF0aBgGIAEoCUgEUgxpbnRlcm5hbFBhdGiIAQESMAoRY3VzdG9tX2VudHJ5cG9pbnQY'
    'ByABKAlIBVIQY3VzdG9tRW50cnlwb2ludIgBARImCgxzZXJ2aWNlX25hbWUYCCABKAlIBlILc2'
    'VydmljZU5hbWWIAQESNQoUY3VzdG9tX2NlcnRfcmVzb2x2ZXIYCSABKAlIB1ISY3VzdG9tQ2Vy'
    'dFJlc29sdmVyiAEBEiIKCnN0cmlwX3BhdGgYCiABKAhICFIJc3RyaXBQYXRoiAEBEiUKC21pZG'
    'RsZXdhcmVzGAsgASgJSAlSC21pZGRsZXdhcmVziAEBEmMKEGNlcnRpZmljYXRlX3R5cGUYDCAB'
    'KA4yJy5vcGVub3hpZGUuYXBpLnYxLkRvbWFpbkNlcnRpZmljYXRlVHlwZUIKukgHggEEEAEgAE'
    'gKUg9jZXJ0aWZpY2F0ZVR5cGWIAQFCBwoFX2hvc3RCCAoGX2h0dHBzQgcKBV9wb3J0QgcKBV9w'
    'YXRoQhAKDl9pbnRlcm5hbF9wYXRoQhQKEl9jdXN0b21fZW50cnlwb2ludEIPCg1fc2VydmljZV'
    '9uYW1lQhcKFV9jdXN0b21fY2VydF9yZXNvbHZlckINCgtfc3RyaXBfcGF0aEIOCgxfbWlkZGxl'
    'd2FyZXNCEwoRX2NlcnRpZmljYXRlX3R5cGU=');

@$core.Deprecated('Use updateDomainResponseDescriptor instead')
const UpdateDomainResponse$json = {
  '1': 'UpdateDomainResponse',
  '2': [
    {
      '1': 'domain',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Domain',
      '10': 'domain'
    },
  ],
};

/// Descriptor for `UpdateDomainResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDomainResponseDescriptor = $convert.base64Decode(
    'ChRVcGRhdGVEb21haW5SZXNwb25zZRIwCgZkb21haW4YASABKAsyGC5vcGVub3hpZGUuYXBpLn'
    'YxLkRvbWFpblIGZG9tYWlu');

@$core.Deprecated('Use deleteDomainRequestDescriptor instead')
const DeleteDomainRequest$json = {
  '1': 'DeleteDomainRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteDomainRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteDomainRequestDescriptor =
    $convert.base64Decode(
        'ChNEZWxldGVEb21haW5SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use validateDomainDnsRequestDescriptor instead')
const ValidateDomainDnsRequest$json = {
  '1': 'ValidateDomainDnsRequest',
  '2': [
    {'1': 'host', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'host'},
    {
      '1': 'server_id',
      '3': 2,
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

/// Descriptor for `ValidateDomainDnsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List validateDomainDnsRequestDescriptor = $convert.base64Decode(
    'ChhWYWxpZGF0ZURvbWFpbkRuc1JlcXVlc3QSHgoEaG9zdBgBIAEoCUIKukgHcgUQARj/AVIEaG'
    '9zdBIpCglzZXJ2ZXJfaWQYAiABKANCB7pIBCICIABIAFIIc2VydmVySWSIAQFCDAoKX3NlcnZl'
    'cl9pZA==');

@$core.Deprecated('Use validateDomainDnsResponseDescriptor instead')
const ValidateDomainDnsResponse$json = {
  '1': 'ValidateDomainDnsResponse',
  '2': [
    {'1': 'is_valid', '3': 1, '4': 1, '5': 8, '10': 'isValid'},
    {'1': 'resolved_ips', '3': 2, '4': 3, '5': 9, '10': 'resolvedIps'},
    {'1': 'expected_ips', '3': 3, '4': 3, '5': 9, '10': 'expectedIps'},
    {'1': 'error', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_error'},
  ],
};

/// Descriptor for `ValidateDomainDnsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List validateDomainDnsResponseDescriptor = $convert.base64Decode(
    'ChlWYWxpZGF0ZURvbWFpbkRuc1Jlc3BvbnNlEhkKCGlzX3ZhbGlkGAEgASgIUgdpc1ZhbGlkEi'
    'EKDHJlc29sdmVkX2lwcxgCIAMoCVILcmVzb2x2ZWRJcHMSIQoMZXhwZWN0ZWRfaXBzGAMgAygJ'
    'UgtleHBlY3RlZElwcxIZCgVlcnJvchgEIAEoCUgAUgVlcnJvcogBAUIICgZfZXJyb3I=');

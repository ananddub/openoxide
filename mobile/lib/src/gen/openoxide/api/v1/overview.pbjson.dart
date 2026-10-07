// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/overview.proto.

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

@$core.Deprecated('Use overviewServiceTypeDescriptor instead')
const OverviewServiceType$json = {
  '1': 'OverviewServiceType',
  '2': [
    {'1': 'OVERVIEW_SERVICE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'OVERVIEW_SERVICE_TYPE_APP', '2': 1},
    {'1': 'OVERVIEW_SERVICE_TYPE_COMPOSE', '2': 2},
    {'1': 'OVERVIEW_SERVICE_TYPE_DATABASE', '2': 3},
  ],
};

/// Descriptor for `OverviewServiceType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List overviewServiceTypeDescriptor = $convert.base64Decode(
    'ChNPdmVydmlld1NlcnZpY2VUeXBlEiUKIU9WRVJWSUVXX1NFUlZJQ0VfVFlQRV9VTlNQRUNJRk'
    'lFRBAAEh0KGU9WRVJWSUVXX1NFUlZJQ0VfVFlQRV9BUFAQARIhCh1PVkVSVklFV19TRVJWSUNF'
    'X1RZUEVfQ09NUE9TRRACEiIKHk9WRVJWSUVXX1NFUlZJQ0VfVFlQRV9EQVRBQkFTRRAD');

@$core.Deprecated('Use overviewBackupTypeDescriptor instead')
const OverviewBackupType$json = {
  '1': 'OverviewBackupType',
  '2': [
    {'1': 'OVERVIEW_BACKUP_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'OVERVIEW_BACKUP_TYPE_DATABASE', '2': 1},
    {'1': 'OVERVIEW_BACKUP_TYPE_VOLUME', '2': 2},
  ],
};

/// Descriptor for `OverviewBackupType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List overviewBackupTypeDescriptor = $convert.base64Decode(
    'ChJPdmVydmlld0JhY2t1cFR5cGUSJAogT1ZFUlZJRVdfQkFDS1VQX1RZUEVfVU5TUEVDSUZJRU'
    'QQABIhCh1PVkVSVklFV19CQUNLVVBfVFlQRV9EQVRBQkFTRRABEh8KG09WRVJWSUVXX0JBQ0tV'
    'UF9UWVBFX1ZPTFVNRRAC');

@$core.Deprecated('Use overviewBackupStatusDescriptor instead')
const OverviewBackupStatus$json = {
  '1': 'OverviewBackupStatus',
  '2': [
    {'1': 'OVERVIEW_BACKUP_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'OVERVIEW_BACKUP_STATUS_ENABLED', '2': 1},
    {'1': 'OVERVIEW_BACKUP_STATUS_DISABLED', '2': 2},
  ],
};

/// Descriptor for `OverviewBackupStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List overviewBackupStatusDescriptor = $convert.base64Decode(
    'ChRPdmVydmlld0JhY2t1cFN0YXR1cxImCiJPVkVSVklFV19CQUNLVVBfU1RBVFVTX1VOU1BFQ0'
    'lGSUVEEAASIgoeT1ZFUlZJRVdfQkFDS1VQX1NUQVRVU19FTkFCTEVEEAESIwofT1ZFUlZJRVdf'
    'QkFDS1VQX1NUQVRVU19ESVNBQkxFRBAC');

@$core.Deprecated('Use overviewServiceItemDescriptor instead')
const OverviewServiceItem$json = {
  '1': 'OverviewServiceItem',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'service_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.OverviewServiceType',
      '10': 'serviceType'
    },
    {'1': 'status', '3': 4, '4': 1, '5': 9, '10': 'status'},
    {'1': 'created_at', '3': 5, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'project_id', '3': 6, '4': 1, '5': 3, '10': 'projectId'},
    {'1': 'project_name', '3': 7, '4': 1, '5': 9, '10': 'projectName'},
    {'1': 'environment_id', '3': 8, '4': 1, '5': 3, '10': 'environmentId'},
    {'1': 'environment_name', '3': 9, '4': 1, '5': 9, '10': 'environmentName'},
    {
      '1': 'db_kind',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'dbKind',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'server_name',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'serverName',
      '17': true
    },
  ],
  '8': [
    {'1': '_db_kind'},
    {'1': '_server_id'},
    {'1': '_server_name'},
  ],
};

/// Descriptor for `OverviewServiceItem`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewServiceItemDescriptor = $convert.base64Decode(
    'ChNPdmVydmlld1NlcnZpY2VJdGVtEg4KAmlkGAEgASgDUgJpZBISCgRuYW1lGAIgASgJUgRuYW'
    '1lEkgKDHNlcnZpY2VfdHlwZRgDIAEoDjIlLm9wZW5veGlkZS5hcGkudjEuT3ZlcnZpZXdTZXJ2'
    'aWNlVHlwZVILc2VydmljZVR5cGUSFgoGc3RhdHVzGAQgASgJUgZzdGF0dXMSHQoKY3JlYXRlZF'
    '9hdBgFIAEoA1IJY3JlYXRlZEF0Eh0KCnByb2plY3RfaWQYBiABKANSCXByb2plY3RJZBIhCgxw'
    'cm9qZWN0X25hbWUYByABKAlSC3Byb2plY3ROYW1lEiUKDmVudmlyb25tZW50X2lkGAggASgDUg'
    '1lbnZpcm9ubWVudElkEikKEGVudmlyb25tZW50X25hbWUYCSABKAlSD2Vudmlyb25tZW50TmFt'
    'ZRIcCgdkYl9raW5kGAogASgJSABSBmRiS2luZIgBARIgCglzZXJ2ZXJfaWQYCyABKANIAVIIc2'
    'VydmVySWSIAQESJAoLc2VydmVyX25hbWUYDCABKAlIAlIKc2VydmVyTmFtZYgBAUIKCghfZGJf'
    'a2luZEIMCgpfc2VydmVyX2lkQg4KDF9zZXJ2ZXJfbmFtZQ==');

@$core.Deprecated('Use overviewDomainItemDescriptor instead')
const OverviewDomainItem$json = {
  '1': 'OverviewDomainItem',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'host', '3': 2, '4': 1, '5': 9, '10': 'host'},
    {'1': 'path', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'path', '17': true},
    {'1': 'port', '3': 4, '4': 1, '5': 5, '9': 1, '10': 'port', '17': true},
    {'1': 'https', '3': 5, '4': 1, '5': 8, '10': 'https'},
    {
      '1': 'application_id',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'composeId',
      '17': true
    },
    {'1': 'service_name', '3': 8, '4': 1, '5': 9, '10': 'serviceName'},
    {'1': 'project_name', '3': 9, '4': 1, '5': 9, '10': 'projectName'},
    {
      '1': 'internal_path',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'internalPath',
      '17': true
    },
    {
      '1': 'custom_entrypoint',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'customEntrypoint',
      '17': true
    },
    {
      '1': 'custom_cert_resolver',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'strip_path', '3': 13, '4': 1, '5': 8, '10': 'stripPath'},
    {'1': 'middlewares', '3': 14, '4': 1, '5': 9, '10': 'middlewares'},
    {'1': 'domain_type', '3': 15, '4': 1, '5': 9, '10': 'domainType'},
    {'1': 'certificate_type', '3': 16, '4': 1, '5': 9, '10': 'certificateType'},
    {
      '1': 'domain_service_name',
      '3': 17,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'domainServiceName',
      '17': true
    },
  ],
  '8': [
    {'1': '_path'},
    {'1': '_port'},
    {'1': '_application_id'},
    {'1': '_compose_id'},
    {'1': '_internal_path'},
    {'1': '_custom_entrypoint'},
    {'1': '_custom_cert_resolver'},
    {'1': '_domain_service_name'},
  ],
};

/// Descriptor for `OverviewDomainItem`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewDomainItemDescriptor = $convert.base64Decode(
    'ChJPdmVydmlld0RvbWFpbkl0ZW0SDgoCaWQYASABKANSAmlkEhIKBGhvc3QYAiABKAlSBGhvc3'
    'QSFwoEcGF0aBgDIAEoCUgAUgRwYXRoiAEBEhcKBHBvcnQYBCABKAVIAVIEcG9ydIgBARIUCgVo'
    'dHRwcxgFIAEoCFIFaHR0cHMSKgoOYXBwbGljYXRpb25faWQYBiABKANIAlINYXBwbGljYXRpb2'
    '5JZIgBARIiCgpjb21wb3NlX2lkGAcgASgDSANSCWNvbXBvc2VJZIgBARIhCgxzZXJ2aWNlX25h'
    'bWUYCCABKAlSC3NlcnZpY2VOYW1lEiEKDHByb2plY3RfbmFtZRgJIAEoCVILcHJvamVjdE5hbW'
    'USKAoNaW50ZXJuYWxfcGF0aBgKIAEoCUgEUgxpbnRlcm5hbFBhdGiIAQESMAoRY3VzdG9tX2Vu'
    'dHJ5cG9pbnQYCyABKAlIBVIQY3VzdG9tRW50cnlwb2ludIgBARI1ChRjdXN0b21fY2VydF9yZX'
    'NvbHZlchgMIAEoCUgGUhJjdXN0b21DZXJ0UmVzb2x2ZXKIAQESHQoKc3RyaXBfcGF0aBgNIAEo'
    'CFIJc3RyaXBQYXRoEiAKC21pZGRsZXdhcmVzGA4gASgJUgttaWRkbGV3YXJlcxIfCgtkb21haW'
    '5fdHlwZRgPIAEoCVIKZG9tYWluVHlwZRIpChBjZXJ0aWZpY2F0ZV90eXBlGBAgASgJUg9jZXJ0'
    'aWZpY2F0ZVR5cGUSMwoTZG9tYWluX3NlcnZpY2VfbmFtZRgRIAEoCUgHUhFkb21haW5TZXJ2aW'
    'NlTmFtZYgBAUIHCgVfcGF0aEIHCgVfcG9ydEIRCg9fYXBwbGljYXRpb25faWRCDQoLX2NvbXBv'
    'c2VfaWRCEAoOX2ludGVybmFsX3BhdGhCFAoSX2N1c3RvbV9lbnRyeXBvaW50QhcKFV9jdXN0b2'
    '1fY2VydF9yZXNvbHZlckIWChRfZG9tYWluX3NlcnZpY2VfbmFtZQ==');

@$core.Deprecated('Use overviewBackupItemDescriptor instead')
const OverviewBackupItem$json = {
  '1': 'OverviewBackupItem',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'backup_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.OverviewBackupType',
      '10': 'backupType'
    },
    {
      '1': 'status',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.OverviewBackupStatus',
      '10': 'status'
    },
    {'1': 'destination', '3': 5, '4': 1, '5': 9, '10': 'destination'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
    {
      '1': 'service_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'serviceId',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'compose_id',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'postgres_id',
      '3': 10,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'postgresId',
      '17': true
    },
    {
      '1': 'mysql_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'mysqlId',
      '17': true
    },
    {
      '1': 'mariadb_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'mariadbId',
      '17': true
    },
    {
      '1': 'mongo_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '9': 6,
      '10': 'mongoId',
      '17': true
    },
    {
      '1': 'redis_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 7,
      '10': 'redisId',
      '17': true
    },
    {
      '1': 'libsql_id',
      '3': 15,
      '4': 1,
      '5': 3,
      '9': 8,
      '10': 'libsqlId',
      '17': true
    },
  ],
  '8': [
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

/// Descriptor for `OverviewBackupItem`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewBackupItemDescriptor = $convert.base64Decode(
    'ChJPdmVydmlld0JhY2t1cEl0ZW0SDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbW'
    'USRQoLYmFja3VwX3R5cGUYAyABKA4yJC5vcGVub3hpZGUuYXBpLnYxLk92ZXJ2aWV3QmFja3Vw'
    'VHlwZVIKYmFja3VwVHlwZRI+CgZzdGF0dXMYBCABKA4yJi5vcGVub3hpZGUuYXBpLnYxLk92ZX'
    'J2aWV3QmFja3VwU3RhdHVzUgZzdGF0dXMSIAoLZGVzdGluYXRpb24YBSABKAlSC2Rlc3RpbmF0'
    'aW9uEh0KCmNyZWF0ZWRfYXQYBiABKANSCWNyZWF0ZWRBdBIiCgpzZXJ2aWNlX2lkGAcgASgDSA'
    'BSCXNlcnZpY2VJZIgBARIqCg5hcHBsaWNhdGlvbl9pZBgIIAEoA0gBUg1hcHBsaWNhdGlvbklk'
    'iAEBEiIKCmNvbXBvc2VfaWQYCSABKANIAlIJY29tcG9zZUlkiAEBEiQKC3Bvc3RncmVzX2lkGA'
    'ogASgDSANSCnBvc3RncmVzSWSIAQESHgoIbXlzcWxfaWQYCyABKANIBFIHbXlzcWxJZIgBARIi'
    'CgptYXJpYWRiX2lkGAwgASgDSAVSCW1hcmlhZGJJZIgBARIeCghtb25nb19pZBgNIAEoA0gGUg'
    'dtb25nb0lkiAEBEh4KCHJlZGlzX2lkGA4gASgDSAdSB3JlZGlzSWSIAQESIAoJbGlic3FsX2lk'
    'GA8gASgDSAhSCGxpYnNxbElkiAEBQg0KC19zZXJ2aWNlX2lkQhEKD19hcHBsaWNhdGlvbl9pZE'
    'INCgtfY29tcG9zZV9pZEIOCgxfcG9zdGdyZXNfaWRCCwoJX215c3FsX2lkQg0KC19tYXJpYWRi'
    'X2lkQgsKCV9tb25nb19pZEILCglfcmVkaXNfaWRCDAoKX2xpYnNxbF9pZA==');

@$core.Deprecated('Use overviewListServicesRequestDescriptor instead')
const OverviewListServicesRequest$json = {
  '1': 'OverviewListServicesRequest',
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
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'environmentId',
      '17': true
    },
    {
      '1': 'limit',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'limit',
      '17': true
    },
    {
      '1': 'offset',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'offset',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_environment_id'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `OverviewListServicesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListServicesRequestDescriptor = $convert.base64Decode(
    'ChtPdmVydmlld0xpc3RTZXJ2aWNlc1JlcXVlc3QSNQoPb3JnYW5pemF0aW9uX2lkGAEgASgDQg'
    'e6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBEjMKDmVudmlyb25tZW50X2lkGAIgASgDQge6'
    'SAQiAiAASAFSDWVudmlyb25tZW50SWSIAQESJQoFbGltaXQYAyABKANCCrpIByIFGOgHKAFIAl'
    'IFbGltaXSIAQESJAoGb2Zmc2V0GAQgASgDQge6SAQiAigASANSBm9mZnNldIgBAUISChBfb3Jn'
    'YW5pemF0aW9uX2lkQhEKD19lbnZpcm9ubWVudF9pZEIICgZfbGltaXRCCQoHX29mZnNldA==');

@$core.Deprecated('Use overviewListServicesResponseDescriptor instead')
const OverviewListServicesResponse$json = {
  '1': 'OverviewListServicesResponse',
  '2': [
    {
      '1': 'services',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewServiceItem',
      '10': 'services'
    },
  ],
};

/// Descriptor for `OverviewListServicesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListServicesResponseDescriptor =
    $convert.base64Decode(
        'ChxPdmVydmlld0xpc3RTZXJ2aWNlc1Jlc3BvbnNlEkEKCHNlcnZpY2VzGAEgAygLMiUub3Blbm'
        '94aWRlLmFwaS52MS5PdmVydmlld1NlcnZpY2VJdGVtUghzZXJ2aWNlcw==');

@$core.Deprecated('Use overviewListDomainsRequestDescriptor instead')
const OverviewListDomainsRequest$json = {
  '1': 'OverviewListDomainsRequest',
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
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'environmentId',
      '17': true
    },
    {
      '1': 'limit',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'limit',
      '17': true
    },
    {
      '1': 'offset',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'offset',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_environment_id'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `OverviewListDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListDomainsRequestDescriptor = $convert.base64Decode(
    'ChpPdmVydmlld0xpc3REb21haW5zUmVxdWVzdBI1Cg9vcmdhbml6YXRpb25faWQYASABKANCB7'
    'pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESMwoOZW52aXJvbm1lbnRfaWQYAiABKANCB7pI'
    'BCICIABIAVINZW52aXJvbm1lbnRJZIgBARIlCgVsaW1pdBgDIAEoA0IKukgHIgUY6AcoAUgCUg'
    'VsaW1pdIgBARIkCgZvZmZzZXQYBCABKANCB7pIBCICKABIA1IGb2Zmc2V0iAEBQhIKEF9vcmdh'
    'bml6YXRpb25faWRCEQoPX2Vudmlyb25tZW50X2lkQggKBl9saW1pdEIJCgdfb2Zmc2V0');

@$core.Deprecated('Use overviewListDomainsResponseDescriptor instead')
const OverviewListDomainsResponse$json = {
  '1': 'OverviewListDomainsResponse',
  '2': [
    {
      '1': 'domains',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewDomainItem',
      '10': 'domains'
    },
  ],
};

/// Descriptor for `OverviewListDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListDomainsResponseDescriptor =
    $convert.base64Decode(
        'ChtPdmVydmlld0xpc3REb21haW5zUmVzcG9uc2USPgoHZG9tYWlucxgBIAMoCzIkLm9wZW5veG'
        'lkZS5hcGkudjEuT3ZlcnZpZXdEb21haW5JdGVtUgdkb21haW5z');

@$core.Deprecated('Use overviewListBackupsRequestDescriptor instead')
const OverviewListBackupsRequest$json = {
  '1': 'OverviewListBackupsRequest',
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
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'environmentId',
      '17': true
    },
    {
      '1': 'limit',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'limit',
      '17': true
    },
    {
      '1': 'offset',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'offset',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_environment_id'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `OverviewListBackupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListBackupsRequestDescriptor = $convert.base64Decode(
    'ChpPdmVydmlld0xpc3RCYWNrdXBzUmVxdWVzdBI1Cg9vcmdhbml6YXRpb25faWQYASABKANCB7'
    'pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESMwoOZW52aXJvbm1lbnRfaWQYAiABKANCB7pI'
    'BCICIABIAVINZW52aXJvbm1lbnRJZIgBARIlCgVsaW1pdBgDIAEoA0IKukgHIgUY6AcoAUgCUg'
    'VsaW1pdIgBARIkCgZvZmZzZXQYBCABKANCB7pIBCICKABIA1IGb2Zmc2V0iAEBQhIKEF9vcmdh'
    'bml6YXRpb25faWRCEQoPX2Vudmlyb25tZW50X2lkQggKBl9saW1pdEIJCgdfb2Zmc2V0');

@$core.Deprecated('Use overviewListBackupsResponseDescriptor instead')
const OverviewListBackupsResponse$json = {
  '1': 'OverviewListBackupsResponse',
  '2': [
    {
      '1': 'backups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewBackupItem',
      '10': 'backups'
    },
  ],
};

/// Descriptor for `OverviewListBackupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListBackupsResponseDescriptor =
    $convert.base64Decode(
        'ChtPdmVydmlld0xpc3RCYWNrdXBzUmVzcG9uc2USPgoHYmFja3VwcxgBIAMoCzIkLm9wZW5veG'
        'lkZS5hcGkudjEuT3ZlcnZpZXdCYWNrdXBJdGVtUgdiYWNrdXBz');

@$core
    .Deprecated('Use overviewListOrganizationServicesRequestDescriptor instead')
const OverviewListOrganizationServicesRequest$json = {
  '1': 'OverviewListOrganizationServicesRequest',
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

/// Descriptor for `OverviewListOrganizationServicesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationServicesRequestDescriptor =
    $convert.base64Decode(
        'CidPdmVydmlld0xpc3RPcmdhbml6YXRpb25TZXJ2aWNlc1JlcXVlc3QSNQoPb3JnYW5pemF0aW'
        '9uX2lkGAEgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBEiUKBWxpbWl0GAIgASgD'
        'Qgq6SAciBRjoBygBSAFSBWxpbWl0iAEBEiQKBm9mZnNldBgDIAEoA0IHukgEIgIoAEgCUgZvZm'
        'ZzZXSIAQFCEgoQX29yZ2FuaXphdGlvbl9pZEIICgZfbGltaXRCCQoHX29mZnNldA==');

@$core.Deprecated(
    'Use overviewListOrganizationServicesResponseDescriptor instead')
const OverviewListOrganizationServicesResponse$json = {
  '1': 'OverviewListOrganizationServicesResponse',
  '2': [
    {
      '1': 'services',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewServiceItem',
      '10': 'services'
    },
  ],
};

/// Descriptor for `OverviewListOrganizationServicesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationServicesResponseDescriptor =
    $convert.base64Decode(
        'CihPdmVydmlld0xpc3RPcmdhbml6YXRpb25TZXJ2aWNlc1Jlc3BvbnNlEkEKCHNlcnZpY2VzGA'
        'EgAygLMiUub3Blbm94aWRlLmFwaS52MS5PdmVydmlld1NlcnZpY2VJdGVtUghzZXJ2aWNlcw==');

@$core
    .Deprecated('Use overviewListOrganizationDomainsRequestDescriptor instead')
const OverviewListOrganizationDomainsRequest$json = {
  '1': 'OverviewListOrganizationDomainsRequest',
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

/// Descriptor for `OverviewListOrganizationDomainsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationDomainsRequestDescriptor =
    $convert.base64Decode(
        'CiZPdmVydmlld0xpc3RPcmdhbml6YXRpb25Eb21haW5zUmVxdWVzdBI1Cg9vcmdhbml6YXRpb2'
        '5faWQYASABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESJQoFbGltaXQYAiABKANC'
        'CrpIByIFGOgHKAFIAVIFbGltaXSIAQESJAoGb2Zmc2V0GAMgASgDQge6SAQiAigASAJSBm9mZn'
        'NldIgBAUISChBfb3JnYW5pemF0aW9uX2lkQggKBl9saW1pdEIJCgdfb2Zmc2V0');

@$core
    .Deprecated('Use overviewListOrganizationDomainsResponseDescriptor instead')
const OverviewListOrganizationDomainsResponse$json = {
  '1': 'OverviewListOrganizationDomainsResponse',
  '2': [
    {
      '1': 'domains',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewDomainItem',
      '10': 'domains'
    },
  ],
};

/// Descriptor for `OverviewListOrganizationDomainsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationDomainsResponseDescriptor =
    $convert.base64Decode(
        'CidPdmVydmlld0xpc3RPcmdhbml6YXRpb25Eb21haW5zUmVzcG9uc2USPgoHZG9tYWlucxgBIA'
        'MoCzIkLm9wZW5veGlkZS5hcGkudjEuT3ZlcnZpZXdEb21haW5JdGVtUgdkb21haW5z');

@$core
    .Deprecated('Use overviewListOrganizationBackupsRequestDescriptor instead')
const OverviewListOrganizationBackupsRequest$json = {
  '1': 'OverviewListOrganizationBackupsRequest',
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

/// Descriptor for `OverviewListOrganizationBackupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationBackupsRequestDescriptor =
    $convert.base64Decode(
        'CiZPdmVydmlld0xpc3RPcmdhbml6YXRpb25CYWNrdXBzUmVxdWVzdBI1Cg9vcmdhbml6YXRpb2'
        '5faWQYASABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESJQoFbGltaXQYAiABKANC'
        'CrpIByIFGOgHKAFIAVIFbGltaXSIAQESJAoGb2Zmc2V0GAMgASgDQge6SAQiAigASAJSBm9mZn'
        'NldIgBAUISChBfb3JnYW5pemF0aW9uX2lkQggKBl9saW1pdEIJCgdfb2Zmc2V0');

@$core
    .Deprecated('Use overviewListOrganizationBackupsResponseDescriptor instead')
const OverviewListOrganizationBackupsResponse$json = {
  '1': 'OverviewListOrganizationBackupsResponse',
  '2': [
    {
      '1': 'backups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.OverviewBackupItem',
      '10': 'backups'
    },
  ],
};

/// Descriptor for `OverviewListOrganizationBackupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List overviewListOrganizationBackupsResponseDescriptor =
    $convert.base64Decode(
        'CidPdmVydmlld0xpc3RPcmdhbml6YXRpb25CYWNrdXBzUmVzcG9uc2USPgoHYmFja3VwcxgBIA'
        'MoCzIkLm9wZW5veGlkZS5hcGkudjEuT3ZlcnZpZXdCYWNrdXBJdGVtUgdiYWNrdXBz');

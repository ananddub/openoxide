// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database.proto.

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

@$core.Deprecated('Use databaseKindDescriptor instead')
const DatabaseKind$json = {
  '1': 'DatabaseKind',
  '2': [
    {'1': 'DATABASE_KIND_UNSPECIFIED', '2': 0},
    {'1': 'DATABASE_KIND_POSTGRES', '2': 1},
    {'1': 'DATABASE_KIND_MYSQL', '2': 2},
    {'1': 'DATABASE_KIND_MARIADB', '2': 3},
    {'1': 'DATABASE_KIND_MONGO', '2': 4},
    {'1': 'DATABASE_KIND_REDIS', '2': 5},
    {'1': 'DATABASE_KIND_LIBSQL', '2': 6},
  ],
};

/// Descriptor for `DatabaseKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List databaseKindDescriptor = $convert.base64Decode(
    'CgxEYXRhYmFzZUtpbmQSHQoZREFUQUJBU0VfS0lORF9VTlNQRUNJRklFRBAAEhoKFkRBVEFCQV'
    'NFX0tJTkRfUE9TVEdSRVMQARIXChNEQVRBQkFTRV9LSU5EX01ZU1FMEAISGQoVREFUQUJBU0Vf'
    'S0lORF9NQVJJQURCEAMSFwoTREFUQUJBU0VfS0lORF9NT05HTxAEEhcKE0RBVEFCQVNFX0tJTk'
    'RfUkVESVMQBRIYChREQVRBQkFTRV9LSU5EX0xJQlNRTBAG');

@$core.Deprecated('Use databaseDescriptor instead')
const Database$json = {
  '1': 'Database',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'description',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'docker_image', '3': 5, '4': 1, '5': 9, '10': 'dockerImage'},
    {
      '1': 'database_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'databaseName',
      '17': true
    },
    {
      '1': 'database_user',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'databaseUser',
      '17': true
    },
    {
      '1': 'database_password',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'databasePassword',
      '17': true
    },
    {
      '1': 'external_port',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'externalPort',
      '17': true
    },
    {
      '1': 'command',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'command',
      '17': true
    },
    {'1': 'args', '3': 11, '4': 3, '5': 9, '10': 'args'},
    {
      '1': 'env_var',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'envVar',
      '17': true
    },
    {
      '1': 'memory_limit',
      '3': 13,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'memoryLimit',
      '17': true
    },
    {
      '1': 'cpu_limit',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'cpuLimit',
      '17': true
    },
    {'1': 'replicas', '3': 15, '4': 1, '5': 3, '10': 'replicas'},
    {'1': 'network_ids', '3': 16, '4': 3, '5': 9, '10': 'networkIds'},
    {'1': 'status', '3': 17, '4': 1, '5': 9, '10': 'status'},
    {
      '1': 'last_error',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'lastError',
      '17': true
    },
    {'1': 'environment_id', '3': 19, '4': 1, '5': 3, '10': 'environmentId'},
    {
      '1': 'server_id',
      '3': 20,
      '4': 1,
      '5': 3,
      '9': 10,
      '10': 'serverId',
      '17': true
    },
    {'1': 'created_at', '3': 21, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 22, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'app_name',
      '3': 23,
      '4': 1,
      '5': 9,
      '9': 11,
      '10': 'appName',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_database_name'},
    {'1': '_database_user'},
    {'1': '_database_password'},
    {'1': '_external_port'},
    {'1': '_command'},
    {'1': '_env_var'},
    {'1': '_memory_limit'},
    {'1': '_cpu_limit'},
    {'1': '_last_error'},
    {'1': '_server_id'},
    {'1': '_app_name'},
  ],
};

/// Descriptor for `Database`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseDescriptor = $convert.base64Decode(
    'CghEYXRhYmFzZRI+CgRraW5kGAEgASgOMh4ub3Blbm94aWRlLmFwaS52MS5EYXRhYmFzZUtpbm'
    'RCCrpIB4IBBBABIABSBGtpbmQSFwoCaWQYAiABKANCB7pIBCICIABSAmlkEhIKBG5hbWUYAyAB'
    'KAlSBG5hbWUSJQoLZGVzY3JpcHRpb24YBCABKAlIAFILZGVzY3JpcHRpb26IAQESIQoMZG9ja2'
    'VyX2ltYWdlGAUgASgJUgtkb2NrZXJJbWFnZRIoCg1kYXRhYmFzZV9uYW1lGAYgASgJSAFSDGRh'
    'dGFiYXNlTmFtZYgBARIoCg1kYXRhYmFzZV91c2VyGAcgASgJSAJSDGRhdGFiYXNlVXNlcogBAR'
    'IwChFkYXRhYmFzZV9wYXNzd29yZBgIIAEoCUgDUhBkYXRhYmFzZVBhc3N3b3JkiAEBEigKDWV4'
    'dGVybmFsX3BvcnQYCSABKANIBFIMZXh0ZXJuYWxQb3J0iAEBEh0KB2NvbW1hbmQYCiABKAlIBV'
    'IHY29tbWFuZIgBARISCgRhcmdzGAsgAygJUgRhcmdzEhwKB2Vudl92YXIYDCABKAlIBlIGZW52'
    'VmFyiAEBEiYKDG1lbW9yeV9saW1pdBgNIAEoCUgHUgttZW1vcnlMaW1pdIgBARIgCgljcHVfbG'
    'ltaXQYDiABKAlICFIIY3B1TGltaXSIAQESGgoIcmVwbGljYXMYDyABKANSCHJlcGxpY2FzEh8K'
    'C25ldHdvcmtfaWRzGBAgAygJUgpuZXR3b3JrSWRzEhYKBnN0YXR1cxgRIAEoCVIGc3RhdHVzEi'
    'IKCmxhc3RfZXJyb3IYEiABKAlICVIJbGFzdEVycm9yiAEBEiUKDmVudmlyb25tZW50X2lkGBMg'
    'ASgDUg1lbnZpcm9ubWVudElkEiAKCXNlcnZlcl9pZBgUIAEoA0gKUghzZXJ2ZXJJZIgBARIdCg'
    'pjcmVhdGVkX2F0GBUgASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgWIAEoA1IJdXBkYXRl'
    'ZEF0Eh4KCGFwcF9uYW1lGBcgASgJSAtSB2FwcE5hbWWIAQFCDgoMX2Rlc2NyaXB0aW9uQhAKDl'
    '9kYXRhYmFzZV9uYW1lQhAKDl9kYXRhYmFzZV91c2VyQhQKEl9kYXRhYmFzZV9wYXNzd29yZEIQ'
    'Cg5fZXh0ZXJuYWxfcG9ydEIKCghfY29tbWFuZEIKCghfZW52X3ZhckIPCg1fbWVtb3J5X2xpbW'
    'l0QgwKCl9jcHVfbGltaXRCDQoLX2xhc3RfZXJyb3JCDAoKX3NlcnZlcl9pZEILCglfYXBwX25h'
    'bWU=');

@$core.Deprecated('Use getDatabaseRequestDescriptor instead')
const GetDatabaseRequest$json = {
  '1': 'GetDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `GetDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDatabaseRequestDescriptor = $convert.base64Decode(
    'ChJHZXREYXRhYmFzZVJlcXVlc3QSMgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuRG'
    'F0YWJhc2VLaW5kUgRraW5kEg4KAmlkGAIgASgDUgJpZA==');

@$core.Deprecated('Use listDatabasesRequestDescriptor instead')
const ListDatabasesRequest$json = {
  '1': 'ListDatabasesRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
  ],
};

/// Descriptor for `ListDatabasesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDatabasesRequestDescriptor = $convert.base64Decode(
    'ChRMaXN0RGF0YWJhc2VzUmVxdWVzdBI+CgRraW5kGAEgASgOMh4ub3Blbm94aWRlLmFwaS52MS'
    '5EYXRhYmFzZUtpbmRCCrpIB4IBBBABIABSBGtpbmQSLgoOZW52aXJvbm1lbnRfaWQYAiABKANC'
    'B7pIBCICIABSDWVudmlyb25tZW50SWQ=');

@$core.Deprecated('Use listDatabasesResponseDescriptor instead')
const ListDatabasesResponse$json = {
  '1': 'ListDatabasesResponse',
  '2': [
    {
      '1': 'databases',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Database',
      '10': 'databases'
    },
  ],
};

/// Descriptor for `ListDatabasesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDatabasesResponseDescriptor = $convert.base64Decode(
    'ChVMaXN0RGF0YWJhc2VzUmVzcG9uc2USOAoJZGF0YWJhc2VzGAEgAygLMhoub3Blbm94aWRlLm'
    'FwaS52MS5EYXRhYmFzZVIJZGF0YWJhc2Vz');

@$core.Deprecated('Use createDatabaseRequestDescriptor instead')
const CreateDatabaseRequest$json = {
  '1': 'CreateDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {
      '1': 'environment_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {
      '1': 'server_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'docker_image',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'dockerImage',
      '17': true
    },
    {
      '1': 'database_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'databaseName',
      '17': true
    },
    {
      '1': 'database_user',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'databaseUser',
      '17': true
    },
    {
      '1': 'database_password',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'databasePassword',
      '17': true
    },
    {
      '1': 'external_port',
      '3': 10,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 6,
      '10': 'externalPort',
      '17': true
    },
    {'1': 'args', '3': 11, '4': 3, '5': 9, '10': 'args'},
    {
      '1': 'env_var',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'envVar',
      '17': true
    },
    {'1': 'network_ids', '3': 13, '4': 3, '5': 9, '10': 'networkIds'},
  ],
  '8': [
    {'1': '_description'},
    {'1': '_server_id'},
    {'1': '_docker_image'},
    {'1': '_database_name'},
    {'1': '_database_user'},
    {'1': '_database_password'},
    {'1': '_external_port'},
    {'1': '_env_var'},
  ],
};

/// Descriptor for `CreateDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDatabaseRequestDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVEYXRhYmFzZVJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEh4KBG5hbWUYAiABKAlCCrpIB3IFEAEY'
    '/wFSBG5hbWUSJQoLZGVzY3JpcHRpb24YAyABKAlIAFILZGVzY3JpcHRpb26IAQESLgoOZW52aX'
    'Jvbm1lbnRfaWQYBCABKANCB7pIBCICIABSDWVudmlyb25tZW50SWQSIAoJc2VydmVyX2lkGAUg'
    'ASgDSAFSCHNlcnZlcklkiAEBEiYKDGRvY2tlcl9pbWFnZRgGIAEoCUgCUgtkb2NrZXJJbWFnZY'
    'gBARIoCg1kYXRhYmFzZV9uYW1lGAcgASgJSANSDGRhdGFiYXNlTmFtZYgBARIoCg1kYXRhYmFz'
    'ZV91c2VyGAggASgJSARSDGRhdGFiYXNlVXNlcogBARIwChFkYXRhYmFzZV9wYXNzd29yZBgJIA'
    'EoCUgFUhBkYXRhYmFzZVBhc3N3b3JkiAEBEjUKDWV4dGVybmFsX3BvcnQYCiABKANCC7pICCIG'
    'GP//AygBSAZSDGV4dGVybmFsUG9ydIgBARISCgRhcmdzGAsgAygJUgRhcmdzEhwKB2Vudl92YX'
    'IYDCABKAlIB1IGZW52VmFyiAEBEh8KC25ldHdvcmtfaWRzGA0gAygJUgpuZXR3b3JrSWRzQg4K'
    'DF9kZXNjcmlwdGlvbkIMCgpfc2VydmVyX2lkQg8KDV9kb2NrZXJfaW1hZ2VCEAoOX2RhdGFiYX'
    'NlX25hbWVCEAoOX2RhdGFiYXNlX3VzZXJCFAoSX2RhdGFiYXNlX3Bhc3N3b3JkQhAKDl9leHRl'
    'cm5hbF9wb3J0QgoKCF9lbnZfdmFy');

@$core.Deprecated('Use updateDatabaseRequestDescriptor instead')
const UpdateDatabaseRequest$json = {
  '1': 'UpdateDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'name', '17': true},
    {
      '1': 'description',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'description',
      '17': true
    },
    {
      '1': 'docker_image',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'dockerImage',
      '17': true
    },
    {
      '1': 'database_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'databaseName',
      '17': true
    },
    {
      '1': 'database_user',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'databaseUser',
      '17': true
    },
    {
      '1': 'database_password',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'databasePassword',
      '17': true
    },
    {
      '1': 'external_port',
      '3': 9,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 6,
      '10': 'externalPort',
      '17': true
    },
    {
      '1': 'command',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'command',
      '17': true
    },
    {'1': 'args', '3': 11, '4': 3, '5': 9, '10': 'args'},
    {
      '1': 'env_var',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'envVar',
      '17': true
    },
    {
      '1': 'memory_limit',
      '3': 13,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'memoryLimit',
      '17': true
    },
    {
      '1': 'cpu_limit',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'cpuLimit',
      '17': true
    },
    {
      '1': 'replicas',
      '3': 15,
      '4': 1,
      '5': 3,
      '9': 11,
      '10': 'replicas',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 16,
      '4': 1,
      '5': 3,
      '9': 12,
      '10': 'serverId',
      '17': true
    },
    {'1': 'network_ids', '3': 17, '4': 3, '5': 9, '10': 'networkIds'},
    {
      '1': 'detach_rustploy_network',
      '3': 18,
      '4': 1,
      '5': 3,
      '9': 13,
      '10': 'detachRustployNetwork',
      '17': true
    },
    {
      '1': 'memory_reservation',
      '3': 19,
      '4': 1,
      '5': 9,
      '9': 14,
      '10': 'memoryReservation',
      '17': true
    },
    {
      '1': 'cpu_reservation',
      '3': 20,
      '4': 1,
      '5': 9,
      '9': 15,
      '10': 'cpuReservation',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_docker_image'},
    {'1': '_database_name'},
    {'1': '_database_user'},
    {'1': '_database_password'},
    {'1': '_external_port'},
    {'1': '_command'},
    {'1': '_env_var'},
    {'1': '_memory_limit'},
    {'1': '_cpu_limit'},
    {'1': '_replicas'},
    {'1': '_server_id'},
    {'1': '_detach_rustploy_network'},
    {'1': '_memory_reservation'},
    {'1': '_cpu_reservation'},
  ],
};

/// Descriptor for `UpdateDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDatabaseRequestDescriptor = $convert.base64Decode(
    'ChVVcGRhdGVEYXRhYmFzZVJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQiAiAAUgJp'
    'ZBIXCgRuYW1lGAMgASgJSABSBG5hbWWIAQESJQoLZGVzY3JpcHRpb24YBCABKAlIAVILZGVzY3'
    'JpcHRpb26IAQESJgoMZG9ja2VyX2ltYWdlGAUgASgJSAJSC2RvY2tlckltYWdliAEBEigKDWRh'
    'dGFiYXNlX25hbWUYBiABKAlIA1IMZGF0YWJhc2VOYW1liAEBEigKDWRhdGFiYXNlX3VzZXIYBy'
    'ABKAlIBFIMZGF0YWJhc2VVc2VyiAEBEjAKEWRhdGFiYXNlX3Bhc3N3b3JkGAggASgJSAVSEGRh'
    'dGFiYXNlUGFzc3dvcmSIAQESNQoNZXh0ZXJuYWxfcG9ydBgJIAEoA0ILukgIIgYY//8DKAFIBl'
    'IMZXh0ZXJuYWxQb3J0iAEBEh0KB2NvbW1hbmQYCiABKAlIB1IHY29tbWFuZIgBARISCgRhcmdz'
    'GAsgAygJUgRhcmdzEhwKB2Vudl92YXIYDCABKAlICFIGZW52VmFyiAEBEiYKDG1lbW9yeV9saW'
    '1pdBgNIAEoCUgJUgttZW1vcnlMaW1pdIgBARIgCgljcHVfbGltaXQYDiABKAlIClIIY3B1TGlt'
    'aXSIAQESHwoIcmVwbGljYXMYDyABKANIC1IIcmVwbGljYXOIAQESIAoJc2VydmVyX2lkGBAgAS'
    'gDSAxSCHNlcnZlcklkiAEBEh8KC25ldHdvcmtfaWRzGBEgAygJUgpuZXR3b3JrSWRzEjsKF2Rl'
    'dGFjaF9ydXN0cGxveV9uZXR3b3JrGBIgASgDSA1SFWRldGFjaFJ1c3RwbG95TmV0d29ya4gBAR'
    'IyChJtZW1vcnlfcmVzZXJ2YXRpb24YEyABKAlIDlIRbWVtb3J5UmVzZXJ2YXRpb26IAQESLAoP'
    'Y3B1X3Jlc2VydmF0aW9uGBQgASgJSA9SDmNwdVJlc2VydmF0aW9uiAEBQgcKBV9uYW1lQg4KDF'
    '9kZXNjcmlwdGlvbkIPCg1fZG9ja2VyX2ltYWdlQhAKDl9kYXRhYmFzZV9uYW1lQhAKDl9kYXRh'
    'YmFzZV91c2VyQhQKEl9kYXRhYmFzZV9wYXNzd29yZEIQCg5fZXh0ZXJuYWxfcG9ydEIKCghfY2'
    '9tbWFuZEIKCghfZW52X3ZhckIPCg1fbWVtb3J5X2xpbWl0QgwKCl9jcHVfbGltaXRCCwoJX3Jl'
    'cGxpY2FzQgwKCl9zZXJ2ZXJfaWRCGgoYX2RldGFjaF9ydXN0cGxveV9uZXR3b3JrQhUKE19tZW'
    '1vcnlfcmVzZXJ2YXRpb25CEgoQX2NwdV9yZXNlcnZhdGlvbg==');

@$core.Deprecated('Use deleteDatabaseRequestDescriptor instead')
const DeleteDatabaseRequest$json = {
  '1': 'DeleteDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteDatabaseRequestDescriptor = $convert.base64Decode(
    'ChVEZWxldGVEYXRhYmFzZVJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQiAiAAUgJp'
    'ZA==');

@$core.Deprecated('Use databaseOperationRequestDescriptor instead')
const DatabaseOperationRequest$json = {
  '1': 'DatabaseOperationRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DatabaseOperationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseOperationRequestDescriptor = $convert.base64Decode(
    'ChhEYXRhYmFzZU9wZXJhdGlvblJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcG'
    'kudjEuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQiAiAA'
    'UgJpZA==');

@$core.Deprecated('Use databaseOperationDescriptor instead')
const DatabaseOperation$json = {
  '1': 'DatabaseOperation',
  '2': [
    {
      '1': 'database',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Database',
      '10': 'database'
    },
    {'1': 'operation', '3': 2, '4': 1, '5': 9, '10': 'operation'},
  ],
};

/// Descriptor for `DatabaseOperation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseOperationDescriptor = $convert.base64Decode(
    'ChFEYXRhYmFzZU9wZXJhdGlvbhI2CghkYXRhYmFzZRgBIAEoCzIaLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VSCGRhdGFiYXNlEhwKCW9wZXJhdGlvbhgCIAEoCVIJb3BlcmF0aW9u');

@$core.Deprecated('Use databaseTargetRequestDescriptor instead')
const DatabaseTargetRequest$json = {
  '1': 'DatabaseTargetRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DatabaseTargetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseTargetRequestDescriptor = $convert.base64Decode(
    'ChVEYXRhYmFzZVRhcmdldFJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQiAiAAUgJp'
    'ZA==');

@$core.Deprecated('Use swarmSettingsRequestDescriptor instead')
const SwarmSettingsRequest$json = {
  '1': 'SwarmSettingsRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SwarmSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmSettingsRequestDescriptor = $convert.base64Decode(
    'ChRTd2FybVNldHRpbmdzUmVxdWVzdBI+CgRraW5kGAEgASgOMh4ub3Blbm94aWRlLmFwaS52MS'
    '5EYXRhYmFzZUtpbmRCCrpIB4IBBBABIABSBGtpbmQSFwoCaWQYAiABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use swarmSettingsDescriptor instead')
const SwarmSettings$json = {
  '1': 'SwarmSettings',
  '2': [
    {
      '1': 'health_check',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'healthCheck',
      '17': true
    },
    {
      '1': 'restart_policy',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'restartPolicy',
      '17': true
    },
    {
      '1': 'placement',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'placement',
      '17': true
    },
    {
      '1': 'update_config',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'updateConfig',
      '17': true
    },
    {
      '1': 'rollback_config',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'rollbackConfig',
      '17': true
    },
    {'1': 'mode', '3': 6, '4': 1, '5': 9, '9': 5, '10': 'mode', '17': true},
    {'1': 'labels', '3': 7, '4': 1, '5': 9, '9': 6, '10': 'labels', '17': true},
    {
      '1': 'network',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'network',
      '17': true
    },
    {
      '1': 'endpoint_spec',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'endpointSpec',
      '17': true
    },
    {
      '1': 'ulimits',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'ulimits',
      '17': true
    },
    {
      '1': 'stop_grace_period',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 10,
      '10': 'stopGracePeriod',
      '17': true
    },
  ],
  '8': [
    {'1': '_health_check'},
    {'1': '_restart_policy'},
    {'1': '_placement'},
    {'1': '_update_config'},
    {'1': '_rollback_config'},
    {'1': '_mode'},
    {'1': '_labels'},
    {'1': '_network'},
    {'1': '_endpoint_spec'},
    {'1': '_ulimits'},
    {'1': '_stop_grace_period'},
  ],
};

/// Descriptor for `SwarmSettings`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List swarmSettingsDescriptor = $convert.base64Decode(
    'Cg1Td2FybVNldHRpbmdzEiYKDGhlYWx0aF9jaGVjaxgBIAEoCUgAUgtoZWFsdGhDaGVja4gBAR'
    'IqCg5yZXN0YXJ0X3BvbGljeRgCIAEoCUgBUg1yZXN0YXJ0UG9saWN5iAEBEiEKCXBsYWNlbWVu'
    'dBgDIAEoCUgCUglwbGFjZW1lbnSIAQESKAoNdXBkYXRlX2NvbmZpZxgEIAEoCUgDUgx1cGRhdG'
    'VDb25maWeIAQESLAoPcm9sbGJhY2tfY29uZmlnGAUgASgJSARSDnJvbGxiYWNrQ29uZmlniAEB'
    'EhcKBG1vZGUYBiABKAlIBVIEbW9kZYgBARIbCgZsYWJlbHMYByABKAlIBlIGbGFiZWxziAEBEh'
    '0KB25ldHdvcmsYCCABKAlIB1IHbmV0d29ya4gBARIoCg1lbmRwb2ludF9zcGVjGAkgASgJSAhS'
    'DGVuZHBvaW50U3BlY4gBARIdCgd1bGltaXRzGAogASgJSAlSB3VsaW1pdHOIAQESLwoRc3RvcF'
    '9ncmFjZV9wZXJpb2QYCyABKANIClIPc3RvcEdyYWNlUGVyaW9kiAEBQg8KDV9oZWFsdGhfY2hl'
    'Y2tCEQoPX3Jlc3RhcnRfcG9saWN5QgwKCl9wbGFjZW1lbnRCEAoOX3VwZGF0ZV9jb25maWdCEg'
    'oQX3JvbGxiYWNrX2NvbmZpZ0IHCgVfbW9kZUIJCgdfbGFiZWxzQgoKCF9uZXR3b3JrQhAKDl9l'
    'bmRwb2ludF9zcGVjQgoKCF91bGltaXRzQhQKEl9zdG9wX2dyYWNlX3BlcmlvZA==');

@$core.Deprecated('Use updateSwarmSettingsRequestDescriptor instead')
const UpdateSwarmSettingsRequest$json = {
  '1': 'UpdateSwarmSettingsRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'settings',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SwarmSettings',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `UpdateSwarmSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateSwarmSettingsRequestDescriptor = $convert.base64Decode(
    'ChpVcGRhdGVTd2FybVNldHRpbmdzUmVxdWVzdBI+CgRraW5kGAEgASgOMh4ub3Blbm94aWRlLm'
    'FwaS52MS5EYXRhYmFzZUtpbmRCCrpIB4IBBBABIABSBGtpbmQSFwoCaWQYAiABKANCB7pIBCIC'
    'IABSAmlkEjsKCHNldHRpbmdzGAMgASgLMh8ub3Blbm94aWRlLmFwaS52MS5Td2FybVNldHRpbm'
    'dzUghzZXR0aW5ncw==');

@$core.Deprecated('Use mountDescriptor instead')
const Mount$json = {
  '1': 'Mount',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'mount_type', '3': 2, '4': 1, '5': 9, '10': 'mountType'},
    {
      '1': 'host_path',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'hostPath',
      '17': true
    },
    {
      '1': 'volume_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'volumeName',
      '17': true
    },
    {
      '1': 'file_path',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'filePath',
      '17': true
    },
    {
      '1': 'content',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'content',
      '17': true
    },
    {'1': 'mount_path', '3': 7, '4': 1, '5': 9, '10': 'mountPath'},
    {'1': 'created_at', '3': 8, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 9, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `Mount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mountDescriptor = $convert.base64Decode(
    'CgVNb3VudBIOCgJpZBgBIAEoA1ICaWQSHQoKbW91bnRfdHlwZRgCIAEoCVIJbW91bnRUeXBlEi'
    'AKCWhvc3RfcGF0aBgDIAEoCUgAUghob3N0UGF0aIgBARIkCgt2b2x1bWVfbmFtZRgEIAEoCUgB'
    'Ugp2b2x1bWVOYW1liAEBEiAKCWZpbGVfcGF0aBgFIAEoCUgCUghmaWxlUGF0aIgBARIdCgdjb2'
    '50ZW50GAYgASgJSANSB2NvbnRlbnSIAQESHQoKbW91bnRfcGF0aBgHIAEoCVIJbW91bnRQYXRo'
    'Eh0KCmNyZWF0ZWRfYXQYCCABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAkgASgDUgl1cG'
    'RhdGVkQXRCDAoKX2hvc3RfcGF0aEIOCgxfdm9sdW1lX25hbWVCDAoKX2ZpbGVfcGF0aEIKCghf'
    'Y29udGVudA==');

@$core.Deprecated('Use mountListDescriptor instead')
const MountList$json = {
  '1': 'MountList',
  '2': [
    {
      '1': 'mounts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Mount',
      '10': 'mounts'
    },
  ],
};

/// Descriptor for `MountList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mountListDescriptor = $convert.base64Decode(
    'CglNb3VudExpc3QSLwoGbW91bnRzGAEgAygLMhcub3Blbm94aWRlLmFwaS52MS5Nb3VudFIGbW'
    '91bnRz');

@$core.Deprecated('Use createMountRequestDescriptor instead')
const CreateMountRequest$json = {
  '1': 'CreateMountRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'database_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'databaseId'},
    {'1': 'mount_type', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'mountType'},
    {
      '1': 'host_path',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'hostPath',
      '17': true
    },
    {
      '1': 'volume_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'volumeName',
      '17': true
    },
    {
      '1': 'file_path',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'filePath',
      '17': true
    },
    {
      '1': 'content',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'content',
      '17': true
    },
    {'1': 'mount_path', '3': 8, '4': 1, '5': 9, '8': {}, '10': 'mountPath'},
  ],
  '8': [
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `CreateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createMountRequestDescriptor = $convert.base64Decode(
    'ChJDcmVhdGVNb3VudFJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuRG'
    'F0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEigKC2RhdGFiYXNlX2lkGAIgASgDQge6SAQi'
    'AiAAUgpkYXRhYmFzZUlkEiYKCm1vdW50X3R5cGUYAyABKAlCB7pIBHICEAFSCW1vdW50VHlwZR'
    'IgCglob3N0X3BhdGgYBCABKAlIAFIIaG9zdFBhdGiIAQESJAoLdm9sdW1lX25hbWUYBSABKAlI'
    'AVIKdm9sdW1lTmFtZYgBARIgCglmaWxlX3BhdGgYBiABKAlIAlIIZmlsZVBhdGiIAQESHQoHY2'
    '9udGVudBgHIAEoCUgDUgdjb250ZW50iAEBEiYKCm1vdW50X3BhdGgYCCABKAlCB7pIBHICEAFS'
    'CW1vdW50UGF0aEIMCgpfaG9zdF9wYXRoQg4KDF92b2x1bWVfbmFtZUIMCgpfZmlsZV9wYXRoQg'
    'oKCF9jb250ZW50');

@$core.Deprecated('Use updateMountRequestDescriptor instead')
const UpdateMountRequest$json = {
  '1': 'UpdateMountRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'database_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'databaseId'},
    {'1': 'id', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'mount_type', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'mountType'},
    {
      '1': 'host_path',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'hostPath',
      '17': true
    },
    {
      '1': 'volume_name',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'volumeName',
      '17': true
    },
    {
      '1': 'file_path',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'filePath',
      '17': true
    },
    {
      '1': 'content',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'content',
      '17': true
    },
    {'1': 'mount_path', '3': 9, '4': 1, '5': 9, '8': {}, '10': 'mountPath'},
  ],
  '8': [
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `UpdateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateMountRequestDescriptor = $convert.base64Decode(
    'ChJVcGRhdGVNb3VudFJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuRG'
    'F0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEigKC2RhdGFiYXNlX2lkGAIgASgDQge6SAQi'
    'AiAAUgpkYXRhYmFzZUlkEhcKAmlkGAMgASgDQge6SAQiAiAAUgJpZBImCgptb3VudF90eXBlGA'
    'QgASgJQge6SARyAhABUgltb3VudFR5cGUSIAoJaG9zdF9wYXRoGAUgASgJSABSCGhvc3RQYXRo'
    'iAEBEiQKC3ZvbHVtZV9uYW1lGAYgASgJSAFSCnZvbHVtZU5hbWWIAQESIAoJZmlsZV9wYXRoGA'
    'cgASgJSAJSCGZpbGVQYXRoiAEBEh0KB2NvbnRlbnQYCCABKAlIA1IHY29udGVudIgBARImCgpt'
    'b3VudF9wYXRoGAkgASgJQge6SARyAhABUgltb3VudFBhdGhCDAoKX2hvc3RfcGF0aEIOCgxfdm'
    '9sdW1lX25hbWVCDAoKX2ZpbGVfcGF0aEIKCghfY29udGVudA==');

@$core.Deprecated('Use deleteMountRequestDescriptor instead')
const DeleteMountRequest$json = {
  '1': 'DeleteMountRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'database_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'databaseId'},
    {'1': 'id', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteMountRequestDescriptor = $convert.base64Decode(
    'ChJEZWxldGVNb3VudFJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuRG'
    'F0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEigKC2RhdGFiYXNlX2lkGAIgASgDQge6SAQi'
    'AiAAUgpkYXRhYmFzZUlkEhcKAmlkGAMgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use dependenciesDescriptor instead')
const Dependencies$json = {
  '1': 'Dependencies',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `Dependencies`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dependenciesDescriptor =
    $convert.base64Decode('CgxEZXBlbmRlbmNpZXMSFAoFaXRlbXMYASADKAlSBWl0ZW1z');

@$core.Deprecated('Use exportDatabaseRequestDescriptor instead')
const ExportDatabaseRequest$json = {
  '1': 'ExportDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'include_secrets', '3': 3, '4': 1, '5': 8, '10': 'includeSecrets'},
  ],
};

/// Descriptor for `ExportDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exportDatabaseRequestDescriptor = $convert.base64Decode(
    'ChVFeHBvcnREYXRhYmFzZVJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQiAiAAUgJp'
    'ZBInCg9pbmNsdWRlX3NlY3JldHMYAyABKAhSDmluY2x1ZGVTZWNyZXRz');

@$core.Deprecated('Use databaseExportDescriptor instead')
const DatabaseExport$json = {
  '1': 'DatabaseExport',
  '2': [
    {'1': 'archive', '3': 1, '4': 1, '5': 9, '10': 'archive'},
  ],
};

/// Descriptor for `DatabaseExport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseExportDescriptor = $convert
    .base64Decode('Cg5EYXRhYmFzZUV4cG9ydBIYCgdhcmNoaXZlGAEgASgJUgdhcmNoaXZl');

@$core.Deprecated('Use importDatabaseRequestDescriptor instead')
const ImportDatabaseRequest$json = {
  '1': 'ImportDatabaseRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {'1': 'archive', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'archive'},
  ],
};

/// Descriptor for `ImportDatabaseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importDatabaseRequestDescriptor = $convert.base64Decode(
    'ChVJbXBvcnREYXRhYmFzZVJlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudj'
    'EuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEi4KDmVudmlyb25tZW50X2lkGAIgASgD'
    'Qge6SAQiAiAAUg1lbnZpcm9ubWVudElkEiYKB2FyY2hpdmUYAyABKAlCDLpICXIHEAEYgICABV'
    'IHYXJjaGl2ZQ==');

@$core.Deprecated('Use databaseConnectionDescriptor instead')
const DatabaseConnection$json = {
  '1': 'DatabaseConnection',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '10': 'kind'
    },
    {'1': 'host', '3': 2, '4': 1, '5': 9, '10': 'host'},
    {'1': 'port', '3': 3, '4': 1, '5': 3, '10': 'port'},
    {'1': 'database', '3': 4, '4': 1, '5': 9, '10': 'database'},
    {'1': 'username', '3': 5, '4': 1, '5': 9, '10': 'username'},
  ],
};

/// Descriptor for `DatabaseConnection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseConnectionDescriptor = $convert.base64Decode(
    'ChJEYXRhYmFzZUNvbm5lY3Rpb24SMgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuRG'
    'F0YWJhc2VLaW5kUgRraW5kEhIKBGhvc3QYAiABKAlSBGhvc3QSEgoEcG9ydBgDIAEoA1IEcG9y'
    'dBIaCghkYXRhYmFzZRgEIAEoCVIIZGF0YWJhc2USGgoIdXNlcm5hbWUYBSABKAlSCHVzZXJuYW'
    '1l');

@$core.Deprecated('Use validationResultDescriptor instead')
const ValidationResult$json = {
  '1': 'ValidationResult',
  '2': [
    {'1': 'valid', '3': 1, '4': 1, '5': 8, '10': 'valid'},
    {
      '1': 'message',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'message',
      '17': true
    },
  ],
  '8': [
    {'1': '_message'},
  ],
};

/// Descriptor for `ValidationResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List validationResultDescriptor = $convert.base64Decode(
    'ChBWYWxpZGF0aW9uUmVzdWx0EhQKBXZhbGlkGAEgASgIUgV2YWxpZBIdCgdtZXNzYWdlGAIgAS'
    'gJSABSB21lc3NhZ2WIAQFCCgoIX21lc3NhZ2U=');

@$core.Deprecated('Use advancedConfigDescriptor instead')
const AdvancedConfig$json = {
  '1': 'AdvancedConfig',
  '2': [
    {
      '1': 'replication_role',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'replicationRole',
      '17': true
    },
    {'1': 'config', '3': 2, '4': 1, '5': 9, '9': 1, '10': 'config', '17': true},
  ],
  '8': [
    {'1': '_replication_role'},
    {'1': '_config'},
  ],
};

/// Descriptor for `AdvancedConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List advancedConfigDescriptor = $convert.base64Decode(
    'Cg5BZHZhbmNlZENvbmZpZxIuChByZXBsaWNhdGlvbl9yb2xlGAEgASgJSABSD3JlcGxpY2F0aW'
    '9uUm9sZYgBARIbCgZjb25maWcYAiABKAlIAVIGY29uZmlniAEBQhMKEV9yZXBsaWNhdGlvbl9y'
    'b2xlQgkKB19jb25maWc=');

@$core.Deprecated('Use updateAdvancedConfigRequestDescriptor instead')
const UpdateAdvancedConfigRequest$json = {
  '1': 'UpdateAdvancedConfigRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.DatabaseKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'config',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AdvancedConfig',
      '10': 'config'
    },
  ],
};

/// Descriptor for `UpdateAdvancedConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateAdvancedConfigRequestDescriptor = $convert.base64Decode(
    'ChtVcGRhdGVBZHZhbmNlZENvbmZpZ1JlcXVlc3QSPgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS'
    '5hcGkudjEuRGF0YWJhc2VLaW5kQgq6SAeCAQQQASAAUgRraW5kEhcKAmlkGAIgASgDQge6SAQi'
    'AiAAUgJpZBI4CgZjb25maWcYAyABKAsyIC5vcGVub3hpZGUuYXBpLnYxLkFkdmFuY2VkQ29uZm'
    'lnUgZjb25maWc=');

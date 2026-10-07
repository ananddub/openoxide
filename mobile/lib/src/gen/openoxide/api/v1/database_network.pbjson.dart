// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database_network.proto.

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

@$core.Deprecated('Use databaseNetworkDescriptor instead')
const DatabaseNetwork$json = {
  '1': 'DatabaseNetwork',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'docker_network_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'dockerNetworkName'
    },
    {
      '1': 'description',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'external', '3': 5, '4': 1, '5': 3, '10': 'external'},
    {
      '1': 'server_id',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_description'},
    {'1': '_server_id'},
  ],
};

/// Descriptor for `DatabaseNetwork`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkDescriptor = $convert.base64Decode(
    'Cg9EYXRhYmFzZU5ldHdvcmsSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSLg'
    'oTZG9ja2VyX25ldHdvcmtfbmFtZRgDIAEoCVIRZG9ja2VyTmV0d29ya05hbWUSJQoLZGVzY3Jp'
    'cHRpb24YBCABKAlIAFILZGVzY3JpcHRpb26IAQESGgoIZXh0ZXJuYWwYBSABKANSCGV4dGVybm'
    'FsEiAKCXNlcnZlcl9pZBgGIAEoA0gBUghzZXJ2ZXJJZIgBARIdCgpjcmVhdGVkX2F0GAcgASgD'
    'UgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgIIAEoA1IJdXBkYXRlZEF0Qg4KDF9kZXNjcmlwdG'
    'lvbkIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use databaseNetworkDependenciesDescriptor instead')
const DatabaseNetworkDependencies$json = {
  '1': 'DatabaseNetworkDependencies',
  '2': [
    {'1': 'applications', '3': 1, '4': 1, '5': 3, '10': 'applications'},
    {'1': 'compose_services', '3': 2, '4': 1, '5': 3, '10': 'composeServices'},
    {'1': 'databases', '3': 3, '4': 1, '5': 3, '10': 'databases'},
  ],
};

/// Descriptor for `DatabaseNetworkDependencies`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkDependenciesDescriptor =
    $convert.base64Decode(
        'ChtEYXRhYmFzZU5ldHdvcmtEZXBlbmRlbmNpZXMSIgoMYXBwbGljYXRpb25zGAEgASgDUgxhcH'
        'BsaWNhdGlvbnMSKQoQY29tcG9zZV9zZXJ2aWNlcxgCIAEoA1IPY29tcG9zZVNlcnZpY2VzEhwK'
        'CWRhdGFiYXNlcxgDIAEoA1IJZGF0YWJhc2Vz');

@$core.Deprecated('Use databaseNetworkListNetworksRequestDescriptor instead')
const DatabaseNetworkListNetworksRequest$json = {
  '1': 'DatabaseNetworkListNetworksRequest',
};

/// Descriptor for `DatabaseNetworkListNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkListNetworksRequestDescriptor =
    $convert.base64Decode('CiJEYXRhYmFzZU5ldHdvcmtMaXN0TmV0d29ya3NSZXF1ZXN0');

@$core.Deprecated('Use databaseNetworkListNetworksResponseDescriptor instead')
const DatabaseNetworkListNetworksResponse$json = {
  '1': 'DatabaseNetworkListNetworksResponse',
  '2': [
    {
      '1': 'networks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetwork',
      '10': 'networks'
    },
  ],
};

/// Descriptor for `DatabaseNetworkListNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkListNetworksResponseDescriptor =
    $convert.base64Decode(
        'CiNEYXRhYmFzZU5ldHdvcmtMaXN0TmV0d29ya3NSZXNwb25zZRI9CghuZXR3b3JrcxgBIAMoCz'
        'IhLm9wZW5veGlkZS5hcGkudjEuRGF0YWJhc2VOZXR3b3JrUghuZXR3b3Jrcw==');

@$core.Deprecated(
    'Use databaseNetworkListServerNetworksRequestDescriptor instead')
const DatabaseNetworkListServerNetworksRequest$json = {
  '1': 'DatabaseNetworkListServerNetworksRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
  ],
};

/// Descriptor for `DatabaseNetworkListServerNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkListServerNetworksRequestDescriptor =
    $convert.base64Decode(
        'CihEYXRhYmFzZU5ldHdvcmtMaXN0U2VydmVyTmV0d29ya3NSZXF1ZXN0EiQKCXNlcnZlcl9pZB'
        'gBIAEoA0IHukgEIgIgAFIIc2VydmVySWQ=');

@$core.Deprecated(
    'Use databaseNetworkListServerNetworksResponseDescriptor instead')
const DatabaseNetworkListServerNetworksResponse$json = {
  '1': 'DatabaseNetworkListServerNetworksResponse',
  '2': [
    {
      '1': 'networks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetwork',
      '10': 'networks'
    },
  ],
};

/// Descriptor for `DatabaseNetworkListServerNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    databaseNetworkListServerNetworksResponseDescriptor = $convert.base64Decode(
        'CilEYXRhYmFzZU5ldHdvcmtMaXN0U2VydmVyTmV0d29ya3NSZXNwb25zZRI9CghuZXR3b3Jrcx'
        'gBIAMoCzIhLm9wZW5veGlkZS5hcGkudjEuRGF0YWJhc2VOZXR3b3JrUghuZXR3b3Jrcw==');

@$core.Deprecated('Use databaseNetworkGetNetworkRequestDescriptor instead')
const DatabaseNetworkGetNetworkRequest$json = {
  '1': 'DatabaseNetworkGetNetworkRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DatabaseNetworkGetNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkGetNetworkRequestDescriptor =
    $convert.base64Decode(
        'CiBEYXRhYmFzZU5ldHdvcmtHZXROZXR3b3JrUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAF'
        'ICaWQ=');

@$core.Deprecated('Use databaseNetworkGetNetworkResponseDescriptor instead')
const DatabaseNetworkGetNetworkResponse$json = {
  '1': 'DatabaseNetworkGetNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `DatabaseNetworkGetNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkGetNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiFEYXRhYmFzZU5ldHdvcmtHZXROZXR3b3JrUmVzcG9uc2USOwoHbmV0d29yaxgBIAEoCzIhLm'
        '9wZW5veGlkZS5hcGkudjEuRGF0YWJhc2VOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use databaseNetworkCreateNetworkRequestDescriptor instead')
const DatabaseNetworkCreateNetworkRequest$json = {
  '1': 'DatabaseNetworkCreateNetworkRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'docker_network_name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'dockerNetworkName'
    },
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
      '1': 'external',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'external',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'serverId',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_external'},
    {'1': '_server_id'},
  ],
};

/// Descriptor for `DatabaseNetworkCreateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkCreateNetworkRequestDescriptor = $convert.base64Decode(
    'CiNEYXRhYmFzZU5ldHdvcmtDcmVhdGVOZXR3b3JrUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SA'
    'dyBRABGP8BUgRuYW1lEjoKE2RvY2tlcl9uZXR3b3JrX25hbWUYAiABKAlCCrpIB3IFEAEY/wFS'
    'EWRvY2tlck5ldHdvcmtOYW1lEiUKC2Rlc2NyaXB0aW9uGAMgASgJSABSC2Rlc2NyaXB0aW9uiA'
    'EBEh8KCGV4dGVybmFsGAQgASgDSAFSCGV4dGVybmFsiAEBEikKCXNlcnZlcl9pZBgFIAEoA0IH'
    'ukgEIgIgAEgCUghzZXJ2ZXJJZIgBAUIOCgxfZGVzY3JpcHRpb25CCwoJX2V4dGVybmFsQgwKCl'
    '9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use databaseNetworkCreateNetworkResponseDescriptor instead')
const DatabaseNetworkCreateNetworkResponse$json = {
  '1': 'DatabaseNetworkCreateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `DatabaseNetworkCreateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkCreateNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiREYXRhYmFzZU5ldHdvcmtDcmVhdGVOZXR3b3JrUmVzcG9uc2USOwoHbmV0d29yaxgBIAEoCz'
        'IhLm9wZW5veGlkZS5hcGkudjEuRGF0YWJhc2VOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use databaseNetworkUpdateNetworkRequestDescriptor instead')
const DatabaseNetworkUpdateNetworkRequest$json = {
  '1': 'DatabaseNetworkUpdateNetworkRequest',
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
      '1': 'docker_network_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'dockerNetworkName',
      '17': true
    },
    {
      '1': 'description',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'description',
      '17': true
    },
    {
      '1': 'external',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'external',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'serverId',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_docker_network_name'},
    {'1': '_description'},
    {'1': '_external'},
    {'1': '_server_id'},
  ],
};

/// Descriptor for `DatabaseNetworkUpdateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkUpdateNetworkRequestDescriptor = $convert.base64Decode(
    'CiNEYXRhYmFzZU5ldHdvcmtVcGRhdGVOZXR3b3JrUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIg'
    'IgAFICaWQSIQoEbmFtZRgCIAEoCUIIukgFcgMY/wFIAFIEbmFtZYgBARI9ChNkb2NrZXJfbmV0'
    'd29ya19uYW1lGAMgASgJQgi6SAVyAxj/AUgBUhFkb2NrZXJOZXR3b3JrTmFtZYgBARIlCgtkZX'
    'NjcmlwdGlvbhgEIAEoCUgCUgtkZXNjcmlwdGlvbogBARIfCghleHRlcm5hbBgFIAEoA0gDUghl'
    'eHRlcm5hbIgBARIgCglzZXJ2ZXJfaWQYBiABKANIBFIIc2VydmVySWSIAQFCBwoFX25hbWVCFg'
    'oUX2RvY2tlcl9uZXR3b3JrX25hbWVCDgoMX2Rlc2NyaXB0aW9uQgsKCV9leHRlcm5hbEIMCgpf'
    'c2VydmVyX2lk');

@$core.Deprecated('Use databaseNetworkUpdateNetworkResponseDescriptor instead')
const DatabaseNetworkUpdateNetworkResponse$json = {
  '1': 'DatabaseNetworkUpdateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `DatabaseNetworkUpdateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkUpdateNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiREYXRhYmFzZU5ldHdvcmtVcGRhdGVOZXR3b3JrUmVzcG9uc2USOwoHbmV0d29yaxgBIAEoCz'
        'IhLm9wZW5veGlkZS5hcGkudjEuRGF0YWJhc2VOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use databaseNetworkDeleteNetworkRequestDescriptor instead')
const DatabaseNetworkDeleteNetworkRequest$json = {
  '1': 'DatabaseNetworkDeleteNetworkRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DatabaseNetworkDeleteNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkDeleteNetworkRequestDescriptor =
    $convert.base64Decode(
        'CiNEYXRhYmFzZU5ldHdvcmtEZWxldGVOZXR3b3JrUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIg'
        'IgAFICaWQ=');

@$core
    .Deprecated('Use databaseNetworkListDependenciesRequestDescriptor instead')
const DatabaseNetworkListDependenciesRequest$json = {
  '1': 'DatabaseNetworkListDependenciesRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DatabaseNetworkListDependenciesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkListDependenciesRequestDescriptor =
    $convert.base64Decode(
        'CiZEYXRhYmFzZU5ldHdvcmtMaXN0RGVwZW5kZW5jaWVzUmVxdWVzdBIXCgJpZBgBIAEoA0IHuk'
        'gEIgIgAFICaWQ=');

@$core
    .Deprecated('Use databaseNetworkListDependenciesResponseDescriptor instead')
const DatabaseNetworkListDependenciesResponse$json = {
  '1': 'DatabaseNetworkListDependenciesResponse',
  '2': [
    {
      '1': 'dependencies',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DatabaseNetworkDependencies',
      '10': 'dependencies'
    },
  ],
};

/// Descriptor for `DatabaseNetworkListDependenciesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List databaseNetworkListDependenciesResponseDescriptor =
    $convert.base64Decode(
        'CidEYXRhYmFzZU5ldHdvcmtMaXN0RGVwZW5kZW5jaWVzUmVzcG9uc2USUQoMZGVwZW5kZW5jaW'
        'VzGAEgASgLMi0ub3Blbm94aWRlLmFwaS52MS5EYXRhYmFzZU5ldHdvcmtEZXBlbmRlbmNpZXNS'
        'DGRlcGVuZGVuY2llcw==');

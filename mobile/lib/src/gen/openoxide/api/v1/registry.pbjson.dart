// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/registry.proto.

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

@$core.Deprecated('Use registryDescriptor instead')
const Registry$json = {
  '1': 'Registry',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'registry_name', '3': 2, '4': 1, '5': 9, '10': 'registryName'},
    {
      '1': 'image_prefix',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'imagePrefix',
      '17': true
    },
    {'1': 'username', '3': 4, '4': 1, '5': 9, '10': 'username'},
    {'1': 'password', '3': 5, '4': 1, '5': 9, '10': 'password'},
    {'1': 'registry_url', '3': 6, '4': 1, '5': 9, '10': 'registryUrl'},
    {'1': 'registry_type', '3': 7, '4': 1, '5': 9, '10': 'registryType'},
    {'1': 'created_at', '3': 8, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 9, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_image_prefix'},
  ],
};

/// Descriptor for `Registry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registryDescriptor = $convert.base64Decode(
    'CghSZWdpc3RyeRIOCgJpZBgBIAEoA1ICaWQSIwoNcmVnaXN0cnlfbmFtZRgCIAEoCVIMcmVnaX'
    'N0cnlOYW1lEiYKDGltYWdlX3ByZWZpeBgDIAEoCUgAUgtpbWFnZVByZWZpeIgBARIaCgh1c2Vy'
    'bmFtZRgEIAEoCVIIdXNlcm5hbWUSGgoIcGFzc3dvcmQYBSABKAlSCHBhc3N3b3JkEiEKDHJlZ2'
    'lzdHJ5X3VybBgGIAEoCVILcmVnaXN0cnlVcmwSIwoNcmVnaXN0cnlfdHlwZRgHIAEoCVIMcmVn'
    'aXN0cnlUeXBlEh0KCmNyZWF0ZWRfYXQYCCABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GA'
    'kgASgDUgl1cGRhdGVkQXRCDwoNX2ltYWdlX3ByZWZpeA==');

@$core.Deprecated('Use registryUsageReferenceDescriptor instead')
const RegistryUsageReference$json = {
  '1': 'RegistryUsageReference',
  '2': [
    {'1': 'resource_id', '3': 1, '4': 1, '5': 9, '10': 'resourceId'},
    {'1': 'resource_name', '3': 2, '4': 1, '5': 9, '10': 'resourceName'},
    {'1': 'resource_type', '3': 3, '4': 1, '5': 9, '10': 'resourceType'},
    {'1': 'usage_kind', '3': 4, '4': 1, '5': 9, '10': 'usageKind'},
  ],
};

/// Descriptor for `RegistryUsageReference`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registryUsageReferenceDescriptor = $convert.base64Decode(
    'ChZSZWdpc3RyeVVzYWdlUmVmZXJlbmNlEh8KC3Jlc291cmNlX2lkGAEgASgJUgpyZXNvdXJjZU'
    'lkEiMKDXJlc291cmNlX25hbWUYAiABKAlSDHJlc291cmNlTmFtZRIjCg1yZXNvdXJjZV90eXBl'
    'GAMgASgJUgxyZXNvdXJjZVR5cGUSHQoKdXNhZ2Vfa2luZBgEIAEoCVIJdXNhZ2VLaW5k');

@$core.Deprecated('Use listRegistriesRequestDescriptor instead')
const ListRegistriesRequest$json = {
  '1': 'ListRegistriesRequest',
};

/// Descriptor for `ListRegistriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistriesRequestDescriptor =
    $convert.base64Decode('ChVMaXN0UmVnaXN0cmllc1JlcXVlc3Q=');

@$core.Deprecated('Use listRegistriesResponseDescriptor instead')
const ListRegistriesResponse$json = {
  '1': 'ListRegistriesResponse',
  '2': [
    {
      '1': 'registries',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Registry',
      '10': 'registries'
    },
  ],
};

/// Descriptor for `ListRegistriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistriesResponseDescriptor =
    $convert.base64Decode(
        'ChZMaXN0UmVnaXN0cmllc1Jlc3BvbnNlEjoKCnJlZ2lzdHJpZXMYASADKAsyGi5vcGVub3hpZG'
        'UuYXBpLnYxLlJlZ2lzdHJ5UgpyZWdpc3RyaWVz');

@$core.Deprecated('Use getRegistryRequestDescriptor instead')
const GetRegistryRequest$json = {
  '1': 'GetRegistryRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetRegistryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRegistryRequestDescriptor =
    $convert.base64Decode(
        'ChJHZXRSZWdpc3RyeVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use getRegistryResponseDescriptor instead')
const GetRegistryResponse$json = {
  '1': 'GetRegistryResponse',
  '2': [
    {
      '1': 'registry',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Registry',
      '10': 'registry'
    },
  ],
};

/// Descriptor for `GetRegistryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRegistryResponseDescriptor = $convert.base64Decode(
    'ChNHZXRSZWdpc3RyeVJlc3BvbnNlEjYKCHJlZ2lzdHJ5GAEgASgLMhoub3Blbm94aWRlLmFwaS'
    '52MS5SZWdpc3RyeVIIcmVnaXN0cnk=');

@$core.Deprecated('Use createRegistryRequestDescriptor instead')
const CreateRegistryRequest$json = {
  '1': 'CreateRegistryRequest',
  '2': [
    {
      '1': 'registry_name',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'registryName'
    },
    {
      '1': 'image_prefix',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'imagePrefix',
      '17': true
    },
    {'1': 'username', '3': 3, '4': 1, '5': 9, '10': 'username'},
    {'1': 'password', '3': 4, '4': 1, '5': 9, '10': 'password'},
    {'1': 'registry_url', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'registryUrl'},
    {
      '1': 'registry_type',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'registryType'
    },
  ],
  '8': [
    {'1': '_image_prefix'},
  ],
};

/// Descriptor for `CreateRegistryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRegistryRequestDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVSZWdpc3RyeVJlcXVlc3QSLwoNcmVnaXN0cnlfbmFtZRgBIAEoCUIKukgHcgUQAR'
    'j/AVIMcmVnaXN0cnlOYW1lEiYKDGltYWdlX3ByZWZpeBgCIAEoCUgAUgtpbWFnZVByZWZpeIgB'
    'ARIaCgh1c2VybmFtZRgDIAEoCVIIdXNlcm5hbWUSGgoIcGFzc3dvcmQYBCABKAlSCHBhc3N3b3'
    'JkEioKDHJlZ2lzdHJ5X3VybBgFIAEoCUIHukgEcgIQAVILcmVnaXN0cnlVcmwSLAoNcmVnaXN0'
    'cnlfdHlwZRgGIAEoCUIHukgEcgIQAVIMcmVnaXN0cnlUeXBlQg8KDV9pbWFnZV9wcmVmaXg=');

@$core.Deprecated('Use createRegistryResponseDescriptor instead')
const CreateRegistryResponse$json = {
  '1': 'CreateRegistryResponse',
  '2': [
    {
      '1': 'registry',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Registry',
      '10': 'registry'
    },
  ],
};

/// Descriptor for `CreateRegistryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRegistryResponseDescriptor =
    $convert.base64Decode(
        'ChZDcmVhdGVSZWdpc3RyeVJlc3BvbnNlEjYKCHJlZ2lzdHJ5GAEgASgLMhoub3Blbm94aWRlLm'
        'FwaS52MS5SZWdpc3RyeVIIcmVnaXN0cnk=');

@$core.Deprecated('Use updateRegistryRequestDescriptor instead')
const UpdateRegistryRequest$json = {
  '1': 'UpdateRegistryRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'registry_name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'registryName',
      '17': true
    },
    {
      '1': 'image_prefix',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'imagePrefix',
      '17': true
    },
    {
      '1': 'username',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'username',
      '17': true
    },
    {
      '1': 'password',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'password',
      '17': true
    },
    {
      '1': 'registry_url',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'registryUrl',
      '17': true
    },
    {
      '1': 'registry_type',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'registryType',
      '17': true
    },
  ],
  '8': [
    {'1': '_registry_name'},
    {'1': '_image_prefix'},
    {'1': '_username'},
    {'1': '_password'},
    {'1': '_registry_url'},
    {'1': '_registry_type'},
  ],
};

/// Descriptor for `UpdateRegistryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRegistryRequestDescriptor = $convert.base64Decode(
    'ChVVcGRhdGVSZWdpc3RyeVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjIKDXJlZ2'
    'lzdHJ5X25hbWUYAiABKAlCCLpIBXIDGP8BSABSDHJlZ2lzdHJ5TmFtZYgBARImCgxpbWFnZV9w'
    'cmVmaXgYAyABKAlIAVILaW1hZ2VQcmVmaXiIAQESHwoIdXNlcm5hbWUYBCABKAlIAlIIdXNlcm'
    '5hbWWIAQESHwoIcGFzc3dvcmQYBSABKAlIA1IIcGFzc3dvcmSIAQESJgoMcmVnaXN0cnlfdXJs'
    'GAYgASgJSARSC3JlZ2lzdHJ5VXJsiAEBEigKDXJlZ2lzdHJ5X3R5cGUYByABKAlIBVIMcmVnaX'
    'N0cnlUeXBliAEBQhAKDl9yZWdpc3RyeV9uYW1lQg8KDV9pbWFnZV9wcmVmaXhCCwoJX3VzZXJu'
    'YW1lQgsKCV9wYXNzd29yZEIPCg1fcmVnaXN0cnlfdXJsQhAKDl9yZWdpc3RyeV90eXBl');

@$core.Deprecated('Use updateRegistryResponseDescriptor instead')
const UpdateRegistryResponse$json = {
  '1': 'UpdateRegistryResponse',
  '2': [
    {
      '1': 'registry',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Registry',
      '10': 'registry'
    },
  ],
};

/// Descriptor for `UpdateRegistryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRegistryResponseDescriptor =
    $convert.base64Decode(
        'ChZVcGRhdGVSZWdpc3RyeVJlc3BvbnNlEjYKCHJlZ2lzdHJ5GAEgASgLMhoub3Blbm94aWRlLm'
        'FwaS52MS5SZWdpc3RyeVIIcmVnaXN0cnk=');

@$core.Deprecated('Use deleteRegistryRequestDescriptor instead')
const DeleteRegistryRequest$json = {
  '1': 'DeleteRegistryRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteRegistryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteRegistryRequestDescriptor =
    $convert.base64Decode(
        'ChVEZWxldGVSZWdpc3RyeVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use testRegistryRequestDescriptor instead')
const TestRegistryRequest$json = {
  '1': 'TestRegistryRequest',
  '2': [
    {
      '1': 'id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'id',
      '17': true
    },
    {'1': 'registry_url', '3': 2, '4': 1, '5': 9, '10': 'registryUrl'},
    {'1': 'username', '3': 3, '4': 1, '5': 9, '10': 'username'},
    {'1': 'password', '3': 4, '4': 1, '5': 9, '10': 'password'},
  ],
  '8': [
    {'1': '_id'},
  ],
};

/// Descriptor for `TestRegistryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testRegistryRequestDescriptor = $convert.base64Decode(
    'ChNUZXN0UmVnaXN0cnlSZXF1ZXN0EhwKAmlkGAEgASgDQge6SAQiAiAASABSAmlkiAEBEiEKDH'
    'JlZ2lzdHJ5X3VybBgCIAEoCVILcmVnaXN0cnlVcmwSGgoIdXNlcm5hbWUYAyABKAlSCHVzZXJu'
    'YW1lEhoKCHBhc3N3b3JkGAQgASgJUghwYXNzd29yZEIFCgNfaWQ=');

@$core.Deprecated('Use testRegistryResponseDescriptor instead')
const TestRegistryResponse$json = {
  '1': 'TestRegistryResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `TestRegistryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testRegistryResponseDescriptor =
    $convert.base64Decode(
        'ChRUZXN0UmVnaXN0cnlSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

@$core.Deprecated('Use listRegistryRepositoriesRequestDescriptor instead')
const ListRegistryRepositoriesRequest$json = {
  '1': 'ListRegistryRepositoriesRequest',
  '2': [
    {'1': 'registry_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'registryId'},
  ],
};

/// Descriptor for `ListRegistryRepositoriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistryRepositoriesRequestDescriptor =
    $convert.base64Decode(
        'Ch9MaXN0UmVnaXN0cnlSZXBvc2l0b3JpZXNSZXF1ZXN0EigKC3JlZ2lzdHJ5X2lkGAEgASgDQg'
        'e6SAQiAiAAUgpyZWdpc3RyeUlk');

@$core.Deprecated('Use listRegistryRepositoriesResponseDescriptor instead')
const ListRegistryRepositoriesResponse$json = {
  '1': 'ListRegistryRepositoriesResponse',
  '2': [
    {'1': 'repositories', '3': 1, '4': 3, '5': 9, '10': 'repositories'},
  ],
};

/// Descriptor for `ListRegistryRepositoriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistryRepositoriesResponseDescriptor =
    $convert.base64Decode(
        'CiBMaXN0UmVnaXN0cnlSZXBvc2l0b3JpZXNSZXNwb25zZRIiCgxyZXBvc2l0b3JpZXMYASADKA'
        'lSDHJlcG9zaXRvcmllcw==');

@$core.Deprecated('Use listRegistryTagsRequestDescriptor instead')
const ListRegistryTagsRequest$json = {
  '1': 'ListRegistryTagsRequest',
  '2': [
    {'1': 'registry_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'registryId'},
    {'1': 'repository', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'repository'},
  ],
};

/// Descriptor for `ListRegistryTagsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistryTagsRequestDescriptor = $convert.base64Decode(
    'ChdMaXN0UmVnaXN0cnlUYWdzUmVxdWVzdBIoCgtyZWdpc3RyeV9pZBgBIAEoA0IHukgEIgIgAF'
    'IKcmVnaXN0cnlJZBInCgpyZXBvc2l0b3J5GAIgASgJQge6SARyAhABUgpyZXBvc2l0b3J5');

@$core.Deprecated('Use listRegistryTagsResponseDescriptor instead')
const ListRegistryTagsResponse$json = {
  '1': 'ListRegistryTagsResponse',
  '2': [
    {'1': 'repository', '3': 1, '4': 1, '5': 9, '10': 'repository'},
    {'1': 'tags', '3': 2, '4': 3, '5': 9, '10': 'tags'},
  ],
};

/// Descriptor for `ListRegistryTagsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRegistryTagsResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0UmVnaXN0cnlUYWdzUmVzcG9uc2USHgoKcmVwb3NpdG9yeRgBIAEoCVIKcmVwb3NpdG'
        '9yeRISCgR0YWdzGAIgAygJUgR0YWdz');

@$core.Deprecated('Use rotateRegistryCredentialsRequestDescriptor instead')
const RotateRegistryCredentialsRequest$json = {
  '1': 'RotateRegistryCredentialsRequest',
  '2': [
    {'1': 'registry_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'registryId'},
    {'1': 'username', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {'1': 'password', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'password'},
  ],
};

/// Descriptor for `RotateRegistryCredentialsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rotateRegistryCredentialsRequestDescriptor =
    $convert.base64Decode(
        'CiBSb3RhdGVSZWdpc3RyeUNyZWRlbnRpYWxzUmVxdWVzdBIoCgtyZWdpc3RyeV9pZBgBIAEoA0'
        'IHukgEIgIgAFIKcmVnaXN0cnlJZBIjCgh1c2VybmFtZRgCIAEoCUIHukgEcgIQAVIIdXNlcm5h'
        'bWUSIwoIcGFzc3dvcmQYAyABKAlCB7pIBHICEAFSCHBhc3N3b3Jk');

@$core.Deprecated('Use rotateRegistryCredentialsResponseDescriptor instead')
const RotateRegistryCredentialsResponse$json = {
  '1': 'RotateRegistryCredentialsResponse',
  '2': [
    {
      '1': 'registry',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Registry',
      '10': 'registry'
    },
  ],
};

/// Descriptor for `RotateRegistryCredentialsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rotateRegistryCredentialsResponseDescriptor =
    $convert.base64Decode(
        'CiFSb3RhdGVSZWdpc3RyeUNyZWRlbnRpYWxzUmVzcG9uc2USNgoIcmVnaXN0cnkYASABKAsyGi'
        '5vcGVub3hpZGUuYXBpLnYxLlJlZ2lzdHJ5UghyZWdpc3RyeQ==');

@$core.Deprecated('Use getRegistryUsageRequestDescriptor instead')
const GetRegistryUsageRequest$json = {
  '1': 'GetRegistryUsageRequest',
  '2': [
    {'1': 'registry_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'registryId'},
  ],
};

/// Descriptor for `GetRegistryUsageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRegistryUsageRequestDescriptor =
    $convert.base64Decode(
        'ChdHZXRSZWdpc3RyeVVzYWdlUmVxdWVzdBIoCgtyZWdpc3RyeV9pZBgBIAEoA0IHukgEIgIgAF'
        'IKcmVnaXN0cnlJZA==');

@$core.Deprecated('Use getRegistryUsageResponseDescriptor instead')
const GetRegistryUsageResponse$json = {
  '1': 'GetRegistryUsageResponse',
  '2': [
    {
      '1': 'usage',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.RegistryUsageReference',
      '10': 'usage'
    },
  ],
};

/// Descriptor for `GetRegistryUsageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRegistryUsageResponseDescriptor =
    $convert.base64Decode(
        'ChhHZXRSZWdpc3RyeVVzYWdlUmVzcG9uc2USPgoFdXNhZ2UYASADKAsyKC5vcGVub3hpZGUuYX'
        'BpLnYxLlJlZ2lzdHJ5VXNhZ2VSZWZlcmVuY2VSBXVzYWdl');

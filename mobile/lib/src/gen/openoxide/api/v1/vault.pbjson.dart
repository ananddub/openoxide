// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/vault.proto.

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

@$core.Deprecated('Use vaultProviderTypeDescriptor instead')
const VaultProviderType$json = {
  '1': 'VaultProviderType',
  '2': [
    {'1': 'VAULT_PROVIDER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'VAULT_PROVIDER_TYPE_HASHICORP', '2': 1},
    {'1': 'VAULT_PROVIDER_TYPE_INFISICAL', '2': 2},
    {'1': 'VAULT_PROVIDER_TYPE_DOPPLER', '2': 3},
    {'1': 'VAULT_PROVIDER_TYPE_AWS', '2': 4},
    {'1': 'VAULT_PROVIDER_TYPE_AWS_PARAMETER_STORE', '2': 5},
    {'1': 'VAULT_PROVIDER_TYPE_SCALEWAY', '2': 6},
    {'1': 'VAULT_PROVIDER_TYPE_AZURE', '2': 7},
  ],
};

/// Descriptor for `VaultProviderType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List vaultProviderTypeDescriptor = $convert.base64Decode(
    'ChFWYXVsdFByb3ZpZGVyVHlwZRIjCh9WQVVMVF9QUk9WSURFUl9UWVBFX1VOU1BFQ0lGSUVEEA'
    'ASIQodVkFVTFRfUFJPVklERVJfVFlQRV9IQVNISUNPUlAQARIhCh1WQVVMVF9QUk9WSURFUl9U'
    'WVBFX0lORklTSUNBTBACEh8KG1ZBVUxUX1BST1ZJREVSX1RZUEVfRE9QUExFUhADEhsKF1ZBVU'
    'xUX1BST1ZJREVSX1RZUEVfQVdTEAQSKwonVkFVTFRfUFJPVklERVJfVFlQRV9BV1NfUEFSQU1F'
    'VEVSX1NUT1JFEAUSIAocVkFVTFRfUFJPVklERVJfVFlQRV9TQ0FMRVdBWRAGEh0KGVZBVUxUX1'
    'BST1ZJREVSX1RZUEVfQVpVUkUQBw==');

@$core.Deprecated('Use vaultProviderDescriptor instead')
const VaultProvider$json = {
  '1': 'VaultProvider',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'provider_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.VaultProviderType',
      '10': 'providerType'
    },
    {'1': 'api_url', '3': 4, '4': 1, '5': 9, '10': 'apiUrl'},
    {'1': 'auth_token', '3': 5, '4': 1, '5': 9, '10': 'authToken'},
    {
      '1': 'namespace',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'namespace',
      '17': true
    },
    {
      '1': 'config_json',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'configJson',
      '17': true
    },
    {'1': 'organization_id', '3': 8, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'assignments',
      '3': 11,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
  '8': [
    {'1': '_namespace'},
    {'1': '_config_json'},
  ],
};

/// Descriptor for `VaultProvider`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List vaultProviderDescriptor = $convert.base64Decode(
    'Cg1WYXVsdFByb3ZpZGVyEg4KAmlkGAEgASgDUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEkgKDX'
    'Byb3ZpZGVyX3R5cGUYAyABKA4yIy5vcGVub3hpZGUuYXBpLnYxLlZhdWx0UHJvdmlkZXJUeXBl'
    'Ugxwcm92aWRlclR5cGUSFwoHYXBpX3VybBgEIAEoCVIGYXBpVXJsEh0KCmF1dGhfdG9rZW4YBS'
    'ABKAlSCWF1dGhUb2tlbhIhCgluYW1lc3BhY2UYBiABKAlIAFIJbmFtZXNwYWNliAEBEiQKC2Nv'
    'bmZpZ19qc29uGAcgASgJSAFSCmNvbmZpZ0pzb26IAQESJwoPb3JnYW5pemF0aW9uX2lkGAggAS'
    'gDUg5vcmdhbml6YXRpb25JZBIdCgpjcmVhdGVkX2F0GAkgASgDUgljcmVhdGVkQXQSHQoKdXBk'
    'YXRlZF9hdBgKIAEoA1IJdXBkYXRlZEF0EksKC2Fzc2lnbm1lbnRzGAsgAygLMikub3Blbm94aW'
    'RlLmFwaS52MS5WYXVsdFByb3ZpZGVyQXNzaWdubWVudFILYXNzaWdubWVudHNCDAoKX25hbWVz'
    'cGFjZUIOCgxfY29uZmlnX2pzb24=');

@$core.Deprecated('Use vaultProviderAssignmentDescriptor instead')
const VaultProviderAssignment$json = {
  '1': 'VaultProviderAssignment',
  '2': [
    {'1': 'project_id', '3': 1, '4': 1, '5': 3, '10': 'projectId'},
    {'1': 'environment_ids', '3': 2, '4': 3, '5': 3, '10': 'environmentIds'},
  ],
};

/// Descriptor for `VaultProviderAssignment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List vaultProviderAssignmentDescriptor =
    $convert.base64Decode(
        'ChdWYXVsdFByb3ZpZGVyQXNzaWdubWVudBIdCgpwcm9qZWN0X2lkGAEgASgDUglwcm9qZWN0SW'
        'QSJwoPZW52aXJvbm1lbnRfaWRzGAIgAygDUg5lbnZpcm9ubWVudElkcw==');

@$core.Deprecated('Use listVaultProvidersRequestDescriptor instead')
const ListVaultProvidersRequest$json = {
  '1': 'ListVaultProvidersRequest',
};

/// Descriptor for `ListVaultProvidersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultProvidersRequestDescriptor =
    $convert.base64Decode('ChlMaXN0VmF1bHRQcm92aWRlcnNSZXF1ZXN0');

@$core.Deprecated('Use listVaultProvidersResponseDescriptor instead')
const ListVaultProvidersResponse$json = {
  '1': 'ListVaultProvidersResponse',
  '2': [
    {
      '1': 'providers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProvider',
      '10': 'providers'
    },
  ],
};

/// Descriptor for `ListVaultProvidersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultProvidersResponseDescriptor =
    $convert.base64Decode(
        'ChpMaXN0VmF1bHRQcm92aWRlcnNSZXNwb25zZRI9Cglwcm92aWRlcnMYASADKAsyHy5vcGVub3'
        'hpZGUuYXBpLnYxLlZhdWx0UHJvdmlkZXJSCXByb3ZpZGVycw==');

@$core.Deprecated('Use getVaultProviderRequestDescriptor instead')
const GetVaultProviderRequest$json = {
  '1': 'GetVaultProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetVaultProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getVaultProviderRequestDescriptor =
    $convert.base64Decode(
        'ChdHZXRWYXVsdFByb3ZpZGVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use getVaultProviderResponseDescriptor instead')
const GetVaultProviderResponse$json = {
  '1': 'GetVaultProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `GetVaultProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getVaultProviderResponseDescriptor =
    $convert.base64Decode(
        'ChhHZXRWYXVsdFByb3ZpZGVyUmVzcG9uc2USOwoIcHJvdmlkZXIYASABKAsyHy5vcGVub3hpZG'
        'UuYXBpLnYxLlZhdWx0UHJvdmlkZXJSCHByb3ZpZGVy');

@$core.Deprecated('Use createVaultProviderRequestDescriptor instead')
const CreateVaultProviderRequest$json = {
  '1': 'CreateVaultProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'provider_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.VaultProviderType',
      '8': {},
      '10': 'providerType'
    },
    {'1': 'api_url', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'apiUrl'},
    {'1': 'auth_token', '3': 4, '4': 1, '5': 9, '10': 'authToken'},
    {
      '1': 'namespace',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'namespace',
      '17': true
    },
    {
      '1': 'config_json',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'configJson',
      '17': true
    },
    {
      '1': 'assignments',
      '3': 7,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
  '8': [
    {'1': '_namespace'},
    {'1': '_config_json'},
  ],
};

/// Descriptor for `CreateVaultProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createVaultProviderRequestDescriptor = $convert.base64Decode(
    'ChpDcmVhdGVWYXVsdFByb3ZpZGVyUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP8BUg'
    'RuYW1lElQKDXByb3ZpZGVyX3R5cGUYAiABKA4yIy5vcGVub3hpZGUuYXBpLnYxLlZhdWx0UHJv'
    'dmlkZXJUeXBlQgq6SAeCAQQQASAAUgxwcm92aWRlclR5cGUSIAoHYXBpX3VybBgDIAEoCUIHuk'
    'gEcgIQAVIGYXBpVXJsEh0KCmF1dGhfdG9rZW4YBCABKAlSCWF1dGhUb2tlbhIhCgluYW1lc3Bh'
    'Y2UYBSABKAlIAFIJbmFtZXNwYWNliAEBEi8KC2NvbmZpZ19qc29uGAYgASgJQgm6SAZyBBiAgE'
    'BIAVIKY29uZmlnSnNvbogBARJLCgthc3NpZ25tZW50cxgHIAMoCzIpLm9wZW5veGlkZS5hcGku'
    'djEuVmF1bHRQcm92aWRlckFzc2lnbm1lbnRSC2Fzc2lnbm1lbnRzQgwKCl9uYW1lc3BhY2VCDg'
    'oMX2NvbmZpZ19qc29u');

@$core.Deprecated('Use createVaultProviderResponseDescriptor instead')
const CreateVaultProviderResponse$json = {
  '1': 'CreateVaultProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `CreateVaultProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createVaultProviderResponseDescriptor =
    $convert.base64Decode(
        'ChtDcmVhdGVWYXVsdFByb3ZpZGVyUmVzcG9uc2USOwoIcHJvdmlkZXIYASABKAsyHy5vcGVub3'
        'hpZGUuYXBpLnYxLlZhdWx0UHJvdmlkZXJSCHByb3ZpZGVy');

@$core.Deprecated('Use updateVaultProviderRequestDescriptor instead')
const UpdateVaultProviderRequest$json = {
  '1': 'UpdateVaultProviderRequest',
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
      '1': 'api_url',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'apiUrl',
      '17': true
    },
    {
      '1': 'auth_token',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'authToken',
      '17': true
    },
    {
      '1': 'namespace',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'namespace',
      '17': true
    },
    {
      '1': 'config_json',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'configJson',
      '17': true
    },
    {
      '1': 'assignments',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.VaultAssignmentsUpdate',
      '9': 5,
      '10': 'assignments',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_api_url'},
    {'1': '_auth_token'},
    {'1': '_namespace'},
    {'1': '_config_json'},
    {'1': '_assignments'},
  ],
};

/// Descriptor for `UpdateVaultProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVaultProviderRequestDescriptor = $convert.base64Decode(
    'ChpVcGRhdGVWYXVsdFByb3ZpZGVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSIQ'
    'oEbmFtZRgCIAEoCUIIukgFcgMY/wFIAFIEbmFtZYgBARIcCgdhcGlfdXJsGAMgASgJSAFSBmFw'
    'aVVybIgBARIiCgphdXRoX3Rva2VuGAQgASgJSAJSCWF1dGhUb2tlbogBARIhCgluYW1lc3BhY2'
    'UYBSABKAlIA1IJbmFtZXNwYWNliAEBEi8KC2NvbmZpZ19qc29uGAYgASgJQgm6SAZyBBiAgEBI'
    'BFIKY29uZmlnSnNvbogBARJPCgthc3NpZ25tZW50cxgHIAEoCzIoLm9wZW5veGlkZS5hcGkudj'
    'EuVmF1bHRBc3NpZ25tZW50c1VwZGF0ZUgFUgthc3NpZ25tZW50c4gBAUIHCgVfbmFtZUIKCghf'
    'YXBpX3VybEINCgtfYXV0aF90b2tlbkIMCgpfbmFtZXNwYWNlQg4KDF9jb25maWdfanNvbkIOCg'
    'xfYXNzaWdubWVudHM=');

@$core.Deprecated('Use vaultAssignmentsUpdateDescriptor instead')
const VaultAssignmentsUpdate$json = {
  '1': 'VaultAssignmentsUpdate',
  '2': [
    {
      '1': 'assignments',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
};

/// Descriptor for `VaultAssignmentsUpdate`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List vaultAssignmentsUpdateDescriptor =
    $convert.base64Decode(
        'ChZWYXVsdEFzc2lnbm1lbnRzVXBkYXRlEksKC2Fzc2lnbm1lbnRzGAEgAygLMikub3Blbm94aW'
        'RlLmFwaS52MS5WYXVsdFByb3ZpZGVyQXNzaWdubWVudFILYXNzaWdubWVudHM=');

@$core.Deprecated('Use updateVaultProviderResponseDescriptor instead')
const UpdateVaultProviderResponse$json = {
  '1': 'UpdateVaultProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateVaultProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVaultProviderResponseDescriptor =
    $convert.base64Decode(
        'ChtVcGRhdGVWYXVsdFByb3ZpZGVyUmVzcG9uc2USOwoIcHJvdmlkZXIYASABKAsyHy5vcGVub3'
        'hpZGUuYXBpLnYxLlZhdWx0UHJvdmlkZXJSCHByb3ZpZGVy');

@$core.Deprecated('Use deleteVaultProviderRequestDescriptor instead')
const DeleteVaultProviderRequest$json = {
  '1': 'DeleteVaultProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteVaultProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteVaultProviderRequestDescriptor =
    $convert.base64Decode(
        'ChpEZWxldGVWYXVsdFByb3ZpZGVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use testVaultProviderRequestDescriptor instead')
const TestVaultProviderRequest$json = {
  '1': 'TestVaultProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `TestVaultProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testVaultProviderRequestDescriptor =
    $convert.base64Decode(
        'ChhUZXN0VmF1bHRQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use testVaultProviderResponseDescriptor instead')
const TestVaultProviderResponse$json = {
  '1': 'TestVaultProviderResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestVaultProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testVaultProviderResponseDescriptor =
    $convert.base64Decode(
        'ChlUZXN0VmF1bHRQcm92aWRlclJlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSGA'
        'oHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');

@$core.Deprecated('Use testVaultConnectionRequestDescriptor instead')
const TestVaultConnectionRequest$json = {
  '1': 'TestVaultConnectionRequest',
  '2': [
    {
      '1': 'provider_type',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.VaultProviderType',
      '8': {},
      '10': 'providerType'
    },
    {'1': 'api_url', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'apiUrl'},
    {'1': 'auth_token', '3': 3, '4': 1, '5': 9, '10': 'authToken'},
    {
      '1': 'namespace',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'namespace',
      '17': true
    },
    {
      '1': 'config_json',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'configJson',
      '17': true
    },
  ],
  '8': [
    {'1': '_namespace'},
    {'1': '_config_json'},
  ],
};

/// Descriptor for `TestVaultConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testVaultConnectionRequestDescriptor = $convert.base64Decode(
    'ChpUZXN0VmF1bHRDb25uZWN0aW9uUmVxdWVzdBJUCg1wcm92aWRlcl90eXBlGAEgASgOMiMub3'
    'Blbm94aWRlLmFwaS52MS5WYXVsdFByb3ZpZGVyVHlwZUIKukgHggEEEAEgAFIMcHJvdmlkZXJU'
    'eXBlEiAKB2FwaV91cmwYAiABKAlCB7pIBHICEAFSBmFwaVVybBIdCgphdXRoX3Rva2VuGAMgAS'
    'gJUglhdXRoVG9rZW4SIQoJbmFtZXNwYWNlGAQgASgJSABSCW5hbWVzcGFjZYgBARIvCgtjb25m'
    'aWdfanNvbhgFIAEoCUIJukgGcgQYgIBASAFSCmNvbmZpZ0pzb26IAQFCDAoKX25hbWVzcGFjZU'
    'IOCgxfY29uZmlnX2pzb24=');

@$core.Deprecated('Use testVaultConnectionResponseDescriptor instead')
const TestVaultConnectionResponse$json = {
  '1': 'TestVaultConnectionResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestVaultConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testVaultConnectionResponseDescriptor =
    $convert.base64Decode(
        'ChtUZXN0VmF1bHRDb25uZWN0aW9uUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2Vzcx'
        'IYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdl');

@$core.Deprecated('Use listVaultSecretsRequestDescriptor instead')
const ListVaultSecretsRequest$json = {
  '1': 'ListVaultSecretsRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `ListVaultSecretsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultSecretsRequestDescriptor =
    $convert.base64Decode(
        'ChdMaXN0VmF1bHRTZWNyZXRzUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIgIgAF'
        'IKcHJvdmlkZXJJZA==');

@$core.Deprecated('Use listVaultSecretsResponseDescriptor instead')
const ListVaultSecretsResponse$json = {
  '1': 'ListVaultSecretsResponse',
  '2': [
    {'1': 'secrets', '3': 1, '4': 3, '5': 9, '10': 'secrets'},
  ],
};

/// Descriptor for `ListVaultSecretsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultSecretsResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0VmF1bHRTZWNyZXRzUmVzcG9uc2USGAoHc2VjcmV0cxgBIAMoCVIHc2VjcmV0cw==');

@$core.Deprecated('Use listVaultAssignmentsRequestDescriptor instead')
const ListVaultAssignmentsRequest$json = {
  '1': 'ListVaultAssignmentsRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `ListVaultAssignmentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultAssignmentsRequestDescriptor =
    $convert.base64Decode(
        'ChtMaXN0VmF1bHRBc3NpZ25tZW50c1JlcXVlc3QSKAoLcHJvdmlkZXJfaWQYASABKANCB7pIBC'
        'ICIABSCnByb3ZpZGVySWQ=');

@$core.Deprecated('Use listVaultAssignmentsResponseDescriptor instead')
const ListVaultAssignmentsResponse$json = {
  '1': 'ListVaultAssignmentsResponse',
  '2': [
    {
      '1': 'assignments',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
};

/// Descriptor for `ListVaultAssignmentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVaultAssignmentsResponseDescriptor =
    $convert.base64Decode(
        'ChxMaXN0VmF1bHRBc3NpZ25tZW50c1Jlc3BvbnNlEksKC2Fzc2lnbm1lbnRzGAEgAygLMikub3'
        'Blbm94aWRlLmFwaS52MS5WYXVsdFByb3ZpZGVyQXNzaWdubWVudFILYXNzaWdubWVudHM=');

@$core.Deprecated('Use updateVaultAssignmentsRequestDescriptor instead')
const UpdateVaultAssignmentsRequest$json = {
  '1': 'UpdateVaultAssignmentsRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {
      '1': 'assignments',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
};

/// Descriptor for `UpdateVaultAssignmentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVaultAssignmentsRequestDescriptor =
    $convert.base64Decode(
        'Ch1VcGRhdGVWYXVsdEFzc2lnbm1lbnRzUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHuk'
        'gEIgIgAFIKcHJvdmlkZXJJZBJLCgthc3NpZ25tZW50cxgCIAMoCzIpLm9wZW5veGlkZS5hcGku'
        'djEuVmF1bHRQcm92aWRlckFzc2lnbm1lbnRSC2Fzc2lnbm1lbnRz');

@$core.Deprecated('Use updateVaultAssignmentsResponseDescriptor instead')
const UpdateVaultAssignmentsResponse$json = {
  '1': 'UpdateVaultAssignmentsResponse',
  '2': [
    {
      '1': 'assignments',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.VaultProviderAssignment',
      '10': 'assignments'
    },
  ],
};

/// Descriptor for `UpdateVaultAssignmentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVaultAssignmentsResponseDescriptor =
    $convert.base64Decode(
        'Ch5VcGRhdGVWYXVsdEFzc2lnbm1lbnRzUmVzcG9uc2USSwoLYXNzaWdubWVudHMYASADKAsyKS'
        '5vcGVub3hpZGUuYXBpLnYxLlZhdWx0UHJvdmlkZXJBc3NpZ25tZW50Ugthc3NpZ25tZW50cw==');

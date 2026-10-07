// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/git_provider.proto.

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

@$core.Deprecated('Use gitProviderTypeDescriptor instead')
const GitProviderType$json = {
  '1': 'GitProviderType',
  '2': [
    {'1': 'GIT_PROVIDER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'GIT_PROVIDER_TYPE_GITHUB', '2': 1},
    {'1': 'GIT_PROVIDER_TYPE_GITLAB', '2': 2},
    {'1': 'GIT_PROVIDER_TYPE_GITEA', '2': 3},
    {'1': 'GIT_PROVIDER_TYPE_BITBUCKET', '2': 4},
  ],
};

/// Descriptor for `GitProviderType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List gitProviderTypeDescriptor = $convert.base64Decode(
    'Cg9HaXRQcm92aWRlclR5cGUSIQodR0lUX1BST1ZJREVSX1RZUEVfVU5TUEVDSUZJRUQQABIcCh'
    'hHSVRfUFJPVklERVJfVFlQRV9HSVRIVUIQARIcChhHSVRfUFJPVklERVJfVFlQRV9HSVRMQUIQ'
    'AhIbChdHSVRfUFJPVklERVJfVFlQRV9HSVRFQRADEh8KG0dJVF9QUk9WSURFUl9UWVBFX0JJVE'
    'JVQ0tFVBAE');

@$core.Deprecated('Use gitProviderConfigDescriptor instead')
const GitProviderConfig$json = {
  '1': 'GitProviderConfig',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'url', '17': true},
    {
      '1': 'internal_url',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'internalUrl',
      '17': true
    },
    {
      '1': 'app_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'appName',
      '17': true
    },
    {'1': 'app_id', '3': 4, '4': 1, '5': 3, '9': 3, '10': 'appId', '17': true},
    {
      '1': 'client_id',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'clientId',
      '17': true
    },
    {
      '1': 'installation_id',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'installationId',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'redirect_uri',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'redirectUri',
      '17': true
    },
    {
      '1': 'group_name',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'groupName',
      '17': true
    },
    {
      '1': 'scopes',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'scopes',
      '17': true
    },
    {
      '1': 'username',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'username',
      '17': true
    },
    {'1': 'email', '3': 12, '4': 1, '5': 9, '9': 11, '10': 'email', '17': true},
    {
      '1': 'workspace',
      '3': 13,
      '4': 1,
      '5': 9,
      '9': 12,
      '10': 'workspace',
      '17': true
    },
    {
      '1': 'has_client_secret',
      '3': 14,
      '4': 1,
      '5': 8,
      '10': 'hasClientSecret'
    },
    {'1': 'has_private_key', '3': 15, '4': 1, '5': 8, '10': 'hasPrivateKey'},
    {'1': 'has_access_token', '3': 16, '4': 1, '5': 8, '10': 'hasAccessToken'},
    {'1': 'has_app_password', '3': 17, '4': 1, '5': 8, '10': 'hasAppPassword'},
    {'1': 'has_api_token', '3': 18, '4': 1, '5': 8, '10': 'hasApiToken'},
  ],
  '8': [
    {'1': '_url'},
    {'1': '_internal_url'},
    {'1': '_app_name'},
    {'1': '_app_id'},
    {'1': '_client_id'},
    {'1': '_installation_id'},
    {'1': '_application_id'},
    {'1': '_redirect_uri'},
    {'1': '_group_name'},
    {'1': '_scopes'},
    {'1': '_username'},
    {'1': '_email'},
    {'1': '_workspace'},
  ],
};

/// Descriptor for `GitProviderConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderConfigDescriptor = $convert.base64Decode(
    'ChFHaXRQcm92aWRlckNvbmZpZxIVCgN1cmwYASABKAlIAFIDdXJsiAEBEiYKDGludGVybmFsX3'
    'VybBgCIAEoCUgBUgtpbnRlcm5hbFVybIgBARIeCghhcHBfbmFtZRgDIAEoCUgCUgdhcHBOYW1l'
    'iAEBEhoKBmFwcF9pZBgEIAEoA0gDUgVhcHBJZIgBARIgCgljbGllbnRfaWQYBSABKAlIBFIIY2'
    'xpZW50SWSIAQESLAoPaW5zdGFsbGF0aW9uX2lkGAYgASgJSAVSDmluc3RhbGxhdGlvbklkiAEB'
    'EioKDmFwcGxpY2F0aW9uX2lkGAcgASgJSAZSDWFwcGxpY2F0aW9uSWSIAQESJgoMcmVkaXJlY3'
    'RfdXJpGAggASgJSAdSC3JlZGlyZWN0VXJpiAEBEiIKCmdyb3VwX25hbWUYCSABKAlICFIJZ3Jv'
    'dXBOYW1liAEBEhsKBnNjb3BlcxgKIAEoCUgJUgZzY29wZXOIAQESHwoIdXNlcm5hbWUYCyABKA'
    'lIClIIdXNlcm5hbWWIAQESGQoFZW1haWwYDCABKAlIC1IFZW1haWyIAQESIQoJd29ya3NwYWNl'
    'GA0gASgJSAxSCXdvcmtzcGFjZYgBARIqChFoYXNfY2xpZW50X3NlY3JldBgOIAEoCFIPaGFzQ2'
    'xpZW50U2VjcmV0EiYKD2hhc19wcml2YXRlX2tleRgPIAEoCFINaGFzUHJpdmF0ZUtleRIoChBo'
    'YXNfYWNjZXNzX3Rva2VuGBAgASgIUg5oYXNBY2Nlc3NUb2tlbhIoChBoYXNfYXBwX3Bhc3N3b3'
    'JkGBEgASgIUg5oYXNBcHBQYXNzd29yZBIiCg1oYXNfYXBpX3Rva2VuGBIgASgIUgtoYXNBcGlU'
    'b2tlbkIGCgRfdXJsQg8KDV9pbnRlcm5hbF91cmxCCwoJX2FwcF9uYW1lQgkKB19hcHBfaWRCDA'
    'oKX2NsaWVudF9pZEISChBfaW5zdGFsbGF0aW9uX2lkQhEKD19hcHBsaWNhdGlvbl9pZEIPCg1f'
    'cmVkaXJlY3RfdXJpQg0KC19ncm91cF9uYW1lQgkKB19zY29wZXNCCwoJX3VzZXJuYW1lQggKBl'
    '9lbWFpbEIMCgpfd29ya3NwYWNl');

@$core.Deprecated('Use gitProviderDescriptor instead')
const GitProvider$json = {
  '1': 'GitProvider',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'provider_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.GitProviderType',
      '10': 'providerType'
    },
    {'1': 'shared', '3': 4, '4': 1, '5': 8, '10': 'shared'},
    {'1': 'configured', '3': 5, '4': 1, '5': 8, '10': 'configured'},
    {
      '1': 'webhook_configured',
      '3': 6,
      '4': 1,
      '5': 8,
      '10': 'webhookConfigured'
    },
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'config',
      '3': 9,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderConfig',
      '10': 'config'
    },
  ],
};

/// Descriptor for `GitProvider`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderDescriptor = $convert.base64Decode(
    'CgtHaXRQcm92aWRlchIOCgJpZBgBIAEoA1ICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRJGCg1wcm'
    '92aWRlcl90eXBlGAMgASgOMiEub3Blbm94aWRlLmFwaS52MS5HaXRQcm92aWRlclR5cGVSDHBy'
    'b3ZpZGVyVHlwZRIWCgZzaGFyZWQYBCABKAhSBnNoYXJlZBIeCgpjb25maWd1cmVkGAUgASgIUg'
    'pjb25maWd1cmVkEi0KEndlYmhvb2tfY29uZmlndXJlZBgGIAEoCFIRd2ViaG9va0NvbmZpZ3Vy'
    'ZWQSHQoKY3JlYXRlZF9hdBgHIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYCCABKANSCX'
    'VwZGF0ZWRBdBI7CgZjb25maWcYCSABKAsyIy5vcGVub3hpZGUuYXBpLnYxLkdpdFByb3ZpZGVy'
    'Q29uZmlnUgZjb25maWc=');

@$core.Deprecated('Use gitProviderRepositoryDescriptor instead')
const GitProviderRepository$json = {
  '1': 'GitProviderRepository',
  '2': [
    {'1': 'owner', '3': 1, '4': 1, '5': 9, '10': 'owner'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'full_name', '3': 3, '4': 1, '5': 9, '10': 'fullName'},
    {'1': 'private', '3': 4, '4': 1, '5': 8, '10': 'private'},
    {
      '1': 'default_branch',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'defaultBranch',
      '17': true
    },
    {
      '1': 'clone_url',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'cloneUrl',
      '17': true
    },
  ],
  '8': [
    {'1': '_default_branch'},
    {'1': '_clone_url'},
  ],
};

/// Descriptor for `GitProviderRepository`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderRepositoryDescriptor = $convert.base64Decode(
    'ChVHaXRQcm92aWRlclJlcG9zaXRvcnkSFAoFb3duZXIYASABKAlSBW93bmVyEhIKBG5hbWUYAi'
    'ABKAlSBG5hbWUSGwoJZnVsbF9uYW1lGAMgASgJUghmdWxsTmFtZRIYCgdwcml2YXRlGAQgASgI'
    'Ugdwcml2YXRlEioKDmRlZmF1bHRfYnJhbmNoGAUgASgJSABSDWRlZmF1bHRCcmFuY2iIAQESIA'
    'oJY2xvbmVfdXJsGAYgASgJSAFSCGNsb25lVXJsiAEBQhEKD19kZWZhdWx0X2JyYW5jaEIMCgpf'
    'Y2xvbmVfdXJs');

@$core.Deprecated('Use gitProviderReferenceDescriptor instead')
const GitProviderReference$json = {
  '1': 'GitProviderReference',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'commit', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'commit', '17': true},
  ],
  '8': [
    {'1': '_commit'},
  ],
};

/// Descriptor for `GitProviderReference`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderReferenceDescriptor = $convert.base64Decode(
    'ChRHaXRQcm92aWRlclJlZmVyZW5jZRISCgRuYW1lGAEgASgJUgRuYW1lEhsKBmNvbW1pdBgCIA'
    'EoCUgAUgZjb21taXSIAQFCCQoHX2NvbW1pdA==');

@$core.Deprecated('Use gitProviderWebhookInfoDescriptor instead')
const GitProviderWebhookInfo$json = {
  '1': 'GitProviderWebhookInfo',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'url', '3': 2, '4': 1, '5': 9, '10': 'url'},
    {'1': 'active', '3': 3, '4': 1, '5': 8, '10': 'active'},
  ],
};

/// Descriptor for `GitProviderWebhookInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderWebhookInfoDescriptor =
    $convert.base64Decode(
        'ChZHaXRQcm92aWRlcldlYmhvb2tJbmZvEg4KAmlkGAEgASgJUgJpZBIQCgN1cmwYAiABKAlSA3'
        'VybBIWCgZhY3RpdmUYAyABKAhSBmFjdGl2ZQ==');

@$core.Deprecated('Use gitProviderCollaboratorPermissionDescriptor instead')
const GitProviderCollaboratorPermission$json = {
  '1': 'GitProviderCollaboratorPermission',
  '2': [
    {'1': 'username', '3': 1, '4': 1, '5': 9, '10': 'username'},
    {'1': 'level', '3': 2, '4': 1, '5': 9, '10': 'level'},
    {'1': 'can_write', '3': 3, '4': 1, '5': 8, '10': 'canWrite'},
    {'1': 'can_admin', '3': 4, '4': 1, '5': 8, '10': 'canAdmin'},
  ],
};

/// Descriptor for `GitProviderCollaboratorPermission`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitProviderCollaboratorPermissionDescriptor =
    $convert.base64Decode(
        'CiFHaXRQcm92aWRlckNvbGxhYm9yYXRvclBlcm1pc3Npb24SGgoIdXNlcm5hbWUYASABKAlSCH'
        'VzZXJuYW1lEhQKBWxldmVsGAIgASgJUgVsZXZlbBIbCgljYW5fd3JpdGUYAyABKAhSCGNhbldy'
        'aXRlEhsKCWNhbl9hZG1pbhgEIAEoCFIIY2FuQWRtaW4=');

@$core.Deprecated('Use listGitProvidersRequestDescriptor instead')
const ListGitProvidersRequest$json = {
  '1': 'ListGitProvidersRequest',
};

/// Descriptor for `ListGitProvidersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listGitProvidersRequestDescriptor =
    $convert.base64Decode('ChdMaXN0R2l0UHJvdmlkZXJzUmVxdWVzdA==');

@$core.Deprecated('Use listGitProvidersResponseDescriptor instead')
const ListGitProvidersResponse$json = {
  '1': 'ListGitProvidersResponse',
  '2': [
    {
      '1': 'providers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'providers'
    },
  ],
};

/// Descriptor for `ListGitProvidersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listGitProvidersResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0R2l0UHJvdmlkZXJzUmVzcG9uc2USOwoJcHJvdmlkZXJzGAEgAygLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5HaXRQcm92aWRlclIJcHJvdmlkZXJz');

@$core.Deprecated('Use getGitProviderRequestDescriptor instead')
const GetGitProviderRequest$json = {
  '1': 'GetGitProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetGitProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGitProviderRequestDescriptor =
    $convert.base64Decode(
        'ChVHZXRHaXRQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use getGitProviderResponseDescriptor instead')
const GetGitProviderResponse$json = {
  '1': 'GetGitProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `GetGitProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGitProviderResponseDescriptor =
    $convert.base64Decode(
        'ChZHZXRHaXRQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm94aWRlLm'
        'FwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use createGithubProviderRequestDescriptor instead')
const CreateGithubProviderRequest$json = {
  '1': 'CreateGithubProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 2, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'app_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'appName',
      '17': true
    },
    {'1': 'app_id', '3': 4, '4': 1, '5': 3, '9': 2, '10': 'appId', '17': true},
    {
      '1': 'client_id',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'clientId',
      '17': true
    },
    {
      '1': 'client_secret',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'clientSecret',
      '17': true
    },
    {
      '1': 'installation_id',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'installationId',
      '17': true
    },
    {
      '1': 'private_key',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'privateKey',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_app_name'},
    {'1': '_app_id'},
    {'1': '_client_id'},
    {'1': '_client_secret'},
    {'1': '_installation_id'},
    {'1': '_private_key'},
  ],
};

/// Descriptor for `CreateGithubProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGithubProviderRequestDescriptor = $convert.base64Decode(
    'ChtDcmVhdGVHaXRodWJQcm92aWRlclJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AV'
    'IEbmFtZRIbCgZzaGFyZWQYAiABKAhIAFIGc2hhcmVkiAEBEigKCGFwcF9uYW1lGAMgASgJQgi6'
    'SAVyAxj/AUgBUgdhcHBOYW1liAEBEhoKBmFwcF9pZBgEIAEoA0gCUgVhcHBJZIgBARIqCgljbG'
    'llbnRfaWQYBSABKAlCCLpIBXIDGP8BSANSCGNsaWVudElkiAEBEjMKDWNsaWVudF9zZWNyZXQY'
    'BiABKAlCCbpIBnIEGICAQEgEUgxjbGllbnRTZWNyZXSIAQESNgoPaW5zdGFsbGF0aW9uX2lkGA'
    'cgASgJQgi6SAVyAxj/AUgFUg5pbnN0YWxsYXRpb25JZIgBARIvCgtwcml2YXRlX2tleRgIIAEo'
    'CUIJukgGcgQYgIBASAZSCnByaXZhdGVLZXmIAQFCCQoHX3NoYXJlZEILCglfYXBwX25hbWVCCQ'
    'oHX2FwcF9pZEIMCgpfY2xpZW50X2lkQhAKDl9jbGllbnRfc2VjcmV0QhIKEF9pbnN0YWxsYXRp'
    'b25faWRCDgoMX3ByaXZhdGVfa2V5');

@$core.Deprecated('Use createGithubProviderResponseDescriptor instead')
const CreateGithubProviderResponse$json = {
  '1': 'CreateGithubProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
    {'1': 'webhook_secret', '3': 2, '4': 1, '5': 9, '10': 'webhookSecret'},
  ],
};

/// Descriptor for `CreateGithubProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGithubProviderResponseDescriptor =
    $convert.base64Decode(
        'ChxDcmVhdGVHaXRodWJQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXISJQoOd2ViaG9va19zZWNyZXQYAiAB'
        'KAlSDXdlYmhvb2tTZWNyZXQ=');

@$core.Deprecated('Use createGitlabProviderRequestDescriptor instead')
const CreateGitlabProviderRequest$json = {
  '1': 'CreateGitlabProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 2, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {'1': 'url', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'url'},
    {
      '1': 'internal_url',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'internalUrl',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'redirect_uri',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'redirectUri',
      '17': true
    },
    {
      '1': 'secret',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'secret',
      '17': true
    },
    {
      '1': 'access_token',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'accessToken',
      '17': true
    },
    {
      '1': 'refresh_token',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'refreshToken',
      '17': true
    },
    {
      '1': 'group_name',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'groupName',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_internal_url'},
    {'1': '_application_id'},
    {'1': '_redirect_uri'},
    {'1': '_secret'},
    {'1': '_access_token'},
    {'1': '_refresh_token'},
    {'1': '_group_name'},
  ],
};

/// Descriptor for `CreateGitlabProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGitlabProviderRequestDescriptor = $convert.base64Decode(
    'ChtDcmVhdGVHaXRsYWJQcm92aWRlclJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AV'
    'IEbmFtZRIbCgZzaGFyZWQYAiABKAhIAFIGc2hhcmVkiAEBEhwKA3VybBgDIAEoCUIKukgHcgUQ'
    'ARiAEFIDdXJsEjAKDGludGVybmFsX3VybBgEIAEoCUIIukgFcgMYgBBIAVILaW50ZXJuYWxVcm'
    'yIAQESNAoOYXBwbGljYXRpb25faWQYBSABKAlCCLpIBXIDGP8BSAJSDWFwcGxpY2F0aW9uSWSI'
    'AQESMAoMcmVkaXJlY3RfdXJpGAYgASgJQgi6SAVyAxiAEEgDUgtyZWRpcmVjdFVyaYgBARImCg'
    'ZzZWNyZXQYByABKAlCCbpIBnIEGICAQEgEUgZzZWNyZXSIAQESMQoMYWNjZXNzX3Rva2VuGAgg'
    'ASgJQgm6SAZyBBiAgEBIBVILYWNjZXNzVG9rZW6IAQESMwoNcmVmcmVzaF90b2tlbhgJIAEoCU'
    'IJukgGcgQYgIBASAZSDHJlZnJlc2hUb2tlbogBARIsCgpncm91cF9uYW1lGAogASgJQgi6SAVy'
    'Axj/AUgHUglncm91cE5hbWWIAQFCCQoHX3NoYXJlZEIPCg1faW50ZXJuYWxfdXJsQhEKD19hcH'
    'BsaWNhdGlvbl9pZEIPCg1fcmVkaXJlY3RfdXJpQgkKB19zZWNyZXRCDwoNX2FjY2Vzc190b2tl'
    'bkIQCg5fcmVmcmVzaF90b2tlbkINCgtfZ3JvdXBfbmFtZQ==');

@$core.Deprecated('Use createGitlabProviderResponseDescriptor instead')
const CreateGitlabProviderResponse$json = {
  '1': 'CreateGitlabProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
    {'1': 'webhook_secret', '3': 2, '4': 1, '5': 9, '10': 'webhookSecret'},
  ],
};

/// Descriptor for `CreateGitlabProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGitlabProviderResponseDescriptor =
    $convert.base64Decode(
        'ChxDcmVhdGVHaXRsYWJQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXISJQoOd2ViaG9va19zZWNyZXQYAiAB'
        'KAlSDXdlYmhvb2tTZWNyZXQ=');

@$core.Deprecated('Use createGiteaProviderRequestDescriptor instead')
const CreateGiteaProviderRequest$json = {
  '1': 'CreateGiteaProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 2, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {'1': 'url', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'url'},
    {
      '1': 'internal_url',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'internalUrl',
      '17': true
    },
    {
      '1': 'redirect_uri',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'redirectUri',
      '17': true
    },
    {
      '1': 'client_id',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'clientId',
      '17': true
    },
    {
      '1': 'client_secret',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'clientSecret',
      '17': true
    },
    {
      '1': 'access_token',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'accessToken',
      '17': true
    },
    {
      '1': 'refresh_token',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'refreshToken',
      '17': true
    },
    {
      '1': 'scopes',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'scopes',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_internal_url'},
    {'1': '_redirect_uri'},
    {'1': '_client_id'},
    {'1': '_client_secret'},
    {'1': '_access_token'},
    {'1': '_refresh_token'},
    {'1': '_scopes'},
  ],
};

/// Descriptor for `CreateGiteaProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGiteaProviderRequestDescriptor = $convert.base64Decode(
    'ChpDcmVhdGVHaXRlYVByb3ZpZGVyUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP8BUg'
    'RuYW1lEhsKBnNoYXJlZBgCIAEoCEgAUgZzaGFyZWSIAQESHAoDdXJsGAMgASgJQgq6SAdyBRAB'
    'GIAQUgN1cmwSMAoMaW50ZXJuYWxfdXJsGAQgASgJQgi6SAVyAxiAEEgBUgtpbnRlcm5hbFVybI'
    'gBARIwCgxyZWRpcmVjdF91cmkYBSABKAlCCLpIBXIDGIAQSAJSC3JlZGlyZWN0VXJpiAEBEioK'
    'CWNsaWVudF9pZBgGIAEoCUIIukgFcgMY/wFIA1IIY2xpZW50SWSIAQESMwoNY2xpZW50X3NlY3'
    'JldBgHIAEoCUIJukgGcgQYgIBASARSDGNsaWVudFNlY3JldIgBARIxCgxhY2Nlc3NfdG9rZW4Y'
    'CCABKAlCCbpIBnIEGICAQEgFUgthY2Nlc3NUb2tlbogBARIzCg1yZWZyZXNoX3Rva2VuGAkgAS'
    'gJQgm6SAZyBBiAgEBIBlIMcmVmcmVzaFRva2VuiAEBEiUKBnNjb3BlcxgKIAEoCUIIukgFcgMY'
    'gAhIB1IGc2NvcGVziAEBQgkKB19zaGFyZWRCDwoNX2ludGVybmFsX3VybEIPCg1fcmVkaXJlY3'
    'RfdXJpQgwKCl9jbGllbnRfaWRCEAoOX2NsaWVudF9zZWNyZXRCDwoNX2FjY2Vzc190b2tlbkIQ'
    'Cg5fcmVmcmVzaF90b2tlbkIJCgdfc2NvcGVz');

@$core.Deprecated('Use createGiteaProviderResponseDescriptor instead')
const CreateGiteaProviderResponse$json = {
  '1': 'CreateGiteaProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
    {'1': 'webhook_secret', '3': 2, '4': 1, '5': 9, '10': 'webhookSecret'},
  ],
};

/// Descriptor for `CreateGiteaProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createGiteaProviderResponseDescriptor =
    $convert.base64Decode(
        'ChtDcmVhdGVHaXRlYVByb3ZpZGVyUmVzcG9uc2USOQoIcHJvdmlkZXIYASABKAsyHS5vcGVub3'
        'hpZGUuYXBpLnYxLkdpdFByb3ZpZGVyUghwcm92aWRlchIlCg53ZWJob29rX3NlY3JldBgCIAEo'
        'CVINd2ViaG9va1NlY3JldA==');

@$core.Deprecated('Use createBitbucketProviderRequestDescriptor instead')
const CreateBitbucketProviderRequest$json = {
  '1': 'CreateBitbucketProviderRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 2, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'username',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'username',
      '17': true
    },
    {
      '1': 'email',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'email',
      '17': true
    },
    {
      '1': 'app_password',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'appPassword',
      '17': true
    },
    {
      '1': 'api_token',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'apiToken',
      '17': true
    },
    {
      '1': 'workspace',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'workspace',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_username'},
    {'1': '_email'},
    {'1': '_app_password'},
    {'1': '_api_token'},
    {'1': '_workspace'},
  ],
};

/// Descriptor for `CreateBitbucketProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createBitbucketProviderRequestDescriptor = $convert.base64Decode(
    'Ch5DcmVhdGVCaXRidWNrZXRQcm92aWRlclJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQAR'
    'j/AVIEbmFtZRIbCgZzaGFyZWQYAiABKAhIAFIGc2hhcmVkiAEBEikKCHVzZXJuYW1lGAMgASgJ'
    'Qgi6SAVyAxj/AUgBUgh1c2VybmFtZYgBARIjCgVlbWFpbBgEIAEoCUIIukgFcgMYwAJIAlIFZW'
    '1haWyIAQESMQoMYXBwX3Bhc3N3b3JkGAUgASgJQgm6SAZyBBiAgEBIA1ILYXBwUGFzc3dvcmSI'
    'AQESKwoJYXBpX3Rva2VuGAYgASgJQgm6SAZyBBiAgEBIBFIIYXBpVG9rZW6IAQESKwoJd29ya3'
    'NwYWNlGAcgASgJQgi6SAVyAxj/AUgFUgl3b3Jrc3BhY2WIAQFCCQoHX3NoYXJlZEILCglfdXNl'
    'cm5hbWVCCAoGX2VtYWlsQg8KDV9hcHBfcGFzc3dvcmRCDAoKX2FwaV90b2tlbkIMCgpfd29ya3'
    'NwYWNl');

@$core.Deprecated('Use createBitbucketProviderResponseDescriptor instead')
const CreateBitbucketProviderResponse$json = {
  '1': 'CreateBitbucketProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
    {'1': 'webhook_secret', '3': 2, '4': 1, '5': 9, '10': 'webhookSecret'},
  ],
};

/// Descriptor for `CreateBitbucketProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createBitbucketProviderResponseDescriptor =
    $convert.base64Decode(
        'Ch9DcmVhdGVCaXRidWNrZXRQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3'
        'Blbm94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXISJQoOd2ViaG9va19zZWNyZXQY'
        'AiABKAlSDXdlYmhvb2tTZWNyZXQ=');

@$core.Deprecated('Use updateGithubProviderRequestDescriptor instead')
const UpdateGithubProviderRequest$json = {
  '1': 'UpdateGithubProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 3, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'app_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'appName',
      '17': true
    },
    {'1': 'app_id', '3': 5, '4': 1, '5': 3, '9': 2, '10': 'appId', '17': true},
    {
      '1': 'client_id',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'clientId',
      '17': true
    },
    {
      '1': 'client_secret',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'clientSecret',
      '17': true
    },
    {
      '1': 'installation_id',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'installationId',
      '17': true
    },
    {
      '1': 'private_key',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'privateKey',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_app_name'},
    {'1': '_app_id'},
    {'1': '_client_id'},
    {'1': '_client_secret'},
    {'1': '_installation_id'},
    {'1': '_private_key'},
  ],
};

/// Descriptor for `UpdateGithubProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGithubProviderRequestDescriptor = $convert.base64Decode(
    'ChtVcGRhdGVHaXRodWJQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEh'
    '4KBG5hbWUYAiABKAlCCrpIB3IFEAEY/wFSBG5hbWUSGwoGc2hhcmVkGAMgASgISABSBnNoYXJl'
    'ZIgBARIoCghhcHBfbmFtZRgEIAEoCUIIukgFcgMY/wFIAVIHYXBwTmFtZYgBARIaCgZhcHBfaW'
    'QYBSABKANIAlIFYXBwSWSIAQESKgoJY2xpZW50X2lkGAYgASgJQgi6SAVyAxj/AUgDUghjbGll'
    'bnRJZIgBARIzCg1jbGllbnRfc2VjcmV0GAcgASgJQgm6SAZyBBiAgEBIBFIMY2xpZW50U2Vjcm'
    'V0iAEBEjYKD2luc3RhbGxhdGlvbl9pZBgIIAEoCUIIukgFcgMY/wFIBVIOaW5zdGFsbGF0aW9u'
    'SWSIAQESLwoLcHJpdmF0ZV9rZXkYCSABKAlCCbpIBnIEGICAQEgGUgpwcml2YXRlS2V5iAEBQg'
    'kKB19zaGFyZWRCCwoJX2FwcF9uYW1lQgkKB19hcHBfaWRCDAoKX2NsaWVudF9pZEIQCg5fY2xp'
    'ZW50X3NlY3JldEISChBfaW5zdGFsbGF0aW9uX2lkQg4KDF9wcml2YXRlX2tleQ==');

@$core.Deprecated('Use updateGithubProviderResponseDescriptor instead')
const UpdateGithubProviderResponse$json = {
  '1': 'UpdateGithubProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateGithubProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGithubProviderResponseDescriptor =
    $convert.base64Decode(
        'ChxVcGRhdGVHaXRodWJQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use updateGitlabProviderRequestDescriptor instead')
const UpdateGitlabProviderRequest$json = {
  '1': 'UpdateGitlabProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 3, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'url',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'url',
      '17': true
    },
    {
      '1': 'internal_url',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'internalUrl',
      '17': true
    },
    {
      '1': 'application_id',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'applicationId',
      '17': true
    },
    {
      '1': 'redirect_uri',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'redirectUri',
      '17': true
    },
    {
      '1': 'secret',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'secret',
      '17': true
    },
    {
      '1': 'access_token',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'accessToken',
      '17': true
    },
    {
      '1': 'refresh_token',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'refreshToken',
      '17': true
    },
    {
      '1': 'group_name',
      '3': 11,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 8,
      '10': 'groupName',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_url'},
    {'1': '_internal_url'},
    {'1': '_application_id'},
    {'1': '_redirect_uri'},
    {'1': '_secret'},
    {'1': '_access_token'},
    {'1': '_refresh_token'},
    {'1': '_group_name'},
  ],
};

/// Descriptor for `UpdateGitlabProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGitlabProviderRequestDescriptor = $convert.base64Decode(
    'ChtVcGRhdGVHaXRsYWJQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEh'
    '4KBG5hbWUYAiABKAlCCrpIB3IFEAEY/wFSBG5hbWUSGwoGc2hhcmVkGAMgASgISABSBnNoYXJl'
    'ZIgBARIfCgN1cmwYBCABKAlCCLpIBXIDGIAQSAFSA3VybIgBARIwCgxpbnRlcm5hbF91cmwYBS'
    'ABKAlCCLpIBXIDGIAQSAJSC2ludGVybmFsVXJsiAEBEjQKDmFwcGxpY2F0aW9uX2lkGAYgASgJ'
    'Qgi6SAVyAxj/AUgDUg1hcHBsaWNhdGlvbklkiAEBEjAKDHJlZGlyZWN0X3VyaRgHIAEoCUIIuk'
    'gFcgMYgBBIBFILcmVkaXJlY3RVcmmIAQESJgoGc2VjcmV0GAggASgJQgm6SAZyBBiAgEBIBVIG'
    'c2VjcmV0iAEBEjEKDGFjY2Vzc190b2tlbhgJIAEoCUIJukgGcgQYgIBASAZSC2FjY2Vzc1Rva2'
    'VuiAEBEjMKDXJlZnJlc2hfdG9rZW4YCiABKAlCCbpIBnIEGICAQEgHUgxyZWZyZXNoVG9rZW6I'
    'AQESLAoKZ3JvdXBfbmFtZRgLIAEoCUIIukgFcgMY/wFICFIJZ3JvdXBOYW1liAEBQgkKB19zaG'
    'FyZWRCBgoEX3VybEIPCg1faW50ZXJuYWxfdXJsQhEKD19hcHBsaWNhdGlvbl9pZEIPCg1fcmVk'
    'aXJlY3RfdXJpQgkKB19zZWNyZXRCDwoNX2FjY2Vzc190b2tlbkIQCg5fcmVmcmVzaF90b2tlbk'
    'INCgtfZ3JvdXBfbmFtZQ==');

@$core.Deprecated('Use updateGitlabProviderResponseDescriptor instead')
const UpdateGitlabProviderResponse$json = {
  '1': 'UpdateGitlabProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateGitlabProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGitlabProviderResponseDescriptor =
    $convert.base64Decode(
        'ChxVcGRhdGVHaXRsYWJQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use updateGiteaProviderRequestDescriptor instead')
const UpdateGiteaProviderRequest$json = {
  '1': 'UpdateGiteaProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 3, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'url',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'url',
      '17': true
    },
    {
      '1': 'internal_url',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'internalUrl',
      '17': true
    },
    {
      '1': 'redirect_uri',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'redirectUri',
      '17': true
    },
    {
      '1': 'client_id',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'clientId',
      '17': true
    },
    {
      '1': 'client_secret',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'clientSecret',
      '17': true
    },
    {
      '1': 'access_token',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 6,
      '10': 'accessToken',
      '17': true
    },
    {
      '1': 'refresh_token',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'refreshToken',
      '17': true
    },
    {
      '1': 'scopes',
      '3': 11,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 8,
      '10': 'scopes',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_url'},
    {'1': '_internal_url'},
    {'1': '_redirect_uri'},
    {'1': '_client_id'},
    {'1': '_client_secret'},
    {'1': '_access_token'},
    {'1': '_refresh_token'},
    {'1': '_scopes'},
  ],
};

/// Descriptor for `UpdateGiteaProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGiteaProviderRequestDescriptor = $convert.base64Decode(
    'ChpVcGRhdGVHaXRlYVByb3ZpZGVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSHg'
    'oEbmFtZRgCIAEoCUIKukgHcgUQARj/AVIEbmFtZRIbCgZzaGFyZWQYAyABKAhIAFIGc2hhcmVk'
    'iAEBEh8KA3VybBgEIAEoCUIIukgFcgMYgBBIAVIDdXJsiAEBEjAKDGludGVybmFsX3VybBgFIA'
    'EoCUIIukgFcgMYgBBIAlILaW50ZXJuYWxVcmyIAQESMAoMcmVkaXJlY3RfdXJpGAYgASgJQgi6'
    'SAVyAxiAEEgDUgtyZWRpcmVjdFVyaYgBARIqCgljbGllbnRfaWQYByABKAlCCLpIBXIDGP8BSA'
    'RSCGNsaWVudElkiAEBEjMKDWNsaWVudF9zZWNyZXQYCCABKAlCCbpIBnIEGICAQEgFUgxjbGll'
    'bnRTZWNyZXSIAQESMQoMYWNjZXNzX3Rva2VuGAkgASgJQgm6SAZyBBiAgEBIBlILYWNjZXNzVG'
    '9rZW6IAQESMwoNcmVmcmVzaF90b2tlbhgKIAEoCUIJukgGcgQYgIBASAdSDHJlZnJlc2hUb2tl'
    'bogBARIlCgZzY29wZXMYCyABKAlCCLpIBXIDGIAISAhSBnNjb3Blc4gBAUIJCgdfc2hhcmVkQg'
    'YKBF91cmxCDwoNX2ludGVybmFsX3VybEIPCg1fcmVkaXJlY3RfdXJpQgwKCl9jbGllbnRfaWRC'
    'EAoOX2NsaWVudF9zZWNyZXRCDwoNX2FjY2Vzc190b2tlbkIQCg5fcmVmcmVzaF90b2tlbkIJCg'
    'dfc2NvcGVz');

@$core.Deprecated('Use updateGiteaProviderResponseDescriptor instead')
const UpdateGiteaProviderResponse$json = {
  '1': 'UpdateGiteaProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateGiteaProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateGiteaProviderResponseDescriptor =
    $convert.base64Decode(
        'ChtVcGRhdGVHaXRlYVByb3ZpZGVyUmVzcG9uc2USOQoIcHJvdmlkZXIYASABKAsyHS5vcGVub3'
        'hpZGUuYXBpLnYxLkdpdFByb3ZpZGVyUghwcm92aWRlcg==');

@$core.Deprecated('Use updateBitbucketProviderRequestDescriptor instead')
const UpdateBitbucketProviderRequest$json = {
  '1': 'UpdateBitbucketProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'shared', '3': 3, '4': 1, '5': 8, '9': 0, '10': 'shared', '17': true},
    {
      '1': 'username',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'username',
      '17': true
    },
    {
      '1': 'email',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'email',
      '17': true
    },
    {
      '1': 'app_password',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'appPassword',
      '17': true
    },
    {
      '1': 'api_token',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'apiToken',
      '17': true
    },
    {
      '1': 'workspace',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 5,
      '10': 'workspace',
      '17': true
    },
  ],
  '8': [
    {'1': '_shared'},
    {'1': '_username'},
    {'1': '_email'},
    {'1': '_app_password'},
    {'1': '_api_token'},
    {'1': '_workspace'},
  ],
};

/// Descriptor for `UpdateBitbucketProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateBitbucketProviderRequestDescriptor = $convert.base64Decode(
    'Ch5VcGRhdGVCaXRidWNrZXRQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAm'
    'lkEh4KBG5hbWUYAiABKAlCCrpIB3IFEAEY/wFSBG5hbWUSGwoGc2hhcmVkGAMgASgISABSBnNo'
    'YXJlZIgBARIpCgh1c2VybmFtZRgEIAEoCUIIukgFcgMY/wFIAVIIdXNlcm5hbWWIAQESIwoFZW'
    '1haWwYBSABKAlCCLpIBXIDGMACSAJSBWVtYWlsiAEBEjEKDGFwcF9wYXNzd29yZBgGIAEoCUIJ'
    'ukgGcgQYgIBASANSC2FwcFBhc3N3b3JkiAEBEisKCWFwaV90b2tlbhgHIAEoCUIJukgGcgQYgI'
    'BASARSCGFwaVRva2VuiAEBEisKCXdvcmtzcGFjZRgIIAEoCUIIukgFcgMY/wFIBVIJd29ya3Nw'
    'YWNliAEBQgkKB19zaGFyZWRCCwoJX3VzZXJuYW1lQggKBl9lbWFpbEIPCg1fYXBwX3Bhc3N3b3'
    'JkQgwKCl9hcGlfdG9rZW5CDAoKX3dvcmtzcGFjZQ==');

@$core.Deprecated('Use updateBitbucketProviderResponseDescriptor instead')
const UpdateBitbucketProviderResponse$json = {
  '1': 'UpdateBitbucketProviderResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `UpdateBitbucketProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateBitbucketProviderResponseDescriptor =
    $convert.base64Decode(
        'Ch9VcGRhdGVCaXRidWNrZXRQcm92aWRlclJlc3BvbnNlEjkKCHByb3ZpZGVyGAEgASgLMh0ub3'
        'Blbm94aWRlLmFwaS52MS5HaXRQcm92aWRlclIIcHJvdmlkZXI=');

@$core.Deprecated('Use deleteGitProviderRequestDescriptor instead')
const DeleteGitProviderRequest$json = {
  '1': 'DeleteGitProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteGitProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteGitProviderRequestDescriptor =
    $convert.base64Decode(
        'ChhEZWxldGVHaXRQcm92aWRlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use testGitProviderRequestDescriptor instead')
const TestGitProviderRequest$json = {
  '1': 'TestGitProviderRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `TestGitProviderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testGitProviderRequestDescriptor =
    $convert.base64Decode(
        'ChZUZXN0R2l0UHJvdmlkZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use testGitProviderResponseDescriptor instead')
const TestGitProviderResponse$json = {
  '1': 'TestGitProviderResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestGitProviderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testGitProviderResponseDescriptor =
    $convert.base64Decode(
        'ChdUZXN0R2l0UHJvdmlkZXJSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEhgKB2'
        '1lc3NhZ2UYAiABKAlSB21lc3NhZ2U=');

@$core.Deprecated('Use gitRotateWebhookSecretRequestDescriptor instead')
const GitRotateWebhookSecretRequest$json = {
  '1': 'GitRotateWebhookSecretRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GitRotateWebhookSecretRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitRotateWebhookSecretRequestDescriptor =
    $convert.base64Decode(
        'Ch1HaXRSb3RhdGVXZWJob29rU2VjcmV0UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use gitRotateWebhookSecretResponseDescriptor instead')
const GitRotateWebhookSecretResponse$json = {
  '1': 'GitRotateWebhookSecretResponse',
  '2': [
    {'1': 'webhook_secret', '3': 1, '4': 1, '5': 9, '10': 'webhookSecret'},
  ],
};

/// Descriptor for `GitRotateWebhookSecretResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitRotateWebhookSecretResponseDescriptor =
    $convert.base64Decode(
        'Ch5HaXRSb3RhdGVXZWJob29rU2VjcmV0UmVzcG9uc2USJQoOd2ViaG9va19zZWNyZXQYASABKA'
        'lSDXdlYmhvb2tTZWNyZXQ=');

@$core.Deprecated('Use gitListRepositoriesRequestDescriptor instead')
const GitListRepositoriesRequest$json = {
  '1': 'GitListRepositoriesRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `GitListRepositoriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListRepositoriesRequestDescriptor =
    $convert.base64Decode(
        'ChpHaXRMaXN0UmVwb3NpdG9yaWVzUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIg'
        'IgAFIKcHJvdmlkZXJJZA==');

@$core.Deprecated('Use gitListRepositoriesResponseDescriptor instead')
const GitListRepositoriesResponse$json = {
  '1': 'GitListRepositoriesResponse',
  '2': [
    {
      '1': 'repositories',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderRepository',
      '10': 'repositories'
    },
  ],
};

/// Descriptor for `GitListRepositoriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListRepositoriesResponseDescriptor =
    $convert.base64Decode(
        'ChtHaXRMaXN0UmVwb3NpdG9yaWVzUmVzcG9uc2USSwoMcmVwb3NpdG9yaWVzGAEgAygLMicub3'
        'Blbm94aWRlLmFwaS52MS5HaXRQcm92aWRlclJlcG9zaXRvcnlSDHJlcG9zaXRvcmllcw==');

@$core.Deprecated('Use gitListRepositoryBranchesRequestDescriptor instead')
const GitListRepositoryBranchesRequest$json = {
  '1': 'GitListRepositoryBranchesRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
  ],
};

/// Descriptor for `GitListRepositoryBranchesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListRepositoryBranchesRequestDescriptor =
    $convert.base64Decode(
        'CiBHaXRMaXN0UmVwb3NpdG9yeUJyYW5jaGVzUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0'
        'IHukgEIgIgAFIKcHJvdmlkZXJJZBIgCgVvd25lchgCIAEoCUIKukgHcgUQARj/AVIFb3duZXIS'
        'KgoKcmVwb3NpdG9yeRgDIAEoCUIKukgHcgUQARj/AVIKcmVwb3NpdG9yeQ==');

@$core.Deprecated('Use gitListRepositoryBranchesResponseDescriptor instead')
const GitListRepositoryBranchesResponse$json = {
  '1': 'GitListRepositoryBranchesResponse',
  '2': [
    {
      '1': 'branches',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderReference',
      '10': 'branches'
    },
  ],
};

/// Descriptor for `GitListRepositoryBranchesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListRepositoryBranchesResponseDescriptor =
    $convert.base64Decode(
        'CiFHaXRMaXN0UmVwb3NpdG9yeUJyYW5jaGVzUmVzcG9uc2USQgoIYnJhbmNoZXMYASADKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLkdpdFByb3ZpZGVyUmVmZXJlbmNlUghicmFuY2hlcw==');

@$core.Deprecated('Use gitListTagsRequestDescriptor instead')
const GitListTagsRequest$json = {
  '1': 'GitListTagsRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
  ],
};

/// Descriptor for `GitListTagsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListTagsRequestDescriptor = $convert.base64Decode(
    'ChJHaXRMaXN0VGFnc1JlcXVlc3QSKAoLcHJvdmlkZXJfaWQYASABKANCB7pIBCICIABSCnByb3'
    'ZpZGVySWQSIAoFb3duZXIYAiABKAlCCrpIB3IFEAEY/wFSBW93bmVyEioKCnJlcG9zaXRvcnkY'
    'AyABKAlCCrpIB3IFEAEY/wFSCnJlcG9zaXRvcnk=');

@$core.Deprecated('Use gitListTagsResponseDescriptor instead')
const GitListTagsResponse$json = {
  '1': 'GitListTagsResponse',
  '2': [
    {
      '1': 'tags',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderReference',
      '10': 'tags'
    },
  ],
};

/// Descriptor for `GitListTagsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitListTagsResponseDescriptor = $convert.base64Decode(
    'ChNHaXRMaXN0VGFnc1Jlc3BvbnNlEjoKBHRhZ3MYASADKAsyJi5vcGVub3hpZGUuYXBpLnYxLk'
    'dpdFByb3ZpZGVyUmVmZXJlbmNlUgR0YWdz');

@$core.Deprecated('Use gitAuthorizeRequestDescriptor instead')
const GitAuthorizeRequest$json = {
  '1': 'GitAuthorizeRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `GitAuthorizeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitAuthorizeRequestDescriptor = $convert.base64Decode(
    'ChNHaXRBdXRob3JpemVSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAiAAUgpwcm'
    '92aWRlcklk');

@$core.Deprecated('Use gitAuthorizeResponseDescriptor instead')
const GitAuthorizeResponse$json = {
  '1': 'GitAuthorizeResponse',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '10': 'url'},
    {'1': 'state', '3': 2, '4': 1, '5': 9, '10': 'state'},
  ],
};

/// Descriptor for `GitAuthorizeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitAuthorizeResponseDescriptor = $convert.base64Decode(
    'ChRHaXRBdXRob3JpemVSZXNwb25zZRIQCgN1cmwYASABKAlSA3VybBIUCgVzdGF0ZRgCIAEoCV'
    'IFc3RhdGU=');

@$core.Deprecated('Use gitOAuthCallbackRequestDescriptor instead')
const GitOAuthCallbackRequest$json = {
  '1': 'GitOAuthCallbackRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {
      '1': 'code',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'code',
      '17': true
    },
    {'1': 'state', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'state'},
    {
      '1': 'installation_id',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'installationId',
      '17': true
    },
    {
      '1': 'setup_action',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'setupAction',
      '17': true
    },
  ],
  '8': [
    {'1': '_code'},
    {'1': '_installation_id'},
    {'1': '_setup_action'},
  ],
};

/// Descriptor for `GitOAuthCallbackRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitOAuthCallbackRequestDescriptor = $convert.base64Decode(
    'ChdHaXRPQXV0aENhbGxiYWNrUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIgIgAF'
    'IKcHJvdmlkZXJJZBIhCgRjb2RlGAIgASgJQgi6SAVyAxiAIEgAUgRjb2RliAEBEiAKBXN0YXRl'
    'GAMgASgJQgq6SAdyBRABGIAgUgVzdGF0ZRI2Cg9pbnN0YWxsYXRpb25faWQYBCABKAlCCLpIBX'
    'IDGP8BSAFSDmluc3RhbGxhdGlvbklkiAEBEjAKDHNldHVwX2FjdGlvbhgFIAEoCUIIukgFcgMY'
    '/wFIAlILc2V0dXBBY3Rpb26IAQFCBwoFX2NvZGVCEgoQX2luc3RhbGxhdGlvbl9pZEIPCg1fc2'
    'V0dXBfYWN0aW9u');

@$core.Deprecated('Use gitOAuthCallbackResponseDescriptor instead')
const GitOAuthCallbackResponse$json = {
  '1': 'GitOAuthCallbackResponse',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProvider',
      '10': 'provider'
    },
  ],
};

/// Descriptor for `GitOAuthCallbackResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitOAuthCallbackResponseDescriptor =
    $convert.base64Decode(
        'ChhHaXRPQXV0aENhbGxiYWNrUmVzcG9uc2USOQoIcHJvdmlkZXIYASABKAsyHS5vcGVub3hpZG'
        'UuYXBpLnYxLkdpdFByb3ZpZGVyUghwcm92aWRlcg==');

@$core.Deprecated('Use gitDisconnectRequestDescriptor instead')
const GitDisconnectRequest$json = {
  '1': 'GitDisconnectRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
  ],
};

/// Descriptor for `GitDisconnectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitDisconnectRequestDescriptor = $convert.base64Decode(
    'ChRHaXREaXNjb25uZWN0UmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIgIgAFIKcH'
    'JvdmlkZXJJZA==');

@$core.Deprecated('Use gitGetCollaboratorPermissionRequestDescriptor instead')
const GitGetCollaboratorPermissionRequest$json = {
  '1': 'GitGetCollaboratorPermissionRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'username', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'username'},
  ],
};

/// Descriptor for `GitGetCollaboratorPermissionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitGetCollaboratorPermissionRequestDescriptor =
    $convert.base64Decode(
        'CiNHaXRHZXRDb2xsYWJvcmF0b3JQZXJtaXNzaW9uUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIA'
        'EoA0IHukgEIgIgAFIKcHJvdmlkZXJJZBIgCgVvd25lchgCIAEoCUIKukgHcgUQARj/AVIFb3du'
        'ZXISKgoKcmVwb3NpdG9yeRgDIAEoCUIKukgHcgUQARj/AVIKcmVwb3NpdG9yeRImCgh1c2Vybm'
        'FtZRgEIAEoCUIKukgHcgUQARj/AVIIdXNlcm5hbWU=');

@$core.Deprecated('Use gitGetCollaboratorPermissionResponseDescriptor instead')
const GitGetCollaboratorPermissionResponse$json = {
  '1': 'GitGetCollaboratorPermissionResponse',
  '2': [
    {
      '1': 'permission',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderCollaboratorPermission',
      '10': 'permission'
    },
  ],
};

/// Descriptor for `GitGetCollaboratorPermissionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitGetCollaboratorPermissionResponseDescriptor =
    $convert.base64Decode(
        'CiRHaXRHZXRDb2xsYWJvcmF0b3JQZXJtaXNzaW9uUmVzcG9uc2USUwoKcGVybWlzc2lvbhgBIA'
        'EoCzIzLm9wZW5veGlkZS5hcGkudjEuR2l0UHJvdmlkZXJDb2xsYWJvcmF0b3JQZXJtaXNzaW9u'
        'UgpwZXJtaXNzaW9u');

@$core.Deprecated('Use gitGetWebhookStatusRequestDescriptor instead')
const GitGetWebhookStatusRequest$json = {
  '1': 'GitGetWebhookStatusRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'callback_url', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `GitGetWebhookStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitGetWebhookStatusRequestDescriptor = $convert.base64Decode(
    'ChpHaXRHZXRXZWJob29rU3RhdHVzUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIg'
    'IgAFIKcHJvdmlkZXJJZBIgCgVvd25lchgCIAEoCUIKukgHcgUQARj/AVIFb3duZXISKgoKcmVw'
    'b3NpdG9yeRgDIAEoCUIKukgHcgUQARj/AVIKcmVwb3NpdG9yeRItCgxjYWxsYmFja191cmwYBC'
    'ABKAlCCrpIB3IFEAEYgBBSC2NhbGxiYWNrVXJs');

@$core.Deprecated('Use gitGetWebhookStatusResponseDescriptor instead')
const GitGetWebhookStatusResponse$json = {
  '1': 'GitGetWebhookStatusResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderWebhookInfo',
      '9': 0,
      '10': 'webhook',
      '17': true
    },
  ],
  '8': [
    {'1': '_webhook'},
  ],
};

/// Descriptor for `GitGetWebhookStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitGetWebhookStatusResponseDescriptor =
    $convert.base64Decode(
        'ChtHaXRHZXRXZWJob29rU3RhdHVzUmVzcG9uc2USRwoHd2ViaG9vaxgBIAEoCzIoLm9wZW5veG'
        'lkZS5hcGkudjEuR2l0UHJvdmlkZXJXZWJob29rSW5mb0gAUgd3ZWJob29riAEBQgoKCF93ZWJo'
        'b29r');

@$core.Deprecated('Use gitInstallWebhookRequestDescriptor instead')
const GitInstallWebhookRequest$json = {
  '1': 'GitInstallWebhookRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'callback_url', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `GitInstallWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitInstallWebhookRequestDescriptor = $convert.base64Decode(
    'ChhHaXRJbnN0YWxsV2ViaG9va1JlcXVlc3QSKAoLcHJvdmlkZXJfaWQYASABKANCB7pIBCICIA'
    'BSCnByb3ZpZGVySWQSIAoFb3duZXIYAiABKAlCCrpIB3IFEAEY/wFSBW93bmVyEioKCnJlcG9z'
    'aXRvcnkYAyABKAlCCrpIB3IFEAEY/wFSCnJlcG9zaXRvcnkSLQoMY2FsbGJhY2tfdXJsGAQgAS'
    'gJQgq6SAdyBRABGIAQUgtjYWxsYmFja1VybA==');

@$core.Deprecated('Use gitInstallWebhookResponseDescriptor instead')
const GitInstallWebhookResponse$json = {
  '1': 'GitInstallWebhookResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderWebhookInfo',
      '10': 'webhook'
    },
  ],
};

/// Descriptor for `GitInstallWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitInstallWebhookResponseDescriptor =
    $convert.base64Decode(
        'ChlHaXRJbnN0YWxsV2ViaG9va1Jlc3BvbnNlEkIKB3dlYmhvb2sYASABKAsyKC5vcGVub3hpZG'
        'UuYXBpLnYxLkdpdFByb3ZpZGVyV2ViaG9va0luZm9SB3dlYmhvb2s=');

@$core.Deprecated('Use gitRecreateWebhookRequestDescriptor instead')
const GitRecreateWebhookRequest$json = {
  '1': 'GitRecreateWebhookRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'callback_url', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `GitRecreateWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitRecreateWebhookRequestDescriptor = $convert.base64Decode(
    'ChlHaXRSZWNyZWF0ZVdlYmhvb2tSZXF1ZXN0EigKC3Byb3ZpZGVyX2lkGAEgASgDQge6SAQiAi'
    'AAUgpwcm92aWRlcklkEiAKBW93bmVyGAIgASgJQgq6SAdyBRABGP8BUgVvd25lchIqCgpyZXBv'
    'c2l0b3J5GAMgASgJQgq6SAdyBRABGP8BUgpyZXBvc2l0b3J5Ei0KDGNhbGxiYWNrX3VybBgEIA'
    'EoCUIKukgHcgUQARiAEFILY2FsbGJhY2tVcmw=');

@$core.Deprecated('Use gitRecreateWebhookResponseDescriptor instead')
const GitRecreateWebhookResponse$json = {
  '1': 'GitRecreateWebhookResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GitProviderWebhookInfo',
      '10': 'webhook'
    },
  ],
};

/// Descriptor for `GitRecreateWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitRecreateWebhookResponseDescriptor =
    $convert.base64Decode(
        'ChpHaXRSZWNyZWF0ZVdlYmhvb2tSZXNwb25zZRJCCgd3ZWJob29rGAEgASgLMigub3Blbm94aW'
        'RlLmFwaS52MS5HaXRQcm92aWRlcldlYmhvb2tJbmZvUgd3ZWJob29r');

@$core.Deprecated('Use gitDeleteWebhookRequestDescriptor instead')
const GitDeleteWebhookRequest$json = {
  '1': 'GitDeleteWebhookRequest',
  '2': [
    {'1': 'provider_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'callback_url', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `GitDeleteWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List gitDeleteWebhookRequestDescriptor = $convert.base64Decode(
    'ChdHaXREZWxldGVXZWJob29rUmVxdWVzdBIoCgtwcm92aWRlcl9pZBgBIAEoA0IHukgEIgIgAF'
    'IKcHJvdmlkZXJJZBIgCgVvd25lchgCIAEoCUIKukgHcgUQARj/AVIFb3duZXISKgoKcmVwb3Np'
    'dG9yeRgDIAEoCUIKukgHcgUQARj/AVIKcmVwb3NpdG9yeRItCgxjYWxsYmFja191cmwYBCABKA'
    'lCCrpIB3IFEAEYgBBSC2NhbGxiYWNrVXJs');

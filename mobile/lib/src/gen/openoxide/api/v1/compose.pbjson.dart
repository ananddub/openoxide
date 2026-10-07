// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/compose.proto.

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

@$core.Deprecated('Use composeMountTypeDescriptor instead')
const ComposeMountType$json = {
  '1': 'ComposeMountType',
  '2': [
    {'1': 'COMPOSE_MOUNT_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'COMPOSE_MOUNT_TYPE_VOLUME', '2': 1},
    {'1': 'COMPOSE_MOUNT_TYPE_BIND', '2': 2},
    {'1': 'COMPOSE_MOUNT_TYPE_FILE', '2': 3},
  ],
};

/// Descriptor for `ComposeMountType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List composeMountTypeDescriptor = $convert.base64Decode(
    'ChBDb21wb3NlTW91bnRUeXBlEiIKHkNPTVBPU0VfTU9VTlRfVFlQRV9VTlNQRUNJRklFRBAAEh'
    '0KGUNPTVBPU0VfTU9VTlRfVFlQRV9WT0xVTUUQARIbChdDT01QT1NFX01PVU5UX1RZUEVfQklO'
    'RBACEhsKF0NPTVBPU0VfTU9VTlRfVFlQRV9GSUxFEAM=');

@$core.Deprecated('Use composePatchTypeDescriptor instead')
const ComposePatchType$json = {
  '1': 'ComposePatchType',
  '2': [
    {'1': 'COMPOSE_PATCH_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'COMPOSE_PATCH_TYPE_UPDATE', '2': 1},
    {'1': 'COMPOSE_PATCH_TYPE_DELETE', '2': 2},
  ],
};

/// Descriptor for `ComposePatchType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List composePatchTypeDescriptor = $convert.base64Decode(
    'ChBDb21wb3NlUGF0Y2hUeXBlEiIKHkNPTVBPU0VfUEFUQ0hfVFlQRV9VTlNQRUNJRklFRBAAEh'
    '0KGUNPTVBPU0VfUEFUQ0hfVFlQRV9VUERBVEUQARIdChlDT01QT1NFX1BBVENIX1RZUEVfREVM'
    'RVRFEAI=');

@$core.Deprecated('Use composeDescriptor instead')
const Compose$json = {
  '1': 'Compose',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'environment_id', '3': 4, '4': 1, '5': 3, '10': 'environmentId'},
    {
      '1': 'server_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {'1': 'status', '3': 6, '4': 1, '5': 9, '10': 'status'},
    {
      '1': 'source_type',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'sourceType',
      '17': true
    },
    {
      '1': 'repository',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'repository',
      '17': true
    },
    {'1': 'branch', '3': 9, '4': 1, '5': 9, '9': 4, '10': 'branch', '17': true},
    {
      '1': 'compose_file',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'composeFile',
      '17': true
    },
    {'1': 'created_at', '3': 11, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 12, '4': 1, '5': 3, '10': 'updatedAt'},
    {'1': 'owner', '3': 13, '4': 1, '5': 9, '9': 6, '10': 'owner', '17': true},
    {
      '1': 'gitlab_repository',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'gitlabRepository',
      '17': true
    },
    {
      '1': 'gitlab_owner',
      '3': 15,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'gitlabOwner',
      '17': true
    },
    {
      '1': 'gitlab_branch',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'gitlabBranch',
      '17': true
    },
    {
      '1': 'gitea_repository',
      '3': 17,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'giteaRepository',
      '17': true
    },
    {
      '1': 'gitea_owner',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 11,
      '10': 'giteaOwner',
      '17': true
    },
    {
      '1': 'gitea_branch',
      '3': 19,
      '4': 1,
      '5': 9,
      '9': 12,
      '10': 'giteaBranch',
      '17': true
    },
    {
      '1': 'bitbucket_repository',
      '3': 20,
      '4': 1,
      '5': 9,
      '9': 13,
      '10': 'bitbucketRepository',
      '17': true
    },
    {
      '1': 'bitbucket_owner',
      '3': 21,
      '4': 1,
      '5': 9,
      '9': 14,
      '10': 'bitbucketOwner',
      '17': true
    },
    {
      '1': 'bitbucket_branch',
      '3': 22,
      '4': 1,
      '5': 9,
      '9': 15,
      '10': 'bitbucketBranch',
      '17': true
    },
    {
      '1': 'custom_git_url',
      '3': 23,
      '4': 1,
      '5': 9,
      '9': 16,
      '10': 'customGitUrl',
      '17': true
    },
    {
      '1': 'custom_git_branch',
      '3': 24,
      '4': 1,
      '5': 9,
      '9': 17,
      '10': 'customGitBranch',
      '17': true
    },
    {
      '1': 'command',
      '3': 25,
      '4': 1,
      '5': 9,
      '9': 18,
      '10': 'command',
      '17': true
    },
    {
      '1': 'compose_path',
      '3': 26,
      '4': 1,
      '5': 9,
      '9': 19,
      '10': 'composePath',
      '17': true
    },
    {
      '1': 'compose_type',
      '3': 27,
      '4': 1,
      '5': 9,
      '9': 20,
      '10': 'composeType',
      '17': true
    },
    {
      '1': 'last_error',
      '3': 28,
      '4': 1,
      '5': 9,
      '9': 21,
      '10': 'lastError',
      '17': true
    },
    {
      '1': 'build_path',
      '3': 29,
      '4': 1,
      '5': 9,
      '9': 22,
      '10': 'buildPath',
      '17': true
    },
    {
      '1': 'custom_git_ssh_key_id',
      '3': 30,
      '4': 1,
      '5': 3,
      '9': 23,
      '10': 'customGitSshKeyId',
      '17': true
    },
    {'1': 'randomize', '3': 31, '4': 1, '5': 8, '10': 'randomize'},
    {
      '1': 'suffix',
      '3': 32,
      '4': 1,
      '5': 9,
      '9': 24,
      '10': 'suffix',
      '17': true
    },
    {
      '1': 'env_var',
      '3': 33,
      '4': 1,
      '5': 9,
      '9': 25,
      '10': 'envVar',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_server_id'},
    {'1': '_source_type'},
    {'1': '_repository'},
    {'1': '_branch'},
    {'1': '_compose_file'},
    {'1': '_owner'},
    {'1': '_gitlab_repository'},
    {'1': '_gitlab_owner'},
    {'1': '_gitlab_branch'},
    {'1': '_gitea_repository'},
    {'1': '_gitea_owner'},
    {'1': '_gitea_branch'},
    {'1': '_bitbucket_repository'},
    {'1': '_bitbucket_owner'},
    {'1': '_bitbucket_branch'},
    {'1': '_custom_git_url'},
    {'1': '_custom_git_branch'},
    {'1': '_command'},
    {'1': '_compose_path'},
    {'1': '_compose_type'},
    {'1': '_last_error'},
    {'1': '_build_path'},
    {'1': '_custom_git_ssh_key_id'},
    {'1': '_suffix'},
    {'1': '_env_var'},
  ],
};

/// Descriptor for `Compose`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeDescriptor = $convert.base64Decode(
    'CgdDb21wb3NlEg4KAmlkGAEgASgDUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEiUKC2Rlc2NyaX'
    'B0aW9uGAMgASgJSABSC2Rlc2NyaXB0aW9uiAEBEiUKDmVudmlyb25tZW50X2lkGAQgASgDUg1l'
    'bnZpcm9ubWVudElkEiAKCXNlcnZlcl9pZBgFIAEoA0gBUghzZXJ2ZXJJZIgBARIWCgZzdGF0dX'
    'MYBiABKAlSBnN0YXR1cxIkCgtzb3VyY2VfdHlwZRgHIAEoCUgCUgpzb3VyY2VUeXBliAEBEiMK'
    'CnJlcG9zaXRvcnkYCCABKAlIA1IKcmVwb3NpdG9yeYgBARIbCgZicmFuY2gYCSABKAlIBFIGYn'
    'JhbmNoiAEBEiYKDGNvbXBvc2VfZmlsZRgKIAEoCUgFUgtjb21wb3NlRmlsZYgBARIdCgpjcmVh'
    'dGVkX2F0GAsgASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgMIAEoA1IJdXBkYXRlZEF0Eh'
    'kKBW93bmVyGA0gASgJSAZSBW93bmVyiAEBEjAKEWdpdGxhYl9yZXBvc2l0b3J5GA4gASgJSAdS'
    'EGdpdGxhYlJlcG9zaXRvcnmIAQESJgoMZ2l0bGFiX293bmVyGA8gASgJSAhSC2dpdGxhYk93bm'
    'VyiAEBEigKDWdpdGxhYl9icmFuY2gYECABKAlICVIMZ2l0bGFiQnJhbmNoiAEBEi4KEGdpdGVh'
    'X3JlcG9zaXRvcnkYESABKAlIClIPZ2l0ZWFSZXBvc2l0b3J5iAEBEiQKC2dpdGVhX293bmVyGB'
    'IgASgJSAtSCmdpdGVhT3duZXKIAQESJgoMZ2l0ZWFfYnJhbmNoGBMgASgJSAxSC2dpdGVhQnJh'
    'bmNoiAEBEjYKFGJpdGJ1Y2tldF9yZXBvc2l0b3J5GBQgASgJSA1SE2JpdGJ1Y2tldFJlcG9zaX'
    'RvcnmIAQESLAoPYml0YnVja2V0X293bmVyGBUgASgJSA5SDmJpdGJ1Y2tldE93bmVyiAEBEi4K'
    'EGJpdGJ1Y2tldF9icmFuY2gYFiABKAlID1IPYml0YnVja2V0QnJhbmNoiAEBEikKDmN1c3RvbV'
    '9naXRfdXJsGBcgASgJSBBSDGN1c3RvbUdpdFVybIgBARIvChFjdXN0b21fZ2l0X2JyYW5jaBgY'
    'IAEoCUgRUg9jdXN0b21HaXRCcmFuY2iIAQESHQoHY29tbWFuZBgZIAEoCUgSUgdjb21tYW5kiA'
    'EBEiYKDGNvbXBvc2VfcGF0aBgaIAEoCUgTUgtjb21wb3NlUGF0aIgBARImCgxjb21wb3NlX3R5'
    'cGUYGyABKAlIFFILY29tcG9zZVR5cGWIAQESIgoKbGFzdF9lcnJvchgcIAEoCUgVUglsYXN0RX'
    'Jyb3KIAQESIgoKYnVpbGRfcGF0aBgdIAEoCUgWUglidWlsZFBhdGiIAQESNQoVY3VzdG9tX2dp'
    'dF9zc2hfa2V5X2lkGB4gASgDSBdSEWN1c3RvbUdpdFNzaEtleUlkiAEBEhwKCXJhbmRvbWl6ZR'
    'gfIAEoCFIJcmFuZG9taXplEhsKBnN1ZmZpeBggIAEoCUgYUgZzdWZmaXiIAQESHAoHZW52X3Zh'
    'chghIAEoCUgZUgZlbnZWYXKIAQFCDgoMX2Rlc2NyaXB0aW9uQgwKCl9zZXJ2ZXJfaWRCDgoMX3'
    'NvdXJjZV90eXBlQg0KC19yZXBvc2l0b3J5QgkKB19icmFuY2hCDwoNX2NvbXBvc2VfZmlsZUII'
    'CgZfb3duZXJCFAoSX2dpdGxhYl9yZXBvc2l0b3J5Qg8KDV9naXRsYWJfb3duZXJCEAoOX2dpdG'
    'xhYl9icmFuY2hCEwoRX2dpdGVhX3JlcG9zaXRvcnlCDgoMX2dpdGVhX293bmVyQg8KDV9naXRl'
    'YV9icmFuY2hCFwoVX2JpdGJ1Y2tldF9yZXBvc2l0b3J5QhIKEF9iaXRidWNrZXRfb3duZXJCEw'
    'oRX2JpdGJ1Y2tldF9icmFuY2hCEQoPX2N1c3RvbV9naXRfdXJsQhQKEl9jdXN0b21fZ2l0X2Jy'
    'YW5jaEIKCghfY29tbWFuZEIPCg1fY29tcG9zZV9wYXRoQg8KDV9jb21wb3NlX3R5cGVCDQoLX2'
    'xhc3RfZXJyb3JCDQoLX2J1aWxkX3BhdGhCGAoWX2N1c3RvbV9naXRfc3NoX2tleV9pZEIJCgdf'
    'c3VmZml4QgoKCF9lbnZfdmFy');

@$core.Deprecated('Use getComposeRequestDescriptor instead')
const GetComposeRequest$json = {
  '1': 'GetComposeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getComposeRequestDescriptor = $convert.base64Decode(
    'ChFHZXRDb21wb3NlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use listComposesRequestDescriptor instead')
const ListComposesRequest$json = {
  '1': 'ListComposesRequest',
  '2': [
    {
      '1': 'environment_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
  ],
};

/// Descriptor for `ListComposesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listComposesRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0Q29tcG9zZXNSZXF1ZXN0Ei4KDmVudmlyb25tZW50X2lkGAEgASgDQge6SAQiAiAAUg'
    '1lbnZpcm9ubWVudElk');

@$core.Deprecated('Use listComposesResponseDescriptor instead')
const ListComposesResponse$json = {
  '1': 'ListComposesResponse',
  '2': [
    {
      '1': 'composes',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Compose',
      '10': 'composes'
    },
  ],
};

/// Descriptor for `ListComposesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listComposesResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0Q29tcG9zZXNSZXNwb25zZRI1Cghjb21wb3NlcxgBIAMoCzIZLm9wZW5veGlkZS5hcG'
    'kudjEuQ29tcG9zZVIIY29tcG9zZXM=');

@$core.Deprecated('Use createComposeRequestDescriptor instead')
const CreateComposeRequest$json = {
  '1': 'CreateComposeRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'description',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'description',
      '17': true
    },
    {
      '1': 'environment_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {
      '1': 'server_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'compose_file',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'composeFile',
      '17': true
    },
    {
      '1': 'compose_type',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'composeType',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_server_id'},
    {'1': '_compose_file'},
    {'1': '_compose_type'},
  ],
};

/// Descriptor for `CreateComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createComposeRequestDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVDb21wb3NlUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP8BUgRuYW1lEi'
    'UKC2Rlc2NyaXB0aW9uGAIgASgJSABSC2Rlc2NyaXB0aW9uiAEBEi4KDmVudmlyb25tZW50X2lk'
    'GAMgASgDQge6SAQiAiAAUg1lbnZpcm9ubWVudElkEiAKCXNlcnZlcl9pZBgEIAEoA0gBUghzZX'
    'J2ZXJJZIgBARImCgxjb21wb3NlX2ZpbGUYBSABKAlIAlILY29tcG9zZUZpbGWIAQESJgoMY29t'
    'cG9zZV90eXBlGAYgASgJSANSC2NvbXBvc2VUeXBliAEBQg4KDF9kZXNjcmlwdGlvbkIMCgpfc2'
    'VydmVyX2lkQg8KDV9jb21wb3NlX2ZpbGVCDwoNX2NvbXBvc2VfdHlwZQ==');

@$core.Deprecated('Use updateComposeRequestDescriptor instead')
const UpdateComposeRequest$json = {
  '1': 'UpdateComposeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name', '17': true},
    {
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'description',
      '17': true
    },
    {
      '1': 'compose_file',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'composeFile',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'serverId',
      '17': true
    },
    {'1': 'network_ids', '3': 6, '4': 3, '5': 9, '10': 'networkIds'},
    {
      '1': 'command',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'command',
      '17': true
    },
    {
      '1': 'randomize',
      '3': 8,
      '4': 1,
      '5': 8,
      '9': 5,
      '10': 'randomize',
      '17': true
    },
    {
      '1': 'env_var',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'envVar',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_compose_file'},
    {'1': '_server_id'},
    {'1': '_command'},
    {'1': '_randomize'},
    {'1': '_env_var'},
  ],
};

/// Descriptor for `UpdateComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateComposeRequestDescriptor = $convert.base64Decode(
    'ChRVcGRhdGVDb21wb3NlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSFwoEbmFtZR'
    'gCIAEoCUgAUgRuYW1liAEBEiUKC2Rlc2NyaXB0aW9uGAMgASgJSAFSC2Rlc2NyaXB0aW9uiAEB'
    'EiYKDGNvbXBvc2VfZmlsZRgEIAEoCUgCUgtjb21wb3NlRmlsZYgBARIgCglzZXJ2ZXJfaWQYBS'
    'ABKANIA1IIc2VydmVySWSIAQESHwoLbmV0d29ya19pZHMYBiADKAlSCm5ldHdvcmtJZHMSHQoH'
    'Y29tbWFuZBgHIAEoCUgEUgdjb21tYW5kiAEBEiEKCXJhbmRvbWl6ZRgIIAEoCEgFUglyYW5kb2'
    '1pemWIAQESHAoHZW52X3ZhchgJIAEoCUgGUgZlbnZWYXKIAQFCBwoFX25hbWVCDgoMX2Rlc2Ny'
    'aXB0aW9uQg8KDV9jb21wb3NlX2ZpbGVCDAoKX3NlcnZlcl9pZEIKCghfY29tbWFuZEIMCgpfcm'
    'FuZG9taXplQgoKCF9lbnZfdmFy');

@$core.Deprecated('Use deleteComposeRequestDescriptor instead')
const DeleteComposeRequest$json = {
  '1': 'DeleteComposeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteComposeRequestDescriptor =
    $convert.base64Decode(
        'ChREZWxldGVDb21wb3NlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use composeOperationRequestDescriptor instead')
const ComposeOperationRequest$json = {
  '1': 'ComposeOperationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ComposeOperationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeOperationRequestDescriptor =
    $convert.base64Decode(
        'ChdDb21wb3NlT3BlcmF0aW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use composeOperationDescriptor instead')
const ComposeOperation$json = {
  '1': 'ComposeOperation',
  '2': [
    {
      '1': 'compose',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Compose',
      '10': 'compose'
    },
    {'1': 'operation', '3': 2, '4': 1, '5': 9, '10': 'operation'},
  ],
};

/// Descriptor for `ComposeOperation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeOperationDescriptor = $convert.base64Decode(
    'ChBDb21wb3NlT3BlcmF0aW9uEjMKB2NvbXBvc2UYASABKAsyGS5vcGVub3hpZGUuYXBpLnYxLk'
    'NvbXBvc2VSB2NvbXBvc2USHAoJb3BlcmF0aW9uGAIgASgJUglvcGVyYXRpb24=');

@$core.Deprecated('Use setComposeSourceRequestDescriptor instead')
const SetComposeSourceRequest$json = {
  '1': 'SetComposeSourceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'repository', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'owner', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'branch', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'branch', '17': true},
    {
      '1': 'build_path',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'buildPath',
      '17': true
    },
    {
      '1': 'provider_id',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'providerId',
      '17': true
    },
    {
      '1': 'ssh_key_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'sshKeyId',
      '17': true
    },
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_provider_id'},
    {'1': '_ssh_key_id'},
  ],
};

/// Descriptor for `SetComposeSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setComposeSourceRequestDescriptor = $convert.base64Decode(
    'ChdTZXRDb21wb3NlU291cmNlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSJwoKcm'
    'Vwb3NpdG9yeRgCIAEoCUIHukgEcgIQAVIKcmVwb3NpdG9yeRIdCgVvd25lchgDIAEoCUIHukgE'
    'cgIQAVIFb3duZXISGwoGYnJhbmNoGAQgASgJSABSBmJyYW5jaIgBARIiCgpidWlsZF9wYXRoGA'
    'UgASgJSAFSCWJ1aWxkUGF0aIgBARIkCgtwcm92aWRlcl9pZBgGIAEoA0gCUgpwcm92aWRlcklk'
    'iAEBEiEKCnNzaF9rZXlfaWQYByABKANIA1IIc3NoS2V5SWSIAQFCCQoHX2JyYW5jaEINCgtfYn'
    'VpbGRfcGF0aEIOCgxfcHJvdmlkZXJfaWRCDQoLX3NzaF9rZXlfaWQ=');

@$core.Deprecated('Use setRawSourceRequestDescriptor instead')
const SetRawSourceRequest$json = {
  '1': 'SetRawSourceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'content'},
    {
      '1': 'file_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'fileName',
      '17': true
    },
  ],
  '8': [
    {'1': '_file_name'},
  ],
};

/// Descriptor for `SetRawSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setRawSourceRequestDescriptor = $convert.base64Decode(
    'ChNTZXRSYXdTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIlCgdjb250ZW'
    '50GAIgASgJQgu6SAhyBhABGICAQFIHY29udGVudBIgCglmaWxlX25hbWUYAyABKAlIAFIIZmls'
    'ZU5hbWWIAQFCDAoKX2ZpbGVfbmFtZQ==');

@$core.Deprecated('Use uploadComposeRequestDescriptor instead')
const UploadComposeRequest$json = {
  '1': 'UploadComposeRequest',
  '2': [
    {
      '1': 'compose_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'composeId'
    },
    {'1': 'chunk', '3': 2, '4': 1, '5': 12, '8': {}, '9': 0, '10': 'chunk'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `UploadComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List uploadComposeRequestDescriptor = $convert.base64Decode(
    'ChRVcGxvYWRDb21wb3NlUmVxdWVzdBIoCgpjb21wb3NlX2lkGAEgASgDQge6SAQiAiAASABSCW'
    'NvbXBvc2VJZBIkCgVjaHVuaxgCIAEoDEIMukgJegcQARiAgIACSABSBWNodW5rQgkKB3BheWxv'
    'YWQ=');

@$core.Deprecated('Use moveComposeRequestDescriptor instead')
const MoveComposeRequest$json = {
  '1': 'MoveComposeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'target_environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'targetEnvironmentId'
    },
  ],
};

/// Descriptor for `MoveComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List moveComposeRequestDescriptor = $convert.base64Decode(
    'ChJNb3ZlQ29tcG9zZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjsKFXRhcmdldF'
    '9lbnZpcm9ubWVudF9pZBgCIAEoA0IHukgEIgIgAFITdGFyZ2V0RW52aXJvbm1lbnRJZA==');

@$core.Deprecated('Use composeWebhookTokenDescriptor instead')
const ComposeWebhookToken$json = {
  '1': 'ComposeWebhookToken',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `ComposeWebhookToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeWebhookTokenDescriptor =
    $convert.base64Decode(
        'ChNDb21wb3NlV2ViaG9va1Rva2VuEhQKBXRva2VuGAEgASgJUgV0b2tlbg==');

@$core.Deprecated('Use composeDependenciesDescriptor instead')
const ComposeDependencies$json = {
  '1': 'ComposeDependencies',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ComposeDependencies`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeDependenciesDescriptor =
    $convert.base64Decode(
        'ChNDb21wb3NlRGVwZW5kZW5jaWVzEhQKBWl0ZW1zGAEgAygJUgVpdGVtcw==');

@$core.Deprecated('Use previewComposeRequestDescriptor instead')
const PreviewComposeRequest$json = {
  '1': 'PreviewComposeRequest',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'content'},
  ],
};

/// Descriptor for `PreviewComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List previewComposeRequestDescriptor = $convert.base64Decode(
    'ChVQcmV2aWV3Q29tcG9zZVJlcXVlc3QSJQoHY29udGVudBgBIAEoCUILukgIcgYQARiAgEBSB2'
    'NvbnRlbnQ=');

@$core.Deprecated('Use composePreviewDescriptor instead')
const ComposePreview$json = {
  '1': 'ComposePreview',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
    {'1': 'services', '3': 2, '4': 3, '5': 9, '10': 'services'},
  ],
};

/// Descriptor for `ComposePreview`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composePreviewDescriptor = $convert.base64Decode(
    'Cg5Db21wb3NlUHJldmlldxIYCgdjb250ZW50GAEgASgJUgdjb250ZW50EhoKCHNlcnZpY2VzGA'
    'IgAygJUghzZXJ2aWNlcw==');

@$core.Deprecated('Use installTemplateRequestDescriptor instead')
const InstallTemplateRequest$json = {
  '1': 'InstallTemplateRequest',
  '2': [
    {'1': 'template', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'template'},
    {
      '1': 'environment_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {'1': 'name', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'name', '17': true},
    {
      '1': 'server_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'env_var',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'envVar',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_server_id'},
    {'1': '_env_var'},
  ],
};

/// Descriptor for `InstallTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List installTemplateRequestDescriptor = $convert.base64Decode(
    'ChZJbnN0YWxsVGVtcGxhdGVSZXF1ZXN0EiMKCHRlbXBsYXRlGAEgASgJQge6SARyAhABUgh0ZW'
    '1wbGF0ZRIuCg5lbnZpcm9ubWVudF9pZBgCIAEoA0IHukgEIgIgAFINZW52aXJvbm1lbnRJZBIX'
    'CgRuYW1lGAMgASgJSABSBG5hbWWIAQESIAoJc2VydmVyX2lkGAQgASgDSAFSCHNlcnZlcklkiA'
    'EBEhwKB2Vudl92YXIYBSABKAlIAlIGZW52VmFyiAEBQgcKBV9uYW1lQgwKCl9zZXJ2ZXJfaWRC'
    'CgoIX2Vudl92YXI=');

@$core.Deprecated('Use exportComposeRequestDescriptor instead')
const ExportComposeRequest$json = {
  '1': 'ExportComposeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'include_secrets', '3': 2, '4': 1, '5': 8, '10': 'includeSecrets'},
  ],
};

/// Descriptor for `ExportComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exportComposeRequestDescriptor = $convert.base64Decode(
    'ChRFeHBvcnRDb21wb3NlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSJwoPaW5jbH'
    'VkZV9zZWNyZXRzGAIgASgIUg5pbmNsdWRlU2VjcmV0cw==');

@$core.Deprecated('Use composeExportDescriptor instead')
const ComposeExport$json = {
  '1': 'ComposeExport',
  '2': [
    {'1': 'archive', '3': 1, '4': 1, '5': 9, '10': 'archive'},
  ],
};

/// Descriptor for `ComposeExport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeExportDescriptor = $convert
    .base64Decode('Cg1Db21wb3NlRXhwb3J0EhgKB2FyY2hpdmUYASABKAlSB2FyY2hpdmU=');

@$core.Deprecated('Use importComposeRequestDescriptor instead')
const ImportComposeRequest$json = {
  '1': 'ImportComposeRequest',
  '2': [
    {
      '1': 'environment_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {'1': 'archive', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'archive'},
  ],
};

/// Descriptor for `ImportComposeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importComposeRequestDescriptor = $convert.base64Decode(
    'ChRJbXBvcnRDb21wb3NlUmVxdWVzdBIuCg5lbnZpcm9ubWVudF9pZBgBIAEoA0IHukgEIgIgAF'
    'INZW52aXJvbm1lbnRJZBImCgdhcmNoaXZlGAIgASgJQgy6SAlyBxABGICAgAVSB2FyY2hpdmU=');

@$core.Deprecated('Use serviceMutationRequestDescriptor instead')
const ServiceMutationRequest$json = {
  '1': 'ServiceMutationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'services', '3': 2, '4': 3, '5': 9, '8': {}, '10': 'services'},
  ],
};

/// Descriptor for `ServiceMutationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serviceMutationRequestDescriptor =
    $convert.base64Decode(
        'ChZTZXJ2aWNlTXV0YXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBImCghzZX'
        'J2aWNlcxgCIAMoCUIKukgHkgEECAEQZFIIc2VydmljZXM=');

@$core.Deprecated('Use updateComposeResourcesRequestDescriptor instead')
const UpdateComposeResourcesRequest$json = {
  '1': 'UpdateComposeResourcesRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'memory_limit',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'memoryLimit',
      '17': true
    },
    {
      '1': 'cpu_limit',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'cpuLimit',
      '17': true
    },
    {
      '1': 'replicas',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'replicas',
      '17': true
    },
  ],
  '8': [
    {'1': '_memory_limit'},
    {'1': '_cpu_limit'},
    {'1': '_replicas'},
  ],
};

/// Descriptor for `UpdateComposeResourcesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateComposeResourcesRequestDescriptor = $convert.base64Decode(
    'Ch1VcGRhdGVDb21wb3NlUmVzb3VyY2VzUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
    'QSJgoMbWVtb3J5X2xpbWl0GAIgASgJSABSC21lbW9yeUxpbWl0iAEBEiAKCWNwdV9saW1pdBgD'
    'IAEoCUgBUghjcHVMaW1pdIgBARIfCghyZXBsaWNhcxgEIAEoA0gCUghyZXBsaWNhc4gBAUIPCg'
    '1fbWVtb3J5X2xpbWl0QgwKCl9jcHVfbGltaXRCCwoJX3JlcGxpY2Fz');

@$core.Deprecated('Use composeMountRequestDescriptor instead')
const ComposeMountRequest$json = {
  '1': 'ComposeMountRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {
      '1': 'id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'id',
      '17': true
    },
    {'1': 'mount_type', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'mountType'},
    {
      '1': 'host_path',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'hostPath',
      '17': true
    },
    {
      '1': 'volume_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'volumeName',
      '17': true
    },
    {
      '1': 'file_path',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'filePath',
      '17': true
    },
    {
      '1': 'content',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'content',
      '17': true
    },
    {'1': 'mount_path', '3': 8, '4': 1, '5': 9, '8': {}, '10': 'mountPath'},
  ],
  '8': [
    {'1': '_id'},
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `ComposeMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeMountRequestDescriptor = $convert.base64Decode(
    'ChNDb21wb3NlTW91bnRSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIABSCWNvbX'
    'Bvc2VJZBIcCgJpZBgCIAEoA0IHukgEIgIgAEgAUgJpZIgBARImCgptb3VudF90eXBlGAMgASgJ'
    'Qge6SARyAhABUgltb3VudFR5cGUSIAoJaG9zdF9wYXRoGAQgASgJSAFSCGhvc3RQYXRoiAEBEi'
    'QKC3ZvbHVtZV9uYW1lGAUgASgJSAJSCnZvbHVtZU5hbWWIAQESIAoJZmlsZV9wYXRoGAYgASgJ'
    'SANSCGZpbGVQYXRoiAEBEh0KB2NvbnRlbnQYByABKAlIBFIHY29udGVudIgBARImCgptb3VudF'
    '9wYXRoGAggASgJQge6SARyAhABUgltb3VudFBhdGhCBQoDX2lkQgwKCl9ob3N0X3BhdGhCDgoM'
    'X3ZvbHVtZV9uYW1lQgwKCl9maWxlX3BhdGhCCgoIX2NvbnRlbnQ=');

@$core.Deprecated('Use composePatchRequestDescriptor instead')
const ComposePatchRequest$json = {
  '1': 'ComposePatchRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {
      '1': 'id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'id',
      '17': true
    },
    {'1': 'patch_type', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'patchType'},
    {'1': 'payload', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'payload'},
  ],
  '8': [
    {'1': '_id'},
  ],
};

/// Descriptor for `ComposePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composePatchRequestDescriptor = $convert.base64Decode(
    'ChNDb21wb3NlUGF0Y2hSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIABSCWNvbX'
    'Bvc2VJZBIcCgJpZBgCIAEoA0IHukgEIgIgAEgAUgJpZIgBARImCgpwYXRjaF90eXBlGAMgASgJ'
    'Qge6SARyAhABUglwYXRjaFR5cGUSJQoHcGF5bG9hZBgEIAEoCUILukgIcgYQARiAgEBSB3BheW'
    'xvYWRCBQoDX2lk');

@$core.Deprecated('Use composeMountDescriptor instead')
const ComposeMount$json = {
  '1': 'ComposeMount',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'mount_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposeMountType',
      '10': 'mountType'
    },
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
    {'1': 'compose_id', '3': 8, '4': 1, '5': 3, '10': 'composeId'},
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `ComposeMount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeMountDescriptor = $convert.base64Decode(
    'CgxDb21wb3NlTW91bnQSDgoCaWQYASABKANSAmlkEkEKCm1vdW50X3R5cGUYAiABKA4yIi5vcG'
    'Vub3hpZGUuYXBpLnYxLkNvbXBvc2VNb3VudFR5cGVSCW1vdW50VHlwZRIgCglob3N0X3BhdGgY'
    'AyABKAlIAFIIaG9zdFBhdGiIAQESJAoLdm9sdW1lX25hbWUYBCABKAlIAVIKdm9sdW1lTmFtZY'
    'gBARIgCglmaWxlX3BhdGgYBSABKAlIAlIIZmlsZVBhdGiIAQESHQoHY29udGVudBgGIAEoCUgD'
    'Ugdjb250ZW50iAEBEh0KCm1vdW50X3BhdGgYByABKAlSCW1vdW50UGF0aBIdCgpjb21wb3NlX2'
    'lkGAggASgDUgljb21wb3NlSWQSHQoKY3JlYXRlZF9hdBgJIAEoA1IJY3JlYXRlZEF0Eh0KCnVw'
    'ZGF0ZWRfYXQYCiABKANSCXVwZGF0ZWRBdEIMCgpfaG9zdF9wYXRoQg4KDF92b2x1bWVfbmFtZU'
    'IMCgpfZmlsZV9wYXRoQgoKCF9jb250ZW50');

@$core.Deprecated('Use composePatchDescriptor instead')
const ComposePatch$json = {
  '1': 'ComposePatch',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'patch_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposePatchType',
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 3, '4': 1, '5': 9, '10': 'filePath'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '10': 'content'},
    {'1': 'compose_id', '3': 6, '4': 1, '5': 3, '10': 'composeId'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `ComposePatch`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composePatchDescriptor = $convert.base64Decode(
    'CgxDb21wb3NlUGF0Y2gSDgoCaWQYASABKANSAmlkEkEKCnBhdGNoX3R5cGUYAiABKA4yIi5vcG'
    'Vub3hpZGUuYXBpLnYxLkNvbXBvc2VQYXRjaFR5cGVSCXBhdGNoVHlwZRIbCglmaWxlX3BhdGgY'
    'AyABKAlSCGZpbGVQYXRoEhgKB2VuYWJsZWQYBCABKAhSB2VuYWJsZWQSGAoHY29udGVudBgFIA'
    'EoCVIHY29udGVudBIdCgpjb21wb3NlX2lkGAYgASgDUgljb21wb3NlSWQSHQoKY3JlYXRlZF9h'
    'dBgHIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYCCABKANSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use composeListMountsRequestDescriptor instead')
const ComposeListMountsRequest$json = {
  '1': 'ComposeListMountsRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
  ],
};

/// Descriptor for `ComposeListMountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeListMountsRequestDescriptor =
    $convert.base64Decode(
        'ChhDb21wb3NlTGlzdE1vdW50c1JlcXVlc3QSJgoKY29tcG9zZV9pZBgBIAEoA0IHukgEIgIgAF'
        'IJY29tcG9zZUlk');

@$core.Deprecated('Use composeListMountsResponseDescriptor instead')
const ComposeListMountsResponse$json = {
  '1': 'ComposeListMountsResponse',
  '2': [
    {
      '1': 'mounts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ComposeMount',
      '10': 'mounts'
    },
  ],
};

/// Descriptor for `ComposeListMountsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeListMountsResponseDescriptor =
    $convert.base64Decode(
        'ChlDb21wb3NlTGlzdE1vdW50c1Jlc3BvbnNlEjYKBm1vdW50cxgBIAMoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuQ29tcG9zZU1vdW50UgZtb3VudHM=');

@$core.Deprecated('Use composeCreateMountRequestDescriptor instead')
const ComposeCreateMountRequest$json = {
  '1': 'ComposeCreateMountRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {
      '1': 'mount_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposeMountType',
      '8': {},
      '10': 'mountType'
    },
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
      '8': {},
      '9': 3,
      '10': 'content',
      '17': true
    },
    {'1': 'mount_path', '3': 7, '4': 1, '5': 9, '8': {}, '10': 'mountPath'},
  ],
  '8': [
    {'1': '_host_path'},
    {'1': '_volume_name'},
    {'1': '_file_path'},
    {'1': '_content'},
  ],
};

/// Descriptor for `ComposeCreateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeCreateMountRequestDescriptor = $convert.base64Decode(
    'ChlDb21wb3NlQ3JlYXRlTW91bnRSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
    'BSCWNvbXBvc2VJZBJNCgptb3VudF90eXBlGAIgASgOMiIub3Blbm94aWRlLmFwaS52MS5Db21w'
    'b3NlTW91bnRUeXBlQgq6SAeCAQQQASAAUgltb3VudFR5cGUSIAoJaG9zdF9wYXRoGAMgASgJSA'
    'BSCGhvc3RQYXRoiAEBEiQKC3ZvbHVtZV9uYW1lGAQgASgJSAFSCnZvbHVtZU5hbWWIAQESIAoJ'
    'ZmlsZV9wYXRoGAUgASgJSAJSCGZpbGVQYXRoiAEBEigKB2NvbnRlbnQYBiABKAlCCbpIBnIEGI'
    'CAQEgDUgdjb250ZW50iAEBEiYKCm1vdW50X3BhdGgYByABKAlCB7pIBHICEAFSCW1vdW50UGF0'
    'aEIMCgpfaG9zdF9wYXRoQg4KDF92b2x1bWVfbmFtZUIMCgpfZmlsZV9wYXRoQgoKCF9jb250ZW'
    '50');

@$core.Deprecated('Use composeCreateMountResponseDescriptor instead')
const ComposeCreateMountResponse$json = {
  '1': 'ComposeCreateMountResponse',
  '2': [
    {
      '1': 'mount',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ComposeMount',
      '10': 'mount'
    },
  ],
};

/// Descriptor for `ComposeCreateMountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeCreateMountResponseDescriptor =
    $convert.base64Decode(
        'ChpDb21wb3NlQ3JlYXRlTW91bnRSZXNwb25zZRI0CgVtb3VudBgBIAEoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuQ29tcG9zZU1vdW50UgVtb3VudA==');

@$core.Deprecated('Use composeUpdateMountRequestDescriptor instead')
const ComposeUpdateMountRequest$json = {
  '1': 'ComposeUpdateMountRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'mount_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposeMountType',
      '8': {},
      '10': 'mountType'
    },
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
      '8': {},
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

/// Descriptor for `ComposeUpdateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeUpdateMountRequestDescriptor = $convert.base64Decode(
    'ChlDb21wb3NlVXBkYXRlTW91bnRSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
    'BSCWNvbXBvc2VJZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSTQoKbW91bnRfdHlwZRgDIAEo'
    'DjIiLm9wZW5veGlkZS5hcGkudjEuQ29tcG9zZU1vdW50VHlwZUIKukgHggEEEAEgAFIJbW91bn'
    'RUeXBlEiAKCWhvc3RfcGF0aBgEIAEoCUgAUghob3N0UGF0aIgBARIkCgt2b2x1bWVfbmFtZRgF'
    'IAEoCUgBUgp2b2x1bWVOYW1liAEBEiAKCWZpbGVfcGF0aBgGIAEoCUgCUghmaWxlUGF0aIgBAR'
    'IoCgdjb250ZW50GAcgASgJQgm6SAZyBBiAgEBIA1IHY29udGVudIgBARImCgptb3VudF9wYXRo'
    'GAggASgJQge6SARyAhABUgltb3VudFBhdGhCDAoKX2hvc3RfcGF0aEIOCgxfdm9sdW1lX25hbW'
    'VCDAoKX2ZpbGVfcGF0aEIKCghfY29udGVudA==');

@$core.Deprecated('Use composeUpdateMountResponseDescriptor instead')
const ComposeUpdateMountResponse$json = {
  '1': 'ComposeUpdateMountResponse',
  '2': [
    {
      '1': 'mount',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ComposeMount',
      '10': 'mount'
    },
  ],
};

/// Descriptor for `ComposeUpdateMountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeUpdateMountResponseDescriptor =
    $convert.base64Decode(
        'ChpDb21wb3NlVXBkYXRlTW91bnRSZXNwb25zZRI0CgVtb3VudBgBIAEoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuQ29tcG9zZU1vdW50UgVtb3VudA==');

@$core.Deprecated('Use composeListPatchesRequestDescriptor instead')
const ComposeListPatchesRequest$json = {
  '1': 'ComposeListPatchesRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
  ],
};

/// Descriptor for `ComposeListPatchesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeListPatchesRequestDescriptor =
    $convert.base64Decode(
        'ChlDb21wb3NlTGlzdFBhdGNoZXNSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
        'BSCWNvbXBvc2VJZA==');

@$core.Deprecated('Use composeListPatchesResponseDescriptor instead')
const ComposeListPatchesResponse$json = {
  '1': 'ComposeListPatchesResponse',
  '2': [
    {
      '1': 'patches',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ComposePatch',
      '10': 'patches'
    },
  ],
};

/// Descriptor for `ComposeListPatchesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeListPatchesResponseDescriptor =
    $convert.base64Decode(
        'ChpDb21wb3NlTGlzdFBhdGNoZXNSZXNwb25zZRI4CgdwYXRjaGVzGAEgAygLMh4ub3Blbm94aW'
        'RlLmFwaS52MS5Db21wb3NlUGF0Y2hSB3BhdGNoZXM=');

@$core.Deprecated('Use composeCreatePatchRequestDescriptor instead')
const ComposeCreatePatchRequest$json = {
  '1': 'ComposeCreatePatchRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {
      '1': 'patch_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposePatchType',
      '8': {},
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'filePath'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'content'},
  ],
};

/// Descriptor for `ComposeCreatePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeCreatePatchRequestDescriptor = $convert.base64Decode(
    'ChlDb21wb3NlQ3JlYXRlUGF0Y2hSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
    'BSCWNvbXBvc2VJZBJNCgpwYXRjaF90eXBlGAIgASgOMiIub3Blbm94aWRlLmFwaS52MS5Db21w'
    'b3NlUGF0Y2hUeXBlQgq6SAeCAQQQASAAUglwYXRjaFR5cGUSJAoJZmlsZV9wYXRoGAMgASgJQg'
    'e6SARyAhABUghmaWxlUGF0aBIYCgdlbmFibGVkGAQgASgIUgdlbmFibGVkEiMKB2NvbnRlbnQY'
    'BSABKAlCCbpIBnIEGICAQFIHY29udGVudA==');

@$core.Deprecated('Use composeCreatePatchResponseDescriptor instead')
const ComposeCreatePatchResponse$json = {
  '1': 'ComposeCreatePatchResponse',
  '2': [
    {
      '1': 'patch',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ComposePatch',
      '10': 'patch'
    },
  ],
};

/// Descriptor for `ComposeCreatePatchResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeCreatePatchResponseDescriptor =
    $convert.base64Decode(
        'ChpDb21wb3NlQ3JlYXRlUGF0Y2hSZXNwb25zZRI0CgVwYXRjaBgBIAEoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuQ29tcG9zZVBhdGNoUgVwYXRjaA==');

@$core.Deprecated('Use composeUpdatePatchRequestDescriptor instead')
const ComposeUpdatePatchRequest$json = {
  '1': 'ComposeUpdatePatchRequest',
  '2': [
    {'1': 'compose_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'composeId'},
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'patch_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ComposePatchType',
      '8': {},
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'filePath'},
    {'1': 'enabled', '3': 5, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 6, '4': 1, '5': 9, '8': {}, '10': 'content'},
  ],
};

/// Descriptor for `ComposeUpdatePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeUpdatePatchRequestDescriptor = $convert.base64Decode(
    'ChlDb21wb3NlVXBkYXRlUGF0Y2hSZXF1ZXN0EiYKCmNvbXBvc2VfaWQYASABKANCB7pIBCICIA'
    'BSCWNvbXBvc2VJZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSTQoKcGF0Y2hfdHlwZRgDIAEo'
    'DjIiLm9wZW5veGlkZS5hcGkudjEuQ29tcG9zZVBhdGNoVHlwZUIKukgHggEEEAEgAFIJcGF0Y2'
    'hUeXBlEiQKCWZpbGVfcGF0aBgEIAEoCUIHukgEcgIQAVIIZmlsZVBhdGgSGAoHZW5hYmxlZBgF'
    'IAEoCFIHZW5hYmxlZBIjCgdjb250ZW50GAYgASgJQgm6SAZyBBiAgEBSB2NvbnRlbnQ=');

@$core.Deprecated('Use composeUpdatePatchResponseDescriptor instead')
const ComposeUpdatePatchResponse$json = {
  '1': 'ComposeUpdatePatchResponse',
  '2': [
    {
      '1': 'patch',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ComposePatch',
      '10': 'patch'
    },
  ],
};

/// Descriptor for `ComposeUpdatePatchResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeUpdatePatchResponseDescriptor =
    $convert.base64Decode(
        'ChpDb21wb3NlVXBkYXRlUGF0Y2hSZXNwb25zZRI0CgVwYXRjaBgBIAEoCzIeLm9wZW5veGlkZS'
        '5hcGkudjEuQ29tcG9zZVBhdGNoUgVwYXRjaA==');

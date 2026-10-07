// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/application.proto.

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

@$core.Deprecated('Use applicationSourceTypeDescriptor instead')
const ApplicationSourceType$json = {
  '1': 'ApplicationSourceType',
  '2': [
    {'1': 'APPLICATION_SOURCE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_SOURCE_TYPE_GITHUB', '2': 1},
    {'1': 'APPLICATION_SOURCE_TYPE_GITLAB', '2': 2},
    {'1': 'APPLICATION_SOURCE_TYPE_BITBUCKET', '2': 3},
    {'1': 'APPLICATION_SOURCE_TYPE_GITEA', '2': 4},
    {'1': 'APPLICATION_SOURCE_TYPE_CUSTOM_GIT', '2': 5},
    {'1': 'APPLICATION_SOURCE_TYPE_DOCKER', '2': 6},
    {'1': 'APPLICATION_SOURCE_TYPE_DROP', '2': 7},
  ],
};

/// Descriptor for `ApplicationSourceType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationSourceTypeDescriptor = $convert.base64Decode(
    'ChVBcHBsaWNhdGlvblNvdXJjZVR5cGUSJwojQVBQTElDQVRJT05fU09VUkNFX1RZUEVfVU5TUE'
    'VDSUZJRUQQABIiCh5BUFBMSUNBVElPTl9TT1VSQ0VfVFlQRV9HSVRIVUIQARIiCh5BUFBMSUNB'
    'VElPTl9TT1VSQ0VfVFlQRV9HSVRMQUIQAhIlCiFBUFBMSUNBVElPTl9TT1VSQ0VfVFlQRV9CSV'
    'RCVUNLRVQQAxIhCh1BUFBMSUNBVElPTl9TT1VSQ0VfVFlQRV9HSVRFQRAEEiYKIkFQUExJQ0FU'
    'SU9OX1NPVVJDRV9UWVBFX0NVU1RPTV9HSVQQBRIiCh5BUFBMSUNBVElPTl9TT1VSQ0VfVFlQRV'
    '9ET0NLRVIQBhIgChxBUFBMSUNBVElPTl9TT1VSQ0VfVFlQRV9EUk9QEAc=');

@$core.Deprecated('Use applicationBuildTypeDescriptor instead')
const ApplicationBuildType$json = {
  '1': 'ApplicationBuildType',
  '2': [
    {'1': 'APPLICATION_BUILD_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_BUILD_TYPE_BUILDPACKS', '2': 1},
    {'1': 'APPLICATION_BUILD_TYPE_DOCKERFILE', '2': 2},
    {'1': 'APPLICATION_BUILD_TYPE_NIXPACKS', '2': 3},
    {'1': 'APPLICATION_BUILD_TYPE_HEROKU', '2': 4},
    {'1': 'APPLICATION_BUILD_TYPE_PAKETO', '2': 5},
    {'1': 'APPLICATION_BUILD_TYPE_RAILPACK', '2': 6},
    {'1': 'APPLICATION_BUILD_TYPE_STATIC', '2': 7},
  ],
};

/// Descriptor for `ApplicationBuildType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationBuildTypeDescriptor = $convert.base64Decode(
    'ChRBcHBsaWNhdGlvbkJ1aWxkVHlwZRImCiJBUFBMSUNBVElPTl9CVUlMRF9UWVBFX1VOU1BFQ0'
    'lGSUVEEAASJQohQVBQTElDQVRJT05fQlVJTERfVFlQRV9CVUlMRFBBQ0tTEAESJQohQVBQTElD'
    'QVRJT05fQlVJTERfVFlQRV9ET0NLRVJGSUxFEAISIwofQVBQTElDQVRJT05fQlVJTERfVFlQRV'
    '9OSVhQQUNLUxADEiEKHUFQUExJQ0FUSU9OX0JVSUxEX1RZUEVfSEVST0tVEAQSIQodQVBQTElD'
    'QVRJT05fQlVJTERfVFlQRV9QQUtFVE8QBRIjCh9BUFBMSUNBVElPTl9CVUlMRF9UWVBFX1JBSU'
    'xQQUNLEAYSIQodQVBQTElDQVRJT05fQlVJTERfVFlQRV9TVEFUSUMQBw==');

@$core.Deprecated('Use applicationPortProtocolDescriptor instead')
const ApplicationPortProtocol$json = {
  '1': 'ApplicationPortProtocol',
  '2': [
    {'1': 'APPLICATION_PORT_PROTOCOL_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_PORT_PROTOCOL_TCP', '2': 1},
    {'1': 'APPLICATION_PORT_PROTOCOL_UDP', '2': 2},
  ],
};

/// Descriptor for `ApplicationPortProtocol`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationPortProtocolDescriptor = $convert.base64Decode(
    'ChdBcHBsaWNhdGlvblBvcnRQcm90b2NvbBIpCiVBUFBMSUNBVElPTl9QT1JUX1BST1RPQ09MX1'
    'VOU1BFQ0lGSUVEEAASIQodQVBQTElDQVRJT05fUE9SVF9QUk9UT0NPTF9UQ1AQARIhCh1BUFBM'
    'SUNBVElPTl9QT1JUX1BST1RPQ09MX1VEUBAC');

@$core.Deprecated('Use applicationPortPublishModeDescriptor instead')
const ApplicationPortPublishMode$json = {
  '1': 'ApplicationPortPublishMode',
  '2': [
    {'1': 'APPLICATION_PORT_PUBLISH_MODE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_PORT_PUBLISH_MODE_HOST', '2': 1},
    {'1': 'APPLICATION_PORT_PUBLISH_MODE_INGRESS', '2': 2},
  ],
};

/// Descriptor for `ApplicationPortPublishMode`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationPortPublishModeDescriptor =
    $convert.base64Decode(
        'ChpBcHBsaWNhdGlvblBvcnRQdWJsaXNoTW9kZRItCilBUFBMSUNBVElPTl9QT1JUX1BVQkxJU0'
        'hfTU9ERV9VTlNQRUNJRklFRBAAEiYKIkFQUExJQ0FUSU9OX1BPUlRfUFVCTElTSF9NT0RFX0hP'
        'U1QQARIpCiVBUFBMSUNBVElPTl9QT1JUX1BVQkxJU0hfTU9ERV9JTkdSRVNTEAI=');

@$core.Deprecated('Use applicationMountTypeDescriptor instead')
const ApplicationMountType$json = {
  '1': 'ApplicationMountType',
  '2': [
    {'1': 'APPLICATION_MOUNT_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_MOUNT_TYPE_VOLUME', '2': 1},
    {'1': 'APPLICATION_MOUNT_TYPE_BIND', '2': 2},
    {'1': 'APPLICATION_MOUNT_TYPE_FILE', '2': 3},
  ],
};

/// Descriptor for `ApplicationMountType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationMountTypeDescriptor = $convert.base64Decode(
    'ChRBcHBsaWNhdGlvbk1vdW50VHlwZRImCiJBUFBMSUNBVElPTl9NT1VOVF9UWVBFX1VOU1BFQ0'
    'lGSUVEEAASIQodQVBQTElDQVRJT05fTU9VTlRfVFlQRV9WT0xVTUUQARIfChtBUFBMSUNBVElP'
    'Tl9NT1VOVF9UWVBFX0JJTkQQAhIfChtBUFBMSUNBVElPTl9NT1VOVF9UWVBFX0ZJTEUQAw==');

@$core.Deprecated('Use applicationPatchTypeDescriptor instead')
const ApplicationPatchType$json = {
  '1': 'ApplicationPatchType',
  '2': [
    {'1': 'APPLICATION_PATCH_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_PATCH_TYPE_CREATE', '2': 1},
    {'1': 'APPLICATION_PATCH_TYPE_UPDATE', '2': 2},
    {'1': 'APPLICATION_PATCH_TYPE_DELETE', '2': 3},
  ],
};

/// Descriptor for `ApplicationPatchType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationPatchTypeDescriptor = $convert.base64Decode(
    'ChRBcHBsaWNhdGlvblBhdGNoVHlwZRImCiJBUFBMSUNBVElPTl9QQVRDSF9UWVBFX1VOU1BFQ0'
    'lGSUVEEAASIQodQVBQTElDQVRJT05fUEFUQ0hfVFlQRV9DUkVBVEUQARIhCh1BUFBMSUNBVElP'
    'Tl9QQVRDSF9UWVBFX1VQREFURRACEiEKHUFQUExJQ0FUSU9OX1BBVENIX1RZUEVfREVMRVRFEA'
    'M=');

@$core.Deprecated('Use applicationMiddlewareTypeDescriptor instead')
const ApplicationMiddlewareType$json = {
  '1': 'ApplicationMiddlewareType',
  '2': [
    {'1': 'APPLICATION_MIDDLEWARE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'APPLICATION_MIDDLEWARE_TYPE_COMPRESS', '2': 1},
    {'1': 'APPLICATION_MIDDLEWARE_TYPE_HEADERS', '2': 2},
    {'1': 'APPLICATION_MIDDLEWARE_TYPE_RATE_LIMIT', '2': 3},
    {'1': 'APPLICATION_MIDDLEWARE_TYPE_IP_ALLOWLIST', '2': 4},
  ],
};

/// Descriptor for `ApplicationMiddlewareType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List applicationMiddlewareTypeDescriptor = $convert.base64Decode(
    'ChlBcHBsaWNhdGlvbk1pZGRsZXdhcmVUeXBlEisKJ0FQUExJQ0FUSU9OX01JRERMRVdBUkVfVF'
    'lQRV9VTlNQRUNJRklFRBAAEigKJEFQUExJQ0FUSU9OX01JRERMRVdBUkVfVFlQRV9DT01QUkVT'
    'UxABEicKI0FQUExJQ0FUSU9OX01JRERMRVdBUkVfVFlQRV9IRUFERVJTEAISKgomQVBQTElDQV'
    'RJT05fTUlERExFV0FSRV9UWVBFX1JBVEVfTElNSVQQAxIsCihBUFBMSUNBVElPTl9NSURETEVX'
    'QVJFX1RZUEVfSVBfQUxMT1dMSVNUEAQ=');

@$core.Deprecated('Use applicationDescriptor instead')
const Application$json = {
  '1': 'Application',
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
    {'1': 'app_name', '3': 4, '4': 1, '5': 9, '10': 'appName'},
    {
      '1': 'source_type',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationSourceType',
      '10': 'sourceType'
    },
    {
      '1': 'build_type',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationBuildType',
      '10': 'buildType'
    },
    {'1': 'status', '3': 7, '4': 1, '5': 9, '10': 'status'},
    {
      '1': 'last_error',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'lastError',
      '17': true
    },
    {'1': 'trigger_type', '3': 9, '4': 1, '5': 9, '10': 'triggerType'},
    {'1': 'environment_id', '3': 10, '4': 1, '5': 3, '10': 'environmentId'},
    {
      '1': 'server_id',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'build_server_id',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'buildServerId',
      '17': true
    },
    {
      '1': 'registry_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'registryId',
      '17': true
    },
    {'1': 'network_ids', '3': 14, '4': 3, '5': 9, '10': 'networkIds'},
    {
      '1': 'detach_rustploy_network',
      '3': 15,
      '4': 1,
      '5': 8,
      '10': 'detachRustployNetwork'
    },
    {
      '1': 'env_var',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'envVar',
      '17': true
    },
    {'1': 'icon', '3': 17, '4': 1, '5': 9, '9': 6, '10': 'icon', '17': true},
    {
      '1': 'repository',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'repository',
      '17': true
    },
    {'1': 'owner', '3': 19, '4': 1, '5': 9, '9': 8, '10': 'owner', '17': true},
    {
      '1': 'branch',
      '3': 20,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'branch',
      '17': true
    },
    {
      '1': 'docker_image',
      '3': 21,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'dockerImage',
      '17': true
    },
    {
      '1': 'registry_url',
      '3': 22,
      '4': 1,
      '5': 9,
      '9': 11,
      '10': 'registryUrl',
      '17': true
    },
    {
      '1': 'custom_git_url',
      '3': 23,
      '4': 1,
      '5': 9,
      '9': 12,
      '10': 'customGitUrl',
      '17': true
    },
    {
      '1': 'custom_git_branch',
      '3': 24,
      '4': 1,
      '5': 9,
      '9': 13,
      '10': 'customGitBranch',
      '17': true
    },
    {
      '1': 'preview_wildcard',
      '3': 25,
      '4': 1,
      '5': 9,
      '9': 14,
      '10': 'previewWildcard',
      '17': true
    },
    {
      '1': 'preview_port',
      '3': 26,
      '4': 1,
      '5': 3,
      '9': 15,
      '10': 'previewPort',
      '17': true
    },
    {'1': 'created_at', '3': 27, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 28, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'dockerfile',
      '3': 29,
      '4': 1,
      '5': 9,
      '9': 16,
      '10': 'dockerfile',
      '17': true
    },
    {
      '1': 'docker_context_path',
      '3': 30,
      '4': 1,
      '5': 9,
      '9': 17,
      '10': 'dockerContextPath',
      '17': true
    },
    {
      '1': 'docker_build_stage',
      '3': 31,
      '4': 1,
      '5': 9,
      '9': 18,
      '10': 'dockerBuildStage',
      '17': true
    },
    {
      '1': 'publish_directory',
      '3': 32,
      '4': 1,
      '5': 9,
      '9': 19,
      '10': 'publishDirectory',
      '17': true
    },
    {
      '1': 'is_static_spa',
      '3': 33,
      '4': 1,
      '5': 8,
      '9': 20,
      '10': 'isStaticSpa',
      '17': true
    },
    {
      '1': 'railpack_version',
      '3': 34,
      '4': 1,
      '5': 9,
      '9': 21,
      '10': 'railpackVersion',
      '17': true
    },
    {
      '1': 'heroku_version',
      '3': 35,
      '4': 1,
      '5': 9,
      '9': 22,
      '10': 'herokuVersion',
      '17': true
    },
    {
      '1': 'build_path',
      '3': 36,
      '4': 1,
      '5': 9,
      '9': 23,
      '10': 'buildPath',
      '17': true
    },
    {
      '1': 'drop_build_path',
      '3': 37,
      '4': 1,
      '5': 9,
      '9': 24,
      '10': 'dropBuildPath',
      '17': true
    },
    {
      '1': 'clean_cache',
      '3': 38,
      '4': 1,
      '5': 8,
      '9': 25,
      '10': 'cleanCache',
      '17': true
    },
    {
      '1': 'auto_deploy',
      '3': 39,
      '4': 1,
      '5': 8,
      '9': 26,
      '10': 'autoDeploy',
      '17': true
    },
    {
      '1': 'memory_reservation',
      '3': 40,
      '4': 1,
      '5': 9,
      '9': 27,
      '10': 'memoryReservation',
      '17': true
    },
    {
      '1': 'memory_limit',
      '3': 41,
      '4': 1,
      '5': 9,
      '9': 28,
      '10': 'memoryLimit',
      '17': true
    },
    {
      '1': 'cpu_reservation',
      '3': 42,
      '4': 1,
      '5': 9,
      '9': 29,
      '10': 'cpuReservation',
      '17': true
    },
    {
      '1': 'cpu_limit',
      '3': 43,
      '4': 1,
      '5': 9,
      '9': 30,
      '10': 'cpuLimit',
      '17': true
    },
    {
      '1': 'replicas',
      '3': 44,
      '4': 1,
      '5': 3,
      '9': 31,
      '10': 'replicas',
      '17': true
    },
    {
      '1': 'command',
      '3': 45,
      '4': 1,
      '5': 9,
      '9': 32,
      '10': 'command',
      '17': true
    },
    {'1': 'args', '3': 46, '4': 1, '5': 9, '9': 33, '10': 'args', '17': true},
    {
      '1': 'watch_paths',
      '3': 47,
      '4': 1,
      '5': 9,
      '9': 34,
      '10': 'watchPaths',
      '17': true
    },
    {
      '1': 'enable_submodules',
      '3': 48,
      '4': 1,
      '5': 8,
      '9': 35,
      '10': 'enableSubmodules',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_last_error'},
    {'1': '_server_id'},
    {'1': '_build_server_id'},
    {'1': '_registry_id'},
    {'1': '_env_var'},
    {'1': '_icon'},
    {'1': '_repository'},
    {'1': '_owner'},
    {'1': '_branch'},
    {'1': '_docker_image'},
    {'1': '_registry_url'},
    {'1': '_custom_git_url'},
    {'1': '_custom_git_branch'},
    {'1': '_preview_wildcard'},
    {'1': '_preview_port'},
    {'1': '_dockerfile'},
    {'1': '_docker_context_path'},
    {'1': '_docker_build_stage'},
    {'1': '_publish_directory'},
    {'1': '_is_static_spa'},
    {'1': '_railpack_version'},
    {'1': '_heroku_version'},
    {'1': '_build_path'},
    {'1': '_drop_build_path'},
    {'1': '_clean_cache'},
    {'1': '_auto_deploy'},
    {'1': '_memory_reservation'},
    {'1': '_memory_limit'},
    {'1': '_cpu_reservation'},
    {'1': '_cpu_limit'},
    {'1': '_replicas'},
    {'1': '_command'},
    {'1': '_args'},
    {'1': '_watch_paths'},
    {'1': '_enable_submodules'},
  ],
};

/// Descriptor for `Application`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDescriptor = $convert.base64Decode(
    'CgtBcHBsaWNhdGlvbhIOCgJpZBgBIAEoA1ICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIlCgtkZX'
    'NjcmlwdGlvbhgDIAEoCUgAUgtkZXNjcmlwdGlvbogBARIZCghhcHBfbmFtZRgEIAEoCVIHYXBw'
    'TmFtZRJICgtzb3VyY2VfdHlwZRgFIAEoDjInLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRpb2'
    '5Tb3VyY2VUeXBlUgpzb3VyY2VUeXBlEkUKCmJ1aWxkX3R5cGUYBiABKA4yJi5vcGVub3hpZGUu'
    'YXBpLnYxLkFwcGxpY2F0aW9uQnVpbGRUeXBlUglidWlsZFR5cGUSFgoGc3RhdHVzGAcgASgJUg'
    'ZzdGF0dXMSIgoKbGFzdF9lcnJvchgIIAEoCUgBUglsYXN0RXJyb3KIAQESIQoMdHJpZ2dlcl90'
    'eXBlGAkgASgJUgt0cmlnZ2VyVHlwZRIlCg5lbnZpcm9ubWVudF9pZBgKIAEoA1INZW52aXJvbm'
    '1lbnRJZBIgCglzZXJ2ZXJfaWQYCyABKANIAlIIc2VydmVySWSIAQESKwoPYnVpbGRfc2VydmVy'
    'X2lkGAwgASgDSANSDWJ1aWxkU2VydmVySWSIAQESJAoLcmVnaXN0cnlfaWQYDSABKANIBFIKcm'
    'VnaXN0cnlJZIgBARIfCgtuZXR3b3JrX2lkcxgOIAMoCVIKbmV0d29ya0lkcxI2ChdkZXRhY2hf'
    'cnVzdHBsb3lfbmV0d29yaxgPIAEoCFIVZGV0YWNoUnVzdHBsb3lOZXR3b3JrEhwKB2Vudl92YX'
    'IYECABKAlIBVIGZW52VmFyiAEBEhcKBGljb24YESABKAlIBlIEaWNvbogBARIjCgpyZXBvc2l0'
    'b3J5GBIgASgJSAdSCnJlcG9zaXRvcnmIAQESGQoFb3duZXIYEyABKAlICFIFb3duZXKIAQESGw'
    'oGYnJhbmNoGBQgASgJSAlSBmJyYW5jaIgBARImCgxkb2NrZXJfaW1hZ2UYFSABKAlIClILZG9j'
    'a2VySW1hZ2WIAQESJgoMcmVnaXN0cnlfdXJsGBYgASgJSAtSC3JlZ2lzdHJ5VXJsiAEBEikKDm'
    'N1c3RvbV9naXRfdXJsGBcgASgJSAxSDGN1c3RvbUdpdFVybIgBARIvChFjdXN0b21fZ2l0X2Jy'
    'YW5jaBgYIAEoCUgNUg9jdXN0b21HaXRCcmFuY2iIAQESLgoQcHJldmlld193aWxkY2FyZBgZIA'
    'EoCUgOUg9wcmV2aWV3V2lsZGNhcmSIAQESJgoMcHJldmlld19wb3J0GBogASgDSA9SC3ByZXZp'
    'ZXdQb3J0iAEBEh0KCmNyZWF0ZWRfYXQYGyABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GB'
    'wgASgDUgl1cGRhdGVkQXQSIwoKZG9ja2VyZmlsZRgdIAEoCUgQUgpkb2NrZXJmaWxliAEBEjMK'
    'E2RvY2tlcl9jb250ZXh0X3BhdGgYHiABKAlIEVIRZG9ja2VyQ29udGV4dFBhdGiIAQESMQoSZG'
    '9ja2VyX2J1aWxkX3N0YWdlGB8gASgJSBJSEGRvY2tlckJ1aWxkU3RhZ2WIAQESMAoRcHVibGlz'
    'aF9kaXJlY3RvcnkYICABKAlIE1IQcHVibGlzaERpcmVjdG9yeYgBARInCg1pc19zdGF0aWNfc3'
    'BhGCEgASgISBRSC2lzU3RhdGljU3BhiAEBEi4KEHJhaWxwYWNrX3ZlcnNpb24YIiABKAlIFVIP'
    'cmFpbHBhY2tWZXJzaW9uiAEBEioKDmhlcm9rdV92ZXJzaW9uGCMgASgJSBZSDWhlcm9rdVZlcn'
    'Npb26IAQESIgoKYnVpbGRfcGF0aBgkIAEoCUgXUglidWlsZFBhdGiIAQESKwoPZHJvcF9idWls'
    'ZF9wYXRoGCUgASgJSBhSDWRyb3BCdWlsZFBhdGiIAQESJAoLY2xlYW5fY2FjaGUYJiABKAhIGV'
    'IKY2xlYW5DYWNoZYgBARIkCgthdXRvX2RlcGxveRgnIAEoCEgaUgphdXRvRGVwbG95iAEBEjIK'
    'Em1lbW9yeV9yZXNlcnZhdGlvbhgoIAEoCUgbUhFtZW1vcnlSZXNlcnZhdGlvbogBARImCgxtZW'
    '1vcnlfbGltaXQYKSABKAlIHFILbWVtb3J5TGltaXSIAQESLAoPY3B1X3Jlc2VydmF0aW9uGCog'
    'ASgJSB1SDmNwdVJlc2VydmF0aW9uiAEBEiAKCWNwdV9saW1pdBgrIAEoCUgeUghjcHVMaW1pdI'
    'gBARIfCghyZXBsaWNhcxgsIAEoA0gfUghyZXBsaWNhc4gBARIdCgdjb21tYW5kGC0gASgJSCBS'
    'B2NvbW1hbmSIAQESFwoEYXJncxguIAEoCUghUgRhcmdziAEBEiQKC3dhdGNoX3BhdGhzGC8gAS'
    'gJSCJSCndhdGNoUGF0aHOIAQESMAoRZW5hYmxlX3N1Ym1vZHVsZXMYMCABKAhII1IQZW5hYmxl'
    'U3VibW9kdWxlc4gBAUIOCgxfZGVzY3JpcHRpb25CDQoLX2xhc3RfZXJyb3JCDAoKX3NlcnZlcl'
    '9pZEISChBfYnVpbGRfc2VydmVyX2lkQg4KDF9yZWdpc3RyeV9pZEIKCghfZW52X3ZhckIHCgVf'
    'aWNvbkINCgtfcmVwb3NpdG9yeUIICgZfb3duZXJCCQoHX2JyYW5jaEIPCg1fZG9ja2VyX2ltYW'
    'dlQg8KDV9yZWdpc3RyeV91cmxCEQoPX2N1c3RvbV9naXRfdXJsQhQKEl9jdXN0b21fZ2l0X2Jy'
    'YW5jaEITChFfcHJldmlld193aWxkY2FyZEIPCg1fcHJldmlld19wb3J0Qg0KC19kb2NrZXJmaW'
    'xlQhYKFF9kb2NrZXJfY29udGV4dF9wYXRoQhUKE19kb2NrZXJfYnVpbGRfc3RhZ2VCFAoSX3B1'
    'Ymxpc2hfZGlyZWN0b3J5QhAKDl9pc19zdGF0aWNfc3BhQhMKEV9yYWlscGFja192ZXJzaW9uQh'
    'EKD19oZXJva3VfdmVyc2lvbkINCgtfYnVpbGRfcGF0aEISChBfZHJvcF9idWlsZF9wYXRoQg4K'
    'DF9jbGVhbl9jYWNoZUIOCgxfYXV0b19kZXBsb3lCFQoTX21lbW9yeV9yZXNlcnZhdGlvbkIPCg'
    '1fbWVtb3J5X2xpbWl0QhIKEF9jcHVfcmVzZXJ2YXRpb25CDAoKX2NwdV9saW1pdEILCglfcmVw'
    'bGljYXNCCgoIX2NvbW1hbmRCBwoFX2FyZ3NCDgoMX3dhdGNoX3BhdGhzQhQKEl9lbmFibGVfc3'
    'VibW9kdWxlcw==');

@$core.Deprecated('Use getApplicationRequestDescriptor instead')
const GetApplicationRequest$json = {
  '1': 'GetApplicationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getApplicationRequestDescriptor =
    $convert.base64Decode(
        'ChVHZXRBcHBsaWNhdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use listApplicationsRequestDescriptor instead')
const ListApplicationsRequest$json = {
  '1': 'ListApplicationsRequest',
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

/// Descriptor for `ListApplicationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listApplicationsRequestDescriptor =
    $convert.base64Decode(
        'ChdMaXN0QXBwbGljYXRpb25zUmVxdWVzdBIuCg5lbnZpcm9ubWVudF9pZBgBIAEoA0IHukgEIg'
        'IgAFINZW52aXJvbm1lbnRJZA==');

@$core.Deprecated('Use listApplicationsResponseDescriptor instead')
const ListApplicationsResponse$json = {
  '1': 'ListApplicationsResponse',
  '2': [
    {
      '1': 'applications',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Application',
      '10': 'applications'
    },
  ],
};

/// Descriptor for `ListApplicationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listApplicationsResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0QXBwbGljYXRpb25zUmVzcG9uc2USQQoMYXBwbGljYXRpb25zGAEgAygLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5BcHBsaWNhdGlvblIMYXBwbGljYXRpb25z');

@$core.Deprecated('Use createApplicationRequestDescriptor instead')
const CreateApplicationRequest$json = {
  '1': 'CreateApplicationRequest',
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
      '8': {},
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'source_type',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationSourceType',
      '10': 'sourceType'
    },
    {
      '1': 'build_type',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationBuildType',
      '10': 'buildType'
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_server_id'},
  ],
};

/// Descriptor for `CreateApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createApplicationRequestDescriptor = $convert.base64Decode(
    'ChhDcmVhdGVBcHBsaWNhdGlvblJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AVIEbm'
    'FtZRIlCgtkZXNjcmlwdGlvbhgCIAEoCUgAUgtkZXNjcmlwdGlvbogBARIuCg5lbnZpcm9ubWVu'
    'dF9pZBgDIAEoA0IHukgEIgIgAFINZW52aXJvbm1lbnRJZBIpCglzZXJ2ZXJfaWQYBCABKANCB7'
    'pIBCICIABIAVIIc2VydmVySWSIAQESSAoLc291cmNlX3R5cGUYBSABKA4yJy5vcGVub3hpZGUu'
    'YXBpLnYxLkFwcGxpY2F0aW9uU291cmNlVHlwZVIKc291cmNlVHlwZRJFCgpidWlsZF90eXBlGA'
    'YgASgOMiYub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbkJ1aWxkVHlwZVIJYnVpbGRUeXBl'
    'Qg4KDF9kZXNjcmlwdGlvbkIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use updateApplicationRequestDescriptor instead')
const UpdateApplicationRequest$json = {
  '1': 'UpdateApplicationRequest',
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
      '1': 'build_type',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationBuildType',
      '9': 2,
      '10': 'buildType',
      '17': true
    },
    {
      '1': 'trigger_type',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'triggerType',
      '17': true
    },
    {
      '1': 'env_var',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'envVar',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'build_server_id',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 6,
      '10': 'buildServerId',
      '17': true
    },
    {
      '1': 'registry_id',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 7,
      '10': 'registryId',
      '17': true
    },
    {'1': 'network_ids', '3': 10, '4': 3, '5': 9, '10': 'networkIds'},
    {
      '1': 'detach_rustploy_network',
      '3': 11,
      '4': 1,
      '5': 8,
      '9': 8,
      '10': 'detachRustployNetwork',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_build_type'},
    {'1': '_trigger_type'},
    {'1': '_env_var'},
    {'1': '_server_id'},
    {'1': '_build_server_id'},
    {'1': '_registry_id'},
    {'1': '_detach_rustploy_network'},
  ],
};

/// Descriptor for `UpdateApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateApplicationRequestDescriptor = $convert.base64Decode(
    'ChhVcGRhdGVBcHBsaWNhdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEhcKBG'
    '5hbWUYAiABKAlIAFIEbmFtZYgBARIlCgtkZXNjcmlwdGlvbhgDIAEoCUgBUgtkZXNjcmlwdGlv'
    'bogBARJKCgpidWlsZF90eXBlGAQgASgOMiYub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk'
    'J1aWxkVHlwZUgCUglidWlsZFR5cGWIAQESJgoMdHJpZ2dlcl90eXBlGAUgASgJSANSC3RyaWdn'
    'ZXJUeXBliAEBEhwKB2Vudl92YXIYBiABKAlIBFIGZW52VmFyiAEBEiAKCXNlcnZlcl9pZBgHIA'
    'EoA0gFUghzZXJ2ZXJJZIgBARIrCg9idWlsZF9zZXJ2ZXJfaWQYCCABKANIBlINYnVpbGRTZXJ2'
    'ZXJJZIgBARIkCgtyZWdpc3RyeV9pZBgJIAEoA0gHUgpyZWdpc3RyeUlkiAEBEh8KC25ldHdvcm'
    'tfaWRzGAogAygJUgpuZXR3b3JrSWRzEjsKF2RldGFjaF9ydXN0cGxveV9uZXR3b3JrGAsgASgI'
    'SAhSFWRldGFjaFJ1c3RwbG95TmV0d29ya4gBAUIHCgVfbmFtZUIOCgxfZGVzY3JpcHRpb25CDQ'
    'oLX2J1aWxkX3R5cGVCDwoNX3RyaWdnZXJfdHlwZUIKCghfZW52X3ZhckIMCgpfc2VydmVyX2lk'
    'QhIKEF9idWlsZF9zZXJ2ZXJfaWRCDgoMX3JlZ2lzdHJ5X2lkQhoKGF9kZXRhY2hfcnVzdHBsb3'
    'lfbmV0d29yaw==');

@$core.Deprecated('Use deleteApplicationRequestDescriptor instead')
const DeleteApplicationRequest$json = {
  '1': 'DeleteApplicationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteApplicationRequestDescriptor =
    $convert.base64Decode(
        'ChhEZWxldGVBcHBsaWNhdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use importApplicationRequestDescriptor instead')
const ImportApplicationRequest$json = {
  '1': 'ImportApplicationRequest',
  '2': [
    {'1': 'archive', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'archive'},
  ],
};

/// Descriptor for `ImportApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importApplicationRequestDescriptor =
    $convert.base64Decode(
        'ChhJbXBvcnRBcHBsaWNhdGlvblJlcXVlc3QSJgoHYXJjaGl2ZRgBIAEoCUIMukgJcgcQARiAgI'
        'AFUgdhcmNoaXZl');

@$core.Deprecated('Use exportApplicationRequestDescriptor instead')
const ExportApplicationRequest$json = {
  '1': 'ExportApplicationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'include_secrets', '3': 2, '4': 1, '5': 8, '10': 'includeSecrets'},
  ],
};

/// Descriptor for `ExportApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exportApplicationRequestDescriptor =
    $convert.base64Decode(
        'ChhFeHBvcnRBcHBsaWNhdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEicKD2'
        'luY2x1ZGVfc2VjcmV0cxgCIAEoCFIOaW5jbHVkZVNlY3JldHM=');

@$core.Deprecated('Use applicationExportDescriptor instead')
const ApplicationExport$json = {
  '1': 'ApplicationExport',
  '2': [
    {'1': 'format', '3': 1, '4': 1, '5': 9, '10': 'format'},
    {'1': 'schema_version', '3': 2, '4': 1, '5': 3, '10': 'schemaVersion'},
    {'1': 'archive', '3': 3, '4': 1, '5': 9, '10': 'archive'},
  ],
};

/// Descriptor for `ApplicationExport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationExportDescriptor = $convert.base64Decode(
    'ChFBcHBsaWNhdGlvbkV4cG9ydBIWCgZmb3JtYXQYASABKAlSBmZvcm1hdBIlCg5zY2hlbWFfdm'
    'Vyc2lvbhgCIAEoA1INc2NoZW1hVmVyc2lvbhIYCgdhcmNoaXZlGAMgASgJUgdhcmNoaXZl');

@$core.Deprecated('Use setGithubSourceRequestDescriptor instead')
const SetGithubSourceRequest$json = {
  '1': 'SetGithubSourceRequest',
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
      '1': 'auto_deploy',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 3,
      '10': 'autoDeploy',
      '17': true
    },
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_provider_id'},
    {'1': '_auto_deploy'},
  ],
};

/// Descriptor for `SetGithubSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setGithubSourceRequestDescriptor = $convert.base64Decode(
    'ChZTZXRHaXRodWJTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBInCgpyZX'
    'Bvc2l0b3J5GAIgASgJQge6SARyAhABUgpyZXBvc2l0b3J5Eh0KBW93bmVyGAMgASgJQge6SARy'
    'AhABUgVvd25lchIbCgZicmFuY2gYBCABKAlIAFIGYnJhbmNoiAEBEiIKCmJ1aWxkX3BhdGgYBS'
    'ABKAlIAVIJYnVpbGRQYXRoiAEBEiQKC3Byb3ZpZGVyX2lkGAYgASgDSAJSCnByb3ZpZGVySWSI'
    'AQESJAoLYXV0b19kZXBsb3kYByABKAhIA1IKYXV0b0RlcGxveYgBAUIJCgdfYnJhbmNoQg0KC1'
    '9idWlsZF9wYXRoQg4KDF9wcm92aWRlcl9pZEIOCgxfYXV0b19kZXBsb3k=');

@$core.Deprecated('Use setGitlabSourceRequestDescriptor instead')
const SetGitlabSourceRequest$json = {
  '1': 'SetGitlabSourceRequest',
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
      '1': 'path_namespace',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'pathNamespace',
      '17': true
    },
    {
      '1': 'provider_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'providerId',
      '17': true
    },
    {
      '1': 'project_id',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'projectId',
      '17': true
    },
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_path_namespace'},
    {'1': '_provider_id'},
    {'1': '_project_id'},
  ],
};

/// Descriptor for `SetGitlabSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setGitlabSourceRequestDescriptor = $convert.base64Decode(
    'ChZTZXRHaXRsYWJTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBInCgpyZX'
    'Bvc2l0b3J5GAIgASgJQge6SARyAhABUgpyZXBvc2l0b3J5Eh0KBW93bmVyGAMgASgJQge6SARy'
    'AhABUgVvd25lchIbCgZicmFuY2gYBCABKAlIAFIGYnJhbmNoiAEBEiIKCmJ1aWxkX3BhdGgYBS'
    'ABKAlIAVIJYnVpbGRQYXRoiAEBEioKDnBhdGhfbmFtZXNwYWNlGAYgASgJSAJSDXBhdGhOYW1l'
    'c3BhY2WIAQESJAoLcHJvdmlkZXJfaWQYByABKANIA1IKcHJvdmlkZXJJZIgBARIiCgpwcm9qZW'
    'N0X2lkGAggASgDSARSCXByb2plY3RJZIgBAUIJCgdfYnJhbmNoQg0KC19idWlsZF9wYXRoQhEK'
    'D19wYXRoX25hbWVzcGFjZUIOCgxfcHJvdmlkZXJfaWRCDQoLX3Byb2plY3RfaWQ=');

@$core.Deprecated('Use setBitbucketSourceRequestDescriptor instead')
const SetBitbucketSourceRequest$json = {
  '1': 'SetBitbucketSourceRequest',
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
      '1': 'repository_slug',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'repositorySlug',
      '17': true
    },
    {
      '1': 'provider_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'providerId',
      '17': true
    },
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_repository_slug'},
    {'1': '_provider_id'},
  ],
};

/// Descriptor for `SetBitbucketSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setBitbucketSourceRequestDescriptor = $convert.base64Decode(
    'ChlTZXRCaXRidWNrZXRTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBInCg'
    'pyZXBvc2l0b3J5GAIgASgJQge6SARyAhABUgpyZXBvc2l0b3J5Eh0KBW93bmVyGAMgASgJQge6'
    'SARyAhABUgVvd25lchIbCgZicmFuY2gYBCABKAlIAFIGYnJhbmNoiAEBEiIKCmJ1aWxkX3BhdG'
    'gYBSABKAlIAVIJYnVpbGRQYXRoiAEBEiwKD3JlcG9zaXRvcnlfc2x1ZxgGIAEoCUgCUg5yZXBv'
    'c2l0b3J5U2x1Z4gBARIkCgtwcm92aWRlcl9pZBgHIAEoA0gDUgpwcm92aWRlcklkiAEBQgkKB1'
    '9icmFuY2hCDQoLX2J1aWxkX3BhdGhCEgoQX3JlcG9zaXRvcnlfc2x1Z0IOCgxfcHJvdmlkZXJf'
    'aWQ=');

@$core.Deprecated('Use setGiteaSourceRequestDescriptor instead')
const SetGiteaSourceRequest$json = {
  '1': 'SetGiteaSourceRequest',
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
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_provider_id'},
  ],
};

/// Descriptor for `SetGiteaSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setGiteaSourceRequestDescriptor = $convert.base64Decode(
    'ChVTZXRHaXRlYVNvdXJjZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEicKCnJlcG'
    '9zaXRvcnkYAiABKAlCB7pIBHICEAFSCnJlcG9zaXRvcnkSHQoFb3duZXIYAyABKAlCB7pIBHIC'
    'EAFSBW93bmVyEhsKBmJyYW5jaBgEIAEoCUgAUgZicmFuY2iIAQESIgoKYnVpbGRfcGF0aBgFIA'
    'EoCUgBUglidWlsZFBhdGiIAQESJAoLcHJvdmlkZXJfaWQYBiABKANIAlIKcHJvdmlkZXJJZIgB'
    'AUIJCgdfYnJhbmNoQg0KC19idWlsZF9wYXRoQg4KDF9wcm92aWRlcl9pZA==');

@$core.Deprecated('Use setCustomGitSourceRequestDescriptor instead')
const SetCustomGitSourceRequest$json = {
  '1': 'SetCustomGitSourceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'url', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'url'},
    {'1': 'branch', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'branch', '17': true},
    {
      '1': 'build_path',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'buildPath',
      '17': true
    },
    {
      '1': 'ssh_key_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'sshKeyId',
      '17': true
    },
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_build_path'},
    {'1': '_ssh_key_id'},
  ],
};

/// Descriptor for `SetCustomGitSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setCustomGitSourceRequestDescriptor = $convert.base64Decode(
    'ChlTZXRDdXN0b21HaXRTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIaCg'
    'N1cmwYAiABKAlCCLpIBXIDiAEBUgN1cmwSGwoGYnJhbmNoGAMgASgJSABSBmJyYW5jaIgBARIi'
    'CgpidWlsZF9wYXRoGAQgASgJSAFSCWJ1aWxkUGF0aIgBARIhCgpzc2hfa2V5X2lkGAUgASgDSA'
    'JSCHNzaEtleUlkiAEBQgkKB19icmFuY2hCDQoLX2J1aWxkX3BhdGhCDQoLX3NzaF9rZXlfaWQ=');

@$core.Deprecated('Use setDockerSourceRequestDescriptor instead')
const SetDockerSourceRequest$json = {
  '1': 'SetDockerSourceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'image', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'image'},
    {
      '1': 'username',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'username',
      '17': true
    },
    {
      '1': 'password',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'password',
      '17': true
    },
    {
      '1': 'registry_url',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'registryUrl',
      '17': true
    },
  ],
  '8': [
    {'1': '_username'},
    {'1': '_password'},
    {'1': '_registry_url'},
  ],
};

/// Descriptor for `SetDockerSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDockerSourceRequestDescriptor = $convert.base64Decode(
    'ChZTZXREb2NrZXJTb3VyY2VSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIdCgVpbW'
    'FnZRgCIAEoCUIHukgEcgIQAVIFaW1hZ2USHwoIdXNlcm5hbWUYAyABKAlIAFIIdXNlcm5hbWWI'
    'AQESHwoIcGFzc3dvcmQYBCABKAlIAVIIcGFzc3dvcmSIAQESJgoMcmVnaXN0cnlfdXJsGAUgAS'
    'gJSAJSC3JlZ2lzdHJ5VXJsiAEBQgsKCV91c2VybmFtZUILCglfcGFzc3dvcmRCDwoNX3JlZ2lz'
    'dHJ5X3VybA==');

@$core.Deprecated('Use setDropSourceRequestDescriptor instead')
const SetDropSourceRequest$json = {
  '1': 'SetDropSourceRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'build_path',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'buildPath',
      '17': true
    },
  ],
  '8': [
    {'1': '_build_path'},
  ],
};

/// Descriptor for `SetDropSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDropSourceRequestDescriptor = $convert.base64Decode(
    'ChRTZXREcm9wU291cmNlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSIgoKYnVpbG'
    'RfcGF0aBgCIAEoCUgAUglidWlsZFBhdGiIAQFCDQoLX2J1aWxkX3BhdGg=');

@$core.Deprecated('Use dropSourceHeaderDescriptor instead')
const DropSourceHeader$json = {
  '1': 'DropSourceHeader',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {
      '1': 'build_path',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'buildPath',
      '17': true
    },
  ],
  '8': [
    {'1': '_build_path'},
  ],
};

/// Descriptor for `DropSourceHeader`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dropSourceHeaderDescriptor = $convert.base64Decode(
    'ChBEcm9wU291cmNlSGVhZGVyEi4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQge6SAQiAiAAUg1hcH'
    'BsaWNhdGlvbklkEiIKCmJ1aWxkX3BhdGgYAiABKAlIAFIJYnVpbGRQYXRoiAEBQg0KC19idWls'
    'ZF9wYXRo');

@$core.Deprecated('Use uploadSourceRequestDescriptor instead')
const UploadSourceRequest$json = {
  '1': 'UploadSourceRequest',
  '2': [
    {
      '1': 'header',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.DropSourceHeader',
      '9': 0,
      '10': 'header'
    },
    {'1': 'chunk', '3': 2, '4': 1, '5': 12, '8': {}, '9': 0, '10': 'chunk'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `UploadSourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List uploadSourceRequestDescriptor = $convert.base64Decode(
    'ChNVcGxvYWRTb3VyY2VSZXF1ZXN0EjwKBmhlYWRlchgBIAEoCzIiLm9wZW5veGlkZS5hcGkudj'
    'EuRHJvcFNvdXJjZUhlYWRlckgAUgZoZWFkZXISJAoFY2h1bmsYAiABKAxCDLpICXoHEAEYgICA'
    'AkgAUgVjaHVua0IJCgdwYXlsb2Fk');

@$core.Deprecated('Use updateBuildConfigRequestDescriptor instead')
const UpdateBuildConfigRequest$json = {
  '1': 'UpdateBuildConfigRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'build_args',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'buildArgs',
      '17': true
    },
    {
      '1': 'build_secrets',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'buildSecrets',
      '17': true
    },
    {
      '1': 'dockerfile',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'dockerfile',
      '17': true
    },
    {
      '1': 'context_path',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'contextPath',
      '17': true
    },
    {
      '1': 'build_stage',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'buildStage',
      '17': true
    },
    {
      '1': 'publish_directory',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'publishDirectory',
      '17': true
    },
    {
      '1': 'static_spa',
      '3': 8,
      '4': 1,
      '5': 8,
      '9': 6,
      '10': 'staticSpa',
      '17': true
    },
    {
      '1': 'create_env_file',
      '3': 9,
      '4': 1,
      '5': 8,
      '9': 7,
      '10': 'createEnvFile',
      '17': true
    },
    {
      '1': 'command',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'command',
      '17': true
    },
    {'1': 'args', '3': 11, '4': 1, '5': 9, '9': 9, '10': 'args', '17': true},
    {
      '1': 'build_path',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'buildPath',
      '17': true
    },
    {
      '1': 'clean_cache',
      '3': 13,
      '4': 1,
      '5': 8,
      '9': 11,
      '10': 'cleanCache',
      '17': true
    },
    {
      '1': 'enable_submodules',
      '3': 14,
      '4': 1,
      '5': 8,
      '9': 12,
      '10': 'enableSubmodules',
      '17': true
    },
    {
      '1': 'watch_paths',
      '3': 15,
      '4': 1,
      '5': 9,
      '9': 13,
      '10': 'watchPaths',
      '17': true
    },
    {
      '1': 'railpack_version',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 14,
      '10': 'railpackVersion',
      '17': true
    },
    {
      '1': 'heroku_version',
      '3': 17,
      '4': 1,
      '5': 9,
      '9': 15,
      '10': 'herokuVersion',
      '17': true
    },
    {
      '1': 'auto_deploy',
      '3': 18,
      '4': 1,
      '5': 8,
      '9': 16,
      '10': 'autoDeploy',
      '17': true
    },
  ],
  '8': [
    {'1': '_build_args'},
    {'1': '_build_secrets'},
    {'1': '_dockerfile'},
    {'1': '_context_path'},
    {'1': '_build_stage'},
    {'1': '_publish_directory'},
    {'1': '_static_spa'},
    {'1': '_create_env_file'},
    {'1': '_command'},
    {'1': '_args'},
    {'1': '_build_path'},
    {'1': '_clean_cache'},
    {'1': '_enable_submodules'},
    {'1': '_watch_paths'},
    {'1': '_railpack_version'},
    {'1': '_heroku_version'},
    {'1': '_auto_deploy'},
  ],
};

/// Descriptor for `UpdateBuildConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateBuildConfigRequestDescriptor = $convert.base64Decode(
    'ChhVcGRhdGVCdWlsZENvbmZpZ1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEiIKCm'
    'J1aWxkX2FyZ3MYAiABKAlIAFIJYnVpbGRBcmdziAEBEigKDWJ1aWxkX3NlY3JldHMYAyABKAlI'
    'AVIMYnVpbGRTZWNyZXRziAEBEiMKCmRvY2tlcmZpbGUYBCABKAlIAlIKZG9ja2VyZmlsZYgBAR'
    'ImCgxjb250ZXh0X3BhdGgYBSABKAlIA1ILY29udGV4dFBhdGiIAQESJAoLYnVpbGRfc3RhZ2UY'
    'BiABKAlIBFIKYnVpbGRTdGFnZYgBARIwChFwdWJsaXNoX2RpcmVjdG9yeRgHIAEoCUgFUhBwdW'
    'JsaXNoRGlyZWN0b3J5iAEBEiIKCnN0YXRpY19zcGEYCCABKAhIBlIJc3RhdGljU3BhiAEBEisK'
    'D2NyZWF0ZV9lbnZfZmlsZRgJIAEoCEgHUg1jcmVhdGVFbnZGaWxliAEBEh0KB2NvbW1hbmQYCi'
    'ABKAlICFIHY29tbWFuZIgBARIXCgRhcmdzGAsgASgJSAlSBGFyZ3OIAQESIgoKYnVpbGRfcGF0'
    'aBgMIAEoCUgKUglidWlsZFBhdGiIAQESJAoLY2xlYW5fY2FjaGUYDSABKAhIC1IKY2xlYW5DYW'
    'NoZYgBARIwChFlbmFibGVfc3VibW9kdWxlcxgOIAEoCEgMUhBlbmFibGVTdWJtb2R1bGVziAEB'
    'EiQKC3dhdGNoX3BhdGhzGA8gASgJSA1SCndhdGNoUGF0aHOIAQESLgoQcmFpbHBhY2tfdmVyc2'
    'lvbhgQIAEoCUgOUg9yYWlscGFja1ZlcnNpb26IAQESKgoOaGVyb2t1X3ZlcnNpb24YESABKAlI'
    'D1INaGVyb2t1VmVyc2lvbogBARIkCgthdXRvX2RlcGxveRgSIAEoCEgQUgphdXRvRGVwbG95iA'
    'EBQg0KC19idWlsZF9hcmdzQhAKDl9idWlsZF9zZWNyZXRzQg0KC19kb2NrZXJmaWxlQg8KDV9j'
    'b250ZXh0X3BhdGhCDgoMX2J1aWxkX3N0YWdlQhQKEl9wdWJsaXNoX2RpcmVjdG9yeUINCgtfc3'
    'RhdGljX3NwYUISChBfY3JlYXRlX2Vudl9maWxlQgoKCF9jb21tYW5kQgcKBV9hcmdzQg0KC19i'
    'dWlsZF9wYXRoQg4KDF9jbGVhbl9jYWNoZUIUChJfZW5hYmxlX3N1Ym1vZHVsZXNCDgoMX3dhdG'
    'NoX3BhdGhzQhMKEV9yYWlscGFja192ZXJzaW9uQhEKD19oZXJva3VfdmVyc2lvbkIOCgxfYXV0'
    'b19kZXBsb3k=');

@$core.Deprecated('Use updateResourceConfigRequestDescriptor instead')
const UpdateResourceConfigRequest$json = {
  '1': 'UpdateResourceConfigRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'memory_reservation',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'memoryReservation',
      '17': true
    },
    {
      '1': 'memory_limit',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'memoryLimit',
      '17': true
    },
    {
      '1': 'cpu_reservation',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'cpuReservation',
      '17': true
    },
    {
      '1': 'cpu_limit',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'cpuLimit',
      '17': true
    },
    {
      '1': 'replicas',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'replicas',
      '17': true
    },
  ],
  '8': [
    {'1': '_memory_reservation'},
    {'1': '_memory_limit'},
    {'1': '_cpu_reservation'},
    {'1': '_cpu_limit'},
    {'1': '_replicas'},
  ],
};

/// Descriptor for `UpdateResourceConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResourceConfigRequestDescriptor = $convert.base64Decode(
    'ChtVcGRhdGVSZXNvdXJjZUNvbmZpZ1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEj'
    'IKEm1lbW9yeV9yZXNlcnZhdGlvbhgCIAEoCUgAUhFtZW1vcnlSZXNlcnZhdGlvbogBARImCgxt'
    'ZW1vcnlfbGltaXQYAyABKAlIAVILbWVtb3J5TGltaXSIAQESLAoPY3B1X3Jlc2VydmF0aW9uGA'
    'QgASgJSAJSDmNwdVJlc2VydmF0aW9uiAEBEiAKCWNwdV9saW1pdBgFIAEoCUgDUghjcHVMaW1p'
    'dIgBARIfCghyZXBsaWNhcxgGIAEoA0gEUghyZXBsaWNhc4gBAUIVChNfbWVtb3J5X3Jlc2Vydm'
    'F0aW9uQg8KDV9tZW1vcnlfbGltaXRCEgoQX2NwdV9yZXNlcnZhdGlvbkIMCgpfY3B1X2xpbWl0'
    'QgsKCV9yZXBsaWNhcw==');

@$core.Deprecated('Use updatePreviewConfigRequestDescriptor instead')
const UpdatePreviewConfigRequest$json = {
  '1': 'UpdatePreviewConfigRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'preview_env',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'previewEnv',
      '17': true
    },
    {
      '1': 'build_args',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'buildArgs',
      '17': true
    },
    {
      '1': 'build_secrets',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'buildSecrets',
      '17': true
    },
    {'1': 'labels', '3': 5, '4': 1, '5': 9, '9': 3, '10': 'labels', '17': true},
    {
      '1': 'wildcard',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'wildcard',
      '17': true
    },
    {
      '1': 'port',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'port',
      '17': true
    },
    {'1': 'https', '3': 8, '4': 1, '5': 8, '10': 'https'},
    {'1': 'path', '3': 9, '4': 1, '5': 9, '10': 'path'},
    {'1': 'certificate_type', '3': 10, '4': 1, '5': 9, '10': 'certificateType'},
    {
      '1': 'custom_cert_resolver',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'limit', '3': 12, '4': 1, '5': 3, '10': 'limit'},
    {'1': 'active', '3': 13, '4': 1, '5': 8, '10': 'active'},
    {
      '1': 'require_collaborator_permissions',
      '3': 14,
      '4': 1,
      '5': 8,
      '10': 'requireCollaboratorPermissions'
    },
  ],
  '8': [
    {'1': '_preview_env'},
    {'1': '_build_args'},
    {'1': '_build_secrets'},
    {'1': '_labels'},
    {'1': '_wildcard'},
    {'1': '_port'},
    {'1': '_custom_cert_resolver'},
  ],
};

/// Descriptor for `UpdatePreviewConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updatePreviewConfigRequestDescriptor = $convert.base64Decode(
    'ChpVcGRhdGVQcmV2aWV3Q29uZmlnUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSJA'
    'oLcHJldmlld19lbnYYAiABKAlIAFIKcHJldmlld0VudogBARIiCgpidWlsZF9hcmdzGAMgASgJ'
    'SAFSCWJ1aWxkQXJnc4gBARIoCg1idWlsZF9zZWNyZXRzGAQgASgJSAJSDGJ1aWxkU2VjcmV0c4'
    'gBARIbCgZsYWJlbHMYBSABKAlIA1IGbGFiZWxziAEBEh8KCHdpbGRjYXJkGAYgASgJSARSCHdp'
    'bGRjYXJkiAEBEiQKBHBvcnQYByABKANCC7pICCIGGP//AygBSAVSBHBvcnSIAQESFAoFaHR0cH'
    'MYCCABKAhSBWh0dHBzEhIKBHBhdGgYCSABKAlSBHBhdGgSKQoQY2VydGlmaWNhdGVfdHlwZRgK'
    'IAEoCVIPY2VydGlmaWNhdGVUeXBlEjUKFGN1c3RvbV9jZXJ0X3Jlc29sdmVyGAsgASgJSAZSEm'
    'N1c3RvbUNlcnRSZXNvbHZlcogBARIUCgVsaW1pdBgMIAEoA1IFbGltaXQSFgoGYWN0aXZlGA0g'
    'ASgIUgZhY3RpdmUSSAogcmVxdWlyZV9jb2xsYWJvcmF0b3JfcGVybWlzc2lvbnMYDiABKAhSHn'
    'JlcXVpcmVDb2xsYWJvcmF0b3JQZXJtaXNzaW9uc0IOCgxfcHJldmlld19lbnZCDQoLX2J1aWxk'
    'X2FyZ3NCEAoOX2J1aWxkX3NlY3JldHNCCQoHX2xhYmVsc0ILCglfd2lsZGNhcmRCBwoFX3Bvcn'
    'RCFwoVX2N1c3RvbV9jZXJ0X3Jlc29sdmVy');

@$core.Deprecated('Use applicationOperationRequestDescriptor instead')
const ApplicationOperationRequest$json = {
  '1': 'ApplicationOperationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationOperationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationOperationRequestDescriptor =
    $convert.base64Decode(
        'ChtBcHBsaWNhdGlvbk9wZXJhdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use applicationOperationDescriptor instead')
const ApplicationOperation$json = {
  '1': 'ApplicationOperation',
  '2': [
    {
      '1': 'application',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Application',
      '10': 'application'
    },
    {'1': 'operation', '3': 2, '4': 1, '5': 9, '10': 'operation'},
  ],
};

/// Descriptor for `ApplicationOperation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationOperationDescriptor = $convert.base64Decode(
    'ChRBcHBsaWNhdGlvbk9wZXJhdGlvbhI/CgthcHBsaWNhdGlvbhgBIAEoCzIdLm9wZW5veGlkZS'
    '5hcGkudjEuQXBwbGljYXRpb25SC2FwcGxpY2F0aW9uEhwKCW9wZXJhdGlvbhgCIAEoCVIJb3Bl'
    'cmF0aW9u');

@$core.Deprecated('Use applicationWebhookTokenDescriptor instead')
const ApplicationWebhookToken$json = {
  '1': 'ApplicationWebhookToken',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `ApplicationWebhookToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationWebhookTokenDescriptor =
    $convert.base64Decode(
        'ChdBcHBsaWNhdGlvbldlYmhvb2tUb2tlbhIUCgV0b2tlbhgBIAEoCVIFdG9rZW4=');

@$core.Deprecated('Use moveApplicationRequestDescriptor instead')
const MoveApplicationRequest$json = {
  '1': 'MoveApplicationRequest',
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

/// Descriptor for `MoveApplicationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List moveApplicationRequestDescriptor = $convert.base64Decode(
    'ChZNb3ZlQXBwbGljYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI7ChV0YX'
    'JnZXRfZW52aXJvbm1lbnRfaWQYAiABKANCB7pIBCICIABSE3RhcmdldEVudmlyb25tZW50SWQ=');

@$core.Deprecated('Use listRollbacksRequestDescriptor instead')
const ListRollbacksRequest$json = {
  '1': 'ListRollbacksRequest',
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

/// Descriptor for `ListRollbacksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRollbacksRequestDescriptor = $convert.base64Decode(
    'ChRMaXN0Um9sbGJhY2tzUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0IHukgEIgIgAF'
    'INYXBwbGljYXRpb25JZA==');

@$core.Deprecated('Use listRollbacksResponseDescriptor instead')
const ListRollbacksResponse$json = {
  '1': 'ListRollbacksResponse',
  '2': [
    {
      '1': 'rollbacks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Rollback',
      '10': 'rollbacks'
    },
  ],
};

/// Descriptor for `ListRollbacksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRollbacksResponseDescriptor = $convert.base64Decode(
    'ChVMaXN0Um9sbGJhY2tzUmVzcG9uc2USOAoJcm9sbGJhY2tzGAEgAygLMhoub3Blbm94aWRlLm'
    'FwaS52MS5Sb2xsYmFja1IJcm9sbGJhY2tz');

@$core.Deprecated('Use rollbackDescriptor instead')
const Rollback$json = {
  '1': 'Rollback',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'application_id', '3': 2, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'status', '3': 3, '4': 1, '5': 9, '10': 'status'},
    {'1': 'commit', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'commit', '17': true},
    {'1': 'created_at', '3': 5, '4': 1, '5': 3, '10': 'createdAt'},
  ],
  '8': [
    {'1': '_commit'},
  ],
};

/// Descriptor for `Rollback`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rollbackDescriptor = $convert.base64Decode(
    'CghSb2xsYmFjaxIOCgJpZBgBIAEoA1ICaWQSJQoOYXBwbGljYXRpb25faWQYAiABKANSDWFwcG'
    'xpY2F0aW9uSWQSFgoGc3RhdHVzGAMgASgJUgZzdGF0dXMSGwoGY29tbWl0GAQgASgJSABSBmNv'
    'bW1pdIgBARIdCgpjcmVhdGVkX2F0GAUgASgDUgljcmVhdGVkQXRCCQoHX2NvbW1pdA==');

@$core.Deprecated('Use triggerRollbackRequestDescriptor instead')
const TriggerRollbackRequest$json = {
  '1': 'TriggerRollbackRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'rollback_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'rollbackId'},
  ],
};

/// Descriptor for `TriggerRollbackRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List triggerRollbackRequestDescriptor = $convert.base64Decode(
    'ChZUcmlnZ2VyUm9sbGJhY2tSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQge6SAQiAi'
    'AAUg1hcHBsaWNhdGlvbklkEigKC3JvbGxiYWNrX2lkGAIgASgDQge6SAQiAiAAUgpyb2xsYmFj'
    'a0lk');

@$core.Deprecated('Use rollbackTriggerDescriptor instead')
const RollbackTrigger$json = {
  '1': 'RollbackTrigger',
  '2': [
    {'1': 'rollback_id', '3': 1, '4': 1, '5': 3, '10': 'rollbackId'},
    {'1': 'status', '3': 2, '4': 1, '5': 9, '10': 'status'},
  ],
};

/// Descriptor for `RollbackTrigger`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rollbackTriggerDescriptor = $convert.base64Decode(
    'Cg9Sb2xsYmFja1RyaWdnZXISHwoLcm9sbGJhY2tfaWQYASABKANSCnJvbGxiYWNrSWQSFgoGc3'
    'RhdHVzGAIgASgJUgZzdGF0dXM=');

@$core.Deprecated('Use deleteRollbackRequestDescriptor instead')
const DeleteRollbackRequest$json = {
  '1': 'DeleteRollbackRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'rollback_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'rollbackId'},
  ],
};

/// Descriptor for `DeleteRollbackRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteRollbackRequestDescriptor = $convert.base64Decode(
    'ChVEZWxldGVSb2xsYmFja1JlcXVlc3QSLgoOYXBwbGljYXRpb25faWQYASABKANCB7pIBCICIA'
    'BSDWFwcGxpY2F0aW9uSWQSKAoLcm9sbGJhY2tfaWQYAiABKANCB7pIBCICIABSCnJvbGxiYWNr'
    'SWQ=');

@$core.Deprecated('Use applicationDependenciesDescriptor instead')
const ApplicationDependencies$json = {
  '1': 'ApplicationDependencies',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationDependencies`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDependenciesDescriptor =
    $convert.base64Decode(
        'ChdBcHBsaWNhdGlvbkRlcGVuZGVuY2llcxIUCgVpdGVtcxgBIAMoCVIFaXRlbXM=');

@$core.Deprecated('Use resourceListRequestDescriptor instead')
const ResourceListRequest$json = {
  '1': 'ResourceListRequest',
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

/// Descriptor for `ResourceListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resourceListRequestDescriptor = $convert.base64Decode(
    'ChNSZXNvdXJjZUxpc3RSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQge6SAQiAiAAUg'
    '1hcHBsaWNhdGlvbklk');

@$core.Deprecated('Use applicationPortsDescriptor instead')
const ApplicationPorts$json = {
  '1': 'ApplicationPorts',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationPorts`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationPortsDescriptor = $convert
    .base64Decode('ChBBcHBsaWNhdGlvblBvcnRzEhQKBWl0ZW1zGAEgAygJUgVpdGVtcw==');

@$core.Deprecated('Use applicationMountsDescriptor instead')
const ApplicationMounts$json = {
  '1': 'ApplicationMounts',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationMounts`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationMountsDescriptor = $convert
    .base64Decode('ChFBcHBsaWNhdGlvbk1vdW50cxIUCgVpdGVtcxgBIAMoCVIFaXRlbXM=');

@$core.Deprecated('Use applicationNetworksDescriptor instead')
const ApplicationNetworks$json = {
  '1': 'ApplicationNetworks',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationNetworks`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationNetworksDescriptor =
    $convert.base64Decode(
        'ChNBcHBsaWNhdGlvbk5ldHdvcmtzEhQKBWl0ZW1zGAEgAygJUgVpdGVtcw==');

@$core.Deprecated('Use applicationPatchesDescriptor instead')
const ApplicationPatches$json = {
  '1': 'ApplicationPatches',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationPatches`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationPatchesDescriptor = $convert
    .base64Decode('ChJBcHBsaWNhdGlvblBhdGNoZXMSFAoFaXRlbXMYASADKAlSBWl0ZW1z');

@$core.Deprecated('Use applicationMiddlewaresDescriptor instead')
const ApplicationMiddlewares$json = {
  '1': 'ApplicationMiddlewares',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationMiddlewares`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationMiddlewaresDescriptor =
    $convert.base64Decode(
        'ChZBcHBsaWNhdGlvbk1pZGRsZXdhcmVzEhQKBWl0ZW1zGAEgAygJUgVpdGVtcw==');

@$core.Deprecated('Use applicationRedirectsDescriptor instead')
const ApplicationRedirects$json = {
  '1': 'ApplicationRedirects',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationRedirects`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationRedirectsDescriptor =
    $convert.base64Decode(
        'ChRBcHBsaWNhdGlvblJlZGlyZWN0cxIUCgVpdGVtcxgBIAMoCVIFaXRlbXM=');

@$core.Deprecated('Use applicationSecurityDescriptor instead')
const ApplicationSecurity$json = {
  '1': 'ApplicationSecurity',
  '2': [
    {'1': 'items', '3': 1, '4': 3, '5': 9, '10': 'items'},
  ],
};

/// Descriptor for `ApplicationSecurity`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationSecurityDescriptor =
    $convert.base64Decode(
        'ChNBcHBsaWNhdGlvblNlY3VyaXR5EhQKBWl0ZW1zGAEgAygJUgVpdGVtcw==');

@$core.Deprecated('Use applicationPortDescriptor instead')
const ApplicationPort$json = {
  '1': 'ApplicationPort',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'published_port', '3': 2, '4': 1, '5': 3, '10': 'publishedPort'},
    {'1': 'target_port', '3': 3, '4': 1, '5': 3, '10': 'targetPort'},
    {
      '1': 'protocol',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortProtocol',
      '10': 'protocol'
    },
    {
      '1': 'publish_mode',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortPublishMode',
      '10': 'publishMode'
    },
    {'1': 'application_id', '3': 6, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `ApplicationPort`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationPortDescriptor = $convert.base64Decode(
    'Cg9BcHBsaWNhdGlvblBvcnQSDgoCaWQYASABKANSAmlkEiUKDnB1Ymxpc2hlZF9wb3J0GAIgAS'
    'gDUg1wdWJsaXNoZWRQb3J0Eh8KC3RhcmdldF9wb3J0GAMgASgDUgp0YXJnZXRQb3J0EkUKCHBy'
    'b3RvY29sGAQgASgOMikub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvblBvcnRQcm90b2NvbF'
    'IIcHJvdG9jb2wSTwoMcHVibGlzaF9tb2RlGAUgASgOMiwub3Blbm94aWRlLmFwaS52MS5BcHBs'
    'aWNhdGlvblBvcnRQdWJsaXNoTW9kZVILcHVibGlzaE1vZGUSJQoOYXBwbGljYXRpb25faWQYBi'
    'ABKANSDWFwcGxpY2F0aW9uSWQSHQoKY3JlYXRlZF9hdBgHIAEoA1IJY3JlYXRlZEF0');

@$core.Deprecated('Use applicationMountDescriptor instead')
const ApplicationMount$json = {
  '1': 'ApplicationMount',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'mount_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMountType',
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
    {'1': 'application_id', '3': 8, '4': 1, '5': 3, '10': 'applicationId'},
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

/// Descriptor for `ApplicationMount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationMountDescriptor = $convert.base64Decode(
    'ChBBcHBsaWNhdGlvbk1vdW50Eg4KAmlkGAEgASgDUgJpZBJFCgptb3VudF90eXBlGAIgASgOMi'
    'Yub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1vdW50VHlwZVIJbW91bnRUeXBlEiAKCWhv'
    'c3RfcGF0aBgDIAEoCUgAUghob3N0UGF0aIgBARIkCgt2b2x1bWVfbmFtZRgEIAEoCUgBUgp2b2'
    'x1bWVOYW1liAEBEiAKCWZpbGVfcGF0aBgFIAEoCUgCUghmaWxlUGF0aIgBARIdCgdjb250ZW50'
    'GAYgASgJSANSB2NvbnRlbnSIAQESHQoKbW91bnRfcGF0aBgHIAEoCVIJbW91bnRQYXRoEiUKDm'
    'FwcGxpY2F0aW9uX2lkGAggASgDUg1hcHBsaWNhdGlvbklkEh0KCmNyZWF0ZWRfYXQYCSABKANS'
    'CWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAogASgDUgl1cGRhdGVkQXRCDAoKX2hvc3RfcGF0aE'
    'IOCgxfdm9sdW1lX25hbWVCDAoKX2ZpbGVfcGF0aEIKCghfY29udGVudA==');

@$core.Deprecated('Use applicationPatchDescriptor instead')
const ApplicationPatch$json = {
  '1': 'ApplicationPatch',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'patch_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPatchType',
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 3, '4': 1, '5': 9, '10': 'filePath'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '10': 'content'},
    {'1': 'application_id', '3': 6, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `ApplicationPatch`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationPatchDescriptor = $convert.base64Decode(
    'ChBBcHBsaWNhdGlvblBhdGNoEg4KAmlkGAEgASgDUgJpZBJFCgpwYXRjaF90eXBlGAIgASgOMi'
    'Yub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvblBhdGNoVHlwZVIJcGF0Y2hUeXBlEhsKCWZp'
    'bGVfcGF0aBgDIAEoCVIIZmlsZVBhdGgSGAoHZW5hYmxlZBgEIAEoCFIHZW5hYmxlZBIYCgdjb2'
    '50ZW50GAUgASgJUgdjb250ZW50EiUKDmFwcGxpY2F0aW9uX2lkGAYgASgDUg1hcHBsaWNhdGlv'
    'bklkEh0KCmNyZWF0ZWRfYXQYByABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAggASgDUg'
    'l1cGRhdGVkQXQ=');

@$core.Deprecated('Use applicationMiddlewareHeaderDescriptor instead')
const ApplicationMiddlewareHeader$json = {
  '1': 'ApplicationMiddlewareHeader',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
};

/// Descriptor for `ApplicationMiddlewareHeader`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationMiddlewareHeaderDescriptor =
    $convert.base64Decode(
        'ChtBcHBsaWNhdGlvbk1pZGRsZXdhcmVIZWFkZXISEgoEbmFtZRgBIAEoCVIEbmFtZRIUCgV2YW'
        'x1ZRgCIAEoCVIFdmFsdWU=');

@$core.Deprecated('Use applicationMiddlewareDescriptor instead')
const ApplicationMiddleware$json = {
  '1': 'ApplicationMiddleware',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'application_id', '3': 2, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'middleware_type',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMiddlewareType',
      '10': 'middlewareType'
    },
    {'1': 'enabled', '3': 5, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'headers',
      '3': 6,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddlewareHeader',
      '10': 'headers'
    },
    {
      '1': 'average',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'average',
      '17': true
    },
    {'1': 'burst', '3': 8, '4': 1, '5': 3, '9': 1, '10': 'burst', '17': true},
    {'1': 'source_ranges', '3': 9, '4': 3, '5': 9, '10': 'sourceRanges'},
    {'1': 'created_at', '3': 10, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 11, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_average'},
    {'1': '_burst'},
  ],
};

/// Descriptor for `ApplicationMiddleware`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationMiddlewareDescriptor = $convert.base64Decode(
    'ChVBcHBsaWNhdGlvbk1pZGRsZXdhcmUSDgoCaWQYASABKANSAmlkEiUKDmFwcGxpY2F0aW9uX2'
    'lkGAIgASgDUg1hcHBsaWNhdGlvbklkEhIKBG5hbWUYAyABKAlSBG5hbWUSVAoPbWlkZGxld2Fy'
    'ZV90eXBlGAQgASgOMisub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1pZGRsZXdhcmVUeX'
    'BlUg5taWRkbGV3YXJlVHlwZRIYCgdlbmFibGVkGAUgASgIUgdlbmFibGVkEkcKB2hlYWRlcnMY'
    'BiADKAsyLS5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uTWlkZGxld2FyZUhlYWRlclIHaG'
    'VhZGVycxIdCgdhdmVyYWdlGAcgASgDSABSB2F2ZXJhZ2WIAQESGQoFYnVyc3QYCCABKANIAVIF'
    'YnVyc3SIAQESIwoNc291cmNlX3JhbmdlcxgJIAMoCVIMc291cmNlUmFuZ2VzEh0KCmNyZWF0ZW'
    'RfYXQYCiABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAsgASgDUgl1cGRhdGVkQXRCCgoI'
    'X2F2ZXJhZ2VCCAoGX2J1cnN0');

@$core.Deprecated('Use applicationRedirectDescriptor instead')
const ApplicationRedirect$json = {
  '1': 'ApplicationRedirect',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'regex', '3': 2, '4': 1, '5': 9, '10': 'regex'},
    {'1': 'replacement', '3': 3, '4': 1, '5': 9, '10': 'replacement'},
    {'1': 'permanent', '3': 4, '4': 1, '5': 8, '10': 'permanent'},
    {
      '1': 'unique_config_key',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'uniqueConfigKey',
      '17': true
    },
    {'1': 'application_id', '3': 6, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_unique_config_key'},
  ],
};

/// Descriptor for `ApplicationRedirect`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationRedirectDescriptor = $convert.base64Decode(
    'ChNBcHBsaWNhdGlvblJlZGlyZWN0Eg4KAmlkGAEgASgDUgJpZBIUCgVyZWdleBgCIAEoCVIFcm'
    'VnZXgSIAoLcmVwbGFjZW1lbnQYAyABKAlSC3JlcGxhY2VtZW50EhwKCXBlcm1hbmVudBgEIAEo'
    'CFIJcGVybWFuZW50Ei8KEXVuaXF1ZV9jb25maWdfa2V5GAUgASgDSABSD3VuaXF1ZUNvbmZpZ0'
    'tleYgBARIlCg5hcHBsaWNhdGlvbl9pZBgGIAEoA1INYXBwbGljYXRpb25JZBIdCgpjcmVhdGVk'
    'X2F0GAcgASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgIIAEoA1IJdXBkYXRlZEF0QhQKEl'
    '91bmlxdWVfY29uZmlnX2tleQ==');

@$core.Deprecated('Use applicationSecurityRuleDescriptor instead')
const ApplicationSecurityRule$json = {
  '1': 'ApplicationSecurityRule',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'username', '3': 2, '4': 1, '5': 9, '10': 'username'},
    {'1': 'has_password', '3': 3, '4': 1, '5': 8, '10': 'hasPassword'},
    {'1': 'application_id', '3': 4, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'created_at', '3': 5, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `ApplicationSecurityRule`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationSecurityRuleDescriptor = $convert.base64Decode(
    'ChdBcHBsaWNhdGlvblNlY3VyaXR5UnVsZRIOCgJpZBgBIAEoA1ICaWQSGgoIdXNlcm5hbWUYAi'
    'ABKAlSCHVzZXJuYW1lEiEKDGhhc19wYXNzd29yZBgDIAEoCFILaGFzUGFzc3dvcmQSJQoOYXBw'
    'bGljYXRpb25faWQYBCABKANSDWFwcGxpY2F0aW9uSWQSHQoKY3JlYXRlZF9hdBgFIAEoA1IJY3'
    'JlYXRlZEF0');

@$core.Deprecated('Use applicationListPortsRequestDescriptor instead')
const ApplicationListPortsRequest$json = {
  '1': 'ApplicationListPortsRequest',
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

/// Descriptor for `ApplicationListPortsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListPortsRequestDescriptor =
    $convert.base64Decode(
        'ChtBcHBsaWNhdGlvbkxpc3RQb3J0c1JlcXVlc3QSLgoOYXBwbGljYXRpb25faWQYASABKANCB7'
        'pIBCICIABSDWFwcGxpY2F0aW9uSWQ=');

@$core.Deprecated('Use applicationListPortsResponseDescriptor instead')
const ApplicationListPortsResponse$json = {
  '1': 'ApplicationListPortsResponse',
  '2': [
    {
      '1': 'ports',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPort',
      '10': 'ports'
    },
  ],
};

/// Descriptor for `ApplicationListPortsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListPortsResponseDescriptor =
    $convert.base64Decode(
        'ChxBcHBsaWNhdGlvbkxpc3RQb3J0c1Jlc3BvbnNlEjcKBXBvcnRzGAEgAygLMiEub3Blbm94aW'
        'RlLmFwaS52MS5BcHBsaWNhdGlvblBvcnRSBXBvcnRz');

@$core.Deprecated('Use applicationCreatePortRequestDescriptor instead')
const ApplicationCreatePortRequest$json = {
  '1': 'ApplicationCreatePortRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'published_port', '3': 2, '4': 1, '5': 3, '10': 'publishedPort'},
    {'1': 'target_port', '3': 3, '4': 1, '5': 3, '10': 'targetPort'},
    {
      '1': 'protocol',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortProtocol',
      '8': {},
      '10': 'protocol'
    },
    {
      '1': 'publish_mode',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortPublishMode',
      '8': {},
      '10': 'publishMode'
    },
  ],
};

/// Descriptor for `ApplicationCreatePortRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreatePortRequestDescriptor = $convert.base64Decode(
    'ChxBcHBsaWNhdGlvbkNyZWF0ZVBvcnRSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQg'
    'e6SAQiAiAAUg1hcHBsaWNhdGlvbklkEiUKDnB1Ymxpc2hlZF9wb3J0GAIgASgDUg1wdWJsaXNo'
    'ZWRQb3J0Eh8KC3RhcmdldF9wb3J0GAMgASgDUgp0YXJnZXRQb3J0ElEKCHByb3RvY29sGAQgAS'
    'gOMikub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvblBvcnRQcm90b2NvbEIKukgHggEEEAEg'
    'AFIIcHJvdG9jb2wSWwoMcHVibGlzaF9tb2RlGAUgASgOMiwub3Blbm94aWRlLmFwaS52MS5BcH'
    'BsaWNhdGlvblBvcnRQdWJsaXNoTW9kZUIKukgHggEEEAEgAFILcHVibGlzaE1vZGU=');

@$core.Deprecated('Use applicationCreatePortResponseDescriptor instead')
const ApplicationCreatePortResponse$json = {
  '1': 'ApplicationCreatePortResponse',
  '2': [
    {
      '1': 'port',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPort',
      '10': 'port'
    },
  ],
};

/// Descriptor for `ApplicationCreatePortResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreatePortResponseDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkNyZWF0ZVBvcnRSZXNwb25zZRI1CgRwb3J0GAEgASgLMiEub3Blbm94aW'
        'RlLmFwaS52MS5BcHBsaWNhdGlvblBvcnRSBHBvcnQ=');

@$core.Deprecated('Use applicationUpdatePortRequestDescriptor instead')
const ApplicationUpdatePortRequest$json = {
  '1': 'ApplicationUpdatePortRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'published_port', '3': 3, '4': 1, '5': 3, '10': 'publishedPort'},
    {'1': 'target_port', '3': 4, '4': 1, '5': 3, '10': 'targetPort'},
    {
      '1': 'protocol',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortProtocol',
      '8': {},
      '10': 'protocol'
    },
    {
      '1': 'publish_mode',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPortPublishMode',
      '8': {},
      '10': 'publishMode'
    },
  ],
};

/// Descriptor for `ApplicationUpdatePortRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdatePortRequestDescriptor = $convert.base64Decode(
    'ChxBcHBsaWNhdGlvblVwZGF0ZVBvcnRSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQg'
    'e6SAQiAiAAUg1hcHBsaWNhdGlvbklkEhcKAmlkGAIgASgDQge6SAQiAiAAUgJpZBIlCg5wdWJs'
    'aXNoZWRfcG9ydBgDIAEoA1INcHVibGlzaGVkUG9ydBIfCgt0YXJnZXRfcG9ydBgEIAEoA1IKdG'
    'FyZ2V0UG9ydBJRCghwcm90b2NvbBgFIAEoDjIpLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRp'
    'b25Qb3J0UHJvdG9jb2xCCrpIB4IBBBABIABSCHByb3RvY29sElsKDHB1Ymxpc2hfbW9kZRgGIA'
    'EoDjIsLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRpb25Qb3J0UHVibGlzaE1vZGVCCrpIB4IB'
    'BBABIABSC3B1Ymxpc2hNb2Rl');

@$core.Deprecated('Use applicationUpdatePortResponseDescriptor instead')
const ApplicationUpdatePortResponse$json = {
  '1': 'ApplicationUpdatePortResponse',
  '2': [
    {
      '1': 'port',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPort',
      '10': 'port'
    },
  ],
};

/// Descriptor for `ApplicationUpdatePortResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdatePortResponseDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvblVwZGF0ZVBvcnRSZXNwb25zZRI1CgRwb3J0GAEgASgLMiEub3Blbm94aW'
        'RlLmFwaS52MS5BcHBsaWNhdGlvblBvcnRSBHBvcnQ=');

@$core.Deprecated('Use applicationDeletePortRequestDescriptor instead')
const ApplicationDeletePortRequest$json = {
  '1': 'ApplicationDeletePortRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeletePortRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeletePortRequestDescriptor =
    $convert.base64Decode(
        'ChxBcHBsaWNhdGlvbkRlbGV0ZVBvcnRSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQg'
        'e6SAQiAiAAUg1hcHBsaWNhdGlvbklkEhcKAmlkGAIgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use applicationListMountsRequestDescriptor instead')
const ApplicationListMountsRequest$json = {
  '1': 'ApplicationListMountsRequest',
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

/// Descriptor for `ApplicationListMountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListMountsRequestDescriptor =
    $convert.base64Decode(
        'ChxBcHBsaWNhdGlvbkxpc3RNb3VudHNSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgASgDQg'
        'e6SAQiAiAAUg1hcHBsaWNhdGlvbklk');

@$core.Deprecated('Use applicationListMountsResponseDescriptor instead')
const ApplicationListMountsResponse$json = {
  '1': 'ApplicationListMountsResponse',
  '2': [
    {
      '1': 'mounts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMount',
      '10': 'mounts'
    },
  ],
};

/// Descriptor for `ApplicationListMountsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListMountsResponseDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkxpc3RNb3VudHNSZXNwb25zZRI6CgZtb3VudHMYASADKAsyIi5vcGVub3'
        'hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uTW91bnRSBm1vdW50cw==');

@$core.Deprecated('Use applicationCreateMountRequestDescriptor instead')
const ApplicationCreateMountRequest$json = {
  '1': 'ApplicationCreateMountRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {
      '1': 'mount_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMountType',
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

/// Descriptor for `ApplicationCreateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateMountRequestDescriptor = $convert.base64Decode(
    'Ch1BcHBsaWNhdGlvbkNyZWF0ZU1vdW50UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
    'IHukgEIgIgAFINYXBwbGljYXRpb25JZBJRCgptb3VudF90eXBlGAIgASgOMiYub3Blbm94aWRl'
    'LmFwaS52MS5BcHBsaWNhdGlvbk1vdW50VHlwZUIKukgHggEEEAEgAFIJbW91bnRUeXBlEiAKCW'
    'hvc3RfcGF0aBgDIAEoCUgAUghob3N0UGF0aIgBARIkCgt2b2x1bWVfbmFtZRgEIAEoCUgBUgp2'
    'b2x1bWVOYW1liAEBEiAKCWZpbGVfcGF0aBgFIAEoCUgCUghmaWxlUGF0aIgBARIoCgdjb250ZW'
    '50GAYgASgJQgm6SAZyBBiAgEBIA1IHY29udGVudIgBARImCgptb3VudF9wYXRoGAcgASgJQge6'
    'SARyAhABUgltb3VudFBhdGhCDAoKX2hvc3RfcGF0aEIOCgxfdm9sdW1lX25hbWVCDAoKX2ZpbG'
    'VfcGF0aEIKCghfY29udGVudA==');

@$core.Deprecated('Use applicationCreateMountResponseDescriptor instead')
const ApplicationCreateMountResponse$json = {
  '1': 'ApplicationCreateMountResponse',
  '2': [
    {
      '1': 'mount',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMount',
      '10': 'mount'
    },
  ],
};

/// Descriptor for `ApplicationCreateMountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateMountResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvbkNyZWF0ZU1vdW50UmVzcG9uc2USOAoFbW91bnQYASABKAsyIi5vcGVub3'
        'hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uTW91bnRSBW1vdW50');

@$core.Deprecated('Use applicationUpdateMountRequestDescriptor instead')
const ApplicationUpdateMountRequest$json = {
  '1': 'ApplicationUpdateMountRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'mount_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMountType',
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

/// Descriptor for `ApplicationUpdateMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateMountRequestDescriptor = $convert.base64Decode(
    'Ch1BcHBsaWNhdGlvblVwZGF0ZU1vdW50UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
    'IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSUQoKbW91'
    'bnRfdHlwZRgDIAEoDjImLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRpb25Nb3VudFR5cGVCCr'
    'pIB4IBBBABIABSCW1vdW50VHlwZRIgCglob3N0X3BhdGgYBCABKAlIAFIIaG9zdFBhdGiIAQES'
    'JAoLdm9sdW1lX25hbWUYBSABKAlIAVIKdm9sdW1lTmFtZYgBARIgCglmaWxlX3BhdGgYBiABKA'
    'lIAlIIZmlsZVBhdGiIAQESKAoHY29udGVudBgHIAEoCUIJukgGcgQYgIBASANSB2NvbnRlbnSI'
    'AQESJgoKbW91bnRfcGF0aBgIIAEoCUIHukgEcgIQAVIJbW91bnRQYXRoQgwKCl9ob3N0X3BhdG'
    'hCDgoMX3ZvbHVtZV9uYW1lQgwKCl9maWxlX3BhdGhCCgoIX2NvbnRlbnQ=');

@$core.Deprecated('Use applicationUpdateMountResponseDescriptor instead')
const ApplicationUpdateMountResponse$json = {
  '1': 'ApplicationUpdateMountResponse',
  '2': [
    {
      '1': 'mount',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMount',
      '10': 'mount'
    },
  ],
};

/// Descriptor for `ApplicationUpdateMountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateMountResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvblVwZGF0ZU1vdW50UmVzcG9uc2USOAoFbW91bnQYASABKAsyIi5vcGVub3'
        'hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uTW91bnRSBW1vdW50');

@$core.Deprecated('Use applicationDeleteMountRequestDescriptor instead')
const ApplicationDeleteMountRequest$json = {
  '1': 'ApplicationDeleteMountRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeleteMountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeleteMountRequestDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkRlbGV0ZU1vdW50UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
        'IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use applicationListPatchesRequestDescriptor instead')
const ApplicationListPatchesRequest$json = {
  '1': 'ApplicationListPatchesRequest',
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

/// Descriptor for `ApplicationListPatchesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListPatchesRequestDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkxpc3RQYXRjaGVzUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
        'IHukgEIgIgAFINYXBwbGljYXRpb25JZA==');

@$core.Deprecated('Use applicationListPatchesResponseDescriptor instead')
const ApplicationListPatchesResponse$json = {
  '1': 'ApplicationListPatchesResponse',
  '2': [
    {
      '1': 'patches',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPatch',
      '10': 'patches'
    },
  ],
};

/// Descriptor for `ApplicationListPatchesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListPatchesResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvbkxpc3RQYXRjaGVzUmVzcG9uc2USPAoHcGF0Y2hlcxgBIAMoCzIiLm9wZW'
        '5veGlkZS5hcGkudjEuQXBwbGljYXRpb25QYXRjaFIHcGF0Y2hlcw==');

@$core.Deprecated('Use applicationCreatePatchRequestDescriptor instead')
const ApplicationCreatePatchRequest$json = {
  '1': 'ApplicationCreatePatchRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {
      '1': 'patch_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPatchType',
      '8': {},
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'filePath'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'content'},
  ],
};

/// Descriptor for `ApplicationCreatePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreatePatchRequestDescriptor = $convert.base64Decode(
    'Ch1BcHBsaWNhdGlvbkNyZWF0ZVBhdGNoUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
    'IHukgEIgIgAFINYXBwbGljYXRpb25JZBJRCgpwYXRjaF90eXBlGAIgASgOMiYub3Blbm94aWRl'
    'LmFwaS52MS5BcHBsaWNhdGlvblBhdGNoVHlwZUIKukgHggEEEAEgAFIJcGF0Y2hUeXBlEiQKCW'
    'ZpbGVfcGF0aBgDIAEoCUIHukgEcgIQAVIIZmlsZVBhdGgSGAoHZW5hYmxlZBgEIAEoCFIHZW5h'
    'YmxlZBIjCgdjb250ZW50GAUgASgJQgm6SAZyBBiAgEBSB2NvbnRlbnQ=');

@$core.Deprecated('Use applicationCreatePatchResponseDescriptor instead')
const ApplicationCreatePatchResponse$json = {
  '1': 'ApplicationCreatePatchResponse',
  '2': [
    {
      '1': 'patch',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPatch',
      '10': 'patch'
    },
  ],
};

/// Descriptor for `ApplicationCreatePatchResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreatePatchResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvbkNyZWF0ZVBhdGNoUmVzcG9uc2USOAoFcGF0Y2gYASABKAsyIi5vcGVub3'
        'hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uUGF0Y2hSBXBhdGNo');

@$core.Deprecated('Use applicationUpdatePatchRequestDescriptor instead')
const ApplicationUpdatePatchRequest$json = {
  '1': 'ApplicationUpdatePatchRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'patch_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationPatchType',
      '8': {},
      '10': 'patchType'
    },
    {'1': 'file_path', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'filePath'},
    {'1': 'enabled', '3': 5, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'content', '3': 6, '4': 1, '5': 9, '8': {}, '10': 'content'},
  ],
};

/// Descriptor for `ApplicationUpdatePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdatePatchRequestDescriptor = $convert.base64Decode(
    'Ch1BcHBsaWNhdGlvblVwZGF0ZVBhdGNoUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
    'IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSUQoKcGF0'
    'Y2hfdHlwZRgDIAEoDjImLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRpb25QYXRjaFR5cGVCCr'
    'pIB4IBBBABIABSCXBhdGNoVHlwZRIkCglmaWxlX3BhdGgYBCABKAlCB7pIBHICEAFSCGZpbGVQ'
    'YXRoEhgKB2VuYWJsZWQYBSABKAhSB2VuYWJsZWQSIwoHY29udGVudBgGIAEoCUIJukgGcgQYgI'
    'BAUgdjb250ZW50');

@$core.Deprecated('Use applicationUpdatePatchResponseDescriptor instead')
const ApplicationUpdatePatchResponse$json = {
  '1': 'ApplicationUpdatePatchResponse',
  '2': [
    {
      '1': 'patch',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationPatch',
      '10': 'patch'
    },
  ],
};

/// Descriptor for `ApplicationUpdatePatchResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdatePatchResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvblVwZGF0ZVBhdGNoUmVzcG9uc2USOAoFcGF0Y2gYASABKAsyIi5vcGVub3'
        'hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uUGF0Y2hSBXBhdGNo');

@$core.Deprecated('Use applicationDeletePatchRequestDescriptor instead')
const ApplicationDeletePatchRequest$json = {
  '1': 'ApplicationDeletePatchRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeletePatchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeletePatchRequestDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkRlbGV0ZVBhdGNoUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
        'IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use applicationListMiddlewaresRequestDescriptor instead')
const ApplicationListMiddlewaresRequest$json = {
  '1': 'ApplicationListMiddlewaresRequest',
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

/// Descriptor for `ApplicationListMiddlewaresRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListMiddlewaresRequestDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvbkxpc3RNaWRkbGV3YXJlc1JlcXVlc3QSLgoOYXBwbGljYXRpb25faWQYAS'
        'ABKANCB7pIBCICIABSDWFwcGxpY2F0aW9uSWQ=');

@$core.Deprecated('Use applicationListMiddlewaresResponseDescriptor instead')
const ApplicationListMiddlewaresResponse$json = {
  '1': 'ApplicationListMiddlewaresResponse',
  '2': [
    {
      '1': 'middlewares',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddleware',
      '10': 'middlewares'
    },
  ],
};

/// Descriptor for `ApplicationListMiddlewaresResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListMiddlewaresResponseDescriptor =
    $convert.base64Decode(
        'CiJBcHBsaWNhdGlvbkxpc3RNaWRkbGV3YXJlc1Jlc3BvbnNlEkkKC21pZGRsZXdhcmVzGAEgAy'
        'gLMicub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1pZGRsZXdhcmVSC21pZGRsZXdhcmVz');

@$core.Deprecated('Use applicationCreateMiddlewareRequestDescriptor instead')
const ApplicationCreateMiddlewareRequest$json = {
  '1': 'ApplicationCreateMiddlewareRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'middleware_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMiddlewareType',
      '8': {},
      '10': 'middlewareType'
    },
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'headers',
      '3': 5,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddlewareHeader',
      '10': 'headers'
    },
    {
      '1': 'average',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'average',
      '17': true
    },
    {'1': 'burst', '3': 7, '4': 1, '5': 3, '9': 1, '10': 'burst', '17': true},
    {'1': 'source_ranges', '3': 8, '4': 3, '5': 9, '10': 'sourceRanges'},
  ],
  '8': [
    {'1': '_average'},
    {'1': '_burst'},
  ],
};

/// Descriptor for `ApplicationCreateMiddlewareRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateMiddlewareRequestDescriptor = $convert.base64Decode(
    'CiJBcHBsaWNhdGlvbkNyZWF0ZU1pZGRsZXdhcmVSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGA'
    'EgASgDQge6SAQiAiAAUg1hcHBsaWNhdGlvbklkEh4KBG5hbWUYAiABKAlCCrpIB3IFEAEY/wFS'
    'BG5hbWUSYAoPbWlkZGxld2FyZV90eXBlGAMgASgOMisub3Blbm94aWRlLmFwaS52MS5BcHBsaW'
    'NhdGlvbk1pZGRsZXdhcmVUeXBlQgq6SAeCAQQQASAAUg5taWRkbGV3YXJlVHlwZRIYCgdlbmFi'
    'bGVkGAQgASgIUgdlbmFibGVkEkcKB2hlYWRlcnMYBSADKAsyLS5vcGVub3hpZGUuYXBpLnYxLk'
    'FwcGxpY2F0aW9uTWlkZGxld2FyZUhlYWRlclIHaGVhZGVycxIdCgdhdmVyYWdlGAYgASgDSABS'
    'B2F2ZXJhZ2WIAQESGQoFYnVyc3QYByABKANIAVIFYnVyc3SIAQESIwoNc291cmNlX3Jhbmdlcx'
    'gIIAMoCVIMc291cmNlUmFuZ2VzQgoKCF9hdmVyYWdlQggKBl9idXJzdA==');

@$core.Deprecated('Use applicationCreateMiddlewareResponseDescriptor instead')
const ApplicationCreateMiddlewareResponse$json = {
  '1': 'ApplicationCreateMiddlewareResponse',
  '2': [
    {
      '1': 'middleware',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddleware',
      '10': 'middleware'
    },
  ],
};

/// Descriptor for `ApplicationCreateMiddlewareResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateMiddlewareResponseDescriptor =
    $convert.base64Decode(
        'CiNBcHBsaWNhdGlvbkNyZWF0ZU1pZGRsZXdhcmVSZXNwb25zZRJHCgptaWRkbGV3YXJlGAEgAS'
        'gLMicub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1pZGRsZXdhcmVSCm1pZGRsZXdhcmU=');

@$core.Deprecated('Use applicationUpdateMiddlewareRequestDescriptor instead')
const ApplicationUpdateMiddlewareRequest$json = {
  '1': 'ApplicationUpdateMiddlewareRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'middleware_type',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ApplicationMiddlewareType',
      '8': {},
      '10': 'middlewareType'
    },
    {'1': 'enabled', '3': 5, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'headers',
      '3': 6,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddlewareHeader',
      '10': 'headers'
    },
    {
      '1': 'average',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'average',
      '17': true
    },
    {'1': 'burst', '3': 8, '4': 1, '5': 3, '9': 1, '10': 'burst', '17': true},
    {'1': 'source_ranges', '3': 9, '4': 3, '5': 9, '10': 'sourceRanges'},
  ],
  '8': [
    {'1': '_average'},
    {'1': '_burst'},
  ],
};

/// Descriptor for `ApplicationUpdateMiddlewareRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateMiddlewareRequestDescriptor = $convert.base64Decode(
    'CiJBcHBsaWNhdGlvblVwZGF0ZU1pZGRsZXdhcmVSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGA'
    'EgASgDQge6SAQiAiAAUg1hcHBsaWNhdGlvbklkEhcKAmlkGAIgASgDQge6SAQiAiAAUgJpZBIe'
    'CgRuYW1lGAMgASgJQgq6SAdyBRABGP8BUgRuYW1lEmAKD21pZGRsZXdhcmVfdHlwZRgEIAEoDj'
    'IrLm9wZW5veGlkZS5hcGkudjEuQXBwbGljYXRpb25NaWRkbGV3YXJlVHlwZUIKukgHggEEEAEg'
    'AFIObWlkZGxld2FyZVR5cGUSGAoHZW5hYmxlZBgFIAEoCFIHZW5hYmxlZBJHCgdoZWFkZXJzGA'
    'YgAygLMi0ub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1pZGRsZXdhcmVIZWFkZXJSB2hl'
    'YWRlcnMSHQoHYXZlcmFnZRgHIAEoA0gAUgdhdmVyYWdliAEBEhkKBWJ1cnN0GAggASgDSAFSBW'
    'J1cnN0iAEBEiMKDXNvdXJjZV9yYW5nZXMYCSADKAlSDHNvdXJjZVJhbmdlc0IKCghfYXZlcmFn'
    'ZUIICgZfYnVyc3Q=');

@$core.Deprecated('Use applicationUpdateMiddlewareResponseDescriptor instead')
const ApplicationUpdateMiddlewareResponse$json = {
  '1': 'ApplicationUpdateMiddlewareResponse',
  '2': [
    {
      '1': 'middleware',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationMiddleware',
      '10': 'middleware'
    },
  ],
};

/// Descriptor for `ApplicationUpdateMiddlewareResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateMiddlewareResponseDescriptor =
    $convert.base64Decode(
        'CiNBcHBsaWNhdGlvblVwZGF0ZU1pZGRsZXdhcmVSZXNwb25zZRJHCgptaWRkbGV3YXJlGAEgAS'
        'gLMicub3Blbm94aWRlLmFwaS52MS5BcHBsaWNhdGlvbk1pZGRsZXdhcmVSCm1pZGRsZXdhcmU=');

@$core.Deprecated('Use applicationDeleteMiddlewareRequestDescriptor instead')
const ApplicationDeleteMiddlewareRequest$json = {
  '1': 'ApplicationDeleteMiddlewareRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeleteMiddlewareRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeleteMiddlewareRequestDescriptor =
    $convert.base64Decode(
        'CiJBcHBsaWNhdGlvbkRlbGV0ZU1pZGRsZXdhcmVSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGA'
        'EgASgDQge6SAQiAiAAUg1hcHBsaWNhdGlvbklkEhcKAmlkGAIgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use applicationListRedirectsRequestDescriptor instead')
const ApplicationListRedirectsRequest$json = {
  '1': 'ApplicationListRedirectsRequest',
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

/// Descriptor for `ApplicationListRedirectsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListRedirectsRequestDescriptor =
    $convert.base64Decode(
        'Ch9BcHBsaWNhdGlvbkxpc3RSZWRpcmVjdHNSZXF1ZXN0Ei4KDmFwcGxpY2F0aW9uX2lkGAEgAS'
        'gDQge6SAQiAiAAUg1hcHBsaWNhdGlvbklk');

@$core.Deprecated('Use applicationListRedirectsResponseDescriptor instead')
const ApplicationListRedirectsResponse$json = {
  '1': 'ApplicationListRedirectsResponse',
  '2': [
    {
      '1': 'redirects',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationRedirect',
      '10': 'redirects'
    },
  ],
};

/// Descriptor for `ApplicationListRedirectsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationListRedirectsResponseDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvbkxpc3RSZWRpcmVjdHNSZXNwb25zZRJDCglyZWRpcmVjdHMYASADKAsyJS'
        '5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uUmVkaXJlY3RSCXJlZGlyZWN0cw==');

@$core.Deprecated('Use applicationCreateRedirectRequestDescriptor instead')
const ApplicationCreateRedirectRequest$json = {
  '1': 'ApplicationCreateRedirectRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'regex', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'regex'},
    {'1': 'replacement', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'replacement'},
    {'1': 'permanent', '3': 4, '4': 1, '5': 8, '10': 'permanent'},
    {
      '1': 'unique_config_key',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'uniqueConfigKey',
      '17': true
    },
  ],
  '8': [
    {'1': '_unique_config_key'},
  ],
};

/// Descriptor for `ApplicationCreateRedirectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateRedirectRequestDescriptor = $convert.base64Decode(
    'CiBBcHBsaWNhdGlvbkNyZWF0ZVJlZGlyZWN0UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
    'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIdCgVyZWdleBgCIAEoCUIHukgEcgIQAVIFcmVn'
    'ZXgSKQoLcmVwbGFjZW1lbnQYAyABKAlCB7pIBHICEAFSC3JlcGxhY2VtZW50EhwKCXBlcm1hbm'
    'VudBgEIAEoCFIJcGVybWFuZW50Ei8KEXVuaXF1ZV9jb25maWdfa2V5GAUgASgDSABSD3VuaXF1'
    'ZUNvbmZpZ0tleYgBAUIUChJfdW5pcXVlX2NvbmZpZ19rZXk=');

@$core.Deprecated('Use applicationCreateRedirectResponseDescriptor instead')
const ApplicationCreateRedirectResponse$json = {
  '1': 'ApplicationCreateRedirectResponse',
  '2': [
    {
      '1': 'redirect',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationRedirect',
      '10': 'redirect'
    },
  ],
};

/// Descriptor for `ApplicationCreateRedirectResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateRedirectResponseDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvbkNyZWF0ZVJlZGlyZWN0UmVzcG9uc2USQQoIcmVkaXJlY3QYASABKAsyJS'
        '5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uUmVkaXJlY3RSCHJlZGlyZWN0');

@$core.Deprecated('Use applicationUpdateRedirectRequestDescriptor instead')
const ApplicationUpdateRedirectRequest$json = {
  '1': 'ApplicationUpdateRedirectRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'regex', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'regex'},
    {'1': 'replacement', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'replacement'},
    {'1': 'permanent', '3': 5, '4': 1, '5': 8, '10': 'permanent'},
    {
      '1': 'unique_config_key',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'uniqueConfigKey',
      '17': true
    },
  ],
  '8': [
    {'1': '_unique_config_key'},
  ],
};

/// Descriptor for `ApplicationUpdateRedirectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateRedirectRequestDescriptor = $convert.base64Decode(
    'CiBBcHBsaWNhdGlvblVwZGF0ZVJlZGlyZWN0UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
    'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSHQoF'
    'cmVnZXgYAyABKAlCB7pIBHICEAFSBXJlZ2V4EikKC3JlcGxhY2VtZW50GAQgASgJQge6SARyAh'
    'ABUgtyZXBsYWNlbWVudBIcCglwZXJtYW5lbnQYBSABKAhSCXBlcm1hbmVudBIvChF1bmlxdWVf'
    'Y29uZmlnX2tleRgGIAEoA0gAUg91bmlxdWVDb25maWdLZXmIAQFCFAoSX3VuaXF1ZV9jb25maW'
    'dfa2V5');

@$core.Deprecated('Use applicationUpdateRedirectResponseDescriptor instead')
const ApplicationUpdateRedirectResponse$json = {
  '1': 'ApplicationUpdateRedirectResponse',
  '2': [
    {
      '1': 'redirect',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationRedirect',
      '10': 'redirect'
    },
  ],
};

/// Descriptor for `ApplicationUpdateRedirectResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateRedirectResponseDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvblVwZGF0ZVJlZGlyZWN0UmVzcG9uc2USQQoIcmVkaXJlY3QYASABKAsyJS'
        '5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uUmVkaXJlY3RSCHJlZGlyZWN0');

@$core.Deprecated('Use applicationDeleteRedirectRequestDescriptor instead')
const ApplicationDeleteRedirectRequest$json = {
  '1': 'ApplicationDeleteRedirectRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeleteRedirectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeleteRedirectRequestDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvbkRlbGV0ZVJlZGlyZWN0UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use applicationGetSecurityRequestDescriptor instead')
const ApplicationGetSecurityRequest$json = {
  '1': 'ApplicationGetSecurityRequest',
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

/// Descriptor for `ApplicationGetSecurityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationGetSecurityRequestDescriptor =
    $convert.base64Decode(
        'Ch1BcHBsaWNhdGlvbkdldFNlY3VyaXR5UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0'
        'IHukgEIgIgAFINYXBwbGljYXRpb25JZA==');

@$core.Deprecated('Use applicationGetSecurityResponseDescriptor instead')
const ApplicationGetSecurityResponse$json = {
  '1': 'ApplicationGetSecurityResponse',
  '2': [
    {
      '1': 'security',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationSecurityRule',
      '10': 'security'
    },
  ],
};

/// Descriptor for `ApplicationGetSecurityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationGetSecurityResponseDescriptor =
    $convert.base64Decode(
        'Ch5BcHBsaWNhdGlvbkdldFNlY3VyaXR5UmVzcG9uc2USRQoIc2VjdXJpdHkYASADKAsyKS5vcG'
        'Vub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uU2VjdXJpdHlSdWxlUghzZWN1cml0eQ==');

@$core.Deprecated('Use applicationCreateSecurityRequestDescriptor instead')
const ApplicationCreateSecurityRequest$json = {
  '1': 'ApplicationCreateSecurityRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'username', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {'1': 'password', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'password'},
  ],
};

/// Descriptor for `ApplicationCreateSecurityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateSecurityRequestDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvbkNyZWF0ZVNlY3VyaXR5UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBImCgh1c2VybmFtZRgCIAEoCUIKukgHcgUQARj/'
        'AVIIdXNlcm5hbWUSIwoIcGFzc3dvcmQYAyABKAlCB7pIBHICEAFSCHBhc3N3b3Jk');

@$core.Deprecated('Use applicationCreateSecurityResponseDescriptor instead')
const ApplicationCreateSecurityResponse$json = {
  '1': 'ApplicationCreateSecurityResponse',
  '2': [
    {
      '1': 'security',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationSecurityRule',
      '10': 'security'
    },
  ],
};

/// Descriptor for `ApplicationCreateSecurityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationCreateSecurityResponseDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvbkNyZWF0ZVNlY3VyaXR5UmVzcG9uc2USRQoIc2VjdXJpdHkYASABKAsyKS'
        '5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uU2VjdXJpdHlSdWxlUghzZWN1cml0eQ==');

@$core.Deprecated('Use applicationUpdateSecurityRequestDescriptor instead')
const ApplicationUpdateSecurityRequest$json = {
  '1': 'ApplicationUpdateSecurityRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'username', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {'1': 'password', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'password'},
  ],
};

/// Descriptor for `ApplicationUpdateSecurityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateSecurityRequestDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvblVwZGF0ZVNlY3VyaXR5UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQSJgoI'
        'dXNlcm5hbWUYAyABKAlCCrpIB3IFEAEY/wFSCHVzZXJuYW1lEiMKCHBhc3N3b3JkGAQgASgJQg'
        'e6SARyAhABUghwYXNzd29yZA==');

@$core.Deprecated('Use applicationUpdateSecurityResponseDescriptor instead')
const ApplicationUpdateSecurityResponse$json = {
  '1': 'ApplicationUpdateSecurityResponse',
  '2': [
    {
      '1': 'security',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ApplicationSecurityRule',
      '10': 'security'
    },
  ],
};

/// Descriptor for `ApplicationUpdateSecurityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateSecurityResponseDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvblVwZGF0ZVNlY3VyaXR5UmVzcG9uc2USRQoIc2VjdXJpdHkYASABKAsyKS'
        '5vcGVub3hpZGUuYXBpLnYxLkFwcGxpY2F0aW9uU2VjdXJpdHlSdWxlUghzZWN1cml0eQ==');

@$core.Deprecated('Use applicationDeleteSecurityRequestDescriptor instead')
const ApplicationDeleteSecurityRequest$json = {
  '1': 'ApplicationDeleteSecurityRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ApplicationDeleteSecurityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationDeleteSecurityRequestDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvbkRlbGV0ZVNlY3VyaXR5UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIXCgJpZBgCIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use applicationUpdateNetworksRequestDescriptor instead')
const ApplicationUpdateNetworksRequest$json = {
  '1': 'ApplicationUpdateNetworksRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'network_ids', '3': 2, '4': 3, '5': 3, '10': 'networkIds'},
    {
      '1': 'detach_rustploy_network',
      '3': 3,
      '4': 1,
      '5': 8,
      '10': 'detachRustployNetwork'
    },
  ],
};

/// Descriptor for `ApplicationUpdateNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateNetworksRequestDescriptor =
    $convert.base64Decode(
        'CiBBcHBsaWNhdGlvblVwZGF0ZU5ldHdvcmtzUmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAFINYXBwbGljYXRpb25JZBIfCgtuZXR3b3JrX2lkcxgCIAMoA1IKbmV0d29y'
        'a0lkcxI2ChdkZXRhY2hfcnVzdHBsb3lfbmV0d29yaxgDIAEoCFIVZGV0YWNoUnVzdHBsb3lOZX'
        'R3b3Jr');

@$core.Deprecated('Use applicationUpdateNetworksResponseDescriptor instead')
const ApplicationUpdateNetworksResponse$json = {
  '1': 'ApplicationUpdateNetworksResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'operation', '3': 2, '4': 1, '5': 9, '10': 'operation'},
  ],
};

/// Descriptor for `ApplicationUpdateNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applicationUpdateNetworksResponseDescriptor =
    $convert.base64Decode(
        'CiFBcHBsaWNhdGlvblVwZGF0ZU5ldHdvcmtzUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3'
        'VjY2VzcxIcCglvcGVyYXRpb24YAiABKAlSCW9wZXJhdGlvbg==');

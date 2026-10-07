// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_controller.proto.

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

@$core.Deprecated('Use serverSetupStepDescriptor instead')
const ServerSetupStep$json = {
  '1': 'ServerSetupStep',
  '2': [
    {'1': 'SERVER_SETUP_STEP_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_SETUP_STEP_DEPENDENCIES', '2': 1},
    {'1': 'SERVER_SETUP_STEP_BUILD_TOOLS', '2': 2},
    {'1': 'SERVER_SETUP_STEP_DIRECTORIES', '2': 3},
    {'1': 'SERVER_SETUP_STEP_SWARM', '2': 4},
    {'1': 'SERVER_SETUP_STEP_NETWORK', '2': 5},
    {'1': 'SERVER_SETUP_STEP_TRAEFIK_CONFIG', '2': 6},
    {'1': 'SERVER_SETUP_STEP_TRAEFIK', '2': 7},
    {'1': 'SERVER_SETUP_STEP_AGENT', '2': 8},
  ],
};

/// Descriptor for `ServerSetupStep`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverSetupStepDescriptor = $convert.base64Decode(
    'Cg9TZXJ2ZXJTZXR1cFN0ZXASIQodU0VSVkVSX1NFVFVQX1NURVBfVU5TUEVDSUZJRUQQABIiCh'
    '5TRVJWRVJfU0VUVVBfU1RFUF9ERVBFTkRFTkNJRVMQARIhCh1TRVJWRVJfU0VUVVBfU1RFUF9C'
    'VUlMRF9UT09MUxACEiEKHVNFUlZFUl9TRVRVUF9TVEVQX0RJUkVDVE9SSUVTEAMSGwoXU0VSVk'
    'VSX1NFVFVQX1NURVBfU1dBUk0QBBIdChlTRVJWRVJfU0VUVVBfU1RFUF9ORVRXT1JLEAUSJAog'
    'U0VSVkVSX1NFVFVQX1NURVBfVFJBRUZJS19DT05GSUcQBhIdChlTRVJWRVJfU0VUVVBfU1RFUF'
    '9UUkFFRklLEAcSGwoXU0VSVkVSX1NFVFVQX1NURVBfQUdFTlQQCA==');

@$core.Deprecated('Use serverToolStateDescriptor instead')
const ServerToolState$json = {
  '1': 'ServerToolState',
  '2': [
    {'1': 'installed', '3': 1, '4': 1, '5': 8, '10': 'installed'},
    {
      '1': 'version',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'version',
      '17': true
    },
  ],
  '8': [
    {'1': '_version'},
  ],
};

/// Descriptor for `ServerToolState`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverToolStateDescriptor = $convert.base64Decode(
    'Cg9TZXJ2ZXJUb29sU3RhdGUSHAoJaW5zdGFsbGVkGAEgASgIUglpbnN0YWxsZWQSHQoHdmVyc2'
    'lvbhgCIAEoCUgAUgd2ZXJzaW9uiAEBQgoKCF92ZXJzaW9u');

@$core.Deprecated('Use serverPortAvailabilityDescriptor instead')
const ServerPortAvailability$json = {
  '1': 'ServerPortAvailability',
  '2': [
    {'1': 'port', '3': 1, '4': 1, '5': 13, '10': 'port'},
    {'1': 'available', '3': 2, '4': 1, '5': 8, '10': 'available'},
  ],
};

/// Descriptor for `ServerPortAvailability`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverPortAvailabilityDescriptor =
    $convert.base64Decode(
        'ChZTZXJ2ZXJQb3J0QXZhaWxhYmlsaXR5EhIKBHBvcnQYASABKA1SBHBvcnQSHAoJYXZhaWxhYm'
        'xlGAIgASgIUglhdmFpbGFibGU=');

@$core.Deprecated('Use serverAuditDescriptor instead')
const ServerAudit$json = {
  '1': 'ServerAudit',
  '2': [
    {'1': 'os_id', '3': 1, '4': 1, '5': 9, '10': 'osId'},
    {'1': 'architecture', '3': 2, '4': 1, '5': 9, '10': 'architecture'},
    {
      '1': 'docker',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'docker'
    },
    {
      '1': 'git',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'git'
    },
    {
      '1': 'rclone',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'rclone'
    },
    {
      '1': 'rsync',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'rsync'
    },
    {
      '1': 'nixpacks',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'nixpacks'
    },
    {
      '1': 'railpack',
      '3': 8,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'railpack'
    },
    {
      '1': 'buildpacks',
      '3': 9,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerToolState',
      '10': 'buildpacks'
    },
    {'1': 'swarm_active', '3': 10, '4': 1, '5': 8, '10': 'swarmActive'},
    {'1': 'network_exists', '3': 11, '4': 1, '5': 8, '10': 'networkExists'},
    {
      '1': 'base_directory_exists',
      '3': 12,
      '4': 1,
      '5': 8,
      '10': 'baseDirectoryExists'
    },
    {
      '1': 'docker_group_member',
      '3': 13,
      '4': 1,
      '5': 8,
      '10': 'dockerGroupMember'
    },
    {
      '1': 'ports',
      '3': 14,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPortAvailability',
      '10': 'ports'
    },
  ],
};

/// Descriptor for `ServerAudit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverAuditDescriptor = $convert.base64Decode(
    'CgtTZXJ2ZXJBdWRpdBITCgVvc19pZBgBIAEoCVIEb3NJZBIiCgxhcmNoaXRlY3R1cmUYAiABKA'
    'lSDGFyY2hpdGVjdHVyZRI5CgZkb2NrZXIYAyABKAsyIS5vcGVub3hpZGUuYXBpLnYxLlNlcnZl'
    'clRvb2xTdGF0ZVIGZG9ja2VyEjMKA2dpdBgEIAEoCzIhLm9wZW5veGlkZS5hcGkudjEuU2Vydm'
    'VyVG9vbFN0YXRlUgNnaXQSOQoGcmNsb25lGAUgASgLMiEub3Blbm94aWRlLmFwaS52MS5TZXJ2'
    'ZXJUb29sU3RhdGVSBnJjbG9uZRI3CgVyc3luYxgGIAEoCzIhLm9wZW5veGlkZS5hcGkudjEuU2'
    'VydmVyVG9vbFN0YXRlUgVyc3luYxI9CghuaXhwYWNrcxgHIAEoCzIhLm9wZW5veGlkZS5hcGku'
    'djEuU2VydmVyVG9vbFN0YXRlUghuaXhwYWNrcxI9CghyYWlscGFjaxgIIAEoCzIhLm9wZW5veG'
    'lkZS5hcGkudjEuU2VydmVyVG9vbFN0YXRlUghyYWlscGFjaxJBCgpidWlsZHBhY2tzGAkgASgL'
    'MiEub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJUb29sU3RhdGVSCmJ1aWxkcGFja3MSIQoMc3dhcm'
    '1fYWN0aXZlGAogASgIUgtzd2FybUFjdGl2ZRIlCg5uZXR3b3JrX2V4aXN0cxgLIAEoCFINbmV0'
    'd29ya0V4aXN0cxIyChViYXNlX2RpcmVjdG9yeV9leGlzdHMYDCABKAhSE2Jhc2VEaXJlY3Rvcn'
    'lFeGlzdHMSLgoTZG9ja2VyX2dyb3VwX21lbWJlchgNIAEoCFIRZG9ja2VyR3JvdXBNZW1iZXIS'
    'PgoFcG9ydHMYDiADKAsyKC5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclBvcnRBdmFpbGFiaWxpdH'
    'lSBXBvcnRz');

@$core.Deprecated('Use serverSetupOutcomeDescriptor instead')
const ServerSetupOutcome$json = {
  '1': 'ServerSetupOutcome',
  '2': [
    {
      '1': 'completed',
      '3': 1,
      '4': 3,
      '5': 14,
      '6': '.openoxide.api.v1.ServerSetupStep',
      '10': 'completed'
    },
    {
      '1': 'audit',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerAudit',
      '10': 'audit'
    },
  ],
};

/// Descriptor for `ServerSetupOutcome`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverSetupOutcomeDescriptor = $convert.base64Decode(
    'ChJTZXJ2ZXJTZXR1cE91dGNvbWUSPwoJY29tcGxldGVkGAEgAygOMiEub3Blbm94aWRlLmFwaS'
    '52MS5TZXJ2ZXJTZXR1cFN0ZXBSCWNvbXBsZXRlZBIzCgVhdWRpdBgCIAEoCzIdLm9wZW5veGlk'
    'ZS5hcGkudjEuU2VydmVyQXVkaXRSBWF1ZGl0');

@$core.Deprecated('Use serverConnectionInfoDescriptor instead')
const ServerConnectionInfo$json = {
  '1': 'ServerConnectionInfo',
  '2': [
    {'1': 'connected', '3': 1, '4': 1, '5': 8, '10': 'connected'},
    {'1': 'reused_sessions', '3': 2, '4': 1, '5': 4, '10': 'reusedSessions'},
    {'1': 'max_pool_size', '3': 3, '4': 1, '5': 4, '10': 'maxPoolSize'},
    {'1': 'connections', '3': 4, '4': 1, '5': 4, '10': 'connections'},
    {'1': 'active_channels', '3': 5, '4': 1, '5': 4, '10': 'activeChannels'},
    {
      '1': 'max_channels_per_session',
      '3': 6,
      '4': 1,
      '5': 4,
      '10': 'maxChannelsPerSession'
    },
  ],
};

/// Descriptor for `ServerConnectionInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverConnectionInfoDescriptor = $convert.base64Decode(
    'ChRTZXJ2ZXJDb25uZWN0aW9uSW5mbxIcCgljb25uZWN0ZWQYASABKAhSCWNvbm5lY3RlZBInCg'
    '9yZXVzZWRfc2Vzc2lvbnMYAiABKARSDnJldXNlZFNlc3Npb25zEiIKDW1heF9wb29sX3NpemUY'
    'AyABKARSC21heFBvb2xTaXplEiAKC2Nvbm5lY3Rpb25zGAQgASgEUgtjb25uZWN0aW9ucxInCg'
    '9hY3RpdmVfY2hhbm5lbHMYBSABKARSDmFjdGl2ZUNoYW5uZWxzEjcKGG1heF9jaGFubmVsc19w'
    'ZXJfc2Vzc2lvbhgGIAEoBFIVbWF4Q2hhbm5lbHNQZXJTZXNzaW9u');

@$core.Deprecated(
    'Use serverControllerTestDirectConnectionRequestDescriptor instead')
const ServerControllerTestDirectConnectionRequest$json = {
  '1': 'ServerControllerTestDirectConnectionRequest',
  '2': [
    {'1': 'ip_address', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'ipAddress'},
    {
      '1': 'port',
      '3': 2,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 0,
      '10': 'port',
      '17': true
    },
    {'1': 'username', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {'1': 'ssh_key_id', '3': 4, '4': 1, '5': 3, '8': {}, '10': 'sshKeyId'},
  ],
  '8': [
    {'1': '_port'},
  ],
};

/// Descriptor for `ServerControllerTestDirectConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    serverControllerTestDirectConnectionRequestDescriptor =
    $convert.base64Decode(
        'CitTZXJ2ZXJDb250cm9sbGVyVGVzdERpcmVjdENvbm5lY3Rpb25SZXF1ZXN0EikKCmlwX2FkZH'
        'Jlc3MYASABKAlCCrpIB3IFEAEY/wFSCWlwQWRkcmVzcxIiCgRwb3J0GAIgASgNQgm6SAYqBBj/'
        '/wNIAFIEcG9ydIgBARImCgh1c2VybmFtZRgDIAEoCUIKukgHcgUQARj/AVIIdXNlcm5hbWUSJQ'
        'oKc3NoX2tleV9pZBgEIAEoA0IHukgEIgIgAFIIc3NoS2V5SWRCBwoFX3BvcnQ=');

@$core.Deprecated(
    'Use serverControllerTestDirectConnectionResponseDescriptor instead')
const ServerControllerTestDirectConnectionResponse$json = {
  '1': 'ServerControllerTestDirectConnectionResponse',
  '2': [
    {
      '1': 'connection',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerConnectionInfo',
      '10': 'connection'
    },
  ],
};

/// Descriptor for `ServerControllerTestDirectConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List
    serverControllerTestDirectConnectionResponseDescriptor =
    $convert.base64Decode(
        'CixTZXJ2ZXJDb250cm9sbGVyVGVzdERpcmVjdENvbm5lY3Rpb25SZXNwb25zZRJGCgpjb25uZW'
        'N0aW9uGAEgASgLMiYub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJDb25uZWN0aW9uSW5mb1IKY29u'
        'bmVjdGlvbg==');

@$core.Deprecated('Use serverControllerTestConnectionRequestDescriptor instead')
const ServerControllerTestConnectionRequest$json = {
  '1': 'ServerControllerTestConnectionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'host_key_fingerprint',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'hostKeyFingerprint',
      '17': true
    },
    {
      '1': 'sudo_password',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'sudoPassword',
      '17': true
    },
    {
      '1': 'pool_size',
      '3': 4,
      '4': 1,
      '5': 13,
      '9': 2,
      '10': 'poolSize',
      '17': true
    },
  ],
  '8': [
    {'1': '_host_key_fingerprint'},
    {'1': '_sudo_password'},
    {'1': '_pool_size'},
  ],
};

/// Descriptor for `ServerControllerTestConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerTestConnectionRequestDescriptor =
    $convert.base64Decode(
        'CiVTZXJ2ZXJDb250cm9sbGVyVGVzdENvbm5lY3Rpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SA'
        'QiAiAAUgJpZBI/ChRob3N0X2tleV9maW5nZXJwcmludBgCIAEoCUIIukgFcgMY/wFIAFISaG9z'
        'dEtleUZpbmdlcnByaW50iAEBEigKDXN1ZG9fcGFzc3dvcmQYAyABKAlIAVIMc3Vkb1Bhc3N3b3'
        'JkiAEBEiAKCXBvb2xfc2l6ZRgEIAEoDUgCUghwb29sU2l6ZYgBAUIXChVfaG9zdF9rZXlfZmlu'
        'Z2VycHJpbnRCEAoOX3N1ZG9fcGFzc3dvcmRCDAoKX3Bvb2xfc2l6ZQ==');

@$core
    .Deprecated('Use serverControllerTestConnectionResponseDescriptor instead')
const ServerControllerTestConnectionResponse$json = {
  '1': 'ServerControllerTestConnectionResponse',
  '2': [
    {
      '1': 'connection',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerConnectionInfo',
      '10': 'connection'
    },
  ],
};

/// Descriptor for `ServerControllerTestConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerTestConnectionResponseDescriptor =
    $convert.base64Decode(
        'CiZTZXJ2ZXJDb250cm9sbGVyVGVzdENvbm5lY3Rpb25SZXNwb25zZRJGCgpjb25uZWN0aW9uGA'
        'EgASgLMiYub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJDb25uZWN0aW9uSW5mb1IKY29ubmVjdGlv'
        'bg==');

@$core.Deprecated('Use serverControllerAuditRequestDescriptor instead')
const ServerControllerAuditRequest$json = {
  '1': 'ServerControllerAuditRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'host_key_fingerprint',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'hostKeyFingerprint',
      '17': true
    },
    {
      '1': 'sudo_password',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'sudoPassword',
      '17': true
    },
    {
      '1': 'pool_size',
      '3': 4,
      '4': 1,
      '5': 13,
      '9': 2,
      '10': 'poolSize',
      '17': true
    },
  ],
  '8': [
    {'1': '_host_key_fingerprint'},
    {'1': '_sudo_password'},
    {'1': '_pool_size'},
  ],
};

/// Descriptor for `ServerControllerAuditRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerAuditRequestDescriptor = $convert.base64Decode(
    'ChxTZXJ2ZXJDb250cm9sbGVyQXVkaXRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZB'
    'I/ChRob3N0X2tleV9maW5nZXJwcmludBgCIAEoCUIIukgFcgMY/wFIAFISaG9zdEtleUZpbmdl'
    'cnByaW50iAEBEigKDXN1ZG9fcGFzc3dvcmQYAyABKAlIAVIMc3Vkb1Bhc3N3b3JkiAEBEiAKCX'
    'Bvb2xfc2l6ZRgEIAEoDUgCUghwb29sU2l6ZYgBAUIXChVfaG9zdF9rZXlfZmluZ2VycHJpbnRC'
    'EAoOX3N1ZG9fcGFzc3dvcmRCDAoKX3Bvb2xfc2l6ZQ==');

@$core.Deprecated('Use serverControllerAuditResponseDescriptor instead')
const ServerControllerAuditResponse$json = {
  '1': 'ServerControllerAuditResponse',
  '2': [
    {
      '1': 'audit',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerAudit',
      '10': 'audit'
    },
  ],
};

/// Descriptor for `ServerControllerAuditResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerAuditResponseDescriptor =
    $convert.base64Decode(
        'Ch1TZXJ2ZXJDb250cm9sbGVyQXVkaXRSZXNwb25zZRIzCgVhdWRpdBgBIAEoCzIdLm9wZW5veG'
        'lkZS5hcGkudjEuU2VydmVyQXVkaXRSBWF1ZGl0');

@$core.Deprecated('Use serverControllerSetupRequestDescriptor instead')
const ServerControllerSetupRequest$json = {
  '1': 'ServerControllerSetupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'install_dependencies',
      '3': 2,
      '4': 1,
      '5': 8,
      '10': 'installDependencies'
    },
    {
      '1': 'advertise_addr',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'advertiseAddr',
      '17': true
    },
    {
      '1': 'acme_email',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'acmeEmail',
      '17': true
    },
    {
      '1': 'host_key_fingerprint',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'hostKeyFingerprint',
      '17': true
    },
    {
      '1': 'sudo_password',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'sudoPassword',
      '17': true
    },
    {
      '1': 'pool_size',
      '3': 7,
      '4': 1,
      '5': 13,
      '9': 4,
      '10': 'poolSize',
      '17': true
    },
  ],
  '8': [
    {'1': '_advertise_addr'},
    {'1': '_acme_email'},
    {'1': '_host_key_fingerprint'},
    {'1': '_sudo_password'},
    {'1': '_pool_size'},
  ],
};

/// Descriptor for `ServerControllerSetupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerSetupRequestDescriptor = $convert.base64Decode(
    'ChxTZXJ2ZXJDb250cm9sbGVyU2V0dXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZB'
    'IxChRpbnN0YWxsX2RlcGVuZGVuY2llcxgCIAEoCFITaW5zdGFsbERlcGVuZGVuY2llcxI0Cg5h'
    'ZHZlcnRpc2VfYWRkchgDIAEoCUIIukgFcgMY/wFIAFINYWR2ZXJ0aXNlQWRkcogBARIsCgphY2'
    '1lX2VtYWlsGAQgASgJQgi6SAVyAxj/AUgBUglhY21lRW1haWyIAQESPwoUaG9zdF9rZXlfZmlu'
    'Z2VycHJpbnQYBSABKAlCCLpIBXIDGP8BSAJSEmhvc3RLZXlGaW5nZXJwcmludIgBARIoCg1zdW'
    'RvX3Bhc3N3b3JkGAYgASgJSANSDHN1ZG9QYXNzd29yZIgBARIgCglwb29sX3NpemUYByABKA1I'
    'BFIIcG9vbFNpemWIAQFCEQoPX2FkdmVydGlzZV9hZGRyQg0KC19hY21lX2VtYWlsQhcKFV9ob3'
    'N0X2tleV9maW5nZXJwcmludEIQCg5fc3Vkb19wYXNzd29yZEIMCgpfcG9vbF9zaXpl');

@$core.Deprecated('Use serverControllerSetupResponseDescriptor instead')
const ServerControllerSetupResponse$json = {
  '1': 'ServerControllerSetupResponse',
  '2': [
    {
      '1': 'outcome',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerSetupOutcome',
      '10': 'outcome'
    },
  ],
};

/// Descriptor for `ServerControllerSetupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerSetupResponseDescriptor =
    $convert.base64Decode(
        'Ch1TZXJ2ZXJDb250cm9sbGVyU2V0dXBSZXNwb25zZRI+CgdvdXRjb21lGAEgASgLMiQub3Blbm'
        '94aWRlLmFwaS52MS5TZXJ2ZXJTZXR1cE91dGNvbWVSB291dGNvbWU=');

@$core
    .Deprecated('Use serverControllerStreamSetupLogsRequestDescriptor instead')
const ServerControllerStreamSetupLogsRequest$json = {
  '1': 'ServerControllerStreamSetupLogsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'install_dependencies',
      '3': 2,
      '4': 1,
      '5': 8,
      '10': 'installDependencies'
    },
    {
      '1': 'advertise_addr',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'advertiseAddr',
      '17': true
    },
    {
      '1': 'acme_email',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'acmeEmail',
      '17': true
    },
    {
      '1': 'host_key_fingerprint',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'hostKeyFingerprint',
      '17': true
    },
    {
      '1': 'sudo_password',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'sudoPassword',
      '17': true
    },
    {
      '1': 'pool_size',
      '3': 7,
      '4': 1,
      '5': 13,
      '9': 4,
      '10': 'poolSize',
      '17': true
    },
  ],
  '8': [
    {'1': '_advertise_addr'},
    {'1': '_acme_email'},
    {'1': '_host_key_fingerprint'},
    {'1': '_sudo_password'},
    {'1': '_pool_size'},
  ],
};

/// Descriptor for `ServerControllerStreamSetupLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerStreamSetupLogsRequestDescriptor = $convert.base64Decode(
    'CiZTZXJ2ZXJDb250cm9sbGVyU3RyZWFtU2V0dXBMb2dzUmVxdWVzdBIXCgJpZBgBIAEoA0IHuk'
    'gEIgIgAFICaWQSMQoUaW5zdGFsbF9kZXBlbmRlbmNpZXMYAiABKAhSE2luc3RhbGxEZXBlbmRl'
    'bmNpZXMSNAoOYWR2ZXJ0aXNlX2FkZHIYAyABKAlCCLpIBXIDGP8BSABSDWFkdmVydGlzZUFkZH'
    'KIAQESLAoKYWNtZV9lbWFpbBgEIAEoCUIIukgFcgMY/wFIAVIJYWNtZUVtYWlsiAEBEj8KFGhv'
    'c3Rfa2V5X2ZpbmdlcnByaW50GAUgASgJQgi6SAVyAxj/AUgCUhJob3N0S2V5RmluZ2VycHJpbn'
    'SIAQESKAoNc3Vkb19wYXNzd29yZBgGIAEoCUgDUgxzdWRvUGFzc3dvcmSIAQESIAoJcG9vbF9z'
    'aXplGAcgASgNSARSCHBvb2xTaXpliAEBQhEKD19hZHZlcnRpc2VfYWRkckINCgtfYWNtZV9lbW'
    'FpbEIXChVfaG9zdF9rZXlfZmluZ2VycHJpbnRCEAoOX3N1ZG9fcGFzc3dvcmRCDAoKX3Bvb2xf'
    'c2l6ZQ==');

@$core
    .Deprecated('Use serverControllerStreamSetupLogsResponseDescriptor instead')
const ServerControllerStreamSetupLogsResponse$json = {
  '1': 'ServerControllerStreamSetupLogsResponse',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '10': 'serverId'},
    {
      '1': 'outcome',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerSetupOutcome',
      '10': 'outcome'
    },
    {'1': 'lines', '3': 3, '4': 3, '5': 9, '10': 'lines'},
  ],
};

/// Descriptor for `ServerControllerStreamSetupLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerStreamSetupLogsResponseDescriptor =
    $convert.base64Decode(
        'CidTZXJ2ZXJDb250cm9sbGVyU3RyZWFtU2V0dXBMb2dzUmVzcG9uc2USGwoJc2VydmVyX2lkGA'
        'EgASgDUghzZXJ2ZXJJZBI+CgdvdXRjb21lGAIgASgLMiQub3Blbm94aWRlLmFwaS52MS5TZXJ2'
        'ZXJTZXR1cE91dGNvbWVSB291dGNvbWUSFAoFbGluZXMYAyADKAlSBWxpbmVz');

@$core.Deprecated('Use serverControllerListSessionsRequestDescriptor instead')
const ServerControllerListSessionsRequest$json = {
  '1': 'ServerControllerListSessionsRequest',
};

/// Descriptor for `ServerControllerListSessionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerListSessionsRequestDescriptor =
    $convert
        .base64Decode('CiNTZXJ2ZXJDb250cm9sbGVyTGlzdFNlc3Npb25zUmVxdWVzdA==');

@$core.Deprecated('Use serverControllerListSessionsResponseDescriptor instead')
const ServerControllerListSessionsResponse$json = {
  '1': 'ServerControllerListSessionsResponse',
  '2': [
    {
      '1': 'sessions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerConnectionInfo',
      '10': 'sessions'
    },
  ],
};

/// Descriptor for `ServerControllerListSessionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerListSessionsResponseDescriptor =
    $convert.base64Decode(
        'CiRTZXJ2ZXJDb250cm9sbGVyTGlzdFNlc3Npb25zUmVzcG9uc2USQgoIc2Vzc2lvbnMYASADKA'
        'syJi5vcGVub3hpZGUuYXBpLnYxLlNlcnZlckNvbm5lY3Rpb25JbmZvUghzZXNzaW9ucw==');

@$core.Deprecated('Use serverControllerDeleteSessionRequestDescriptor instead')
const ServerControllerDeleteSessionRequest$json = {
  '1': 'ServerControllerDeleteSessionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerControllerDeleteSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverControllerDeleteSessionRequestDescriptor =
    $convert.base64Decode(
        'CiRTZXJ2ZXJDb250cm9sbGVyRGVsZXRlU2Vzc2lvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBC'
        'ICIABSAmlk');

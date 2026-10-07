// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server.proto.

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

@$core.Deprecated('Use serverStatusDescriptor instead')
const ServerStatus$json = {
  '1': 'ServerStatus',
  '2': [
    {'1': 'SERVER_STATUS_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_STATUS_ACTIVE', '2': 1},
    {'1': 'SERVER_STATUS_INACTIVE', '2': 2},
  ],
};

/// Descriptor for `ServerStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverStatusDescriptor = $convert.base64Decode(
    'CgxTZXJ2ZXJTdGF0dXMSHQoZU0VSVkVSX1NUQVRVU19VTlNQRUNJRklFRBAAEhgKFFNFUlZFUl'
    '9TVEFUVVNfQUNUSVZFEAESGgoWU0VSVkVSX1NUQVRVU19JTkFDVElWRRAC');

@$core.Deprecated('Use serverTypeDescriptor instead')
const ServerType$json = {
  '1': 'ServerType',
  '2': [
    {'1': 'SERVER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'SERVER_TYPE_DEPLOY', '2': 1},
    {'1': 'SERVER_TYPE_BUILD', '2': 2},
  ],
};

/// Descriptor for `ServerType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List serverTypeDescriptor = $convert.base64Decode(
    'CgpTZXJ2ZXJUeXBlEhsKF1NFUlZFUl9UWVBFX1VOU1BFQ0lGSUVEEAASFgoSU0VSVkVSX1RZUE'
    'VfREVQTE9ZEAESFQoRU0VSVkVSX1RZUEVfQlVJTEQQAg==');

@$core.Deprecated('Use serverDescriptor instead')
const Server$json = {
  '1': 'Server',
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
    {'1': 'ip_address', '3': 4, '4': 1, '5': 9, '10': 'ipAddress'},
    {'1': 'port', '3': 5, '4': 1, '5': 3, '10': 'port'},
    {'1': 'username', '3': 6, '4': 1, '5': 9, '10': 'username'},
    {'1': 'app_name', '3': 7, '4': 1, '5': 9, '10': 'appName'},
    {
      '1': 'server_status',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerStatus',
      '10': 'serverStatus'
    },
    {
      '1': 'server_type',
      '3': 9,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerType',
      '10': 'serverType'
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 10,
      '4': 1,
      '5': 8,
      '10': 'enableDockerCleanup'
    },
    {
      '1': 'log_cleanup_cron',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'logCleanupCron',
      '17': true
    },
    {'1': 'command', '3': 12, '4': 1, '5': 9, '10': 'command'},
    {'1': 'metrics_config', '3': 13, '4': 1, '5': 9, '10': 'metricsConfig'},
    {
      '1': 'ssh_key_id',
      '3': 14,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'sshKeyId',
      '17': true
    },
    {'1': 'created_at', '3': 15, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 16, '4': 1, '5': 3, '10': 'updatedAt'},
    {
      '1': 'build_memory_limit',
      '3': 17,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_log_cleanup_cron'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `Server`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverDescriptor = $convert.base64Decode(
    'CgZTZXJ2ZXISDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSJQoLZGVzY3JpcH'
    'Rpb24YAyABKAlIAFILZGVzY3JpcHRpb26IAQESHQoKaXBfYWRkcmVzcxgEIAEoCVIJaXBBZGRy'
    'ZXNzEhIKBHBvcnQYBSABKANSBHBvcnQSGgoIdXNlcm5hbWUYBiABKAlSCHVzZXJuYW1lEhkKCG'
    'FwcF9uYW1lGAcgASgJUgdhcHBOYW1lEkMKDXNlcnZlcl9zdGF0dXMYCCABKA4yHi5vcGVub3hp'
    'ZGUuYXBpLnYxLlNlcnZlclN0YXR1c1IMc2VydmVyU3RhdHVzEj0KC3NlcnZlcl90eXBlGAkgAS'
    'gOMhwub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJUeXBlUgpzZXJ2ZXJUeXBlEjIKFWVuYWJsZV9k'
    'b2NrZXJfY2xlYW51cBgKIAEoCFITZW5hYmxlRG9ja2VyQ2xlYW51cBItChBsb2dfY2xlYW51cF'
    '9jcm9uGAsgASgJSAFSDmxvZ0NsZWFudXBDcm9uiAEBEhgKB2NvbW1hbmQYDCABKAlSB2NvbW1h'
    'bmQSJQoObWV0cmljc19jb25maWcYDSABKAlSDW1ldHJpY3NDb25maWcSIQoKc3NoX2tleV9pZB'
    'gOIAEoA0gCUghzc2hLZXlJZIgBARIdCgpjcmVhdGVkX2F0GA8gASgDUgljcmVhdGVkQXQSHQoK'
    'dXBkYXRlZF9hdBgQIAEoA1IJdXBkYXRlZEF0EjEKEmJ1aWxkX21lbW9yeV9saW1pdBgRIAEoCU'
    'gDUhBidWlsZE1lbW9yeUxpbWl0iAEBEisKD2J1aWxkX2NwdV9saW1pdBgSIAEoCUgEUg1idWls'
    'ZENwdUxpbWl0iAEBQg4KDF9kZXNjcmlwdGlvbkITChFfbG9nX2NsZWFudXBfY3JvbkINCgtfc3'
    'NoX2tleV9pZEIVChNfYnVpbGRfbWVtb3J5X2xpbWl0QhIKEF9idWlsZF9jcHVfbGltaXQ=');

@$core.Deprecated('Use serverListServersRequestDescriptor instead')
const ServerListServersRequest$json = {
  '1': 'ServerListServersRequest',
};

/// Descriptor for `ServerListServersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListServersRequestDescriptor =
    $convert.base64Decode('ChhTZXJ2ZXJMaXN0U2VydmVyc1JlcXVlc3Q=');

@$core.Deprecated('Use serverListServersResponseDescriptor instead')
const ServerListServersResponse$json = {
  '1': 'ServerListServersResponse',
  '2': [
    {
      '1': 'servers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Server',
      '10': 'servers'
    },
  ],
};

/// Descriptor for `ServerListServersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListServersResponseDescriptor =
    $convert.base64Decode(
        'ChlTZXJ2ZXJMaXN0U2VydmVyc1Jlc3BvbnNlEjIKB3NlcnZlcnMYASADKAsyGC5vcGVub3hpZG'
        'UuYXBpLnYxLlNlcnZlclIHc2VydmVycw==');

@$core.Deprecated('Use serverGetServerRequestDescriptor instead')
const ServerGetServerRequest$json = {
  '1': 'ServerGetServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerGetServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverGetServerRequestDescriptor =
    $convert.base64Decode(
        'ChZTZXJ2ZXJHZXRTZXJ2ZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use serverGetServerResponseDescriptor instead')
const ServerGetServerResponse$json = {
  '1': 'ServerGetServerResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Server',
      '10': 'server'
    },
  ],
};

/// Descriptor for `ServerGetServerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverGetServerResponseDescriptor =
    $convert.base64Decode(
        'ChdTZXJ2ZXJHZXRTZXJ2ZXJSZXNwb25zZRIwCgZzZXJ2ZXIYASABKAsyGC5vcGVub3hpZGUuYX'
        'BpLnYxLlNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use serverCreateServerRequestDescriptor instead')
const ServerCreateServerRequest$json = {
  '1': 'ServerCreateServerRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'description',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'description',
      '17': true
    },
    {'1': 'ip_address', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'ipAddress'},
    {'1': 'port', '3': 4, '4': 1, '5': 3, '10': 'port'},
    {'1': 'username', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {
      '1': 'server_type',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerType',
      '8': {},
      '10': 'serverType'
    },
    {
      '1': 'ssh_key_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'sshKeyId',
      '17': true
    },
    {
      '1': 'build_memory_limit',
      '3': 8,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_description'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `ServerCreateServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCreateServerRequestDescriptor = $convert.base64Decode(
    'ChlTZXJ2ZXJDcmVhdGVTZXJ2ZXJSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG'
    '5hbWUSLwoLZGVzY3JpcHRpb24YAiABKAlCCLpIBXIDGOgHSABSC2Rlc2NyaXB0aW9uiAEBEikK'
    'CmlwX2FkZHJlc3MYAyABKAlCCrpIB3IFEAEY/wFSCWlwQWRkcmVzcxISCgRwb3J0GAQgASgDUg'
    'Rwb3J0EiQKCHVzZXJuYW1lGAUgASgJQgi6SAVyAxj/AVIIdXNlcm5hbWUSSQoLc2VydmVyX3R5'
    'cGUYBiABKA4yHC5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclR5cGVCCrpIB4IBBBABIABSCnNlcn'
    'ZlclR5cGUSKgoKc3NoX2tleV9pZBgHIAEoA0IHukgEIgIgAEgBUghzc2hLZXlJZIgBARI7ChJi'
    'dWlsZF9tZW1vcnlfbGltaXQYCCABKAlCCLpIBXIDGP8BSAJSEGJ1aWxkTWVtb3J5TGltaXSIAQ'
    'ESNQoPYnVpbGRfY3B1X2xpbWl0GAkgASgJQgi6SAVyAxj/AUgDUg1idWlsZENwdUxpbWl0iAEB'
    'Qg4KDF9kZXNjcmlwdGlvbkINCgtfc3NoX2tleV9pZEIVChNfYnVpbGRfbWVtb3J5X2xpbWl0Qh'
    'IKEF9idWlsZF9jcHVfbGltaXQ=');

@$core.Deprecated('Use serverCreateServerResponseDescriptor instead')
const ServerCreateServerResponse$json = {
  '1': 'ServerCreateServerResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Server',
      '10': 'server'
    },
  ],
};

/// Descriptor for `ServerCreateServerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCreateServerResponseDescriptor =
    $convert.base64Decode(
        'ChpTZXJ2ZXJDcmVhdGVTZXJ2ZXJSZXNwb25zZRIwCgZzZXJ2ZXIYASABKAsyGC5vcGVub3hpZG'
        'UuYXBpLnYxLlNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use serverUpdateServerRequestDescriptor instead')
const ServerUpdateServerRequest$json = {
  '1': 'ServerUpdateServerRequest',
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
      '1': 'description',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'description',
      '17': true
    },
    {
      '1': 'ip_address',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'ipAddress',
      '17': true
    },
    {'1': 'port', '3': 5, '4': 1, '5': 3, '9': 3, '10': 'port', '17': true},
    {
      '1': 'username',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'username',
      '17': true
    },
    {
      '1': 'server_status',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerStatus',
      '8': {},
      '9': 5,
      '10': 'serverStatus',
      '17': true
    },
    {
      '1': 'server_type',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerType',
      '8': {},
      '9': 6,
      '10': 'serverType',
      '17': true
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 9,
      '4': 1,
      '5': 8,
      '9': 7,
      '10': 'enableDockerCleanup',
      '17': true
    },
    {
      '1': 'log_cleanup_cron',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'logCleanupCron',
      '17': true
    },
    {
      '1': 'command',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'command',
      '17': true
    },
    {
      '1': 'metrics_config',
      '3': 12,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 10,
      '10': 'metricsConfig',
      '17': true
    },
    {
      '1': 'ssh_key_id',
      '3': 13,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 11,
      '10': 'sshKeyId',
      '17': true
    },
    {
      '1': 'build_memory_limit',
      '3': 14,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 12,
      '10': 'buildMemoryLimit',
      '17': true
    },
    {
      '1': 'build_cpu_limit',
      '3': 15,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 13,
      '10': 'buildCpuLimit',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_ip_address'},
    {'1': '_port'},
    {'1': '_username'},
    {'1': '_server_status'},
    {'1': '_server_type'},
    {'1': '_enable_docker_cleanup'},
    {'1': '_log_cleanup_cron'},
    {'1': '_command'},
    {'1': '_metrics_config'},
    {'1': '_ssh_key_id'},
    {'1': '_build_memory_limit'},
    {'1': '_build_cpu_limit'},
  ],
};

/// Descriptor for `ServerUpdateServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverUpdateServerRequestDescriptor = $convert.base64Decode(
    'ChlTZXJ2ZXJVcGRhdGVTZXJ2ZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIjCg'
    'RuYW1lGAIgASgJQgq6SAdyBRABGP8BSABSBG5hbWWIAQESLwoLZGVzY3JpcHRpb24YAyABKAlC'
    'CLpIBXIDGOgHSAFSC2Rlc2NyaXB0aW9uiAEBEiwKCmlwX2FkZHJlc3MYBCABKAlCCLpIBXIDGP'
    '8BSAJSCWlwQWRkcmVzc4gBARIXCgRwb3J0GAUgASgDSANSBHBvcnSIAQESKQoIdXNlcm5hbWUY'
    'BiABKAlCCLpIBXIDGP8BSARSCHVzZXJuYW1liAEBElQKDXNlcnZlcl9zdGF0dXMYByABKA4yHi'
    '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclN0YXR1c0IKukgHggEEEAEgAEgFUgxzZXJ2ZXJTdGF0'
    'dXOIAQESTgoLc2VydmVyX3R5cGUYCCABKA4yHC5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclR5cG'
    'VCCrpIB4IBBBABIABIBlIKc2VydmVyVHlwZYgBARI3ChVlbmFibGVfZG9ja2VyX2NsZWFudXAY'
    'CSABKAhIB1ITZW5hYmxlRG9ja2VyQ2xlYW51cIgBARItChBsb2dfY2xlYW51cF9jcm9uGAogAS'
    'gJSAhSDmxvZ0NsZWFudXBDcm9uiAEBEh0KB2NvbW1hbmQYCyABKAlICVIHY29tbWFuZIgBARI1'
    'Cg5tZXRyaWNzX2NvbmZpZxgMIAEoCUIJukgGcgQYgIBASApSDW1ldHJpY3NDb25maWeIAQESKg'
    'oKc3NoX2tleV9pZBgNIAEoA0IHukgEIgIgAEgLUghzc2hLZXlJZIgBARI7ChJidWlsZF9tZW1v'
    'cnlfbGltaXQYDiABKAlCCLpIBXIDGP8BSAxSEGJ1aWxkTWVtb3J5TGltaXSIAQESNQoPYnVpbG'
    'RfY3B1X2xpbWl0GA8gASgJQgi6SAVyAxj/AUgNUg1idWlsZENwdUxpbWl0iAEBQgcKBV9uYW1l'
    'Qg4KDF9kZXNjcmlwdGlvbkINCgtfaXBfYWRkcmVzc0IHCgVfcG9ydEILCglfdXNlcm5hbWVCEA'
    'oOX3NlcnZlcl9zdGF0dXNCDgoMX3NlcnZlcl90eXBlQhgKFl9lbmFibGVfZG9ja2VyX2NsZWFu'
    'dXBCEwoRX2xvZ19jbGVhbnVwX2Nyb25CCgoIX2NvbW1hbmRCEQoPX21ldHJpY3NfY29uZmlnQg'
    '0KC19zc2hfa2V5X2lkQhUKE19idWlsZF9tZW1vcnlfbGltaXRCEgoQX2J1aWxkX2NwdV9saW1p'
    'dA==');

@$core.Deprecated('Use serverUpdateServerResponseDescriptor instead')
const ServerUpdateServerResponse$json = {
  '1': 'ServerUpdateServerResponse',
  '2': [
    {
      '1': 'server',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Server',
      '10': 'server'
    },
  ],
};

/// Descriptor for `ServerUpdateServerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverUpdateServerResponseDescriptor =
    $convert.base64Decode(
        'ChpTZXJ2ZXJVcGRhdGVTZXJ2ZXJSZXNwb25zZRIwCgZzZXJ2ZXIYASABKAsyGC5vcGVub3hpZG'
        'UuYXBpLnYxLlNlcnZlclIGc2VydmVy');

@$core.Deprecated('Use serverDeleteServerRequestDescriptor instead')
const ServerDeleteServerRequest$json = {
  '1': 'ServerDeleteServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerDeleteServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverDeleteServerRequestDescriptor =
    $convert.base64Decode(
        'ChlTZXJ2ZXJEZWxldGVTZXJ2ZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use serverTestConnectionRequestDescriptor instead')
const ServerTestConnectionRequest$json = {
  '1': 'ServerTestConnectionRequest',
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

/// Descriptor for `ServerTestConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverTestConnectionRequestDescriptor = $convert.base64Decode(
    'ChtTZXJ2ZXJUZXN0Q29ubmVjdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEj'
    '8KFGhvc3Rfa2V5X2ZpbmdlcnByaW50GAIgASgJQgi6SAVyAxj/AUgAUhJob3N0S2V5RmluZ2Vy'
    'cHJpbnSIAQESKAoNc3Vkb19wYXNzd29yZBgDIAEoCUgBUgxzdWRvUGFzc3dvcmSIAQESIAoJcG'
    '9vbF9zaXplGAQgASgNSAJSCHBvb2xTaXpliAEBQhcKFV9ob3N0X2tleV9maW5nZXJwcmludEIQ'
    'Cg5fc3Vkb19wYXNzd29yZEIMCgpfcG9vbF9zaXpl');

@$core.Deprecated('Use serverTestConnectionResponseDescriptor instead')
const ServerTestConnectionResponse$json = {
  '1': 'ServerTestConnectionResponse',
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

/// Descriptor for `ServerTestConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverTestConnectionResponseDescriptor =
    $convert.base64Decode(
        'ChxTZXJ2ZXJUZXN0Q29ubmVjdGlvblJlc3BvbnNlEkYKCmNvbm5lY3Rpb24YASABKAsyJi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNlcnZlckNvbm5lY3Rpb25JbmZvUgpjb25uZWN0aW9u');

@$core.Deprecated('Use serverSetupServerRequestDescriptor instead')
const ServerSetupServerRequest$json = {
  '1': 'ServerSetupServerRequest',
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

/// Descriptor for `ServerSetupServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverSetupServerRequestDescriptor = $convert.base64Decode(
    'ChhTZXJ2ZXJTZXR1cFNlcnZlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjEKFG'
    'luc3RhbGxfZGVwZW5kZW5jaWVzGAIgASgIUhNpbnN0YWxsRGVwZW5kZW5jaWVzEjQKDmFkdmVy'
    'dGlzZV9hZGRyGAMgASgJQgi6SAVyAxj/AUgAUg1hZHZlcnRpc2VBZGRyiAEBEiwKCmFjbWVfZW'
    '1haWwYBCABKAlCCLpIBXIDGP8BSAFSCWFjbWVFbWFpbIgBARI/ChRob3N0X2tleV9maW5nZXJw'
    'cmludBgFIAEoCUIIukgFcgMY/wFIAlISaG9zdEtleUZpbmdlcnByaW50iAEBEigKDXN1ZG9fcG'
    'Fzc3dvcmQYBiABKAlIA1IMc3Vkb1Bhc3N3b3JkiAEBEiAKCXBvb2xfc2l6ZRgHIAEoDUgEUghw'
    'b29sU2l6ZYgBAUIRCg9fYWR2ZXJ0aXNlX2FkZHJCDQoLX2FjbWVfZW1haWxCFwoVX2hvc3Rfa2'
    'V5X2ZpbmdlcnByaW50QhAKDl9zdWRvX3Bhc3N3b3JkQgwKCl9wb29sX3NpemU=');

@$core.Deprecated('Use serverSetupServerResponseDescriptor instead')
const ServerSetupServerResponse$json = {
  '1': 'ServerSetupServerResponse',
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

/// Descriptor for `ServerSetupServerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverSetupServerResponseDescriptor =
    $convert.base64Decode(
        'ChlTZXJ2ZXJTZXR1cFNlcnZlclJlc3BvbnNlEj4KB291dGNvbWUYASABKAsyJC5vcGVub3hpZG'
        'UuYXBpLnYxLlNlcnZlclNldHVwT3V0Y29tZVIHb3V0Y29tZQ==');

@$core.Deprecated('Use serverStartServerRequestDescriptor instead')
const ServerStartServerRequest$json = {
  '1': 'ServerStartServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerStartServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverStartServerRequestDescriptor =
    $convert.base64Decode(
        'ChhTZXJ2ZXJTdGFydFNlcnZlclJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use serverStopServerRequestDescriptor instead')
const ServerStopServerRequest$json = {
  '1': 'ServerStopServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerStopServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverStopServerRequestDescriptor =
    $convert.base64Decode(
        'ChdTZXJ2ZXJTdG9wU2VydmVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use serverRestartServerRequestDescriptor instead')
const ServerRestartServerRequest$json = {
  '1': 'ServerRestartServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerRestartServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverRestartServerRequestDescriptor =
    $convert.base64Decode(
        'ChpTZXJ2ZXJSZXN0YXJ0U2VydmVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use serverRemoveServerRequestDescriptor instead')
const ServerRemoveServerRequest$json = {
  '1': 'ServerRemoveServerRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerRemoveServerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverRemoveServerRequestDescriptor =
    $convert.base64Decode(
        'ChlTZXJ2ZXJSZW1vdmVTZXJ2ZXJSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use serverGetServerErrorsRequestDescriptor instead')
const ServerGetServerErrorsRequest$json = {
  '1': 'ServerGetServerErrorsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerGetServerErrorsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverGetServerErrorsRequestDescriptor =
    $convert.base64Decode(
        'ChxTZXJ2ZXJHZXRTZXJ2ZXJFcnJvcnNSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use serverGetServerErrorsResponseDescriptor instead')
const ServerGetServerErrorsResponse$json = {
  '1': 'ServerGetServerErrorsResponse',
  '2': [
    {'1': 'errors', '3': 1, '4': 3, '5': 9, '10': 'errors'},
  ],
};

/// Descriptor for `ServerGetServerErrorsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverGetServerErrorsResponseDescriptor =
    $convert.base64Decode(
        'Ch1TZXJ2ZXJHZXRTZXJ2ZXJFcnJvcnNSZXNwb25zZRIWCgZlcnJvcnMYASADKAlSBmVycm9ycw'
        '==');

@$core.Deprecated('Use serverListRemoteServersRequestDescriptor instead')
const ServerListRemoteServersRequest$json = {
  '1': 'ServerListRemoteServersRequest',
};

/// Descriptor for `ServerListRemoteServersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListRemoteServersRequestDescriptor =
    $convert.base64Decode('Ch5TZXJ2ZXJMaXN0UmVtb3RlU2VydmVyc1JlcXVlc3Q=');

@$core.Deprecated('Use serverListRemoteServersResponseDescriptor instead')
const ServerListRemoteServersResponse$json = {
  '1': 'ServerListRemoteServersResponse',
  '2': [
    {
      '1': 'servers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Server',
      '10': 'servers'
    },
  ],
};

/// Descriptor for `ServerListRemoteServersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListRemoteServersResponseDescriptor =
    $convert.base64Decode(
        'Ch9TZXJ2ZXJMaXN0UmVtb3RlU2VydmVyc1Jlc3BvbnNlEjIKB3NlcnZlcnMYASADKAsyGC5vcG'
        'Vub3hpZGUuYXBpLnYxLlNlcnZlclIHc2VydmVycw==');

@$core.Deprecated('Use serverListPrivateNetworksRequestDescriptor instead')
const ServerListPrivateNetworksRequest$json = {
  '1': 'ServerListPrivateNetworksRequest',
};

/// Descriptor for `ServerListPrivateNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListPrivateNetworksRequestDescriptor =
    $convert.base64Decode('CiBTZXJ2ZXJMaXN0UHJpdmF0ZU5ldHdvcmtzUmVxdWVzdA==');

@$core.Deprecated('Use serverListPrivateNetworksResponseDescriptor instead')
const ServerListPrivateNetworksResponse$json = {
  '1': 'ServerListPrivateNetworksResponse',
  '2': [
    {
      '1': 'networks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'networks'
    },
  ],
};

/// Descriptor for `ServerListPrivateNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverListPrivateNetworksResponseDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJMaXN0UHJpdmF0ZU5ldHdvcmtzUmVzcG9uc2USQgoIbmV0d29ya3MYASADKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUghuZXR3b3Jrcw==');

@$core.Deprecated('Use serverCreatePrivateNetworkRequestDescriptor instead')
const ServerCreatePrivateNetworkRequest$json = {
  '1': 'ServerCreatePrivateNetworkRequest',
  '2': [
    {'1': 'server_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'serverId'},
    {
      '1': 'connection_mode',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerConnectionMode',
      '8': {},
      '10': 'connectionMode'
    },
    {
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkProvider',
      '8': {},
      '9': 0,
      '10': 'provider',
      '17': true
    },
    {
      '1': 'private_host',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'privateHost',
      '17': true
    },
    {
      '1': 'tunnel_address',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'tunnelAddress',
      '17': true
    },
    {
      '1': 'public_key',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'publicKey',
      '17': true
    },
    {
      '1': 'endpoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'endpoint',
      '17': true
    },
    {
      '1': 'listen_port',
      '3': 8,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 5,
      '10': 'listenPort',
      '17': true
    },
    {
      '1': 'persistent_keepalive',
      '3': 9,
      '4': 1,
      '5': 13,
      '9': 6,
      '10': 'persistentKeepalive',
      '17': true
    },
    {
      '1': 'dns_name',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'dnsName',
      '17': true
    },
    {'1': 'routes', '3': 11, '4': 3, '5': 9, '8': {}, '10': 'routes'},
  ],
  '8': [
    {'1': '_provider'},
    {'1': '_private_host'},
    {'1': '_tunnel_address'},
    {'1': '_public_key'},
    {'1': '_endpoint'},
    {'1': '_listen_port'},
    {'1': '_persistent_keepalive'},
    {'1': '_dns_name'},
  ],
};

/// Descriptor for `ServerCreatePrivateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCreatePrivateNetworkRequestDescriptor = $convert.base64Decode(
    'CiFTZXJ2ZXJDcmVhdGVQcml2YXRlTmV0d29ya1JlcXVlc3QSJAoJc2VydmVyX2lkGAEgASgDQg'
    'e6SAQiAiAAUghzZXJ2ZXJJZBJbCg9jb25uZWN0aW9uX21vZGUYAiABKA4yJi5vcGVub3hpZGUu'
    'YXBpLnYxLlNlcnZlckNvbm5lY3Rpb25Nb2RlQgq6SAeCAQQQASAAUg5jb25uZWN0aW9uTW9kZR'
    'JZCghwcm92aWRlchgDIAEoDjIuLm9wZW5veGlkZS5hcGkudjEuU2VydmVyUHJpdmF0ZU5ldHdv'
    'cmtQcm92aWRlckIIukgFggECEAFIAFIIcHJvdmlkZXKIAQESMAoMcHJpdmF0ZV9ob3N0GAQgAS'
    'gJQgi6SAVyAxj/AUgBUgtwcml2YXRlSG9zdIgBARI0Cg50dW5uZWxfYWRkcmVzcxgFIAEoCUII'
    'ukgFcgMY/wFIAlINdHVubmVsQWRkcmVzc4gBARIsCgpwdWJsaWNfa2V5GAYgASgJQgi6SAVyAx'
    'j/AUgDUglwdWJsaWNLZXmIAQESKQoIZW5kcG9pbnQYByABKAlCCLpIBXIDGP8BSARSCGVuZHBv'
    'aW50iAEBEi8KC2xpc3Rlbl9wb3J0GAggASgNQgm6SAYqBBj//wNIBVIKbGlzdGVuUG9ydIgBAR'
    'I2ChRwZXJzaXN0ZW50X2tlZXBhbGl2ZRgJIAEoDUgGUhNwZXJzaXN0ZW50S2VlcGFsaXZliAEB'
    'EigKCGRuc19uYW1lGAogASgJQgi6SAVyAxj/AUgHUgdkbnNOYW1liAEBEiAKBnJvdXRlcxgLIA'
    'MoCUIIukgFkgECEEBSBnJvdXRlc0ILCglfcHJvdmlkZXJCDwoNX3ByaXZhdGVfaG9zdEIRCg9f'
    'dHVubmVsX2FkZHJlc3NCDQoLX3B1YmxpY19rZXlCCwoJX2VuZHBvaW50Qg4KDF9saXN0ZW5fcG'
    '9ydEIXChVfcGVyc2lzdGVudF9rZWVwYWxpdmVCCwoJX2Ruc19uYW1l');

@$core.Deprecated('Use serverCreatePrivateNetworkResponseDescriptor instead')
const ServerCreatePrivateNetworkResponse$json = {
  '1': 'ServerCreatePrivateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerCreatePrivateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverCreatePrivateNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJDcmVhdGVQcml2YXRlTmV0d29ya1Jlc3BvbnNlEkAKB25ldHdvcmsYASABKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use serverUpdatePrivateNetworkRequestDescriptor instead')
const ServerUpdatePrivateNetworkRequest$json = {
  '1': 'ServerUpdatePrivateNetworkRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'connection_mode',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerConnectionMode',
      '8': {},
      '10': 'connectionMode'
    },
    {
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.ServerPrivateNetworkProvider',
      '8': {},
      '9': 0,
      '10': 'provider',
      '17': true
    },
    {
      '1': 'private_host',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'privateHost',
      '17': true
    },
    {
      '1': 'tunnel_address',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'tunnelAddress',
      '17': true
    },
    {
      '1': 'public_key',
      '3': 6,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'publicKey',
      '17': true
    },
    {
      '1': 'endpoint',
      '3': 7,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'endpoint',
      '17': true
    },
    {
      '1': 'listen_port',
      '3': 8,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 5,
      '10': 'listenPort',
      '17': true
    },
    {
      '1': 'persistent_keepalive',
      '3': 9,
      '4': 1,
      '5': 13,
      '9': 6,
      '10': 'persistentKeepalive',
      '17': true
    },
    {
      '1': 'dns_name',
      '3': 10,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 7,
      '10': 'dnsName',
      '17': true
    },
    {'1': 'routes', '3': 11, '4': 3, '5': 9, '8': {}, '10': 'routes'},
  ],
  '8': [
    {'1': '_provider'},
    {'1': '_private_host'},
    {'1': '_tunnel_address'},
    {'1': '_public_key'},
    {'1': '_endpoint'},
    {'1': '_listen_port'},
    {'1': '_persistent_keepalive'},
    {'1': '_dns_name'},
  ],
};

/// Descriptor for `ServerUpdatePrivateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverUpdatePrivateNetworkRequestDescriptor = $convert.base64Decode(
    'CiFTZXJ2ZXJVcGRhdGVQcml2YXRlTmV0d29ya1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
    'BSAmlkElsKD2Nvbm5lY3Rpb25fbW9kZRgCIAEoDjImLm9wZW5veGlkZS5hcGkudjEuU2VydmVy'
    'Q29ubmVjdGlvbk1vZGVCCrpIB4IBBBABIABSDmNvbm5lY3Rpb25Nb2RlElkKCHByb3ZpZGVyGA'
    'MgASgOMi4ub3Blbm94aWRlLmFwaS52MS5TZXJ2ZXJQcml2YXRlTmV0d29ya1Byb3ZpZGVyQgi6'
    'SAWCAQIQAUgAUghwcm92aWRlcogBARIwCgxwcml2YXRlX2hvc3QYBCABKAlCCLpIBXIDGP8BSA'
    'FSC3ByaXZhdGVIb3N0iAEBEjQKDnR1bm5lbF9hZGRyZXNzGAUgASgJQgi6SAVyAxj/AUgCUg10'
    'dW5uZWxBZGRyZXNziAEBEiwKCnB1YmxpY19rZXkYBiABKAlCCLpIBXIDGP8BSANSCXB1YmxpY0'
    'tleYgBARIpCghlbmRwb2ludBgHIAEoCUIIukgFcgMY/wFIBFIIZW5kcG9pbnSIAQESLwoLbGlz'
    'dGVuX3BvcnQYCCABKA1CCbpIBioEGP//A0gFUgpsaXN0ZW5Qb3J0iAEBEjYKFHBlcnNpc3Rlbn'
    'Rfa2VlcGFsaXZlGAkgASgNSAZSE3BlcnNpc3RlbnRLZWVwYWxpdmWIAQESKAoIZG5zX25hbWUY'
    'CiABKAlCCLpIBXIDGP8BSAdSB2Ruc05hbWWIAQESIAoGcm91dGVzGAsgAygJQgi6SAWSAQIQQF'
    'IGcm91dGVzQgsKCV9wcm92aWRlckIPCg1fcHJpdmF0ZV9ob3N0QhEKD190dW5uZWxfYWRkcmVz'
    'c0INCgtfcHVibGljX2tleUILCglfZW5kcG9pbnRCDgoMX2xpc3Rlbl9wb3J0QhcKFV9wZXJzaX'
    'N0ZW50X2tlZXBhbGl2ZUILCglfZG5zX25hbWU=');

@$core.Deprecated('Use serverUpdatePrivateNetworkResponseDescriptor instead')
const ServerUpdatePrivateNetworkResponse$json = {
  '1': 'ServerUpdatePrivateNetworkResponse',
  '2': [
    {
      '1': 'network',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.ServerPrivateNetwork',
      '10': 'network'
    },
  ],
};

/// Descriptor for `ServerUpdatePrivateNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverUpdatePrivateNetworkResponseDescriptor =
    $convert.base64Decode(
        'CiJTZXJ2ZXJVcGRhdGVQcml2YXRlTmV0d29ya1Jlc3BvbnNlEkAKB25ldHdvcmsYASABKAsyJi'
        '5vcGVub3hpZGUuYXBpLnYxLlNlcnZlclByaXZhdGVOZXR3b3JrUgduZXR3b3Jr');

@$core.Deprecated('Use serverDeletePrivateNetworkRequestDescriptor instead')
const ServerDeletePrivateNetworkRequest$json = {
  '1': 'ServerDeletePrivateNetworkRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ServerDeletePrivateNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverDeletePrivateNetworkRequestDescriptor =
    $convert.base64Decode(
        'CiFTZXJ2ZXJEZWxldGVQcml2YXRlTmV0d29ya1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

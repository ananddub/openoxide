// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ssh_key.proto.

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

@$core.Deprecated('Use sshKeyTypeDescriptor instead')
const SshKeyType$json = {
  '1': 'SshKeyType',
  '2': [
    {'1': 'SSH_KEY_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'SSH_KEY_TYPE_ED25519', '2': 1},
    {'1': 'SSH_KEY_TYPE_RSA', '2': 2},
  ],
};

/// Descriptor for `SshKeyType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List sshKeyTypeDescriptor = $convert.base64Decode(
    'CgpTc2hLZXlUeXBlEhwKGFNTSF9LRVlfVFlQRV9VTlNQRUNJRklFRBAAEhgKFFNTSF9LRVlfVF'
    'lQRV9FRDI1NTE5EAESFAoQU1NIX0tFWV9UWVBFX1JTQRAC');

@$core.Deprecated('Use sshKeyDescriptor instead')
const SshKey$json = {
  '1': 'SshKey',
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
    {'1': 'public_key', '3': 4, '4': 1, '5': 9, '10': 'publicKey'},
    {
      '1': 'private_key',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'privateKey',
      '17': true
    },
    {'1': 'has_private_key', '3': 6, '4': 1, '5': 8, '10': 'hasPrivateKey'},
    {
      '1': 'last_used_at',
      '3': 7,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'lastUsedAt',
      '17': true
    },
    {'1': 'created_at', '3': 8, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 9, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_description'},
    {'1': '_private_key'},
    {'1': '_last_used_at'},
  ],
};

/// Descriptor for `SshKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyDescriptor = $convert.base64Decode(
    'CgZTc2hLZXkSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSJQoLZGVzY3JpcH'
    'Rpb24YAyABKAlIAFILZGVzY3JpcHRpb26IAQESHQoKcHVibGljX2tleRgEIAEoCVIJcHVibGlj'
    'S2V5EiQKC3ByaXZhdGVfa2V5GAUgASgJSAFSCnByaXZhdGVLZXmIAQESJgoPaGFzX3ByaXZhdG'
    'Vfa2V5GAYgASgIUg1oYXNQcml2YXRlS2V5EiUKDGxhc3RfdXNlZF9hdBgHIAEoA0gCUgpsYXN0'
    'VXNlZEF0iAEBEh0KCmNyZWF0ZWRfYXQYCCABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GA'
    'kgASgDUgl1cGRhdGVkQXRCDgoMX2Rlc2NyaXB0aW9uQg4KDF9wcml2YXRlX2tleUIPCg1fbGFz'
    'dF91c2VkX2F0');

@$core.Deprecated('Use sshKeyListKeysRequestDescriptor instead')
const SshKeyListKeysRequest$json = {
  '1': 'SshKeyListKeysRequest',
};

/// Descriptor for `SshKeyListKeysRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyListKeysRequestDescriptor =
    $convert.base64Decode('ChVTc2hLZXlMaXN0S2V5c1JlcXVlc3Q=');

@$core.Deprecated('Use sshKeyListKeysResponseDescriptor instead')
const SshKeyListKeysResponse$json = {
  '1': 'SshKeyListKeysResponse',
  '2': [
    {
      '1': 'keys',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.SshKey',
      '10': 'keys'
    },
  ],
};

/// Descriptor for `SshKeyListKeysResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyListKeysResponseDescriptor =
    $convert.base64Decode(
        'ChZTc2hLZXlMaXN0S2V5c1Jlc3BvbnNlEiwKBGtleXMYASADKAsyGC5vcGVub3hpZGUuYXBpLn'
        'YxLlNzaEtleVIEa2V5cw==');

@$core.Deprecated('Use sshKeyGetKeyRequestDescriptor instead')
const SshKeyGetKeyRequest$json = {
  '1': 'SshKeyGetKeyRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SshKeyGetKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGetKeyRequestDescriptor =
    $convert.base64Decode(
        'ChNTc2hLZXlHZXRLZXlSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use sshKeyGetKeyResponseDescriptor instead')
const SshKeyGetKeyResponse$json = {
  '1': 'SshKeyGetKeyResponse',
  '2': [
    {
      '1': 'key',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SshKey',
      '10': 'key'
    },
  ],
};

/// Descriptor for `SshKeyGetKeyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGetKeyResponseDescriptor = $convert.base64Decode(
    'ChRTc2hLZXlHZXRLZXlSZXNwb25zZRIqCgNrZXkYASABKAsyGC5vcGVub3hpZGUuYXBpLnYxLl'
    'NzaEtleVIDa2V5');

@$core.Deprecated('Use sshKeyCreateKeyRequestDescriptor instead')
const SshKeyCreateKeyRequest$json = {
  '1': 'SshKeyCreateKeyRequest',
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
    {'1': 'public_key', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'publicKey'},
    {'1': 'private_key', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'privateKey'},
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `SshKeyCreateKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyCreateKeyRequestDescriptor = $convert.base64Decode(
    'ChZTc2hLZXlDcmVhdGVLZXlSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG5hbW'
    'USLwoLZGVzY3JpcHRpb24YAiABKAlCCLpIBXIDGIAISABSC2Rlc2NyaXB0aW9uiAEBEioKCnB1'
    'YmxpY19rZXkYAyABKAlCC7pICHIGEAEYgIBAUglwdWJsaWNLZXkSKgoLcHJpdmF0ZV9rZXkYBC'
    'ABKAlCCbpIBnIEGICAQFIKcHJpdmF0ZUtleUIOCgxfZGVzY3JpcHRpb24=');

@$core.Deprecated('Use sshKeyCreateKeyResponseDescriptor instead')
const SshKeyCreateKeyResponse$json = {
  '1': 'SshKeyCreateKeyResponse',
  '2': [
    {
      '1': 'key',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SshKey',
      '10': 'key'
    },
  ],
};

/// Descriptor for `SshKeyCreateKeyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyCreateKeyResponseDescriptor =
    $convert.base64Decode(
        'ChdTc2hLZXlDcmVhdGVLZXlSZXNwb25zZRIqCgNrZXkYASABKAsyGC5vcGVub3hpZGUuYXBpLn'
        'YxLlNzaEtleVIDa2V5');

@$core.Deprecated('Use sshKeyGenerateKeyRequestDescriptor instead')
const SshKeyGenerateKeyRequest$json = {
  '1': 'SshKeyGenerateKeyRequest',
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
    {
      '1': 'key_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SshKeyType',
      '8': {},
      '10': 'keyType'
    },
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `SshKeyGenerateKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGenerateKeyRequestDescriptor = $convert.base64Decode(
    'ChhTc2hLZXlHZW5lcmF0ZUtleVJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AVIEbm'
    'FtZRIvCgtkZXNjcmlwdGlvbhgCIAEoCUIIukgFcgMYgAhIAFILZGVzY3JpcHRpb26IAQESQwoI'
    'a2V5X3R5cGUYAyABKA4yHC5vcGVub3hpZGUuYXBpLnYxLlNzaEtleVR5cGVCCrpIB4IBBBABIA'
    'BSB2tleVR5cGVCDgoMX2Rlc2NyaXB0aW9u');

@$core.Deprecated('Use sshKeyGenerateKeyResponseDescriptor instead')
const SshKeyGenerateKeyResponse$json = {
  '1': 'SshKeyGenerateKeyResponse',
  '2': [
    {
      '1': 'key',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SshKey',
      '10': 'key'
    },
  ],
};

/// Descriptor for `SshKeyGenerateKeyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGenerateKeyResponseDescriptor =
    $convert.base64Decode(
        'ChlTc2hLZXlHZW5lcmF0ZUtleVJlc3BvbnNlEioKA2tleRgBIAEoCzIYLm9wZW5veGlkZS5hcG'
        'kudjEuU3NoS2V5UgNrZXk=');

@$core.Deprecated('Use sshKeyGeneratePairRequestDescriptor instead')
const SshKeyGeneratePairRequest$json = {
  '1': 'SshKeyGeneratePairRequest',
  '2': [
    {
      '1': 'key_type',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SshKeyType',
      '8': {},
      '9': 0,
      '10': 'keyType',
      '17': true
    },
  ],
  '8': [
    {'1': '_key_type'},
  ],
};

/// Descriptor for `SshKeyGeneratePairRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGeneratePairRequestDescriptor = $convert.base64Decode(
    'ChlTc2hLZXlHZW5lcmF0ZVBhaXJSZXF1ZXN0EkgKCGtleV90eXBlGAEgASgOMhwub3Blbm94aW'
    'RlLmFwaS52MS5Tc2hLZXlUeXBlQgq6SAeCAQQQASAASABSB2tleVR5cGWIAQFCCwoJX2tleV90'
    'eXBl');

@$core.Deprecated('Use sshKeyGeneratePairResponseDescriptor instead')
const SshKeyGeneratePairResponse$json = {
  '1': 'SshKeyGeneratePairResponse',
  '2': [
    {'1': 'private_key', '3': 1, '4': 1, '5': 9, '10': 'privateKey'},
    {'1': 'public_key', '3': 2, '4': 1, '5': 9, '10': 'publicKey'},
  ],
};

/// Descriptor for `SshKeyGeneratePairResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyGeneratePairResponseDescriptor =
    $convert.base64Decode(
        'ChpTc2hLZXlHZW5lcmF0ZVBhaXJSZXNwb25zZRIfCgtwcml2YXRlX2tleRgBIAEoCVIKcHJpdm'
        'F0ZUtleRIdCgpwdWJsaWNfa2V5GAIgASgJUglwdWJsaWNLZXk=');

@$core.Deprecated('Use sshKeyUpdateKeyRequestDescriptor instead')
const SshKeyUpdateKeyRequest$json = {
  '1': 'SshKeyUpdateKeyRequest',
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
      '1': 'public_key',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'publicKey',
      '17': true
    },
    {
      '1': 'private_key',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'privateKey',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_public_key'},
    {'1': '_private_key'},
  ],
};

/// Descriptor for `SshKeyUpdateKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyUpdateKeyRequestDescriptor = $convert.base64Decode(
    'ChZTc2hLZXlVcGRhdGVLZXlSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIhCgRuYW'
    '1lGAIgASgJQgi6SAVyAxj/AUgAUgRuYW1liAEBEi8KC2Rlc2NyaXB0aW9uGAMgASgJQgi6SAVy'
    'AxiACEgBUgtkZXNjcmlwdGlvbogBARItCgpwdWJsaWNfa2V5GAQgASgJQgm6SAZyBBiAgEBIAl'
    'IJcHVibGljS2V5iAEBEi8KC3ByaXZhdGVfa2V5GAUgASgJQgm6SAZyBBiAgEBIA1IKcHJpdmF0'
    'ZUtleYgBAUIHCgVfbmFtZUIOCgxfZGVzY3JpcHRpb25CDQoLX3B1YmxpY19rZXlCDgoMX3ByaX'
    'ZhdGVfa2V5');

@$core.Deprecated('Use sshKeyUpdateKeyResponseDescriptor instead')
const SshKeyUpdateKeyResponse$json = {
  '1': 'SshKeyUpdateKeyResponse',
  '2': [
    {
      '1': 'key',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.SshKey',
      '10': 'key'
    },
  ],
};

/// Descriptor for `SshKeyUpdateKeyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyUpdateKeyResponseDescriptor =
    $convert.base64Decode(
        'ChdTc2hLZXlVcGRhdGVLZXlSZXNwb25zZRIqCgNrZXkYASABKAsyGC5vcGVub3hpZGUuYXBpLn'
        'YxLlNzaEtleVIDa2V5');

@$core.Deprecated('Use sshKeyMarkUsedRequestDescriptor instead')
const SshKeyMarkUsedRequest$json = {
  '1': 'SshKeyMarkUsedRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SshKeyMarkUsedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyMarkUsedRequestDescriptor =
    $convert.base64Decode(
        'ChVTc2hLZXlNYXJrVXNlZFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use sshKeyDeleteKeyRequestDescriptor instead')
const SshKeyDeleteKeyRequest$json = {
  '1': 'SshKeyDeleteKeyRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SshKeyDeleteKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sshKeyDeleteKeyRequestDescriptor =
    $convert.base64Decode(
        'ChZTc2hLZXlEZWxldGVLZXlSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

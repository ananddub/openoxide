// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/common.proto.

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

@$core.Deprecated('Use emptyDescriptor instead')
const Empty$json = {
  '1': 'Empty',
};

/// Descriptor for `Empty`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emptyDescriptor =
    $convert.base64Decode('CgVFbXB0eQ==');

@$core.Deprecated('Use userDescriptor instead')
const User$json = {
  '1': 'User',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'email', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'email', '17': true},
    {
      '1': 'first_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'firstName',
      '17': true
    },
    {
      '1': 'last_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'lastName',
      '17': true
    },
    {'1': 'avatar', '3': 5, '4': 1, '5': 9, '10': 'avatar'},
    {'1': 'role', '3': 6, '4': 1, '5': 9, '9': 3, '10': 'role', '17': true},
    {'1': 'group_id', '3': 7, '4': 1, '5': 3, '10': 'groupId'},
  ],
  '8': [
    {'1': '_email'},
    {'1': '_first_name'},
    {'1': '_last_name'},
    {'1': '_role'},
  ],
};

/// Descriptor for `User`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userDescriptor = $convert.base64Decode(
    'CgRVc2VyEg4KAmlkGAEgASgDUgJpZBIZCgVlbWFpbBgCIAEoCUgAUgVlbWFpbIgBARIiCgpmaX'
    'JzdF9uYW1lGAMgASgJSAFSCWZpcnN0TmFtZYgBARIgCglsYXN0X25hbWUYBCABKAlIAlIIbGFz'
    'dE5hbWWIAQESFgoGYXZhdGFyGAUgASgJUgZhdmF0YXISFwoEcm9sZRgGIAEoCUgDUgRyb2xliA'
    'EBEhkKCGdyb3VwX2lkGAcgASgDUgdncm91cElkQggKBl9lbWFpbEINCgtfZmlyc3RfbmFtZUIM'
    'CgpfbGFzdF9uYW1lQgcKBV9yb2xl');

@$core.Deprecated('Use tokenPairDescriptor instead')
const TokenPair$json = {
  '1': 'TokenPair',
  '2': [
    {'1': 'access_token', '3': 1, '4': 1, '5': 9, '10': 'accessToken'},
    {'1': 'refresh_token', '3': 2, '4': 1, '5': 9, '10': 'refreshToken'},
  ],
};

/// Descriptor for `TokenPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List tokenPairDescriptor = $convert.base64Decode(
    'CglUb2tlblBhaXISIQoMYWNjZXNzX3Rva2VuGAEgASgJUgthY2Nlc3NUb2tlbhIjCg1yZWZyZX'
    'NoX3Rva2VuGAIgASgJUgxyZWZyZXNoVG9rZW4=');

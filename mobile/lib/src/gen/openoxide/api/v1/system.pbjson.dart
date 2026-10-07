// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/system.proto.

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

@$core.Deprecated('Use systemHealthRequestDescriptor instead')
const SystemHealthRequest$json = {
  '1': 'SystemHealthRequest',
};

/// Descriptor for `SystemHealthRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List systemHealthRequestDescriptor =
    $convert.base64Decode('ChNTeXN0ZW1IZWFsdGhSZXF1ZXN0');

@$core.Deprecated('Use systemHealthResponseDescriptor instead')
const SystemHealthResponse$json = {
  '1': 'SystemHealthResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'message',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'message',
      '17': true
    },
  ],
  '8': [
    {'1': '_message'},
  ],
};

/// Descriptor for `SystemHealthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List systemHealthResponseDescriptor = $convert.base64Decode(
    'ChRTeXN0ZW1IZWFsdGhSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEicKB21lc3'
    'NhZ2UYAiABKAlCCLpIBXIDGIAgSABSB21lc3NhZ2WIAQFCCgoIX21lc3NhZ2U=');

@$core.Deprecated('Use systemTestConfigRequestDescriptor instead')
const SystemTestConfigRequest$json = {
  '1': 'SystemTestConfigRequest',
};

/// Descriptor for `SystemTestConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List systemTestConfigRequestDescriptor =
    $convert.base64Decode('ChdTeXN0ZW1UZXN0Q29uZmlnUmVxdWVzdA==');

@$core.Deprecated('Use systemTestConfigResponseDescriptor instead')
const SystemTestConfigResponse$json = {
  '1': 'SystemTestConfigResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'message',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'message',
      '17': true
    },
  ],
  '8': [
    {'1': '_message'},
  ],
};

/// Descriptor for `SystemTestConfigResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List systemTestConfigResponseDescriptor =
    $convert.base64Decode(
        'ChhTeXN0ZW1UZXN0Q29uZmlnUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2VzcxInCg'
        'dtZXNzYWdlGAIgASgJQgi6SAVyAxiAIEgAUgdtZXNzYWdliAEBQgoKCF9tZXNzYWdl');

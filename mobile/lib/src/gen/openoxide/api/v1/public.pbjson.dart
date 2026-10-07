// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/public.proto.

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

@$core.Deprecated('Use listGitBranchesRequestDescriptor instead')
const ListGitBranchesRequest$json = {
  '1': 'ListGitBranchesRequest',
  '2': [
    {
      '1': 'repository_url',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'repositoryUrl'
    },
  ],
};

/// Descriptor for `ListGitBranchesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listGitBranchesRequestDescriptor =
    $convert.base64Decode(
        'ChZMaXN0R2l0QnJhbmNoZXNSZXF1ZXN0Ei8KDnJlcG9zaXRvcnlfdXJsGAEgASgJQgi6SAVyA4'
        'gBAVINcmVwb3NpdG9yeVVybA==');

@$core.Deprecated('Use listGitBranchesResponseDescriptor instead')
const ListGitBranchesResponse$json = {
  '1': 'ListGitBranchesResponse',
  '2': [
    {'1': 'branches', '3': 1, '4': 3, '5': 9, '10': 'branches'},
  ],
};

/// Descriptor for `ListGitBranchesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listGitBranchesResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0R2l0QnJhbmNoZXNSZXNwb25zZRIaCghicmFuY2hlcxgBIAMoCVIIYnJhbmNoZXM=');

@$core.Deprecated('Use generateDomainRequestDescriptor instead')
const GenerateDomainRequest$json = {
  '1': 'GenerateDomainRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
  ],
};

/// Descriptor for `GenerateDomainRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List generateDomainRequestDescriptor = $convert.base64Decode(
    'ChVHZW5lcmF0ZURvbWFpblJlcXVlc3QSHwoEbmFtZRgBIAEoCUILukgIcgYQARg/aAFSBG5hbW'
    'U=');

@$core.Deprecated('Use generateDomainResponseDescriptor instead')
const GenerateDomainResponse$json = {
  '1': 'GenerateDomainResponse',
  '2': [
    {'1': 'domain', '3': 1, '4': 1, '5': 9, '10': 'domain'},
  ],
};

/// Descriptor for `GenerateDomainResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List generateDomainResponseDescriptor =
    $convert.base64Decode(
        'ChZHZW5lcmF0ZURvbWFpblJlc3BvbnNlEhYKBmRvbWFpbhgBIAEoCVIGZG9tYWlu');

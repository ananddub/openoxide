// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/destination.proto.

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

@$core.Deprecated('Use destinationDescriptor instead')
const Destination$json = {
  '1': 'Destination',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'provider', '3': 3, '4': 1, '5': 9, '10': 'provider'},
    {'1': 'access_key', '3': 4, '4': 1, '5': 9, '10': 'accessKey'},
    {'1': 'secret_access_key', '3': 5, '4': 1, '5': 9, '10': 'secretAccessKey'},
    {'1': 'bucket', '3': 6, '4': 1, '5': 9, '10': 'bucket'},
    {'1': 'region', '3': 7, '4': 1, '5': 9, '10': 'region'},
    {'1': 'endpoint', '3': 8, '4': 1, '5': 9, '10': 'endpoint'},
    {'1': 'organization_id', '3': 9, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 10, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 11, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Destination`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List destinationDescriptor = $convert.base64Decode(
    'CgtEZXN0aW5hdGlvbhIOCgJpZBgBIAEoCVICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIaCghwcm'
    '92aWRlchgDIAEoCVIIcHJvdmlkZXISHQoKYWNjZXNzX2tleRgEIAEoCVIJYWNjZXNzS2V5EioK'
    'EXNlY3JldF9hY2Nlc3Nfa2V5GAUgASgJUg9zZWNyZXRBY2Nlc3NLZXkSFgoGYnVja2V0GAYgAS'
    'gJUgZidWNrZXQSFgoGcmVnaW9uGAcgASgJUgZyZWdpb24SGgoIZW5kcG9pbnQYCCABKAlSCGVu'
    'ZHBvaW50EicKD29yZ2FuaXphdGlvbl9pZBgJIAEoA1IOb3JnYW5pemF0aW9uSWQSHQoKY3JlYX'
    'RlZF9hdBgKIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYCyABKANSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use listDestinationsRequestDescriptor instead')
const ListDestinationsRequest$json = {
  '1': 'ListDestinationsRequest',
};

/// Descriptor for `ListDestinationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDestinationsRequestDescriptor =
    $convert.base64Decode('ChdMaXN0RGVzdGluYXRpb25zUmVxdWVzdA==');

@$core.Deprecated('Use listDestinationsResponseDescriptor instead')
const ListDestinationsResponse$json = {
  '1': 'ListDestinationsResponse',
  '2': [
    {
      '1': 'destinations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Destination',
      '10': 'destinations'
    },
  ],
};

/// Descriptor for `ListDestinationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDestinationsResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0RGVzdGluYXRpb25zUmVzcG9uc2USQQoMZGVzdGluYXRpb25zGAEgAygLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5EZXN0aW5hdGlvblIMZGVzdGluYXRpb25z');

@$core.Deprecated('Use getDestinationRequestDescriptor instead')
const GetDestinationRequest$json = {
  '1': 'GetDestinationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDestinationRequestDescriptor =
    $convert.base64Decode(
        'ChVHZXREZXN0aW5hdGlvblJlcXVlc3QSFwoCaWQYASABKAlCB7pIBHICEAFSAmlk');

@$core.Deprecated('Use getDestinationResponseDescriptor instead')
const GetDestinationResponse$json = {
  '1': 'GetDestinationResponse',
  '2': [
    {
      '1': 'destination',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Destination',
      '10': 'destination'
    },
  ],
};

/// Descriptor for `GetDestinationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDestinationResponseDescriptor =
    $convert.base64Decode(
        'ChZHZXREZXN0aW5hdGlvblJlc3BvbnNlEj8KC2Rlc3RpbmF0aW9uGAEgASgLMh0ub3Blbm94aW'
        'RlLmFwaS52MS5EZXN0aW5hdGlvblILZGVzdGluYXRpb24=');

@$core.Deprecated('Use createDestinationRequestDescriptor instead')
const CreateDestinationRequest$json = {
  '1': 'CreateDestinationRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'provider', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'provider'},
    {'1': 'access_key', '3': 3, '4': 1, '5': 9, '10': 'accessKey'},
    {'1': 'secret_access_key', '3': 4, '4': 1, '5': 9, '10': 'secretAccessKey'},
    {'1': 'bucket', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'bucket'},
    {'1': 'region', '3': 6, '4': 1, '5': 9, '10': 'region'},
    {'1': 'endpoint', '3': 7, '4': 1, '5': 9, '10': 'endpoint'},
  ],
};

/// Descriptor for `CreateDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDestinationRequestDescriptor = $convert.base64Decode(
    'ChhDcmVhdGVEZXN0aW5hdGlvblJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AVIEbm'
    'FtZRIjCghwcm92aWRlchgCIAEoCUIHukgEcgIQAVIIcHJvdmlkZXISHQoKYWNjZXNzX2tleRgD'
    'IAEoCVIJYWNjZXNzS2V5EioKEXNlY3JldF9hY2Nlc3Nfa2V5GAQgASgJUg9zZWNyZXRBY2Nlc3'
    'NLZXkSHwoGYnVja2V0GAUgASgJQge6SARyAhABUgZidWNrZXQSFgoGcmVnaW9uGAYgASgJUgZy'
    'ZWdpb24SGgoIZW5kcG9pbnQYByABKAlSCGVuZHBvaW50');

@$core.Deprecated('Use createDestinationResponseDescriptor instead')
const CreateDestinationResponse$json = {
  '1': 'CreateDestinationResponse',
  '2': [
    {
      '1': 'destination',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Destination',
      '10': 'destination'
    },
  ],
};

/// Descriptor for `CreateDestinationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDestinationResponseDescriptor =
    $convert.base64Decode(
        'ChlDcmVhdGVEZXN0aW5hdGlvblJlc3BvbnNlEj8KC2Rlc3RpbmF0aW9uGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5EZXN0aW5hdGlvblILZGVzdGluYXRpb24=');

@$core.Deprecated('Use updateDestinationRequestDescriptor instead')
const UpdateDestinationRequest$json = {
  '1': 'UpdateDestinationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'id'},
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
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'provider',
      '17': true
    },
    {
      '1': 'access_key',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'accessKey',
      '17': true
    },
    {
      '1': 'secret_access_key',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'secretAccessKey',
      '17': true
    },
    {'1': 'bucket', '3': 6, '4': 1, '5': 9, '9': 4, '10': 'bucket', '17': true},
    {'1': 'region', '3': 7, '4': 1, '5': 9, '9': 5, '10': 'region', '17': true},
    {
      '1': 'endpoint',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'endpoint',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_provider'},
    {'1': '_access_key'},
    {'1': '_secret_access_key'},
    {'1': '_bucket'},
    {'1': '_region'},
    {'1': '_endpoint'},
  ],
};

/// Descriptor for `UpdateDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDestinationRequestDescriptor = $convert.base64Decode(
    'ChhVcGRhdGVEZXN0aW5hdGlvblJlcXVlc3QSFwoCaWQYASABKAlCB7pIBHICEAFSAmlkEiEKBG'
    '5hbWUYAiABKAlCCLpIBXIDGP8BSABSBG5hbWWIAQESHwoIcHJvdmlkZXIYAyABKAlIAVIIcHJv'
    'dmlkZXKIAQESIgoKYWNjZXNzX2tleRgEIAEoCUgCUglhY2Nlc3NLZXmIAQESLwoRc2VjcmV0X2'
    'FjY2Vzc19rZXkYBSABKAlIA1IPc2VjcmV0QWNjZXNzS2V5iAEBEhsKBmJ1Y2tldBgGIAEoCUgE'
    'UgZidWNrZXSIAQESGwoGcmVnaW9uGAcgASgJSAVSBnJlZ2lvbogBARIfCghlbmRwb2ludBgIIA'
    'EoCUgGUghlbmRwb2ludIgBAUIHCgVfbmFtZUILCglfcHJvdmlkZXJCDQoLX2FjY2Vzc19rZXlC'
    'FAoSX3NlY3JldF9hY2Nlc3Nfa2V5QgkKB19idWNrZXRCCQoHX3JlZ2lvbkILCglfZW5kcG9pbn'
    'Q=');

@$core.Deprecated('Use updateDestinationResponseDescriptor instead')
const UpdateDestinationResponse$json = {
  '1': 'UpdateDestinationResponse',
  '2': [
    {
      '1': 'destination',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Destination',
      '10': 'destination'
    },
  ],
};

/// Descriptor for `UpdateDestinationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateDestinationResponseDescriptor =
    $convert.base64Decode(
        'ChlVcGRhdGVEZXN0aW5hdGlvblJlc3BvbnNlEj8KC2Rlc3RpbmF0aW9uGAEgASgLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5EZXN0aW5hdGlvblILZGVzdGluYXRpb24=');

@$core.Deprecated('Use deleteDestinationRequestDescriptor instead')
const DeleteDestinationRequest$json = {
  '1': 'DeleteDestinationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteDestinationRequestDescriptor =
    $convert.base64Decode(
        'ChhEZWxldGVEZXN0aW5hdGlvblJlcXVlc3QSFwoCaWQYASABKAlCB7pIBHICEAFSAmlk');

@$core.Deprecated('Use testDestinationRequestDescriptor instead')
const TestDestinationRequest$json = {
  '1': 'TestDestinationRequest',
  '2': [
    {'1': 'provider', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'provider'},
    {'1': 'access_key', '3': 2, '4': 1, '5': 9, '10': 'accessKey'},
    {'1': 'secret_access_key', '3': 3, '4': 1, '5': 9, '10': 'secretAccessKey'},
    {'1': 'bucket', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'bucket'},
    {'1': 'region', '3': 5, '4': 1, '5': 9, '10': 'region'},
    {'1': 'endpoint', '3': 6, '4': 1, '5': 9, '10': 'endpoint'},
  ],
};

/// Descriptor for `TestDestinationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDestinationRequestDescriptor = $convert.base64Decode(
    'ChZUZXN0RGVzdGluYXRpb25SZXF1ZXN0EiMKCHByb3ZpZGVyGAEgASgJQge6SARyAhABUghwcm'
    '92aWRlchIdCgphY2Nlc3Nfa2V5GAIgASgJUglhY2Nlc3NLZXkSKgoRc2VjcmV0X2FjY2Vzc19r'
    'ZXkYAyABKAlSD3NlY3JldEFjY2Vzc0tleRIfCgZidWNrZXQYBCABKAlCB7pIBHICEAFSBmJ1Y2'
    'tldBIWCgZyZWdpb24YBSABKAlSBnJlZ2lvbhIaCghlbmRwb2ludBgGIAEoCVIIZW5kcG9pbnQ=');

@$core.Deprecated('Use testDestinationResponseDescriptor instead')
const TestDestinationResponse$json = {
  '1': 'TestDestinationResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `TestDestinationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testDestinationResponseDescriptor =
    $convert.base64Decode(
        'ChdUZXN0RGVzdGluYXRpb25SZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

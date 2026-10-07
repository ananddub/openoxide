// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/environment.proto.

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

@$core.Deprecated('Use environmentDescriptor instead')
const Environment$json = {
  '1': 'Environment',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
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
    {
      '1': 'environment_variables',
      '3': 4,
      '4': 1,
      '5': 9,
      '10': 'environmentVariables'
    },
    {'1': 'is_default', '3': 5, '4': 1, '5': 8, '10': 'isDefault'},
    {'1': 'project_id', '3': 6, '4': 1, '5': 3, '10': 'projectId'},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `Environment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List environmentDescriptor = $convert.base64Decode(
    'CgtFbnZpcm9ubWVudBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSEgoEbmFtZRgCIAEoCVIEbm'
    'FtZRIlCgtkZXNjcmlwdGlvbhgDIAEoCUgAUgtkZXNjcmlwdGlvbogBARIzChVlbnZpcm9ubWVu'
    'dF92YXJpYWJsZXMYBCABKAlSFGVudmlyb25tZW50VmFyaWFibGVzEh0KCmlzX2RlZmF1bHQYBS'
    'ABKAhSCWlzRGVmYXVsdBIdCgpwcm9qZWN0X2lkGAYgASgDUglwcm9qZWN0SWQSHQoKY3JlYXRl'
    'ZF9hdBgHIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYCCABKANSCXVwZGF0ZWRBdEIOCg'
    'xfZGVzY3JpcHRpb24=');

@$core.Deprecated('Use getEnvironmentRequestDescriptor instead')
const GetEnvironmentRequest$json = {
  '1': 'GetEnvironmentRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `GetEnvironmentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEnvironmentRequestDescriptor = $convert
    .base64Decode('ChVHZXRFbnZpcm9ubWVudFJlcXVlc3QSDgoCaWQYASABKANSAmlk');

@$core.Deprecated('Use listEnvironmentsRequestDescriptor instead')
const ListEnvironmentsRequest$json = {
  '1': 'ListEnvironmentsRequest',
  '2': [
    {'1': 'project_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'projectId'},
  ],
};

/// Descriptor for `ListEnvironmentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEnvironmentsRequestDescriptor =
    $convert.base64Decode(
        'ChdMaXN0RW52aXJvbm1lbnRzUmVxdWVzdBImCgpwcm9qZWN0X2lkGAEgASgDQge6SAQiAiAAUg'
        'lwcm9qZWN0SWQ=');

@$core.Deprecated('Use listEnvironmentsResponseDescriptor instead')
const ListEnvironmentsResponse$json = {
  '1': 'ListEnvironmentsResponse',
  '2': [
    {
      '1': 'environments',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Environment',
      '10': 'environments'
    },
  ],
};

/// Descriptor for `ListEnvironmentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEnvironmentsResponseDescriptor =
    $convert.base64Decode(
        'ChhMaXN0RW52aXJvbm1lbnRzUmVzcG9uc2USQQoMZW52aXJvbm1lbnRzGAEgAygLMh0ub3Blbm'
        '94aWRlLmFwaS52MS5FbnZpcm9ubWVudFIMZW52aXJvbm1lbnRz');

@$core.Deprecated('Use createEnvironmentRequestDescriptor instead')
const CreateEnvironmentRequest$json = {
  '1': 'CreateEnvironmentRequest',
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
      '1': 'environment_variables',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'environmentVariables'
    },
    {'1': 'is_default', '3': 4, '4': 1, '5': 8, '10': 'isDefault'},
    {'1': 'project_id', '3': 5, '4': 1, '5': 3, '8': {}, '10': 'projectId'},
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `CreateEnvironmentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createEnvironmentRequestDescriptor = $convert.base64Decode(
    'ChhDcmVhdGVFbnZpcm9ubWVudFJlcXVlc3QSHgoEbmFtZRgBIAEoCUIKukgHcgUQARj/AVIEbm'
    'FtZRIlCgtkZXNjcmlwdGlvbhgCIAEoCUgAUgtkZXNjcmlwdGlvbogBARIzChVlbnZpcm9ubWVu'
    'dF92YXJpYWJsZXMYAyABKAlSFGVudmlyb25tZW50VmFyaWFibGVzEh0KCmlzX2RlZmF1bHQYBC'
    'ABKAhSCWlzRGVmYXVsdBImCgpwcm9qZWN0X2lkGAUgASgDQge6SAQiAiAAUglwcm9qZWN0SWRC'
    'DgoMX2Rlc2NyaXB0aW9u');

@$core.Deprecated('Use updateEnvironmentRequestDescriptor instead')
const UpdateEnvironmentRequest$json = {
  '1': 'UpdateEnvironmentRequest',
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
      '1': 'environment_variables',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'environmentVariables',
      '17': true
    },
    {
      '1': 'is_default',
      '3': 5,
      '4': 1,
      '5': 8,
      '9': 3,
      '10': 'isDefault',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_environment_variables'},
    {'1': '_is_default'},
  ],
};

/// Descriptor for `UpdateEnvironmentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateEnvironmentRequestDescriptor = $convert.base64Decode(
    'ChhVcGRhdGVFbnZpcm9ubWVudFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEhcKBG'
    '5hbWUYAiABKAlIAFIEbmFtZYgBARIlCgtkZXNjcmlwdGlvbhgDIAEoCUgBUgtkZXNjcmlwdGlv'
    'bogBARI4ChVlbnZpcm9ubWVudF92YXJpYWJsZXMYBCABKAlIAlIUZW52aXJvbm1lbnRWYXJpYW'
    'JsZXOIAQESIgoKaXNfZGVmYXVsdBgFIAEoCEgDUglpc0RlZmF1bHSIAQFCBwoFX25hbWVCDgoM'
    'X2Rlc2NyaXB0aW9uQhgKFl9lbnZpcm9ubWVudF92YXJpYWJsZXNCDQoLX2lzX2RlZmF1bHQ=');

@$core.Deprecated('Use setDefaultEnvironmentRequestDescriptor instead')
const SetDefaultEnvironmentRequest$json = {
  '1': 'SetDefaultEnvironmentRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `SetDefaultEnvironmentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDefaultEnvironmentRequestDescriptor =
    $convert.base64Decode(
        'ChxTZXREZWZhdWx0RW52aXJvbm1lbnRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use deleteEnvironmentRequestDescriptor instead')
const DeleteEnvironmentRequest$json = {
  '1': 'DeleteEnvironmentRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteEnvironmentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteEnvironmentRequestDescriptor =
    $convert.base64Decode(
        'ChhEZWxldGVFbnZpcm9ubWVudFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

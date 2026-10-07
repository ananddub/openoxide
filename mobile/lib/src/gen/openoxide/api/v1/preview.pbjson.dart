// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/preview.proto.

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

@$core.Deprecated('Use previewDeploymentDescriptor instead')
const PreviewDeployment$json = {
  '1': 'PreviewDeployment',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'application_id', '3': 2, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'status', '3': 3, '4': 1, '5': 9, '10': 'status'},
    {'1': 'branch', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'branch', '17': true},
    {'1': 'url', '3': 5, '4': 1, '5': 9, '9': 1, '10': 'url', '17': true},
    {'1': 'error', '3': 6, '4': 1, '5': 9, '9': 2, '10': 'error', '17': true},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_branch'},
    {'1': '_url'},
    {'1': '_error'},
  ],
};

/// Descriptor for `PreviewDeployment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List previewDeploymentDescriptor = $convert.base64Decode(
    'ChFQcmV2aWV3RGVwbG95bWVudBIOCgJpZBgBIAEoA1ICaWQSJQoOYXBwbGljYXRpb25faWQYAi'
    'ABKANSDWFwcGxpY2F0aW9uSWQSFgoGc3RhdHVzGAMgASgJUgZzdGF0dXMSGwoGYnJhbmNoGAQg'
    'ASgJSABSBmJyYW5jaIgBARIVCgN1cmwYBSABKAlIAVIDdXJsiAEBEhkKBWVycm9yGAYgASgJSA'
    'JSBWVycm9yiAEBEh0KCmNyZWF0ZWRfYXQYByABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0'
    'GAggASgDUgl1cGRhdGVkQXRCCQoHX2JyYW5jaEIGCgRfdXJsQggKBl9lcnJvcg==');

@$core.Deprecated('Use listPreviewsRequestDescriptor instead')
const ListPreviewsRequest$json = {
  '1': 'ListPreviewsRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'applicationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_application_id'},
  ],
};

/// Descriptor for `ListPreviewsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPreviewsRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0UHJldmlld3NSZXF1ZXN0EjMKDmFwcGxpY2F0aW9uX2lkGAEgASgDQge6SAQiAiAASA'
    'BSDWFwcGxpY2F0aW9uSWSIAQFCEQoPX2FwcGxpY2F0aW9uX2lk');

@$core.Deprecated('Use listPreviewsResponseDescriptor instead')
const ListPreviewsResponse$json = {
  '1': 'ListPreviewsResponse',
  '2': [
    {
      '1': 'previews',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PreviewDeployment',
      '10': 'previews'
    },
  ],
};

/// Descriptor for `ListPreviewsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPreviewsResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0UHJldmlld3NSZXNwb25zZRI/CghwcmV2aWV3cxgBIAMoCzIjLm9wZW5veGlkZS5hcG'
    'kudjEuUHJldmlld0RlcGxveW1lbnRSCHByZXZpZXdz');

@$core.Deprecated('Use getPreviewRequestDescriptor instead')
const GetPreviewRequest$json = {
  '1': 'GetPreviewRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetPreviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPreviewRequestDescriptor = $convert.base64Decode(
    'ChFHZXRQcmV2aWV3UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use getPreviewResponseDescriptor instead')
const GetPreviewResponse$json = {
  '1': 'GetPreviewResponse',
  '2': [
    {
      '1': 'preview',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PreviewDeployment',
      '10': 'preview'
    },
  ],
};

/// Descriptor for `GetPreviewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPreviewResponseDescriptor = $convert.base64Decode(
    'ChJHZXRQcmV2aWV3UmVzcG9uc2USPQoHcHJldmlldxgBIAEoCzIjLm9wZW5veGlkZS5hcGkudj'
    'EuUHJldmlld0RlcGxveW1lbnRSB3ByZXZpZXc=');

@$core.Deprecated('Use createPreviewRequestDescriptor instead')
const CreatePreviewRequest$json = {
  '1': 'CreatePreviewRequest',
  '2': [
    {
      '1': 'application_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'applicationId'
    },
    {'1': 'branch', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'branch'},
    {'1': 'commit', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'commit', '17': true},
  ],
  '8': [
    {'1': '_commit'},
  ],
};

/// Descriptor for `CreatePreviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPreviewRequestDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVQcmV2aWV3UmVxdWVzdBIuCg5hcHBsaWNhdGlvbl9pZBgBIAEoA0IHukgEIgIgAF'
    'INYXBwbGljYXRpb25JZBIiCgZicmFuY2gYAiABKAlCCrpIB3IFEAEY/wFSBmJyYW5jaBIbCgZj'
    'b21taXQYAyABKAlIAFIGY29tbWl0iAEBQgkKB19jb21taXQ=');

@$core.Deprecated('Use createPreviewResponseDescriptor instead')
const CreatePreviewResponse$json = {
  '1': 'CreatePreviewResponse',
  '2': [
    {
      '1': 'preview',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PreviewDeployment',
      '10': 'preview'
    },
  ],
};

/// Descriptor for `CreatePreviewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPreviewResponseDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVQcmV2aWV3UmVzcG9uc2USPQoHcHJldmlldxgBIAEoCzIjLm9wZW5veGlkZS5hcG'
    'kudjEuUHJldmlld0RlcGxveW1lbnRSB3ByZXZpZXc=');

@$core.Deprecated('Use redeployPreviewRequestDescriptor instead')
const RedeployPreviewRequest$json = {
  '1': 'RedeployPreviewRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RedeployPreviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List redeployPreviewRequestDescriptor =
    $convert.base64Decode(
        'ChZSZWRlcGxveVByZXZpZXdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use redeployPreviewResponseDescriptor instead')
const RedeployPreviewResponse$json = {
  '1': 'RedeployPreviewResponse',
  '2': [
    {
      '1': 'preview',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PreviewDeployment',
      '10': 'preview'
    },
  ],
};

/// Descriptor for `RedeployPreviewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List redeployPreviewResponseDescriptor =
    $convert.base64Decode(
        'ChdSZWRlcGxveVByZXZpZXdSZXNwb25zZRI9CgdwcmV2aWV3GAEgASgLMiMub3Blbm94aWRlLm'
        'FwaS52MS5QcmV2aWV3RGVwbG95bWVudFIHcHJldmlldw==');

@$core.Deprecated('Use deletePreviewRequestDescriptor instead')
const DeletePreviewRequest$json = {
  '1': 'DeletePreviewRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeletePreviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deletePreviewRequestDescriptor =
    $convert.base64Decode(
        'ChREZWxldGVQcmV2aWV3UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

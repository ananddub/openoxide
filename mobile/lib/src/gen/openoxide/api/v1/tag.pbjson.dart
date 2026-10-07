// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/tag.proto.

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

@$core.Deprecated('Use tagDescriptor instead')
const Tag$json = {
  '1': 'Tag',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'color', '3': 3, '4': 1, '5': 9, '10': 'color'},
    {'1': 'organization_id', '3': 4, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 5, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `Tag`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List tagDescriptor = $convert.base64Decode(
    'CgNUYWcSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSFAoFY29sb3IYAyABKA'
    'lSBWNvbG9yEicKD29yZ2FuaXphdGlvbl9pZBgEIAEoA1IOb3JnYW5pemF0aW9uSWQSHQoKY3Jl'
    'YXRlZF9hdBgFIAEoA1IJY3JlYXRlZEF0');

@$core.Deprecated('Use listTagsRequestDescriptor instead')
const ListTagsRequest$json = {
  '1': 'ListTagsRequest',
};

/// Descriptor for `ListTagsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTagsRequestDescriptor =
    $convert.base64Decode('Cg9MaXN0VGFnc1JlcXVlc3Q=');

@$core.Deprecated('Use listTagsResponseDescriptor instead')
const ListTagsResponse$json = {
  '1': 'ListTagsResponse',
  '2': [
    {
      '1': 'tags',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Tag',
      '10': 'tags'
    },
  ],
};

/// Descriptor for `ListTagsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTagsResponseDescriptor = $convert.base64Decode(
    'ChBMaXN0VGFnc1Jlc3BvbnNlEikKBHRhZ3MYASADKAsyFS5vcGVub3hpZGUuYXBpLnYxLlRhZ1'
    'IEdGFncw==');

@$core.Deprecated('Use getTagRequestDescriptor instead')
const GetTagRequest$json = {
  '1': 'GetTagRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTagRequestDescriptor = $convert
    .base64Decode('Cg1HZXRUYWdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use getTagResponseDescriptor instead')
const GetTagResponse$json = {
  '1': 'GetTagResponse',
  '2': [
    {
      '1': 'tag',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Tag',
      '10': 'tag'
    },
  ],
};

/// Descriptor for `GetTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTagResponseDescriptor = $convert.base64Decode(
    'Cg5HZXRUYWdSZXNwb25zZRInCgN0YWcYASABKAsyFS5vcGVub3hpZGUuYXBpLnYxLlRhZ1IDdG'
    'Fn');

@$core.Deprecated('Use createTagRequestDescriptor instead')
const CreateTagRequest$json = {
  '1': 'CreateTagRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'color', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'color'},
    {
      '1': 'organization_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `CreateTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTagRequestDescriptor = $convert.base64Decode(
    'ChBDcmVhdGVUYWdSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG5hbWUSHQoFY2'
    '9sb3IYAiABKAlCB7pIBHICEAFSBWNvbG9yEjUKD29yZ2FuaXphdGlvbl9pZBgDIAEoA0IHukgE'
    'IgIgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use createTagResponseDescriptor instead')
const CreateTagResponse$json = {
  '1': 'CreateTagResponse',
  '2': [
    {
      '1': 'tag',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Tag',
      '10': 'tag'
    },
  ],
};

/// Descriptor for `CreateTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTagResponseDescriptor = $convert.base64Decode(
    'ChFDcmVhdGVUYWdSZXNwb25zZRInCgN0YWcYASABKAsyFS5vcGVub3hpZGUuYXBpLnYxLlRhZ1'
    'IDdGFn');

@$core.Deprecated('Use updateTagRequestDescriptor instead')
const UpdateTagRequest$json = {
  '1': 'UpdateTagRequest',
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
    {'1': 'color', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'color', '17': true},
  ],
  '8': [
    {'1': '_name'},
    {'1': '_color'},
  ],
};

/// Descriptor for `UpdateTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateTagRequestDescriptor = $convert.base64Decode(
    'ChBVcGRhdGVUYWdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIhCgRuYW1lGAIgAS'
    'gJQgi6SAVyAxj/AUgAUgRuYW1liAEBEhkKBWNvbG9yGAMgASgJSAFSBWNvbG9yiAEBQgcKBV9u'
    'YW1lQggKBl9jb2xvcg==');

@$core.Deprecated('Use updateTagResponseDescriptor instead')
const UpdateTagResponse$json = {
  '1': 'UpdateTagResponse',
  '2': [
    {
      '1': 'tag',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Tag',
      '10': 'tag'
    },
  ],
};

/// Descriptor for `UpdateTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateTagResponseDescriptor = $convert.base64Decode(
    'ChFVcGRhdGVUYWdSZXNwb25zZRInCgN0YWcYASABKAsyFS5vcGVub3hpZGUuYXBpLnYxLlRhZ1'
    'IDdGFn');

@$core.Deprecated('Use deleteTagRequestDescriptor instead')
const DeleteTagRequest$json = {
  '1': 'DeleteTagRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteTagRequestDescriptor = $convert.base64Decode(
    'ChBEZWxldGVUYWdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use addTagRequestDescriptor instead')
const AddTagRequest$json = {
  '1': 'AddTagRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `AddTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addTagRequestDescriptor = $convert
    .base64Decode('Cg1BZGRUYWdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use addTagResponseDescriptor instead')
const AddTagResponse$json = {
  '1': 'AddTagResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `AddTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addTagResponseDescriptor = $convert
    .base64Decode('Cg5BZGRUYWdSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

@$core.Deprecated('Use removeTagRequestDescriptor instead')
const RemoveTagRequest$json = {
  '1': 'RemoveTagRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RemoveTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeTagRequestDescriptor = $convert.base64Decode(
    'ChBSZW1vdmVUYWdSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use listProjectTagsRequestDescriptor instead')
const ListProjectTagsRequest$json = {
  '1': 'ListProjectTagsRequest',
  '2': [
    {'1': 'project_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'projectId'},
  ],
};

/// Descriptor for `ListProjectTagsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listProjectTagsRequestDescriptor =
    $convert.base64Decode(
        'ChZMaXN0UHJvamVjdFRhZ3NSZXF1ZXN0EiYKCnByb2plY3RfaWQYASABKANCB7pIBCICIABSCX'
        'Byb2plY3RJZA==');

@$core.Deprecated('Use listProjectTagsResponseDescriptor instead')
const ListProjectTagsResponse$json = {
  '1': 'ListProjectTagsResponse',
  '2': [
    {
      '1': 'tags',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Tag',
      '10': 'tags'
    },
  ],
};

/// Descriptor for `ListProjectTagsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listProjectTagsResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0UHJvamVjdFRhZ3NSZXNwb25zZRIpCgR0YWdzGAEgAygLMhUub3Blbm94aWRlLmFwaS'
        '52MS5UYWdSBHRhZ3M=');

@$core.Deprecated('Use attachProjectTagRequestDescriptor instead')
const AttachProjectTagRequest$json = {
  '1': 'AttachProjectTagRequest',
  '2': [
    {'1': 'project_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'projectId'},
    {'1': 'tag_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'tagId'},
  ],
};

/// Descriptor for `AttachProjectTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List attachProjectTagRequestDescriptor =
    $convert.base64Decode(
        'ChdBdHRhY2hQcm9qZWN0VGFnUmVxdWVzdBImCgpwcm9qZWN0X2lkGAEgASgDQge6SAQiAiAAUg'
        'lwcm9qZWN0SWQSHgoGdGFnX2lkGAIgASgDQge6SAQiAiAAUgV0YWdJZA==');

@$core.Deprecated('Use attachProjectTagResponseDescriptor instead')
const AttachProjectTagResponse$json = {
  '1': 'AttachProjectTagResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `AttachProjectTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List attachProjectTagResponseDescriptor =
    $convert.base64Decode(
        'ChhBdHRhY2hQcm9qZWN0VGFnUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2Vzcw==');

@$core.Deprecated('Use detachProjectTagRequestDescriptor instead')
const DetachProjectTagRequest$json = {
  '1': 'DetachProjectTagRequest',
  '2': [
    {'1': 'project_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'projectId'},
    {'1': 'tag_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'tagId'},
  ],
};

/// Descriptor for `DetachProjectTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List detachProjectTagRequestDescriptor =
    $convert.base64Decode(
        'ChdEZXRhY2hQcm9qZWN0VGFnUmVxdWVzdBImCgpwcm9qZWN0X2lkGAEgASgDQge6SAQiAiAAUg'
        'lwcm9qZWN0SWQSHgoGdGFnX2lkGAIgASgDQge6SAQiAiAAUgV0YWdJZA==');

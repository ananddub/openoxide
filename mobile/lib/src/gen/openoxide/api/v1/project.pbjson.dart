// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/project.proto.

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

@$core.Deprecated('Use projectDescriptor instead')
const Project$json = {
  '1': 'Project',
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
    {'1': 'organization_id', '3': 5, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 7, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `Project`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List projectDescriptor = $convert.base64Decode(
    'CgdQcm9qZWN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEi'
    'UKC2Rlc2NyaXB0aW9uGAMgASgJSABSC2Rlc2NyaXB0aW9uiAEBEjMKFWVudmlyb25tZW50X3Zh'
    'cmlhYmxlcxgEIAEoCVIUZW52aXJvbm1lbnRWYXJpYWJsZXMSJwoPb3JnYW5pemF0aW9uX2lkGA'
    'UgASgDUg5vcmdhbml6YXRpb25JZBIdCgpjcmVhdGVkX2F0GAYgASgDUgljcmVhdGVkQXQSHQoK'
    'dXBkYXRlZF9hdBgHIAEoA1IJdXBkYXRlZEF0Qg4KDF9kZXNjcmlwdGlvbg==');

@$core.Deprecated('Use getProjectRequestDescriptor instead')
const GetProjectRequest$json = {
  '1': 'GetProjectRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `GetProjectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getProjectRequestDescriptor =
    $convert.base64Decode('ChFHZXRQcm9qZWN0UmVxdWVzdBIOCgJpZBgBIAEoA1ICaWQ=');

@$core.Deprecated('Use listProjectsRequestDescriptor instead')
const ListProjectsRequest$json = {
  '1': 'ListProjectsRequest',
  '2': [
    {
      '1': 'organization_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
};

/// Descriptor for `ListProjectsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listProjectsRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0UHJvamVjdHNSZXF1ZXN0EjAKD29yZ2FuaXphdGlvbl9pZBgBIAEoA0IHukgEIgIgAF'
    'IOb3JnYW5pemF0aW9uSWQ=');

@$core.Deprecated('Use listProjectsResponseDescriptor instead')
const ListProjectsResponse$json = {
  '1': 'ListProjectsResponse',
  '2': [
    {
      '1': 'projects',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Project',
      '10': 'projects'
    },
  ],
};

/// Descriptor for `ListProjectsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listProjectsResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0UHJvamVjdHNSZXNwb25zZRI1Cghwcm9qZWN0cxgBIAMoCzIZLm9wZW5veGlkZS5hcG'
    'kudjEuUHJvamVjdFIIcHJvamVjdHM=');

@$core.Deprecated('Use createProjectRequestDescriptor instead')
const CreateProjectRequest$json = {
  '1': 'CreateProjectRequest',
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
    {
      '1': 'organization_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
  '8': [
    {'1': '_description'},
  ],
};

/// Descriptor for `CreateProjectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createProjectRequestDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVQcm9qZWN0UmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP8BUgRuYW1lEi'
    'UKC2Rlc2NyaXB0aW9uGAIgASgJSABSC2Rlc2NyaXB0aW9uiAEBEjMKFWVudmlyb25tZW50X3Zh'
    'cmlhYmxlcxgDIAEoCVIUZW52aXJvbm1lbnRWYXJpYWJsZXMSMAoPb3JnYW5pemF0aW9uX2lkGA'
    'QgASgDQge6SAQiAiAAUg5vcmdhbml6YXRpb25JZEIOCgxfZGVzY3JpcHRpb24=');

@$core.Deprecated('Use updateProjectRequestDescriptor instead')
const UpdateProjectRequest$json = {
  '1': 'UpdateProjectRequest',
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
  ],
  '8': [
    {'1': '_name'},
    {'1': '_description'},
    {'1': '_environment_variables'},
  ],
};

/// Descriptor for `UpdateProjectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateProjectRequestDescriptor = $convert.base64Decode(
    'ChRVcGRhdGVQcm9qZWN0UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSFwoEbmFtZR'
    'gCIAEoCUgAUgRuYW1liAEBEiUKC2Rlc2NyaXB0aW9uGAMgASgJSAFSC2Rlc2NyaXB0aW9uiAEB'
    'EjgKFWVudmlyb25tZW50X3ZhcmlhYmxlcxgEIAEoCUgCUhRlbnZpcm9ubWVudFZhcmlhYmxlc4'
    'gBAUIHCgVfbmFtZUIOCgxfZGVzY3JpcHRpb25CGAoWX2Vudmlyb25tZW50X3ZhcmlhYmxlcw==');

@$core.Deprecated('Use deleteProjectRequestDescriptor instead')
const DeleteProjectRequest$json = {
  '1': 'DeleteProjectRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteProjectRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteProjectRequestDescriptor =
    $convert.base64Decode(
        'ChREZWxldGVQcm9qZWN0UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

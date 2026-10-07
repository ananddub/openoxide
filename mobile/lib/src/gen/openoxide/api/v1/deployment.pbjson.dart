// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/deployment.proto.

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

@$core.Deprecated('Use deploymentDescriptor instead')
const Deployment$json = {
  '1': 'Deployment',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'resource_type', '3': 2, '4': 1, '5': 9, '10': 'resourceType'},
    {'1': 'resource_id', '3': 3, '4': 1, '5': 3, '10': 'resourceId'},
    {'1': 'status', '3': 4, '4': 1, '5': 9, '10': 'status'},
    {'1': 'error', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
    {'1': 'logs', '3': 6, '4': 1, '5': 9, '9': 1, '10': 'logs', '17': true},
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
    {'1': 'user_id', '3': 9, '4': 1, '5': 3, '10': 'userId'},
    {
      '1': 'user_email',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'userEmail',
      '17': true
    },
  ],
  '8': [
    {'1': '_error'},
    {'1': '_logs'},
    {'1': '_user_email'},
  ],
};

/// Descriptor for `Deployment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deploymentDescriptor = $convert.base64Decode(
    'CgpEZXBsb3ltZW50Eg4KAmlkGAEgASgDUgJpZBIjCg1yZXNvdXJjZV90eXBlGAIgASgJUgxyZX'
    'NvdXJjZVR5cGUSHwoLcmVzb3VyY2VfaWQYAyABKANSCnJlc291cmNlSWQSFgoGc3RhdHVzGAQg'
    'ASgJUgZzdGF0dXMSGQoFZXJyb3IYBSABKAlIAFIFZXJyb3KIAQESFwoEbG9ncxgGIAEoCUgBUg'
    'Rsb2dziAEBEh0KCmNyZWF0ZWRfYXQYByABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAgg'
    'ASgDUgl1cGRhdGVkQXQSFwoHdXNlcl9pZBgJIAEoA1IGdXNlcklkEiIKCnVzZXJfZW1haWwYCi'
    'ABKAlIAlIJdXNlckVtYWlsiAEBQggKBl9lcnJvckIHCgVfbG9nc0INCgtfdXNlcl9lbWFpbA==');

@$core.Deprecated('Use listDeploymentsRequestDescriptor instead')
const ListDeploymentsRequest$json = {
  '1': 'ListDeploymentsRequest',
  '2': [
    {
      '1': 'resource_type',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'resourceType',
      '17': true
    },
    {
      '1': 'resource_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'resourceId',
      '17': true
    },
    {
      '1': 'limit',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'limit',
      '17': true
    },
    {
      '1': 'offset',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 3,
      '10': 'offset',
      '17': true
    },
  ],
  '8': [
    {'1': '_resource_type'},
    {'1': '_resource_id'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `ListDeploymentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDeploymentsRequestDescriptor = $convert.base64Decode(
    'ChZMaXN0RGVwbG95bWVudHNSZXF1ZXN0EigKDXJlc291cmNlX3R5cGUYASABKAlIAFIMcmVzb3'
    'VyY2VUeXBliAEBEi0KC3Jlc291cmNlX2lkGAIgASgDQge6SAQiAiAASAFSCnJlc291cmNlSWSI'
    'AQESJQoFbGltaXQYAyABKANCCrpIByIFGOgHKAFIAlIFbGltaXSIAQESJAoGb2Zmc2V0GAQgAS'
    'gDQge6SAQiAigASANSBm9mZnNldIgBAUIQCg5fcmVzb3VyY2VfdHlwZUIOCgxfcmVzb3VyY2Vf'
    'aWRCCAoGX2xpbWl0QgkKB19vZmZzZXQ=');

@$core.Deprecated('Use listDeploymentsResponseDescriptor instead')
const ListDeploymentsResponse$json = {
  '1': 'ListDeploymentsResponse',
  '2': [
    {
      '1': 'deployments',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Deployment',
      '10': 'deployments'
    },
  ],
};

/// Descriptor for `ListDeploymentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDeploymentsResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0RGVwbG95bWVudHNSZXNwb25zZRI+CgtkZXBsb3ltZW50cxgBIAMoCzIcLm9wZW5veG'
        'lkZS5hcGkudjEuRGVwbG95bWVudFILZGVwbG95bWVudHM=');

@$core.Deprecated('Use getDeploymentRequestDescriptor instead')
const GetDeploymentRequest$json = {
  '1': 'GetDeploymentRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetDeploymentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDeploymentRequestDescriptor =
    $convert.base64Decode(
        'ChRHZXREZXBsb3ltZW50UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use searchLogsRequestDescriptor instead')
const SearchLogsRequest$json = {
  '1': 'SearchLogsRequest',
  '2': [
    {'1': 'query', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'query'},
    {
      '1': 'resource_type',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'resourceType',
      '17': true
    },
    {
      '1': 'resource_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'resourceId',
      '17': true
    },
  ],
  '8': [
    {'1': '_resource_type'},
    {'1': '_resource_id'},
  ],
};

/// Descriptor for `SearchLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchLogsRequestDescriptor = $convert.base64Decode(
    'ChFTZWFyY2hMb2dzUmVxdWVzdBIgCgVxdWVyeRgBIAEoCUIKukgHcgUQARjoB1IFcXVlcnkSKA'
    'oNcmVzb3VyY2VfdHlwZRgCIAEoCUgAUgxyZXNvdXJjZVR5cGWIAQESJAoLcmVzb3VyY2VfaWQY'
    'AyABKANIAVIKcmVzb3VyY2VJZIgBAUIQCg5fcmVzb3VyY2VfdHlwZUIOCgxfcmVzb3VyY2VfaW'
    'Q=');

@$core.Deprecated('Use streamTargetRequestDescriptor instead')
const StreamTargetRequest$json = {
  '1': 'StreamTargetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `StreamTargetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List streamTargetRequestDescriptor =
    $convert.base64Decode(
        'ChNTdHJlYW1UYXJnZXRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use dockerStatsRequestDescriptor instead')
const DockerStatsRequest$json = {
  '1': 'DockerStatsRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'serverId',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `DockerStatsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dockerStatsRequestDescriptor = $convert.base64Decode(
    'ChJEb2NrZXJTdGF0c1JlcXVlc3QSIAoJc2VydmVyX2lkGAEgASgDSABSCHNlcnZlcklkiAEBQg'
    'wKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use logTargetRequestDescriptor instead')
const LogTargetRequest$json = {
  '1': 'LogTargetRequest',
  '2': [
    {'1': 'target', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'target'},
    {
      '1': 'server_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'tail',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'tail',
      '17': true
    },
    {'1': 'filter', '3': 4, '4': 1, '5': 9, '9': 2, '10': 'filter', '17': true},
    {
      '1': 'timestamps',
      '3': 5,
      '4': 1,
      '5': 8,
      '9': 3,
      '10': 'timestamps',
      '17': true
    },
    {'1': 'follow', '3': 6, '4': 1, '5': 8, '9': 4, '10': 'follow', '17': true},
  ],
  '8': [
    {'1': '_server_id'},
    {'1': '_tail'},
    {'1': '_filter'},
    {'1': '_timestamps'},
    {'1': '_follow'},
  ],
};

/// Descriptor for `LogTargetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logTargetRequestDescriptor = $convert.base64Decode(
    'ChBMb2dUYXJnZXRSZXF1ZXN0Eh8KBnRhcmdldBgBIAEoCUIHukgEcgIQAVIGdGFyZ2V0EiAKCX'
    'NlcnZlcl9pZBgCIAEoA0gAUghzZXJ2ZXJJZIgBARIkCgR0YWlsGAMgASgDQgu6SAgiBhigjQYo'
    'AUgBUgR0YWlsiAEBEhsKBmZpbHRlchgEIAEoCUgCUgZmaWx0ZXKIAQESIwoKdGltZXN0YW1wcx'
    'gFIAEoCEgDUgp0aW1lc3RhbXBziAEBEhsKBmZvbGxvdxgGIAEoCEgEUgZmb2xsb3eIAQFCDAoK'
    'X3NlcnZlcl9pZEIHCgVfdGFpbEIJCgdfZmlsdGVyQg0KC190aW1lc3RhbXBzQgkKB19mb2xsb3'
    'c=');

@$core.Deprecated('Use composeLogsRequestDescriptor instead')
const ComposeLogsRequest$json = {
  '1': 'ComposeLogsRequest',
  '2': [
    {
      '1': 'compose_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'composeId',
      '17': true
    },
    {
      '1': 'service',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'service',
      '17': true
    },
    {
      '1': 'server_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'timestamps',
      '3': 4,
      '4': 1,
      '5': 8,
      '9': 3,
      '10': 'timestamps',
      '17': true
    },
    {'1': 'follow', '3': 5, '4': 1, '5': 8, '9': 4, '10': 'follow', '17': true},
    {
      '1': 'tail',
      '3': 6,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'tail',
      '17': true
    },
  ],
  '8': [
    {'1': '_compose_id'},
    {'1': '_service'},
    {'1': '_server_id'},
    {'1': '_timestamps'},
    {'1': '_follow'},
    {'1': '_tail'},
  ],
};

/// Descriptor for `ComposeLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List composeLogsRequestDescriptor = $convert.base64Decode(
    'ChJDb21wb3NlTG9nc1JlcXVlc3QSKwoKY29tcG9zZV9pZBgBIAEoA0IHukgEIgIgAEgAUgljb2'
    '1wb3NlSWSIAQESJwoHc2VydmljZRgCIAEoCUIIukgFcgMY/wFIAVIHc2VydmljZYgBARIpCglz'
    'ZXJ2ZXJfaWQYAyABKANCB7pIBCICIABIAlIIc2VydmVySWSIAQESIwoKdGltZXN0YW1wcxgEIA'
    'EoCEgDUgp0aW1lc3RhbXBziAEBEhsKBmZvbGxvdxgFIAEoCEgEUgZmb2xsb3eIAQESJAoEdGFp'
    'bBgGIAEoA0ILukgIIgYYoI0GKAFIBVIEdGFpbIgBAUINCgtfY29tcG9zZV9pZEIKCghfc2Vydm'
    'ljZUIMCgpfc2VydmVyX2lkQg0KC190aW1lc3RhbXBzQgkKB19mb2xsb3dCBwoFX3RhaWw=');

@$core.Deprecated('Use resourceTargetRequestDescriptor instead')
const ResourceTargetRequest$json = {
  '1': 'ResourceTargetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `ResourceTargetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resourceTargetRequestDescriptor =
    $convert.base64Decode(
        'ChVSZXNvdXJjZVRhcmdldFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use cancelDeploymentRequestDescriptor instead')
const CancelDeploymentRequest$json = {
  '1': 'CancelDeploymentRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `CancelDeploymentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cancelDeploymentRequestDescriptor =
    $convert.base64Decode(
        'ChdDYW5jZWxEZXBsb3ltZW50UmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use deploymentEventDescriptor instead')
const DeploymentEvent$json = {
  '1': 'DeploymentEvent',
  '2': [
    {'1': 'type', '3': 1, '4': 1, '5': 9, '10': 'type'},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 3, '10': 'timestamp'},
    {'1': 'payload', '3': 3, '4': 1, '5': 9, '10': 'payload'},
    {'1': 'error', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
  ],
  '8': [
    {'1': '_error'},
  ],
};

/// Descriptor for `DeploymentEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deploymentEventDescriptor = $convert.base64Decode(
    'Cg9EZXBsb3ltZW50RXZlbnQSEgoEdHlwZRgBIAEoCVIEdHlwZRIcCgl0aW1lc3RhbXAYAiABKA'
    'NSCXRpbWVzdGFtcBIYCgdwYXlsb2FkGAMgASgJUgdwYXlsb2FkEhkKBWVycm9yGAQgASgJSABS'
    'BWVycm9yiAEBQggKBl9lcnJvcg==');

@$core.Deprecated('Use logExportDescriptor instead')
const LogExport$json = {
  '1': 'LogExport',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
    {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
  ],
};

/// Descriptor for `LogExport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logExportDescriptor = $convert.base64Decode(
    'CglMb2dFeHBvcnQSGAoHY29udGVudBgBIAEoCVIHY29udGVudBIbCglmaWxlX25hbWUYAiABKA'
    'lSCGZpbGVOYW1l');

@$core.Deprecated('Use containerListDescriptor instead')
const ContainerList$json = {
  '1': 'ContainerList',
  '2': [
    {
      '1': 'containers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Container',
      '10': 'containers'
    },
  ],
};

/// Descriptor for `ContainerList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List containerListDescriptor = $convert.base64Decode(
    'Cg1Db250YWluZXJMaXN0EjsKCmNvbnRhaW5lcnMYASADKAsyGy5vcGVub3hpZGUuYXBpLnYxLk'
    'NvbnRhaW5lclIKY29udGFpbmVycw==');

@$core.Deprecated('Use containerDescriptor instead')
const Container$json = {
  '1': 'Container',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'image', '3': 3, '4': 1, '5': 9, '10': 'image'},
    {'1': 'state', '3': 4, '4': 1, '5': 9, '10': 'state'},
    {'1': 'status', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'status', '17': true},
  ],
  '8': [
    {'1': '_status'},
  ],
};

/// Descriptor for `Container`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List containerDescriptor = $convert.base64Decode(
    'CglDb250YWluZXISDgoCaWQYASABKAlSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSFAoFaW1hZ2'
    'UYAyABKAlSBWltYWdlEhQKBXN0YXRlGAQgASgJUgVzdGF0ZRIbCgZzdGF0dXMYBSABKAlIAFIG'
    'c3RhdHVziAEBQgkKB19zdGF0dXM=');

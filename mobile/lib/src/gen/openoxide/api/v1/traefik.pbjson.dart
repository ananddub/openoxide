// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/traefik.proto.

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

@$core.Deprecated('Use traefikFileNodeTypeDescriptor instead')
const TraefikFileNodeType$json = {
  '1': 'TraefikFileNodeType',
  '2': [
    {'1': 'TRAEFIK_FILE_NODE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'TRAEFIK_FILE_NODE_TYPE_FILE', '2': 1},
    {'1': 'TRAEFIK_FILE_NODE_TYPE_DIRECTORY', '2': 2},
  ],
};

/// Descriptor for `TraefikFileNodeType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List traefikFileNodeTypeDescriptor = $convert.base64Decode(
    'ChNUcmFlZmlrRmlsZU5vZGVUeXBlEiYKIlRSQUVGSUtfRklMRV9OT0RFX1RZUEVfVU5TUEVDSU'
    'ZJRUQQABIfChtUUkFFRklLX0ZJTEVfTk9ERV9UWVBFX0ZJTEUQARIkCiBUUkFFRklLX0ZJTEVf'
    'Tk9ERV9UWVBFX0RJUkVDVE9SWRAC');

@$core.Deprecated('Use traefikMiddlewareKindDescriptor instead')
const TraefikMiddlewareKind$json = {
  '1': 'TraefikMiddlewareKind',
  '2': [
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_UNSPECIFIED', '2': 0},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_STRIP_PREFIX', '2': 1},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_ADD_PREFIX', '2': 2},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_REDIRECT_SCHEME', '2': 3},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_COMPRESS', '2': 4},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_RATE_LIMIT', '2': 5},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_IP_ALLOW_LIST', '2': 6},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_REQUEST_HEADERS', '2': 7},
    {'1': 'TRAEFIK_MIDDLEWARE_KIND_RESPONSE_HEADERS', '2': 8},
  ],
};

/// Descriptor for `TraefikMiddlewareKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List traefikMiddlewareKindDescriptor = $convert.base64Decode(
    'ChVUcmFlZmlrTWlkZGxld2FyZUtpbmQSJwojVFJBRUZJS19NSURETEVXQVJFX0tJTkRfVU5TUE'
    'VDSUZJRUQQABIoCiRUUkFFRklLX01JRERMRVdBUkVfS0lORF9TVFJJUF9QUkVGSVgQARImCiJU'
    'UkFFRklLX01JRERMRVdBUkVfS0lORF9BRERfUFJFRklYEAISKwonVFJBRUZJS19NSURETEVXQV'
    'JFX0tJTkRfUkVESVJFQ1RfU0NIRU1FEAMSJAogVFJBRUZJS19NSURETEVXQVJFX0tJTkRfQ09N'
    'UFJFU1MQBBImCiJUUkFFRklLX01JRERMRVdBUkVfS0lORF9SQVRFX0xJTUlUEAUSKQolVFJBRU'
    'ZJS19NSURETEVXQVJFX0tJTkRfSVBfQUxMT1dfTElTVBAGEisKJ1RSQUVGSUtfTUlERExFV0FS'
    'RV9LSU5EX1JFUVVFU1RfSEVBREVSUxAHEiwKKFRSQUVGSUtfTUlERExFV0FSRV9LSU5EX1JFU1'
    'BPTlNFX0hFQURFUlMQCA==');

@$core.Deprecated('Use traefikFileNodeDescriptor instead')
const TraefikFileNode$json = {
  '1': 'TraefikFileNode',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'relative_path', '3': 2, '4': 1, '5': 9, '10': 'relativePath'},
    {'1': 'size', '3': 3, '4': 1, '5': 4, '10': 'size'},
    {'1': 'is_readonly', '3': 4, '4': 1, '5': 8, '10': 'isReadonly'},
    {'1': 'modified_at', '3': 5, '4': 1, '5': 4, '10': 'modifiedAt'},
  ],
};

/// Descriptor for `TraefikFileNode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikFileNodeDescriptor = $convert.base64Decode(
    'Cg9UcmFlZmlrRmlsZU5vZGUSEgoEbmFtZRgBIAEoCVIEbmFtZRIjCg1yZWxhdGl2ZV9wYXRoGA'
    'IgASgJUgxyZWxhdGl2ZVBhdGgSEgoEc2l6ZRgDIAEoBFIEc2l6ZRIfCgtpc19yZWFkb25seRgE'
    'IAEoCFIKaXNSZWFkb25seRIfCgttb2RpZmllZF9hdBgFIAEoBFIKbW9kaWZpZWRBdA==');

@$core.Deprecated('Use traefikFileTreeNodeDescriptor instead')
const TraefikFileTreeNode$json = {
  '1': 'TraefikFileTreeNode',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'relative_path', '3': 2, '4': 1, '5': 9, '10': 'relativePath'},
    {
      '1': 'node_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.TraefikFileNodeType',
      '10': 'nodeType'
    },
    {'1': 'size', '3': 4, '4': 1, '5': 4, '10': 'size'},
    {'1': 'is_readonly', '3': 5, '4': 1, '5': 8, '10': 'isReadonly'},
    {'1': 'modified_at', '3': 6, '4': 1, '5': 4, '10': 'modifiedAt'},
    {
      '1': 'children',
      '3': 7,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikFileTreeNode',
      '10': 'children'
    },
  ],
};

/// Descriptor for `TraefikFileTreeNode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikFileTreeNodeDescriptor = $convert.base64Decode(
    'ChNUcmFlZmlrRmlsZVRyZWVOb2RlEhIKBG5hbWUYASABKAlSBG5hbWUSIwoNcmVsYXRpdmVfcG'
    'F0aBgCIAEoCVIMcmVsYXRpdmVQYXRoEkIKCW5vZGVfdHlwZRgDIAEoDjIlLm9wZW5veGlkZS5h'
    'cGkudjEuVHJhZWZpa0ZpbGVOb2RlVHlwZVIIbm9kZVR5cGUSEgoEc2l6ZRgEIAEoBFIEc2l6ZR'
    'IfCgtpc19yZWFkb25seRgFIAEoCFIKaXNSZWFkb25seRIfCgttb2RpZmllZF9hdBgGIAEoBFIK'
    'bW9kaWZpZWRBdBJBCghjaGlsZHJlbhgHIAMoCzIlLm9wZW5veGlkZS5hcGkudjEuVHJhZWZpa0'
    'ZpbGVUcmVlTm9kZVIIY2hpbGRyZW4=');

@$core.Deprecated('Use traefikFileContentDescriptor instead')
const TraefikFileContent$json = {
  '1': 'TraefikFileContent',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
    {'1': 'is_readonly', '3': 3, '4': 1, '5': 8, '10': 'isReadonly'},
  ],
};

/// Descriptor for `TraefikFileContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikFileContentDescriptor = $convert.base64Decode(
    'ChJUcmFlZmlrRmlsZUNvbnRlbnQSEgoEcGF0aBgBIAEoCVIEcGF0aBIYCgdjb250ZW50GAIgAS'
    'gJUgdjb250ZW50Eh8KC2lzX3JlYWRvbmx5GAMgASgIUgppc1JlYWRvbmx5');

@$core.Deprecated('Use traefikHealthDescriptor instead')
const TraefikHealth$json = {
  '1': 'TraefikHealth',
  '2': [
    {'1': 'is_healthy', '3': 1, '4': 1, '5': 8, '10': 'isHealthy'},
    {'1': 'rawdata_status', '3': 2, '4': 1, '5': 9, '10': 'rawdataStatus'},
    {
      '1': 'configuration_errors',
      '3': 3,
      '4': 3,
      '5': 9,
      '10': 'configurationErrors'
    },
  ],
};

/// Descriptor for `TraefikHealth`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikHealthDescriptor = $convert.base64Decode(
    'Cg1UcmFlZmlrSGVhbHRoEh0KCmlzX2hlYWx0aHkYASABKAhSCWlzSGVhbHRoeRIlCg5yYXdkYX'
    'RhX3N0YXR1cxgCIAEoCVINcmF3ZGF0YVN0YXR1cxIxChRjb25maWd1cmF0aW9uX2Vycm9ycxgD'
    'IAMoCVITY29uZmlndXJhdGlvbkVycm9ycw==');

@$core.Deprecated('Use traefikRequestsStatusDescriptor instead')
const TraefikRequestsStatus$json = {
  '1': 'TraefikRequestsStatus',
  '2': [
    {'1': 'is_active', '3': 1, '4': 1, '5': 8, '10': 'isActive'},
    {'1': 'log_path', '3': 2, '4': 1, '5': 9, '10': 'logPath'},
    {'1': 'cron_expression', '3': 3, '4': 1, '5': 9, '10': 'cronExpression'},
  ],
};

/// Descriptor for `TraefikRequestsStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikRequestsStatusDescriptor = $convert.base64Decode(
    'ChVUcmFlZmlrUmVxdWVzdHNTdGF0dXMSGwoJaXNfYWN0aXZlGAEgASgIUghpc0FjdGl2ZRIZCg'
    'hsb2dfcGF0aBgCIAEoCVIHbG9nUGF0aBInCg9jcm9uX2V4cHJlc3Npb24YAyABKAlSDmNyb25F'
    'eHByZXNzaW9u');

@$core.Deprecated('Use traefikLogEntryDescriptor instead')
const TraefikLogEntry$json = {
  '1': 'TraefikLogEntry',
  '2': [
    {'1': 'timestamp', '3': 1, '4': 1, '5': 9, '10': 'timestamp'},
    {'1': 'client_ip', '3': 2, '4': 1, '5': 9, '10': 'clientIp'},
    {'1': 'method', '3': 3, '4': 1, '5': 9, '10': 'method'},
    {'1': 'path', '3': 4, '4': 1, '5': 9, '10': 'path'},
    {'1': 'status', '3': 5, '4': 1, '5': 13, '10': 'status'},
    {'1': 'duration_ms', '3': 6, '4': 1, '5': 1, '10': 'durationMs'},
    {'1': 'service_name', '3': 7, '4': 1, '5': 9, '10': 'serviceName'},
    {'1': 'router_name', '3': 8, '4': 1, '5': 9, '10': 'routerName'},
  ],
};

/// Descriptor for `TraefikLogEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikLogEntryDescriptor = $convert.base64Decode(
    'Cg9UcmFlZmlrTG9nRW50cnkSHAoJdGltZXN0YW1wGAEgASgJUgl0aW1lc3RhbXASGwoJY2xpZW'
    '50X2lwGAIgASgJUghjbGllbnRJcBIWCgZtZXRob2QYAyABKAlSBm1ldGhvZBISCgRwYXRoGAQg'
    'ASgJUgRwYXRoEhYKBnN0YXR1cxgFIAEoDVIGc3RhdHVzEh8KC2R1cmF0aW9uX21zGAYgASgBUg'
    'pkdXJhdGlvbk1zEiEKDHNlcnZpY2VfbmFtZRgHIAEoCVILc2VydmljZU5hbWUSHwoLcm91dGVy'
    'X25hbWUYCCABKAlSCnJvdXRlck5hbWU=');

@$core.Deprecated('Use traefikVersionDescriptor instead')
const TraefikVersion$json = {
  '1': 'TraefikVersion',
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
    {'1': 'current_image', '3': 2, '4': 1, '5': 9, '10': 'currentImage'},
    {'1': 'desired_version', '3': 3, '4': 1, '5': 9, '10': 'desiredVersion'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikVersion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikVersionDescriptor = $convert.base64Decode(
    'Cg5UcmFlZmlrVmVyc2lvbhIgCglzZXJ2ZXJfaWQYASABKANIAFIIc2VydmVySWSIAQESIwoNY3'
    'VycmVudF9pbWFnZRgCIAEoCVIMY3VycmVudEltYWdlEicKD2Rlc2lyZWRfdmVyc2lvbhgDIAEo'
    'CVIOZGVzaXJlZFZlcnNpb25CDAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use traefikMiddlewareDescriptor instead')
const TraefikMiddleware$json = {
  '1': 'TraefikMiddleware',
  '2': [
    {'1': 'reference', '3': 1, '4': 1, '5': 9, '10': 'reference'},
    {
      '1': 'labels',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikMiddleware.LabelsEntry',
      '10': 'labels'
    },
  ],
  '3': [TraefikMiddleware_LabelsEntry$json],
};

@$core.Deprecated('Use traefikMiddlewareDescriptor instead')
const TraefikMiddleware_LabelsEntry$json = {
  '1': 'LabelsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `TraefikMiddleware`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikMiddlewareDescriptor = $convert.base64Decode(
    'ChFUcmFlZmlrTWlkZGxld2FyZRIcCglyZWZlcmVuY2UYASABKAlSCXJlZmVyZW5jZRJHCgZsYW'
    'JlbHMYAiADKAsyLy5vcGVub3hpZGUuYXBpLnYxLlRyYWVmaWtNaWRkbGV3YXJlLkxhYmVsc0Vu'
    'dHJ5UgZsYWJlbHMaOQoLTGFiZWxzRW50cnkSEAoDa2V5GAEgASgJUgNrZXkSFAoFdmFsdWUYAi'
    'ABKAlSBXZhbHVlOgI4AQ==');

@$core.Deprecated('Use traefikListFilesRequestDescriptor instead')
const TraefikListFilesRequest$json = {
  '1': 'TraefikListFilesRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikListFilesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikListFilesRequestDescriptor =
    $convert.base64Decode(
        'ChdUcmFlZmlrTGlzdEZpbGVzUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIABIAF'
        'IIc2VydmVySWSIAQFCDAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use traefikListFilesResponseDescriptor instead')
const TraefikListFilesResponse$json = {
  '1': 'TraefikListFilesResponse',
  '2': [
    {
      '1': 'files',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikFileNode',
      '10': 'files'
    },
  ],
};

/// Descriptor for `TraefikListFilesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikListFilesResponseDescriptor =
    $convert.base64Decode(
        'ChhUcmFlZmlrTGlzdEZpbGVzUmVzcG9uc2USNwoFZmlsZXMYASADKAsyIS5vcGVub3hpZGUuYX'
        'BpLnYxLlRyYWVmaWtGaWxlTm9kZVIFZmlsZXM=');

@$core.Deprecated('Use traefikGetFileTreeRequestDescriptor instead')
const TraefikGetFileTreeRequest$json = {
  '1': 'TraefikGetFileTreeRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikGetFileTreeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetFileTreeRequestDescriptor =
    $convert.base64Decode(
        'ChlUcmFlZmlrR2V0RmlsZVRyZWVSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
        'gAUghzZXJ2ZXJJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use traefikGetFileTreeResponseDescriptor instead')
const TraefikGetFileTreeResponse$json = {
  '1': 'TraefikGetFileTreeResponse',
  '2': [
    {
      '1': 'nodes',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikFileTreeNode',
      '10': 'nodes'
    },
  ],
};

/// Descriptor for `TraefikGetFileTreeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetFileTreeResponseDescriptor =
    $convert.base64Decode(
        'ChpUcmFlZmlrR2V0RmlsZVRyZWVSZXNwb25zZRI7CgVub2RlcxgBIAMoCzIlLm9wZW5veGlkZS'
        '5hcGkudjEuVHJhZWZpa0ZpbGVUcmVlTm9kZVIFbm9kZXM=');

@$core.Deprecated('Use traefikGetFileContentRequestDescriptor instead')
const TraefikGetFileContentRequest$json = {
  '1': 'TraefikGetFileContentRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'path', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'path'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikGetFileContentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetFileContentRequestDescriptor =
    $convert.base64Decode(
        'ChxUcmFlZmlrR2V0RmlsZUNvbnRlbnRSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIg'
        'IgAEgAUghzZXJ2ZXJJZIgBARIbCgRwYXRoGAIgASgJQge6SARyAhABUgRwYXRoQgwKCl9zZXJ2'
        'ZXJfaWQ=');

@$core.Deprecated('Use traefikGetFileContentResponseDescriptor instead')
const TraefikGetFileContentResponse$json = {
  '1': 'TraefikGetFileContentResponse',
  '2': [
    {
      '1': 'file',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikFileContent',
      '10': 'file'
    },
  ],
};

/// Descriptor for `TraefikGetFileContentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetFileContentResponseDescriptor =
    $convert.base64Decode(
        'Ch1UcmFlZmlrR2V0RmlsZUNvbnRlbnRSZXNwb25zZRI4CgRmaWxlGAEgASgLMiQub3Blbm94aW'
        'RlLmFwaS52MS5UcmFlZmlrRmlsZUNvbnRlbnRSBGZpbGU=');

@$core.Deprecated('Use traefikUpdateFileContentRequestDescriptor instead')
const TraefikUpdateFileContentRequest$json = {
  '1': 'TraefikUpdateFileContentRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'path', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'path'},
    {'1': 'content', '3': 3, '4': 1, '5': 9, '10': 'content'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikUpdateFileContentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikUpdateFileContentRequestDescriptor =
    $convert.base64Decode(
        'Ch9UcmFlZmlrVXBkYXRlRmlsZUNvbnRlbnRSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHuk'
        'gEIgIgAEgAUghzZXJ2ZXJJZIgBARIbCgRwYXRoGAIgASgJQge6SARyAhABUgRwYXRoEhgKB2Nv'
        'bnRlbnQYAyABKAlSB2NvbnRlbnRCDAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use traefikHealthRequestDescriptor instead')
const TraefikHealthRequest$json = {
  '1': 'TraefikHealthRequest',
};

/// Descriptor for `TraefikHealthRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikHealthRequestDescriptor =
    $convert.base64Decode('ChRUcmFlZmlrSGVhbHRoUmVxdWVzdA==');

@$core.Deprecated('Use traefikHealthResponseDescriptor instead')
const TraefikHealthResponse$json = {
  '1': 'TraefikHealthResponse',
  '2': [
    {
      '1': 'health',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikHealth',
      '10': 'health'
    },
  ],
};

/// Descriptor for `TraefikHealthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikHealthResponseDescriptor = $convert.base64Decode(
    'ChVUcmFlZmlrSGVhbHRoUmVzcG9uc2USNwoGaGVhbHRoGAEgASgLMh8ub3Blbm94aWRlLmFwaS'
    '52MS5UcmFlZmlrSGVhbHRoUgZoZWFsdGg=');

@$core.Deprecated('Use traefikGetRequestStatusRequestDescriptor instead')
const TraefikGetRequestStatusRequest$json = {
  '1': 'TraefikGetRequestStatusRequest',
};

/// Descriptor for `TraefikGetRequestStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetRequestStatusRequestDescriptor =
    $convert.base64Decode('Ch5UcmFlZmlrR2V0UmVxdWVzdFN0YXR1c1JlcXVlc3Q=');

@$core.Deprecated('Use traefikGetRequestStatusResponseDescriptor instead')
const TraefikGetRequestStatusResponse$json = {
  '1': 'TraefikGetRequestStatusResponse',
  '2': [
    {
      '1': 'status',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikRequestsStatus',
      '10': 'status'
    },
  ],
};

/// Descriptor for `TraefikGetRequestStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetRequestStatusResponseDescriptor =
    $convert.base64Decode(
        'Ch9UcmFlZmlrR2V0UmVxdWVzdFN0YXR1c1Jlc3BvbnNlEj8KBnN0YXR1cxgBIAEoCzInLm9wZW'
        '5veGlkZS5hcGkudjEuVHJhZWZpa1JlcXVlc3RzU3RhdHVzUgZzdGF0dXM=');

@$core.Deprecated('Use traefikToggleRequestsRequestDescriptor instead')
const TraefikToggleRequestsRequest$json = {
  '1': 'TraefikToggleRequestsRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'enable', '3': 2, '4': 1, '5': 8, '10': 'enable'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikToggleRequestsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikToggleRequestsRequestDescriptor =
    $convert.base64Decode(
        'ChxUcmFlZmlrVG9nZ2xlUmVxdWVzdHNSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIg'
        'IgAEgAUghzZXJ2ZXJJZIgBARIWCgZlbmFibGUYAiABKAhSBmVuYWJsZUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use traefikToggleRequestsResponseDescriptor instead')
const TraefikToggleRequestsResponse$json = {
  '1': 'TraefikToggleRequestsResponse',
  '2': [
    {'1': 'active', '3': 1, '4': 1, '5': 8, '10': 'active'},
  ],
};

/// Descriptor for `TraefikToggleRequestsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikToggleRequestsResponseDescriptor =
    $convert.base64Decode(
        'Ch1UcmFlZmlrVG9nZ2xlUmVxdWVzdHNSZXNwb25zZRIWCgZhY3RpdmUYASABKAhSBmFjdGl2ZQ'
        '==');

@$core.Deprecated('Use traefikGetRequestLogsRequestDescriptor instead')
const TraefikGetRequestLogsRequest$json = {
  '1': 'TraefikGetRequestLogsRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'page',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'page',
      '17': true
    },
    {
      '1': 'page_size',
      '3': 3,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 2,
      '10': 'pageSize',
      '17': true
    },
    {
      '1': 'search',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'search',
      '17': true
    },
    {
      '1': 'status',
      '3': 5,
      '4': 1,
      '5': 13,
      '9': 4,
      '10': 'status',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_id'},
    {'1': '_page'},
    {'1': '_page_size'},
    {'1': '_search'},
    {'1': '_status'},
  ],
};

/// Descriptor for `TraefikGetRequestLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetRequestLogsRequestDescriptor = $convert.base64Decode(
    'ChxUcmFlZmlrR2V0UmVxdWVzdExvZ3NSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIg'
    'IgAEgAUghzZXJ2ZXJJZIgBARIgCgRwYWdlGAIgASgDQge6SAQiAigBSAFSBHBhZ2WIAQESKwoJ'
    'cGFnZV9zaXplGAMgASgDQgm6SAYiBBhkKAFIAlIIcGFnZVNpemWIAQESJQoGc2VhcmNoGAQgAS'
    'gJQgi6SAVyAxjoB0gDUgZzZWFyY2iIAQESGwoGc3RhdHVzGAUgASgNSARSBnN0YXR1c4gBAUIM'
    'Cgpfc2VydmVyX2lkQgcKBV9wYWdlQgwKCl9wYWdlX3NpemVCCQoHX3NlYXJjaEIJCgdfc3RhdH'
    'Vz');

@$core.Deprecated('Use traefikGetRequestLogsResponseDescriptor instead')
const TraefikGetRequestLogsResponse$json = {
  '1': 'TraefikGetRequestLogsResponse',
  '2': [
    {
      '1': 'entries',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikLogEntry',
      '10': 'entries'
    },
    {'1': 'total_count', '3': 2, '4': 1, '5': 3, '10': 'totalCount'},
    {'1': 'page', '3': 3, '4': 1, '5': 3, '10': 'page'},
    {'1': 'page_size', '3': 4, '4': 1, '5': 3, '10': 'pageSize'},
  ],
};

/// Descriptor for `TraefikGetRequestLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetRequestLogsResponseDescriptor = $convert.base64Decode(
    'Ch1UcmFlZmlrR2V0UmVxdWVzdExvZ3NSZXNwb25zZRI7CgdlbnRyaWVzGAEgAygLMiEub3Blbm'
    '94aWRlLmFwaS52MS5UcmFlZmlrTG9nRW50cnlSB2VudHJpZXMSHwoLdG90YWxfY291bnQYAiAB'
    'KANSCnRvdGFsQ291bnQSEgoEcGFnZRgDIAEoA1IEcGFnZRIbCglwYWdlX3NpemUYBCABKANSCH'
    'BhZ2VTaXpl');

@$core.Deprecated('Use traefikGetVersionRequestDescriptor instead')
const TraefikGetVersionRequest$json = {
  '1': 'TraefikGetVersionRequest',
};

/// Descriptor for `TraefikGetVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetVersionRequestDescriptor =
    $convert.base64Decode('ChhUcmFlZmlrR2V0VmVyc2lvblJlcXVlc3Q=');

@$core.Deprecated('Use traefikGetVersionResponseDescriptor instead')
const TraefikGetVersionResponse$json = {
  '1': 'TraefikGetVersionResponse',
  '2': [
    {
      '1': 'version',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikVersion',
      '10': 'version'
    },
  ],
};

/// Descriptor for `TraefikGetVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikGetVersionResponseDescriptor =
    $convert.base64Decode(
        'ChlUcmFlZmlrR2V0VmVyc2lvblJlc3BvbnNlEjoKB3ZlcnNpb24YASABKAsyIC5vcGVub3hpZG'
        'UuYXBpLnYxLlRyYWVmaWtWZXJzaW9uUgd2ZXJzaW9u');

@$core.Deprecated('Use traefikUpdateVersionRequestDescriptor instead')
const TraefikUpdateVersionRequest$json = {
  '1': 'TraefikUpdateVersionRequest',
  '2': [
    {
      '1': 'server_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {'1': 'version', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'version'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `TraefikUpdateVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikUpdateVersionRequestDescriptor =
    $convert.base64Decode(
        'ChtUcmFlZmlrVXBkYXRlVmVyc2lvblJlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAi'
        'AASABSCHNlcnZlcklkiAEBEiEKB3ZlcnNpb24YAiABKAlCB7pIBHICEAFSB3ZlcnNpb25CDAoK'
        'X3NlcnZlcl9pZA==');

@$core.Deprecated('Use traefikUpdateVersionResponseDescriptor instead')
const TraefikUpdateVersionResponse$json = {
  '1': 'TraefikUpdateVersionResponse',
  '2': [
    {
      '1': 'version',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikVersion',
      '10': 'version'
    },
  ],
};

/// Descriptor for `TraefikUpdateVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikUpdateVersionResponseDescriptor =
    $convert.base64Decode(
        'ChxUcmFlZmlrVXBkYXRlVmVyc2lvblJlc3BvbnNlEjoKB3ZlcnNpb24YASABKAsyIC5vcGVub3'
        'hpZGUuYXBpLnYxLlRyYWVmaWtWZXJzaW9uUgd2ZXJzaW9u');

@$core.Deprecated('Use traefikRenderMiddlewareRequestDescriptor instead')
const TraefikRenderMiddlewareRequest$json = {
  '1': 'TraefikRenderMiddlewareRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.TraefikMiddlewareKind',
      '8': {},
      '10': 'kind'
    },
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'values',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikRenderMiddlewareRequest.ValuesEntry',
      '10': 'values'
    },
    {'1': 'list', '3': 4, '4': 3, '5': 9, '10': 'list'},
  ],
  '3': [TraefikRenderMiddlewareRequest_ValuesEntry$json],
};

@$core.Deprecated('Use traefikRenderMiddlewareRequestDescriptor instead')
const TraefikRenderMiddlewareRequest_ValuesEntry$json = {
  '1': 'ValuesEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `TraefikRenderMiddlewareRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikRenderMiddlewareRequestDescriptor = $convert.base64Decode(
    'Ch5UcmFlZmlrUmVuZGVyTWlkZGxld2FyZVJlcXVlc3QSRwoEa2luZBgBIAEoDjInLm9wZW5veG'
    'lkZS5hcGkudjEuVHJhZWZpa01pZGRsZXdhcmVLaW5kQgq6SAeCAQQQASAAUgRraW5kEh4KBG5h'
    'bWUYAiABKAlCCrpIB3IFEAEY/wFSBG5hbWUSVAoGdmFsdWVzGAMgAygLMjwub3Blbm94aWRlLm'
    'FwaS52MS5UcmFlZmlrUmVuZGVyTWlkZGxld2FyZVJlcXVlc3QuVmFsdWVzRW50cnlSBnZhbHVl'
    'cxISCgRsaXN0GAQgAygJUgRsaXN0GjkKC1ZhbHVlc0VudHJ5EhAKA2tleRgBIAEoCVIDa2V5Eh'
    'QKBXZhbHVlGAIgASgJUgV2YWx1ZToCOAE=');

@$core.Deprecated('Use traefikRenderMiddlewareResponseDescriptor instead')
const TraefikRenderMiddlewareResponse$json = {
  '1': 'TraefikRenderMiddlewareResponse',
  '2': [
    {
      '1': 'middleware',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TraefikMiddleware',
      '10': 'middleware'
    },
  ],
};

/// Descriptor for `TraefikRenderMiddlewareResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traefikRenderMiddlewareResponseDescriptor =
    $convert.base64Decode(
        'Ch9UcmFlZmlrUmVuZGVyTWlkZGxld2FyZVJlc3BvbnNlEkMKCm1pZGRsZXdhcmUYASABKAsyIy'
        '5vcGVub3hpZGUuYXBpLnYxLlRyYWVmaWtNaWRkbGV3YXJlUgptaWRkbGV3YXJl');

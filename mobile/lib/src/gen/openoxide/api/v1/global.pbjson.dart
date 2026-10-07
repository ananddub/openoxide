// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/global.proto.

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

@$core.Deprecated('Use globalContainerActionDescriptor instead')
const GlobalContainerAction$json = {
  '1': 'GlobalContainerAction',
  '2': [
    {'1': 'GLOBAL_CONTAINER_ACTION_UNSPECIFIED', '2': 0},
    {'1': 'GLOBAL_CONTAINER_ACTION_START', '2': 1},
    {'1': 'GLOBAL_CONTAINER_ACTION_STOP', '2': 2},
    {'1': 'GLOBAL_CONTAINER_ACTION_RESTART', '2': 3},
    {'1': 'GLOBAL_CONTAINER_ACTION_KILL', '2': 4},
    {'1': 'GLOBAL_CONTAINER_ACTION_PAUSE', '2': 5},
    {'1': 'GLOBAL_CONTAINER_ACTION_UNPAUSE', '2': 6},
  ],
};

/// Descriptor for `GlobalContainerAction`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List globalContainerActionDescriptor = $convert.base64Decode(
    'ChVHbG9iYWxDb250YWluZXJBY3Rpb24SJwojR0xPQkFMX0NPTlRBSU5FUl9BQ1RJT05fVU5TUE'
    'VDSUZJRUQQABIhCh1HTE9CQUxfQ09OVEFJTkVSX0FDVElPTl9TVEFSVBABEiAKHEdMT0JBTF9D'
    'T05UQUlORVJfQUNUSU9OX1NUT1AQAhIjCh9HTE9CQUxfQ09OVEFJTkVSX0FDVElPTl9SRVNUQV'
    'JUEAMSIAocR0xPQkFMX0NPTlRBSU5FUl9BQ1RJT05fS0lMTBAEEiEKHUdMT0JBTF9DT05UQUlO'
    'RVJfQUNUSU9OX1BBVVNFEAUSIwofR0xPQkFMX0NPTlRBSU5FUl9BQ1RJT05fVU5QQVVTRRAG');

@$core.Deprecated('Use globalContainerDescriptor instead')
const GlobalContainer$json = {
  '1': 'GlobalContainer',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'names', '3': 2, '4': 1, '5': 9, '10': 'names'},
    {'1': 'image', '3': 3, '4': 1, '5': 9, '10': 'image'},
    {'1': 'command', '3': 4, '4': 1, '5': 9, '10': 'command'},
    {'1': 'created_at', '3': 5, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'running_for', '3': 6, '4': 1, '5': 9, '10': 'runningFor'},
    {'1': 'ports', '3': 7, '4': 1, '5': 9, '10': 'ports'},
    {'1': 'status', '3': 8, '4': 1, '5': 9, '10': 'status'},
    {'1': 'state', '3': 9, '4': 1, '5': 9, '10': 'state'},
    {'1': 'size', '3': 10, '4': 1, '5': 9, '10': 'size'},
    {'1': 'labels', '3': 11, '4': 1, '5': 9, '10': 'labels'},
    {'1': 'mounts', '3': 12, '4': 1, '5': 9, '10': 'mounts'},
    {'1': 'networks', '3': 13, '4': 1, '5': 9, '10': 'networks'},
  ],
};

/// Descriptor for `GlobalContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalContainerDescriptor = $convert.base64Decode(
    'Cg9HbG9iYWxDb250YWluZXISDgoCaWQYASABKAlSAmlkEhQKBW5hbWVzGAIgASgJUgVuYW1lcx'
    'IUCgVpbWFnZRgDIAEoCVIFaW1hZ2USGAoHY29tbWFuZBgEIAEoCVIHY29tbWFuZBIdCgpjcmVh'
    'dGVkX2F0GAUgASgJUgljcmVhdGVkQXQSHwoLcnVubmluZ19mb3IYBiABKAlSCnJ1bm5pbmdGb3'
    'ISFAoFcG9ydHMYByABKAlSBXBvcnRzEhYKBnN0YXR1cxgIIAEoCVIGc3RhdHVzEhQKBXN0YXRl'
    'GAkgASgJUgVzdGF0ZRISCgRzaXplGAogASgJUgRzaXplEhYKBmxhYmVscxgLIAEoCVIGbGFiZW'
    'xzEhYKBm1vdW50cxgMIAEoCVIGbW91bnRzEhoKCG5ldHdvcmtzGA0gASgJUghuZXR3b3Jrcw==');

@$core.Deprecated('Use globalImageDescriptor instead')
const GlobalImage$json = {
  '1': 'GlobalImage',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'repository', '3': 2, '4': 1, '5': 9, '10': 'repository'},
    {'1': 'tag', '3': 3, '4': 1, '5': 9, '10': 'tag'},
    {'1': 'digest', '3': 4, '4': 1, '5': 9, '10': 'digest'},
    {'1': 'created_at', '3': 5, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'created_since', '3': 6, '4': 1, '5': 9, '10': 'createdSince'},
    {'1': 'size', '3': 7, '4': 1, '5': 9, '10': 'size'},
    {'1': 'shared_size', '3': 8, '4': 1, '5': 9, '10': 'sharedSize'},
    {'1': 'unique_size', '3': 9, '4': 1, '5': 9, '10': 'uniqueSize'},
    {'1': 'containers', '3': 10, '4': 1, '5': 9, '10': 'containers'},
  ],
};

/// Descriptor for `GlobalImage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalImageDescriptor = $convert.base64Decode(
    'CgtHbG9iYWxJbWFnZRIOCgJpZBgBIAEoCVICaWQSHgoKcmVwb3NpdG9yeRgCIAEoCVIKcmVwb3'
    'NpdG9yeRIQCgN0YWcYAyABKAlSA3RhZxIWCgZkaWdlc3QYBCABKAlSBmRpZ2VzdBIdCgpjcmVh'
    'dGVkX2F0GAUgASgJUgljcmVhdGVkQXQSIwoNY3JlYXRlZF9zaW5jZRgGIAEoCVIMY3JlYXRlZF'
    'NpbmNlEhIKBHNpemUYByABKAlSBHNpemUSHwoLc2hhcmVkX3NpemUYCCABKAlSCnNoYXJlZFNp'
    'emUSHwoLdW5pcXVlX3NpemUYCSABKAlSCnVuaXF1ZVNpemUSHgoKY29udGFpbmVycxgKIAEoCV'
    'IKY29udGFpbmVycw==');

@$core.Deprecated('Use globalNetworkDescriptor instead')
const GlobalNetwork$json = {
  '1': 'GlobalNetwork',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'driver', '3': 3, '4': 1, '5': 9, '10': 'driver'},
    {'1': 'scope', '3': 4, '4': 1, '5': 9, '10': 'scope'},
    {'1': 'ipv6', '3': 5, '4': 1, '5': 9, '10': 'ipv6'},
    {'1': 'internal', '3': 6, '4': 1, '5': 9, '10': 'internal'},
    {'1': 'labels', '3': 7, '4': 1, '5': 9, '10': 'labels'},
    {'1': 'created_at', '3': 8, '4': 1, '5': 9, '10': 'createdAt'},
  ],
};

/// Descriptor for `GlobalNetwork`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalNetworkDescriptor = $convert.base64Decode(
    'Cg1HbG9iYWxOZXR3b3JrEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEhYKBm'
    'RyaXZlchgDIAEoCVIGZHJpdmVyEhQKBXNjb3BlGAQgASgJUgVzY29wZRISCgRpcHY2GAUgASgJ'
    'UgRpcHY2EhoKCGludGVybmFsGAYgASgJUghpbnRlcm5hbBIWCgZsYWJlbHMYByABKAlSBmxhYm'
    'VscxIdCgpjcmVhdGVkX2F0GAggASgJUgljcmVhdGVkQXQ=');

@$core.Deprecated('Use globalVolumeDescriptor instead')
const GlobalVolume$json = {
  '1': 'GlobalVolume',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'driver', '3': 2, '4': 1, '5': 9, '10': 'driver'},
    {'1': 'scope', '3': 3, '4': 1, '5': 9, '10': 'scope'},
    {'1': 'mountpoint', '3': 4, '4': 1, '5': 9, '10': 'mountpoint'},
    {'1': 'labels', '3': 5, '4': 1, '5': 9, '10': 'labels'},
    {'1': 'availability', '3': 6, '4': 1, '5': 9, '10': 'availability'},
    {'1': 'group', '3': 7, '4': 1, '5': 9, '10': 'group'},
    {'1': 'links', '3': 8, '4': 1, '5': 9, '10': 'links'},
    {'1': 'size', '3': 9, '4': 1, '5': 9, '10': 'size'},
  ],
};

/// Descriptor for `GlobalVolume`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalVolumeDescriptor = $convert.base64Decode(
    'CgxHbG9iYWxWb2x1bWUSEgoEbmFtZRgBIAEoCVIEbmFtZRIWCgZkcml2ZXIYAiABKAlSBmRyaX'
    'ZlchIUCgVzY29wZRgDIAEoCVIFc2NvcGUSHgoKbW91bnRwb2ludBgEIAEoCVIKbW91bnRwb2lu'
    'dBIWCgZsYWJlbHMYBSABKAlSBmxhYmVscxIiCgxhdmFpbGFiaWxpdHkYBiABKAlSDGF2YWlsYW'
    'JpbGl0eRIUCgVncm91cBgHIAEoCVIFZ3JvdXASFAoFbGlua3MYCCABKAlSBWxpbmtzEhIKBHNp'
    'emUYCSABKAlSBHNpemU=');

@$core.Deprecated('Use globalDockerOutputDescriptor instead')
const GlobalDockerOutput$json = {
  '1': 'GlobalDockerOutput',
  '2': [
    {'1': 'stdout', '3': 1, '4': 1, '5': 9, '10': 'stdout'},
    {'1': 'stderr', '3': 2, '4': 1, '5': 9, '10': 'stderr'},
  ],
};

/// Descriptor for `GlobalDockerOutput`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDockerOutputDescriptor = $convert.base64Decode(
    'ChJHbG9iYWxEb2NrZXJPdXRwdXQSFgoGc3Rkb3V0GAEgASgJUgZzdGRvdXQSFgoGc3RkZXJyGA'
    'IgASgJUgZzdGRlcnI=');

@$core.Deprecated('Use globalUploadFileHeaderDescriptor instead')
const GlobalUploadFileHeader$json = {
  '1': 'GlobalUploadFileHeader',
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
    {'1': 'container_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'destination', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'destination'},
    {'1': 'filename', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'filename'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalUploadFileHeader`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadFileHeaderDescriptor = $convert.base64Decode(
    'ChZHbG9iYWxVcGxvYWRGaWxlSGVhZGVyEikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUg'
    'hzZXJ2ZXJJZIgBARJECgxjb250YWluZXJfaWQYAiABKAlCIbpIHnIcEAEY/wEyFV5bYS16QS1a'
    'MC05Ll86L0ArLV0rJFILY29udGFpbmVySWQSLAoLZGVzdGluYXRpb24YAyABKAlCCrpIB3IFEA'
    'EYgCBSC2Rlc3RpbmF0aW9uEiYKCGZpbGVuYW1lGAQgASgJQgq6SAdyBRABGIAgUghmaWxlbmFt'
    'ZUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalUploadFileRequestDescriptor instead')
const GlobalUploadFileRequest$json = {
  '1': 'GlobalUploadFileRequest',
  '2': [
    {
      '1': 'header',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalUploadFileHeader',
      '9': 0,
      '10': 'header'
    },
    {'1': 'chunk', '3': 2, '4': 1, '5': 12, '8': {}, '9': 0, '10': 'chunk'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `GlobalUploadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadFileRequestDescriptor = $convert.base64Decode(
    'ChdHbG9iYWxVcGxvYWRGaWxlUmVxdWVzdBJCCgZoZWFkZXIYASABKAsyKC5vcGVub3hpZGUuYX'
    'BpLnYxLkdsb2JhbFVwbG9hZEZpbGVIZWFkZXJIAFIGaGVhZGVyEiQKBWNodW5rGAIgASgMQgy6'
    'SAl6BxABGICAgAJIAFIFY2h1bmtCCQoHcGF5bG9hZA==');

@$core.Deprecated('Use globalUploadFileResponseDescriptor instead')
const GlobalUploadFileResponse$json = {
  '1': 'GlobalUploadFileResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'output',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalDockerOutput',
      '10': 'output'
    },
  ],
};

/// Descriptor for `GlobalUploadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadFileResponseDescriptor = $convert.base64Decode(
    'ChhHbG9iYWxVcGxvYWRGaWxlUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2VzcxI8Cg'
    'ZvdXRwdXQYAiABKAsyJC5vcGVub3hpZGUuYXBpLnYxLkdsb2JhbERvY2tlck91dHB1dFIGb3V0'
    'cHV0');

@$core.Deprecated('Use globalDownloadFileRequestDescriptor instead')
const GlobalDownloadFileRequest$json = {
  '1': 'GlobalDownloadFileRequest',
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
    {'1': 'container_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'path', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'path'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalDownloadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDownloadFileRequestDescriptor = $convert.base64Decode(
    'ChlHbG9iYWxEb3dubG9hZEZpbGVSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARJECgxjb250YWluZXJfaWQYAiABKAlCIbpIHnIcEAEY/wEyFV5bYS16'
    'QS1aMC05Ll86L0ArLV0rJFILY29udGFpbmVySWQSHgoEcGF0aBgDIAEoCUIKukgHcgUQARiAIF'
    'IEcGF0aEIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalDownloadFileResponseDescriptor instead')
const GlobalDownloadFileResponse$json = {
  '1': 'GlobalDownloadFileResponse',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
    {'1': 'path', '3': 2, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `GlobalDownloadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDownloadFileResponseDescriptor =
    $convert.base64Decode(
        'ChpHbG9iYWxEb3dubG9hZEZpbGVSZXNwb25zZRIYCgdjb250ZW50GAEgASgMUgdjb250ZW50Eh'
        'IKBHBhdGgYAiABKAlSBHBhdGg=');

@$core.Deprecated('Use globalUploadContainerFileHeaderDescriptor instead')
const GlobalUploadContainerFileHeader$json = {
  '1': 'GlobalUploadContainerFileHeader',
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
    {'1': 'container_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'destination', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'destination'},
    {'1': 'filename', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'filename'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalUploadContainerFileHeader`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadContainerFileHeaderDescriptor = $convert.base64Decode(
    'Ch9HbG9iYWxVcGxvYWRDb250YWluZXJGaWxlSGVhZGVyEikKCXNlcnZlcl9pZBgBIAEoA0IHuk'
    'gEIgIgAEgAUghzZXJ2ZXJJZIgBARJECgxjb250YWluZXJfaWQYAiABKAlCIbpIHnIcEAEY/wEy'
    'FV5bYS16QS1aMC05Ll86L0ArLV0rJFILY29udGFpbmVySWQSLAoLZGVzdGluYXRpb24YAyABKA'
    'lCCrpIB3IFEAEYgCBSC2Rlc3RpbmF0aW9uEiYKCGZpbGVuYW1lGAQgASgJQgq6SAdyBRABGIAg'
    'UghmaWxlbmFtZUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalUploadContainerFileRequestDescriptor instead')
const GlobalUploadContainerFileRequest$json = {
  '1': 'GlobalUploadContainerFileRequest',
  '2': [
    {
      '1': 'header',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalUploadContainerFileHeader',
      '9': 0,
      '10': 'header'
    },
    {'1': 'chunk', '3': 2, '4': 1, '5': 12, '8': {}, '9': 0, '10': 'chunk'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `GlobalUploadContainerFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadContainerFileRequestDescriptor =
    $convert.base64Decode(
        'CiBHbG9iYWxVcGxvYWRDb250YWluZXJGaWxlUmVxdWVzdBJLCgZoZWFkZXIYASABKAsyMS5vcG'
        'Vub3hpZGUuYXBpLnYxLkdsb2JhbFVwbG9hZENvbnRhaW5lckZpbGVIZWFkZXJIAFIGaGVhZGVy'
        'EiQKBWNodW5rGAIgASgMQgy6SAl6BxABGICAgAJIAFIFY2h1bmtCCQoHcGF5bG9hZA==');

@$core.Deprecated('Use globalUploadContainerFileResponseDescriptor instead')
const GlobalUploadContainerFileResponse$json = {
  '1': 'GlobalUploadContainerFileResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'output',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalDockerOutput',
      '10': 'output'
    },
  ],
};

/// Descriptor for `GlobalUploadContainerFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalUploadContainerFileResponseDescriptor =
    $convert.base64Decode(
        'CiFHbG9iYWxVcGxvYWRDb250YWluZXJGaWxlUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3'
        'VjY2VzcxI8CgZvdXRwdXQYAiABKAsyJC5vcGVub3hpZGUuYXBpLnYxLkdsb2JhbERvY2tlck91'
        'dHB1dFIGb3V0cHV0');

@$core.Deprecated('Use globalDownloadContainerFileRequestDescriptor instead')
const GlobalDownloadContainerFileRequest$json = {
  '1': 'GlobalDownloadContainerFileRequest',
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
    {'1': 'container_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'path', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'path'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalDownloadContainerFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDownloadContainerFileRequestDescriptor =
    $convert.base64Decode(
        'CiJHbG9iYWxEb3dubG9hZENvbnRhaW5lckZpbGVSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0'
        'IHukgEIgIgAEgAUghzZXJ2ZXJJZIgBARJECgxjb250YWluZXJfaWQYAiABKAlCIbpIHnIcEAEY'
        '/wEyFV5bYS16QS1aMC05Ll86L0ArLV0rJFILY29udGFpbmVySWQSHgoEcGF0aBgDIAEoCUIKuk'
        'gHcgUQARiAIFIEcGF0aEIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalDownloadContainerFileResponseDescriptor instead')
const GlobalDownloadContainerFileResponse$json = {
  '1': 'GlobalDownloadContainerFileResponse',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
    {'1': 'path', '3': 2, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `GlobalDownloadContainerFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDownloadContainerFileResponseDescriptor =
    $convert.base64Decode(
        'CiNHbG9iYWxEb3dubG9hZENvbnRhaW5lckZpbGVSZXNwb25zZRIYCgdjb250ZW50GAEgASgMUg'
        'djb250ZW50EhIKBHBhdGgYAiABKAlSBHBhdGg=');

@$core.Deprecated('Use globalListContainersRequestDescriptor instead')
const GlobalListContainersRequest$json = {
  '1': 'GlobalListContainersRequest',
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

/// Descriptor for `GlobalListContainersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListContainersRequestDescriptor =
    $convert.base64Decode(
        'ChtHbG9iYWxMaXN0Q29udGFpbmVyc1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAi'
        'AASABSCHNlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalListContainersResponseDescriptor instead')
const GlobalListContainersResponse$json = {
  '1': 'GlobalListContainersResponse',
  '2': [
    {
      '1': 'containers',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalContainer',
      '10': 'containers'
    },
  ],
};

/// Descriptor for `GlobalListContainersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListContainersResponseDescriptor =
    $convert.base64Decode(
        'ChxHbG9iYWxMaXN0Q29udGFpbmVyc1Jlc3BvbnNlEkEKCmNvbnRhaW5lcnMYASADKAsyIS5vcG'
        'Vub3hpZGUuYXBpLnYxLkdsb2JhbENvbnRhaW5lclIKY29udGFpbmVycw==');

@$core.Deprecated('Use globalListImagesRequestDescriptor instead')
const GlobalListImagesRequest$json = {
  '1': 'GlobalListImagesRequest',
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

/// Descriptor for `GlobalListImagesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListImagesRequestDescriptor =
    $convert.base64Decode(
        'ChdHbG9iYWxMaXN0SW1hZ2VzUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIABIAF'
        'IIc2VydmVySWSIAQFCDAoKX3NlcnZlcl9pZA==');

@$core.Deprecated('Use globalListImagesResponseDescriptor instead')
const GlobalListImagesResponse$json = {
  '1': 'GlobalListImagesResponse',
  '2': [
    {
      '1': 'images',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalImage',
      '10': 'images'
    },
  ],
};

/// Descriptor for `GlobalListImagesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListImagesResponseDescriptor =
    $convert.base64Decode(
        'ChhHbG9iYWxMaXN0SW1hZ2VzUmVzcG9uc2USNQoGaW1hZ2VzGAEgAygLMh0ub3Blbm94aWRlLm'
        'FwaS52MS5HbG9iYWxJbWFnZVIGaW1hZ2Vz');

@$core.Deprecated('Use globalListNetworksRequestDescriptor instead')
const GlobalListNetworksRequest$json = {
  '1': 'GlobalListNetworksRequest',
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

/// Descriptor for `GlobalListNetworksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListNetworksRequestDescriptor =
    $convert.base64Decode(
        'ChlHbG9iYWxMaXN0TmV0d29ya3NSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
        'gAUghzZXJ2ZXJJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalListNetworksResponseDescriptor instead')
const GlobalListNetworksResponse$json = {
  '1': 'GlobalListNetworksResponse',
  '2': [
    {
      '1': 'networks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalNetwork',
      '10': 'networks'
    },
  ],
};

/// Descriptor for `GlobalListNetworksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListNetworksResponseDescriptor =
    $convert.base64Decode(
        'ChpHbG9iYWxMaXN0TmV0d29ya3NSZXNwb25zZRI7CghuZXR3b3JrcxgBIAMoCzIfLm9wZW5veG'
        'lkZS5hcGkudjEuR2xvYmFsTmV0d29ya1IIbmV0d29ya3M=');

@$core.Deprecated('Use globalListVolumesRequestDescriptor instead')
const GlobalListVolumesRequest$json = {
  '1': 'GlobalListVolumesRequest',
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

/// Descriptor for `GlobalListVolumesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListVolumesRequestDescriptor =
    $convert.base64Decode(
        'ChhHbG9iYWxMaXN0Vm9sdW1lc1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAiAASA'
        'BSCHNlcnZlcklkiAEBQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalListVolumesResponseDescriptor instead')
const GlobalListVolumesResponse$json = {
  '1': 'GlobalListVolumesResponse',
  '2': [
    {
      '1': 'volumes',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalVolume',
      '10': 'volumes'
    },
  ],
};

/// Descriptor for `GlobalListVolumesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalListVolumesResponseDescriptor =
    $convert.base64Decode(
        'ChlHbG9iYWxMaXN0Vm9sdW1lc1Jlc3BvbnNlEjgKB3ZvbHVtZXMYASADKAsyHi5vcGVub3hpZG'
        'UuYXBpLnYxLkdsb2JhbFZvbHVtZVIHdm9sdW1lcw==');

@$core.Deprecated('Use globalInspectContainerRequestDescriptor instead')
const GlobalInspectContainerRequest$json = {
  '1': 'GlobalInspectContainerRequest',
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
    {'1': 'container_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalInspectContainerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectContainerRequestDescriptor =
    $convert.base64Decode(
        'Ch1HbG9iYWxJbnNwZWN0Q29udGFpbmVyUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBC'
        'ICIABIAFIIc2VydmVySWSIAQESRAoMY29udGFpbmVyX2lkGAIgASgJQiG6SB5yHBABGP8BMhVe'
        'W2EtekEtWjAtOS5fOi9AKy1dKyRSC2NvbnRhaW5lcklkQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalInspectContainerResponseDescriptor instead')
const GlobalInspectContainerResponse$json = {
  '1': 'GlobalInspectContainerResponse',
  '2': [
    {'1': 'data_json', '3': 1, '4': 1, '5': 9, '10': 'dataJson'},
  ],
};

/// Descriptor for `GlobalInspectContainerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectContainerResponseDescriptor =
    $convert.base64Decode(
        'Ch5HbG9iYWxJbnNwZWN0Q29udGFpbmVyUmVzcG9uc2USGwoJZGF0YV9qc29uGAEgASgJUghkYX'
        'RhSnNvbg==');

@$core.Deprecated('Use globalInspectImageRequestDescriptor instead')
const GlobalInspectImageRequest$json = {
  '1': 'GlobalInspectImageRequest',
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
    {'1': 'image_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'imageId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalInspectImageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectImageRequestDescriptor = $convert.base64Decode(
    'ChlHbG9iYWxJbnNwZWN0SW1hZ2VSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUghzZXJ2ZXJJZIgBARI8CghpbWFnZV9pZBgCIAEoCUIhukgechwQARj/ATIVXlthLXpBLVow'
    'LTkuXzovQCstXSskUgdpbWFnZUlkQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalInspectImageResponseDescriptor instead')
const GlobalInspectImageResponse$json = {
  '1': 'GlobalInspectImageResponse',
  '2': [
    {'1': 'data_json', '3': 1, '4': 1, '5': 9, '10': 'dataJson'},
  ],
};

/// Descriptor for `GlobalInspectImageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectImageResponseDescriptor =
    $convert.base64Decode(
        'ChpHbG9iYWxJbnNwZWN0SW1hZ2VSZXNwb25zZRIbCglkYXRhX2pzb24YASABKAlSCGRhdGFKc2'
        '9u');

@$core.Deprecated('Use globalInspectNetworkRequestDescriptor instead')
const GlobalInspectNetworkRequest$json = {
  '1': 'GlobalInspectNetworkRequest',
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
    {'1': 'network_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'networkId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalInspectNetworkRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectNetworkRequestDescriptor =
    $convert.base64Decode(
        'ChtHbG9iYWxJbnNwZWN0TmV0d29ya1JlcXVlc3QSKQoJc2VydmVyX2lkGAEgASgDQge6SAQiAi'
        'AASABSCHNlcnZlcklkiAEBEkAKCm5ldHdvcmtfaWQYAiABKAlCIbpIHnIcEAEY/wEyFV5bYS16'
        'QS1aMC05Ll86L0ArLV0rJFIJbmV0d29ya0lkQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalInspectNetworkResponseDescriptor instead')
const GlobalInspectNetworkResponse$json = {
  '1': 'GlobalInspectNetworkResponse',
  '2': [
    {'1': 'data_json', '3': 1, '4': 1, '5': 9, '10': 'dataJson'},
  ],
};

/// Descriptor for `GlobalInspectNetworkResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectNetworkResponseDescriptor =
    $convert.base64Decode(
        'ChxHbG9iYWxJbnNwZWN0TmV0d29ya1Jlc3BvbnNlEhsKCWRhdGFfanNvbhgBIAEoCVIIZGF0YU'
        'pzb24=');

@$core.Deprecated('Use globalInspectVolumeRequestDescriptor instead')
const GlobalInspectVolumeRequest$json = {
  '1': 'GlobalInspectVolumeRequest',
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
    {'1': 'volume_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'volumeId'},
  ],
  '8': [
    {'1': '_server_id'},
  ],
};

/// Descriptor for `GlobalInspectVolumeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectVolumeRequestDescriptor =
    $convert.base64Decode(
        'ChpHbG9iYWxJbnNwZWN0Vm9sdW1lUmVxdWVzdBIpCglzZXJ2ZXJfaWQYASABKANCB7pIBCICIA'
        'BIAFIIc2VydmVySWSIAQESPgoJdm9sdW1lX2lkGAIgASgJQiG6SB5yHBABGP8BMhVeW2EtekEt'
        'WjAtOS5fOi9AKy1dKyRSCHZvbHVtZUlkQgwKCl9zZXJ2ZXJfaWQ=');

@$core.Deprecated('Use globalInspectVolumeResponseDescriptor instead')
const GlobalInspectVolumeResponse$json = {
  '1': 'GlobalInspectVolumeResponse',
  '2': [
    {'1': 'data_json', '3': 1, '4': 1, '5': 9, '10': 'dataJson'},
  ],
};

/// Descriptor for `GlobalInspectVolumeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalInspectVolumeResponseDescriptor =
    $convert.base64Decode(
        'ChtHbG9iYWxJbnNwZWN0Vm9sdW1lUmVzcG9uc2USGwoJZGF0YV9qc29uGAEgASgJUghkYXRhSn'
        'Nvbg==');

@$core.Deprecated('Use globalContainerActionRequestDescriptor instead')
const GlobalContainerActionRequest$json = {
  '1': 'GlobalContainerActionRequest',
  '2': [
    {'1': 'container_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {
      '1': 'action',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.GlobalContainerAction',
      '8': {},
      '10': 'action'
    },
  ],
};

/// Descriptor for `GlobalContainerActionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalContainerActionRequestDescriptor = $convert.base64Decode(
    'ChxHbG9iYWxDb250YWluZXJBY3Rpb25SZXF1ZXN0EkQKDGNvbnRhaW5lcl9pZBgBIAEoCUIhuk'
    'gechwQARj/ATIVXlthLXpBLVowLTkuXzovQCstXSskUgtjb250YWluZXJJZBJLCgZhY3Rpb24Y'
    'AiABKA4yJy5vcGVub3hpZGUuYXBpLnYxLkdsb2JhbENvbnRhaW5lckFjdGlvbkIKukgHggEEEA'
    'EgAFIGYWN0aW9u');

@$core.Deprecated('Use globalContainerActionResponseDescriptor instead')
const GlobalContainerActionResponse$json = {
  '1': 'GlobalContainerActionResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'output',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalDockerOutput',
      '10': 'output'
    },
  ],
};

/// Descriptor for `GlobalContainerActionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalContainerActionResponseDescriptor =
    $convert.base64Decode(
        'Ch1HbG9iYWxDb250YWluZXJBY3Rpb25SZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZX'
        'NzEjwKBm91dHB1dBgCIAEoCzIkLm9wZW5veGlkZS5hcGkudjEuR2xvYmFsRG9ja2VyT3V0cHV0'
        'UgZvdXRwdXQ=');

@$core.Deprecated('Use globalRemoveContainerRequestDescriptor instead')
const GlobalRemoveContainerRequest$json = {
  '1': 'GlobalRemoveContainerRequest',
  '2': [
    {'1': 'container_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'force', '3': 2, '4': 1, '5': 8, '10': 'force'},
    {'1': 'volumes', '3': 3, '4': 1, '5': 8, '10': 'volumes'},
  ],
};

/// Descriptor for `GlobalRemoveContainerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalRemoveContainerRequestDescriptor =
    $convert.base64Decode(
        'ChxHbG9iYWxSZW1vdmVDb250YWluZXJSZXF1ZXN0EkQKDGNvbnRhaW5lcl9pZBgBIAEoCUIhuk'
        'gechwQARj/ATIVXlthLXpBLVowLTkuXzovQCstXSskUgtjb250YWluZXJJZBIUCgVmb3JjZRgC'
        'IAEoCFIFZm9yY2USGAoHdm9sdW1lcxgDIAEoCFIHdm9sdW1lcw==');

@$core.Deprecated('Use globalDeleteContainerRequestDescriptor instead')
const GlobalDeleteContainerRequest$json = {
  '1': 'GlobalDeleteContainerRequest',
  '2': [
    {'1': 'container_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'containerId'},
    {'1': 'force', '3': 2, '4': 1, '5': 8, '10': 'force'},
    {'1': 'volumes', '3': 3, '4': 1, '5': 8, '10': 'volumes'},
  ],
};

/// Descriptor for `GlobalDeleteContainerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDeleteContainerRequestDescriptor =
    $convert.base64Decode(
        'ChxHbG9iYWxEZWxldGVDb250YWluZXJSZXF1ZXN0EkQKDGNvbnRhaW5lcl9pZBgBIAEoCUIhuk'
        'gechwQARj/ATIVXlthLXpBLVowLTkuXzovQCstXSskUgtjb250YWluZXJJZBIUCgVmb3JjZRgC'
        'IAEoCFIFZm9yY2USGAoHdm9sdW1lcxgDIAEoCFIHdm9sdW1lcw==');

@$core.Deprecated('Use globalDiskUsageRequestDescriptor instead')
const GlobalDiskUsageRequest$json = {
  '1': 'GlobalDiskUsageRequest',
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

/// Descriptor for `GlobalDiskUsageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDiskUsageRequestDescriptor =
    $convert.base64Decode(
        'ChZHbG9iYWxEaXNrVXNhZ2VSZXF1ZXN0EikKCXNlcnZlcl9pZBgBIAEoA0IHukgEIgIgAEgAUg'
        'hzZXJ2ZXJJZIgBAUIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use globalDiskUsageResponseDescriptor instead')
const GlobalDiskUsageResponse$json = {
  '1': 'GlobalDiskUsageResponse',
  '2': [
    {'1': 'data_json', '3': 1, '4': 1, '5': 9, '10': 'dataJson'},
  ],
};

/// Descriptor for `GlobalDiskUsageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalDiskUsageResponseDescriptor =
    $convert.base64Decode(
        'ChdHbG9iYWxEaXNrVXNhZ2VSZXNwb25zZRIbCglkYXRhX2pzb24YASABKAlSCGRhdGFKc29u');

@$core.Deprecated('Use globalPruneRequestDescriptor instead')
const GlobalPruneRequest$json = {
  '1': 'GlobalPruneRequest',
  '2': [
    {'1': 'containers', '3': 1, '4': 1, '5': 8, '10': 'containers'},
    {'1': 'images', '3': 2, '4': 1, '5': 8, '10': 'images'},
    {'1': 'networks', '3': 3, '4': 1, '5': 8, '10': 'networks'},
    {'1': 'volumes', '3': 4, '4': 1, '5': 8, '10': 'volumes'},
  ],
};

/// Descriptor for `GlobalPruneRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalPruneRequestDescriptor = $convert.base64Decode(
    'ChJHbG9iYWxQcnVuZVJlcXVlc3QSHgoKY29udGFpbmVycxgBIAEoCFIKY29udGFpbmVycxIWCg'
    'ZpbWFnZXMYAiABKAhSBmltYWdlcxIaCghuZXR3b3JrcxgDIAEoCFIIbmV0d29ya3MSGAoHdm9s'
    'dW1lcxgEIAEoCFIHdm9sdW1lcw==');

@$core.Deprecated('Use globalPruneResponseDescriptor instead')
const GlobalPruneResponse$json = {
  '1': 'GlobalPruneResponse',
  '2': [
    {
      '1': 'operations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.GlobalDockerOutput',
      '10': 'operations'
    },
  ],
};

/// Descriptor for `GlobalPruneResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List globalPruneResponseDescriptor = $convert.base64Decode(
    'ChNHbG9iYWxQcnVuZVJlc3BvbnNlEkQKCm9wZXJhdGlvbnMYASADKAsyJC5vcGVub3hpZGUuYX'
    'BpLnYxLkdsb2JhbERvY2tlck91dHB1dFIKb3BlcmF0aW9ucw==');

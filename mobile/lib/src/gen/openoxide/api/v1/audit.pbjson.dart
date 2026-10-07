// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/audit.proto.

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

@$core.Deprecated('Use auditLogDescriptor instead')
const AuditLog$json = {
  '1': 'AuditLog',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'user_email', '3': 2, '4': 1, '5': 9, '10': 'userEmail'},
    {'1': 'user_role', '3': 3, '4': 1, '5': 9, '10': 'userRole'},
    {'1': 'action', '3': 4, '4': 1, '5': 9, '10': 'action'},
    {'1': 'resource_type', '3': 5, '4': 1, '5': 9, '10': 'resourceType'},
    {
      '1': 'resource_id',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'resourceId',
      '17': true
    },
    {
      '1': 'resource_name',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'resourceName',
      '17': true
    },
    {
      '1': 'metadata',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'metadata',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 9,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'user_id',
      '3': 10,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'userId',
      '17': true
    },
    {'1': 'created_at', '3': 11, '4': 1, '5': 3, '10': 'createdAt'},
  ],
  '8': [
    {'1': '_resource_id'},
    {'1': '_resource_name'},
    {'1': '_metadata'},
    {'1': '_organization_id'},
    {'1': '_user_id'},
  ],
};

/// Descriptor for `AuditLog`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List auditLogDescriptor = $convert.base64Decode(
    'CghBdWRpdExvZxIOCgJpZBgBIAEoA1ICaWQSHQoKdXNlcl9lbWFpbBgCIAEoCVIJdXNlckVtYW'
    'lsEhsKCXVzZXJfcm9sZRgDIAEoCVIIdXNlclJvbGUSFgoGYWN0aW9uGAQgASgJUgZhY3Rpb24S'
    'IwoNcmVzb3VyY2VfdHlwZRgFIAEoCVIMcmVzb3VyY2VUeXBlEiQKC3Jlc291cmNlX2lkGAYgAS'
    'gJSABSCnJlc291cmNlSWSIAQESKAoNcmVzb3VyY2VfbmFtZRgHIAEoCUgBUgxyZXNvdXJjZU5h'
    'bWWIAQESHwoIbWV0YWRhdGEYCCABKAlIAlIIbWV0YWRhdGGIAQESLAoPb3JnYW5pemF0aW9uX2'
    'lkGAkgASgDSANSDm9yZ2FuaXphdGlvbklkiAEBEhwKB3VzZXJfaWQYCiABKANIBFIGdXNlcklk'
    'iAEBEh0KCmNyZWF0ZWRfYXQYCyABKANSCWNyZWF0ZWRBdEIOCgxfcmVzb3VyY2VfaWRCEAoOX3'
    'Jlc291cmNlX25hbWVCCwoJX21ldGFkYXRhQhIKEF9vcmdhbml6YXRpb25faWRCCgoIX3VzZXJf'
    'aWQ=');

@$core.Deprecated('Use auditListAuditLogsRequestDescriptor instead')
const AuditListAuditLogsRequest$json = {
  '1': 'AuditListAuditLogsRequest',
  '2': [
    {
      '1': 'organization_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'action',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'action',
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
    {'1': '_organization_id'},
    {'1': '_action'},
    {'1': '_limit'},
    {'1': '_offset'},
  ],
};

/// Descriptor for `AuditListAuditLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List auditListAuditLogsRequestDescriptor = $convert.base64Decode(
    'ChlBdWRpdExpc3RBdWRpdExvZ3NSZXF1ZXN0EjUKD29yZ2FuaXphdGlvbl9pZBgBIAEoA0IHuk'
    'gEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBARIlCgZhY3Rpb24YAiABKAlCCLpIBXIDGP8BSAFS'
    'BmFjdGlvbogBARIlCgVsaW1pdBgDIAEoA0IKukgHIgUY9AMoAUgCUgVsaW1pdIgBARIkCgZvZm'
    'ZzZXQYBCABKANCB7pIBCICKABIA1IGb2Zmc2V0iAEBQhIKEF9vcmdhbml6YXRpb25faWRCCQoH'
    'X2FjdGlvbkIICgZfbGltaXRCCQoHX29mZnNldA==');

@$core.Deprecated('Use auditListAuditLogsResponseDescriptor instead')
const AuditListAuditLogsResponse$json = {
  '1': 'AuditListAuditLogsResponse',
  '2': [
    {
      '1': 'logs',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AuditLog',
      '10': 'logs'
    },
  ],
};

/// Descriptor for `AuditListAuditLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List auditListAuditLogsResponseDescriptor =
    $convert.base64Decode(
        'ChpBdWRpdExpc3RBdWRpdExvZ3NSZXNwb25zZRIuCgRsb2dzGAEgAygLMhoub3Blbm94aWRlLm'
        'FwaS52MS5BdWRpdExvZ1IEbG9ncw==');

@$core.Deprecated('Use auditExportAuditLogsRequestDescriptor instead')
const AuditExportAuditLogsRequest$json = {
  '1': 'AuditExportAuditLogsRequest',
  '2': [
    {
      '1': 'action',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'action',
      '17': true
    },
  ],
  '8': [
    {'1': '_action'},
  ],
};

/// Descriptor for `AuditExportAuditLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List auditExportAuditLogsRequestDescriptor =
    $convert.base64Decode(
        'ChtBdWRpdEV4cG9ydEF1ZGl0TG9nc1JlcXVlc3QSJQoGYWN0aW9uGAEgASgJQgi6SAVyAxj/AU'
        'gAUgZhY3Rpb26IAQFCCQoHX2FjdGlvbg==');

@$core.Deprecated('Use auditExportAuditLogsResponseDescriptor instead')
const AuditExportAuditLogsResponse$json = {
  '1': 'AuditExportAuditLogsResponse',
  '2': [
    {'1': 'csv', '3': 1, '4': 1, '5': 9, '10': 'csv'},
  ],
};

/// Descriptor for `AuditExportAuditLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List auditExportAuditLogsResponseDescriptor =
    $convert.base64Decode(
        'ChxBdWRpdEV4cG9ydEF1ZGl0TG9nc1Jlc3BvbnNlEhAKA2NzdhgBIAEoCVIDY3N2');

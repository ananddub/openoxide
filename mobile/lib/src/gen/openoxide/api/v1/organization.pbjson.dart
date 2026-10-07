// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/organization.proto.

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

@$core.Deprecated('Use organizationDescriptor instead')
const Organization$json = {
  '1': 'Organization',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'logo', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'logo', '17': true},
    {'1': 'slug', '3': 4, '4': 1, '5': 9, '10': 'slug'},
    {'1': 'owner_id', '3': 5, '4': 1, '5': 3, '10': 'ownerId'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 7, '4': 1, '5': 3, '10': 'updatedAt'},
    {'1': 'role', '3': 8, '4': 1, '5': 9, '9': 1, '10': 'role', '17': true},
  ],
  '8': [
    {'1': '_logo'},
    {'1': '_role'},
  ],
};

/// Descriptor for `Organization`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List organizationDescriptor = $convert.base64Decode(
    'CgxPcmdhbml6YXRpb24SDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSFwoEbG'
    '9nbxgDIAEoCUgAUgRsb2dviAEBEhIKBHNsdWcYBCABKAlSBHNsdWcSGQoIb3duZXJfaWQYBSAB'
    'KANSB293bmVySWQSHQoKY3JlYXRlZF9hdBgGIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYX'
    'QYByABKANSCXVwZGF0ZWRBdBIXCgRyb2xlGAggASgJSAFSBHJvbGWIAQFCBwoFX2xvZ29CBwoF'
    'X3JvbGU=');

@$core.Deprecated('Use leaveOrganizationRequestDescriptor instead')
const LeaveOrganizationRequest$json = {
  '1': 'LeaveOrganizationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `LeaveOrganizationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List leaveOrganizationRequestDescriptor =
    $convert.base64Decode(
        'ChhMZWF2ZU9yZ2FuaXphdGlvblJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlk');

@$core.Deprecated('Use getOrganizationRequestDescriptor instead')
const GetOrganizationRequest$json = {
  '1': 'GetOrganizationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetOrganizationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getOrganizationRequestDescriptor =
    $convert.base64Decode(
        'ChZHZXRPcmdhbml6YXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use listOrganizationsRequestDescriptor instead')
const ListOrganizationsRequest$json = {
  '1': 'ListOrganizationsRequest',
};

/// Descriptor for `ListOrganizationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listOrganizationsRequestDescriptor =
    $convert.base64Decode('ChhMaXN0T3JnYW5pemF0aW9uc1JlcXVlc3Q=');

@$core.Deprecated('Use listOrganizationsResponseDescriptor instead')
const ListOrganizationsResponse$json = {
  '1': 'ListOrganizationsResponse',
  '2': [
    {
      '1': 'organizations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Organization',
      '10': 'organizations'
    },
  ],
};

/// Descriptor for `ListOrganizationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listOrganizationsResponseDescriptor =
    $convert.base64Decode(
        'ChlMaXN0T3JnYW5pemF0aW9uc1Jlc3BvbnNlEkQKDW9yZ2FuaXphdGlvbnMYASADKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLk9yZ2FuaXphdGlvblINb3JnYW5pemF0aW9ucw==');

@$core.Deprecated('Use createOrganizationRequestDescriptor instead')
const CreateOrganizationRequest$json = {
  '1': 'CreateOrganizationRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'logo', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'logo', '17': true},
    {'1': 'slug', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'slug', '17': true},
  ],
  '8': [
    {'1': '_logo'},
    {'1': '_slug'},
  ],
};

/// Descriptor for `CreateOrganizationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createOrganizationRequestDescriptor = $convert.base64Decode(
    'ChlDcmVhdGVPcmdhbml6YXRpb25SZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG'
    '5hbWUSFwoEbG9nbxgCIAEoCUgAUgRsb2dviAEBEhcKBHNsdWcYAyABKAlIAVIEc2x1Z4gBAUIH'
    'CgVfbG9nb0IHCgVfc2x1Zw==');

@$core.Deprecated('Use updateOrganizationRequestDescriptor instead')
const UpdateOrganizationRequest$json = {
  '1': 'UpdateOrganizationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name', '17': true},
    {'1': 'logo', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'logo', '17': true},
    {'1': 'slug', '3': 4, '4': 1, '5': 9, '9': 2, '10': 'slug', '17': true},
  ],
  '8': [
    {'1': '_name'},
    {'1': '_logo'},
    {'1': '_slug'},
  ],
};

/// Descriptor for `UpdateOrganizationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateOrganizationRequestDescriptor = $convert.base64Decode(
    'ChlVcGRhdGVPcmdhbml6YXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBIXCg'
    'RuYW1lGAIgASgJSABSBG5hbWWIAQESFwoEbG9nbxgDIAEoCUgBUgRsb2dviAEBEhcKBHNsdWcY'
    'BCABKAlIAlIEc2x1Z4gBAUIHCgVfbmFtZUIHCgVfbG9nb0IHCgVfc2x1Zw==');

@$core.Deprecated('Use deleteOrganizationRequestDescriptor instead')
const DeleteOrganizationRequest$json = {
  '1': 'DeleteOrganizationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeleteOrganizationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteOrganizationRequestDescriptor =
    $convert.base64Decode(
        'ChlEZWxldGVPcmdhbml6YXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

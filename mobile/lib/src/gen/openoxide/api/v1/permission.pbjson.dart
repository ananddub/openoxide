// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/permission.proto.

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

@$core.Deprecated('Use permissionUserRoleDescriptor instead')
const PermissionUserRole$json = {
  '1': 'PermissionUserRole',
  '2': [
    {'1': 'PERMISSION_USER_ROLE_UNSPECIFIED', '2': 0},
    {'1': 'PERMISSION_USER_ROLE_OWNER', '2': 1},
    {'1': 'PERMISSION_USER_ROLE_ADMIN', '2': 2},
    {'1': 'PERMISSION_USER_ROLE_MEMBER', '2': 3},
  ],
};

/// Descriptor for `PermissionUserRole`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List permissionUserRoleDescriptor = $convert.base64Decode(
    'ChJQZXJtaXNzaW9uVXNlclJvbGUSJAogUEVSTUlTU0lPTl9VU0VSX1JPTEVfVU5TUEVDSUZJRU'
    'QQABIeChpQRVJNSVNTSU9OX1VTRVJfUk9MRV9PV05FUhABEh4KGlBFUk1JU1NJT05fVVNFUl9S'
    'T0xFX0FETUlOEAISHwobUEVSTUlTU0lPTl9VU0VSX1JPTEVfTUVNQkVSEAM=');

@$core.Deprecated('Use permissionPolicyEffectDescriptor instead')
const PermissionPolicyEffect$json = {
  '1': 'PermissionPolicyEffect',
  '2': [
    {'1': 'PERMISSION_POLICY_EFFECT_UNSPECIFIED', '2': 0},
    {'1': 'PERMISSION_POLICY_EFFECT_GRANT', '2': 1},
    {'1': 'PERMISSION_POLICY_EFFECT_DENY', '2': 2},
  ],
};

/// Descriptor for `PermissionPolicyEffect`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List permissionPolicyEffectDescriptor = $convert.base64Decode(
    'ChZQZXJtaXNzaW9uUG9saWN5RWZmZWN0EigKJFBFUk1JU1NJT05fUE9MSUNZX0VGRkVDVF9VTl'
    'NQRUNJRklFRBAAEiIKHlBFUk1JU1NJT05fUE9MSUNZX0VGRkVDVF9HUkFOVBABEiEKHVBFUk1J'
    'U1NJT05fUE9MSUNZX0VGRkVDVF9ERU5ZEAI=');

@$core.Deprecated('Use permissionGroupDescriptor instead')
const PermissionGroup$json = {
  '1': 'PermissionGroup',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'is_system', '3': 3, '4': 1, '5': 8, '10': 'isSystem'},
    {
      '1': 'organization_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
    {'1': 'actions', '3': 5, '4': 3, '5': 9, '10': 'actions'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 7, '4': 1, '5': 3, '10': 'updatedAt'},
    {'1': 'is_active', '3': 8, '4': 1, '5': 8, '10': 'isActive'},
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `PermissionGroup`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionGroupDescriptor = $convert.base64Decode(
    'Cg9QZXJtaXNzaW9uR3JvdXASDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSGw'
    'oJaXNfc3lzdGVtGAMgASgIUghpc1N5c3RlbRIsCg9vcmdhbml6YXRpb25faWQYBCABKANIAFIO'
    'b3JnYW5pemF0aW9uSWSIAQESGAoHYWN0aW9ucxgFIAMoCVIHYWN0aW9ucxIdCgpjcmVhdGVkX2'
    'F0GAYgASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgHIAEoA1IJdXBkYXRlZEF0EhsKCWlz'
    'X2FjdGl2ZRgIIAEoCFIIaXNBY3RpdmVCEgoQX29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use permissionPolicyDescriptor instead')
const PermissionPolicy$json = {
  '1': 'PermissionPolicy',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'action', '3': 2, '4': 1, '5': 9, '10': 'action'},
  ],
};

/// Descriptor for `PermissionPolicy`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionPolicyDescriptor = $convert.base64Decode(
    'ChBQZXJtaXNzaW9uUG9saWN5Eg4KAmlkGAEgASgDUgJpZBIWCgZhY3Rpb24YAiABKAlSBmFjdG'
    'lvbg==');

@$core.Deprecated('Use permissionMemberDescriptor instead')
const PermissionMember$json = {
  '1': 'PermissionMember',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '9': 0, '10': 'id', '17': true},
    {'1': 'user_id', '3': 2, '4': 1, '5': 3, '10': 'userId'},
    {'1': 'email', '3': 3, '4': 1, '5': 9, '10': 'email'},
    {'1': 'avatar', '3': 4, '4': 1, '5': 9, '9': 1, '10': 'avatar', '17': true},
    {'1': 'organization_id', '3': 5, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'role',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '9': 2,
      '10': 'role',
      '17': true
    },
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 3, '10': 'updatedAt'},
    {'1': 'group_id', '3': 9, '4': 1, '5': 3, '10': 'groupId'},
    {
      '1': 'group_name',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'groupName',
      '17': true
    },
  ],
  '8': [
    {'1': '_id'},
    {'1': '_avatar'},
    {'1': '_role'},
    {'1': '_group_name'},
  ],
};

/// Descriptor for `PermissionMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionMemberDescriptor = $convert.base64Decode(
    'ChBQZXJtaXNzaW9uTWVtYmVyEhMKAmlkGAEgASgDSABSAmlkiAEBEhcKB3VzZXJfaWQYAiABKA'
    'NSBnVzZXJJZBIUCgVlbWFpbBgDIAEoCVIFZW1haWwSGwoGYXZhdGFyGAQgASgJSAFSBmF2YXRh'
    'cogBARInCg9vcmdhbml6YXRpb25faWQYBSABKANSDm9yZ2FuaXphdGlvbklkEj0KBHJvbGUYBi'
    'ABKA4yJC5vcGVub3hpZGUuYXBpLnYxLlBlcm1pc3Npb25Vc2VyUm9sZUgCUgRyb2xliAEBEh0K'
    'CmNyZWF0ZWRfYXQYByABKANSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAggASgDUgl1cGRhdG'
    'VkQXQSGQoIZ3JvdXBfaWQYCSABKANSB2dyb3VwSWQSIgoKZ3JvdXBfbmFtZRgKIAEoCUgDUgln'
    'cm91cE5hbWWIAQFCBQoDX2lkQgkKB19hdmF0YXJCBwoFX3JvbGVCDQoLX2dyb3VwX25hbWU=');

@$core.Deprecated('Use permissionInviteDescriptor instead')
const PermissionInvite$json = {
  '1': 'PermissionInvite',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'email', '3': 2, '4': 1, '5': 9, '10': 'email'},
    {
      '1': 'role',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '9': 0,
      '10': 'role',
      '17': true
    },
    {'1': 'group_id', '3': 4, '4': 1, '5': 3, '10': 'groupId'},
    {'1': 'organization_id', '3': 5, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'invited_by', '3': 6, '4': 1, '5': 3, '10': 'invitedBy'},
    {'1': 'expired_at', '3': 7, '4': 1, '5': 3, '10': 'expiredAt'},
    {'1': 'created_at', '3': 8, '4': 1, '5': 3, '10': 'createdAt'},
  ],
  '8': [
    {'1': '_role'},
  ],
};

/// Descriptor for `PermissionInvite`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionInviteDescriptor = $convert.base64Decode(
    'ChBQZXJtaXNzaW9uSW52aXRlEg4KAmlkGAEgASgDUgJpZBIUCgVlbWFpbBgCIAEoCVIFZW1haW'
    'wSPQoEcm9sZRgDIAEoDjIkLm9wZW5veGlkZS5hcGkudjEuUGVybWlzc2lvblVzZXJSb2xlSABS'
    'BHJvbGWIAQESGQoIZ3JvdXBfaWQYBCABKANSB2dyb3VwSWQSJwoPb3JnYW5pemF0aW9uX2lkGA'
    'UgASgDUg5vcmdhbml6YXRpb25JZBIdCgppbnZpdGVkX2J5GAYgASgDUglpbnZpdGVkQnkSHQoK'
    'ZXhwaXJlZF9hdBgHIAEoA1IJZXhwaXJlZEF0Eh0KCmNyZWF0ZWRfYXQYCCABKANSCWNyZWF0ZW'
    'RBdEIHCgVfcm9sZQ==');

@$core.Deprecated('Use permissionUserPolicyEntryDescriptor instead')
const PermissionUserPolicyEntry$json = {
  '1': 'PermissionUserPolicyEntry',
  '2': [
    {'1': 'policy_id', '3': 1, '4': 1, '5': 3, '10': 'policyId'},
    {
      '1': 'effect',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionPolicyEffect',
      '8': {},
      '10': 'effect'
    },
  ],
};

/// Descriptor for `PermissionUserPolicyEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionUserPolicyEntryDescriptor = $convert.base64Decode(
    'ChlQZXJtaXNzaW9uVXNlclBvbGljeUVudHJ5EhsKCXBvbGljeV9pZBgBIAEoA1IIcG9saWN5SW'
    'QSTAoGZWZmZWN0GAIgASgOMigub3Blbm94aWRlLmFwaS52MS5QZXJtaXNzaW9uUG9saWN5RWZm'
    'ZWN0Qgq6SAeCAQQQASAAUgZlZmZlY3Q=');

@$core.Deprecated('Use listPermissionsRequestDescriptor instead')
const ListPermissionsRequest$json = {
  '1': 'ListPermissionsRequest',
};

/// Descriptor for `ListPermissionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPermissionsRequestDescriptor =
    $convert.base64Decode('ChZMaXN0UGVybWlzc2lvbnNSZXF1ZXN0');

@$core.Deprecated('Use listPermissionsResponseDescriptor instead')
const ListPermissionsResponse$json = {
  '1': 'ListPermissionsResponse',
  '2': [
    {
      '1': 'groups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionGroup',
      '10': 'groups'
    },
  ],
};

/// Descriptor for `ListPermissionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPermissionsResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0UGVybWlzc2lvbnNSZXNwb25zZRI5CgZncm91cHMYASADKAsyIS5vcGVub3hpZGUuYX'
        'BpLnYxLlBlcm1pc3Npb25Hcm91cFIGZ3JvdXBz');

@$core.Deprecated('Use getPermissionRequestDescriptor instead')
const GetPermissionRequest$json = {
  '1': 'GetPermissionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `GetPermissionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPermissionRequestDescriptor =
    $convert.base64Decode(
        'ChRHZXRQZXJtaXNzaW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use getPermissionResponseDescriptor instead')
const GetPermissionResponse$json = {
  '1': 'GetPermissionResponse',
  '2': [
    {
      '1': 'group',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionGroup',
      '10': 'group'
    },
  ],
};

/// Descriptor for `GetPermissionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPermissionResponseDescriptor = $convert.base64Decode(
    'ChVHZXRQZXJtaXNzaW9uUmVzcG9uc2USNwoFZ3JvdXAYASABKAsyIS5vcGVub3hpZGUuYXBpLn'
    'YxLlBlcm1pc3Npb25Hcm91cFIFZ3JvdXA=');

@$core.Deprecated('Use createPermissionRequestDescriptor instead')
const CreatePermissionRequest$json = {
  '1': 'CreatePermissionRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'actions', '3': 2, '4': 3, '5': 9, '10': 'actions'},
  ],
};

/// Descriptor for `CreatePermissionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPermissionRequestDescriptor =
    $convert.base64Decode(
        'ChdDcmVhdGVQZXJtaXNzaW9uUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBRABGP8BUgRuYW'
        '1lEhgKB2FjdGlvbnMYAiADKAlSB2FjdGlvbnM=');

@$core.Deprecated('Use createPermissionResponseDescriptor instead')
const CreatePermissionResponse$json = {
  '1': 'CreatePermissionResponse',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `CreatePermissionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPermissionResponseDescriptor = $convert
    .base64Decode('ChhDcmVhdGVQZXJtaXNzaW9uUmVzcG9uc2USDgoCaWQYASABKANSAmlk');

@$core.Deprecated('Use updatePermissionRequestDescriptor instead')
const UpdatePermissionRequest$json = {
  '1': 'UpdatePermissionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'actions', '3': 3, '4': 3, '5': 9, '10': 'actions'},
  ],
};

/// Descriptor for `UpdatePermissionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updatePermissionRequestDescriptor = $convert.base64Decode(
    'ChdVcGRhdGVQZXJtaXNzaW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSHgoEbm'
    'FtZRgCIAEoCUIKukgHcgUQARj/AVIEbmFtZRIYCgdhY3Rpb25zGAMgAygJUgdhY3Rpb25z');

@$core.Deprecated('Use deletePermissionRequestDescriptor instead')
const DeletePermissionRequest$json = {
  '1': 'DeletePermissionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `DeletePermissionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deletePermissionRequestDescriptor =
    $convert.base64Decode(
        'ChdEZWxldGVQZXJtaXNzaW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQ=');

@$core.Deprecated('Use permissionListGroupsRequestDescriptor instead')
const PermissionListGroupsRequest$json = {
  '1': 'PermissionListGroupsRequest',
};

/// Descriptor for `PermissionListGroupsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListGroupsRequestDescriptor =
    $convert.base64Decode('ChtQZXJtaXNzaW9uTGlzdEdyb3Vwc1JlcXVlc3Q=');

@$core.Deprecated('Use permissionListGroupsResponseDescriptor instead')
const PermissionListGroupsResponse$json = {
  '1': 'PermissionListGroupsResponse',
  '2': [
    {
      '1': 'groups',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionGroup',
      '10': 'groups'
    },
  ],
};

/// Descriptor for `PermissionListGroupsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListGroupsResponseDescriptor =
    $convert.base64Decode(
        'ChxQZXJtaXNzaW9uTGlzdEdyb3Vwc1Jlc3BvbnNlEjkKBmdyb3VwcxgBIAMoCzIhLm9wZW5veG'
        'lkZS5hcGkudjEuUGVybWlzc2lvbkdyb3VwUgZncm91cHM=');

@$core.Deprecated('Use permissionGetGroupRequestDescriptor instead')
const PermissionGetGroupRequest$json = {
  '1': 'PermissionGetGroupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `PermissionGetGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionGetGroupRequestDescriptor =
    $convert.base64Decode(
        'ChlQZXJtaXNzaW9uR2V0R3JvdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA==');

@$core.Deprecated('Use permissionGetGroupResponseDescriptor instead')
const PermissionGetGroupResponse$json = {
  '1': 'PermissionGetGroupResponse',
  '2': [
    {
      '1': 'group',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionGroup',
      '10': 'group'
    },
  ],
};

/// Descriptor for `PermissionGetGroupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionGetGroupResponseDescriptor =
    $convert.base64Decode(
        'ChpQZXJtaXNzaW9uR2V0R3JvdXBSZXNwb25zZRI3CgVncm91cBgBIAEoCzIhLm9wZW5veGlkZS'
        '5hcGkudjEuUGVybWlzc2lvbkdyb3VwUgVncm91cA==');

@$core.Deprecated('Use permissionCreateGroupRequestDescriptor instead')
const PermissionCreateGroupRequest$json = {
  '1': 'PermissionCreateGroupRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'actions', '3': 2, '4': 3, '5': 9, '10': 'actions'},
  ],
};

/// Descriptor for `PermissionCreateGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionCreateGroupRequestDescriptor =
    $convert.base64Decode(
        'ChxQZXJtaXNzaW9uQ3JlYXRlR3JvdXBSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/w'
        'FSBG5hbWUSGAoHYWN0aW9ucxgCIAMoCVIHYWN0aW9ucw==');

@$core.Deprecated('Use permissionCreateGroupResponseDescriptor instead')
const PermissionCreateGroupResponse$json = {
  '1': 'PermissionCreateGroupResponse',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `PermissionCreateGroupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionCreateGroupResponseDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uQ3JlYXRlR3JvdXBSZXNwb25zZRIOCgJpZBgBIAEoA1ICaWQ=');

@$core.Deprecated('Use permissionUpdateGroupRequestDescriptor instead')
const PermissionUpdateGroupRequest$json = {
  '1': 'PermissionUpdateGroupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'actions', '3': 3, '4': 3, '5': 9, '10': 'actions'},
  ],
};

/// Descriptor for `PermissionUpdateGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionUpdateGroupRequestDescriptor =
    $convert.base64Decode(
        'ChxQZXJtaXNzaW9uVXBkYXRlR3JvdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZB'
        'IeCgRuYW1lGAIgASgJQgq6SAdyBRABGP8BUgRuYW1lEhgKB2FjdGlvbnMYAyADKAlSB2FjdGlv'
        'bnM=');

@$core.Deprecated('Use permissionDeleteGroupRequestDescriptor instead')
const PermissionDeleteGroupRequest$json = {
  '1': 'PermissionDeleteGroupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `PermissionDeleteGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionDeleteGroupRequestDescriptor =
    $convert.base64Decode(
        'ChxQZXJtaXNzaW9uRGVsZXRlR3JvdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZA'
        '==');

@$core.Deprecated('Use permissionListPoliciesRequestDescriptor instead')
const PermissionListPoliciesRequest$json = {
  '1': 'PermissionListPoliciesRequest',
};

/// Descriptor for `PermissionListPoliciesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListPoliciesRequestDescriptor =
    $convert.base64Decode('Ch1QZXJtaXNzaW9uTGlzdFBvbGljaWVzUmVxdWVzdA==');

@$core.Deprecated('Use permissionListPoliciesResponseDescriptor instead')
const PermissionListPoliciesResponse$json = {
  '1': 'PermissionListPoliciesResponse',
  '2': [
    {
      '1': 'policies',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionPolicy',
      '10': 'policies'
    },
  ],
};

/// Descriptor for `PermissionListPoliciesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListPoliciesResponseDescriptor =
    $convert.base64Decode(
        'Ch5QZXJtaXNzaW9uTGlzdFBvbGljaWVzUmVzcG9uc2USPgoIcG9saWNpZXMYASADKAsyIi5vcG'
        'Vub3hpZGUuYXBpLnYxLlBlcm1pc3Npb25Qb2xpY3lSCHBvbGljaWVz');

@$core.Deprecated('Use getMyPermissionsRequestDescriptor instead')
const GetMyPermissionsRequest$json = {
  '1': 'GetMyPermissionsRequest',
};

/// Descriptor for `GetMyPermissionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getMyPermissionsRequestDescriptor =
    $convert.base64Decode('ChdHZXRNeVBlcm1pc3Npb25zUmVxdWVzdA==');

@$core.Deprecated('Use getMyPermissionsResponseDescriptor instead')
const GetMyPermissionsResponse$json = {
  '1': 'GetMyPermissionsResponse',
  '2': [
    {
      '1': 'role',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '10': 'role'
    },
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'actions', '3': 3, '4': 3, '5': 9, '10': 'actions'},
  ],
};

/// Descriptor for `GetMyPermissionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getMyPermissionsResponseDescriptor = $convert.base64Decode(
    'ChhHZXRNeVBlcm1pc3Npb25zUmVzcG9uc2USOAoEcm9sZRgBIAEoDjIkLm9wZW5veGlkZS5hcG'
    'kudjEuUGVybWlzc2lvblVzZXJSb2xlUgRyb2xlEicKD29yZ2FuaXphdGlvbl9pZBgCIAEoA1IO'
    'b3JnYW5pemF0aW9uSWQSGAoHYWN0aW9ucxgDIAMoCVIHYWN0aW9ucw==');

@$core.Deprecated('Use permissionAssignMemberGroupRequestDescriptor instead')
const PermissionAssignMemberGroupRequest$json = {
  '1': 'PermissionAssignMemberGroupRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 3, '10': 'groupId'},
  ],
};

/// Descriptor for `PermissionAssignMemberGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionAssignMemberGroupRequestDescriptor =
    $convert.base64Decode(
        'CiJQZXJtaXNzaW9uQXNzaWduTWVtYmVyR3JvdXBSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAi'
        'AAUgJpZBIZCghncm91cF9pZBgCIAEoA1IHZ3JvdXBJZA==');

@$core.Deprecated('Use permissionSetGroupActiveRequestDescriptor instead')
const PermissionSetGroupActiveRequest$json = {
  '1': 'PermissionSetGroupActiveRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {'1': 'is_active', '3': 2, '4': 1, '5': 8, '10': 'isActive'},
  ],
};

/// Descriptor for `PermissionSetGroupActiveRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionSetGroupActiveRequestDescriptor =
    $convert.base64Decode(
        'Ch9QZXJtaXNzaW9uU2V0R3JvdXBBY3RpdmVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUg'
        'JpZBIbCglpc19hY3RpdmUYAiABKAhSCGlzQWN0aXZl');

@$core.Deprecated('Use permissionListUserPoliciesRequestDescriptor instead')
const PermissionListUserPoliciesRequest$json = {
  '1': 'PermissionListUserPoliciesRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `PermissionListUserPoliciesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListUserPoliciesRequestDescriptor =
    $convert.base64Decode(
        'CiFQZXJtaXNzaW9uTGlzdFVzZXJQb2xpY2llc1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlk');

@$core.Deprecated('Use permissionListUserPoliciesResponseDescriptor instead')
const PermissionListUserPoliciesResponse$json = {
  '1': 'PermissionListUserPoliciesResponse',
  '2': [
    {
      '1': 'policies',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionUserPolicyEntry',
      '10': 'policies'
    },
  ],
};

/// Descriptor for `PermissionListUserPoliciesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListUserPoliciesResponseDescriptor =
    $convert.base64Decode(
        'CiJQZXJtaXNzaW9uTGlzdFVzZXJQb2xpY2llc1Jlc3BvbnNlEkcKCHBvbGljaWVzGAEgAygLMi'
        'sub3Blbm94aWRlLmFwaS52MS5QZXJtaXNzaW9uVXNlclBvbGljeUVudHJ5Ughwb2xpY2llcw==');

@$core.Deprecated('Use permissionListMembersRequestDescriptor instead')
const PermissionListMembersRequest$json = {
  '1': 'PermissionListMembersRequest',
};

/// Descriptor for `PermissionListMembersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListMembersRequestDescriptor =
    $convert.base64Decode('ChxQZXJtaXNzaW9uTGlzdE1lbWJlcnNSZXF1ZXN0');

@$core.Deprecated('Use permissionListMembersResponseDescriptor instead')
const PermissionListMembersResponse$json = {
  '1': 'PermissionListMembersResponse',
  '2': [
    {
      '1': 'members',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionMember',
      '10': 'members'
    },
  ],
};

/// Descriptor for `PermissionListMembersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListMembersResponseDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uTGlzdE1lbWJlcnNSZXNwb25zZRI8CgdtZW1iZXJzGAEgAygLMiIub3Blbm'
        '94aWRlLmFwaS52MS5QZXJtaXNzaW9uTWVtYmVyUgdtZW1iZXJz');

@$core.Deprecated('Use permissionAddMemberRequestDescriptor instead')
const PermissionAddMemberRequest$json = {
  '1': 'PermissionAddMemberRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'email'},
    {
      '1': 'password',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'password',
      '17': true
    },
    {
      '1': 'role',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '8': {},
      '10': 'role'
    },
    {
      '1': 'group_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'groupId',
      '17': true
    },
  ],
  '8': [
    {'1': '_password'},
    {'1': '_group_id'},
  ],
};

/// Descriptor for `PermissionAddMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionAddMemberRequestDescriptor = $convert.base64Decode(
    'ChpQZXJtaXNzaW9uQWRkTWVtYmVyUmVxdWVzdBIgCgVlbWFpbBgBIAEoCUIKukgHcgUQARjAAl'
    'IFZW1haWwSKQoIcGFzc3dvcmQYAiABKAlCCLpIBXIDGIAISABSCHBhc3N3b3JkiAEBEkYKBHJv'
    'bGUYAyABKA4yJC5vcGVub3hpZGUuYXBpLnYxLlBlcm1pc3Npb25Vc2VyUm9sZUIMukgJggEGEA'
    'EgACABUgRyb2xlEicKCGdyb3VwX2lkGAQgASgDQge6SAQiAiAASAFSB2dyb3VwSWSIAQFCCwoJ'
    'X3Bhc3N3b3JkQgsKCV9ncm91cF9pZA==');

@$core.Deprecated('Use permissionUpdateMemberRoleRequestDescriptor instead')
const PermissionUpdateMemberRoleRequest$json = {
  '1': 'PermissionUpdateMemberRoleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'role',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '8': {},
      '10': 'role'
    },
  ],
};

/// Descriptor for `PermissionUpdateMemberRoleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionUpdateMemberRoleRequestDescriptor =
    $convert.base64Decode(
        'CiFQZXJtaXNzaW9uVXBkYXRlTWVtYmVyUm9sZVJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIA'
        'BSAmlkEkYKBHJvbGUYAiABKA4yJC5vcGVub3hpZGUuYXBpLnYxLlBlcm1pc3Npb25Vc2VyUm9s'
        'ZUIMukgJggEGEAEgACABUgRyb2xl');

@$core.Deprecated('Use permissionRemoveMemberRequestDescriptor instead')
const PermissionRemoveMemberRequest$json = {
  '1': 'PermissionRemoveMemberRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `PermissionRemoveMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionRemoveMemberRequestDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uUmVtb3ZlTWVtYmVyUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use permissionListInvitesRequestDescriptor instead')
const PermissionListInvitesRequest$json = {
  '1': 'PermissionListInvitesRequest',
};

/// Descriptor for `PermissionListInvitesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListInvitesRequestDescriptor =
    $convert.base64Decode('ChxQZXJtaXNzaW9uTGlzdEludml0ZXNSZXF1ZXN0');

@$core.Deprecated('Use permissionListInvitesResponseDescriptor instead')
const PermissionListInvitesResponse$json = {
  '1': 'PermissionListInvitesResponse',
  '2': [
    {
      '1': 'invites',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionInvite',
      '10': 'invites'
    },
  ],
};

/// Descriptor for `PermissionListInvitesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionListInvitesResponseDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uTGlzdEludml0ZXNSZXNwb25zZRI8CgdpbnZpdGVzGAEgAygLMiIub3Blbm'
        '94aWRlLmFwaS52MS5QZXJtaXNzaW9uSW52aXRlUgdpbnZpdGVz');

@$core.Deprecated('Use permissionCreateInviteRequestDescriptor instead')
const PermissionCreateInviteRequest$json = {
  '1': 'PermissionCreateInviteRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'email'},
    {
      '1': 'role',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.PermissionUserRole',
      '8': {},
      '10': 'role'
    },
    {'1': 'group_id', '3': 3, '4': 1, '5': 3, '10': 'groupId'},
  ],
};

/// Descriptor for `PermissionCreateInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionCreateInviteRequestDescriptor = $convert.base64Decode(
    'Ch1QZXJtaXNzaW9uQ3JlYXRlSW52aXRlUmVxdWVzdBIgCgVlbWFpbBgBIAEoCUIKukgHcgUQAR'
    'jAAlIFZW1haWwSRgoEcm9sZRgCIAEoDjIkLm9wZW5veGlkZS5hcGkudjEuUGVybWlzc2lvblVz'
    'ZXJSb2xlQgy6SAmCAQYQASAAIAFSBHJvbGUSGQoIZ3JvdXBfaWQYAyABKANSB2dyb3VwSWQ=');

@$core.Deprecated('Use permissionCreateInviteResponseDescriptor instead')
const PermissionCreateInviteResponse$json = {
  '1': 'PermissionCreateInviteResponse',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `PermissionCreateInviteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionCreateInviteResponseDescriptor =
    $convert.base64Decode(
        'Ch5QZXJtaXNzaW9uQ3JlYXRlSW52aXRlUmVzcG9uc2USFAoFdG9rZW4YASABKAlSBXRva2Vu');

@$core.Deprecated('Use permissionAcceptInviteRequestDescriptor instead')
const PermissionAcceptInviteRequest$json = {
  '1': 'PermissionAcceptInviteRequest',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'token'},
  ],
};

/// Descriptor for `PermissionAcceptInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionAcceptInviteRequestDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uQWNjZXB0SW52aXRlUmVxdWVzdBIgCgV0b2tlbhgBIAEoCUIKukgHcgUQAR'
        'iAIFIFdG9rZW4=');

@$core.Deprecated('Use permissionCancelInviteRequestDescriptor instead')
const PermissionCancelInviteRequest$json = {
  '1': 'PermissionCancelInviteRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `PermissionCancelInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionCancelInviteRequestDescriptor =
    $convert.base64Decode(
        'Ch1QZXJtaXNzaW9uQ2FuY2VsSW52aXRlUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaW'
        'Q=');

@$core.Deprecated('Use permissionReplaceUserPoliciesRequestDescriptor instead')
const PermissionReplaceUserPoliciesRequest$json = {
  '1': 'PermissionReplaceUserPoliciesRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'policies',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PermissionUserPolicyEntry',
      '10': 'policies'
    },
  ],
};

/// Descriptor for `PermissionReplaceUserPoliciesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List permissionReplaceUserPoliciesRequestDescriptor =
    $convert.base64Decode(
        'CiRQZXJtaXNzaW9uUmVwbGFjZVVzZXJQb2xpY2llc1JlcXVlc3QSFwoCaWQYASABKANCB7pIBC'
        'ICIABSAmlkEkcKCHBvbGljaWVzGAIgAygLMisub3Blbm94aWRlLmFwaS52MS5QZXJtaXNzaW9u'
        'VXNlclBvbGljeUVudHJ5Ughwb2xpY2llcw==');

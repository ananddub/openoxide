// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/permission.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Membership roles (`UserRole` string enum); assignment endpoints only accept `ADMIN` or `MEMBER`.
class PermissionUserRole extends $pb.ProtobufEnum {
  static const PermissionUserRole PERMISSION_USER_ROLE_UNSPECIFIED =
      PermissionUserRole._(
          0, _omitEnumNames ? '' : 'PERMISSION_USER_ROLE_UNSPECIFIED');
  static const PermissionUserRole PERMISSION_USER_ROLE_OWNER =
      PermissionUserRole._(
          1, _omitEnumNames ? '' : 'PERMISSION_USER_ROLE_OWNER');
  static const PermissionUserRole PERMISSION_USER_ROLE_ADMIN =
      PermissionUserRole._(
          2, _omitEnumNames ? '' : 'PERMISSION_USER_ROLE_ADMIN');
  static const PermissionUserRole PERMISSION_USER_ROLE_MEMBER =
      PermissionUserRole._(
          3, _omitEnumNames ? '' : 'PERMISSION_USER_ROLE_MEMBER');

  static const $core.List<PermissionUserRole> values = <PermissionUserRole>[
    PERMISSION_USER_ROLE_UNSPECIFIED,
    PERMISSION_USER_ROLE_OWNER,
    PERMISSION_USER_ROLE_ADMIN,
    PERMISSION_USER_ROLE_MEMBER,
  ];

  static final $core.List<PermissionUserRole?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static PermissionUserRole? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const PermissionUserRole._(super.value, super.name);
}

/// Policy effect on a per-user policy override (`PolicyEffect` string enum).
class PermissionPolicyEffect extends $pb.ProtobufEnum {
  static const PermissionPolicyEffect PERMISSION_POLICY_EFFECT_UNSPECIFIED =
      PermissionPolicyEffect._(
          0, _omitEnumNames ? '' : 'PERMISSION_POLICY_EFFECT_UNSPECIFIED');
  static const PermissionPolicyEffect PERMISSION_POLICY_EFFECT_GRANT =
      PermissionPolicyEffect._(
          1, _omitEnumNames ? '' : 'PERMISSION_POLICY_EFFECT_GRANT');
  static const PermissionPolicyEffect PERMISSION_POLICY_EFFECT_DENY =
      PermissionPolicyEffect._(
          2, _omitEnumNames ? '' : 'PERMISSION_POLICY_EFFECT_DENY');

  static const $core.List<PermissionPolicyEffect> values =
      <PermissionPolicyEffect>[
    PERMISSION_POLICY_EFFECT_UNSPECIFIED,
    PERMISSION_POLICY_EFFECT_GRANT,
    PERMISSION_POLICY_EFFECT_DENY,
  ];

  static final $core.List<PermissionPolicyEffect?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static PermissionPolicyEffect? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const PermissionPolicyEffect._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

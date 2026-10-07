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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'permission.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'permission.pbenum.dart';

/// Permission group ("role"): a named action set inside an organization.
class PermissionGroup extends $pb.GeneratedMessage {
  factory PermissionGroup({
    $fixnum.Int64? id,
    $core.String? name,
    $core.bool? isSystem,
    $fixnum.Int64? organizationId,
    $core.Iterable<$core.String>? actions,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.bool? isActive,
  }) {
    final result = PermissionGroup._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (isSystem != null) result.isSystem = isSystem;
    if (organizationId != null) result.organizationId = organizationId;
    if (actions != null) result.actions.addAll(actions);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (isActive != null) result.isActive = isActive;
    return result;
  }

  PermissionGroup._();

  factory PermissionGroup.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGroup()..mergeFromBuffer(data, registry);
  factory PermissionGroup.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGroup()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionGroup',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionGroup.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOB(3, _omitFieldNames ? '' : 'isSystem')
    ..aInt64(4, _omitFieldNames ? '' : 'organizationId')
    ..pPS(5, _omitFieldNames ? '' : 'actions')
    ..aInt64(6, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(7, _omitFieldNames ? '' : 'updatedAt')
    ..aOB(8, _omitFieldNames ? '' : 'isActive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGroup clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGroup copyWith(void Function(PermissionGroup) updates) =>
      super.copyWith((message) => updates(message as PermissionGroup))
          as PermissionGroup;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PermissionGroup() / PermissionGroup.new instead')
  static PermissionGroup create() => PermissionGroup._();
  static $pb.GeneratedMessage $_createMessage() => PermissionGroup._();
  @$core.override
  PermissionGroup createEmptyInstance() => PermissionGroup._();
  @$core.pragma('dart2js:noInline')
  static PermissionGroup getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PermissionGroup>(
          PermissionGroup.$_createMessage);
  static PermissionGroup? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get isSystem => $_getBF(2);
  @$pb.TagNumber(3)
  set isSystem($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasIsSystem() => $_has(2);
  @$pb.TagNumber(3)
  void clearIsSystem() => $_clearField(3);

  /// Null for global system groups.
  @$pb.TagNumber(4)
  $fixnum.Int64 get organizationId => $_getI64(3);
  @$pb.TagNumber(4)
  set organizationId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOrganizationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearOrganizationId() => $_clearField(4);

  /// Effective action strings, e.g. "PROJECT:read"; `*` grants everything.
  @$pb.TagNumber(5)
  $pb.PbList<$core.String> get actions => $_getList(4);

  @$pb.TagNumber(6)
  $fixnum.Int64 get createdAt => $_getI64(5);
  @$pb.TagNumber(6)
  set createdAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCreatedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get updatedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set updatedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasUpdatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearUpdatedAt() => $_clearField(7);

  /// Disabled groups grant nothing to their members until re-enabled.
  @$pb.TagNumber(8)
  $core.bool get isActive => $_getBF(7);
  @$pb.TagNumber(8)
  set isActive($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasIsActive() => $_has(7);
  @$pb.TagNumber(8)
  void clearIsActive() => $_clearField(8);
}

/// Catalog entry of an addressable policy.
class PermissionPolicy extends $pb.GeneratedMessage {
  factory PermissionPolicy({
    $fixnum.Int64? id,
    $core.String? action,
  }) {
    final result = PermissionPolicy._();
    if (id != null) result.id = id;
    if (action != null) result.action = action;
    return result;
  }

  PermissionPolicy._();

  factory PermissionPolicy.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionPolicy()..mergeFromBuffer(data, registry);
  factory PermissionPolicy.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionPolicy()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionPolicy',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionPolicy.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'action')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionPolicy clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionPolicy copyWith(void Function(PermissionPolicy) updates) =>
      super.copyWith((message) => updates(message as PermissionPolicy))
          as PermissionPolicy;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PermissionPolicy() / PermissionPolicy.new instead')
  static PermissionPolicy create() => PermissionPolicy._();
  static $pb.GeneratedMessage $_createMessage() => PermissionPolicy._();
  @$core.override
  PermissionPolicy createEmptyInstance() => PermissionPolicy._();
  @$core.pragma('dart2js:noInline')
  static PermissionPolicy getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PermissionPolicy>(
          PermissionPolicy.$_createMessage);
  static PermissionPolicy? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get action => $_getSZ(1);
  @$pb.TagNumber(2)
  set action($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);
}

/// Organization member projection.
class PermissionMember extends $pb.GeneratedMessage {
  factory PermissionMember({
    $fixnum.Int64? id,
    $fixnum.Int64? userId,
    $core.String? email,
    $core.String? avatar,
    $fixnum.Int64? organizationId,
    PermissionUserRole? role,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $fixnum.Int64? groupId,
    $core.String? groupName,
  }) {
    final result = PermissionMember._();
    if (id != null) result.id = id;
    if (userId != null) result.userId = userId;
    if (email != null) result.email = email;
    if (avatar != null) result.avatar = avatar;
    if (organizationId != null) result.organizationId = organizationId;
    if (role != null) result.role = role;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (groupId != null) result.groupId = groupId;
    if (groupName != null) result.groupName = groupName;
    return result;
  }

  PermissionMember._();

  factory PermissionMember.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionMember()..mergeFromBuffer(data, registry);
  factory PermissionMember.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionMember()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionMember',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionMember.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'userId')
    ..aOS(3, _omitFieldNames ? '' : 'email')
    ..aOS(4, _omitFieldNames ? '' : 'avatar')
    ..aInt64(5, _omitFieldNames ? '' : 'organizationId')
    ..aE<PermissionUserRole>(6, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..aInt64(9, _omitFieldNames ? '' : 'groupId')
    ..aOS(10, _omitFieldNames ? '' : 'groupName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionMember clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionMember copyWith(void Function(PermissionMember) updates) =>
      super.copyWith((message) => updates(message as PermissionMember))
          as PermissionMember;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PermissionMember() / PermissionMember.new instead')
  static PermissionMember create() => PermissionMember._();
  static $pb.GeneratedMessage $_createMessage() => PermissionMember._();
  @$core.override
  PermissionMember createEmptyInstance() => PermissionMember._();
  @$core.pragma('dart2js:noInline')
  static PermissionMember getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PermissionMember>(
          PermissionMember.$_createMessage);
  static PermissionMember? _defaultInstance;

  /// Row id of the membership; absent when the projection has no row.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get userId => $_getI64(1);
  @$pb.TagNumber(2)
  set userId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get email => $_getSZ(2);
  @$pb.TagNumber(3)
  set email($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasEmail() => $_has(2);
  @$pb.TagNumber(3)
  void clearEmail() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get avatar => $_getSZ(3);
  @$pb.TagNumber(4)
  set avatar($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAvatar() => $_has(3);
  @$pb.TagNumber(4)
  void clearAvatar() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get organizationId => $_getI64(4);
  @$pb.TagNumber(5)
  set organizationId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOrganizationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearOrganizationId() => $_clearField(5);

  @$pb.TagNumber(6)
  PermissionUserRole get role => $_getN(5);
  @$pb.TagNumber(6)
  set role(PermissionUserRole value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasRole() => $_has(5);
  @$pb.TagNumber(6)
  void clearRole() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get updatedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set updatedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasUpdatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearUpdatedAt() => $_clearField(8);

  /// Permission group granting this member's fine-grained actions; 0 when none.
  @$pb.TagNumber(9)
  $fixnum.Int64 get groupId => $_getI64(8);
  @$pb.TagNumber(9)
  set groupId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasGroupId() => $_has(8);
  @$pb.TagNumber(9)
  void clearGroupId() => $_clearField(9);

  /// Display name of `group_id`, absent when no group is assigned.
  @$pb.TagNumber(10)
  $core.String get groupName => $_getSZ(9);
  @$pb.TagNumber(10)
  set groupName($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasGroupName() => $_has(9);
  @$pb.TagNumber(10)
  void clearGroupName() => $_clearField(10);
}

/// Pending organization invitation.
class PermissionInvite extends $pb.GeneratedMessage {
  factory PermissionInvite({
    $fixnum.Int64? id,
    $core.String? email,
    PermissionUserRole? role,
    $fixnum.Int64? groupId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? invitedBy,
    $fixnum.Int64? expiredAt,
    $fixnum.Int64? createdAt,
  }) {
    final result = PermissionInvite._();
    if (id != null) result.id = id;
    if (email != null) result.email = email;
    if (role != null) result.role = role;
    if (groupId != null) result.groupId = groupId;
    if (organizationId != null) result.organizationId = organizationId;
    if (invitedBy != null) result.invitedBy = invitedBy;
    if (expiredAt != null) result.expiredAt = expiredAt;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  PermissionInvite._();

  factory PermissionInvite.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionInvite()..mergeFromBuffer(data, registry);
  factory PermissionInvite.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionInvite()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionInvite',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionInvite.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'email')
    ..aE<PermissionUserRole>(3, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..aInt64(4, _omitFieldNames ? '' : 'groupId')
    ..aInt64(5, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(6, _omitFieldNames ? '' : 'invitedBy')
    ..aInt64(7, _omitFieldNames ? '' : 'expiredAt')
    ..aInt64(8, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionInvite clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionInvite copyWith(void Function(PermissionInvite) updates) =>
      super.copyWith((message) => updates(message as PermissionInvite))
          as PermissionInvite;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PermissionInvite() / PermissionInvite.new instead')
  static PermissionInvite create() => PermissionInvite._();
  static $pb.GeneratedMessage $_createMessage() => PermissionInvite._();
  @$core.override
  PermissionInvite createEmptyInstance() => PermissionInvite._();
  @$core.pragma('dart2js:noInline')
  static PermissionInvite getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PermissionInvite>(
          PermissionInvite.$_createMessage);
  static PermissionInvite? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get email => $_getSZ(1);
  @$pb.TagNumber(2)
  set email($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEmail() => $_has(1);
  @$pb.TagNumber(2)
  void clearEmail() => $_clearField(2);

  @$pb.TagNumber(3)
  PermissionUserRole get role => $_getN(2);
  @$pb.TagNumber(3)
  set role(PermissionUserRole value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearRole() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get groupId => $_getI64(3);
  @$pb.TagNumber(4)
  set groupId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasGroupId() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get organizationId => $_getI64(4);
  @$pb.TagNumber(5)
  set organizationId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOrganizationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearOrganizationId() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get invitedBy => $_getI64(5);
  @$pb.TagNumber(6)
  set invitedBy($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasInvitedBy() => $_has(5);
  @$pb.TagNumber(6)
  void clearInvitedBy() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get expiredAt => $_getI64(6);
  @$pb.TagNumber(7)
  set expiredAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasExpiredAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearExpiredAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get createdAt => $_getI64(7);
  @$pb.TagNumber(8)
  set createdAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCreatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedAt() => $_clearField(8);
}

/// One entry of a full per-user policy replacement.
class PermissionUserPolicyEntry extends $pb.GeneratedMessage {
  factory PermissionUserPolicyEntry({
    $fixnum.Int64? policyId,
    PermissionPolicyEffect? effect,
  }) {
    final result = PermissionUserPolicyEntry._();
    if (policyId != null) result.policyId = policyId;
    if (effect != null) result.effect = effect;
    return result;
  }

  PermissionUserPolicyEntry._();

  factory PermissionUserPolicyEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUserPolicyEntry()..mergeFromBuffer(data, registry);
  factory PermissionUserPolicyEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUserPolicyEntry()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionUserPolicyEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionUserPolicyEntry.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'policyId')
    ..aE<PermissionPolicyEffect>(2, _omitFieldNames ? '' : 'effect',
        enumValues: PermissionPolicyEffect.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUserPolicyEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUserPolicyEntry copyWith(
          void Function(PermissionUserPolicyEntry) updates) =>
      super.copyWith((message) => updates(message as PermissionUserPolicyEntry))
          as PermissionUserPolicyEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionUserPolicyEntry() / PermissionUserPolicyEntry.new instead')
  static PermissionUserPolicyEntry create() => PermissionUserPolicyEntry._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionUserPolicyEntry._();
  @$core.override
  PermissionUserPolicyEntry createEmptyInstance() =>
      PermissionUserPolicyEntry._();
  @$core.pragma('dart2js:noInline')
  static PermissionUserPolicyEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionUserPolicyEntry>(
          PermissionUserPolicyEntry.$_createMessage);
  static PermissionUserPolicyEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get policyId => $_getI64(0);
  @$pb.TagNumber(1)
  set policyId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPolicyId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPolicyId() => $_clearField(1);

  @$pb.TagNumber(2)
  PermissionPolicyEffect get effect => $_getN(1);
  @$pb.TagNumber(2)
  set effect(PermissionPolicyEffect value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasEffect() => $_has(1);
  @$pb.TagNumber(2)
  void clearEffect() => $_clearField(2);
}

class ListPermissionsRequest extends $pb.GeneratedMessage {
  factory ListPermissionsRequest() => ListPermissionsRequest._();

  ListPermissionsRequest._();

  factory ListPermissionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPermissionsRequest()..mergeFromBuffer(data, registry);
  factory ListPermissionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPermissionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPermissionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPermissionsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPermissionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPermissionsRequest copyWith(
          void Function(ListPermissionsRequest) updates) =>
      super.copyWith((message) => updates(message as ListPermissionsRequest))
          as ListPermissionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListPermissionsRequest() / ListPermissionsRequest.new instead')
  static ListPermissionsRequest create() => ListPermissionsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListPermissionsRequest._();
  @$core.override
  ListPermissionsRequest createEmptyInstance() => ListPermissionsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListPermissionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPermissionsRequest>(
          ListPermissionsRequest.$_createMessage);
  static ListPermissionsRequest? _defaultInstance;
}

class ListPermissionsResponse extends $pb.GeneratedMessage {
  factory ListPermissionsResponse({
    $core.Iterable<PermissionGroup>? groups,
  }) {
    final result = ListPermissionsResponse._();
    if (groups != null) result.groups.addAll(groups);
    return result;
  }

  ListPermissionsResponse._();

  factory ListPermissionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPermissionsResponse()..mergeFromBuffer(data, registry);
  factory ListPermissionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPermissionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPermissionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPermissionsResponse.$_createMessage)
    ..pPM<PermissionGroup>(1, _omitFieldNames ? '' : 'groups',
        subBuilder: PermissionGroup.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPermissionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPermissionsResponse copyWith(
          void Function(ListPermissionsResponse) updates) =>
      super.copyWith((message) => updates(message as ListPermissionsResponse))
          as ListPermissionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListPermissionsResponse() / ListPermissionsResponse.new instead')
  static ListPermissionsResponse create() => ListPermissionsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListPermissionsResponse._();
  @$core.override
  ListPermissionsResponse createEmptyInstance() => ListPermissionsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListPermissionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPermissionsResponse>(
          ListPermissionsResponse.$_createMessage);
  static ListPermissionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionGroup> get groups => $_getList(0);
}

class GetPermissionRequest extends $pb.GeneratedMessage {
  factory GetPermissionRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetPermissionRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetPermissionRequest._();

  factory GetPermissionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPermissionRequest()..mergeFromBuffer(data, registry);
  factory GetPermissionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPermissionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPermissionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetPermissionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPermissionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPermissionRequest copyWith(void Function(GetPermissionRequest) updates) =>
      super.copyWith((message) => updates(message as GetPermissionRequest))
          as GetPermissionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetPermissionRequest() / GetPermissionRequest.new instead')
  static GetPermissionRequest create() => GetPermissionRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetPermissionRequest._();
  @$core.override
  GetPermissionRequest createEmptyInstance() => GetPermissionRequest._();
  @$core.pragma('dart2js:noInline')
  static GetPermissionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetPermissionRequest>(
          GetPermissionRequest.$_createMessage);
  static GetPermissionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetPermissionResponse extends $pb.GeneratedMessage {
  factory GetPermissionResponse({
    PermissionGroup? group,
  }) {
    final result = GetPermissionResponse._();
    if (group != null) result.group = group;
    return result;
  }

  GetPermissionResponse._();

  factory GetPermissionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPermissionResponse()..mergeFromBuffer(data, registry);
  factory GetPermissionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPermissionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPermissionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetPermissionResponse.$_createMessage)
    ..aOM<PermissionGroup>(1, _omitFieldNames ? '' : 'group',
        subBuilder: PermissionGroup.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPermissionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPermissionResponse copyWith(
          void Function(GetPermissionResponse) updates) =>
      super.copyWith((message) => updates(message as GetPermissionResponse))
          as GetPermissionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetPermissionResponse() / GetPermissionResponse.new instead')
  static GetPermissionResponse create() => GetPermissionResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetPermissionResponse._();
  @$core.override
  GetPermissionResponse createEmptyInstance() => GetPermissionResponse._();
  @$core.pragma('dart2js:noInline')
  static GetPermissionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetPermissionResponse>(
          GetPermissionResponse.$_createMessage);
  static GetPermissionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PermissionGroup get group => $_getN(0);
  @$pb.TagNumber(1)
  set group(PermissionGroup value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => $_clearField(1);
  @$pb.TagNumber(1)
  PermissionGroup ensureGroup() => $_ensure(0);
}

class CreatePermissionRequest extends $pb.GeneratedMessage {
  factory CreatePermissionRequest({
    $core.String? name,
    $core.Iterable<$core.String>? actions,
  }) {
    final result = CreatePermissionRequest._();
    if (name != null) result.name = name;
    if (actions != null) result.actions.addAll(actions);
    return result;
  }

  CreatePermissionRequest._();

  factory CreatePermissionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePermissionRequest()..mergeFromBuffer(data, registry);
  factory CreatePermissionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePermissionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePermissionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatePermissionRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..pPS(2, _omitFieldNames ? '' : 'actions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePermissionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePermissionRequest copyWith(
          void Function(CreatePermissionRequest) updates) =>
      super.copyWith((message) => updates(message as CreatePermissionRequest))
          as CreatePermissionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatePermissionRequest() / CreatePermissionRequest.new instead')
  static CreatePermissionRequest create() => CreatePermissionRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreatePermissionRequest._();
  @$core.override
  CreatePermissionRequest createEmptyInstance() => CreatePermissionRequest._();
  @$core.pragma('dart2js:noInline')
  static CreatePermissionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePermissionRequest>(
          CreatePermissionRequest.$_createMessage);
  static CreatePermissionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get actions => $_getList(1);
}

class CreatePermissionResponse extends $pb.GeneratedMessage {
  factory CreatePermissionResponse({
    $fixnum.Int64? id,
  }) {
    final result = CreatePermissionResponse._();
    if (id != null) result.id = id;
    return result;
  }

  CreatePermissionResponse._();

  factory CreatePermissionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePermissionResponse()..mergeFromBuffer(data, registry);
  factory CreatePermissionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePermissionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePermissionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatePermissionResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePermissionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePermissionResponse copyWith(
          void Function(CreatePermissionResponse) updates) =>
      super.copyWith((message) => updates(message as CreatePermissionResponse))
          as CreatePermissionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatePermissionResponse() / CreatePermissionResponse.new instead')
  static CreatePermissionResponse create() => CreatePermissionResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreatePermissionResponse._();
  @$core.override
  CreatePermissionResponse createEmptyInstance() =>
      CreatePermissionResponse._();
  @$core.pragma('dart2js:noInline')
  static CreatePermissionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePermissionResponse>(
          CreatePermissionResponse.$_createMessage);
  static CreatePermissionResponse? _defaultInstance;

  /// The create endpoint only echoes the new row id.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class UpdatePermissionRequest extends $pb.GeneratedMessage {
  factory UpdatePermissionRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.Iterable<$core.String>? actions,
  }) {
    final result = UpdatePermissionRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (actions != null) result.actions.addAll(actions);
    return result;
  }

  UpdatePermissionRequest._();

  factory UpdatePermissionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdatePermissionRequest()..mergeFromBuffer(data, registry);
  factory UpdatePermissionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdatePermissionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdatePermissionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdatePermissionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..pPS(3, _omitFieldNames ? '' : 'actions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdatePermissionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdatePermissionRequest copyWith(
          void Function(UpdatePermissionRequest) updates) =>
      super.copyWith((message) => updates(message as UpdatePermissionRequest))
          as UpdatePermissionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdatePermissionRequest() / UpdatePermissionRequest.new instead')
  static UpdatePermissionRequest create() => UpdatePermissionRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdatePermissionRequest._();
  @$core.override
  UpdatePermissionRequest createEmptyInstance() => UpdatePermissionRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdatePermissionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdatePermissionRequest>(
          UpdatePermissionRequest.$_createMessage);
  static UpdatePermissionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// Full replacement of the group's action list.
  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get actions => $_getList(2);
}

class DeletePermissionRequest extends $pb.GeneratedMessage {
  factory DeletePermissionRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeletePermissionRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeletePermissionRequest._();

  factory DeletePermissionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeletePermissionRequest()..mergeFromBuffer(data, registry);
  factory DeletePermissionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeletePermissionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeletePermissionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeletePermissionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePermissionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePermissionRequest copyWith(
          void Function(DeletePermissionRequest) updates) =>
      super.copyWith((message) => updates(message as DeletePermissionRequest))
          as DeletePermissionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeletePermissionRequest() / DeletePermissionRequest.new instead')
  static DeletePermissionRequest create() => DeletePermissionRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeletePermissionRequest._();
  @$core.override
  DeletePermissionRequest createEmptyInstance() => DeletePermissionRequest._();
  @$core.pragma('dart2js:noInline')
  static DeletePermissionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeletePermissionRequest>(
          DeletePermissionRequest.$_createMessage);
  static DeletePermissionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionListGroupsRequest extends $pb.GeneratedMessage {
  factory PermissionListGroupsRequest() => PermissionListGroupsRequest._();

  PermissionListGroupsRequest._();

  factory PermissionListGroupsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListGroupsRequest()..mergeFromBuffer(data, registry);
  factory PermissionListGroupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListGroupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListGroupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListGroupsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListGroupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListGroupsRequest copyWith(
          void Function(PermissionListGroupsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListGroupsRequest))
          as PermissionListGroupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListGroupsRequest() / PermissionListGroupsRequest.new instead')
  static PermissionListGroupsRequest create() =>
      PermissionListGroupsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListGroupsRequest._();
  @$core.override
  PermissionListGroupsRequest createEmptyInstance() =>
      PermissionListGroupsRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionListGroupsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListGroupsRequest>(
          PermissionListGroupsRequest.$_createMessage);
  static PermissionListGroupsRequest? _defaultInstance;
}

class PermissionListGroupsResponse extends $pb.GeneratedMessage {
  factory PermissionListGroupsResponse({
    $core.Iterable<PermissionGroup>? groups,
  }) {
    final result = PermissionListGroupsResponse._();
    if (groups != null) result.groups.addAll(groups);
    return result;
  }

  PermissionListGroupsResponse._();

  factory PermissionListGroupsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListGroupsResponse()..mergeFromBuffer(data, registry);
  factory PermissionListGroupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListGroupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListGroupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListGroupsResponse.$_createMessage)
    ..pPM<PermissionGroup>(1, _omitFieldNames ? '' : 'groups',
        subBuilder: PermissionGroup.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListGroupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListGroupsResponse copyWith(
          void Function(PermissionListGroupsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListGroupsResponse))
          as PermissionListGroupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListGroupsResponse() / PermissionListGroupsResponse.new instead')
  static PermissionListGroupsResponse create() =>
      PermissionListGroupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListGroupsResponse._();
  @$core.override
  PermissionListGroupsResponse createEmptyInstance() =>
      PermissionListGroupsResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionListGroupsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListGroupsResponse>(
          PermissionListGroupsResponse.$_createMessage);
  static PermissionListGroupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionGroup> get groups => $_getList(0);
}

class PermissionGetGroupRequest extends $pb.GeneratedMessage {
  factory PermissionGetGroupRequest({
    $fixnum.Int64? id,
  }) {
    final result = PermissionGetGroupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionGetGroupRequest._();

  factory PermissionGetGroupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGetGroupRequest()..mergeFromBuffer(data, registry);
  factory PermissionGetGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGetGroupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionGetGroupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionGetGroupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGetGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGetGroupRequest copyWith(
          void Function(PermissionGetGroupRequest) updates) =>
      super.copyWith((message) => updates(message as PermissionGetGroupRequest))
          as PermissionGetGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionGetGroupRequest() / PermissionGetGroupRequest.new instead')
  static PermissionGetGroupRequest create() => PermissionGetGroupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionGetGroupRequest._();
  @$core.override
  PermissionGetGroupRequest createEmptyInstance() =>
      PermissionGetGroupRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionGetGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionGetGroupRequest>(
          PermissionGetGroupRequest.$_createMessage);
  static PermissionGetGroupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionGetGroupResponse extends $pb.GeneratedMessage {
  factory PermissionGetGroupResponse({
    PermissionGroup? group,
  }) {
    final result = PermissionGetGroupResponse._();
    if (group != null) result.group = group;
    return result;
  }

  PermissionGetGroupResponse._();

  factory PermissionGetGroupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGetGroupResponse()..mergeFromBuffer(data, registry);
  factory PermissionGetGroupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionGetGroupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionGetGroupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionGetGroupResponse.$_createMessage)
    ..aOM<PermissionGroup>(1, _omitFieldNames ? '' : 'group',
        subBuilder: PermissionGroup.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGetGroupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionGetGroupResponse copyWith(
          void Function(PermissionGetGroupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionGetGroupResponse))
          as PermissionGetGroupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionGetGroupResponse() / PermissionGetGroupResponse.new instead')
  static PermissionGetGroupResponse create() => PermissionGetGroupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionGetGroupResponse._();
  @$core.override
  PermissionGetGroupResponse createEmptyInstance() =>
      PermissionGetGroupResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionGetGroupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionGetGroupResponse>(
          PermissionGetGroupResponse.$_createMessage);
  static PermissionGetGroupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PermissionGroup get group => $_getN(0);
  @$pb.TagNumber(1)
  set group(PermissionGroup value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => $_clearField(1);
  @$pb.TagNumber(1)
  PermissionGroup ensureGroup() => $_ensure(0);
}

class PermissionCreateGroupRequest extends $pb.GeneratedMessage {
  factory PermissionCreateGroupRequest({
    $core.String? name,
    $core.Iterable<$core.String>? actions,
  }) {
    final result = PermissionCreateGroupRequest._();
    if (name != null) result.name = name;
    if (actions != null) result.actions.addAll(actions);
    return result;
  }

  PermissionCreateGroupRequest._();

  factory PermissionCreateGroupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateGroupRequest()..mergeFromBuffer(data, registry);
  factory PermissionCreateGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateGroupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionCreateGroupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionCreateGroupRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..pPS(2, _omitFieldNames ? '' : 'actions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateGroupRequest copyWith(
          void Function(PermissionCreateGroupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionCreateGroupRequest))
          as PermissionCreateGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionCreateGroupRequest() / PermissionCreateGroupRequest.new instead')
  static PermissionCreateGroupRequest create() =>
      PermissionCreateGroupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionCreateGroupRequest._();
  @$core.override
  PermissionCreateGroupRequest createEmptyInstance() =>
      PermissionCreateGroupRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionCreateGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionCreateGroupRequest>(
          PermissionCreateGroupRequest.$_createMessage);
  static PermissionCreateGroupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get actions => $_getList(1);
}

class PermissionCreateGroupResponse extends $pb.GeneratedMessage {
  factory PermissionCreateGroupResponse({
    $fixnum.Int64? id,
  }) {
    final result = PermissionCreateGroupResponse._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionCreateGroupResponse._();

  factory PermissionCreateGroupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateGroupResponse()..mergeFromBuffer(data, registry);
  factory PermissionCreateGroupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateGroupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionCreateGroupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionCreateGroupResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateGroupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateGroupResponse copyWith(
          void Function(PermissionCreateGroupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionCreateGroupResponse))
          as PermissionCreateGroupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionCreateGroupResponse() / PermissionCreateGroupResponse.new instead')
  static PermissionCreateGroupResponse create() =>
      PermissionCreateGroupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionCreateGroupResponse._();
  @$core.override
  PermissionCreateGroupResponse createEmptyInstance() =>
      PermissionCreateGroupResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionCreateGroupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionCreateGroupResponse>(
          PermissionCreateGroupResponse.$_createMessage);
  static PermissionCreateGroupResponse? _defaultInstance;

  /// The create endpoint only echoes the new row id.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionUpdateGroupRequest extends $pb.GeneratedMessage {
  factory PermissionUpdateGroupRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.Iterable<$core.String>? actions,
  }) {
    final result = PermissionUpdateGroupRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (actions != null) result.actions.addAll(actions);
    return result;
  }

  PermissionUpdateGroupRequest._();

  factory PermissionUpdateGroupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUpdateGroupRequest()..mergeFromBuffer(data, registry);
  factory PermissionUpdateGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUpdateGroupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionUpdateGroupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionUpdateGroupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..pPS(3, _omitFieldNames ? '' : 'actions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUpdateGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUpdateGroupRequest copyWith(
          void Function(PermissionUpdateGroupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionUpdateGroupRequest))
          as PermissionUpdateGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionUpdateGroupRequest() / PermissionUpdateGroupRequest.new instead')
  static PermissionUpdateGroupRequest create() =>
      PermissionUpdateGroupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionUpdateGroupRequest._();
  @$core.override
  PermissionUpdateGroupRequest createEmptyInstance() =>
      PermissionUpdateGroupRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionUpdateGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionUpdateGroupRequest>(
          PermissionUpdateGroupRequest.$_createMessage);
  static PermissionUpdateGroupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// Full replacement of the group's action list.
  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get actions => $_getList(2);
}

class PermissionDeleteGroupRequest extends $pb.GeneratedMessage {
  factory PermissionDeleteGroupRequest({
    $fixnum.Int64? id,
  }) {
    final result = PermissionDeleteGroupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionDeleteGroupRequest._();

  factory PermissionDeleteGroupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionDeleteGroupRequest()..mergeFromBuffer(data, registry);
  factory PermissionDeleteGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionDeleteGroupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionDeleteGroupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionDeleteGroupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionDeleteGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionDeleteGroupRequest copyWith(
          void Function(PermissionDeleteGroupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionDeleteGroupRequest))
          as PermissionDeleteGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionDeleteGroupRequest() / PermissionDeleteGroupRequest.new instead')
  static PermissionDeleteGroupRequest create() =>
      PermissionDeleteGroupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionDeleteGroupRequest._();
  @$core.override
  PermissionDeleteGroupRequest createEmptyInstance() =>
      PermissionDeleteGroupRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionDeleteGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionDeleteGroupRequest>(
          PermissionDeleteGroupRequest.$_createMessage);
  static PermissionDeleteGroupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionListPoliciesRequest extends $pb.GeneratedMessage {
  factory PermissionListPoliciesRequest() => PermissionListPoliciesRequest._();

  PermissionListPoliciesRequest._();

  factory PermissionListPoliciesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListPoliciesRequest()..mergeFromBuffer(data, registry);
  factory PermissionListPoliciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListPoliciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListPoliciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListPoliciesRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListPoliciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListPoliciesRequest copyWith(
          void Function(PermissionListPoliciesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListPoliciesRequest))
          as PermissionListPoliciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListPoliciesRequest() / PermissionListPoliciesRequest.new instead')
  static PermissionListPoliciesRequest create() =>
      PermissionListPoliciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListPoliciesRequest._();
  @$core.override
  PermissionListPoliciesRequest createEmptyInstance() =>
      PermissionListPoliciesRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionListPoliciesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListPoliciesRequest>(
          PermissionListPoliciesRequest.$_createMessage);
  static PermissionListPoliciesRequest? _defaultInstance;
}

class PermissionListPoliciesResponse extends $pb.GeneratedMessage {
  factory PermissionListPoliciesResponse({
    $core.Iterable<PermissionPolicy>? policies,
  }) {
    final result = PermissionListPoliciesResponse._();
    if (policies != null) result.policies.addAll(policies);
    return result;
  }

  PermissionListPoliciesResponse._();

  factory PermissionListPoliciesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListPoliciesResponse()..mergeFromBuffer(data, registry);
  factory PermissionListPoliciesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListPoliciesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListPoliciesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListPoliciesResponse.$_createMessage)
    ..pPM<PermissionPolicy>(1, _omitFieldNames ? '' : 'policies',
        subBuilder: PermissionPolicy.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListPoliciesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListPoliciesResponse copyWith(
          void Function(PermissionListPoliciesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListPoliciesResponse))
          as PermissionListPoliciesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListPoliciesResponse() / PermissionListPoliciesResponse.new instead')
  static PermissionListPoliciesResponse create() =>
      PermissionListPoliciesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListPoliciesResponse._();
  @$core.override
  PermissionListPoliciesResponse createEmptyInstance() =>
      PermissionListPoliciesResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionListPoliciesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListPoliciesResponse>(
          PermissionListPoliciesResponse.$_createMessage);
  static PermissionListPoliciesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionPolicy> get policies => $_getList(0);
}

class GetMyPermissionsRequest extends $pb.GeneratedMessage {
  factory GetMyPermissionsRequest() => GetMyPermissionsRequest._();

  GetMyPermissionsRequest._();

  factory GetMyPermissionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetMyPermissionsRequest()..mergeFromBuffer(data, registry);
  factory GetMyPermissionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetMyPermissionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetMyPermissionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetMyPermissionsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMyPermissionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMyPermissionsRequest copyWith(
          void Function(GetMyPermissionsRequest) updates) =>
      super.copyWith((message) => updates(message as GetMyPermissionsRequest))
          as GetMyPermissionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetMyPermissionsRequest() / GetMyPermissionsRequest.new instead')
  static GetMyPermissionsRequest create() => GetMyPermissionsRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetMyPermissionsRequest._();
  @$core.override
  GetMyPermissionsRequest createEmptyInstance() => GetMyPermissionsRequest._();
  @$core.pragma('dart2js:noInline')
  static GetMyPermissionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetMyPermissionsRequest>(
          GetMyPermissionsRequest.$_createMessage);
  static GetMyPermissionsRequest? _defaultInstance;
}

class GetMyPermissionsResponse extends $pb.GeneratedMessage {
  factory GetMyPermissionsResponse({
    PermissionUserRole? role,
    $fixnum.Int64? organizationId,
    $core.Iterable<$core.String>? actions,
  }) {
    final result = GetMyPermissionsResponse._();
    if (role != null) result.role = role;
    if (organizationId != null) result.organizationId = organizationId;
    if (actions != null) result.actions.addAll(actions);
    return result;
  }

  GetMyPermissionsResponse._();

  factory GetMyPermissionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetMyPermissionsResponse()..mergeFromBuffer(data, registry);
  factory GetMyPermissionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetMyPermissionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetMyPermissionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetMyPermissionsResponse.$_createMessage)
    ..aE<PermissionUserRole>(1, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..pPS(3, _omitFieldNames ? '' : 'actions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMyPermissionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMyPermissionsResponse copyWith(
          void Function(GetMyPermissionsResponse) updates) =>
      super.copyWith((message) => updates(message as GetMyPermissionsResponse))
          as GetMyPermissionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetMyPermissionsResponse() / GetMyPermissionsResponse.new instead')
  static GetMyPermissionsResponse create() => GetMyPermissionsResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetMyPermissionsResponse._();
  @$core.override
  GetMyPermissionsResponse createEmptyInstance() =>
      GetMyPermissionsResponse._();
  @$core.pragma('dart2js:noInline')
  static GetMyPermissionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetMyPermissionsResponse>(
          GetMyPermissionsResponse.$_createMessage);
  static GetMyPermissionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PermissionUserRole get role => $_getN(0);
  @$pb.TagNumber(1)
  set role(PermissionUserRole value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  /// Effective action strings; `*` for owners/admins.
  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get actions => $_getList(2);
}

class PermissionAssignMemberGroupRequest extends $pb.GeneratedMessage {
  factory PermissionAssignMemberGroupRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? groupId,
  }) {
    final result = PermissionAssignMemberGroupRequest._();
    if (id != null) result.id = id;
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  PermissionAssignMemberGroupRequest._();

  factory PermissionAssignMemberGroupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAssignMemberGroupRequest()..mergeFromBuffer(data, registry);
  factory PermissionAssignMemberGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAssignMemberGroupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionAssignMemberGroupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionAssignMemberGroupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'groupId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAssignMemberGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAssignMemberGroupRequest copyWith(
          void Function(PermissionAssignMemberGroupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as PermissionAssignMemberGroupRequest))
          as PermissionAssignMemberGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionAssignMemberGroupRequest() / PermissionAssignMemberGroupRequest.new instead')
  static PermissionAssignMemberGroupRequest create() =>
      PermissionAssignMemberGroupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionAssignMemberGroupRequest._();
  @$core.override
  PermissionAssignMemberGroupRequest createEmptyInstance() =>
      PermissionAssignMemberGroupRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionAssignMemberGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionAssignMemberGroupRequest>(
          PermissionAssignMemberGroupRequest.$_createMessage);
  static PermissionAssignMemberGroupRequest? _defaultInstance;

  /// User id of the member whose group assignment changes.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// New permission group; 0 detaches the member from every group (role-only access).
  @$pb.TagNumber(2)
  $fixnum.Int64 get groupId => $_getI64(1);
  @$pb.TagNumber(2)
  set groupId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => $_clearField(2);
}

class PermissionSetGroupActiveRequest extends $pb.GeneratedMessage {
  factory PermissionSetGroupActiveRequest({
    $fixnum.Int64? id,
    $core.bool? isActive,
  }) {
    final result = PermissionSetGroupActiveRequest._();
    if (id != null) result.id = id;
    if (isActive != null) result.isActive = isActive;
    return result;
  }

  PermissionSetGroupActiveRequest._();

  factory PermissionSetGroupActiveRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionSetGroupActiveRequest()..mergeFromBuffer(data, registry);
  factory PermissionSetGroupActiveRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionSetGroupActiveRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionSetGroupActiveRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionSetGroupActiveRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'isActive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionSetGroupActiveRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionSetGroupActiveRequest copyWith(
          void Function(PermissionSetGroupActiveRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionSetGroupActiveRequest))
          as PermissionSetGroupActiveRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionSetGroupActiveRequest() / PermissionSetGroupActiveRequest.new instead')
  static PermissionSetGroupActiveRequest create() =>
      PermissionSetGroupActiveRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionSetGroupActiveRequest._();
  @$core.override
  PermissionSetGroupActiveRequest createEmptyInstance() =>
      PermissionSetGroupActiveRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionSetGroupActiveRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionSetGroupActiveRequest>(
          PermissionSetGroupActiveRequest.$_createMessage);
  static PermissionSetGroupActiveRequest? _defaultInstance;

  /// Permission group id to enable or disable.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get isActive => $_getBF(1);
  @$pb.TagNumber(2)
  set isActive($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasIsActive() => $_has(1);
  @$pb.TagNumber(2)
  void clearIsActive() => $_clearField(2);
}

class PermissionListUserPoliciesRequest extends $pb.GeneratedMessage {
  factory PermissionListUserPoliciesRequest({
    $fixnum.Int64? id,
  }) {
    final result = PermissionListUserPoliciesRequest._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionListUserPoliciesRequest._();

  factory PermissionListUserPoliciesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListUserPoliciesRequest()..mergeFromBuffer(data, registry);
  factory PermissionListUserPoliciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListUserPoliciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListUserPoliciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListUserPoliciesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListUserPoliciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListUserPoliciesRequest copyWith(
          void Function(PermissionListUserPoliciesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as PermissionListUserPoliciesRequest))
          as PermissionListUserPoliciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListUserPoliciesRequest() / PermissionListUserPoliciesRequest.new instead')
  static PermissionListUserPoliciesRequest create() =>
      PermissionListUserPoliciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListUserPoliciesRequest._();
  @$core.override
  PermissionListUserPoliciesRequest createEmptyInstance() =>
      PermissionListUserPoliciesRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionListUserPoliciesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListUserPoliciesRequest>(
          PermissionListUserPoliciesRequest.$_createMessage);
  static PermissionListUserPoliciesRequest? _defaultInstance;

  /// User id whose policy overrides to read.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionListUserPoliciesResponse extends $pb.GeneratedMessage {
  factory PermissionListUserPoliciesResponse({
    $core.Iterable<PermissionUserPolicyEntry>? policies,
  }) {
    final result = PermissionListUserPoliciesResponse._();
    if (policies != null) result.policies.addAll(policies);
    return result;
  }

  PermissionListUserPoliciesResponse._();

  factory PermissionListUserPoliciesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListUserPoliciesResponse()..mergeFromBuffer(data, registry);
  factory PermissionListUserPoliciesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListUserPoliciesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListUserPoliciesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListUserPoliciesResponse.$_createMessage)
    ..pPM<PermissionUserPolicyEntry>(1, _omitFieldNames ? '' : 'policies',
        subBuilder: PermissionUserPolicyEntry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListUserPoliciesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListUserPoliciesResponse copyWith(
          void Function(PermissionListUserPoliciesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as PermissionListUserPoliciesResponse))
          as PermissionListUserPoliciesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListUserPoliciesResponse() / PermissionListUserPoliciesResponse.new instead')
  static PermissionListUserPoliciesResponse create() =>
      PermissionListUserPoliciesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListUserPoliciesResponse._();
  @$core.override
  PermissionListUserPoliciesResponse createEmptyInstance() =>
      PermissionListUserPoliciesResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionListUserPoliciesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListUserPoliciesResponse>(
          PermissionListUserPoliciesResponse.$_createMessage);
  static PermissionListUserPoliciesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionUserPolicyEntry> get policies => $_getList(0);
}

class PermissionListMembersRequest extends $pb.GeneratedMessage {
  factory PermissionListMembersRequest() => PermissionListMembersRequest._();

  PermissionListMembersRequest._();

  factory PermissionListMembersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListMembersRequest()..mergeFromBuffer(data, registry);
  factory PermissionListMembersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListMembersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListMembersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListMembersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListMembersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListMembersRequest copyWith(
          void Function(PermissionListMembersRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListMembersRequest))
          as PermissionListMembersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListMembersRequest() / PermissionListMembersRequest.new instead')
  static PermissionListMembersRequest create() =>
      PermissionListMembersRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListMembersRequest._();
  @$core.override
  PermissionListMembersRequest createEmptyInstance() =>
      PermissionListMembersRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionListMembersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListMembersRequest>(
          PermissionListMembersRequest.$_createMessage);
  static PermissionListMembersRequest? _defaultInstance;
}

class PermissionListMembersResponse extends $pb.GeneratedMessage {
  factory PermissionListMembersResponse({
    $core.Iterable<PermissionMember>? members,
  }) {
    final result = PermissionListMembersResponse._();
    if (members != null) result.members.addAll(members);
    return result;
  }

  PermissionListMembersResponse._();

  factory PermissionListMembersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListMembersResponse()..mergeFromBuffer(data, registry);
  factory PermissionListMembersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListMembersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListMembersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListMembersResponse.$_createMessage)
    ..pPM<PermissionMember>(1, _omitFieldNames ? '' : 'members',
        subBuilder: PermissionMember.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListMembersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListMembersResponse copyWith(
          void Function(PermissionListMembersResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListMembersResponse))
          as PermissionListMembersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListMembersResponse() / PermissionListMembersResponse.new instead')
  static PermissionListMembersResponse create() =>
      PermissionListMembersResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListMembersResponse._();
  @$core.override
  PermissionListMembersResponse createEmptyInstance() =>
      PermissionListMembersResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionListMembersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListMembersResponse>(
          PermissionListMembersResponse.$_createMessage);
  static PermissionListMembersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionMember> get members => $_getList(0);
}

class PermissionAddMemberRequest extends $pb.GeneratedMessage {
  factory PermissionAddMemberRequest({
    $core.String? email,
    $core.String? password,
    PermissionUserRole? role,
    $fixnum.Int64? groupId,
  }) {
    final result = PermissionAddMemberRequest._();
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    if (role != null) result.role = role;
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  PermissionAddMemberRequest._();

  factory PermissionAddMemberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAddMemberRequest()..mergeFromBuffer(data, registry);
  factory PermissionAddMemberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAddMemberRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionAddMemberRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionAddMemberRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..aE<PermissionUserRole>(3, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..aInt64(4, _omitFieldNames ? '' : 'groupId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAddMemberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAddMemberRequest copyWith(
          void Function(PermissionAddMemberRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionAddMemberRequest))
          as PermissionAddMemberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionAddMemberRequest() / PermissionAddMemberRequest.new instead')
  static PermissionAddMemberRequest create() => PermissionAddMemberRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionAddMemberRequest._();
  @$core.override
  PermissionAddMemberRequest createEmptyInstance() =>
      PermissionAddMemberRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionAddMemberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionAddMemberRequest>(
          PermissionAddMemberRequest.$_createMessage);
  static PermissionAddMemberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);

  @$pb.TagNumber(3)
  PermissionUserRole get role => $_getN(2);
  @$pb.TagNumber(3)
  set role(PermissionUserRole value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearRole() => $_clearField(3);

  /// Permission group for the new membership; access comes from the role only when absent.
  @$pb.TagNumber(4)
  $fixnum.Int64 get groupId => $_getI64(3);
  @$pb.TagNumber(4)
  set groupId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasGroupId() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupId() => $_clearField(4);
}

class PermissionUpdateMemberRoleRequest extends $pb.GeneratedMessage {
  factory PermissionUpdateMemberRoleRequest({
    $fixnum.Int64? id,
    PermissionUserRole? role,
  }) {
    final result = PermissionUpdateMemberRoleRequest._();
    if (id != null) result.id = id;
    if (role != null) result.role = role;
    return result;
  }

  PermissionUpdateMemberRoleRequest._();

  factory PermissionUpdateMemberRoleRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUpdateMemberRoleRequest()..mergeFromBuffer(data, registry);
  factory PermissionUpdateMemberRoleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionUpdateMemberRoleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionUpdateMemberRoleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionUpdateMemberRoleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<PermissionUserRole>(2, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUpdateMemberRoleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionUpdateMemberRoleRequest copyWith(
          void Function(PermissionUpdateMemberRoleRequest) updates) =>
      super.copyWith((message) =>
              updates(message as PermissionUpdateMemberRoleRequest))
          as PermissionUpdateMemberRoleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionUpdateMemberRoleRequest() / PermissionUpdateMemberRoleRequest.new instead')
  static PermissionUpdateMemberRoleRequest create() =>
      PermissionUpdateMemberRoleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionUpdateMemberRoleRequest._();
  @$core.override
  PermissionUpdateMemberRoleRequest createEmptyInstance() =>
      PermissionUpdateMemberRoleRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionUpdateMemberRoleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionUpdateMemberRoleRequest>(
          PermissionUpdateMemberRoleRequest.$_createMessage);
  static PermissionUpdateMemberRoleRequest? _defaultInstance;

  /// User id of the member.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  PermissionUserRole get role => $_getN(1);
  @$pb.TagNumber(2)
  set role(PermissionUserRole value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);
}

class PermissionRemoveMemberRequest extends $pb.GeneratedMessage {
  factory PermissionRemoveMemberRequest({
    $fixnum.Int64? id,
  }) {
    final result = PermissionRemoveMemberRequest._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionRemoveMemberRequest._();

  factory PermissionRemoveMemberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionRemoveMemberRequest()..mergeFromBuffer(data, registry);
  factory PermissionRemoveMemberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionRemoveMemberRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionRemoveMemberRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionRemoveMemberRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionRemoveMemberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionRemoveMemberRequest copyWith(
          void Function(PermissionRemoveMemberRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionRemoveMemberRequest))
          as PermissionRemoveMemberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionRemoveMemberRequest() / PermissionRemoveMemberRequest.new instead')
  static PermissionRemoveMemberRequest create() =>
      PermissionRemoveMemberRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionRemoveMemberRequest._();
  @$core.override
  PermissionRemoveMemberRequest createEmptyInstance() =>
      PermissionRemoveMemberRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionRemoveMemberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionRemoveMemberRequest>(
          PermissionRemoveMemberRequest.$_createMessage);
  static PermissionRemoveMemberRequest? _defaultInstance;

  /// User id of the member.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class PermissionListInvitesRequest extends $pb.GeneratedMessage {
  factory PermissionListInvitesRequest() => PermissionListInvitesRequest._();

  PermissionListInvitesRequest._();

  factory PermissionListInvitesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListInvitesRequest()..mergeFromBuffer(data, registry);
  factory PermissionListInvitesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListInvitesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListInvitesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListInvitesRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListInvitesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListInvitesRequest copyWith(
          void Function(PermissionListInvitesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListInvitesRequest))
          as PermissionListInvitesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListInvitesRequest() / PermissionListInvitesRequest.new instead')
  static PermissionListInvitesRequest create() =>
      PermissionListInvitesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListInvitesRequest._();
  @$core.override
  PermissionListInvitesRequest createEmptyInstance() =>
      PermissionListInvitesRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionListInvitesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListInvitesRequest>(
          PermissionListInvitesRequest.$_createMessage);
  static PermissionListInvitesRequest? _defaultInstance;
}

class PermissionListInvitesResponse extends $pb.GeneratedMessage {
  factory PermissionListInvitesResponse({
    $core.Iterable<PermissionInvite>? invites,
  }) {
    final result = PermissionListInvitesResponse._();
    if (invites != null) result.invites.addAll(invites);
    return result;
  }

  PermissionListInvitesResponse._();

  factory PermissionListInvitesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListInvitesResponse()..mergeFromBuffer(data, registry);
  factory PermissionListInvitesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionListInvitesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionListInvitesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionListInvitesResponse.$_createMessage)
    ..pPM<PermissionInvite>(1, _omitFieldNames ? '' : 'invites',
        subBuilder: PermissionInvite.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListInvitesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionListInvitesResponse copyWith(
          void Function(PermissionListInvitesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionListInvitesResponse))
          as PermissionListInvitesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionListInvitesResponse() / PermissionListInvitesResponse.new instead')
  static PermissionListInvitesResponse create() =>
      PermissionListInvitesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionListInvitesResponse._();
  @$core.override
  PermissionListInvitesResponse createEmptyInstance() =>
      PermissionListInvitesResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionListInvitesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionListInvitesResponse>(
          PermissionListInvitesResponse.$_createMessage);
  static PermissionListInvitesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PermissionInvite> get invites => $_getList(0);
}

class PermissionCreateInviteRequest extends $pb.GeneratedMessage {
  factory PermissionCreateInviteRequest({
    $core.String? email,
    PermissionUserRole? role,
    $fixnum.Int64? groupId,
  }) {
    final result = PermissionCreateInviteRequest._();
    if (email != null) result.email = email;
    if (role != null) result.role = role;
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  PermissionCreateInviteRequest._();

  factory PermissionCreateInviteRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateInviteRequest()..mergeFromBuffer(data, registry);
  factory PermissionCreateInviteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateInviteRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionCreateInviteRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionCreateInviteRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aE<PermissionUserRole>(2, _omitFieldNames ? '' : 'role',
        enumValues: PermissionUserRole.values)
    ..aInt64(3, _omitFieldNames ? '' : 'groupId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateInviteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateInviteRequest copyWith(
          void Function(PermissionCreateInviteRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionCreateInviteRequest))
          as PermissionCreateInviteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionCreateInviteRequest() / PermissionCreateInviteRequest.new instead')
  static PermissionCreateInviteRequest create() =>
      PermissionCreateInviteRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionCreateInviteRequest._();
  @$core.override
  PermissionCreateInviteRequest createEmptyInstance() =>
      PermissionCreateInviteRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionCreateInviteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionCreateInviteRequest>(
          PermissionCreateInviteRequest.$_createMessage);
  static PermissionCreateInviteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  PermissionUserRole get role => $_getN(1);
  @$pb.TagNumber(2)
  set role(PermissionUserRole value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);

  /// Permission group carried into the membership on accept; 0 means role-only access.
  @$pb.TagNumber(3)
  $fixnum.Int64 get groupId => $_getI64(2);
  @$pb.TagNumber(3)
  set groupId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasGroupId() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupId() => $_clearField(3);
}

class PermissionCreateInviteResponse extends $pb.GeneratedMessage {
  factory PermissionCreateInviteResponse({
    $core.String? token,
  }) {
    final result = PermissionCreateInviteResponse._();
    if (token != null) result.token = token;
    return result;
  }

  PermissionCreateInviteResponse._();

  factory PermissionCreateInviteResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateInviteResponse()..mergeFromBuffer(data, registry);
  factory PermissionCreateInviteResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCreateInviteResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionCreateInviteResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionCreateInviteResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateInviteResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCreateInviteResponse copyWith(
          void Function(PermissionCreateInviteResponse) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionCreateInviteResponse))
          as PermissionCreateInviteResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionCreateInviteResponse() / PermissionCreateInviteResponse.new instead')
  static PermissionCreateInviteResponse create() =>
      PermissionCreateInviteResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionCreateInviteResponse._();
  @$core.override
  PermissionCreateInviteResponse createEmptyInstance() =>
      PermissionCreateInviteResponse._();
  @$core.pragma('dart2js:noInline')
  static PermissionCreateInviteResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionCreateInviteResponse>(
          PermissionCreateInviteResponse.$_createMessage);
  static PermissionCreateInviteResponse? _defaultInstance;

  /// Opaque invitation token to be delivered to the invitee.
  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);
}

class PermissionAcceptInviteRequest extends $pb.GeneratedMessage {
  factory PermissionAcceptInviteRequest({
    $core.String? token,
  }) {
    final result = PermissionAcceptInviteRequest._();
    if (token != null) result.token = token;
    return result;
  }

  PermissionAcceptInviteRequest._();

  factory PermissionAcceptInviteRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAcceptInviteRequest()..mergeFromBuffer(data, registry);
  factory PermissionAcceptInviteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionAcceptInviteRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionAcceptInviteRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionAcceptInviteRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAcceptInviteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionAcceptInviteRequest copyWith(
          void Function(PermissionAcceptInviteRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionAcceptInviteRequest))
          as PermissionAcceptInviteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionAcceptInviteRequest() / PermissionAcceptInviteRequest.new instead')
  static PermissionAcceptInviteRequest create() =>
      PermissionAcceptInviteRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionAcceptInviteRequest._();
  @$core.override
  PermissionAcceptInviteRequest createEmptyInstance() =>
      PermissionAcceptInviteRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionAcceptInviteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionAcceptInviteRequest>(
          PermissionAcceptInviteRequest.$_createMessage);
  static PermissionAcceptInviteRequest? _defaultInstance;

  /// Opaque invitation token; no row id is carried.
  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);
}

class PermissionCancelInviteRequest extends $pb.GeneratedMessage {
  factory PermissionCancelInviteRequest({
    $fixnum.Int64? id,
  }) {
    final result = PermissionCancelInviteRequest._();
    if (id != null) result.id = id;
    return result;
  }

  PermissionCancelInviteRequest._();

  factory PermissionCancelInviteRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCancelInviteRequest()..mergeFromBuffer(data, registry);
  factory PermissionCancelInviteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionCancelInviteRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionCancelInviteRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionCancelInviteRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCancelInviteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionCancelInviteRequest copyWith(
          void Function(PermissionCancelInviteRequest) updates) =>
      super.copyWith(
              (message) => updates(message as PermissionCancelInviteRequest))
          as PermissionCancelInviteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionCancelInviteRequest() / PermissionCancelInviteRequest.new instead')
  static PermissionCancelInviteRequest create() =>
      PermissionCancelInviteRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionCancelInviteRequest._();
  @$core.override
  PermissionCancelInviteRequest createEmptyInstance() =>
      PermissionCancelInviteRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionCancelInviteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PermissionCancelInviteRequest>(
          PermissionCancelInviteRequest.$_createMessage);
  static PermissionCancelInviteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Full replacement of the target user's policy override list.
class PermissionReplaceUserPoliciesRequest extends $pb.GeneratedMessage {
  factory PermissionReplaceUserPoliciesRequest({
    $fixnum.Int64? id,
    $core.Iterable<PermissionUserPolicyEntry>? policies,
  }) {
    final result = PermissionReplaceUserPoliciesRequest._();
    if (id != null) result.id = id;
    if (policies != null) result.policies.addAll(policies);
    return result;
  }

  PermissionReplaceUserPoliciesRequest._();

  factory PermissionReplaceUserPoliciesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionReplaceUserPoliciesRequest()..mergeFromBuffer(data, registry);
  factory PermissionReplaceUserPoliciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PermissionReplaceUserPoliciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PermissionReplaceUserPoliciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PermissionReplaceUserPoliciesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..pPM<PermissionUserPolicyEntry>(2, _omitFieldNames ? '' : 'policies',
        subBuilder: PermissionUserPolicyEntry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionReplaceUserPoliciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PermissionReplaceUserPoliciesRequest copyWith(
          void Function(PermissionReplaceUserPoliciesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as PermissionReplaceUserPoliciesRequest))
          as PermissionReplaceUserPoliciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PermissionReplaceUserPoliciesRequest() / PermissionReplaceUserPoliciesRequest.new instead')
  static PermissionReplaceUserPoliciesRequest create() =>
      PermissionReplaceUserPoliciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      PermissionReplaceUserPoliciesRequest._();
  @$core.override
  PermissionReplaceUserPoliciesRequest createEmptyInstance() =>
      PermissionReplaceUserPoliciesRequest._();
  @$core.pragma('dart2js:noInline')
  static PermissionReplaceUserPoliciesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              PermissionReplaceUserPoliciesRequest>(
          PermissionReplaceUserPoliciesRequest.$_createMessage);
  static PermissionReplaceUserPoliciesRequest? _defaultInstance;

  /// User id whose policies are replaced.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<PermissionUserPolicyEntry> get policies => $_getList(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

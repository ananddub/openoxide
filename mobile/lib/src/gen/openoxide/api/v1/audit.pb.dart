// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/audit.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class AuditLog extends $pb.GeneratedMessage {
  factory AuditLog({
    $fixnum.Int64? id,
    $core.String? userEmail,
    $core.String? userRole,
    $core.String? action,
    $core.String? resourceType,
    $core.String? resourceId,
    $core.String? resourceName,
    $core.String? metadata,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? userId,
    $fixnum.Int64? createdAt,
  }) {
    final result = AuditLog._();
    if (id != null) result.id = id;
    if (userEmail != null) result.userEmail = userEmail;
    if (userRole != null) result.userRole = userRole;
    if (action != null) result.action = action;
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    if (resourceName != null) result.resourceName = resourceName;
    if (metadata != null) result.metadata = metadata;
    if (organizationId != null) result.organizationId = organizationId;
    if (userId != null) result.userId = userId;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  AuditLog._();

  factory AuditLog.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditLog()..mergeFromBuffer(data, registry);
  factory AuditLog.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditLog()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuditLog',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuditLog.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'userEmail')
    ..aOS(3, _omitFieldNames ? '' : 'userRole')
    ..aOS(4, _omitFieldNames ? '' : 'action')
    ..aOS(5, _omitFieldNames ? '' : 'resourceType')
    ..aOS(6, _omitFieldNames ? '' : 'resourceId')
    ..aOS(7, _omitFieldNames ? '' : 'resourceName')
    ..aOS(8, _omitFieldNames ? '' : 'metadata')
    ..aInt64(9, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(10, _omitFieldNames ? '' : 'userId')
    ..aInt64(11, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditLog clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditLog copyWith(void Function(AuditLog) updates) =>
      super.copyWith((message) => updates(message as AuditLog)) as AuditLog;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AuditLog() / AuditLog.new instead')
  static AuditLog create() => AuditLog._();
  static $pb.GeneratedMessage $_createMessage() => AuditLog._();
  @$core.override
  AuditLog createEmptyInstance() => AuditLog._();
  @$core.pragma('dart2js:noInline')
  static AuditLog getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuditLog>(AuditLog.$_createMessage);
  static AuditLog? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get userEmail => $_getSZ(1);
  @$pb.TagNumber(2)
  set userEmail($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserEmail() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserEmail() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get userRole => $_getSZ(2);
  @$pb.TagNumber(3)
  set userRole($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUserRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserRole() => $_clearField(3);

  /// Performed action verb (not a closed set; written by callers).
  @$pb.TagNumber(4)
  $core.String get action => $_getSZ(3);
  @$pb.TagNumber(4)
  set action($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAction() => $_has(3);
  @$pb.TagNumber(4)
  void clearAction() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get resourceType => $_getSZ(4);
  @$pb.TagNumber(5)
  set resourceType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasResourceType() => $_has(4);
  @$pb.TagNumber(5)
  void clearResourceType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get resourceId => $_getSZ(5);
  @$pb.TagNumber(6)
  set resourceId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasResourceId() => $_has(5);
  @$pb.TagNumber(6)
  void clearResourceId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get resourceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set resourceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasResourceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearResourceName() => $_clearField(7);

  /// Opaque JSON audit context.
  @$pb.TagNumber(8)
  $core.String get metadata => $_getSZ(7);
  @$pb.TagNumber(8)
  set metadata($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasMetadata() => $_has(7);
  @$pb.TagNumber(8)
  void clearMetadata() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get organizationId => $_getI64(8);
  @$pb.TagNumber(9)
  set organizationId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOrganizationId() => $_has(8);
  @$pb.TagNumber(9)
  void clearOrganizationId() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get userId => $_getI64(9);
  @$pb.TagNumber(10)
  set userId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUserId() => $_has(9);
  @$pb.TagNumber(10)
  void clearUserId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get createdAt => $_getI64(10);
  @$pb.TagNumber(11)
  set createdAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCreatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCreatedAt() => $_clearField(11);
}

/// `action` narrows the list to one action verb; absent means all actions.
class AuditListAuditLogsRequest extends $pb.GeneratedMessage {
  factory AuditListAuditLogsRequest({
    $fixnum.Int64? organizationId,
    $core.String? action,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = AuditListAuditLogsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (action != null) result.action = action;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  AuditListAuditLogsRequest._();

  factory AuditListAuditLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditListAuditLogsRequest()..mergeFromBuffer(data, registry);
  factory AuditListAuditLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditListAuditLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuditListAuditLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuditListAuditLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aOS(2, _omitFieldNames ? '' : 'action')
    ..aInt64(3, _omitFieldNames ? '' : 'limit')
    ..aInt64(4, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditListAuditLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditListAuditLogsRequest copyWith(
          void Function(AuditListAuditLogsRequest) updates) =>
      super.copyWith((message) => updates(message as AuditListAuditLogsRequest))
          as AuditListAuditLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AuditListAuditLogsRequest() / AuditListAuditLogsRequest.new instead')
  static AuditListAuditLogsRequest create() => AuditListAuditLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AuditListAuditLogsRequest._();
  @$core.override
  AuditListAuditLogsRequest createEmptyInstance() =>
      AuditListAuditLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static AuditListAuditLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuditListAuditLogsRequest>(
          AuditListAuditLogsRequest.$_createMessage);
  static AuditListAuditLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get action => $_getSZ(1);
  @$pb.TagNumber(2)
  set action($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get limit => $_getI64(2);
  @$pb.TagNumber(3)
  set limit($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLimit() => $_has(2);
  @$pb.TagNumber(3)
  void clearLimit() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get offset => $_getI64(3);
  @$pb.TagNumber(4)
  set offset($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOffset() => $_has(3);
  @$pb.TagNumber(4)
  void clearOffset() => $_clearField(4);
}

class AuditListAuditLogsResponse extends $pb.GeneratedMessage {
  factory AuditListAuditLogsResponse({
    $core.Iterable<AuditLog>? logs,
  }) {
    final result = AuditListAuditLogsResponse._();
    if (logs != null) result.logs.addAll(logs);
    return result;
  }

  AuditListAuditLogsResponse._();

  factory AuditListAuditLogsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditListAuditLogsResponse()..mergeFromBuffer(data, registry);
  factory AuditListAuditLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditListAuditLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuditListAuditLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuditListAuditLogsResponse.$_createMessage)
    ..pPM<AuditLog>(1, _omitFieldNames ? '' : 'logs',
        subBuilder: AuditLog.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditListAuditLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditListAuditLogsResponse copyWith(
          void Function(AuditListAuditLogsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as AuditListAuditLogsResponse))
          as AuditListAuditLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AuditListAuditLogsResponse() / AuditListAuditLogsResponse.new instead')
  static AuditListAuditLogsResponse create() => AuditListAuditLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AuditListAuditLogsResponse._();
  @$core.override
  AuditListAuditLogsResponse createEmptyInstance() =>
      AuditListAuditLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static AuditListAuditLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuditListAuditLogsResponse>(
          AuditListAuditLogsResponse.$_createMessage);
  static AuditListAuditLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AuditLog> get logs => $_getList(0);
}

/// Export is organization-scoped from the authenticated session and capped server-side at 500 rows.
class AuditExportAuditLogsRequest extends $pb.GeneratedMessage {
  factory AuditExportAuditLogsRequest({
    $core.String? action,
  }) {
    final result = AuditExportAuditLogsRequest._();
    if (action != null) result.action = action;
    return result;
  }

  AuditExportAuditLogsRequest._();

  factory AuditExportAuditLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditExportAuditLogsRequest()..mergeFromBuffer(data, registry);
  factory AuditExportAuditLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditExportAuditLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuditExportAuditLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuditExportAuditLogsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'action')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditExportAuditLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditExportAuditLogsRequest copyWith(
          void Function(AuditExportAuditLogsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as AuditExportAuditLogsRequest))
          as AuditExportAuditLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AuditExportAuditLogsRequest() / AuditExportAuditLogsRequest.new instead')
  static AuditExportAuditLogsRequest create() =>
      AuditExportAuditLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AuditExportAuditLogsRequest._();
  @$core.override
  AuditExportAuditLogsRequest createEmptyInstance() =>
      AuditExportAuditLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static AuditExportAuditLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuditExportAuditLogsRequest>(
          AuditExportAuditLogsRequest.$_createMessage);
  static AuditExportAuditLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get action => $_getSZ(0);
  @$pb.TagNumber(1)
  set action($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);
}

/// CSV with header id,created_at,user_email,user_role,action,resource_type,resource_id,resource_name.
class AuditExportAuditLogsResponse extends $pb.GeneratedMessage {
  factory AuditExportAuditLogsResponse({
    $core.String? csv,
  }) {
    final result = AuditExportAuditLogsResponse._();
    if (csv != null) result.csv = csv;
    return result;
  }

  AuditExportAuditLogsResponse._();

  factory AuditExportAuditLogsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditExportAuditLogsResponse()..mergeFromBuffer(data, registry);
  factory AuditExportAuditLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuditExportAuditLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuditExportAuditLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuditExportAuditLogsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'csv')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditExportAuditLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuditExportAuditLogsResponse copyWith(
          void Function(AuditExportAuditLogsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as AuditExportAuditLogsResponse))
          as AuditExportAuditLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AuditExportAuditLogsResponse() / AuditExportAuditLogsResponse.new instead')
  static AuditExportAuditLogsResponse create() =>
      AuditExportAuditLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AuditExportAuditLogsResponse._();
  @$core.override
  AuditExportAuditLogsResponse createEmptyInstance() =>
      AuditExportAuditLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static AuditExportAuditLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuditExportAuditLogsResponse>(
          AuditExportAuditLogsResponse.$_createMessage);
  static AuditExportAuditLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get csv => $_getSZ(0);
  @$pb.TagNumber(1)
  set csv($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCsv() => $_has(0);
  @$pb.TagNumber(1)
  void clearCsv() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

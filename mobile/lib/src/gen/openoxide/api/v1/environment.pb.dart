// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/environment.proto.

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

class Environment extends $pb.GeneratedMessage {
  factory Environment({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? environmentVariables,
    $core.bool? isDefault,
    $fixnum.Int64? projectId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Environment._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentVariables != null)
      result.environmentVariables = environmentVariables;
    if (isDefault != null) result.isDefault = isDefault;
    if (projectId != null) result.projectId = projectId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Environment._();

  factory Environment.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Environment()..mergeFromBuffer(data, registry);
  factory Environment.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Environment()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Environment',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Environment.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'environmentVariables')
    ..aOB(5, _omitFieldNames ? '' : 'isDefault')
    ..aInt64(6, _omitFieldNames ? '' : 'projectId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Environment clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Environment copyWith(void Function(Environment) updates) =>
      super.copyWith((message) => updates(message as Environment))
          as Environment;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Environment() / Environment.new instead')
  static Environment create() => Environment._();
  static $pb.GeneratedMessage $_createMessage() => Environment._();
  @$core.override
  Environment createEmptyInstance() => Environment._();
  @$core.pragma('dart2js:noInline')
  static Environment getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Environment>(
          Environment.$_createMessage);
  static Environment? _defaultInstance;

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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get environmentVariables => $_getSZ(3);
  @$pb.TagNumber(4)
  set environmentVariables($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnvironmentVariables() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnvironmentVariables() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isDefault => $_getBF(4);
  @$pb.TagNumber(5)
  set isDefault($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIsDefault() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsDefault() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get projectId => $_getI64(5);
  @$pb.TagNumber(6)
  set projectId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasProjectId() => $_has(5);
  @$pb.TagNumber(6)
  void clearProjectId() => $_clearField(6);

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
}

class GetEnvironmentRequest extends $pb.GeneratedMessage {
  factory GetEnvironmentRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetEnvironmentRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetEnvironmentRequest._();

  factory GetEnvironmentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetEnvironmentRequest()..mergeFromBuffer(data, registry);
  factory GetEnvironmentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetEnvironmentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetEnvironmentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetEnvironmentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetEnvironmentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetEnvironmentRequest copyWith(
          void Function(GetEnvironmentRequest) updates) =>
      super.copyWith((message) => updates(message as GetEnvironmentRequest))
          as GetEnvironmentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetEnvironmentRequest() / GetEnvironmentRequest.new instead')
  static GetEnvironmentRequest create() => GetEnvironmentRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetEnvironmentRequest._();
  @$core.override
  GetEnvironmentRequest createEmptyInstance() => GetEnvironmentRequest._();
  @$core.pragma('dart2js:noInline')
  static GetEnvironmentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEnvironmentRequest>(
          GetEnvironmentRequest.$_createMessage);
  static GetEnvironmentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListEnvironmentsRequest extends $pb.GeneratedMessage {
  factory ListEnvironmentsRequest({
    $fixnum.Int64? projectId,
  }) {
    final result = ListEnvironmentsRequest._();
    if (projectId != null) result.projectId = projectId;
    return result;
  }

  ListEnvironmentsRequest._();

  factory ListEnvironmentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListEnvironmentsRequest()..mergeFromBuffer(data, registry);
  factory ListEnvironmentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListEnvironmentsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListEnvironmentsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListEnvironmentsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'projectId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEnvironmentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEnvironmentsRequest copyWith(
          void Function(ListEnvironmentsRequest) updates) =>
      super.copyWith((message) => updates(message as ListEnvironmentsRequest))
          as ListEnvironmentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListEnvironmentsRequest() / ListEnvironmentsRequest.new instead')
  static ListEnvironmentsRequest create() => ListEnvironmentsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListEnvironmentsRequest._();
  @$core.override
  ListEnvironmentsRequest createEmptyInstance() => ListEnvironmentsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListEnvironmentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEnvironmentsRequest>(
          ListEnvironmentsRequest.$_createMessage);
  static ListEnvironmentsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get projectId => $_getI64(0);
  @$pb.TagNumber(1)
  set projectId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectId() => $_clearField(1);
}

class ListEnvironmentsResponse extends $pb.GeneratedMessage {
  factory ListEnvironmentsResponse({
    $core.Iterable<Environment>? environments,
  }) {
    final result = ListEnvironmentsResponse._();
    if (environments != null) result.environments.addAll(environments);
    return result;
  }

  ListEnvironmentsResponse._();

  factory ListEnvironmentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListEnvironmentsResponse()..mergeFromBuffer(data, registry);
  factory ListEnvironmentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListEnvironmentsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListEnvironmentsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListEnvironmentsResponse.$_createMessage)
    ..pPM<Environment>(1, _omitFieldNames ? '' : 'environments',
        subBuilder: Environment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEnvironmentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEnvironmentsResponse copyWith(
          void Function(ListEnvironmentsResponse) updates) =>
      super.copyWith((message) => updates(message as ListEnvironmentsResponse))
          as ListEnvironmentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListEnvironmentsResponse() / ListEnvironmentsResponse.new instead')
  static ListEnvironmentsResponse create() => ListEnvironmentsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListEnvironmentsResponse._();
  @$core.override
  ListEnvironmentsResponse createEmptyInstance() =>
      ListEnvironmentsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListEnvironmentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEnvironmentsResponse>(
          ListEnvironmentsResponse.$_createMessage);
  static ListEnvironmentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Environment> get environments => $_getList(0);
}

class CreateEnvironmentRequest extends $pb.GeneratedMessage {
  factory CreateEnvironmentRequest({
    $core.String? name,
    $core.String? description,
    $core.String? environmentVariables,
    $core.bool? isDefault,
    $fixnum.Int64? projectId,
  }) {
    final result = CreateEnvironmentRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentVariables != null)
      result.environmentVariables = environmentVariables;
    if (isDefault != null) result.isDefault = isDefault;
    if (projectId != null) result.projectId = projectId;
    return result;
  }

  CreateEnvironmentRequest._();

  factory CreateEnvironmentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateEnvironmentRequest()..mergeFromBuffer(data, registry);
  factory CreateEnvironmentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateEnvironmentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateEnvironmentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateEnvironmentRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aOS(3, _omitFieldNames ? '' : 'environmentVariables')
    ..aOB(4, _omitFieldNames ? '' : 'isDefault')
    ..aInt64(5, _omitFieldNames ? '' : 'projectId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateEnvironmentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateEnvironmentRequest copyWith(
          void Function(CreateEnvironmentRequest) updates) =>
      super.copyWith((message) => updates(message as CreateEnvironmentRequest))
          as CreateEnvironmentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateEnvironmentRequest() / CreateEnvironmentRequest.new instead')
  static CreateEnvironmentRequest create() => CreateEnvironmentRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateEnvironmentRequest._();
  @$core.override
  CreateEnvironmentRequest createEmptyInstance() =>
      CreateEnvironmentRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateEnvironmentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateEnvironmentRequest>(
          CreateEnvironmentRequest.$_createMessage);
  static CreateEnvironmentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get environmentVariables => $_getSZ(2);
  @$pb.TagNumber(3)
  set environmentVariables($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasEnvironmentVariables() => $_has(2);
  @$pb.TagNumber(3)
  void clearEnvironmentVariables() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get isDefault => $_getBF(3);
  @$pb.TagNumber(4)
  set isDefault($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasIsDefault() => $_has(3);
  @$pb.TagNumber(4)
  void clearIsDefault() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get projectId => $_getI64(4);
  @$pb.TagNumber(5)
  set projectId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasProjectId() => $_has(4);
  @$pb.TagNumber(5)
  void clearProjectId() => $_clearField(5);
}

class UpdateEnvironmentRequest extends $pb.GeneratedMessage {
  factory UpdateEnvironmentRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? environmentVariables,
    $core.bool? isDefault,
  }) {
    final result = UpdateEnvironmentRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentVariables != null)
      result.environmentVariables = environmentVariables;
    if (isDefault != null) result.isDefault = isDefault;
    return result;
  }

  UpdateEnvironmentRequest._();

  factory UpdateEnvironmentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateEnvironmentRequest()..mergeFromBuffer(data, registry);
  factory UpdateEnvironmentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateEnvironmentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateEnvironmentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateEnvironmentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'environmentVariables')
    ..aOB(5, _omitFieldNames ? '' : 'isDefault')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateEnvironmentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateEnvironmentRequest copyWith(
          void Function(UpdateEnvironmentRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateEnvironmentRequest))
          as UpdateEnvironmentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateEnvironmentRequest() / UpdateEnvironmentRequest.new instead')
  static UpdateEnvironmentRequest create() => UpdateEnvironmentRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateEnvironmentRequest._();
  @$core.override
  UpdateEnvironmentRequest createEmptyInstance() =>
      UpdateEnvironmentRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateEnvironmentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateEnvironmentRequest>(
          UpdateEnvironmentRequest.$_createMessage);
  static UpdateEnvironmentRequest? _defaultInstance;

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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get environmentVariables => $_getSZ(3);
  @$pb.TagNumber(4)
  set environmentVariables($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnvironmentVariables() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnvironmentVariables() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isDefault => $_getBF(4);
  @$pb.TagNumber(5)
  set isDefault($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIsDefault() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsDefault() => $_clearField(5);
}

class SetDefaultEnvironmentRequest extends $pb.GeneratedMessage {
  factory SetDefaultEnvironmentRequest({
    $fixnum.Int64? id,
  }) {
    final result = SetDefaultEnvironmentRequest._();
    if (id != null) result.id = id;
    return result;
  }

  SetDefaultEnvironmentRequest._();

  factory SetDefaultEnvironmentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDefaultEnvironmentRequest()..mergeFromBuffer(data, registry);
  factory SetDefaultEnvironmentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDefaultEnvironmentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetDefaultEnvironmentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetDefaultEnvironmentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDefaultEnvironmentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDefaultEnvironmentRequest copyWith(
          void Function(SetDefaultEnvironmentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SetDefaultEnvironmentRequest))
          as SetDefaultEnvironmentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetDefaultEnvironmentRequest() / SetDefaultEnvironmentRequest.new instead')
  static SetDefaultEnvironmentRequest create() =>
      SetDefaultEnvironmentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SetDefaultEnvironmentRequest._();
  @$core.override
  SetDefaultEnvironmentRequest createEmptyInstance() =>
      SetDefaultEnvironmentRequest._();
  @$core.pragma('dart2js:noInline')
  static SetDefaultEnvironmentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetDefaultEnvironmentRequest>(
          SetDefaultEnvironmentRequest.$_createMessage);
  static SetDefaultEnvironmentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DeleteEnvironmentRequest extends $pb.GeneratedMessage {
  factory DeleteEnvironmentRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteEnvironmentRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteEnvironmentRequest._();

  factory DeleteEnvironmentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteEnvironmentRequest()..mergeFromBuffer(data, registry);
  factory DeleteEnvironmentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteEnvironmentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteEnvironmentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteEnvironmentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteEnvironmentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteEnvironmentRequest copyWith(
          void Function(DeleteEnvironmentRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteEnvironmentRequest))
          as DeleteEnvironmentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteEnvironmentRequest() / DeleteEnvironmentRequest.new instead')
  static DeleteEnvironmentRequest create() => DeleteEnvironmentRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteEnvironmentRequest._();
  @$core.override
  DeleteEnvironmentRequest createEmptyInstance() =>
      DeleteEnvironmentRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteEnvironmentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteEnvironmentRequest>(
          DeleteEnvironmentRequest.$_createMessage);
  static DeleteEnvironmentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

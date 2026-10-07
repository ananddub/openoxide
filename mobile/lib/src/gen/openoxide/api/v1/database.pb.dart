// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'database.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'database.pbenum.dart';

class Database extends $pb.GeneratedMessage {
  factory Database({
    DatabaseKind? kind,
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? dockerImage,
    $core.String? databaseName,
    $core.String? databaseUser,
    $core.String? databasePassword,
    $fixnum.Int64? externalPort,
    $core.String? command,
    $core.Iterable<$core.String>? args,
    $core.String? envVar,
    $core.String? memoryLimit,
    $core.String? cpuLimit,
    $fixnum.Int64? replicas,
    $core.Iterable<$core.String>? networkIds,
    $core.String? status,
    $core.String? lastError,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.String? appName,
  }) {
    final result = Database._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (dockerImage != null) result.dockerImage = dockerImage;
    if (databaseName != null) result.databaseName = databaseName;
    if (databaseUser != null) result.databaseUser = databaseUser;
    if (databasePassword != null) result.databasePassword = databasePassword;
    if (externalPort != null) result.externalPort = externalPort;
    if (command != null) result.command = command;
    if (args != null) result.args.addAll(args);
    if (envVar != null) result.envVar = envVar;
    if (memoryLimit != null) result.memoryLimit = memoryLimit;
    if (cpuLimit != null) result.cpuLimit = cpuLimit;
    if (replicas != null) result.replicas = replicas;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (status != null) result.status = status;
    if (lastError != null) result.lastError = lastError;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (appName != null) result.appName = appName;
    return result;
  }

  Database._();

  factory Database.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Database()..mergeFromBuffer(data, registry);
  factory Database.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Database()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Database',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Database.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aOS(5, _omitFieldNames ? '' : 'dockerImage')
    ..aOS(6, _omitFieldNames ? '' : 'databaseName')
    ..aOS(7, _omitFieldNames ? '' : 'databaseUser')
    ..aOS(8, _omitFieldNames ? '' : 'databasePassword')
    ..aInt64(9, _omitFieldNames ? '' : 'externalPort')
    ..aOS(10, _omitFieldNames ? '' : 'command')
    ..pPS(11, _omitFieldNames ? '' : 'args')
    ..aOS(12, _omitFieldNames ? '' : 'envVar')
    ..aOS(13, _omitFieldNames ? '' : 'memoryLimit')
    ..aOS(14, _omitFieldNames ? '' : 'cpuLimit')
    ..aInt64(15, _omitFieldNames ? '' : 'replicas')
    ..pPS(16, _omitFieldNames ? '' : 'networkIds')
    ..aOS(17, _omitFieldNames ? '' : 'status')
    ..aOS(18, _omitFieldNames ? '' : 'lastError')
    ..aInt64(19, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(20, _omitFieldNames ? '' : 'serverId')
    ..aInt64(21, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(22, _omitFieldNames ? '' : 'updatedAt')
    ..aOS(23, _omitFieldNames ? '' : 'appName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Database clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Database copyWith(void Function(Database) updates) =>
      super.copyWith((message) => updates(message as Database)) as Database;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Database() / Database.new instead')
  static Database create() => Database._();
  static $pb.GeneratedMessage $_createMessage() => Database._();
  @$core.override
  Database createEmptyInstance() => Database._();
  @$core.pragma('dart2js:noInline')
  static Database getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Database>(Database.$_createMessage);
  static Database? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get dockerImage => $_getSZ(4);
  @$pb.TagNumber(5)
  set dockerImage($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDockerImage() => $_has(4);
  @$pb.TagNumber(5)
  void clearDockerImage() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get databaseName => $_getSZ(5);
  @$pb.TagNumber(6)
  set databaseName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDatabaseName() => $_has(5);
  @$pb.TagNumber(6)
  void clearDatabaseName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get databaseUser => $_getSZ(6);
  @$pb.TagNumber(7)
  set databaseUser($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDatabaseUser() => $_has(6);
  @$pb.TagNumber(7)
  void clearDatabaseUser() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get databasePassword => $_getSZ(7);
  @$pb.TagNumber(8)
  set databasePassword($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabasePassword() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabasePassword() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get externalPort => $_getI64(8);
  @$pb.TagNumber(9)
  set externalPort($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasExternalPort() => $_has(8);
  @$pb.TagNumber(9)
  void clearExternalPort() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get command => $_getSZ(9);
  @$pb.TagNumber(10)
  set command($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCommand() => $_has(9);
  @$pb.TagNumber(10)
  void clearCommand() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<$core.String> get args => $_getList(10);

  @$pb.TagNumber(12)
  $core.String get envVar => $_getSZ(11);
  @$pb.TagNumber(12)
  set envVar($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnvVar() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnvVar() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get memoryLimit => $_getSZ(12);
  @$pb.TagNumber(13)
  set memoryLimit($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasMemoryLimit() => $_has(12);
  @$pb.TagNumber(13)
  void clearMemoryLimit() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get cpuLimit => $_getSZ(13);
  @$pb.TagNumber(14)
  set cpuLimit($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasCpuLimit() => $_has(13);
  @$pb.TagNumber(14)
  void clearCpuLimit() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get replicas => $_getI64(14);
  @$pb.TagNumber(15)
  set replicas($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasReplicas() => $_has(14);
  @$pb.TagNumber(15)
  void clearReplicas() => $_clearField(15);

  @$pb.TagNumber(16)
  $pb.PbList<$core.String> get networkIds => $_getList(15);

  @$pb.TagNumber(17)
  $core.String get status => $_getSZ(16);
  @$pb.TagNumber(17)
  set status($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasStatus() => $_has(16);
  @$pb.TagNumber(17)
  void clearStatus() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.String get lastError => $_getSZ(17);
  @$pb.TagNumber(18)
  set lastError($core.String value) => $_setString(17, value);
  @$pb.TagNumber(18)
  $core.bool hasLastError() => $_has(17);
  @$pb.TagNumber(18)
  void clearLastError() => $_clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get environmentId => $_getI64(18);
  @$pb.TagNumber(19)
  set environmentId($fixnum.Int64 value) => $_setInt64(18, value);
  @$pb.TagNumber(19)
  $core.bool hasEnvironmentId() => $_has(18);
  @$pb.TagNumber(19)
  void clearEnvironmentId() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get serverId => $_getI64(19);
  @$pb.TagNumber(20)
  set serverId($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasServerId() => $_has(19);
  @$pb.TagNumber(20)
  void clearServerId() => $_clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get createdAt => $_getI64(20);
  @$pb.TagNumber(21)
  set createdAt($fixnum.Int64 value) => $_setInt64(20, value);
  @$pb.TagNumber(21)
  $core.bool hasCreatedAt() => $_has(20);
  @$pb.TagNumber(21)
  void clearCreatedAt() => $_clearField(21);

  @$pb.TagNumber(22)
  $fixnum.Int64 get updatedAt => $_getI64(21);
  @$pb.TagNumber(22)
  set updatedAt($fixnum.Int64 value) => $_setInt64(21, value);
  @$pb.TagNumber(22)
  $core.bool hasUpdatedAt() => $_has(21);
  @$pb.TagNumber(22)
  void clearUpdatedAt() => $_clearField(22);

  /// Real container/service handle (`postgres-test-1ed127`); `name` is the human label. Log streams and exec target this, never `name`, so same-labeled DBs don't collide.
  @$pb.TagNumber(23)
  $core.String get appName => $_getSZ(22);
  @$pb.TagNumber(23)
  set appName($core.String value) => $_setString(22, value);
  @$pb.TagNumber(23)
  $core.bool hasAppName() => $_has(22);
  @$pb.TagNumber(23)
  void clearAppName() => $_clearField(23);
}

class GetDatabaseRequest extends $pb.GeneratedMessage {
  factory GetDatabaseRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
  }) {
    final result = GetDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    return result;
  }

  GetDatabaseRequest._();

  factory GetDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDatabaseRequest()..mergeFromBuffer(data, registry);
  factory GetDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDatabaseRequest copyWith(void Function(GetDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as GetDatabaseRequest))
          as GetDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetDatabaseRequest() / GetDatabaseRequest.new instead')
  static GetDatabaseRequest create() => GetDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetDatabaseRequest._();
  @$core.override
  GetDatabaseRequest createEmptyInstance() => GetDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static GetDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDatabaseRequest>(
          GetDatabaseRequest.$_createMessage);
  static GetDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ListDatabasesRequest extends $pb.GeneratedMessage {
  factory ListDatabasesRequest({
    DatabaseKind? kind,
    $fixnum.Int64? environmentId,
  }) {
    final result = ListDatabasesRequest._();
    if (kind != null) result.kind = kind;
    if (environmentId != null) result.environmentId = environmentId;
    return result;
  }

  ListDatabasesRequest._();

  factory ListDatabasesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDatabasesRequest()..mergeFromBuffer(data, registry);
  factory ListDatabasesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDatabasesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDatabasesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDatabasesRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDatabasesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDatabasesRequest copyWith(void Function(ListDatabasesRequest) updates) =>
      super.copyWith((message) => updates(message as ListDatabasesRequest))
          as ListDatabasesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDatabasesRequest() / ListDatabasesRequest.new instead')
  static ListDatabasesRequest create() => ListDatabasesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDatabasesRequest._();
  @$core.override
  ListDatabasesRequest createEmptyInstance() => ListDatabasesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDatabasesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDatabasesRequest>(
          ListDatabasesRequest.$_createMessage);
  static ListDatabasesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);
}

class ListDatabasesResponse extends $pb.GeneratedMessage {
  factory ListDatabasesResponse({
    $core.Iterable<Database>? databases,
  }) {
    final result = ListDatabasesResponse._();
    if (databases != null) result.databases.addAll(databases);
    return result;
  }

  ListDatabasesResponse._();

  factory ListDatabasesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDatabasesResponse()..mergeFromBuffer(data, registry);
  factory ListDatabasesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDatabasesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDatabasesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDatabasesResponse.$_createMessage)
    ..pPM<Database>(1, _omitFieldNames ? '' : 'databases',
        subBuilder: Database.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDatabasesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDatabasesResponse copyWith(
          void Function(ListDatabasesResponse) updates) =>
      super.copyWith((message) => updates(message as ListDatabasesResponse))
          as ListDatabasesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDatabasesResponse() / ListDatabasesResponse.new instead')
  static ListDatabasesResponse create() => ListDatabasesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDatabasesResponse._();
  @$core.override
  ListDatabasesResponse createEmptyInstance() => ListDatabasesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDatabasesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDatabasesResponse>(
          ListDatabasesResponse.$_createMessage);
  static ListDatabasesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Database> get databases => $_getList(0);
}

class CreateDatabaseRequest extends $pb.GeneratedMessage {
  factory CreateDatabaseRequest({
    DatabaseKind? kind,
    $core.String? name,
    $core.String? description,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $core.String? dockerImage,
    $core.String? databaseName,
    $core.String? databaseUser,
    $core.String? databasePassword,
    $fixnum.Int64? externalPort,
    $core.Iterable<$core.String>? args,
    $core.String? envVar,
    $core.Iterable<$core.String>? networkIds,
  }) {
    final result = CreateDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (dockerImage != null) result.dockerImage = dockerImage;
    if (databaseName != null) result.databaseName = databaseName;
    if (databaseUser != null) result.databaseUser = databaseUser;
    if (databasePassword != null) result.databasePassword = databasePassword;
    if (externalPort != null) result.externalPort = externalPort;
    if (args != null) result.args.addAll(args);
    if (envVar != null) result.envVar = envVar;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    return result;
  }

  CreateDatabaseRequest._();

  factory CreateDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDatabaseRequest()..mergeFromBuffer(data, registry);
  factory CreateDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aInt64(4, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(5, _omitFieldNames ? '' : 'serverId')
    ..aOS(6, _omitFieldNames ? '' : 'dockerImage')
    ..aOS(7, _omitFieldNames ? '' : 'databaseName')
    ..aOS(8, _omitFieldNames ? '' : 'databaseUser')
    ..aOS(9, _omitFieldNames ? '' : 'databasePassword')
    ..aInt64(10, _omitFieldNames ? '' : 'externalPort')
    ..pPS(11, _omitFieldNames ? '' : 'args')
    ..aOS(12, _omitFieldNames ? '' : 'envVar')
    ..pPS(13, _omitFieldNames ? '' : 'networkIds')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDatabaseRequest copyWith(
          void Function(CreateDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDatabaseRequest))
          as CreateDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDatabaseRequest() / CreateDatabaseRequest.new instead')
  static CreateDatabaseRequest create() => CreateDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateDatabaseRequest._();
  @$core.override
  CreateDatabaseRequest createEmptyInstance() => CreateDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDatabaseRequest>(
          CreateDatabaseRequest.$_createMessage);
  static CreateDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

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
  $fixnum.Int64 get environmentId => $_getI64(3);
  @$pb.TagNumber(4)
  set environmentId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnvironmentId() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnvironmentId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get serverId => $_getI64(4);
  @$pb.TagNumber(5)
  set serverId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServerId() => $_has(4);
  @$pb.TagNumber(5)
  void clearServerId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get dockerImage => $_getSZ(5);
  @$pb.TagNumber(6)
  set dockerImage($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDockerImage() => $_has(5);
  @$pb.TagNumber(6)
  void clearDockerImage() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get databaseName => $_getSZ(6);
  @$pb.TagNumber(7)
  set databaseName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDatabaseName() => $_has(6);
  @$pb.TagNumber(7)
  void clearDatabaseName() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get databaseUser => $_getSZ(7);
  @$pb.TagNumber(8)
  set databaseUser($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabaseUser() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabaseUser() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get databasePassword => $_getSZ(8);
  @$pb.TagNumber(9)
  set databasePassword($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasDatabasePassword() => $_has(8);
  @$pb.TagNumber(9)
  void clearDatabasePassword() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get externalPort => $_getI64(9);
  @$pb.TagNumber(10)
  set externalPort($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasExternalPort() => $_has(9);
  @$pb.TagNumber(10)
  void clearExternalPort() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<$core.String> get args => $_getList(10);

  @$pb.TagNumber(12)
  $core.String get envVar => $_getSZ(11);
  @$pb.TagNumber(12)
  set envVar($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnvVar() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnvVar() => $_clearField(12);

  @$pb.TagNumber(13)
  $pb.PbList<$core.String> get networkIds => $_getList(12);
}

class UpdateDatabaseRequest extends $pb.GeneratedMessage {
  factory UpdateDatabaseRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? dockerImage,
    $core.String? databaseName,
    $core.String? databaseUser,
    $core.String? databasePassword,
    $fixnum.Int64? externalPort,
    $core.String? command,
    $core.Iterable<$core.String>? args,
    $core.String? envVar,
    $core.String? memoryLimit,
    $core.String? cpuLimit,
    $fixnum.Int64? replicas,
    $fixnum.Int64? serverId,
    $core.Iterable<$core.String>? networkIds,
    $fixnum.Int64? detachRustployNetwork,
    $core.String? memoryReservation,
    $core.String? cpuReservation,
  }) {
    final result = UpdateDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (dockerImage != null) result.dockerImage = dockerImage;
    if (databaseName != null) result.databaseName = databaseName;
    if (databaseUser != null) result.databaseUser = databaseUser;
    if (databasePassword != null) result.databasePassword = databasePassword;
    if (externalPort != null) result.externalPort = externalPort;
    if (command != null) result.command = command;
    if (args != null) result.args.addAll(args);
    if (envVar != null) result.envVar = envVar;
    if (memoryLimit != null) result.memoryLimit = memoryLimit;
    if (cpuLimit != null) result.cpuLimit = cpuLimit;
    if (replicas != null) result.replicas = replicas;
    if (serverId != null) result.serverId = serverId;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (detachRustployNetwork != null)
      result.detachRustployNetwork = detachRustployNetwork;
    if (memoryReservation != null) result.memoryReservation = memoryReservation;
    if (cpuReservation != null) result.cpuReservation = cpuReservation;
    return result;
  }

  UpdateDatabaseRequest._();

  factory UpdateDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDatabaseRequest()..mergeFromBuffer(data, registry);
  factory UpdateDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aOS(5, _omitFieldNames ? '' : 'dockerImage')
    ..aOS(6, _omitFieldNames ? '' : 'databaseName')
    ..aOS(7, _omitFieldNames ? '' : 'databaseUser')
    ..aOS(8, _omitFieldNames ? '' : 'databasePassword')
    ..aInt64(9, _omitFieldNames ? '' : 'externalPort')
    ..aOS(10, _omitFieldNames ? '' : 'command')
    ..pPS(11, _omitFieldNames ? '' : 'args')
    ..aOS(12, _omitFieldNames ? '' : 'envVar')
    ..aOS(13, _omitFieldNames ? '' : 'memoryLimit')
    ..aOS(14, _omitFieldNames ? '' : 'cpuLimit')
    ..aInt64(15, _omitFieldNames ? '' : 'replicas')
    ..aInt64(16, _omitFieldNames ? '' : 'serverId')
    ..pPS(17, _omitFieldNames ? '' : 'networkIds')
    ..aInt64(18, _omitFieldNames ? '' : 'detachRustployNetwork')
    ..aOS(19, _omitFieldNames ? '' : 'memoryReservation')
    ..aOS(20, _omitFieldNames ? '' : 'cpuReservation')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDatabaseRequest copyWith(
          void Function(UpdateDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateDatabaseRequest))
          as UpdateDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDatabaseRequest() / UpdateDatabaseRequest.new instead')
  static UpdateDatabaseRequest create() => UpdateDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDatabaseRequest._();
  @$core.override
  UpdateDatabaseRequest createEmptyInstance() => UpdateDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDatabaseRequest>(
          UpdateDatabaseRequest.$_createMessage);
  static UpdateDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get dockerImage => $_getSZ(4);
  @$pb.TagNumber(5)
  set dockerImage($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDockerImage() => $_has(4);
  @$pb.TagNumber(5)
  void clearDockerImage() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get databaseName => $_getSZ(5);
  @$pb.TagNumber(6)
  set databaseName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDatabaseName() => $_has(5);
  @$pb.TagNumber(6)
  void clearDatabaseName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get databaseUser => $_getSZ(6);
  @$pb.TagNumber(7)
  set databaseUser($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDatabaseUser() => $_has(6);
  @$pb.TagNumber(7)
  void clearDatabaseUser() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get databasePassword => $_getSZ(7);
  @$pb.TagNumber(8)
  set databasePassword($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabasePassword() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabasePassword() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get externalPort => $_getI64(8);
  @$pb.TagNumber(9)
  set externalPort($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasExternalPort() => $_has(8);
  @$pb.TagNumber(9)
  void clearExternalPort() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get command => $_getSZ(9);
  @$pb.TagNumber(10)
  set command($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCommand() => $_has(9);
  @$pb.TagNumber(10)
  void clearCommand() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<$core.String> get args => $_getList(10);

  @$pb.TagNumber(12)
  $core.String get envVar => $_getSZ(11);
  @$pb.TagNumber(12)
  set envVar($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnvVar() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnvVar() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get memoryLimit => $_getSZ(12);
  @$pb.TagNumber(13)
  set memoryLimit($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasMemoryLimit() => $_has(12);
  @$pb.TagNumber(13)
  void clearMemoryLimit() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get cpuLimit => $_getSZ(13);
  @$pb.TagNumber(14)
  set cpuLimit($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasCpuLimit() => $_has(13);
  @$pb.TagNumber(14)
  void clearCpuLimit() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get replicas => $_getI64(14);
  @$pb.TagNumber(15)
  set replicas($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasReplicas() => $_has(14);
  @$pb.TagNumber(15)
  void clearReplicas() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get serverId => $_getI64(15);
  @$pb.TagNumber(16)
  set serverId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasServerId() => $_has(15);
  @$pb.TagNumber(16)
  void clearServerId() => $_clearField(16);

  @$pb.TagNumber(17)
  $pb.PbList<$core.String> get networkIds => $_getList(16);

  @$pb.TagNumber(18)
  $fixnum.Int64 get detachRustployNetwork => $_getI64(17);
  @$pb.TagNumber(18)
  set detachRustployNetwork($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasDetachRustployNetwork() => $_has(17);
  @$pb.TagNumber(18)
  void clearDetachRustployNetwork() => $_clearField(18);

  @$pb.TagNumber(19)
  $core.String get memoryReservation => $_getSZ(18);
  @$pb.TagNumber(19)
  set memoryReservation($core.String value) => $_setString(18, value);
  @$pb.TagNumber(19)
  $core.bool hasMemoryReservation() => $_has(18);
  @$pb.TagNumber(19)
  void clearMemoryReservation() => $_clearField(19);

  @$pb.TagNumber(20)
  $core.String get cpuReservation => $_getSZ(19);
  @$pb.TagNumber(20)
  set cpuReservation($core.String value) => $_setString(19, value);
  @$pb.TagNumber(20)
  $core.bool hasCpuReservation() => $_has(19);
  @$pb.TagNumber(20)
  void clearCpuReservation() => $_clearField(20);
}

class DeleteDatabaseRequest extends $pb.GeneratedMessage {
  factory DeleteDatabaseRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
  }) {
    final result = DeleteDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    return result;
  }

  DeleteDatabaseRequest._();

  factory DeleteDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDatabaseRequest()..mergeFromBuffer(data, registry);
  factory DeleteDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDatabaseRequest copyWith(
          void Function(DeleteDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteDatabaseRequest))
          as DeleteDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteDatabaseRequest() / DeleteDatabaseRequest.new instead')
  static DeleteDatabaseRequest create() => DeleteDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteDatabaseRequest._();
  @$core.override
  DeleteDatabaseRequest createEmptyInstance() => DeleteDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteDatabaseRequest>(
          DeleteDatabaseRequest.$_createMessage);
  static DeleteDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class DatabaseOperationRequest extends $pb.GeneratedMessage {
  factory DatabaseOperationRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
  }) {
    final result = DatabaseOperationRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    return result;
  }

  DatabaseOperationRequest._();

  factory DatabaseOperationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseOperationRequest()..mergeFromBuffer(data, registry);
  factory DatabaseOperationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseOperationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseOperationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseOperationRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseOperationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseOperationRequest copyWith(
          void Function(DatabaseOperationRequest) updates) =>
      super.copyWith((message) => updates(message as DatabaseOperationRequest))
          as DatabaseOperationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseOperationRequest() / DatabaseOperationRequest.new instead')
  static DatabaseOperationRequest create() => DatabaseOperationRequest._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseOperationRequest._();
  @$core.override
  DatabaseOperationRequest createEmptyInstance() =>
      DatabaseOperationRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseOperationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseOperationRequest>(
          DatabaseOperationRequest.$_createMessage);
  static DatabaseOperationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class DatabaseOperation extends $pb.GeneratedMessage {
  factory DatabaseOperation({
    Database? database,
    $core.String? operation,
  }) {
    final result = DatabaseOperation._();
    if (database != null) result.database = database;
    if (operation != null) result.operation = operation;
    return result;
  }

  DatabaseOperation._();

  factory DatabaseOperation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseOperation()..mergeFromBuffer(data, registry);
  factory DatabaseOperation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseOperation()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseOperation',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseOperation.$_createMessage)
    ..aOM<Database>(1, _omitFieldNames ? '' : 'database',
        subBuilder: Database.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'operation')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseOperation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseOperation copyWith(void Function(DatabaseOperation) updates) =>
      super.copyWith((message) => updates(message as DatabaseOperation))
          as DatabaseOperation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DatabaseOperation() / DatabaseOperation.new instead')
  static DatabaseOperation create() => DatabaseOperation._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseOperation._();
  @$core.override
  DatabaseOperation createEmptyInstance() => DatabaseOperation._();
  @$core.pragma('dart2js:noInline')
  static DatabaseOperation getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DatabaseOperation>(
          DatabaseOperation.$_createMessage);
  static DatabaseOperation? _defaultInstance;

  @$pb.TagNumber(1)
  Database get database => $_getN(0);
  @$pb.TagNumber(1)
  set database(Database value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDatabase() => $_has(0);
  @$pb.TagNumber(1)
  void clearDatabase() => $_clearField(1);
  @$pb.TagNumber(1)
  Database ensureDatabase() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get operation => $_getSZ(1);
  @$pb.TagNumber(2)
  set operation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOperation() => $_has(1);
  @$pb.TagNumber(2)
  void clearOperation() => $_clearField(2);
}

class DatabaseTargetRequest extends $pb.GeneratedMessage {
  factory DatabaseTargetRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
  }) {
    final result = DatabaseTargetRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    return result;
  }

  DatabaseTargetRequest._();

  factory DatabaseTargetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseTargetRequest()..mergeFromBuffer(data, registry);
  factory DatabaseTargetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseTargetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseTargetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseTargetRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseTargetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseTargetRequest copyWith(
          void Function(DatabaseTargetRequest) updates) =>
      super.copyWith((message) => updates(message as DatabaseTargetRequest))
          as DatabaseTargetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseTargetRequest() / DatabaseTargetRequest.new instead')
  static DatabaseTargetRequest create() => DatabaseTargetRequest._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseTargetRequest._();
  @$core.override
  DatabaseTargetRequest createEmptyInstance() => DatabaseTargetRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseTargetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseTargetRequest>(
          DatabaseTargetRequest.$_createMessage);
  static DatabaseTargetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class SwarmSettingsRequest extends $pb.GeneratedMessage {
  factory SwarmSettingsRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
  }) {
    final result = SwarmSettingsRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    return result;
  }

  SwarmSettingsRequest._();

  factory SwarmSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettingsRequest()..mergeFromBuffer(data, registry);
  factory SwarmSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmSettingsRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettingsRequest copyWith(void Function(SwarmSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmSettingsRequest))
          as SwarmSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmSettingsRequest() / SwarmSettingsRequest.new instead')
  static SwarmSettingsRequest create() => SwarmSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmSettingsRequest._();
  @$core.override
  SwarmSettingsRequest createEmptyInstance() => SwarmSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmSettingsRequest>(
          SwarmSettingsRequest.$_createMessage);
  static SwarmSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class SwarmSettings extends $pb.GeneratedMessage {
  factory SwarmSettings({
    $core.String? healthCheck,
    $core.String? restartPolicy,
    $core.String? placement,
    $core.String? updateConfig,
    $core.String? rollbackConfig,
    $core.String? mode,
    $core.String? labels,
    $core.String? network,
    $core.String? endpointSpec,
    $core.String? ulimits,
    $fixnum.Int64? stopGracePeriod,
  }) {
    final result = SwarmSettings._();
    if (healthCheck != null) result.healthCheck = healthCheck;
    if (restartPolicy != null) result.restartPolicy = restartPolicy;
    if (placement != null) result.placement = placement;
    if (updateConfig != null) result.updateConfig = updateConfig;
    if (rollbackConfig != null) result.rollbackConfig = rollbackConfig;
    if (mode != null) result.mode = mode;
    if (labels != null) result.labels = labels;
    if (network != null) result.network = network;
    if (endpointSpec != null) result.endpointSpec = endpointSpec;
    if (ulimits != null) result.ulimits = ulimits;
    if (stopGracePeriod != null) result.stopGracePeriod = stopGracePeriod;
    return result;
  }

  SwarmSettings._();

  factory SwarmSettings.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettings()..mergeFromBuffer(data, registry);
  factory SwarmSettings.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettings()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmSettings',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmSettings.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'healthCheck')
    ..aOS(2, _omitFieldNames ? '' : 'restartPolicy')
    ..aOS(3, _omitFieldNames ? '' : 'placement')
    ..aOS(4, _omitFieldNames ? '' : 'updateConfig')
    ..aOS(5, _omitFieldNames ? '' : 'rollbackConfig')
    ..aOS(6, _omitFieldNames ? '' : 'mode')
    ..aOS(7, _omitFieldNames ? '' : 'labels')
    ..aOS(8, _omitFieldNames ? '' : 'network')
    ..aOS(9, _omitFieldNames ? '' : 'endpointSpec')
    ..aOS(10, _omitFieldNames ? '' : 'ulimits')
    ..aInt64(11, _omitFieldNames ? '' : 'stopGracePeriod')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettings clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettings copyWith(void Function(SwarmSettings) updates) =>
      super.copyWith((message) => updates(message as SwarmSettings))
          as SwarmSettings;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmSettings() / SwarmSettings.new instead')
  static SwarmSettings create() => SwarmSettings._();
  static $pb.GeneratedMessage $_createMessage() => SwarmSettings._();
  @$core.override
  SwarmSettings createEmptyInstance() => SwarmSettings._();
  @$core.pragma('dart2js:noInline')
  static SwarmSettings getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmSettings>(
          SwarmSettings.$_createMessage);
  static SwarmSettings? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get healthCheck => $_getSZ(0);
  @$pb.TagNumber(1)
  set healthCheck($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHealthCheck() => $_has(0);
  @$pb.TagNumber(1)
  void clearHealthCheck() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get restartPolicy => $_getSZ(1);
  @$pb.TagNumber(2)
  set restartPolicy($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRestartPolicy() => $_has(1);
  @$pb.TagNumber(2)
  void clearRestartPolicy() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get placement => $_getSZ(2);
  @$pb.TagNumber(3)
  set placement($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPlacement() => $_has(2);
  @$pb.TagNumber(3)
  void clearPlacement() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get updateConfig => $_getSZ(3);
  @$pb.TagNumber(4)
  set updateConfig($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUpdateConfig() => $_has(3);
  @$pb.TagNumber(4)
  void clearUpdateConfig() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get rollbackConfig => $_getSZ(4);
  @$pb.TagNumber(5)
  set rollbackConfig($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRollbackConfig() => $_has(4);
  @$pb.TagNumber(5)
  void clearRollbackConfig() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get mode => $_getSZ(5);
  @$pb.TagNumber(6)
  set mode($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasMode() => $_has(5);
  @$pb.TagNumber(6)
  void clearMode() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get labels => $_getSZ(6);
  @$pb.TagNumber(7)
  set labels($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasLabels() => $_has(6);
  @$pb.TagNumber(7)
  void clearLabels() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get network => $_getSZ(7);
  @$pb.TagNumber(8)
  set network($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasNetwork() => $_has(7);
  @$pb.TagNumber(8)
  void clearNetwork() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get endpointSpec => $_getSZ(8);
  @$pb.TagNumber(9)
  set endpointSpec($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEndpointSpec() => $_has(8);
  @$pb.TagNumber(9)
  void clearEndpointSpec() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get ulimits => $_getSZ(9);
  @$pb.TagNumber(10)
  set ulimits($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUlimits() => $_has(9);
  @$pb.TagNumber(10)
  void clearUlimits() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get stopGracePeriod => $_getI64(10);
  @$pb.TagNumber(11)
  set stopGracePeriod($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasStopGracePeriod() => $_has(10);
  @$pb.TagNumber(11)
  void clearStopGracePeriod() => $_clearField(11);
}

class UpdateSwarmSettingsRequest extends $pb.GeneratedMessage {
  factory UpdateSwarmSettingsRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
    SwarmSettings? settings,
  }) {
    final result = UpdateSwarmSettingsRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    if (settings != null) result.settings = settings;
    return result;
  }

  UpdateSwarmSettingsRequest._();

  factory UpdateSwarmSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateSwarmSettingsRequest()..mergeFromBuffer(data, registry);
  factory UpdateSwarmSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateSwarmSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateSwarmSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateSwarmSettingsRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOM<SwarmSettings>(3, _omitFieldNames ? '' : 'settings',
        subBuilder: SwarmSettings.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateSwarmSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateSwarmSettingsRequest copyWith(
          void Function(UpdateSwarmSettingsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateSwarmSettingsRequest))
          as UpdateSwarmSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateSwarmSettingsRequest() / UpdateSwarmSettingsRequest.new instead')
  static UpdateSwarmSettingsRequest create() => UpdateSwarmSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateSwarmSettingsRequest._();
  @$core.override
  UpdateSwarmSettingsRequest createEmptyInstance() =>
      UpdateSwarmSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateSwarmSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateSwarmSettingsRequest>(
          UpdateSwarmSettingsRequest.$_createMessage);
  static UpdateSwarmSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  SwarmSettings get settings => $_getN(2);
  @$pb.TagNumber(3)
  set settings(SwarmSettings value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSettings() => $_has(2);
  @$pb.TagNumber(3)
  void clearSettings() => $_clearField(3);
  @$pb.TagNumber(3)
  SwarmSettings ensureSettings() => $_ensure(2);
}

class Mount extends $pb.GeneratedMessage {
  factory Mount({
    $fixnum.Int64? id,
    $core.String? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Mount._();
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Mount._();

  factory Mount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Mount()..mergeFromBuffer(data, registry);
  factory Mount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Mount()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Mount',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Mount.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'mountType')
    ..aOS(3, _omitFieldNames ? '' : 'hostPath')
    ..aOS(4, _omitFieldNames ? '' : 'volumeName')
    ..aOS(5, _omitFieldNames ? '' : 'filePath')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..aOS(7, _omitFieldNames ? '' : 'mountPath')
    ..aInt64(8, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(9, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Mount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Mount copyWith(void Function(Mount) updates) =>
      super.copyWith((message) => updates(message as Mount)) as Mount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Mount() / Mount.new instead')
  static Mount create() => Mount._();
  static $pb.GeneratedMessage $_createMessage() => Mount._();
  @$core.override
  Mount createEmptyInstance() => Mount._();
  @$core.pragma('dart2js:noInline')
  static Mount getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Mount>(Mount.$_createMessage);
  static Mount? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get mountType => $_getSZ(1);
  @$pb.TagNumber(2)
  set mountType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMountType() => $_has(1);
  @$pb.TagNumber(2)
  void clearMountType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get hostPath => $_getSZ(2);
  @$pb.TagNumber(3)
  set hostPath($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHostPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearHostPath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get volumeName => $_getSZ(3);
  @$pb.TagNumber(4)
  set volumeName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasVolumeName() => $_has(3);
  @$pb.TagNumber(4)
  void clearVolumeName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get filePath => $_getSZ(4);
  @$pb.TagNumber(5)
  set filePath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFilePath() => $_has(4);
  @$pb.TagNumber(5)
  void clearFilePath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get mountPath => $_getSZ(6);
  @$pb.TagNumber(7)
  set mountPath($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasMountPath() => $_has(6);
  @$pb.TagNumber(7)
  void clearMountPath() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get createdAt => $_getI64(7);
  @$pb.TagNumber(8)
  set createdAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCreatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedAt() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get updatedAt => $_getI64(8);
  @$pb.TagNumber(9)
  set updatedAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUpdatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearUpdatedAt() => $_clearField(9);
}

class MountList extends $pb.GeneratedMessage {
  factory MountList({
    $core.Iterable<Mount>? mounts,
  }) {
    final result = MountList._();
    if (mounts != null) result.mounts.addAll(mounts);
    return result;
  }

  MountList._();

  factory MountList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MountList()..mergeFromBuffer(data, registry);
  factory MountList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MountList()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MountList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MountList.$_createMessage)
    ..pPM<Mount>(1, _omitFieldNames ? '' : 'mounts',
        subBuilder: Mount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MountList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MountList copyWith(void Function(MountList) updates) =>
      super.copyWith((message) => updates(message as MountList)) as MountList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use MountList() / MountList.new instead')
  static MountList create() => MountList._();
  static $pb.GeneratedMessage $_createMessage() => MountList._();
  @$core.override
  MountList createEmptyInstance() => MountList._();
  @$core.pragma('dart2js:noInline')
  static MountList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MountList>(MountList.$_createMessage);
  static MountList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Mount> get mounts => $_getList(0);
}

class CreateMountRequest extends $pb.GeneratedMessage {
  factory CreateMountRequest({
    DatabaseKind? kind,
    $fixnum.Int64? databaseId,
    $core.String? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = CreateMountRequest._();
    if (kind != null) result.kind = kind;
    if (databaseId != null) result.databaseId = databaseId;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  CreateMountRequest._();

  factory CreateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateMountRequest()..mergeFromBuffer(data, registry);
  factory CreateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateMountRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'databaseId')
    ..aOS(3, _omitFieldNames ? '' : 'mountType')
    ..aOS(4, _omitFieldNames ? '' : 'hostPath')
    ..aOS(5, _omitFieldNames ? '' : 'volumeName')
    ..aOS(6, _omitFieldNames ? '' : 'filePath')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..aOS(8, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateMountRequest copyWith(void Function(CreateMountRequest) updates) =>
      super.copyWith((message) => updates(message as CreateMountRequest))
          as CreateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CreateMountRequest() / CreateMountRequest.new instead')
  static CreateMountRequest create() => CreateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateMountRequest._();
  @$core.override
  CreateMountRequest createEmptyInstance() => CreateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateMountRequest>(
          CreateMountRequest.$_createMessage);
  static CreateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get databaseId => $_getI64(1);
  @$pb.TagNumber(2)
  set databaseId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDatabaseId() => $_has(1);
  @$pb.TagNumber(2)
  void clearDatabaseId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get mountType => $_getSZ(2);
  @$pb.TagNumber(3)
  set mountType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMountType() => $_has(2);
  @$pb.TagNumber(3)
  void clearMountType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get hostPath => $_getSZ(3);
  @$pb.TagNumber(4)
  set hostPath($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasHostPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearHostPath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get volumeName => $_getSZ(4);
  @$pb.TagNumber(5)
  set volumeName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasVolumeName() => $_has(4);
  @$pb.TagNumber(5)
  void clearVolumeName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get filePath => $_getSZ(5);
  @$pb.TagNumber(6)
  set filePath($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasFilePath() => $_has(5);
  @$pb.TagNumber(6)
  void clearFilePath() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get mountPath => $_getSZ(7);
  @$pb.TagNumber(8)
  set mountPath($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasMountPath() => $_has(7);
  @$pb.TagNumber(8)
  void clearMountPath() => $_clearField(8);
}

class UpdateMountRequest extends $pb.GeneratedMessage {
  factory UpdateMountRequest({
    DatabaseKind? kind,
    $fixnum.Int64? databaseId,
    $fixnum.Int64? id,
    $core.String? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = UpdateMountRequest._();
    if (kind != null) result.kind = kind;
    if (databaseId != null) result.databaseId = databaseId;
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  UpdateMountRequest._();

  factory UpdateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateMountRequest()..mergeFromBuffer(data, registry);
  factory UpdateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateMountRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'databaseId')
    ..aInt64(3, _omitFieldNames ? '' : 'id')
    ..aOS(4, _omitFieldNames ? '' : 'mountType')
    ..aOS(5, _omitFieldNames ? '' : 'hostPath')
    ..aOS(6, _omitFieldNames ? '' : 'volumeName')
    ..aOS(7, _omitFieldNames ? '' : 'filePath')
    ..aOS(8, _omitFieldNames ? '' : 'content')
    ..aOS(9, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateMountRequest copyWith(void Function(UpdateMountRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateMountRequest))
          as UpdateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use UpdateMountRequest() / UpdateMountRequest.new instead')
  static UpdateMountRequest create() => UpdateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateMountRequest._();
  @$core.override
  UpdateMountRequest createEmptyInstance() => UpdateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateMountRequest>(
          UpdateMountRequest.$_createMessage);
  static UpdateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get databaseId => $_getI64(1);
  @$pb.TagNumber(2)
  set databaseId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDatabaseId() => $_has(1);
  @$pb.TagNumber(2)
  void clearDatabaseId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get id => $_getI64(2);
  @$pb.TagNumber(3)
  set id($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasId() => $_has(2);
  @$pb.TagNumber(3)
  void clearId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get mountType => $_getSZ(3);
  @$pb.TagNumber(4)
  set mountType($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMountType() => $_has(3);
  @$pb.TagNumber(4)
  void clearMountType() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get hostPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set hostPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHostPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearHostPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get volumeName => $_getSZ(5);
  @$pb.TagNumber(6)
  set volumeName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasVolumeName() => $_has(5);
  @$pb.TagNumber(6)
  void clearVolumeName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get filePath => $_getSZ(6);
  @$pb.TagNumber(7)
  set filePath($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasFilePath() => $_has(6);
  @$pb.TagNumber(7)
  void clearFilePath() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get content => $_getSZ(7);
  @$pb.TagNumber(8)
  set content($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasContent() => $_has(7);
  @$pb.TagNumber(8)
  void clearContent() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get mountPath => $_getSZ(8);
  @$pb.TagNumber(9)
  set mountPath($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasMountPath() => $_has(8);
  @$pb.TagNumber(9)
  void clearMountPath() => $_clearField(9);
}

class DeleteMountRequest extends $pb.GeneratedMessage {
  factory DeleteMountRequest({
    DatabaseKind? kind,
    $fixnum.Int64? databaseId,
    $fixnum.Int64? id,
  }) {
    final result = DeleteMountRequest._();
    if (kind != null) result.kind = kind;
    if (databaseId != null) result.databaseId = databaseId;
    if (id != null) result.id = id;
    return result;
  }

  DeleteMountRequest._();

  factory DeleteMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteMountRequest()..mergeFromBuffer(data, registry);
  factory DeleteMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteMountRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'databaseId')
    ..aInt64(3, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteMountRequest copyWith(void Function(DeleteMountRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteMountRequest))
          as DeleteMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DeleteMountRequest() / DeleteMountRequest.new instead')
  static DeleteMountRequest create() => DeleteMountRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteMountRequest._();
  @$core.override
  DeleteMountRequest createEmptyInstance() => DeleteMountRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteMountRequest>(
          DeleteMountRequest.$_createMessage);
  static DeleteMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get databaseId => $_getI64(1);
  @$pb.TagNumber(2)
  set databaseId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDatabaseId() => $_has(1);
  @$pb.TagNumber(2)
  void clearDatabaseId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get id => $_getI64(2);
  @$pb.TagNumber(3)
  set id($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasId() => $_has(2);
  @$pb.TagNumber(3)
  void clearId() => $_clearField(3);
}

class Dependencies extends $pb.GeneratedMessage {
  factory Dependencies({
    $core.Iterable<$core.String>? items,
  }) {
    final result = Dependencies._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  Dependencies._();

  factory Dependencies.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Dependencies()..mergeFromBuffer(data, registry);
  factory Dependencies.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Dependencies()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Dependencies',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Dependencies.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Dependencies clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Dependencies copyWith(void Function(Dependencies) updates) =>
      super.copyWith((message) => updates(message as Dependencies))
          as Dependencies;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Dependencies() / Dependencies.new instead')
  static Dependencies create() => Dependencies._();
  static $pb.GeneratedMessage $_createMessage() => Dependencies._();
  @$core.override
  Dependencies createEmptyInstance() => Dependencies._();
  @$core.pragma('dart2js:noInline')
  static Dependencies getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Dependencies>(
          Dependencies.$_createMessage);
  static Dependencies? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ExportDatabaseRequest extends $pb.GeneratedMessage {
  factory ExportDatabaseRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
    $core.bool? includeSecrets,
  }) {
    final result = ExportDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    if (includeSecrets != null) result.includeSecrets = includeSecrets;
    return result;
  }

  ExportDatabaseRequest._();

  factory ExportDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportDatabaseRequest()..mergeFromBuffer(data, registry);
  factory ExportDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExportDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ExportDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOB(3, _omitFieldNames ? '' : 'includeSecrets')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportDatabaseRequest copyWith(
          void Function(ExportDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as ExportDatabaseRequest))
          as ExportDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ExportDatabaseRequest() / ExportDatabaseRequest.new instead')
  static ExportDatabaseRequest create() => ExportDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => ExportDatabaseRequest._();
  @$core.override
  ExportDatabaseRequest createEmptyInstance() => ExportDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static ExportDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExportDatabaseRequest>(
          ExportDatabaseRequest.$_createMessage);
  static ExportDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get includeSecrets => $_getBF(2);
  @$pb.TagNumber(3)
  set includeSecrets($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasIncludeSecrets() => $_has(2);
  @$pb.TagNumber(3)
  void clearIncludeSecrets() => $_clearField(3);
}

class DatabaseExport extends $pb.GeneratedMessage {
  factory DatabaseExport({
    $core.String? archive,
  }) {
    final result = DatabaseExport._();
    if (archive != null) result.archive = archive;
    return result;
  }

  DatabaseExport._();

  factory DatabaseExport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseExport()..mergeFromBuffer(data, registry);
  factory DatabaseExport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseExport()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseExport',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseExport.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseExport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseExport copyWith(void Function(DatabaseExport) updates) =>
      super.copyWith((message) => updates(message as DatabaseExport))
          as DatabaseExport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DatabaseExport() / DatabaseExport.new instead')
  static DatabaseExport create() => DatabaseExport._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseExport._();
  @$core.override
  DatabaseExport createEmptyInstance() => DatabaseExport._();
  @$core.pragma('dart2js:noInline')
  static DatabaseExport getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DatabaseExport>(
          DatabaseExport.$_createMessage);
  static DatabaseExport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get archive => $_getSZ(0);
  @$pb.TagNumber(1)
  set archive($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);
}

class ImportDatabaseRequest extends $pb.GeneratedMessage {
  factory ImportDatabaseRequest({
    DatabaseKind? kind,
    $fixnum.Int64? environmentId,
    $core.String? archive,
  }) {
    final result = ImportDatabaseRequest._();
    if (kind != null) result.kind = kind;
    if (environmentId != null) result.environmentId = environmentId;
    if (archive != null) result.archive = archive;
    return result;
  }

  ImportDatabaseRequest._();

  factory ImportDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportDatabaseRequest()..mergeFromBuffer(data, registry);
  factory ImportDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ImportDatabaseRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..aOS(3, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportDatabaseRequest copyWith(
          void Function(ImportDatabaseRequest) updates) =>
      super.copyWith((message) => updates(message as ImportDatabaseRequest))
          as ImportDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportDatabaseRequest() / ImportDatabaseRequest.new instead')
  static ImportDatabaseRequest create() => ImportDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() => ImportDatabaseRequest._();
  @$core.override
  ImportDatabaseRequest createEmptyInstance() => ImportDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static ImportDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportDatabaseRequest>(
          ImportDatabaseRequest.$_createMessage);
  static ImportDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get archive => $_getSZ(2);
  @$pb.TagNumber(3)
  set archive($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasArchive() => $_has(2);
  @$pb.TagNumber(3)
  void clearArchive() => $_clearField(3);
}

class DatabaseConnection extends $pb.GeneratedMessage {
  factory DatabaseConnection({
    DatabaseKind? kind,
    $core.String? host,
    $fixnum.Int64? port,
    $core.String? database,
    $core.String? username,
  }) {
    final result = DatabaseConnection._();
    if (kind != null) result.kind = kind;
    if (host != null) result.host = host;
    if (port != null) result.port = port;
    if (database != null) result.database = database;
    if (username != null) result.username = username;
    return result;
  }

  DatabaseConnection._();

  factory DatabaseConnection.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseConnection()..mergeFromBuffer(data, registry);
  factory DatabaseConnection.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseConnection()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseConnection',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseConnection.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aOS(2, _omitFieldNames ? '' : 'host')
    ..aInt64(3, _omitFieldNames ? '' : 'port')
    ..aOS(4, _omitFieldNames ? '' : 'database')
    ..aOS(5, _omitFieldNames ? '' : 'username')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseConnection clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseConnection copyWith(void Function(DatabaseConnection) updates) =>
      super.copyWith((message) => updates(message as DatabaseConnection))
          as DatabaseConnection;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DatabaseConnection() / DatabaseConnection.new instead')
  static DatabaseConnection create() => DatabaseConnection._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseConnection._();
  @$core.override
  DatabaseConnection createEmptyInstance() => DatabaseConnection._();
  @$core.pragma('dart2js:noInline')
  static DatabaseConnection getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseConnection>(
          DatabaseConnection.$_createMessage);
  static DatabaseConnection? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get host => $_getSZ(1);
  @$pb.TagNumber(2)
  set host($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHost() => $_has(1);
  @$pb.TagNumber(2)
  void clearHost() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get port => $_getI64(2);
  @$pb.TagNumber(3)
  set port($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPort() => $_has(2);
  @$pb.TagNumber(3)
  void clearPort() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get database => $_getSZ(3);
  @$pb.TagNumber(4)
  set database($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDatabase() => $_has(3);
  @$pb.TagNumber(4)
  void clearDatabase() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get username => $_getSZ(4);
  @$pb.TagNumber(5)
  set username($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUsername() => $_has(4);
  @$pb.TagNumber(5)
  void clearUsername() => $_clearField(5);
}

class ValidationResult extends $pb.GeneratedMessage {
  factory ValidationResult({
    $core.bool? valid,
    $core.String? message,
  }) {
    final result = ValidationResult._();
    if (valid != null) result.valid = valid;
    if (message != null) result.message = message;
    return result;
  }

  ValidationResult._();

  factory ValidationResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidationResult()..mergeFromBuffer(data, registry);
  factory ValidationResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidationResult()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ValidationResult',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ValidationResult.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'valid')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidationResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidationResult copyWith(void Function(ValidationResult) updates) =>
      super.copyWith((message) => updates(message as ValidationResult))
          as ValidationResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ValidationResult() / ValidationResult.new instead')
  static ValidationResult create() => ValidationResult._();
  static $pb.GeneratedMessage $_createMessage() => ValidationResult._();
  @$core.override
  ValidationResult createEmptyInstance() => ValidationResult._();
  @$core.pragma('dart2js:noInline')
  static ValidationResult getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ValidationResult>(
          ValidationResult.$_createMessage);
  static ValidationResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get valid => $_getBF(0);
  @$pb.TagNumber(1)
  set valid($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasValid() => $_has(0);
  @$pb.TagNumber(1)
  void clearValid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

class AdvancedConfig extends $pb.GeneratedMessage {
  factory AdvancedConfig({
    $core.String? replicationRole,
    $core.String? config,
  }) {
    final result = AdvancedConfig._();
    if (replicationRole != null) result.replicationRole = replicationRole;
    if (config != null) result.config = config;
    return result;
  }

  AdvancedConfig._();

  factory AdvancedConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AdvancedConfig()..mergeFromBuffer(data, registry);
  factory AdvancedConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AdvancedConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AdvancedConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AdvancedConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'replicationRole')
    ..aOS(2, _omitFieldNames ? '' : 'config')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AdvancedConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AdvancedConfig copyWith(void Function(AdvancedConfig) updates) =>
      super.copyWith((message) => updates(message as AdvancedConfig))
          as AdvancedConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AdvancedConfig() / AdvancedConfig.new instead')
  static AdvancedConfig create() => AdvancedConfig._();
  static $pb.GeneratedMessage $_createMessage() => AdvancedConfig._();
  @$core.override
  AdvancedConfig createEmptyInstance() => AdvancedConfig._();
  @$core.pragma('dart2js:noInline')
  static AdvancedConfig getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AdvancedConfig>(
          AdvancedConfig.$_createMessage);
  static AdvancedConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get replicationRole => $_getSZ(0);
  @$pb.TagNumber(1)
  set replicationRole($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasReplicationRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearReplicationRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get config => $_getSZ(1);
  @$pb.TagNumber(2)
  set config($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConfig() => $_has(1);
  @$pb.TagNumber(2)
  void clearConfig() => $_clearField(2);
}

class UpdateAdvancedConfigRequest extends $pb.GeneratedMessage {
  factory UpdateAdvancedConfigRequest({
    DatabaseKind? kind,
    $fixnum.Int64? id,
    AdvancedConfig? config,
  }) {
    final result = UpdateAdvancedConfigRequest._();
    if (kind != null) result.kind = kind;
    if (id != null) result.id = id;
    if (config != null) result.config = config;
    return result;
  }

  UpdateAdvancedConfigRequest._();

  factory UpdateAdvancedConfigRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAdvancedConfigRequest()..mergeFromBuffer(data, registry);
  factory UpdateAdvancedConfigRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAdvancedConfigRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateAdvancedConfigRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateAdvancedConfigRequest.$_createMessage)
    ..aE<DatabaseKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: DatabaseKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOM<AdvancedConfig>(3, _omitFieldNames ? '' : 'config',
        subBuilder: AdvancedConfig.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAdvancedConfigRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAdvancedConfigRequest copyWith(
          void Function(UpdateAdvancedConfigRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateAdvancedConfigRequest))
          as UpdateAdvancedConfigRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateAdvancedConfigRequest() / UpdateAdvancedConfigRequest.new instead')
  static UpdateAdvancedConfigRequest create() =>
      UpdateAdvancedConfigRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateAdvancedConfigRequest._();
  @$core.override
  UpdateAdvancedConfigRequest createEmptyInstance() =>
      UpdateAdvancedConfigRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateAdvancedConfigRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateAdvancedConfigRequest>(
          UpdateAdvancedConfigRequest.$_createMessage);
  static UpdateAdvancedConfigRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(DatabaseKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  AdvancedConfig get config => $_getN(2);
  @$pb.TagNumber(3)
  set config(AdvancedConfig value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasConfig() => $_has(2);
  @$pb.TagNumber(3)
  void clearConfig() => $_clearField(3);
  @$pb.TagNumber(3)
  AdvancedConfig ensureConfig() => $_ensure(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

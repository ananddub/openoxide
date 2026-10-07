// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database_network.proto.

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

class DatabaseNetwork extends $pb.GeneratedMessage {
  factory DatabaseNetwork({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? dockerNetworkName,
    $core.String? description,
    $fixnum.Int64? external,
    $fixnum.Int64? serverId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = DatabaseNetwork._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (dockerNetworkName != null) result.dockerNetworkName = dockerNetworkName;
    if (description != null) result.description = description;
    if (external != null) result.external = external;
    if (serverId != null) result.serverId = serverId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  DatabaseNetwork._();

  factory DatabaseNetwork.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetwork()..mergeFromBuffer(data, registry);
  factory DatabaseNetwork.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetwork()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetwork',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetwork.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'dockerNetworkName')
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aInt64(5, _omitFieldNames ? '' : 'external')
    ..aInt64(6, _omitFieldNames ? '' : 'serverId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetwork clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetwork copyWith(void Function(DatabaseNetwork) updates) =>
      super.copyWith((message) => updates(message as DatabaseNetwork))
          as DatabaseNetwork;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DatabaseNetwork() / DatabaseNetwork.new instead')
  static DatabaseNetwork create() => DatabaseNetwork._();
  static $pb.GeneratedMessage $_createMessage() => DatabaseNetwork._();
  @$core.override
  DatabaseNetwork createEmptyInstance() => DatabaseNetwork._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetwork getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DatabaseNetwork>(
          DatabaseNetwork.$_createMessage);
  static DatabaseNetwork? _defaultInstance;

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
  $core.String get dockerNetworkName => $_getSZ(2);
  @$pb.TagNumber(3)
  set dockerNetworkName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDockerNetworkName() => $_has(2);
  @$pb.TagNumber(3)
  void clearDockerNetworkName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => $_clearField(4);

  /// Docker external-network flag stored as an integer (0/1) column.
  @$pb.TagNumber(5)
  $fixnum.Int64 get external => $_getI64(4);
  @$pb.TagNumber(5)
  set external($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasExternal() => $_has(4);
  @$pb.TagNumber(5)
  void clearExternal() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get serverId => $_getI64(5);
  @$pb.TagNumber(6)
  set serverId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServerId() => $_has(5);
  @$pb.TagNumber(6)
  void clearServerId() => $_clearField(6);

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

class DatabaseNetworkDependencies extends $pb.GeneratedMessage {
  factory DatabaseNetworkDependencies({
    $fixnum.Int64? applications,
    $fixnum.Int64? composeServices,
    $fixnum.Int64? databases,
  }) {
    final result = DatabaseNetworkDependencies._();
    if (applications != null) result.applications = applications;
    if (composeServices != null) result.composeServices = composeServices;
    if (databases != null) result.databases = databases;
    return result;
  }

  DatabaseNetworkDependencies._();

  factory DatabaseNetworkDependencies.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkDependencies()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkDependencies.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkDependencies()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkDependencies',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkDependencies.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applications')
    ..aInt64(2, _omitFieldNames ? '' : 'composeServices')
    ..aInt64(3, _omitFieldNames ? '' : 'databases')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkDependencies clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkDependencies copyWith(
          void Function(DatabaseNetworkDependencies) updates) =>
      super.copyWith(
              (message) => updates(message as DatabaseNetworkDependencies))
          as DatabaseNetworkDependencies;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkDependencies() / DatabaseNetworkDependencies.new instead')
  static DatabaseNetworkDependencies create() =>
      DatabaseNetworkDependencies._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkDependencies._();
  @$core.override
  DatabaseNetworkDependencies createEmptyInstance() =>
      DatabaseNetworkDependencies._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkDependencies getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseNetworkDependencies>(
          DatabaseNetworkDependencies.$_createMessage);
  static DatabaseNetworkDependencies? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applications => $_getI64(0);
  @$pb.TagNumber(1)
  set applications($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplications() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplications() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get composeServices => $_getI64(1);
  @$pb.TagNumber(2)
  set composeServices($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasComposeServices() => $_has(1);
  @$pb.TagNumber(2)
  void clearComposeServices() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get databases => $_getI64(2);
  @$pb.TagNumber(3)
  set databases($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDatabases() => $_has(2);
  @$pb.TagNumber(3)
  void clearDatabases() => $_clearField(3);
}

class DatabaseNetworkListNetworksRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkListNetworksRequest() =>
      DatabaseNetworkListNetworksRequest._();

  DatabaseNetworkListNetworksRequest._();

  factory DatabaseNetworkListNetworksRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListNetworksRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkListNetworksRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListNetworksRequest copyWith(
          void Function(DatabaseNetworkListNetworksRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListNetworksRequest))
          as DatabaseNetworkListNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListNetworksRequest() / DatabaseNetworkListNetworksRequest.new instead')
  static DatabaseNetworkListNetworksRequest create() =>
      DatabaseNetworkListNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListNetworksRequest._();
  @$core.override
  DatabaseNetworkListNetworksRequest createEmptyInstance() =>
      DatabaseNetworkListNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListNetworksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseNetworkListNetworksRequest>(
          DatabaseNetworkListNetworksRequest.$_createMessage);
  static DatabaseNetworkListNetworksRequest? _defaultInstance;
}

class DatabaseNetworkListNetworksResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkListNetworksResponse({
    $core.Iterable<DatabaseNetwork>? networks,
  }) {
    final result = DatabaseNetworkListNetworksResponse._();
    if (networks != null) result.networks.addAll(networks);
    return result;
  }

  DatabaseNetworkListNetworksResponse._();

  factory DatabaseNetworkListNetworksResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListNetworksResponse()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListNetworksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkListNetworksResponse.$_createMessage)
    ..pPM<DatabaseNetwork>(1, _omitFieldNames ? '' : 'networks',
        subBuilder: DatabaseNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListNetworksResponse copyWith(
          void Function(DatabaseNetworkListNetworksResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListNetworksResponse))
          as DatabaseNetworkListNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListNetworksResponse() / DatabaseNetworkListNetworksResponse.new instead')
  static DatabaseNetworkListNetworksResponse create() =>
      DatabaseNetworkListNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListNetworksResponse._();
  @$core.override
  DatabaseNetworkListNetworksResponse createEmptyInstance() =>
      DatabaseNetworkListNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListNetworksResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkListNetworksResponse>(
          DatabaseNetworkListNetworksResponse.$_createMessage);
  static DatabaseNetworkListNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DatabaseNetwork> get networks => $_getList(0);
}

class DatabaseNetworkListServerNetworksRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkListServerNetworksRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = DatabaseNetworkListServerNetworksRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  DatabaseNetworkListServerNetworksRequest._();

  factory DatabaseNetworkListServerNetworksRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListServerNetworksRequest()
        ..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListServerNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListServerNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListServerNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          DatabaseNetworkListServerNetworksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListServerNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListServerNetworksRequest copyWith(
          void Function(DatabaseNetworkListServerNetworksRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListServerNetworksRequest))
          as DatabaseNetworkListServerNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListServerNetworksRequest() / DatabaseNetworkListServerNetworksRequest.new instead')
  static DatabaseNetworkListServerNetworksRequest create() =>
      DatabaseNetworkListServerNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListServerNetworksRequest._();
  @$core.override
  DatabaseNetworkListServerNetworksRequest createEmptyInstance() =>
      DatabaseNetworkListServerNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListServerNetworksRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkListServerNetworksRequest>(
          DatabaseNetworkListServerNetworksRequest.$_createMessage);
  static DatabaseNetworkListServerNetworksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class DatabaseNetworkListServerNetworksResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkListServerNetworksResponse({
    $core.Iterable<DatabaseNetwork>? networks,
  }) {
    final result = DatabaseNetworkListServerNetworksResponse._();
    if (networks != null) result.networks.addAll(networks);
    return result;
  }

  DatabaseNetworkListServerNetworksResponse._();

  factory DatabaseNetworkListServerNetworksResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListServerNetworksResponse()
        ..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListServerNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListServerNetworksResponse()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListServerNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          DatabaseNetworkListServerNetworksResponse.$_createMessage)
    ..pPM<DatabaseNetwork>(1, _omitFieldNames ? '' : 'networks',
        subBuilder: DatabaseNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListServerNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListServerNetworksResponse copyWith(
          void Function(DatabaseNetworkListServerNetworksResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListServerNetworksResponse))
          as DatabaseNetworkListServerNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListServerNetworksResponse() / DatabaseNetworkListServerNetworksResponse.new instead')
  static DatabaseNetworkListServerNetworksResponse create() =>
      DatabaseNetworkListServerNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListServerNetworksResponse._();
  @$core.override
  DatabaseNetworkListServerNetworksResponse createEmptyInstance() =>
      DatabaseNetworkListServerNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListServerNetworksResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkListServerNetworksResponse>(
          DatabaseNetworkListServerNetworksResponse.$_createMessage);
  static DatabaseNetworkListServerNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DatabaseNetwork> get networks => $_getList(0);
}

class DatabaseNetworkGetNetworkRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkGetNetworkRequest({
    $fixnum.Int64? id,
  }) {
    final result = DatabaseNetworkGetNetworkRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DatabaseNetworkGetNetworkRequest._();

  factory DatabaseNetworkGetNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkGetNetworkRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkGetNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkGetNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkGetNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkGetNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkGetNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkGetNetworkRequest copyWith(
          void Function(DatabaseNetworkGetNetworkRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DatabaseNetworkGetNetworkRequest))
          as DatabaseNetworkGetNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkGetNetworkRequest() / DatabaseNetworkGetNetworkRequest.new instead')
  static DatabaseNetworkGetNetworkRequest create() =>
      DatabaseNetworkGetNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkGetNetworkRequest._();
  @$core.override
  DatabaseNetworkGetNetworkRequest createEmptyInstance() =>
      DatabaseNetworkGetNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkGetNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseNetworkGetNetworkRequest>(
          DatabaseNetworkGetNetworkRequest.$_createMessage);
  static DatabaseNetworkGetNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DatabaseNetworkGetNetworkResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkGetNetworkResponse({
    DatabaseNetwork? network,
  }) {
    final result = DatabaseNetworkGetNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  DatabaseNetworkGetNetworkResponse._();

  factory DatabaseNetworkGetNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkGetNetworkResponse()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkGetNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkGetNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkGetNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkGetNetworkResponse.$_createMessage)
    ..aOM<DatabaseNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: DatabaseNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkGetNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkGetNetworkResponse copyWith(
          void Function(DatabaseNetworkGetNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkGetNetworkResponse))
          as DatabaseNetworkGetNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkGetNetworkResponse() / DatabaseNetworkGetNetworkResponse.new instead')
  static DatabaseNetworkGetNetworkResponse create() =>
      DatabaseNetworkGetNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkGetNetworkResponse._();
  @$core.override
  DatabaseNetworkGetNetworkResponse createEmptyInstance() =>
      DatabaseNetworkGetNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkGetNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DatabaseNetworkGetNetworkResponse>(
          DatabaseNetworkGetNetworkResponse.$_createMessage);
  static DatabaseNetworkGetNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(DatabaseNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  DatabaseNetwork ensureNetwork() => $_ensure(0);
}

class DatabaseNetworkCreateNetworkRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkCreateNetworkRequest({
    $core.String? name,
    $core.String? dockerNetworkName,
    $core.String? description,
    $fixnum.Int64? external,
    $fixnum.Int64? serverId,
  }) {
    final result = DatabaseNetworkCreateNetworkRequest._();
    if (name != null) result.name = name;
    if (dockerNetworkName != null) result.dockerNetworkName = dockerNetworkName;
    if (description != null) result.description = description;
    if (external != null) result.external = external;
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  DatabaseNetworkCreateNetworkRequest._();

  factory DatabaseNetworkCreateNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkCreateNetworkRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkCreateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkCreateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkCreateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkCreateNetworkRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'dockerNetworkName')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aInt64(4, _omitFieldNames ? '' : 'external')
    ..aInt64(5, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkCreateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkCreateNetworkRequest copyWith(
          void Function(DatabaseNetworkCreateNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkCreateNetworkRequest))
          as DatabaseNetworkCreateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkCreateNetworkRequest() / DatabaseNetworkCreateNetworkRequest.new instead')
  static DatabaseNetworkCreateNetworkRequest create() =>
      DatabaseNetworkCreateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkCreateNetworkRequest._();
  @$core.override
  DatabaseNetworkCreateNetworkRequest createEmptyInstance() =>
      DatabaseNetworkCreateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkCreateNetworkRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkCreateNetworkRequest>(
          DatabaseNetworkCreateNetworkRequest.$_createMessage);
  static DatabaseNetworkCreateNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get dockerNetworkName => $_getSZ(1);
  @$pb.TagNumber(2)
  set dockerNetworkName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDockerNetworkName() => $_has(1);
  @$pb.TagNumber(2)
  void clearDockerNetworkName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get external => $_getI64(3);
  @$pb.TagNumber(4)
  set external($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasExternal() => $_has(3);
  @$pb.TagNumber(4)
  void clearExternal() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get serverId => $_getI64(4);
  @$pb.TagNumber(5)
  set serverId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServerId() => $_has(4);
  @$pb.TagNumber(5)
  void clearServerId() => $_clearField(5);
}

class DatabaseNetworkCreateNetworkResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkCreateNetworkResponse({
    DatabaseNetwork? network,
  }) {
    final result = DatabaseNetworkCreateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  DatabaseNetworkCreateNetworkResponse._();

  factory DatabaseNetworkCreateNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkCreateNetworkResponse()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkCreateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkCreateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkCreateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkCreateNetworkResponse.$_createMessage)
    ..aOM<DatabaseNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: DatabaseNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkCreateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkCreateNetworkResponse copyWith(
          void Function(DatabaseNetworkCreateNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkCreateNetworkResponse))
          as DatabaseNetworkCreateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkCreateNetworkResponse() / DatabaseNetworkCreateNetworkResponse.new instead')
  static DatabaseNetworkCreateNetworkResponse create() =>
      DatabaseNetworkCreateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkCreateNetworkResponse._();
  @$core.override
  DatabaseNetworkCreateNetworkResponse createEmptyInstance() =>
      DatabaseNetworkCreateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkCreateNetworkResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkCreateNetworkResponse>(
          DatabaseNetworkCreateNetworkResponse.$_createMessage);
  static DatabaseNetworkCreateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(DatabaseNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  DatabaseNetwork ensureNetwork() => $_ensure(0);
}

class DatabaseNetworkUpdateNetworkRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkUpdateNetworkRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? dockerNetworkName,
    $core.String? description,
    $fixnum.Int64? external,
    $fixnum.Int64? serverId,
  }) {
    final result = DatabaseNetworkUpdateNetworkRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (dockerNetworkName != null) result.dockerNetworkName = dockerNetworkName;
    if (description != null) result.description = description;
    if (external != null) result.external = external;
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  DatabaseNetworkUpdateNetworkRequest._();

  factory DatabaseNetworkUpdateNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkUpdateNetworkRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkUpdateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkUpdateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkUpdateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkUpdateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'dockerNetworkName')
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aInt64(5, _omitFieldNames ? '' : 'external')
    ..aInt64(6, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkUpdateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkUpdateNetworkRequest copyWith(
          void Function(DatabaseNetworkUpdateNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkUpdateNetworkRequest))
          as DatabaseNetworkUpdateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkUpdateNetworkRequest() / DatabaseNetworkUpdateNetworkRequest.new instead')
  static DatabaseNetworkUpdateNetworkRequest create() =>
      DatabaseNetworkUpdateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkUpdateNetworkRequest._();
  @$core.override
  DatabaseNetworkUpdateNetworkRequest createEmptyInstance() =>
      DatabaseNetworkUpdateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkUpdateNetworkRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkUpdateNetworkRequest>(
          DatabaseNetworkUpdateNetworkRequest.$_createMessage);
  static DatabaseNetworkUpdateNetworkRequest? _defaultInstance;

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
  $core.String get dockerNetworkName => $_getSZ(2);
  @$pb.TagNumber(3)
  set dockerNetworkName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDockerNetworkName() => $_has(2);
  @$pb.TagNumber(3)
  void clearDockerNetworkName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get external => $_getI64(4);
  @$pb.TagNumber(5)
  set external($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasExternal() => $_has(4);
  @$pb.TagNumber(5)
  void clearExternal() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get serverId => $_getI64(5);
  @$pb.TagNumber(6)
  set serverId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServerId() => $_has(5);
  @$pb.TagNumber(6)
  void clearServerId() => $_clearField(6);
}

class DatabaseNetworkUpdateNetworkResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkUpdateNetworkResponse({
    DatabaseNetwork? network,
  }) {
    final result = DatabaseNetworkUpdateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  DatabaseNetworkUpdateNetworkResponse._();

  factory DatabaseNetworkUpdateNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkUpdateNetworkResponse()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkUpdateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkUpdateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkUpdateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkUpdateNetworkResponse.$_createMessage)
    ..aOM<DatabaseNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: DatabaseNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkUpdateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkUpdateNetworkResponse copyWith(
          void Function(DatabaseNetworkUpdateNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkUpdateNetworkResponse))
          as DatabaseNetworkUpdateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkUpdateNetworkResponse() / DatabaseNetworkUpdateNetworkResponse.new instead')
  static DatabaseNetworkUpdateNetworkResponse create() =>
      DatabaseNetworkUpdateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkUpdateNetworkResponse._();
  @$core.override
  DatabaseNetworkUpdateNetworkResponse createEmptyInstance() =>
      DatabaseNetworkUpdateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkUpdateNetworkResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkUpdateNetworkResponse>(
          DatabaseNetworkUpdateNetworkResponse.$_createMessage);
  static DatabaseNetworkUpdateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(DatabaseNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  DatabaseNetwork ensureNetwork() => $_ensure(0);
}

class DatabaseNetworkDeleteNetworkRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkDeleteNetworkRequest({
    $fixnum.Int64? id,
  }) {
    final result = DatabaseNetworkDeleteNetworkRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DatabaseNetworkDeleteNetworkRequest._();

  factory DatabaseNetworkDeleteNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkDeleteNetworkRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkDeleteNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkDeleteNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkDeleteNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DatabaseNetworkDeleteNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkDeleteNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkDeleteNetworkRequest copyWith(
          void Function(DatabaseNetworkDeleteNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkDeleteNetworkRequest))
          as DatabaseNetworkDeleteNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkDeleteNetworkRequest() / DatabaseNetworkDeleteNetworkRequest.new instead')
  static DatabaseNetworkDeleteNetworkRequest create() =>
      DatabaseNetworkDeleteNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkDeleteNetworkRequest._();
  @$core.override
  DatabaseNetworkDeleteNetworkRequest createEmptyInstance() =>
      DatabaseNetworkDeleteNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkDeleteNetworkRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkDeleteNetworkRequest>(
          DatabaseNetworkDeleteNetworkRequest.$_createMessage);
  static DatabaseNetworkDeleteNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DatabaseNetworkListDependenciesRequest extends $pb.GeneratedMessage {
  factory DatabaseNetworkListDependenciesRequest({
    $fixnum.Int64? id,
  }) {
    final result = DatabaseNetworkListDependenciesRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DatabaseNetworkListDependenciesRequest._();

  factory DatabaseNetworkListDependenciesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListDependenciesRequest()..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListDependenciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListDependenciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListDependenciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          DatabaseNetworkListDependenciesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListDependenciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListDependenciesRequest copyWith(
          void Function(DatabaseNetworkListDependenciesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListDependenciesRequest))
          as DatabaseNetworkListDependenciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListDependenciesRequest() / DatabaseNetworkListDependenciesRequest.new instead')
  static DatabaseNetworkListDependenciesRequest create() =>
      DatabaseNetworkListDependenciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListDependenciesRequest._();
  @$core.override
  DatabaseNetworkListDependenciesRequest createEmptyInstance() =>
      DatabaseNetworkListDependenciesRequest._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListDependenciesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkListDependenciesRequest>(
          DatabaseNetworkListDependenciesRequest.$_createMessage);
  static DatabaseNetworkListDependenciesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DatabaseNetworkListDependenciesResponse extends $pb.GeneratedMessage {
  factory DatabaseNetworkListDependenciesResponse({
    DatabaseNetworkDependencies? dependencies,
  }) {
    final result = DatabaseNetworkListDependenciesResponse._();
    if (dependencies != null) result.dependencies = dependencies;
    return result;
  }

  DatabaseNetworkListDependenciesResponse._();

  factory DatabaseNetworkListDependenciesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListDependenciesResponse()
        ..mergeFromBuffer(data, registry);
  factory DatabaseNetworkListDependenciesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DatabaseNetworkListDependenciesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DatabaseNetworkListDependenciesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          DatabaseNetworkListDependenciesResponse.$_createMessage)
    ..aOM<DatabaseNetworkDependencies>(1, _omitFieldNames ? '' : 'dependencies',
        subBuilder: DatabaseNetworkDependencies.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListDependenciesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DatabaseNetworkListDependenciesResponse copyWith(
          void Function(DatabaseNetworkListDependenciesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as DatabaseNetworkListDependenciesResponse))
          as DatabaseNetworkListDependenciesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DatabaseNetworkListDependenciesResponse() / DatabaseNetworkListDependenciesResponse.new instead')
  static DatabaseNetworkListDependenciesResponse create() =>
      DatabaseNetworkListDependenciesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      DatabaseNetworkListDependenciesResponse._();
  @$core.override
  DatabaseNetworkListDependenciesResponse createEmptyInstance() =>
      DatabaseNetworkListDependenciesResponse._();
  @$core.pragma('dart2js:noInline')
  static DatabaseNetworkListDependenciesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              DatabaseNetworkListDependenciesResponse>(
          DatabaseNetworkListDependenciesResponse.$_createMessage);
  static DatabaseNetworkListDependenciesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DatabaseNetworkDependencies get dependencies => $_getN(0);
  @$pb.TagNumber(1)
  set dependencies(DatabaseNetworkDependencies value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDependencies() => $_has(0);
  @$pb.TagNumber(1)
  void clearDependencies() => $_clearField(1);
  @$pb.TagNumber(1)
  DatabaseNetworkDependencies ensureDependencies() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'server.pbenum.dart';
import 'server_controller.pb.dart' as $2;
import 'server_private_network.pb.dart' as $3;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'server.pbenum.dart';

class Server extends $pb.GeneratedMessage {
  factory Server({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    $core.String? appName,
    ServerStatus? serverStatus,
    ServerType? serverType,
    $core.bool? enableDockerCleanup,
    $core.String? logCleanupCron,
    $core.String? command,
    $core.String? metricsConfig,
    $fixnum.Int64? sshKeyId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.String? buildMemoryLimit,
    $core.String? buildCpuLimit,
  }) {
    final result = Server._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (ipAddress != null) result.ipAddress = ipAddress;
    if (port != null) result.port = port;
    if (username != null) result.username = username;
    if (appName != null) result.appName = appName;
    if (serverStatus != null) result.serverStatus = serverStatus;
    if (serverType != null) result.serverType = serverType;
    if (enableDockerCleanup != null)
      result.enableDockerCleanup = enableDockerCleanup;
    if (logCleanupCron != null) result.logCleanupCron = logCleanupCron;
    if (command != null) result.command = command;
    if (metricsConfig != null) result.metricsConfig = metricsConfig;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (buildMemoryLimit != null) result.buildMemoryLimit = buildMemoryLimit;
    if (buildCpuLimit != null) result.buildCpuLimit = buildCpuLimit;
    return result;
  }

  Server._();

  factory Server.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Server()..mergeFromBuffer(data, registry);
  factory Server.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Server()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Server',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Server.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(5, _omitFieldNames ? '' : 'port')
    ..aOS(6, _omitFieldNames ? '' : 'username')
    ..aOS(7, _omitFieldNames ? '' : 'appName')
    ..aE<ServerStatus>(8, _omitFieldNames ? '' : 'serverStatus',
        enumValues: ServerStatus.values)
    ..aE<ServerType>(9, _omitFieldNames ? '' : 'serverType',
        enumValues: ServerType.values)
    ..aOB(10, _omitFieldNames ? '' : 'enableDockerCleanup')
    ..aOS(11, _omitFieldNames ? '' : 'logCleanupCron')
    ..aOS(12, _omitFieldNames ? '' : 'command')
    ..aOS(13, _omitFieldNames ? '' : 'metricsConfig')
    ..aInt64(14, _omitFieldNames ? '' : 'sshKeyId')
    ..aInt64(15, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(16, _omitFieldNames ? '' : 'updatedAt')
    ..aOS(17, _omitFieldNames ? '' : 'buildMemoryLimit')
    ..aOS(18, _omitFieldNames ? '' : 'buildCpuLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Server clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Server copyWith(void Function(Server) updates) =>
      super.copyWith((message) => updates(message as Server)) as Server;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Server() / Server.new instead')
  static Server create() => Server._();
  static $pb.GeneratedMessage $_createMessage() => Server._();
  @$core.override
  Server createEmptyInstance() => Server._();
  @$core.pragma('dart2js:noInline')
  static Server getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Server>(Server.$_createMessage);
  static Server? _defaultInstance;

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
  $core.String get ipAddress => $_getSZ(3);
  @$pb.TagNumber(4)
  set ipAddress($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasIpAddress() => $_has(3);
  @$pb.TagNumber(4)
  void clearIpAddress() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get port => $_getI64(4);
  @$pb.TagNumber(5)
  set port($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPort() => $_has(4);
  @$pb.TagNumber(5)
  void clearPort() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get username => $_getSZ(5);
  @$pb.TagNumber(6)
  set username($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasUsername() => $_has(5);
  @$pb.TagNumber(6)
  void clearUsername() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get appName => $_getSZ(6);
  @$pb.TagNumber(7)
  set appName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAppName() => $_has(6);
  @$pb.TagNumber(7)
  void clearAppName() => $_clearField(7);

  @$pb.TagNumber(8)
  ServerStatus get serverStatus => $_getN(7);
  @$pb.TagNumber(8)
  set serverStatus(ServerStatus value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasServerStatus() => $_has(7);
  @$pb.TagNumber(8)
  void clearServerStatus() => $_clearField(8);

  @$pb.TagNumber(9)
  ServerType get serverType => $_getN(8);
  @$pb.TagNumber(9)
  set serverType(ServerType value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasServerType() => $_has(8);
  @$pb.TagNumber(9)
  void clearServerType() => $_clearField(9);

  /// Integer-as-boolean in storage.
  @$pb.TagNumber(10)
  $core.bool get enableDockerCleanup => $_getBF(9);
  @$pb.TagNumber(10)
  set enableDockerCleanup($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasEnableDockerCleanup() => $_has(9);
  @$pb.TagNumber(10)
  void clearEnableDockerCleanup() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get logCleanupCron => $_getSZ(10);
  @$pb.TagNumber(11)
  set logCleanupCron($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLogCleanupCron() => $_has(10);
  @$pb.TagNumber(11)
  void clearLogCleanupCron() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get command => $_getSZ(11);
  @$pb.TagNumber(12)
  set command($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasCommand() => $_has(11);
  @$pb.TagNumber(12)
  void clearCommand() => $_clearField(12);

  /// Metrics collection configuration as a JSON document.
  @$pb.TagNumber(13)
  $core.String get metricsConfig => $_getSZ(12);
  @$pb.TagNumber(13)
  set metricsConfig($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasMetricsConfig() => $_has(12);
  @$pb.TagNumber(13)
  void clearMetricsConfig() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get sshKeyId => $_getI64(13);
  @$pb.TagNumber(14)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasSshKeyId() => $_has(13);
  @$pb.TagNumber(14)
  void clearSshKeyId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get createdAt => $_getI64(14);
  @$pb.TagNumber(15)
  set createdAt($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasCreatedAt() => $_has(14);
  @$pb.TagNumber(15)
  void clearCreatedAt() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get updatedAt => $_getI64(15);
  @$pb.TagNumber(16)
  set updatedAt($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasUpdatedAt() => $_has(15);
  @$pb.TagNumber(16)
  void clearUpdatedAt() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.String get buildMemoryLimit => $_getSZ(16);
  @$pb.TagNumber(17)
  set buildMemoryLimit($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasBuildMemoryLimit() => $_has(16);
  @$pb.TagNumber(17)
  void clearBuildMemoryLimit() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.String get buildCpuLimit => $_getSZ(17);
  @$pb.TagNumber(18)
  set buildCpuLimit($core.String value) => $_setString(17, value);
  @$pb.TagNumber(18)
  $core.bool hasBuildCpuLimit() => $_has(17);
  @$pb.TagNumber(18)
  void clearBuildCpuLimit() => $_clearField(18);
}

class ServerListServersRequest extends $pb.GeneratedMessage {
  factory ServerListServersRequest() => ServerListServersRequest._();

  ServerListServersRequest._();

  factory ServerListServersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListServersRequest()..mergeFromBuffer(data, registry);
  factory ServerListServersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListServersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListServersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListServersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListServersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListServersRequest copyWith(
          void Function(ServerListServersRequest) updates) =>
      super.copyWith((message) => updates(message as ServerListServersRequest))
          as ServerListServersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListServersRequest() / ServerListServersRequest.new instead')
  static ServerListServersRequest create() => ServerListServersRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServerListServersRequest._();
  @$core.override
  ServerListServersRequest createEmptyInstance() =>
      ServerListServersRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerListServersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListServersRequest>(
          ServerListServersRequest.$_createMessage);
  static ServerListServersRequest? _defaultInstance;
}

class ServerListServersResponse extends $pb.GeneratedMessage {
  factory ServerListServersResponse({
    $core.Iterable<Server>? servers,
  }) {
    final result = ServerListServersResponse._();
    if (servers != null) result.servers.addAll(servers);
    return result;
  }

  ServerListServersResponse._();

  factory ServerListServersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListServersResponse()..mergeFromBuffer(data, registry);
  factory ServerListServersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListServersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListServersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListServersResponse.$_createMessage)
    ..pPM<Server>(1, _omitFieldNames ? '' : 'servers',
        subBuilder: Server.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListServersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListServersResponse copyWith(
          void Function(ServerListServersResponse) updates) =>
      super.copyWith((message) => updates(message as ServerListServersResponse))
          as ServerListServersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListServersResponse() / ServerListServersResponse.new instead')
  static ServerListServersResponse create() => ServerListServersResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerListServersResponse._();
  @$core.override
  ServerListServersResponse createEmptyInstance() =>
      ServerListServersResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerListServersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListServersResponse>(
          ServerListServersResponse.$_createMessage);
  static ServerListServersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Server> get servers => $_getList(0);
}

class ServerGetServerRequest extends $pb.GeneratedMessage {
  factory ServerGetServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerGetServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerGetServerRequest._();

  factory ServerGetServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerRequest()..mergeFromBuffer(data, registry);
  factory ServerGetServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerGetServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerGetServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerRequest copyWith(
          void Function(ServerGetServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerGetServerRequest))
          as ServerGetServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerGetServerRequest() / ServerGetServerRequest.new instead')
  static ServerGetServerRequest create() => ServerGetServerRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServerGetServerRequest._();
  @$core.override
  ServerGetServerRequest createEmptyInstance() => ServerGetServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerGetServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerGetServerRequest>(
          ServerGetServerRequest.$_createMessage);
  static ServerGetServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerGetServerResponse extends $pb.GeneratedMessage {
  factory ServerGetServerResponse({
    Server? server,
  }) {
    final result = ServerGetServerResponse._();
    if (server != null) result.server = server;
    return result;
  }

  ServerGetServerResponse._();

  factory ServerGetServerResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerResponse()..mergeFromBuffer(data, registry);
  factory ServerGetServerResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerGetServerResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerGetServerResponse.$_createMessage)
    ..aOM<Server>(1, _omitFieldNames ? '' : 'server',
        subBuilder: Server.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerResponse copyWith(
          void Function(ServerGetServerResponse) updates) =>
      super.copyWith((message) => updates(message as ServerGetServerResponse))
          as ServerGetServerResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerGetServerResponse() / ServerGetServerResponse.new instead')
  static ServerGetServerResponse create() => ServerGetServerResponse._();
  static $pb.GeneratedMessage $_createMessage() => ServerGetServerResponse._();
  @$core.override
  ServerGetServerResponse createEmptyInstance() => ServerGetServerResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerGetServerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerGetServerResponse>(
          ServerGetServerResponse.$_createMessage);
  static ServerGetServerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Server get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(Server value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  Server ensureServer() => $_ensure(0);
}

class ServerCreateServerRequest extends $pb.GeneratedMessage {
  factory ServerCreateServerRequest({
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    ServerType? serverType,
    $fixnum.Int64? sshKeyId,
    $core.String? buildMemoryLimit,
    $core.String? buildCpuLimit,
  }) {
    final result = ServerCreateServerRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (ipAddress != null) result.ipAddress = ipAddress;
    if (port != null) result.port = port;
    if (username != null) result.username = username;
    if (serverType != null) result.serverType = serverType;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    if (buildMemoryLimit != null) result.buildMemoryLimit = buildMemoryLimit;
    if (buildCpuLimit != null) result.buildCpuLimit = buildCpuLimit;
    return result;
  }

  ServerCreateServerRequest._();

  factory ServerCreateServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreateServerRequest()..mergeFromBuffer(data, registry);
  factory ServerCreateServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreateServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCreateServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCreateServerRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aOS(3, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(4, _omitFieldNames ? '' : 'port')
    ..aOS(5, _omitFieldNames ? '' : 'username')
    ..aE<ServerType>(6, _omitFieldNames ? '' : 'serverType',
        enumValues: ServerType.values)
    ..aInt64(7, _omitFieldNames ? '' : 'sshKeyId')
    ..aOS(8, _omitFieldNames ? '' : 'buildMemoryLimit')
    ..aOS(9, _omitFieldNames ? '' : 'buildCpuLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreateServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreateServerRequest copyWith(
          void Function(ServerCreateServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerCreateServerRequest))
          as ServerCreateServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerCreateServerRequest() / ServerCreateServerRequest.new instead')
  static ServerCreateServerRequest create() => ServerCreateServerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerCreateServerRequest._();
  @$core.override
  ServerCreateServerRequest createEmptyInstance() =>
      ServerCreateServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerCreateServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCreateServerRequest>(
          ServerCreateServerRequest.$_createMessage);
  static ServerCreateServerRequest? _defaultInstance;

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
  $core.String get ipAddress => $_getSZ(2);
  @$pb.TagNumber(3)
  set ipAddress($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasIpAddress() => $_has(2);
  @$pb.TagNumber(3)
  void clearIpAddress() => $_clearField(3);

  /// SSH port; 0 means the default 22.
  @$pb.TagNumber(4)
  $fixnum.Int64 get port => $_getI64(3);
  @$pb.TagNumber(4)
  set port($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPort() => $_has(3);
  @$pb.TagNumber(4)
  void clearPort() => $_clearField(4);

  /// SSH user; empty means the default "root".
  @$pb.TagNumber(5)
  $core.String get username => $_getSZ(4);
  @$pb.TagNumber(5)
  set username($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUsername() => $_has(4);
  @$pb.TagNumber(5)
  void clearUsername() => $_clearField(5);

  @$pb.TagNumber(6)
  ServerType get serverType => $_getN(5);
  @$pb.TagNumber(6)
  set serverType(ServerType value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasServerType() => $_has(5);
  @$pb.TagNumber(6)
  void clearServerType() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get sshKeyId => $_getI64(6);
  @$pb.TagNumber(7)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSshKeyId() => $_has(6);
  @$pb.TagNumber(7)
  void clearSshKeyId() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get buildMemoryLimit => $_getSZ(7);
  @$pb.TagNumber(8)
  set buildMemoryLimit($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBuildMemoryLimit() => $_has(7);
  @$pb.TagNumber(8)
  void clearBuildMemoryLimit() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get buildCpuLimit => $_getSZ(8);
  @$pb.TagNumber(9)
  set buildCpuLimit($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasBuildCpuLimit() => $_has(8);
  @$pb.TagNumber(9)
  void clearBuildCpuLimit() => $_clearField(9);
}

class ServerCreateServerResponse extends $pb.GeneratedMessage {
  factory ServerCreateServerResponse({
    Server? server,
  }) {
    final result = ServerCreateServerResponse._();
    if (server != null) result.server = server;
    return result;
  }

  ServerCreateServerResponse._();

  factory ServerCreateServerResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreateServerResponse()..mergeFromBuffer(data, registry);
  factory ServerCreateServerResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreateServerResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCreateServerResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCreateServerResponse.$_createMessage)
    ..aOM<Server>(1, _omitFieldNames ? '' : 'server',
        subBuilder: Server.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreateServerResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreateServerResponse copyWith(
          void Function(ServerCreateServerResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerCreateServerResponse))
          as ServerCreateServerResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerCreateServerResponse() / ServerCreateServerResponse.new instead')
  static ServerCreateServerResponse create() => ServerCreateServerResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerCreateServerResponse._();
  @$core.override
  ServerCreateServerResponse createEmptyInstance() =>
      ServerCreateServerResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerCreateServerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCreateServerResponse>(
          ServerCreateServerResponse.$_createMessage);
  static ServerCreateServerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Server get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(Server value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  Server ensureServer() => $_ensure(0);
}

/// PATCH semantics: absent fields keep their stored values.
class ServerUpdateServerRequest extends $pb.GeneratedMessage {
  factory ServerUpdateServerRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    ServerStatus? serverStatus,
    ServerType? serverType,
    $core.bool? enableDockerCleanup,
    $core.String? logCleanupCron,
    $core.String? command,
    $core.String? metricsConfig,
    $fixnum.Int64? sshKeyId,
    $core.String? buildMemoryLimit,
    $core.String? buildCpuLimit,
  }) {
    final result = ServerUpdateServerRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (ipAddress != null) result.ipAddress = ipAddress;
    if (port != null) result.port = port;
    if (username != null) result.username = username;
    if (serverStatus != null) result.serverStatus = serverStatus;
    if (serverType != null) result.serverType = serverType;
    if (enableDockerCleanup != null)
      result.enableDockerCleanup = enableDockerCleanup;
    if (logCleanupCron != null) result.logCleanupCron = logCleanupCron;
    if (command != null) result.command = command;
    if (metricsConfig != null) result.metricsConfig = metricsConfig;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    if (buildMemoryLimit != null) result.buildMemoryLimit = buildMemoryLimit;
    if (buildCpuLimit != null) result.buildCpuLimit = buildCpuLimit;
    return result;
  }

  ServerUpdateServerRequest._();

  factory ServerUpdateServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdateServerRequest()..mergeFromBuffer(data, registry);
  factory ServerUpdateServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdateServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerUpdateServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerUpdateServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(5, _omitFieldNames ? '' : 'port')
    ..aOS(6, _omitFieldNames ? '' : 'username')
    ..aE<ServerStatus>(7, _omitFieldNames ? '' : 'serverStatus',
        enumValues: ServerStatus.values)
    ..aE<ServerType>(8, _omitFieldNames ? '' : 'serverType',
        enumValues: ServerType.values)
    ..aOB(9, _omitFieldNames ? '' : 'enableDockerCleanup')
    ..aOS(10, _omitFieldNames ? '' : 'logCleanupCron')
    ..aOS(11, _omitFieldNames ? '' : 'command')
    ..aOS(12, _omitFieldNames ? '' : 'metricsConfig')
    ..aInt64(13, _omitFieldNames ? '' : 'sshKeyId')
    ..aOS(14, _omitFieldNames ? '' : 'buildMemoryLimit')
    ..aOS(15, _omitFieldNames ? '' : 'buildCpuLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdateServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdateServerRequest copyWith(
          void Function(ServerUpdateServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerUpdateServerRequest))
          as ServerUpdateServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerUpdateServerRequest() / ServerUpdateServerRequest.new instead')
  static ServerUpdateServerRequest create() => ServerUpdateServerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerUpdateServerRequest._();
  @$core.override
  ServerUpdateServerRequest createEmptyInstance() =>
      ServerUpdateServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerUpdateServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerUpdateServerRequest>(
          ServerUpdateServerRequest.$_createMessage);
  static ServerUpdateServerRequest? _defaultInstance;

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
  $core.String get ipAddress => $_getSZ(3);
  @$pb.TagNumber(4)
  set ipAddress($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasIpAddress() => $_has(3);
  @$pb.TagNumber(4)
  void clearIpAddress() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get port => $_getI64(4);
  @$pb.TagNumber(5)
  set port($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPort() => $_has(4);
  @$pb.TagNumber(5)
  void clearPort() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get username => $_getSZ(5);
  @$pb.TagNumber(6)
  set username($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasUsername() => $_has(5);
  @$pb.TagNumber(6)
  void clearUsername() => $_clearField(6);

  @$pb.TagNumber(7)
  ServerStatus get serverStatus => $_getN(6);
  @$pb.TagNumber(7)
  set serverStatus(ServerStatus value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasServerStatus() => $_has(6);
  @$pb.TagNumber(7)
  void clearServerStatus() => $_clearField(7);

  @$pb.TagNumber(8)
  ServerType get serverType => $_getN(7);
  @$pb.TagNumber(8)
  set serverType(ServerType value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasServerType() => $_has(7);
  @$pb.TagNumber(8)
  void clearServerType() => $_clearField(8);

  /// Integer-as-boolean in storage.
  @$pb.TagNumber(9)
  $core.bool get enableDockerCleanup => $_getBF(8);
  @$pb.TagNumber(9)
  set enableDockerCleanup($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEnableDockerCleanup() => $_has(8);
  @$pb.TagNumber(9)
  void clearEnableDockerCleanup() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get logCleanupCron => $_getSZ(9);
  @$pb.TagNumber(10)
  set logCleanupCron($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasLogCleanupCron() => $_has(9);
  @$pb.TagNumber(10)
  void clearLogCleanupCron() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get command => $_getSZ(10);
  @$pb.TagNumber(11)
  set command($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCommand() => $_has(10);
  @$pb.TagNumber(11)
  void clearCommand() => $_clearField(11);

  /// Metrics collection configuration as a JSON document.
  @$pb.TagNumber(12)
  $core.String get metricsConfig => $_getSZ(11);
  @$pb.TagNumber(12)
  set metricsConfig($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasMetricsConfig() => $_has(11);
  @$pb.TagNumber(12)
  void clearMetricsConfig() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get sshKeyId => $_getI64(12);
  @$pb.TagNumber(13)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasSshKeyId() => $_has(12);
  @$pb.TagNumber(13)
  void clearSshKeyId() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get buildMemoryLimit => $_getSZ(13);
  @$pb.TagNumber(14)
  set buildMemoryLimit($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasBuildMemoryLimit() => $_has(13);
  @$pb.TagNumber(14)
  void clearBuildMemoryLimit() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get buildCpuLimit => $_getSZ(14);
  @$pb.TagNumber(15)
  set buildCpuLimit($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasBuildCpuLimit() => $_has(14);
  @$pb.TagNumber(15)
  void clearBuildCpuLimit() => $_clearField(15);
}

class ServerUpdateServerResponse extends $pb.GeneratedMessage {
  factory ServerUpdateServerResponse({
    Server? server,
  }) {
    final result = ServerUpdateServerResponse._();
    if (server != null) result.server = server;
    return result;
  }

  ServerUpdateServerResponse._();

  factory ServerUpdateServerResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdateServerResponse()..mergeFromBuffer(data, registry);
  factory ServerUpdateServerResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdateServerResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerUpdateServerResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerUpdateServerResponse.$_createMessage)
    ..aOM<Server>(1, _omitFieldNames ? '' : 'server',
        subBuilder: Server.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdateServerResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdateServerResponse copyWith(
          void Function(ServerUpdateServerResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerUpdateServerResponse))
          as ServerUpdateServerResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerUpdateServerResponse() / ServerUpdateServerResponse.new instead')
  static ServerUpdateServerResponse create() => ServerUpdateServerResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerUpdateServerResponse._();
  @$core.override
  ServerUpdateServerResponse createEmptyInstance() =>
      ServerUpdateServerResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerUpdateServerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerUpdateServerResponse>(
          ServerUpdateServerResponse.$_createMessage);
  static ServerUpdateServerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Server get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(Server value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  Server ensureServer() => $_ensure(0);
}

class ServerDeleteServerRequest extends $pb.GeneratedMessage {
  factory ServerDeleteServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerDeleteServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerDeleteServerRequest._();

  factory ServerDeleteServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerDeleteServerRequest()..mergeFromBuffer(data, registry);
  factory ServerDeleteServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerDeleteServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerDeleteServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerDeleteServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerDeleteServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerDeleteServerRequest copyWith(
          void Function(ServerDeleteServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerDeleteServerRequest))
          as ServerDeleteServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerDeleteServerRequest() / ServerDeleteServerRequest.new instead')
  static ServerDeleteServerRequest create() => ServerDeleteServerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerDeleteServerRequest._();
  @$core.override
  ServerDeleteServerRequest createEmptyInstance() =>
      ServerDeleteServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerDeleteServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerDeleteServerRequest>(
          ServerDeleteServerRequest.$_createMessage);
  static ServerDeleteServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerTestConnectionRequest extends $pb.GeneratedMessage {
  factory ServerTestConnectionRequest({
    $fixnum.Int64? id,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerTestConnectionRequest._();
    if (id != null) result.id = id;
    if (hostKeyFingerprint != null)
      result.hostKeyFingerprint = hostKeyFingerprint;
    if (sudoPassword != null) result.sudoPassword = sudoPassword;
    if (poolSize != null) result.poolSize = poolSize;
    return result;
  }

  ServerTestConnectionRequest._();

  factory ServerTestConnectionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerTestConnectionRequest()..mergeFromBuffer(data, registry);
  factory ServerTestConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerTestConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerTestConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerTestConnectionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(3, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(4, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerTestConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerTestConnectionRequest copyWith(
          void Function(ServerTestConnectionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerTestConnectionRequest))
          as ServerTestConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerTestConnectionRequest() / ServerTestConnectionRequest.new instead')
  static ServerTestConnectionRequest create() =>
      ServerTestConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerTestConnectionRequest._();
  @$core.override
  ServerTestConnectionRequest createEmptyInstance() =>
      ServerTestConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerTestConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerTestConnectionRequest>(
          ServerTestConnectionRequest.$_createMessage);
  static ServerTestConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// SSH host-key SHA-256 fingerprint pin; empty disables pinning.
  @$pb.TagNumber(2)
  $core.String get hostKeyFingerprint => $_getSZ(1);
  @$pb.TagNumber(2)
  set hostKeyFingerprint($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHostKeyFingerprint() => $_has(1);
  @$pb.TagNumber(2)
  void clearHostKeyFingerprint() => $_clearField(2);

  /// Sudo credential document; only sent for privileged steps.
  @$pb.TagNumber(3)
  $core.String get sudoPassword => $_getSZ(2);
  @$pb.TagNumber(3)
  set sudoPassword($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSudoPassword() => $_has(2);
  @$pb.TagNumber(3)
  void clearSudoPassword() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get poolSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set poolSize($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPoolSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearPoolSize() => $_clearField(4);
}

class ServerTestConnectionResponse extends $pb.GeneratedMessage {
  factory ServerTestConnectionResponse({
    $2.ServerConnectionInfo? connection,
  }) {
    final result = ServerTestConnectionResponse._();
    if (connection != null) result.connection = connection;
    return result;
  }

  ServerTestConnectionResponse._();

  factory ServerTestConnectionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerTestConnectionResponse()..mergeFromBuffer(data, registry);
  factory ServerTestConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerTestConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerTestConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerTestConnectionResponse.$_createMessage)
    ..aOM<$2.ServerConnectionInfo>(1, _omitFieldNames ? '' : 'connection',
        subBuilder: $2.ServerConnectionInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerTestConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerTestConnectionResponse copyWith(
          void Function(ServerTestConnectionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerTestConnectionResponse))
          as ServerTestConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerTestConnectionResponse() / ServerTestConnectionResponse.new instead')
  static ServerTestConnectionResponse create() =>
      ServerTestConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerTestConnectionResponse._();
  @$core.override
  ServerTestConnectionResponse createEmptyInstance() =>
      ServerTestConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerTestConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerTestConnectionResponse>(
          ServerTestConnectionResponse.$_createMessage);
  static ServerTestConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.ServerConnectionInfo get connection => $_getN(0);
  @$pb.TagNumber(1)
  set connection($2.ServerConnectionInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasConnection() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnection() => $_clearField(1);
  @$pb.TagNumber(1)
  $2.ServerConnectionInfo ensureConnection() => $_ensure(0);
}

class ServerSetupServerRequest extends $pb.GeneratedMessage {
  factory ServerSetupServerRequest({
    $fixnum.Int64? id,
    $core.bool? installDependencies,
    $core.String? advertiseAddr,
    $core.String? acmeEmail,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerSetupServerRequest._();
    if (id != null) result.id = id;
    if (installDependencies != null)
      result.installDependencies = installDependencies;
    if (advertiseAddr != null) result.advertiseAddr = advertiseAddr;
    if (acmeEmail != null) result.acmeEmail = acmeEmail;
    if (hostKeyFingerprint != null)
      result.hostKeyFingerprint = hostKeyFingerprint;
    if (sudoPassword != null) result.sudoPassword = sudoPassword;
    if (poolSize != null) result.poolSize = poolSize;
    return result;
  }

  ServerSetupServerRequest._();

  factory ServerSetupServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupServerRequest()..mergeFromBuffer(data, registry);
  factory ServerSetupServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerSetupServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerSetupServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'installDependencies')
    ..aOS(3, _omitFieldNames ? '' : 'advertiseAddr')
    ..aOS(4, _omitFieldNames ? '' : 'acmeEmail')
    ..aOS(5, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(6, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(7, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupServerRequest copyWith(
          void Function(ServerSetupServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerSetupServerRequest))
          as ServerSetupServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerSetupServerRequest() / ServerSetupServerRequest.new instead')
  static ServerSetupServerRequest create() => ServerSetupServerRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServerSetupServerRequest._();
  @$core.override
  ServerSetupServerRequest createEmptyInstance() =>
      ServerSetupServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerSetupServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerSetupServerRequest>(
          ServerSetupServerRequest.$_createMessage);
  static ServerSetupServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get installDependencies => $_getBF(1);
  @$pb.TagNumber(2)
  set installDependencies($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasInstallDependencies() => $_has(1);
  @$pb.TagNumber(2)
  void clearInstallDependencies() => $_clearField(2);

  /// Swarm advertise address override; server's private/public IP when unset.
  @$pb.TagNumber(3)
  $core.String get advertiseAddr => $_getSZ(2);
  @$pb.TagNumber(3)
  set advertiseAddr($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAdvertiseAddr() => $_has(2);
  @$pb.TagNumber(3)
  void clearAdvertiseAddr() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get acmeEmail => $_getSZ(3);
  @$pb.TagNumber(4)
  set acmeEmail($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAcmeEmail() => $_has(3);
  @$pb.TagNumber(4)
  void clearAcmeEmail() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get hostKeyFingerprint => $_getSZ(4);
  @$pb.TagNumber(5)
  set hostKeyFingerprint($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHostKeyFingerprint() => $_has(4);
  @$pb.TagNumber(5)
  void clearHostKeyFingerprint() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get sudoPassword => $_getSZ(5);
  @$pb.TagNumber(6)
  set sudoPassword($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasSudoPassword() => $_has(5);
  @$pb.TagNumber(6)
  void clearSudoPassword() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.int get poolSize => $_getIZ(6);
  @$pb.TagNumber(7)
  set poolSize($core.int value) => $_setUnsignedInt32(6, value);
  @$pb.TagNumber(7)
  $core.bool hasPoolSize() => $_has(6);
  @$pb.TagNumber(7)
  void clearPoolSize() => $_clearField(7);
}

class ServerSetupServerResponse extends $pb.GeneratedMessage {
  factory ServerSetupServerResponse({
    $2.ServerSetupOutcome? outcome,
  }) {
    final result = ServerSetupServerResponse._();
    if (outcome != null) result.outcome = outcome;
    return result;
  }

  ServerSetupServerResponse._();

  factory ServerSetupServerResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupServerResponse()..mergeFromBuffer(data, registry);
  factory ServerSetupServerResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupServerResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerSetupServerResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerSetupServerResponse.$_createMessage)
    ..aOM<$2.ServerSetupOutcome>(1, _omitFieldNames ? '' : 'outcome',
        subBuilder: $2.ServerSetupOutcome.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupServerResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupServerResponse copyWith(
          void Function(ServerSetupServerResponse) updates) =>
      super.copyWith((message) => updates(message as ServerSetupServerResponse))
          as ServerSetupServerResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerSetupServerResponse() / ServerSetupServerResponse.new instead')
  static ServerSetupServerResponse create() => ServerSetupServerResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerSetupServerResponse._();
  @$core.override
  ServerSetupServerResponse createEmptyInstance() =>
      ServerSetupServerResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerSetupServerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerSetupServerResponse>(
          ServerSetupServerResponse.$_createMessage);
  static ServerSetupServerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.ServerSetupOutcome get outcome => $_getN(0);
  @$pb.TagNumber(1)
  set outcome($2.ServerSetupOutcome value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasOutcome() => $_has(0);
  @$pb.TagNumber(1)
  void clearOutcome() => $_clearField(1);
  @$pb.TagNumber(1)
  $2.ServerSetupOutcome ensureOutcome() => $_ensure(0);
}

class ServerStartServerRequest extends $pb.GeneratedMessage {
  factory ServerStartServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerStartServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerStartServerRequest._();

  factory ServerStartServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerStartServerRequest()..mergeFromBuffer(data, registry);
  factory ServerStartServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerStartServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerStartServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerStartServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerStartServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerStartServerRequest copyWith(
          void Function(ServerStartServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerStartServerRequest))
          as ServerStartServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerStartServerRequest() / ServerStartServerRequest.new instead')
  static ServerStartServerRequest create() => ServerStartServerRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServerStartServerRequest._();
  @$core.override
  ServerStartServerRequest createEmptyInstance() =>
      ServerStartServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerStartServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerStartServerRequest>(
          ServerStartServerRequest.$_createMessage);
  static ServerStartServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerStopServerRequest extends $pb.GeneratedMessage {
  factory ServerStopServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerStopServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerStopServerRequest._();

  factory ServerStopServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerStopServerRequest()..mergeFromBuffer(data, registry);
  factory ServerStopServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerStopServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerStopServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerStopServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerStopServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerStopServerRequest copyWith(
          void Function(ServerStopServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerStopServerRequest))
          as ServerStopServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerStopServerRequest() / ServerStopServerRequest.new instead')
  static ServerStopServerRequest create() => ServerStopServerRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServerStopServerRequest._();
  @$core.override
  ServerStopServerRequest createEmptyInstance() => ServerStopServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerStopServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerStopServerRequest>(
          ServerStopServerRequest.$_createMessage);
  static ServerStopServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerRestartServerRequest extends $pb.GeneratedMessage {
  factory ServerRestartServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerRestartServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerRestartServerRequest._();

  factory ServerRestartServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerRestartServerRequest()..mergeFromBuffer(data, registry);
  factory ServerRestartServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerRestartServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerRestartServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerRestartServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerRestartServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerRestartServerRequest copyWith(
          void Function(ServerRestartServerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerRestartServerRequest))
          as ServerRestartServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerRestartServerRequest() / ServerRestartServerRequest.new instead')
  static ServerRestartServerRequest create() => ServerRestartServerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerRestartServerRequest._();
  @$core.override
  ServerRestartServerRequest createEmptyInstance() =>
      ServerRestartServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerRestartServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerRestartServerRequest>(
          ServerRestartServerRequest.$_createMessage);
  static ServerRestartServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Detaches the panel from a server without deleting the row.
class ServerRemoveServerRequest extends $pb.GeneratedMessage {
  factory ServerRemoveServerRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerRemoveServerRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerRemoveServerRequest._();

  factory ServerRemoveServerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerRemoveServerRequest()..mergeFromBuffer(data, registry);
  factory ServerRemoveServerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerRemoveServerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerRemoveServerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerRemoveServerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerRemoveServerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerRemoveServerRequest copyWith(
          void Function(ServerRemoveServerRequest) updates) =>
      super.copyWith((message) => updates(message as ServerRemoveServerRequest))
          as ServerRemoveServerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerRemoveServerRequest() / ServerRemoveServerRequest.new instead')
  static ServerRemoveServerRequest create() => ServerRemoveServerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerRemoveServerRequest._();
  @$core.override
  ServerRemoveServerRequest createEmptyInstance() =>
      ServerRemoveServerRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerRemoveServerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerRemoveServerRequest>(
          ServerRemoveServerRequest.$_createMessage);
  static ServerRemoveServerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerGetServerErrorsRequest extends $pb.GeneratedMessage {
  factory ServerGetServerErrorsRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerGetServerErrorsRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerGetServerErrorsRequest._();

  factory ServerGetServerErrorsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerErrorsRequest()..mergeFromBuffer(data, registry);
  factory ServerGetServerErrorsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerErrorsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerGetServerErrorsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerGetServerErrorsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerErrorsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerErrorsRequest copyWith(
          void Function(ServerGetServerErrorsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerGetServerErrorsRequest))
          as ServerGetServerErrorsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerGetServerErrorsRequest() / ServerGetServerErrorsRequest.new instead')
  static ServerGetServerErrorsRequest create() =>
      ServerGetServerErrorsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerGetServerErrorsRequest._();
  @$core.override
  ServerGetServerErrorsRequest createEmptyInstance() =>
      ServerGetServerErrorsRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerGetServerErrorsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerGetServerErrorsRequest>(
          ServerGetServerErrorsRequest.$_createMessage);
  static ServerGetServerErrorsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerGetServerErrorsResponse extends $pb.GeneratedMessage {
  factory ServerGetServerErrorsResponse({
    $core.Iterable<$core.String>? errors,
  }) {
    final result = ServerGetServerErrorsResponse._();
    if (errors != null) result.errors.addAll(errors);
    return result;
  }

  ServerGetServerErrorsResponse._();

  factory ServerGetServerErrorsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerErrorsResponse()..mergeFromBuffer(data, registry);
  factory ServerGetServerErrorsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerGetServerErrorsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerGetServerErrorsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerGetServerErrorsResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'errors')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerErrorsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerGetServerErrorsResponse copyWith(
          void Function(ServerGetServerErrorsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerGetServerErrorsResponse))
          as ServerGetServerErrorsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerGetServerErrorsResponse() / ServerGetServerErrorsResponse.new instead')
  static ServerGetServerErrorsResponse create() =>
      ServerGetServerErrorsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerGetServerErrorsResponse._();
  @$core.override
  ServerGetServerErrorsResponse createEmptyInstance() =>
      ServerGetServerErrorsResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerGetServerErrorsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerGetServerErrorsResponse>(
          ServerGetServerErrorsResponse.$_createMessage);
  static ServerGetServerErrorsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get errors => $_getList(0);
}

class ServerListRemoteServersRequest extends $pb.GeneratedMessage {
  factory ServerListRemoteServersRequest() =>
      ServerListRemoteServersRequest._();

  ServerListRemoteServersRequest._();

  factory ServerListRemoteServersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListRemoteServersRequest()..mergeFromBuffer(data, registry);
  factory ServerListRemoteServersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListRemoteServersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListRemoteServersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListRemoteServersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListRemoteServersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListRemoteServersRequest copyWith(
          void Function(ServerListRemoteServersRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerListRemoteServersRequest))
          as ServerListRemoteServersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListRemoteServersRequest() / ServerListRemoteServersRequest.new instead')
  static ServerListRemoteServersRequest create() =>
      ServerListRemoteServersRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerListRemoteServersRequest._();
  @$core.override
  ServerListRemoteServersRequest createEmptyInstance() =>
      ServerListRemoteServersRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerListRemoteServersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListRemoteServersRequest>(
          ServerListRemoteServersRequest.$_createMessage);
  static ServerListRemoteServersRequest? _defaultInstance;
}

class ServerListRemoteServersResponse extends $pb.GeneratedMessage {
  factory ServerListRemoteServersResponse({
    $core.Iterable<Server>? servers,
  }) {
    final result = ServerListRemoteServersResponse._();
    if (servers != null) result.servers.addAll(servers);
    return result;
  }

  ServerListRemoteServersResponse._();

  factory ServerListRemoteServersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListRemoteServersResponse()..mergeFromBuffer(data, registry);
  factory ServerListRemoteServersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListRemoteServersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListRemoteServersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListRemoteServersResponse.$_createMessage)
    ..pPM<Server>(1, _omitFieldNames ? '' : 'servers',
        subBuilder: Server.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListRemoteServersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListRemoteServersResponse copyWith(
          void Function(ServerListRemoteServersResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerListRemoteServersResponse))
          as ServerListRemoteServersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListRemoteServersResponse() / ServerListRemoteServersResponse.new instead')
  static ServerListRemoteServersResponse create() =>
      ServerListRemoteServersResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerListRemoteServersResponse._();
  @$core.override
  ServerListRemoteServersResponse createEmptyInstance() =>
      ServerListRemoteServersResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerListRemoteServersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListRemoteServersResponse>(
          ServerListRemoteServersResponse.$_createMessage);
  static ServerListRemoteServersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Server> get servers => $_getList(0);
}

class ServerListPrivateNetworksRequest extends $pb.GeneratedMessage {
  factory ServerListPrivateNetworksRequest() =>
      ServerListPrivateNetworksRequest._();

  ServerListPrivateNetworksRequest._();

  factory ServerListPrivateNetworksRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListPrivateNetworksRequest()..mergeFromBuffer(data, registry);
  factory ServerListPrivateNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListPrivateNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListPrivateNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListPrivateNetworksRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListPrivateNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListPrivateNetworksRequest copyWith(
          void Function(ServerListPrivateNetworksRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerListPrivateNetworksRequest))
          as ServerListPrivateNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListPrivateNetworksRequest() / ServerListPrivateNetworksRequest.new instead')
  static ServerListPrivateNetworksRequest create() =>
      ServerListPrivateNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerListPrivateNetworksRequest._();
  @$core.override
  ServerListPrivateNetworksRequest createEmptyInstance() =>
      ServerListPrivateNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerListPrivateNetworksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListPrivateNetworksRequest>(
          ServerListPrivateNetworksRequest.$_createMessage);
  static ServerListPrivateNetworksRequest? _defaultInstance;
}

class ServerListPrivateNetworksResponse extends $pb.GeneratedMessage {
  factory ServerListPrivateNetworksResponse({
    $core.Iterable<$3.ServerPrivateNetwork>? networks,
  }) {
    final result = ServerListPrivateNetworksResponse._();
    if (networks != null) result.networks.addAll(networks);
    return result;
  }

  ServerListPrivateNetworksResponse._();

  factory ServerListPrivateNetworksResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListPrivateNetworksResponse()..mergeFromBuffer(data, registry);
  factory ServerListPrivateNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerListPrivateNetworksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerListPrivateNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerListPrivateNetworksResponse.$_createMessage)
    ..pPM<$3.ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'networks',
        subBuilder: $3.ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListPrivateNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerListPrivateNetworksResponse copyWith(
          void Function(ServerListPrivateNetworksResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerListPrivateNetworksResponse))
          as ServerListPrivateNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerListPrivateNetworksResponse() / ServerListPrivateNetworksResponse.new instead')
  static ServerListPrivateNetworksResponse create() =>
      ServerListPrivateNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerListPrivateNetworksResponse._();
  @$core.override
  ServerListPrivateNetworksResponse createEmptyInstance() =>
      ServerListPrivateNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerListPrivateNetworksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerListPrivateNetworksResponse>(
          ServerListPrivateNetworksResponse.$_createMessage);
  static ServerListPrivateNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$3.ServerPrivateNetwork> get networks => $_getList(0);
}

class ServerCreatePrivateNetworkRequest extends $pb.GeneratedMessage {
  factory ServerCreatePrivateNetworkRequest({
    $fixnum.Int64? serverId,
    $3.ServerConnectionMode? connectionMode,
    $3.ServerPrivateNetworkProvider? provider,
    $core.String? privateHost,
    $core.String? tunnelAddress,
    $core.String? publicKey,
    $core.String? endpoint,
    $core.int? listenPort,
    $core.int? persistentKeepalive,
    $core.String? dnsName,
    $core.Iterable<$core.String>? routes,
  }) {
    final result = ServerCreatePrivateNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    if (connectionMode != null) result.connectionMode = connectionMode;
    if (provider != null) result.provider = provider;
    if (privateHost != null) result.privateHost = privateHost;
    if (tunnelAddress != null) result.tunnelAddress = tunnelAddress;
    if (publicKey != null) result.publicKey = publicKey;
    if (endpoint != null) result.endpoint = endpoint;
    if (listenPort != null) result.listenPort = listenPort;
    if (persistentKeepalive != null)
      result.persistentKeepalive = persistentKeepalive;
    if (dnsName != null) result.dnsName = dnsName;
    if (routes != null) result.routes.addAll(routes);
    return result;
  }

  ServerCreatePrivateNetworkRequest._();

  factory ServerCreatePrivateNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreatePrivateNetworkRequest()..mergeFromBuffer(data, registry);
  factory ServerCreatePrivateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreatePrivateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCreatePrivateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCreatePrivateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aE<$3.ServerConnectionMode>(2, _omitFieldNames ? '' : 'connectionMode',
        enumValues: $3.ServerConnectionMode.values)
    ..aE<$3.ServerPrivateNetworkProvider>(3, _omitFieldNames ? '' : 'provider',
        enumValues: $3.ServerPrivateNetworkProvider.values)
    ..aOS(4, _omitFieldNames ? '' : 'privateHost')
    ..aOS(5, _omitFieldNames ? '' : 'tunnelAddress')
    ..aOS(6, _omitFieldNames ? '' : 'publicKey')
    ..aOS(7, _omitFieldNames ? '' : 'endpoint')
    ..aI(8, _omitFieldNames ? '' : 'listenPort', fieldType: $pb.PbFieldType.OU3)
    ..aI(9, _omitFieldNames ? '' : 'persistentKeepalive',
        fieldType: $pb.PbFieldType.OU3)
    ..aOS(10, _omitFieldNames ? '' : 'dnsName')
    ..pPS(11, _omitFieldNames ? '' : 'routes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreatePrivateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreatePrivateNetworkRequest copyWith(
          void Function(ServerCreatePrivateNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerCreatePrivateNetworkRequest))
          as ServerCreatePrivateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerCreatePrivateNetworkRequest() / ServerCreatePrivateNetworkRequest.new instead')
  static ServerCreatePrivateNetworkRequest create() =>
      ServerCreatePrivateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerCreatePrivateNetworkRequest._();
  @$core.override
  ServerCreatePrivateNetworkRequest createEmptyInstance() =>
      ServerCreatePrivateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerCreatePrivateNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCreatePrivateNetworkRequest>(
          ServerCreatePrivateNetworkRequest.$_createMessage);
  static ServerCreatePrivateNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $3.ServerConnectionMode get connectionMode => $_getN(1);
  @$pb.TagNumber(2)
  set connectionMode($3.ServerConnectionMode value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasConnectionMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearConnectionMode() => $_clearField(2);

  @$pb.TagNumber(3)
  $3.ServerPrivateNetworkProvider get provider => $_getN(2);
  @$pb.TagNumber(3)
  set provider($3.ServerPrivateNetworkProvider value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get privateHost => $_getSZ(3);
  @$pb.TagNumber(4)
  set privateHost($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivateHost() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivateHost() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get tunnelAddress => $_getSZ(4);
  @$pb.TagNumber(5)
  set tunnelAddress($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTunnelAddress() => $_has(4);
  @$pb.TagNumber(5)
  void clearTunnelAddress() => $_clearField(5);

  /// WireGuard public key (base64 key material).
  @$pb.TagNumber(6)
  $core.String get publicKey => $_getSZ(5);
  @$pb.TagNumber(6)
  set publicKey($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPublicKey() => $_has(5);
  @$pb.TagNumber(6)
  void clearPublicKey() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get endpoint => $_getSZ(6);
  @$pb.TagNumber(7)
  set endpoint($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasEndpoint() => $_has(6);
  @$pb.TagNumber(7)
  void clearEndpoint() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.int get listenPort => $_getIZ(7);
  @$pb.TagNumber(8)
  set listenPort($core.int value) => $_setUnsignedInt32(7, value);
  @$pb.TagNumber(8)
  $core.bool hasListenPort() => $_has(7);
  @$pb.TagNumber(8)
  void clearListenPort() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.int get persistentKeepalive => $_getIZ(8);
  @$pb.TagNumber(9)
  set persistentKeepalive($core.int value) => $_setUnsignedInt32(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPersistentKeepalive() => $_has(8);
  @$pb.TagNumber(9)
  void clearPersistentKeepalive() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get dnsName => $_getSZ(9);
  @$pb.TagNumber(10)
  set dnsName($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasDnsName() => $_has(9);
  @$pb.TagNumber(10)
  void clearDnsName() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<$core.String> get routes => $_getList(10);
}

class ServerCreatePrivateNetworkResponse extends $pb.GeneratedMessage {
  factory ServerCreatePrivateNetworkResponse({
    $3.ServerPrivateNetwork? network,
  }) {
    final result = ServerCreatePrivateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerCreatePrivateNetworkResponse._();

  factory ServerCreatePrivateNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreatePrivateNetworkResponse()..mergeFromBuffer(data, registry);
  factory ServerCreatePrivateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCreatePrivateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCreatePrivateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCreatePrivateNetworkResponse.$_createMessage)
    ..aOM<$3.ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: $3.ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreatePrivateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCreatePrivateNetworkResponse copyWith(
          void Function(ServerCreatePrivateNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerCreatePrivateNetworkResponse))
          as ServerCreatePrivateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerCreatePrivateNetworkResponse() / ServerCreatePrivateNetworkResponse.new instead')
  static ServerCreatePrivateNetworkResponse create() =>
      ServerCreatePrivateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerCreatePrivateNetworkResponse._();
  @$core.override
  ServerCreatePrivateNetworkResponse createEmptyInstance() =>
      ServerCreatePrivateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerCreatePrivateNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCreatePrivateNetworkResponse>(
          ServerCreatePrivateNetworkResponse.$_createMessage);
  static ServerCreatePrivateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $3.ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network($3.ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  $3.ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerUpdatePrivateNetworkRequest extends $pb.GeneratedMessage {
  factory ServerUpdatePrivateNetworkRequest({
    $fixnum.Int64? id,
    $3.ServerConnectionMode? connectionMode,
    $3.ServerPrivateNetworkProvider? provider,
    $core.String? privateHost,
    $core.String? tunnelAddress,
    $core.String? publicKey,
    $core.String? endpoint,
    $core.int? listenPort,
    $core.int? persistentKeepalive,
    $core.String? dnsName,
    $core.Iterable<$core.String>? routes,
  }) {
    final result = ServerUpdatePrivateNetworkRequest._();
    if (id != null) result.id = id;
    if (connectionMode != null) result.connectionMode = connectionMode;
    if (provider != null) result.provider = provider;
    if (privateHost != null) result.privateHost = privateHost;
    if (tunnelAddress != null) result.tunnelAddress = tunnelAddress;
    if (publicKey != null) result.publicKey = publicKey;
    if (endpoint != null) result.endpoint = endpoint;
    if (listenPort != null) result.listenPort = listenPort;
    if (persistentKeepalive != null)
      result.persistentKeepalive = persistentKeepalive;
    if (dnsName != null) result.dnsName = dnsName;
    if (routes != null) result.routes.addAll(routes);
    return result;
  }

  ServerUpdatePrivateNetworkRequest._();

  factory ServerUpdatePrivateNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdatePrivateNetworkRequest()..mergeFromBuffer(data, registry);
  factory ServerUpdatePrivateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdatePrivateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerUpdatePrivateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerUpdatePrivateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<$3.ServerConnectionMode>(2, _omitFieldNames ? '' : 'connectionMode',
        enumValues: $3.ServerConnectionMode.values)
    ..aE<$3.ServerPrivateNetworkProvider>(3, _omitFieldNames ? '' : 'provider',
        enumValues: $3.ServerPrivateNetworkProvider.values)
    ..aOS(4, _omitFieldNames ? '' : 'privateHost')
    ..aOS(5, _omitFieldNames ? '' : 'tunnelAddress')
    ..aOS(6, _omitFieldNames ? '' : 'publicKey')
    ..aOS(7, _omitFieldNames ? '' : 'endpoint')
    ..aI(8, _omitFieldNames ? '' : 'listenPort', fieldType: $pb.PbFieldType.OU3)
    ..aI(9, _omitFieldNames ? '' : 'persistentKeepalive',
        fieldType: $pb.PbFieldType.OU3)
    ..aOS(10, _omitFieldNames ? '' : 'dnsName')
    ..pPS(11, _omitFieldNames ? '' : 'routes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdatePrivateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdatePrivateNetworkRequest copyWith(
          void Function(ServerUpdatePrivateNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerUpdatePrivateNetworkRequest))
          as ServerUpdatePrivateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerUpdatePrivateNetworkRequest() / ServerUpdatePrivateNetworkRequest.new instead')
  static ServerUpdatePrivateNetworkRequest create() =>
      ServerUpdatePrivateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerUpdatePrivateNetworkRequest._();
  @$core.override
  ServerUpdatePrivateNetworkRequest createEmptyInstance() =>
      ServerUpdatePrivateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerUpdatePrivateNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerUpdatePrivateNetworkRequest>(
          ServerUpdatePrivateNetworkRequest.$_createMessage);
  static ServerUpdatePrivateNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $3.ServerConnectionMode get connectionMode => $_getN(1);
  @$pb.TagNumber(2)
  set connectionMode($3.ServerConnectionMode value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasConnectionMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearConnectionMode() => $_clearField(2);

  @$pb.TagNumber(3)
  $3.ServerPrivateNetworkProvider get provider => $_getN(2);
  @$pb.TagNumber(3)
  set provider($3.ServerPrivateNetworkProvider value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get privateHost => $_getSZ(3);
  @$pb.TagNumber(4)
  set privateHost($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivateHost() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivateHost() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get tunnelAddress => $_getSZ(4);
  @$pb.TagNumber(5)
  set tunnelAddress($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTunnelAddress() => $_has(4);
  @$pb.TagNumber(5)
  void clearTunnelAddress() => $_clearField(5);

  /// WireGuard public key (base64 key material).
  @$pb.TagNumber(6)
  $core.String get publicKey => $_getSZ(5);
  @$pb.TagNumber(6)
  set publicKey($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPublicKey() => $_has(5);
  @$pb.TagNumber(6)
  void clearPublicKey() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get endpoint => $_getSZ(6);
  @$pb.TagNumber(7)
  set endpoint($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasEndpoint() => $_has(6);
  @$pb.TagNumber(7)
  void clearEndpoint() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.int get listenPort => $_getIZ(7);
  @$pb.TagNumber(8)
  set listenPort($core.int value) => $_setUnsignedInt32(7, value);
  @$pb.TagNumber(8)
  $core.bool hasListenPort() => $_has(7);
  @$pb.TagNumber(8)
  void clearListenPort() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.int get persistentKeepalive => $_getIZ(8);
  @$pb.TagNumber(9)
  set persistentKeepalive($core.int value) => $_setUnsignedInt32(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPersistentKeepalive() => $_has(8);
  @$pb.TagNumber(9)
  void clearPersistentKeepalive() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get dnsName => $_getSZ(9);
  @$pb.TagNumber(10)
  set dnsName($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasDnsName() => $_has(9);
  @$pb.TagNumber(10)
  void clearDnsName() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<$core.String> get routes => $_getList(10);
}

class ServerUpdatePrivateNetworkResponse extends $pb.GeneratedMessage {
  factory ServerUpdatePrivateNetworkResponse({
    $3.ServerPrivateNetwork? network,
  }) {
    final result = ServerUpdatePrivateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerUpdatePrivateNetworkResponse._();

  factory ServerUpdatePrivateNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdatePrivateNetworkResponse()..mergeFromBuffer(data, registry);
  factory ServerUpdatePrivateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerUpdatePrivateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerUpdatePrivateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerUpdatePrivateNetworkResponse.$_createMessage)
    ..aOM<$3.ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: $3.ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdatePrivateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerUpdatePrivateNetworkResponse copyWith(
          void Function(ServerUpdatePrivateNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerUpdatePrivateNetworkResponse))
          as ServerUpdatePrivateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerUpdatePrivateNetworkResponse() / ServerUpdatePrivateNetworkResponse.new instead')
  static ServerUpdatePrivateNetworkResponse create() =>
      ServerUpdatePrivateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerUpdatePrivateNetworkResponse._();
  @$core.override
  ServerUpdatePrivateNetworkResponse createEmptyInstance() =>
      ServerUpdatePrivateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerUpdatePrivateNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerUpdatePrivateNetworkResponse>(
          ServerUpdatePrivateNetworkResponse.$_createMessage);
  static ServerUpdatePrivateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $3.ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network($3.ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  $3.ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerDeletePrivateNetworkRequest extends $pb.GeneratedMessage {
  factory ServerDeletePrivateNetworkRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerDeletePrivateNetworkRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerDeletePrivateNetworkRequest._();

  factory ServerDeletePrivateNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerDeletePrivateNetworkRequest()..mergeFromBuffer(data, registry);
  factory ServerDeletePrivateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerDeletePrivateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerDeletePrivateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerDeletePrivateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerDeletePrivateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerDeletePrivateNetworkRequest copyWith(
          void Function(ServerDeletePrivateNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerDeletePrivateNetworkRequest))
          as ServerDeletePrivateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerDeletePrivateNetworkRequest() / ServerDeletePrivateNetworkRequest.new instead')
  static ServerDeletePrivateNetworkRequest create() =>
      ServerDeletePrivateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerDeletePrivateNetworkRequest._();
  @$core.override
  ServerDeletePrivateNetworkRequest createEmptyInstance() =>
      ServerDeletePrivateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerDeletePrivateNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerDeletePrivateNetworkRequest>(
          ServerDeletePrivateNetworkRequest.$_createMessage);
  static ServerDeletePrivateNetworkRequest? _defaultInstance;

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

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_management.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'server_management.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'server_management.pbenum.dart';

class ServerCleanupPolicy extends $pb.GeneratedMessage {
  factory ServerCleanupPolicy({
    $core.bool? containers,
    $core.bool? images,
    $core.bool? networks,
    $core.bool? volumes,
    $core.bool? packages,
  }) {
    final result = ServerCleanupPolicy._();
    if (containers != null) result.containers = containers;
    if (images != null) result.images = images;
    if (networks != null) result.networks = networks;
    if (volumes != null) result.volumes = volumes;
    if (packages != null) result.packages = packages;
    return result;
  }

  ServerCleanupPolicy._();

  factory ServerCleanupPolicy.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCleanupPolicy()..mergeFromBuffer(data, registry);
  factory ServerCleanupPolicy.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCleanupPolicy()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCleanupPolicy',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCleanupPolicy.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'containers')
    ..aOB(2, _omitFieldNames ? '' : 'images')
    ..aOB(3, _omitFieldNames ? '' : 'networks')
    ..aOB(4, _omitFieldNames ? '' : 'volumes')
    ..aOB(5, _omitFieldNames ? '' : 'packages')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCleanupPolicy clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCleanupPolicy copyWith(void Function(ServerCleanupPolicy) updates) =>
      super.copyWith((message) => updates(message as ServerCleanupPolicy))
          as ServerCleanupPolicy;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ServerCleanupPolicy() / ServerCleanupPolicy.new instead')
  static ServerCleanupPolicy create() => ServerCleanupPolicy._();
  static $pb.GeneratedMessage $_createMessage() => ServerCleanupPolicy._();
  @$core.override
  ServerCleanupPolicy createEmptyInstance() => ServerCleanupPolicy._();
  @$core.pragma('dart2js:noInline')
  static ServerCleanupPolicy getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCleanupPolicy>(
          ServerCleanupPolicy.$_createMessage);
  static ServerCleanupPolicy? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get containers => $_getBF(0);
  @$pb.TagNumber(1)
  set containers($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainers() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainers() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get images => $_getBF(1);
  @$pb.TagNumber(2)
  set images($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasImages() => $_has(1);
  @$pb.TagNumber(2)
  void clearImages() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get networks => $_getBF(2);
  @$pb.TagNumber(3)
  set networks($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasNetworks() => $_has(2);
  @$pb.TagNumber(3)
  void clearNetworks() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get volumes => $_getBF(3);
  @$pb.TagNumber(4)
  set volumes($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasVolumes() => $_has(3);
  @$pb.TagNumber(4)
  void clearVolumes() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get packages => $_getBF(4);
  @$pb.TagNumber(5)
  set packages($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPackages() => $_has(4);
  @$pb.TagNumber(5)
  void clearPackages() => $_clearField(5);
}

class ServerManagement extends $pb.GeneratedMessage {
  factory ServerManagement({
    $fixnum.Int64? serverId,
    $core.bool? maintenanceMode,
    $core.String? maintenanceMessage,
    $core.Iterable<$core.MapEntry<$core.String, $core.String>>? labels,
    ServerCleanupPolicy? cleanupPolicy,
    $core.bool? gpuEnabled,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ServerManagement._();
    if (serverId != null) result.serverId = serverId;
    if (maintenanceMode != null) result.maintenanceMode = maintenanceMode;
    if (maintenanceMessage != null)
      result.maintenanceMessage = maintenanceMessage;
    if (labels != null) result.labels.addEntries(labels);
    if (cleanupPolicy != null) result.cleanupPolicy = cleanupPolicy;
    if (gpuEnabled != null) result.gpuEnabled = gpuEnabled;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ServerManagement._();

  factory ServerManagement.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagement()..mergeFromBuffer(data, registry);
  factory ServerManagement.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagement()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagement',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagement.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOB(2, _omitFieldNames ? '' : 'maintenanceMode')
    ..aOS(3, _omitFieldNames ? '' : 'maintenanceMessage')
    ..m<$core.String, $core.String>(4, _omitFieldNames ? '' : 'labels',
        entryClassName: 'ServerManagement.LabelsEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OS,
        packageName: const $pb.PackageName('openoxide.api.v1'))
    ..aOM<ServerCleanupPolicy>(5, _omitFieldNames ? '' : 'cleanupPolicy',
        subBuilder: ServerCleanupPolicy.$_createMessage)
    ..aOB(6, _omitFieldNames ? '' : 'gpuEnabled')
    ..aInt64(7, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagement clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagement copyWith(void Function(ServerManagement) updates) =>
      super.copyWith((message) => updates(message as ServerManagement))
          as ServerManagement;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerManagement() / ServerManagement.new instead')
  static ServerManagement create() => ServerManagement._();
  static $pb.GeneratedMessage $_createMessage() => ServerManagement._();
  @$core.override
  ServerManagement createEmptyInstance() => ServerManagement._();
  @$core.pragma('dart2js:noInline')
  static ServerManagement getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerManagement>(
          ServerManagement.$_createMessage);
  static ServerManagement? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get maintenanceMode => $_getBF(1);
  @$pb.TagNumber(2)
  set maintenanceMode($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMaintenanceMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearMaintenanceMode() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get maintenanceMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set maintenanceMessage($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMaintenanceMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearMaintenanceMessage() => $_clearField(3);

  @$pb.TagNumber(4)
  $pb.PbMap<$core.String, $core.String> get labels => $_getMap(3);

  @$pb.TagNumber(5)
  ServerCleanupPolicy get cleanupPolicy => $_getN(4);
  @$pb.TagNumber(5)
  set cleanupPolicy(ServerCleanupPolicy value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasCleanupPolicy() => $_has(4);
  @$pb.TagNumber(5)
  void clearCleanupPolicy() => $_clearField(5);
  @$pb.TagNumber(5)
  ServerCleanupPolicy ensureCleanupPolicy() => $_ensure(4);

  @$pb.TagNumber(6)
  $core.bool get gpuEnabled => $_getBF(5);
  @$pb.TagNumber(6)
  set gpuEnabled($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasGpuEnabled() => $_has(5);
  @$pb.TagNumber(6)
  void clearGpuEnabled() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get updatedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set updatedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasUpdatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearUpdatedAt() => $_clearField(7);
}

/// Generic remote-command outcome returned by repair/GPU/cleanup/upgrade/diagnostic runs.
class ServerActionResult extends $pb.GeneratedMessage {
  factory ServerActionResult({
    $core.bool? success,
    $core.String? stdout,
    $core.String? stderr,
  }) {
    final result = ServerActionResult._();
    if (success != null) result.success = success;
    if (stdout != null) result.stdout = stdout;
    if (stderr != null) result.stderr = stderr;
    return result;
  }

  ServerActionResult._();

  factory ServerActionResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerActionResult()..mergeFromBuffer(data, registry);
  factory ServerActionResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerActionResult()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerActionResult',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerActionResult.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'stdout')
    ..aOS(3, _omitFieldNames ? '' : 'stderr')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerActionResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerActionResult copyWith(void Function(ServerActionResult) updates) =>
      super.copyWith((message) => updates(message as ServerActionResult))
          as ServerActionResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerActionResult() / ServerActionResult.new instead')
  static ServerActionResult create() => ServerActionResult._();
  static $pb.GeneratedMessage $_createMessage() => ServerActionResult._();
  @$core.override
  ServerActionResult createEmptyInstance() => ServerActionResult._();
  @$core.pragma('dart2js:noInline')
  static ServerActionResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerActionResult>(
          ServerActionResult.$_createMessage);
  static ServerActionResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get stdout => $_getSZ(1);
  @$pb.TagNumber(2)
  set stdout($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStdout() => $_has(1);
  @$pb.TagNumber(2)
  void clearStdout() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get stderr => $_getSZ(2);
  @$pb.TagNumber(3)
  set stderr($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStderr() => $_has(2);
  @$pb.TagNumber(3)
  void clearStderr() => $_clearField(3);
}

class ServerBackup extends $pb.GeneratedMessage {
  factory ServerBackup({
    $core.String? remotePath,
    $fixnum.Int64? createdAt,
  }) {
    final result = ServerBackup._();
    if (remotePath != null) result.remotePath = remotePath;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  ServerBackup._();

  factory ServerBackup.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerBackup()..mergeFromBuffer(data, registry);
  factory ServerBackup.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerBackup()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerBackup',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerBackup.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'remotePath')
    ..aInt64(2, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerBackup clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerBackup copyWith(void Function(ServerBackup) updates) =>
      super.copyWith((message) => updates(message as ServerBackup))
          as ServerBackup;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerBackup() / ServerBackup.new instead')
  static ServerBackup create() => ServerBackup._();
  static $pb.GeneratedMessage $_createMessage() => ServerBackup._();
  @$core.override
  ServerBackup createEmptyInstance() => ServerBackup._();
  @$core.pragma('dart2js:noInline')
  static ServerBackup getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerBackup>(
          ServerBackup.$_createMessage);
  static ServerBackup? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get remotePath => $_getSZ(0);
  @$pb.TagNumber(1)
  set remotePath($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRemotePath() => $_has(0);
  @$pb.TagNumber(1)
  void clearRemotePath() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get createdAt => $_getI64(1);
  @$pb.TagNumber(2)
  set createdAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCreatedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearCreatedAt() => $_clearField(2);
}

class ServerCleanupExecution extends $pb.GeneratedMessage {
  factory ServerCleanupExecution({
    $fixnum.Int64? id,
    $fixnum.Int64? serverId,
    ServerCleanupStatus? status,
    $core.String? policy,
    $core.String? stdout,
    $core.String? stderr,
    $fixnum.Int64? startedAt,
    $fixnum.Int64? finishedAt,
  }) {
    final result = ServerCleanupExecution._();
    if (id != null) result.id = id;
    if (serverId != null) result.serverId = serverId;
    if (status != null) result.status = status;
    if (policy != null) result.policy = policy;
    if (stdout != null) result.stdout = stdout;
    if (stderr != null) result.stderr = stderr;
    if (startedAt != null) result.startedAt = startedAt;
    if (finishedAt != null) result.finishedAt = finishedAt;
    return result;
  }

  ServerCleanupExecution._();

  factory ServerCleanupExecution.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCleanupExecution()..mergeFromBuffer(data, registry);
  factory ServerCleanupExecution.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerCleanupExecution()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerCleanupExecution',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerCleanupExecution.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'serverId')
    ..aE<ServerCleanupStatus>(3, _omitFieldNames ? '' : 'status',
        enumValues: ServerCleanupStatus.values)
    ..aOS(4, _omitFieldNames ? '' : 'policy')
    ..aOS(5, _omitFieldNames ? '' : 'stdout')
    ..aOS(6, _omitFieldNames ? '' : 'stderr')
    ..aInt64(7, _omitFieldNames ? '' : 'startedAt')
    ..aInt64(8, _omitFieldNames ? '' : 'finishedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCleanupExecution clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerCleanupExecution copyWith(
          void Function(ServerCleanupExecution) updates) =>
      super.copyWith((message) => updates(message as ServerCleanupExecution))
          as ServerCleanupExecution;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerCleanupExecution() / ServerCleanupExecution.new instead')
  static ServerCleanupExecution create() => ServerCleanupExecution._();
  static $pb.GeneratedMessage $_createMessage() => ServerCleanupExecution._();
  @$core.override
  ServerCleanupExecution createEmptyInstance() => ServerCleanupExecution._();
  @$core.pragma('dart2js:noInline')
  static ServerCleanupExecution getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerCleanupExecution>(
          ServerCleanupExecution.$_createMessage);
  static ServerCleanupExecution? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get serverId => $_getI64(1);
  @$pb.TagNumber(2)
  set serverId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerId() => $_clearField(2);

  @$pb.TagNumber(3)
  ServerCleanupStatus get status => $_getN(2);
  @$pb.TagNumber(3)
  set status(ServerCleanupStatus value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  /// Cleanup policy snapshot as a JSON document.
  @$pb.TagNumber(4)
  $core.String get policy => $_getSZ(3);
  @$pb.TagNumber(4)
  set policy($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPolicy() => $_has(3);
  @$pb.TagNumber(4)
  void clearPolicy() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get stdout => $_getSZ(4);
  @$pb.TagNumber(5)
  set stdout($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStdout() => $_has(4);
  @$pb.TagNumber(5)
  void clearStdout() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get stderr => $_getSZ(5);
  @$pb.TagNumber(6)
  set stderr($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasStderr() => $_has(5);
  @$pb.TagNumber(6)
  void clearStderr() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get startedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set startedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasStartedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearStartedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get finishedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set finishedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasFinishedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearFinishedAt() => $_clearField(8);
}

class ServerManagementGetRequest extends $pb.GeneratedMessage {
  factory ServerManagementGetRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementGetRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementGetRequest._();

  factory ServerManagementGetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementGetRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementGetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementGetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementGetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementGetRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementGetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementGetRequest copyWith(
          void Function(ServerManagementGetRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementGetRequest))
          as ServerManagementGetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementGetRequest() / ServerManagementGetRequest.new instead')
  static ServerManagementGetRequest create() => ServerManagementGetRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementGetRequest._();
  @$core.override
  ServerManagementGetRequest createEmptyInstance() =>
      ServerManagementGetRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementGetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementGetRequest>(
          ServerManagementGetRequest.$_createMessage);
  static ServerManagementGetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementGetResponse extends $pb.GeneratedMessage {
  factory ServerManagementGetResponse({
    ServerManagement? management,
  }) {
    final result = ServerManagementGetResponse._();
    if (management != null) result.management = management;
    return result;
  }

  ServerManagementGetResponse._();

  factory ServerManagementGetResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementGetResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementGetResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementGetResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementGetResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementGetResponse.$_createMessage)
    ..aOM<ServerManagement>(1, _omitFieldNames ? '' : 'management',
        subBuilder: ServerManagement.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementGetResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementGetResponse copyWith(
          void Function(ServerManagementGetResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementGetResponse))
          as ServerManagementGetResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementGetResponse() / ServerManagementGetResponse.new instead')
  static ServerManagementGetResponse create() =>
      ServerManagementGetResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementGetResponse._();
  @$core.override
  ServerManagementGetResponse createEmptyInstance() =>
      ServerManagementGetResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementGetResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementGetResponse>(
          ServerManagementGetResponse.$_createMessage);
  static ServerManagementGetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerManagement get management => $_getN(0);
  @$pb.TagNumber(1)
  set management(ServerManagement value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasManagement() => $_has(0);
  @$pb.TagNumber(1)
  void clearManagement() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerManagement ensureManagement() => $_ensure(0);
}

/// Full replacement semantics (PUT): every field is required.
class ServerManagementUpdateRequest extends $pb.GeneratedMessage {
  factory ServerManagementUpdateRequest({
    $fixnum.Int64? id,
    $core.bool? maintenanceMode,
    $core.String? maintenanceMessage,
    $core.Iterable<$core.MapEntry<$core.String, $core.String>>? labels,
    ServerCleanupPolicy? cleanupPolicy,
    $core.bool? gpuEnabled,
  }) {
    final result = ServerManagementUpdateRequest._();
    if (id != null) result.id = id;
    if (maintenanceMode != null) result.maintenanceMode = maintenanceMode;
    if (maintenanceMessage != null)
      result.maintenanceMessage = maintenanceMessage;
    if (labels != null) result.labels.addEntries(labels);
    if (cleanupPolicy != null) result.cleanupPolicy = cleanupPolicy;
    if (gpuEnabled != null) result.gpuEnabled = gpuEnabled;
    return result;
  }

  ServerManagementUpdateRequest._();

  factory ServerManagementUpdateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpdateRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementUpdateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpdateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementUpdateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementUpdateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'maintenanceMode')
    ..aOS(3, _omitFieldNames ? '' : 'maintenanceMessage')
    ..m<$core.String, $core.String>(4, _omitFieldNames ? '' : 'labels',
        entryClassName: 'ServerManagementUpdateRequest.LabelsEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OS,
        packageName: const $pb.PackageName('openoxide.api.v1'))
    ..aOM<ServerCleanupPolicy>(5, _omitFieldNames ? '' : 'cleanupPolicy',
        subBuilder: ServerCleanupPolicy.$_createMessage)
    ..aOB(6, _omitFieldNames ? '' : 'gpuEnabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpdateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpdateRequest copyWith(
          void Function(ServerManagementUpdateRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementUpdateRequest))
          as ServerManagementUpdateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementUpdateRequest() / ServerManagementUpdateRequest.new instead')
  static ServerManagementUpdateRequest create() =>
      ServerManagementUpdateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementUpdateRequest._();
  @$core.override
  ServerManagementUpdateRequest createEmptyInstance() =>
      ServerManagementUpdateRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementUpdateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementUpdateRequest>(
          ServerManagementUpdateRequest.$_createMessage);
  static ServerManagementUpdateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get maintenanceMode => $_getBF(1);
  @$pb.TagNumber(2)
  set maintenanceMode($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMaintenanceMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearMaintenanceMode() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get maintenanceMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set maintenanceMessage($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMaintenanceMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearMaintenanceMessage() => $_clearField(3);

  @$pb.TagNumber(4)
  $pb.PbMap<$core.String, $core.String> get labels => $_getMap(3);

  @$pb.TagNumber(5)
  ServerCleanupPolicy get cleanupPolicy => $_getN(4);
  @$pb.TagNumber(5)
  set cleanupPolicy(ServerCleanupPolicy value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasCleanupPolicy() => $_has(4);
  @$pb.TagNumber(5)
  void clearCleanupPolicy() => $_clearField(5);
  @$pb.TagNumber(5)
  ServerCleanupPolicy ensureCleanupPolicy() => $_ensure(4);

  @$pb.TagNumber(6)
  $core.bool get gpuEnabled => $_getBF(5);
  @$pb.TagNumber(6)
  set gpuEnabled($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasGpuEnabled() => $_has(5);
  @$pb.TagNumber(6)
  void clearGpuEnabled() => $_clearField(6);
}

class ServerManagementUpdateResponse extends $pb.GeneratedMessage {
  factory ServerManagementUpdateResponse({
    ServerManagement? management,
  }) {
    final result = ServerManagementUpdateResponse._();
    if (management != null) result.management = management;
    return result;
  }

  ServerManagementUpdateResponse._();

  factory ServerManagementUpdateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpdateResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementUpdateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpdateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementUpdateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementUpdateResponse.$_createMessage)
    ..aOM<ServerManagement>(1, _omitFieldNames ? '' : 'management',
        subBuilder: ServerManagement.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpdateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpdateResponse copyWith(
          void Function(ServerManagementUpdateResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementUpdateResponse))
          as ServerManagementUpdateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementUpdateResponse() / ServerManagementUpdateResponse.new instead')
  static ServerManagementUpdateResponse create() =>
      ServerManagementUpdateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementUpdateResponse._();
  @$core.override
  ServerManagementUpdateResponse createEmptyInstance() =>
      ServerManagementUpdateResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementUpdateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementUpdateResponse>(
          ServerManagementUpdateResponse.$_createMessage);
  static ServerManagementUpdateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerManagement get management => $_getN(0);
  @$pb.TagNumber(1)
  set management(ServerManagement value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasManagement() => $_has(0);
  @$pb.TagNumber(1)
  void clearManagement() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerManagement ensureManagement() => $_ensure(0);
}

class ServerManagementRepairAuditRequest extends $pb.GeneratedMessage {
  factory ServerManagementRepairAuditRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementRepairAuditRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementRepairAuditRequest._();

  factory ServerManagementRepairAuditRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRepairAuditRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementRepairAuditRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRepairAuditRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementRepairAuditRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementRepairAuditRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRepairAuditRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRepairAuditRequest copyWith(
          void Function(ServerManagementRepairAuditRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementRepairAuditRequest))
          as ServerManagementRepairAuditRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementRepairAuditRequest() / ServerManagementRepairAuditRequest.new instead')
  static ServerManagementRepairAuditRequest create() =>
      ServerManagementRepairAuditRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementRepairAuditRequest._();
  @$core.override
  ServerManagementRepairAuditRequest createEmptyInstance() =>
      ServerManagementRepairAuditRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementRepairAuditRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementRepairAuditRequest>(
          ServerManagementRepairAuditRequest.$_createMessage);
  static ServerManagementRepairAuditRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementRepairAuditResponse extends $pb.GeneratedMessage {
  factory ServerManagementRepairAuditResponse({
    ServerActionResult? result,
  }) {
    final result$ = ServerManagementRepairAuditResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ServerManagementRepairAuditResponse._();

  factory ServerManagementRepairAuditResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRepairAuditResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementRepairAuditResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRepairAuditResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementRepairAuditResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementRepairAuditResponse.$_createMessage)
    ..aOM<ServerActionResult>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ServerActionResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRepairAuditResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRepairAuditResponse copyWith(
          void Function(ServerManagementRepairAuditResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementRepairAuditResponse))
          as ServerManagementRepairAuditResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementRepairAuditResponse() / ServerManagementRepairAuditResponse.new instead')
  static ServerManagementRepairAuditResponse create() =>
      ServerManagementRepairAuditResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementRepairAuditResponse._();
  @$core.override
  ServerManagementRepairAuditResponse createEmptyInstance() =>
      ServerManagementRepairAuditResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementRepairAuditResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementRepairAuditResponse>(
          ServerManagementRepairAuditResponse.$_createMessage);
  static ServerManagementRepairAuditResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerActionResult get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ServerActionResult value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerActionResult ensureResult() => $_ensure(0);
}

class ServerManagementConfigureGpuRequest extends $pb.GeneratedMessage {
  factory ServerManagementConfigureGpuRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementConfigureGpuRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementConfigureGpuRequest._();

  factory ServerManagementConfigureGpuRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementConfigureGpuRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementConfigureGpuRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementConfigureGpuRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementConfigureGpuRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementConfigureGpuRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementConfigureGpuRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementConfigureGpuRequest copyWith(
          void Function(ServerManagementConfigureGpuRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementConfigureGpuRequest))
          as ServerManagementConfigureGpuRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementConfigureGpuRequest() / ServerManagementConfigureGpuRequest.new instead')
  static ServerManagementConfigureGpuRequest create() =>
      ServerManagementConfigureGpuRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementConfigureGpuRequest._();
  @$core.override
  ServerManagementConfigureGpuRequest createEmptyInstance() =>
      ServerManagementConfigureGpuRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementConfigureGpuRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementConfigureGpuRequest>(
          ServerManagementConfigureGpuRequest.$_createMessage);
  static ServerManagementConfigureGpuRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementConfigureGpuResponse extends $pb.GeneratedMessage {
  factory ServerManagementConfigureGpuResponse({
    ServerActionResult? result,
  }) {
    final result$ = ServerManagementConfigureGpuResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ServerManagementConfigureGpuResponse._();

  factory ServerManagementConfigureGpuResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementConfigureGpuResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementConfigureGpuResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementConfigureGpuResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementConfigureGpuResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementConfigureGpuResponse.$_createMessage)
    ..aOM<ServerActionResult>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ServerActionResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementConfigureGpuResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementConfigureGpuResponse copyWith(
          void Function(ServerManagementConfigureGpuResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementConfigureGpuResponse))
          as ServerManagementConfigureGpuResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementConfigureGpuResponse() / ServerManagementConfigureGpuResponse.new instead')
  static ServerManagementConfigureGpuResponse create() =>
      ServerManagementConfigureGpuResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementConfigureGpuResponse._();
  @$core.override
  ServerManagementConfigureGpuResponse createEmptyInstance() =>
      ServerManagementConfigureGpuResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementConfigureGpuResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementConfigureGpuResponse>(
          ServerManagementConfigureGpuResponse.$_createMessage);
  static ServerManagementConfigureGpuResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerActionResult get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ServerActionResult value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerActionResult ensureResult() => $_ensure(0);
}

class ServerManagementRunCleanupRequest extends $pb.GeneratedMessage {
  factory ServerManagementRunCleanupRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementRunCleanupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementRunCleanupRequest._();

  factory ServerManagementRunCleanupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRunCleanupRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementRunCleanupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRunCleanupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementRunCleanupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementRunCleanupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRunCleanupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRunCleanupRequest copyWith(
          void Function(ServerManagementRunCleanupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementRunCleanupRequest))
          as ServerManagementRunCleanupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementRunCleanupRequest() / ServerManagementRunCleanupRequest.new instead')
  static ServerManagementRunCleanupRequest create() =>
      ServerManagementRunCleanupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementRunCleanupRequest._();
  @$core.override
  ServerManagementRunCleanupRequest createEmptyInstance() =>
      ServerManagementRunCleanupRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementRunCleanupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementRunCleanupRequest>(
          ServerManagementRunCleanupRequest.$_createMessage);
  static ServerManagementRunCleanupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementRunCleanupResponse extends $pb.GeneratedMessage {
  factory ServerManagementRunCleanupResponse({
    ServerActionResult? result,
  }) {
    final result$ = ServerManagementRunCleanupResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ServerManagementRunCleanupResponse._();

  factory ServerManagementRunCleanupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRunCleanupResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementRunCleanupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementRunCleanupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementRunCleanupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementRunCleanupResponse.$_createMessage)
    ..aOM<ServerActionResult>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ServerActionResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRunCleanupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementRunCleanupResponse copyWith(
          void Function(ServerManagementRunCleanupResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementRunCleanupResponse))
          as ServerManagementRunCleanupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementRunCleanupResponse() / ServerManagementRunCleanupResponse.new instead')
  static ServerManagementRunCleanupResponse create() =>
      ServerManagementRunCleanupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementRunCleanupResponse._();
  @$core.override
  ServerManagementRunCleanupResponse createEmptyInstance() =>
      ServerManagementRunCleanupResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementRunCleanupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementRunCleanupResponse>(
          ServerManagementRunCleanupResponse.$_createMessage);
  static ServerManagementRunCleanupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerActionResult get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ServerActionResult value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerActionResult ensureResult() => $_ensure(0);
}

class ServerManagementListCleanupHistoryRequest extends $pb.GeneratedMessage {
  factory ServerManagementListCleanupHistoryRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = ServerManagementListCleanupHistoryRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  ServerManagementListCleanupHistoryRequest._();

  factory ServerManagementListCleanupHistoryRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementListCleanupHistoryRequest()
        ..mergeFromBuffer(data, registry);
  factory ServerManagementListCleanupHistoryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementListCleanupHistoryRequest()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementListCleanupHistoryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerManagementListCleanupHistoryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementListCleanupHistoryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementListCleanupHistoryRequest copyWith(
          void Function(ServerManagementListCleanupHistoryRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementListCleanupHistoryRequest))
          as ServerManagementListCleanupHistoryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementListCleanupHistoryRequest() / ServerManagementListCleanupHistoryRequest.new instead')
  static ServerManagementListCleanupHistoryRequest create() =>
      ServerManagementListCleanupHistoryRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementListCleanupHistoryRequest._();
  @$core.override
  ServerManagementListCleanupHistoryRequest createEmptyInstance() =>
      ServerManagementListCleanupHistoryRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementListCleanupHistoryRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementListCleanupHistoryRequest>(
          ServerManagementListCleanupHistoryRequest.$_createMessage);
  static ServerManagementListCleanupHistoryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class ServerManagementListCleanupHistoryResponse extends $pb.GeneratedMessage {
  factory ServerManagementListCleanupHistoryResponse({
    $core.Iterable<ServerCleanupExecution>? executions,
  }) {
    final result = ServerManagementListCleanupHistoryResponse._();
    if (executions != null) result.executions.addAll(executions);
    return result;
  }

  ServerManagementListCleanupHistoryResponse._();

  factory ServerManagementListCleanupHistoryResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementListCleanupHistoryResponse()
        ..mergeFromBuffer(data, registry);
  factory ServerManagementListCleanupHistoryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementListCleanupHistoryResponse()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementListCleanupHistoryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerManagementListCleanupHistoryResponse.$_createMessage)
    ..pPM<ServerCleanupExecution>(1, _omitFieldNames ? '' : 'executions',
        subBuilder: ServerCleanupExecution.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementListCleanupHistoryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementListCleanupHistoryResponse copyWith(
          void Function(ServerManagementListCleanupHistoryResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementListCleanupHistoryResponse))
          as ServerManagementListCleanupHistoryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementListCleanupHistoryResponse() / ServerManagementListCleanupHistoryResponse.new instead')
  static ServerManagementListCleanupHistoryResponse create() =>
      ServerManagementListCleanupHistoryResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementListCleanupHistoryResponse._();
  @$core.override
  ServerManagementListCleanupHistoryResponse createEmptyInstance() =>
      ServerManagementListCleanupHistoryResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementListCleanupHistoryResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementListCleanupHistoryResponse>(
          ServerManagementListCleanupHistoryResponse.$_createMessage);
  static ServerManagementListCleanupHistoryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ServerCleanupExecution> get executions => $_getList(0);
}

class ServerManagementUpgradeRequest extends $pb.GeneratedMessage {
  factory ServerManagementUpgradeRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementUpgradeRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementUpgradeRequest._();

  factory ServerManagementUpgradeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpgradeRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementUpgradeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpgradeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementUpgradeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementUpgradeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpgradeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpgradeRequest copyWith(
          void Function(ServerManagementUpgradeRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementUpgradeRequest))
          as ServerManagementUpgradeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementUpgradeRequest() / ServerManagementUpgradeRequest.new instead')
  static ServerManagementUpgradeRequest create() =>
      ServerManagementUpgradeRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementUpgradeRequest._();
  @$core.override
  ServerManagementUpgradeRequest createEmptyInstance() =>
      ServerManagementUpgradeRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementUpgradeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementUpgradeRequest>(
          ServerManagementUpgradeRequest.$_createMessage);
  static ServerManagementUpgradeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementUpgradeResponse extends $pb.GeneratedMessage {
  factory ServerManagementUpgradeResponse({
    ServerActionResult? result,
  }) {
    final result$ = ServerManagementUpgradeResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ServerManagementUpgradeResponse._();

  factory ServerManagementUpgradeResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpgradeResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementUpgradeResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementUpgradeResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementUpgradeResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementUpgradeResponse.$_createMessage)
    ..aOM<ServerActionResult>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ServerActionResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpgradeResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementUpgradeResponse copyWith(
          void Function(ServerManagementUpgradeResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementUpgradeResponse))
          as ServerManagementUpgradeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementUpgradeResponse() / ServerManagementUpgradeResponse.new instead')
  static ServerManagementUpgradeResponse create() =>
      ServerManagementUpgradeResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementUpgradeResponse._();
  @$core.override
  ServerManagementUpgradeResponse createEmptyInstance() =>
      ServerManagementUpgradeResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementUpgradeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementUpgradeResponse>(
          ServerManagementUpgradeResponse.$_createMessage);
  static ServerManagementUpgradeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerActionResult get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ServerActionResult value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerActionResult ensureResult() => $_ensure(0);
}

class ServerManagementBackupRequest extends $pb.GeneratedMessage {
  factory ServerManagementBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementBackupRequest._();

  factory ServerManagementBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementBackupRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementBackupRequest copyWith(
          void Function(ServerManagementBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementBackupRequest))
          as ServerManagementBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementBackupRequest() / ServerManagementBackupRequest.new instead')
  static ServerManagementBackupRequest create() =>
      ServerManagementBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementBackupRequest._();
  @$core.override
  ServerManagementBackupRequest createEmptyInstance() =>
      ServerManagementBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementBackupRequest>(
          ServerManagementBackupRequest.$_createMessage);
  static ServerManagementBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementBackupResponse extends $pb.GeneratedMessage {
  factory ServerManagementBackupResponse({
    ServerBackup? backup,
  }) {
    final result = ServerManagementBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  ServerManagementBackupResponse._();

  factory ServerManagementBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementBackupResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementBackupResponse.$_createMessage)
    ..aOM<ServerBackup>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: ServerBackup.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementBackupResponse copyWith(
          void Function(ServerManagementBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerManagementBackupResponse))
          as ServerManagementBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementBackupResponse() / ServerManagementBackupResponse.new instead')
  static ServerManagementBackupResponse create() =>
      ServerManagementBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementBackupResponse._();
  @$core.override
  ServerManagementBackupResponse createEmptyInstance() =>
      ServerManagementBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementBackupResponse>(
          ServerManagementBackupResponse.$_createMessage);
  static ServerManagementBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerBackup get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(ServerBackup value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerBackup ensureBackup() => $_ensure(0);
}

class ServerManagementDiagnosticsRequest extends $pb.GeneratedMessage {
  factory ServerManagementDiagnosticsRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerManagementDiagnosticsRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerManagementDiagnosticsRequest._();

  factory ServerManagementDiagnosticsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementDiagnosticsRequest()..mergeFromBuffer(data, registry);
  factory ServerManagementDiagnosticsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementDiagnosticsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementDiagnosticsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementDiagnosticsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementDiagnosticsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementDiagnosticsRequest copyWith(
          void Function(ServerManagementDiagnosticsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementDiagnosticsRequest))
          as ServerManagementDiagnosticsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementDiagnosticsRequest() / ServerManagementDiagnosticsRequest.new instead')
  static ServerManagementDiagnosticsRequest create() =>
      ServerManagementDiagnosticsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementDiagnosticsRequest._();
  @$core.override
  ServerManagementDiagnosticsRequest createEmptyInstance() =>
      ServerManagementDiagnosticsRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementDiagnosticsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerManagementDiagnosticsRequest>(
          ServerManagementDiagnosticsRequest.$_createMessage);
  static ServerManagementDiagnosticsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerManagementDiagnosticsResponse extends $pb.GeneratedMessage {
  factory ServerManagementDiagnosticsResponse({
    ServerActionResult? result,
  }) {
    final result$ = ServerManagementDiagnosticsResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ServerManagementDiagnosticsResponse._();

  factory ServerManagementDiagnosticsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementDiagnosticsResponse()..mergeFromBuffer(data, registry);
  factory ServerManagementDiagnosticsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerManagementDiagnosticsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerManagementDiagnosticsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerManagementDiagnosticsResponse.$_createMessage)
    ..aOM<ServerActionResult>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ServerActionResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementDiagnosticsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerManagementDiagnosticsResponse copyWith(
          void Function(ServerManagementDiagnosticsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerManagementDiagnosticsResponse))
          as ServerManagementDiagnosticsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerManagementDiagnosticsResponse() / ServerManagementDiagnosticsResponse.new instead')
  static ServerManagementDiagnosticsResponse create() =>
      ServerManagementDiagnosticsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerManagementDiagnosticsResponse._();
  @$core.override
  ServerManagementDiagnosticsResponse createEmptyInstance() =>
      ServerManagementDiagnosticsResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerManagementDiagnosticsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerManagementDiagnosticsResponse>(
          ServerManagementDiagnosticsResponse.$_createMessage);
  static ServerManagementDiagnosticsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerActionResult get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ServerActionResult value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerActionResult ensureResult() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

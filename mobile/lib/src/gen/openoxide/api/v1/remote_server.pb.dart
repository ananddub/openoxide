// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/remote_server.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'remote_server.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'remote_server.pbenum.dart';

class RemoteServer extends $pb.GeneratedMessage {
  factory RemoteServer({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    $core.String? appName,
    RemoteServerStatus? serverStatus,
    RemoteServerType? serverType,
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
    final result = RemoteServer._();
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

  RemoteServer._();

  factory RemoteServer.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServer()..mergeFromBuffer(data, registry);
  factory RemoteServer.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServer()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServer',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServer.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(5, _omitFieldNames ? '' : 'port')
    ..aOS(6, _omitFieldNames ? '' : 'username')
    ..aOS(7, _omitFieldNames ? '' : 'appName')
    ..aE<RemoteServerStatus>(8, _omitFieldNames ? '' : 'serverStatus',
        enumValues: RemoteServerStatus.values)
    ..aE<RemoteServerType>(9, _omitFieldNames ? '' : 'serverType',
        enumValues: RemoteServerType.values)
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
  RemoteServer clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServer copyWith(void Function(RemoteServer) updates) =>
      super.copyWith((message) => updates(message as RemoteServer))
          as RemoteServer;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use RemoteServer() / RemoteServer.new instead')
  static RemoteServer create() => RemoteServer._();
  static $pb.GeneratedMessage $_createMessage() => RemoteServer._();
  @$core.override
  RemoteServer createEmptyInstance() => RemoteServer._();
  @$core.pragma('dart2js:noInline')
  static RemoteServer getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoteServer>(
          RemoteServer.$_createMessage);
  static RemoteServer? _defaultInstance;

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
  RemoteServerStatus get serverStatus => $_getN(7);
  @$pb.TagNumber(8)
  set serverStatus(RemoteServerStatus value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasServerStatus() => $_has(7);
  @$pb.TagNumber(8)
  void clearServerStatus() => $_clearField(8);

  @$pb.TagNumber(9)
  RemoteServerType get serverType => $_getN(8);
  @$pb.TagNumber(9)
  set serverType(RemoteServerType value) => $_setField(9, value);
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

/// Result of `server_migrations` metadata moves (whole/dependency scope).
class RemoteServerDependencyMigration extends $pb.GeneratedMessage {
  factory RemoteServerDependencyMigration({
    $core.String? migrationId,
    RemoteServerDependencyMigrationStatus? status,
    $fixnum.Int64? sourceServerId,
    $fixnum.Int64? targetServerId,
    $fixnum.Int64? applications,
    $fixnum.Int64? buildAssignments,
    $fixnum.Int64? composeProjects,
    $fixnum.Int64? databases,
    $fixnum.Int64? certificates,
    $fixnum.Int64? schedules,
    $fixnum.Int64? queuedApplications,
    $fixnum.Int64? queuedComposeProjects,
    $core.String? error,
  }) {
    final result = RemoteServerDependencyMigration._();
    if (migrationId != null) result.migrationId = migrationId;
    if (status != null) result.status = status;
    if (sourceServerId != null) result.sourceServerId = sourceServerId;
    if (targetServerId != null) result.targetServerId = targetServerId;
    if (applications != null) result.applications = applications;
    if (buildAssignments != null) result.buildAssignments = buildAssignments;
    if (composeProjects != null) result.composeProjects = composeProjects;
    if (databases != null) result.databases = databases;
    if (certificates != null) result.certificates = certificates;
    if (schedules != null) result.schedules = schedules;
    if (queuedApplications != null)
      result.queuedApplications = queuedApplications;
    if (queuedComposeProjects != null)
      result.queuedComposeProjects = queuedComposeProjects;
    if (error != null) result.error = error;
    return result;
  }

  RemoteServerDependencyMigration._();

  factory RemoteServerDependencyMigration.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDependencyMigration()..mergeFromBuffer(data, registry);
  factory RemoteServerDependencyMigration.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDependencyMigration()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerDependencyMigration',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerDependencyMigration.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..aE<RemoteServerDependencyMigrationStatus>(
        2, _omitFieldNames ? '' : 'status',
        enumValues: RemoteServerDependencyMigrationStatus.values)
    ..aInt64(3, _omitFieldNames ? '' : 'sourceServerId')
    ..aInt64(4, _omitFieldNames ? '' : 'targetServerId')
    ..aInt64(5, _omitFieldNames ? '' : 'applications')
    ..aInt64(6, _omitFieldNames ? '' : 'buildAssignments')
    ..aInt64(7, _omitFieldNames ? '' : 'composeProjects')
    ..aInt64(8, _omitFieldNames ? '' : 'databases')
    ..aInt64(9, _omitFieldNames ? '' : 'certificates')
    ..aInt64(10, _omitFieldNames ? '' : 'schedules')
    ..aInt64(11, _omitFieldNames ? '' : 'queuedApplications')
    ..aInt64(12, _omitFieldNames ? '' : 'queuedComposeProjects')
    ..aOS(13, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDependencyMigration clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDependencyMigration copyWith(
          void Function(RemoteServerDependencyMigration) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerDependencyMigration))
          as RemoteServerDependencyMigration;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerDependencyMigration() / RemoteServerDependencyMigration.new instead')
  static RemoteServerDependencyMigration create() =>
      RemoteServerDependencyMigration._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerDependencyMigration._();
  @$core.override
  RemoteServerDependencyMigration createEmptyInstance() =>
      RemoteServerDependencyMigration._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerDependencyMigration getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerDependencyMigration>(
          RemoteServerDependencyMigration.$_createMessage);
  static RemoteServerDependencyMigration? _defaultInstance;

  /// UUID migration identifier.
  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);

  @$pb.TagNumber(2)
  RemoteServerDependencyMigrationStatus get status => $_getN(1);
  @$pb.TagNumber(2)
  set status(RemoteServerDependencyMigrationStatus value) =>
      $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sourceServerId => $_getI64(2);
  @$pb.TagNumber(3)
  set sourceServerId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSourceServerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSourceServerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get targetServerId => $_getI64(3);
  @$pb.TagNumber(4)
  set targetServerId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTargetServerId() => $_has(3);
  @$pb.TagNumber(4)
  void clearTargetServerId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get applications => $_getI64(4);
  @$pb.TagNumber(5)
  set applications($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasApplications() => $_has(4);
  @$pb.TagNumber(5)
  void clearApplications() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get buildAssignments => $_getI64(5);
  @$pb.TagNumber(6)
  set buildAssignments($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasBuildAssignments() => $_has(5);
  @$pb.TagNumber(6)
  void clearBuildAssignments() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get composeProjects => $_getI64(6);
  @$pb.TagNumber(7)
  set composeProjects($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasComposeProjects() => $_has(6);
  @$pb.TagNumber(7)
  void clearComposeProjects() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get databases => $_getI64(7);
  @$pb.TagNumber(8)
  set databases($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabases() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabases() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get certificates => $_getI64(8);
  @$pb.TagNumber(9)
  set certificates($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCertificates() => $_has(8);
  @$pb.TagNumber(9)
  void clearCertificates() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get schedules => $_getI64(9);
  @$pb.TagNumber(10)
  set schedules($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasSchedules() => $_has(9);
  @$pb.TagNumber(10)
  void clearSchedules() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get queuedApplications => $_getI64(10);
  @$pb.TagNumber(11)
  set queuedApplications($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasQueuedApplications() => $_has(10);
  @$pb.TagNumber(11)
  void clearQueuedApplications() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get queuedComposeProjects => $_getI64(11);
  @$pb.TagNumber(12)
  set queuedComposeProjects($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasQueuedComposeProjects() => $_has(11);
  @$pb.TagNumber(12)
  void clearQueuedComposeProjects() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get error => $_getSZ(12);
  @$pb.TagNumber(13)
  set error($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasError() => $_has(12);
  @$pb.TagNumber(13)
  void clearError() => $_clearField(13);
}

/// A stateful `application_migrations` row (single-service migration).
class RemoteServerApplicationMigration extends $pb.GeneratedMessage {
  factory RemoteServerApplicationMigration({
    $core.String? id,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? sourceServerId,
    $fixnum.Int64? targetServerId,
    RemoteServerApplicationMigrationStatus? status,
    RemoteServerMigrationPhase? phase,
    $fixnum.Int64? progress,
    $core.bool? cancelRequested,
    $core.String? error,
    $core.String? logPath,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = RemoteServerApplicationMigration._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (serviceId != null) result.serviceId = serviceId;
    if (sourceServerId != null) result.sourceServerId = sourceServerId;
    if (targetServerId != null) result.targetServerId = targetServerId;
    if (status != null) result.status = status;
    if (phase != null) result.phase = phase;
    if (progress != null) result.progress = progress;
    if (cancelRequested != null) result.cancelRequested = cancelRequested;
    if (error != null) result.error = error;
    if (logPath != null) result.logPath = logPath;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  RemoteServerApplicationMigration._();

  factory RemoteServerApplicationMigration.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerApplicationMigration()..mergeFromBuffer(data, registry);
  factory RemoteServerApplicationMigration.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerApplicationMigration()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerApplicationMigration',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerApplicationMigration.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(3, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(4, _omitFieldNames ? '' : 'sourceServerId')
    ..aInt64(5, _omitFieldNames ? '' : 'targetServerId')
    ..aE<RemoteServerApplicationMigrationStatus>(
        6, _omitFieldNames ? '' : 'status',
        enumValues: RemoteServerApplicationMigrationStatus.values)
    ..aE<RemoteServerMigrationPhase>(7, _omitFieldNames ? '' : 'phase',
        enumValues: RemoteServerMigrationPhase.values)
    ..aInt64(8, _omitFieldNames ? '' : 'progress')
    ..aOB(9, _omitFieldNames ? '' : 'cancelRequested')
    ..aOS(10, _omitFieldNames ? '' : 'error')
    ..aOS(11, _omitFieldNames ? '' : 'logPath')
    ..aInt64(12, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(13, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerApplicationMigration clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerApplicationMigration copyWith(
          void Function(RemoteServerApplicationMigration) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerApplicationMigration))
          as RemoteServerApplicationMigration;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerApplicationMigration() / RemoteServerApplicationMigration.new instead')
  static RemoteServerApplicationMigration create() =>
      RemoteServerApplicationMigration._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerApplicationMigration._();
  @$core.override
  RemoteServerApplicationMigration createEmptyInstance() =>
      RemoteServerApplicationMigration._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerApplicationMigration getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerApplicationMigration>(
          RemoteServerApplicationMigration.$_createMessage);
  static RemoteServerApplicationMigration? _defaultInstance;

  /// UUID primary key.
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get serviceId => $_getI64(2);
  @$pb.TagNumber(3)
  set serviceId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasServiceId() => $_has(2);
  @$pb.TagNumber(3)
  void clearServiceId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get sourceServerId => $_getI64(3);
  @$pb.TagNumber(4)
  set sourceServerId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSourceServerId() => $_has(3);
  @$pb.TagNumber(4)
  void clearSourceServerId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get targetServerId => $_getI64(4);
  @$pb.TagNumber(5)
  set targetServerId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTargetServerId() => $_has(4);
  @$pb.TagNumber(5)
  void clearTargetServerId() => $_clearField(5);

  @$pb.TagNumber(6)
  RemoteServerApplicationMigrationStatus get status => $_getN(5);
  @$pb.TagNumber(6)
  set status(RemoteServerApplicationMigrationStatus value) =>
      $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearStatus() => $_clearField(6);

  @$pb.TagNumber(7)
  RemoteServerMigrationPhase get phase => $_getN(6);
  @$pb.TagNumber(7)
  set phase(RemoteServerMigrationPhase value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasPhase() => $_has(6);
  @$pb.TagNumber(7)
  void clearPhase() => $_clearField(7);

  /// Percent complete (0-100).
  @$pb.TagNumber(8)
  $fixnum.Int64 get progress => $_getI64(7);
  @$pb.TagNumber(8)
  set progress($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasProgress() => $_has(7);
  @$pb.TagNumber(8)
  void clearProgress() => $_clearField(8);

  /// Cooperative cancellation flag, integer-as-boolean in storage.
  @$pb.TagNumber(9)
  $core.bool get cancelRequested => $_getBF(8);
  @$pb.TagNumber(9)
  set cancelRequested($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCancelRequested() => $_has(8);
  @$pb.TagNumber(9)
  void clearCancelRequested() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get error => $_getSZ(9);
  @$pb.TagNumber(10)
  set error($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasError() => $_has(9);
  @$pb.TagNumber(10)
  void clearError() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get logPath => $_getSZ(10);
  @$pb.TagNumber(11)
  set logPath($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLogPath() => $_has(10);
  @$pb.TagNumber(11)
  void clearLogPath() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get createdAt => $_getI64(11);
  @$pb.TagNumber(12)
  set createdAt($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasCreatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearCreatedAt() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get updatedAt => $_getI64(12);
  @$pb.TagNumber(13)
  set updatedAt($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasUpdatedAt() => $_has(12);
  @$pb.TagNumber(13)
  void clearUpdatedAt() => $_clearField(13);
}

/// A per-object checkpoint of a stateful migration.
class RemoteServerMigrationObject extends $pb.GeneratedMessage {
  factory RemoteServerMigrationObject({
    $core.String? migrationId,
    $core.String? objKey,
    RemoteServerMigrationObjectKind? kind,
    $core.String? sourceRef,
    $core.String? destRef,
    $core.String? status,
    $fixnum.Int64? bytes,
    $core.String? error,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = RemoteServerMigrationObject._();
    if (migrationId != null) result.migrationId = migrationId;
    if (objKey != null) result.objKey = objKey;
    if (kind != null) result.kind = kind;
    if (sourceRef != null) result.sourceRef = sourceRef;
    if (destRef != null) result.destRef = destRef;
    if (status != null) result.status = status;
    if (bytes != null) result.bytes = bytes;
    if (error != null) result.error = error;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  RemoteServerMigrationObject._();

  factory RemoteServerMigrationObject.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrationObject()..mergeFromBuffer(data, registry);
  factory RemoteServerMigrationObject.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrationObject()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerMigrationObject',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerMigrationObject.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..aOS(2, _omitFieldNames ? '' : 'objKey')
    ..aE<RemoteServerMigrationObjectKind>(3, _omitFieldNames ? '' : 'kind',
        enumValues: RemoteServerMigrationObjectKind.values)
    ..aOS(4, _omitFieldNames ? '' : 'sourceRef')
    ..aOS(5, _omitFieldNames ? '' : 'destRef')
    ..aOS(6, _omitFieldNames ? '' : 'status')
    ..aInt64(7, _omitFieldNames ? '' : 'bytes')
    ..aOS(8, _omitFieldNames ? '' : 'error')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrationObject clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrationObject copyWith(
          void Function(RemoteServerMigrationObject) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerMigrationObject))
          as RemoteServerMigrationObject;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerMigrationObject() / RemoteServerMigrationObject.new instead')
  static RemoteServerMigrationObject create() =>
      RemoteServerMigrationObject._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerMigrationObject._();
  @$core.override
  RemoteServerMigrationObject createEmptyInstance() =>
      RemoteServerMigrationObject._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerMigrationObject getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerMigrationObject>(
          RemoteServerMigrationObject.$_createMessage);
  static RemoteServerMigrationObject? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);

  /// Hash-derived unique key of the tracked object within the migration.
  @$pb.TagNumber(2)
  $core.String get objKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjKey() => $_clearField(2);

  @$pb.TagNumber(3)
  RemoteServerMigrationObjectKind get kind => $_getN(2);
  @$pb.TagNumber(3)
  set kind(RemoteServerMigrationObjectKind value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasKind() => $_has(2);
  @$pb.TagNumber(3)
  void clearKind() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get sourceRef => $_getSZ(3);
  @$pb.TagNumber(4)
  set sourceRef($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSourceRef() => $_has(3);
  @$pb.TagNumber(4)
  void clearSourceRef() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get destRef => $_getSZ(4);
  @$pb.TagNumber(5)
  set destRef($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDestRef() => $_has(4);
  @$pb.TagNumber(5)
  void clearDestRef() => $_clearField(5);

  /// Checkpoint status as recorded by the runner (open set: e.g. "PENDING", "VERIFIED", "APPLIED", "DONE", "FAILED").
  @$pb.TagNumber(6)
  $core.String get status => $_getSZ(5);
  @$pb.TagNumber(6)
  set status($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearStatus() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get bytes => $_getI64(6);
  @$pb.TagNumber(7)
  set bytes($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBytes() => $_has(6);
  @$pb.TagNumber(7)
  void clearBytes() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get error => $_getSZ(7);
  @$pb.TagNumber(8)
  set error($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasError() => $_has(7);
  @$pb.TagNumber(8)
  void clearError() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get createdAt => $_getI64(8);
  @$pb.TagNumber(9)
  set createdAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCreatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearCreatedAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get updatedAt => $_getI64(9);
  @$pb.TagNumber(10)
  set updatedAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUpdatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearUpdatedAt() => $_clearField(10);
}

class RemoteServerListRequest extends $pb.GeneratedMessage {
  factory RemoteServerListRequest() => RemoteServerListRequest._();

  RemoteServerListRequest._();

  factory RemoteServerListRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerListRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerListRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListRequest copyWith(
          void Function(RemoteServerListRequest) updates) =>
      super.copyWith((message) => updates(message as RemoteServerListRequest))
          as RemoteServerListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListRequest() / RemoteServerListRequest.new instead')
  static RemoteServerListRequest create() => RemoteServerListRequest._();
  static $pb.GeneratedMessage $_createMessage() => RemoteServerListRequest._();
  @$core.override
  RemoteServerListRequest createEmptyInstance() => RemoteServerListRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerListRequest>(
          RemoteServerListRequest.$_createMessage);
  static RemoteServerListRequest? _defaultInstance;
}

class RemoteServerListResponse extends $pb.GeneratedMessage {
  factory RemoteServerListResponse({
    $core.Iterable<RemoteServer>? servers,
  }) {
    final result = RemoteServerListResponse._();
    if (servers != null) result.servers.addAll(servers);
    return result;
  }

  RemoteServerListResponse._();

  factory RemoteServerListResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerListResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerListResponse.$_createMessage)
    ..pPM<RemoteServer>(1, _omitFieldNames ? '' : 'servers',
        subBuilder: RemoteServer.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListResponse copyWith(
          void Function(RemoteServerListResponse) updates) =>
      super.copyWith((message) => updates(message as RemoteServerListResponse))
          as RemoteServerListResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListResponse() / RemoteServerListResponse.new instead')
  static RemoteServerListResponse create() => RemoteServerListResponse._();
  static $pb.GeneratedMessage $_createMessage() => RemoteServerListResponse._();
  @$core.override
  RemoteServerListResponse createEmptyInstance() =>
      RemoteServerListResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerListResponse>(
          RemoteServerListResponse.$_createMessage);
  static RemoteServerListResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RemoteServer> get servers => $_getList(0);
}

class RemoteServerGetRequest extends $pb.GeneratedMessage {
  factory RemoteServerGetRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoteServerGetRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoteServerGetRequest._();

  factory RemoteServerGetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerGetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerGetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerGetRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetRequest copyWith(
          void Function(RemoteServerGetRequest) updates) =>
      super.copyWith((message) => updates(message as RemoteServerGetRequest))
          as RemoteServerGetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerGetRequest() / RemoteServerGetRequest.new instead')
  static RemoteServerGetRequest create() => RemoteServerGetRequest._();
  static $pb.GeneratedMessage $_createMessage() => RemoteServerGetRequest._();
  @$core.override
  RemoteServerGetRequest createEmptyInstance() => RemoteServerGetRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerGetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerGetRequest>(
          RemoteServerGetRequest.$_createMessage);
  static RemoteServerGetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RemoteServerGetResponse extends $pb.GeneratedMessage {
  factory RemoteServerGetResponse({
    RemoteServer? server,
  }) {
    final result = RemoteServerGetResponse._();
    if (server != null) result.server = server;
    return result;
  }

  RemoteServerGetResponse._();

  factory RemoteServerGetResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerGetResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerGetResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerGetResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetResponse copyWith(
          void Function(RemoteServerGetResponse) updates) =>
      super.copyWith((message) => updates(message as RemoteServerGetResponse))
          as RemoteServerGetResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerGetResponse() / RemoteServerGetResponse.new instead')
  static RemoteServerGetResponse create() => RemoteServerGetResponse._();
  static $pb.GeneratedMessage $_createMessage() => RemoteServerGetResponse._();
  @$core.override
  RemoteServerGetResponse createEmptyInstance() => RemoteServerGetResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerGetResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerGetResponse>(
          RemoteServerGetResponse.$_createMessage);
  static RemoteServerGetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);
}

class RemoteServerCreateRequest extends $pb.GeneratedMessage {
  factory RemoteServerCreateRequest({
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    RemoteServerType? serverType,
    $fixnum.Int64? sshKeyId,
    $core.String? buildMemoryLimit,
    $core.String? buildCpuLimit,
  }) {
    final result = RemoteServerCreateRequest._();
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

  RemoteServerCreateRequest._();

  factory RemoteServerCreateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCreateRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerCreateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCreateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerCreateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerCreateRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aOS(3, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(4, _omitFieldNames ? '' : 'port')
    ..aOS(5, _omitFieldNames ? '' : 'username')
    ..aE<RemoteServerType>(6, _omitFieldNames ? '' : 'serverType',
        enumValues: RemoteServerType.values)
    ..aInt64(7, _omitFieldNames ? '' : 'sshKeyId')
    ..aOS(8, _omitFieldNames ? '' : 'buildMemoryLimit')
    ..aOS(9, _omitFieldNames ? '' : 'buildCpuLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCreateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCreateRequest copyWith(
          void Function(RemoteServerCreateRequest) updates) =>
      super.copyWith((message) => updates(message as RemoteServerCreateRequest))
          as RemoteServerCreateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerCreateRequest() / RemoteServerCreateRequest.new instead')
  static RemoteServerCreateRequest create() => RemoteServerCreateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerCreateRequest._();
  @$core.override
  RemoteServerCreateRequest createEmptyInstance() =>
      RemoteServerCreateRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerCreateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerCreateRequest>(
          RemoteServerCreateRequest.$_createMessage);
  static RemoteServerCreateRequest? _defaultInstance;

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
  RemoteServerType get serverType => $_getN(5);
  @$pb.TagNumber(6)
  set serverType(RemoteServerType value) => $_setField(6, value);
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

class RemoteServerCreateResponse extends $pb.GeneratedMessage {
  factory RemoteServerCreateResponse({
    RemoteServer? server,
  }) {
    final result = RemoteServerCreateResponse._();
    if (server != null) result.server = server;
    return result;
  }

  RemoteServerCreateResponse._();

  factory RemoteServerCreateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCreateResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerCreateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCreateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerCreateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerCreateResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCreateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCreateResponse copyWith(
          void Function(RemoteServerCreateResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerCreateResponse))
          as RemoteServerCreateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerCreateResponse() / RemoteServerCreateResponse.new instead')
  static RemoteServerCreateResponse create() => RemoteServerCreateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerCreateResponse._();
  @$core.override
  RemoteServerCreateResponse createEmptyInstance() =>
      RemoteServerCreateResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerCreateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerCreateResponse>(
          RemoteServerCreateResponse.$_createMessage);
  static RemoteServerCreateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);
}

/// PATCH semantics: absent fields keep their stored values.
class RemoteServerUpdateRequest extends $pb.GeneratedMessage {
  factory RemoteServerUpdateRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? ipAddress,
    $fixnum.Int64? port,
    $core.String? username,
    RemoteServerStatus? serverStatus,
    RemoteServerType? serverType,
    $core.bool? enableDockerCleanup,
    $core.String? logCleanupCron,
    $core.String? command,
    $core.String? metricsConfig,
    $fixnum.Int64? sshKeyId,
    $core.String? buildMemoryLimit,
    $core.String? buildCpuLimit,
  }) {
    final result = RemoteServerUpdateRequest._();
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

  RemoteServerUpdateRequest._();

  factory RemoteServerUpdateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerUpdateRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerUpdateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerUpdateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerUpdateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerUpdateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'ipAddress')
    ..aInt64(5, _omitFieldNames ? '' : 'port')
    ..aOS(6, _omitFieldNames ? '' : 'username')
    ..aE<RemoteServerStatus>(7, _omitFieldNames ? '' : 'serverStatus',
        enumValues: RemoteServerStatus.values)
    ..aE<RemoteServerType>(8, _omitFieldNames ? '' : 'serverType',
        enumValues: RemoteServerType.values)
    ..aOB(9, _omitFieldNames ? '' : 'enableDockerCleanup')
    ..aOS(10, _omitFieldNames ? '' : 'logCleanupCron')
    ..aOS(11, _omitFieldNames ? '' : 'command')
    ..aOS(12, _omitFieldNames ? '' : 'metricsConfig')
    ..aInt64(13, _omitFieldNames ? '' : 'sshKeyId')
    ..aOS(14, _omitFieldNames ? '' : 'buildMemoryLimit')
    ..aOS(15, _omitFieldNames ? '' : 'buildCpuLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerUpdateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerUpdateRequest copyWith(
          void Function(RemoteServerUpdateRequest) updates) =>
      super.copyWith((message) => updates(message as RemoteServerUpdateRequest))
          as RemoteServerUpdateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerUpdateRequest() / RemoteServerUpdateRequest.new instead')
  static RemoteServerUpdateRequest create() => RemoteServerUpdateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerUpdateRequest._();
  @$core.override
  RemoteServerUpdateRequest createEmptyInstance() =>
      RemoteServerUpdateRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerUpdateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerUpdateRequest>(
          RemoteServerUpdateRequest.$_createMessage);
  static RemoteServerUpdateRequest? _defaultInstance;

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
  RemoteServerStatus get serverStatus => $_getN(6);
  @$pb.TagNumber(7)
  set serverStatus(RemoteServerStatus value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasServerStatus() => $_has(6);
  @$pb.TagNumber(7)
  void clearServerStatus() => $_clearField(7);

  @$pb.TagNumber(8)
  RemoteServerType get serverType => $_getN(7);
  @$pb.TagNumber(8)
  set serverType(RemoteServerType value) => $_setField(8, value);
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

class RemoteServerUpdateResponse extends $pb.GeneratedMessage {
  factory RemoteServerUpdateResponse({
    RemoteServer? server,
  }) {
    final result = RemoteServerUpdateResponse._();
    if (server != null) result.server = server;
    return result;
  }

  RemoteServerUpdateResponse._();

  factory RemoteServerUpdateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerUpdateResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerUpdateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerUpdateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerUpdateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerUpdateResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerUpdateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerUpdateResponse copyWith(
          void Function(RemoteServerUpdateResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerUpdateResponse))
          as RemoteServerUpdateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerUpdateResponse() / RemoteServerUpdateResponse.new instead')
  static RemoteServerUpdateResponse create() => RemoteServerUpdateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerUpdateResponse._();
  @$core.override
  RemoteServerUpdateResponse createEmptyInstance() =>
      RemoteServerUpdateResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerUpdateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerUpdateResponse>(
          RemoteServerUpdateResponse.$_createMessage);
  static RemoteServerUpdateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);
}

class RemoteServerActivateRequest extends $pb.GeneratedMessage {
  factory RemoteServerActivateRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoteServerActivateRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoteServerActivateRequest._();

  factory RemoteServerActivateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerActivateRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerActivateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerActivateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerActivateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerActivateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerActivateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerActivateRequest copyWith(
          void Function(RemoteServerActivateRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerActivateRequest))
          as RemoteServerActivateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerActivateRequest() / RemoteServerActivateRequest.new instead')
  static RemoteServerActivateRequest create() =>
      RemoteServerActivateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerActivateRequest._();
  @$core.override
  RemoteServerActivateRequest createEmptyInstance() =>
      RemoteServerActivateRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerActivateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerActivateRequest>(
          RemoteServerActivateRequest.$_createMessage);
  static RemoteServerActivateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RemoteServerActivateResponse extends $pb.GeneratedMessage {
  factory RemoteServerActivateResponse({
    RemoteServer? server,
    RemoteServerAction? action,
  }) {
    final result = RemoteServerActivateResponse._();
    if (server != null) result.server = server;
    if (action != null) result.action = action;
    return result;
  }

  RemoteServerActivateResponse._();

  factory RemoteServerActivateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerActivateResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerActivateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerActivateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerActivateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerActivateResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..aE<RemoteServerAction>(2, _omitFieldNames ? '' : 'action',
        enumValues: RemoteServerAction.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerActivateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerActivateResponse copyWith(
          void Function(RemoteServerActivateResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerActivateResponse))
          as RemoteServerActivateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerActivateResponse() / RemoteServerActivateResponse.new instead')
  static RemoteServerActivateResponse create() =>
      RemoteServerActivateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerActivateResponse._();
  @$core.override
  RemoteServerActivateResponse createEmptyInstance() =>
      RemoteServerActivateResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerActivateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerActivateResponse>(
          RemoteServerActivateResponse.$_createMessage);
  static RemoteServerActivateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);

  @$pb.TagNumber(2)
  RemoteServerAction get action => $_getN(1);
  @$pb.TagNumber(2)
  set action(RemoteServerAction value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);
}

class RemoteServerDeactivateRequest extends $pb.GeneratedMessage {
  factory RemoteServerDeactivateRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoteServerDeactivateRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoteServerDeactivateRequest._();

  factory RemoteServerDeactivateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeactivateRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerDeactivateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeactivateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerDeactivateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerDeactivateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeactivateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeactivateRequest copyWith(
          void Function(RemoteServerDeactivateRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerDeactivateRequest))
          as RemoteServerDeactivateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerDeactivateRequest() / RemoteServerDeactivateRequest.new instead')
  static RemoteServerDeactivateRequest create() =>
      RemoteServerDeactivateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerDeactivateRequest._();
  @$core.override
  RemoteServerDeactivateRequest createEmptyInstance() =>
      RemoteServerDeactivateRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerDeactivateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerDeactivateRequest>(
          RemoteServerDeactivateRequest.$_createMessage);
  static RemoteServerDeactivateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RemoteServerDeactivateResponse extends $pb.GeneratedMessage {
  factory RemoteServerDeactivateResponse({
    RemoteServer? server,
    RemoteServerAction? action,
  }) {
    final result = RemoteServerDeactivateResponse._();
    if (server != null) result.server = server;
    if (action != null) result.action = action;
    return result;
  }

  RemoteServerDeactivateResponse._();

  factory RemoteServerDeactivateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeactivateResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerDeactivateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeactivateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerDeactivateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerDeactivateResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..aE<RemoteServerAction>(2, _omitFieldNames ? '' : 'action',
        enumValues: RemoteServerAction.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeactivateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeactivateResponse copyWith(
          void Function(RemoteServerDeactivateResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerDeactivateResponse))
          as RemoteServerDeactivateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerDeactivateResponse() / RemoteServerDeactivateResponse.new instead')
  static RemoteServerDeactivateResponse create() =>
      RemoteServerDeactivateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerDeactivateResponse._();
  @$core.override
  RemoteServerDeactivateResponse createEmptyInstance() =>
      RemoteServerDeactivateResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerDeactivateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerDeactivateResponse>(
          RemoteServerDeactivateResponse.$_createMessage);
  static RemoteServerDeactivateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);

  @$pb.TagNumber(2)
  RemoteServerAction get action => $_getN(1);
  @$pb.TagNumber(2)
  set action(RemoteServerAction value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);
}

class RemoteServerTestConnectionRequest extends $pb.GeneratedMessage {
  factory RemoteServerTestConnectionRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoteServerTestConnectionRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoteServerTestConnectionRequest._();

  factory RemoteServerTestConnectionRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerTestConnectionRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerTestConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerTestConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerTestConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerTestConnectionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerTestConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerTestConnectionRequest copyWith(
          void Function(RemoteServerTestConnectionRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerTestConnectionRequest))
          as RemoteServerTestConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerTestConnectionRequest() / RemoteServerTestConnectionRequest.new instead')
  static RemoteServerTestConnectionRequest create() =>
      RemoteServerTestConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerTestConnectionRequest._();
  @$core.override
  RemoteServerTestConnectionRequest createEmptyInstance() =>
      RemoteServerTestConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerTestConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerTestConnectionRequest>(
          RemoteServerTestConnectionRequest.$_createMessage);
  static RemoteServerTestConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RemoteServerTestConnectionResponse extends $pb.GeneratedMessage {
  factory RemoteServerTestConnectionResponse({
    RemoteServer? server,
    RemoteServerAction? action,
  }) {
    final result = RemoteServerTestConnectionResponse._();
    if (server != null) result.server = server;
    if (action != null) result.action = action;
    return result;
  }

  RemoteServerTestConnectionResponse._();

  factory RemoteServerTestConnectionResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerTestConnectionResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerTestConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerTestConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerTestConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerTestConnectionResponse.$_createMessage)
    ..aOM<RemoteServer>(1, _omitFieldNames ? '' : 'server',
        subBuilder: RemoteServer.$_createMessage)
    ..aE<RemoteServerAction>(2, _omitFieldNames ? '' : 'action',
        enumValues: RemoteServerAction.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerTestConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerTestConnectionResponse copyWith(
          void Function(RemoteServerTestConnectionResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerTestConnectionResponse))
          as RemoteServerTestConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerTestConnectionResponse() / RemoteServerTestConnectionResponse.new instead')
  static RemoteServerTestConnectionResponse create() =>
      RemoteServerTestConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerTestConnectionResponse._();
  @$core.override
  RemoteServerTestConnectionResponse createEmptyInstance() =>
      RemoteServerTestConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerTestConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerTestConnectionResponse>(
          RemoteServerTestConnectionResponse.$_createMessage);
  static RemoteServerTestConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServer get server => $_getN(0);
  @$pb.TagNumber(1)
  set server(RemoteServer value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearServer() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServer ensureServer() => $_ensure(0);

  @$pb.TagNumber(2)
  RemoteServerAction get action => $_getN(1);
  @$pb.TagNumber(2)
  set action(RemoteServerAction value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);
}

class RemoteServerDeleteRequest extends $pb.GeneratedMessage {
  factory RemoteServerDeleteRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoteServerDeleteRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoteServerDeleteRequest._();

  factory RemoteServerDeleteRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeleteRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerDeleteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerDeleteRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerDeleteRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerDeleteRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeleteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerDeleteRequest copyWith(
          void Function(RemoteServerDeleteRequest) updates) =>
      super.copyWith((message) => updates(message as RemoteServerDeleteRequest))
          as RemoteServerDeleteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerDeleteRequest() / RemoteServerDeleteRequest.new instead')
  static RemoteServerDeleteRequest create() => RemoteServerDeleteRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerDeleteRequest._();
  @$core.override
  RemoteServerDeleteRequest createEmptyInstance() =>
      RemoteServerDeleteRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerDeleteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerDeleteRequest>(
          RemoteServerDeleteRequest.$_createMessage);
  static RemoteServerDeleteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RemoteServerMigrateDependenciesRequest extends $pb.GeneratedMessage {
  factory RemoteServerMigrateDependenciesRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? targetServerId,
  }) {
    final result = RemoteServerMigrateDependenciesRequest._();
    if (id != null) result.id = id;
    if (targetServerId != null) result.targetServerId = targetServerId;
    return result;
  }

  RemoteServerMigrateDependenciesRequest._();

  factory RemoteServerMigrateDependenciesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateDependenciesRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerMigrateDependenciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateDependenciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerMigrateDependenciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          RemoteServerMigrateDependenciesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'targetServerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateDependenciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateDependenciesRequest copyWith(
          void Function(RemoteServerMigrateDependenciesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerMigrateDependenciesRequest))
          as RemoteServerMigrateDependenciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerMigrateDependenciesRequest() / RemoteServerMigrateDependenciesRequest.new instead')
  static RemoteServerMigrateDependenciesRequest create() =>
      RemoteServerMigrateDependenciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerMigrateDependenciesRequest._();
  @$core.override
  RemoteServerMigrateDependenciesRequest createEmptyInstance() =>
      RemoteServerMigrateDependenciesRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerMigrateDependenciesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerMigrateDependenciesRequest>(
          RemoteServerMigrateDependenciesRequest.$_createMessage);
  static RemoteServerMigrateDependenciesRequest? _defaultInstance;

  /// Source server id.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get targetServerId => $_getI64(1);
  @$pb.TagNumber(2)
  set targetServerId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetServerId() => $_clearField(2);
}

class RemoteServerMigrateDependenciesResponse extends $pb.GeneratedMessage {
  factory RemoteServerMigrateDependenciesResponse({
    RemoteServerDependencyMigration? migration,
  }) {
    final result = RemoteServerMigrateDependenciesResponse._();
    if (migration != null) result.migration = migration;
    return result;
  }

  RemoteServerMigrateDependenciesResponse._();

  factory RemoteServerMigrateDependenciesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateDependenciesResponse()
        ..mergeFromBuffer(data, registry);
  factory RemoteServerMigrateDependenciesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateDependenciesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerMigrateDependenciesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          RemoteServerMigrateDependenciesResponse.$_createMessage)
    ..aOM<RemoteServerDependencyMigration>(
        1, _omitFieldNames ? '' : 'migration',
        subBuilder: RemoteServerDependencyMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateDependenciesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateDependenciesResponse copyWith(
          void Function(RemoteServerMigrateDependenciesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerMigrateDependenciesResponse))
          as RemoteServerMigrateDependenciesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerMigrateDependenciesResponse() / RemoteServerMigrateDependenciesResponse.new instead')
  static RemoteServerMigrateDependenciesResponse create() =>
      RemoteServerMigrateDependenciesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerMigrateDependenciesResponse._();
  @$core.override
  RemoteServerMigrateDependenciesResponse createEmptyInstance() =>
      RemoteServerMigrateDependenciesResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerMigrateDependenciesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerMigrateDependenciesResponse>(
          RemoteServerMigrateDependenciesResponse.$_createMessage);
  static RemoteServerMigrateDependenciesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServerDependencyMigration get migration => $_getN(0);
  @$pb.TagNumber(1)
  set migration(RemoteServerDependencyMigration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMigration() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigration() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServerDependencyMigration ensureMigration() => $_ensure(0);
}

class RemoteServerGetMigrationRequest extends $pb.GeneratedMessage {
  factory RemoteServerGetMigrationRequest({
    $core.String? migrationId,
  }) {
    final result = RemoteServerGetMigrationRequest._();
    if (migrationId != null) result.migrationId = migrationId;
    return result;
  }

  RemoteServerGetMigrationRequest._();

  factory RemoteServerGetMigrationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetMigrationRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerGetMigrationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetMigrationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerGetMigrationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerGetMigrationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetMigrationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetMigrationRequest copyWith(
          void Function(RemoteServerGetMigrationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerGetMigrationRequest))
          as RemoteServerGetMigrationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerGetMigrationRequest() / RemoteServerGetMigrationRequest.new instead')
  static RemoteServerGetMigrationRequest create() =>
      RemoteServerGetMigrationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerGetMigrationRequest._();
  @$core.override
  RemoteServerGetMigrationRequest createEmptyInstance() =>
      RemoteServerGetMigrationRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerGetMigrationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerGetMigrationRequest>(
          RemoteServerGetMigrationRequest.$_createMessage);
  static RemoteServerGetMigrationRequest? _defaultInstance;

  /// Dependency-migration UUID as returned by MigrateDependencies.
  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);
}

class RemoteServerGetMigrationResponse extends $pb.GeneratedMessage {
  factory RemoteServerGetMigrationResponse({
    RemoteServerDependencyMigration? migration,
  }) {
    final result = RemoteServerGetMigrationResponse._();
    if (migration != null) result.migration = migration;
    return result;
  }

  RemoteServerGetMigrationResponse._();

  factory RemoteServerGetMigrationResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetMigrationResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerGetMigrationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerGetMigrationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerGetMigrationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerGetMigrationResponse.$_createMessage)
    ..aOM<RemoteServerDependencyMigration>(
        1, _omitFieldNames ? '' : 'migration',
        subBuilder: RemoteServerDependencyMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetMigrationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerGetMigrationResponse copyWith(
          void Function(RemoteServerGetMigrationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemoteServerGetMigrationResponse))
          as RemoteServerGetMigrationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerGetMigrationResponse() / RemoteServerGetMigrationResponse.new instead')
  static RemoteServerGetMigrationResponse create() =>
      RemoteServerGetMigrationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerGetMigrationResponse._();
  @$core.override
  RemoteServerGetMigrationResponse createEmptyInstance() =>
      RemoteServerGetMigrationResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerGetMigrationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerGetMigrationResponse>(
          RemoteServerGetMigrationResponse.$_createMessage);
  static RemoteServerGetMigrationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServerDependencyMigration get migration => $_getN(0);
  @$pb.TagNumber(1)
  set migration(RemoteServerDependencyMigration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMigration() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigration() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServerDependencyMigration ensureMigration() => $_ensure(0);
}

class RemoteServerRollbackMigrationRequest extends $pb.GeneratedMessage {
  factory RemoteServerRollbackMigrationRequest({
    $core.String? migrationId,
  }) {
    final result = RemoteServerRollbackMigrationRequest._();
    if (migrationId != null) result.migrationId = migrationId;
    return result;
  }

  RemoteServerRollbackMigrationRequest._();

  factory RemoteServerRollbackMigrationRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerRollbackMigrationRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerRollbackMigrationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerRollbackMigrationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerRollbackMigrationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerRollbackMigrationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerRollbackMigrationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerRollbackMigrationRequest copyWith(
          void Function(RemoteServerRollbackMigrationRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerRollbackMigrationRequest))
          as RemoteServerRollbackMigrationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerRollbackMigrationRequest() / RemoteServerRollbackMigrationRequest.new instead')
  static RemoteServerRollbackMigrationRequest create() =>
      RemoteServerRollbackMigrationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerRollbackMigrationRequest._();
  @$core.override
  RemoteServerRollbackMigrationRequest createEmptyInstance() =>
      RemoteServerRollbackMigrationRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerRollbackMigrationRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerRollbackMigrationRequest>(
          RemoteServerRollbackMigrationRequest.$_createMessage);
  static RemoteServerRollbackMigrationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);
}

class RemoteServerRollbackMigrationResponse extends $pb.GeneratedMessage {
  factory RemoteServerRollbackMigrationResponse({
    RemoteServerDependencyMigration? migration,
  }) {
    final result = RemoteServerRollbackMigrationResponse._();
    if (migration != null) result.migration = migration;
    return result;
  }

  RemoteServerRollbackMigrationResponse._();

  factory RemoteServerRollbackMigrationResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerRollbackMigrationResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerRollbackMigrationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerRollbackMigrationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerRollbackMigrationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          RemoteServerRollbackMigrationResponse.$_createMessage)
    ..aOM<RemoteServerDependencyMigration>(
        1, _omitFieldNames ? '' : 'migration',
        subBuilder: RemoteServerDependencyMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerRollbackMigrationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerRollbackMigrationResponse copyWith(
          void Function(RemoteServerRollbackMigrationResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerRollbackMigrationResponse))
          as RemoteServerRollbackMigrationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerRollbackMigrationResponse() / RemoteServerRollbackMigrationResponse.new instead')
  static RemoteServerRollbackMigrationResponse create() =>
      RemoteServerRollbackMigrationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerRollbackMigrationResponse._();
  @$core.override
  RemoteServerRollbackMigrationResponse createEmptyInstance() =>
      RemoteServerRollbackMigrationResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerRollbackMigrationResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerRollbackMigrationResponse>(
          RemoteServerRollbackMigrationResponse.$_createMessage);
  static RemoteServerRollbackMigrationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServerDependencyMigration get migration => $_getN(0);
  @$pb.TagNumber(1)
  set migration(RemoteServerDependencyMigration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMigration() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigration() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServerDependencyMigration ensureMigration() => $_ensure(0);
}

class RemoteServerCancelMigrationRequest extends $pb.GeneratedMessage {
  factory RemoteServerCancelMigrationRequest({
    $core.String? migrationId,
  }) {
    final result = RemoteServerCancelMigrationRequest._();
    if (migrationId != null) result.migrationId = migrationId;
    return result;
  }

  RemoteServerCancelMigrationRequest._();

  factory RemoteServerCancelMigrationRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCancelMigrationRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerCancelMigrationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCancelMigrationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerCancelMigrationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerCancelMigrationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCancelMigrationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCancelMigrationRequest copyWith(
          void Function(RemoteServerCancelMigrationRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerCancelMigrationRequest))
          as RemoteServerCancelMigrationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerCancelMigrationRequest() / RemoteServerCancelMigrationRequest.new instead')
  static RemoteServerCancelMigrationRequest create() =>
      RemoteServerCancelMigrationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerCancelMigrationRequest._();
  @$core.override
  RemoteServerCancelMigrationRequest createEmptyInstance() =>
      RemoteServerCancelMigrationRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerCancelMigrationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerCancelMigrationRequest>(
          RemoteServerCancelMigrationRequest.$_createMessage);
  static RemoteServerCancelMigrationRequest? _defaultInstance;

  /// Stateful application-migration UUID.
  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);
}

class RemoteServerCancelMigrationResponse extends $pb.GeneratedMessage {
  factory RemoteServerCancelMigrationResponse({
    RemoteServerApplicationMigration? migration,
  }) {
    final result = RemoteServerCancelMigrationResponse._();
    if (migration != null) result.migration = migration;
    return result;
  }

  RemoteServerCancelMigrationResponse._();

  factory RemoteServerCancelMigrationResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCancelMigrationResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerCancelMigrationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerCancelMigrationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerCancelMigrationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerCancelMigrationResponse.$_createMessage)
    ..aOM<RemoteServerApplicationMigration>(
        1, _omitFieldNames ? '' : 'migration',
        subBuilder: RemoteServerApplicationMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCancelMigrationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerCancelMigrationResponse copyWith(
          void Function(RemoteServerCancelMigrationResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerCancelMigrationResponse))
          as RemoteServerCancelMigrationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerCancelMigrationResponse() / RemoteServerCancelMigrationResponse.new instead')
  static RemoteServerCancelMigrationResponse create() =>
      RemoteServerCancelMigrationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerCancelMigrationResponse._();
  @$core.override
  RemoteServerCancelMigrationResponse createEmptyInstance() =>
      RemoteServerCancelMigrationResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerCancelMigrationResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerCancelMigrationResponse>(
          RemoteServerCancelMigrationResponse.$_createMessage);
  static RemoteServerCancelMigrationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServerApplicationMigration get migration => $_getN(0);
  @$pb.TagNumber(1)
  set migration(RemoteServerApplicationMigration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMigration() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigration() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServerApplicationMigration ensureMigration() => $_ensure(0);
}

class RemoteServerMigrateServiceRequest extends $pb.GeneratedMessage {
  factory RemoteServerMigrateServiceRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? targetServerId,
  }) {
    final result = RemoteServerMigrateServiceRequest._();
    if (id != null) result.id = id;
    if (targetServerId != null) result.targetServerId = targetServerId;
    return result;
  }

  RemoteServerMigrateServiceRequest._();

  factory RemoteServerMigrateServiceRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateServiceRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerMigrateServiceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateServiceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerMigrateServiceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerMigrateServiceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'targetServerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateServiceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateServiceRequest copyWith(
          void Function(RemoteServerMigrateServiceRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerMigrateServiceRequest))
          as RemoteServerMigrateServiceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerMigrateServiceRequest() / RemoteServerMigrateServiceRequest.new instead')
  static RemoteServerMigrateServiceRequest create() =>
      RemoteServerMigrateServiceRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerMigrateServiceRequest._();
  @$core.override
  RemoteServerMigrateServiceRequest createEmptyInstance() =>
      RemoteServerMigrateServiceRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerMigrateServiceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerMigrateServiceRequest>(
          RemoteServerMigrateServiceRequest.$_createMessage);
  static RemoteServerMigrateServiceRequest? _defaultInstance;

  /// Application/compose service id.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get targetServerId => $_getI64(1);
  @$pb.TagNumber(2)
  set targetServerId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetServerId() => $_clearField(2);
}

class RemoteServerMigrateServiceResponse extends $pb.GeneratedMessage {
  factory RemoteServerMigrateServiceResponse({
    RemoteServerApplicationMigration? migration,
  }) {
    final result = RemoteServerMigrateServiceResponse._();
    if (migration != null) result.migration = migration;
    return result;
  }

  RemoteServerMigrateServiceResponse._();

  factory RemoteServerMigrateServiceResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateServiceResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerMigrateServiceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerMigrateServiceResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerMigrateServiceResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerMigrateServiceResponse.$_createMessage)
    ..aOM<RemoteServerApplicationMigration>(
        1, _omitFieldNames ? '' : 'migration',
        subBuilder: RemoteServerApplicationMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateServiceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerMigrateServiceResponse copyWith(
          void Function(RemoteServerMigrateServiceResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerMigrateServiceResponse))
          as RemoteServerMigrateServiceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerMigrateServiceResponse() / RemoteServerMigrateServiceResponse.new instead')
  static RemoteServerMigrateServiceResponse create() =>
      RemoteServerMigrateServiceResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerMigrateServiceResponse._();
  @$core.override
  RemoteServerMigrateServiceResponse createEmptyInstance() =>
      RemoteServerMigrateServiceResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerMigrateServiceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerMigrateServiceResponse>(
          RemoteServerMigrateServiceResponse.$_createMessage);
  static RemoteServerMigrateServiceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  RemoteServerApplicationMigration get migration => $_getN(0);
  @$pb.TagNumber(1)
  set migration(RemoteServerApplicationMigration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMigration() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigration() => $_clearField(1);
  @$pb.TagNumber(1)
  RemoteServerApplicationMigration ensureMigration() => $_ensure(0);
}

class RemoteServerListMigrationsRequest extends $pb.GeneratedMessage {
  factory RemoteServerListMigrationsRequest({
    $fixnum.Int64? serviceId,
  }) {
    final result = RemoteServerListMigrationsRequest._();
    if (serviceId != null) result.serviceId = serviceId;
    return result;
  }

  RemoteServerListMigrationsRequest._();

  factory RemoteServerListMigrationsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationsRequest()..mergeFromBuffer(data, registry);
  factory RemoteServerListMigrationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListMigrationsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerListMigrationsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serviceId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationsRequest copyWith(
          void Function(RemoteServerListMigrationsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerListMigrationsRequest))
          as RemoteServerListMigrationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListMigrationsRequest() / RemoteServerListMigrationsRequest.new instead')
  static RemoteServerListMigrationsRequest create() =>
      RemoteServerListMigrationsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerListMigrationsRequest._();
  @$core.override
  RemoteServerListMigrationsRequest createEmptyInstance() =>
      RemoteServerListMigrationsRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListMigrationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerListMigrationsRequest>(
          RemoteServerListMigrationsRequest.$_createMessage);
  static RemoteServerListMigrationsRequest? _defaultInstance;

  /// Application/compose service id.
  @$pb.TagNumber(1)
  $fixnum.Int64 get serviceId => $_getI64(0);
  @$pb.TagNumber(1)
  set serviceId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServiceId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServiceId() => $_clearField(1);
}

class RemoteServerListMigrationsResponse extends $pb.GeneratedMessage {
  factory RemoteServerListMigrationsResponse({
    $core.Iterable<RemoteServerApplicationMigration>? migrations,
  }) {
    final result = RemoteServerListMigrationsResponse._();
    if (migrations != null) result.migrations.addAll(migrations);
    return result;
  }

  RemoteServerListMigrationsResponse._();

  factory RemoteServerListMigrationsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationsResponse()..mergeFromBuffer(data, registry);
  factory RemoteServerListMigrationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListMigrationsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoteServerListMigrationsResponse.$_createMessage)
    ..pPM<RemoteServerApplicationMigration>(
        1, _omitFieldNames ? '' : 'migrations',
        subBuilder: RemoteServerApplicationMigration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationsResponse copyWith(
          void Function(RemoteServerListMigrationsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerListMigrationsResponse))
          as RemoteServerListMigrationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListMigrationsResponse() / RemoteServerListMigrationsResponse.new instead')
  static RemoteServerListMigrationsResponse create() =>
      RemoteServerListMigrationsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerListMigrationsResponse._();
  @$core.override
  RemoteServerListMigrationsResponse createEmptyInstance() =>
      RemoteServerListMigrationsResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListMigrationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoteServerListMigrationsResponse>(
          RemoteServerListMigrationsResponse.$_createMessage);
  static RemoteServerListMigrationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RemoteServerApplicationMigration> get migrations => $_getList(0);
}

class RemoteServerListMigrationObjectsRequest extends $pb.GeneratedMessage {
  factory RemoteServerListMigrationObjectsRequest({
    $core.String? migrationId,
  }) {
    final result = RemoteServerListMigrationObjectsRequest._();
    if (migrationId != null) result.migrationId = migrationId;
    return result;
  }

  RemoteServerListMigrationObjectsRequest._();

  factory RemoteServerListMigrationObjectsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationObjectsRequest()
        ..mergeFromBuffer(data, registry);
  factory RemoteServerListMigrationObjectsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationObjectsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListMigrationObjectsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          RemoteServerListMigrationObjectsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'migrationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationObjectsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationObjectsRequest copyWith(
          void Function(RemoteServerListMigrationObjectsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerListMigrationObjectsRequest))
          as RemoteServerListMigrationObjectsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListMigrationObjectsRequest() / RemoteServerListMigrationObjectsRequest.new instead')
  static RemoteServerListMigrationObjectsRequest create() =>
      RemoteServerListMigrationObjectsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerListMigrationObjectsRequest._();
  @$core.override
  RemoteServerListMigrationObjectsRequest createEmptyInstance() =>
      RemoteServerListMigrationObjectsRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListMigrationObjectsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerListMigrationObjectsRequest>(
          RemoteServerListMigrationObjectsRequest.$_createMessage);
  static RemoteServerListMigrationObjectsRequest? _defaultInstance;

  /// Stateful application-migration UUID.
  @$pb.TagNumber(1)
  $core.String get migrationId => $_getSZ(0);
  @$pb.TagNumber(1)
  set migrationId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMigrationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMigrationId() => $_clearField(1);
}

class RemoteServerListMigrationObjectsResponse extends $pb.GeneratedMessage {
  factory RemoteServerListMigrationObjectsResponse({
    $core.Iterable<RemoteServerMigrationObject>? objects,
  }) {
    final result = RemoteServerListMigrationObjectsResponse._();
    if (objects != null) result.objects.addAll(objects);
    return result;
  }

  RemoteServerListMigrationObjectsResponse._();

  factory RemoteServerListMigrationObjectsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationObjectsResponse()
        ..mergeFromBuffer(data, registry);
  factory RemoteServerListMigrationObjectsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoteServerListMigrationObjectsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoteServerListMigrationObjectsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          RemoteServerListMigrationObjectsResponse.$_createMessage)
    ..pPM<RemoteServerMigrationObject>(1, _omitFieldNames ? '' : 'objects',
        subBuilder: RemoteServerMigrationObject.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationObjectsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoteServerListMigrationObjectsResponse copyWith(
          void Function(RemoteServerListMigrationObjectsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoteServerListMigrationObjectsResponse))
          as RemoteServerListMigrationObjectsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RemoteServerListMigrationObjectsResponse() / RemoteServerListMigrationObjectsResponse.new instead')
  static RemoteServerListMigrationObjectsResponse create() =>
      RemoteServerListMigrationObjectsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RemoteServerListMigrationObjectsResponse._();
  @$core.override
  RemoteServerListMigrationObjectsResponse createEmptyInstance() =>
      RemoteServerListMigrationObjectsResponse._();
  @$core.pragma('dart2js:noInline')
  static RemoteServerListMigrationObjectsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              RemoteServerListMigrationObjectsResponse>(
          RemoteServerListMigrationObjectsResponse.$_createMessage);
  static RemoteServerListMigrationObjectsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RemoteServerMigrationObject> get objects => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

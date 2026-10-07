// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_controller.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'server_controller.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'server_controller.pbenum.dart';

class ServerToolState extends $pb.GeneratedMessage {
  factory ServerToolState({
    $core.bool? installed,
    $core.String? version,
  }) {
    final result = ServerToolState._();
    if (installed != null) result.installed = installed;
    if (version != null) result.version = version;
    return result;
  }

  ServerToolState._();

  factory ServerToolState.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerToolState()..mergeFromBuffer(data, registry);
  factory ServerToolState.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerToolState()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerToolState',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerToolState.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'installed')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerToolState clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerToolState copyWith(void Function(ServerToolState) updates) =>
      super.copyWith((message) => updates(message as ServerToolState))
          as ServerToolState;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerToolState() / ServerToolState.new instead')
  static ServerToolState create() => ServerToolState._();
  static $pb.GeneratedMessage $_createMessage() => ServerToolState._();
  @$core.override
  ServerToolState createEmptyInstance() => ServerToolState._();
  @$core.pragma('dart2js:noInline')
  static ServerToolState getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerToolState>(
          ServerToolState.$_createMessage);
  static ServerToolState? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get installed => $_getBF(0);
  @$pb.TagNumber(1)
  set installed($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasInstalled() => $_has(0);
  @$pb.TagNumber(1)
  void clearInstalled() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get version => $_getSZ(1);
  @$pb.TagNumber(2)
  set version($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);
}

class ServerPortAvailability extends $pb.GeneratedMessage {
  factory ServerPortAvailability({
    $core.int? port,
    $core.bool? available,
  }) {
    final result = ServerPortAvailability._();
    if (port != null) result.port = port;
    if (available != null) result.available = available;
    return result;
  }

  ServerPortAvailability._();

  factory ServerPortAvailability.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPortAvailability()..mergeFromBuffer(data, registry);
  factory ServerPortAvailability.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPortAvailability()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPortAvailability',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPortAvailability.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'port', fieldType: $pb.PbFieldType.OU3)
    ..aOB(2, _omitFieldNames ? '' : 'available')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPortAvailability clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPortAvailability copyWith(
          void Function(ServerPortAvailability) updates) =>
      super.copyWith((message) => updates(message as ServerPortAvailability))
          as ServerPortAvailability;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPortAvailability() / ServerPortAvailability.new instead')
  static ServerPortAvailability create() => ServerPortAvailability._();
  static $pb.GeneratedMessage $_createMessage() => ServerPortAvailability._();
  @$core.override
  ServerPortAvailability createEmptyInstance() => ServerPortAvailability._();
  @$core.pragma('dart2js:noInline')
  static ServerPortAvailability getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPortAvailability>(
          ServerPortAvailability.$_createMessage);
  static ServerPortAvailability? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get port => $_getIZ(0);
  @$pb.TagNumber(1)
  set port($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPort() => $_has(0);
  @$pb.TagNumber(1)
  void clearPort() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get available => $_getBF(1);
  @$pb.TagNumber(2)
  set available($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAvailable() => $_has(1);
  @$pb.TagNumber(2)
  void clearAvailable() => $_clearField(2);
}

class ServerAudit extends $pb.GeneratedMessage {
  factory ServerAudit({
    $core.String? osId,
    $core.String? architecture,
    ServerToolState? docker,
    ServerToolState? git,
    ServerToolState? rclone,
    ServerToolState? rsync,
    ServerToolState? nixpacks,
    ServerToolState? railpack,
    ServerToolState? buildpacks,
    $core.bool? swarmActive,
    $core.bool? networkExists,
    $core.bool? baseDirectoryExists,
    $core.bool? dockerGroupMember,
    $core.Iterable<ServerPortAvailability>? ports,
  }) {
    final result = ServerAudit._();
    if (osId != null) result.osId = osId;
    if (architecture != null) result.architecture = architecture;
    if (docker != null) result.docker = docker;
    if (git != null) result.git = git;
    if (rclone != null) result.rclone = rclone;
    if (rsync != null) result.rsync = rsync;
    if (nixpacks != null) result.nixpacks = nixpacks;
    if (railpack != null) result.railpack = railpack;
    if (buildpacks != null) result.buildpacks = buildpacks;
    if (swarmActive != null) result.swarmActive = swarmActive;
    if (networkExists != null) result.networkExists = networkExists;
    if (baseDirectoryExists != null)
      result.baseDirectoryExists = baseDirectoryExists;
    if (dockerGroupMember != null) result.dockerGroupMember = dockerGroupMember;
    if (ports != null) result.ports.addAll(ports);
    return result;
  }

  ServerAudit._();

  factory ServerAudit.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerAudit()..mergeFromBuffer(data, registry);
  factory ServerAudit.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerAudit()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerAudit',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerAudit.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'osId')
    ..aOS(2, _omitFieldNames ? '' : 'architecture')
    ..aOM<ServerToolState>(3, _omitFieldNames ? '' : 'docker',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(4, _omitFieldNames ? '' : 'git',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(5, _omitFieldNames ? '' : 'rclone',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(6, _omitFieldNames ? '' : 'rsync',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(7, _omitFieldNames ? '' : 'nixpacks',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(8, _omitFieldNames ? '' : 'railpack',
        subBuilder: ServerToolState.$_createMessage)
    ..aOM<ServerToolState>(9, _omitFieldNames ? '' : 'buildpacks',
        subBuilder: ServerToolState.$_createMessage)
    ..aOB(10, _omitFieldNames ? '' : 'swarmActive')
    ..aOB(11, _omitFieldNames ? '' : 'networkExists')
    ..aOB(12, _omitFieldNames ? '' : 'baseDirectoryExists')
    ..aOB(13, _omitFieldNames ? '' : 'dockerGroupMember')
    ..pPM<ServerPortAvailability>(14, _omitFieldNames ? '' : 'ports',
        subBuilder: ServerPortAvailability.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerAudit clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerAudit copyWith(void Function(ServerAudit) updates) =>
      super.copyWith((message) => updates(message as ServerAudit))
          as ServerAudit;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerAudit() / ServerAudit.new instead')
  static ServerAudit create() => ServerAudit._();
  static $pb.GeneratedMessage $_createMessage() => ServerAudit._();
  @$core.override
  ServerAudit createEmptyInstance() => ServerAudit._();
  @$core.pragma('dart2js:noInline')
  static ServerAudit getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerAudit>(
          ServerAudit.$_createMessage);
  static ServerAudit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get osId => $_getSZ(0);
  @$pb.TagNumber(1)
  set osId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOsId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOsId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get architecture => $_getSZ(1);
  @$pb.TagNumber(2)
  set architecture($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasArchitecture() => $_has(1);
  @$pb.TagNumber(2)
  void clearArchitecture() => $_clearField(2);

  @$pb.TagNumber(3)
  ServerToolState get docker => $_getN(2);
  @$pb.TagNumber(3)
  set docker(ServerToolState value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasDocker() => $_has(2);
  @$pb.TagNumber(3)
  void clearDocker() => $_clearField(3);
  @$pb.TagNumber(3)
  ServerToolState ensureDocker() => $_ensure(2);

  @$pb.TagNumber(4)
  ServerToolState get git => $_getN(3);
  @$pb.TagNumber(4)
  set git(ServerToolState value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasGit() => $_has(3);
  @$pb.TagNumber(4)
  void clearGit() => $_clearField(4);
  @$pb.TagNumber(4)
  ServerToolState ensureGit() => $_ensure(3);

  @$pb.TagNumber(5)
  ServerToolState get rclone => $_getN(4);
  @$pb.TagNumber(5)
  set rclone(ServerToolState value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasRclone() => $_has(4);
  @$pb.TagNumber(5)
  void clearRclone() => $_clearField(5);
  @$pb.TagNumber(5)
  ServerToolState ensureRclone() => $_ensure(4);

  @$pb.TagNumber(6)
  ServerToolState get rsync => $_getN(5);
  @$pb.TagNumber(6)
  set rsync(ServerToolState value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasRsync() => $_has(5);
  @$pb.TagNumber(6)
  void clearRsync() => $_clearField(6);
  @$pb.TagNumber(6)
  ServerToolState ensureRsync() => $_ensure(5);

  @$pb.TagNumber(7)
  ServerToolState get nixpacks => $_getN(6);
  @$pb.TagNumber(7)
  set nixpacks(ServerToolState value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasNixpacks() => $_has(6);
  @$pb.TagNumber(7)
  void clearNixpacks() => $_clearField(7);
  @$pb.TagNumber(7)
  ServerToolState ensureNixpacks() => $_ensure(6);

  @$pb.TagNumber(8)
  ServerToolState get railpack => $_getN(7);
  @$pb.TagNumber(8)
  set railpack(ServerToolState value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasRailpack() => $_has(7);
  @$pb.TagNumber(8)
  void clearRailpack() => $_clearField(8);
  @$pb.TagNumber(8)
  ServerToolState ensureRailpack() => $_ensure(7);

  @$pb.TagNumber(9)
  ServerToolState get buildpacks => $_getN(8);
  @$pb.TagNumber(9)
  set buildpacks(ServerToolState value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasBuildpacks() => $_has(8);
  @$pb.TagNumber(9)
  void clearBuildpacks() => $_clearField(9);
  @$pb.TagNumber(9)
  ServerToolState ensureBuildpacks() => $_ensure(8);

  @$pb.TagNumber(10)
  $core.bool get swarmActive => $_getBF(9);
  @$pb.TagNumber(10)
  set swarmActive($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasSwarmActive() => $_has(9);
  @$pb.TagNumber(10)
  void clearSwarmActive() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get networkExists => $_getBF(10);
  @$pb.TagNumber(11)
  set networkExists($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasNetworkExists() => $_has(10);
  @$pb.TagNumber(11)
  void clearNetworkExists() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get baseDirectoryExists => $_getBF(11);
  @$pb.TagNumber(12)
  set baseDirectoryExists($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasBaseDirectoryExists() => $_has(11);
  @$pb.TagNumber(12)
  void clearBaseDirectoryExists() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get dockerGroupMember => $_getBF(12);
  @$pb.TagNumber(13)
  set dockerGroupMember($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasDockerGroupMember() => $_has(12);
  @$pb.TagNumber(13)
  void clearDockerGroupMember() => $_clearField(13);

  @$pb.TagNumber(14)
  $pb.PbList<ServerPortAvailability> get ports => $_getList(13);
}

class ServerSetupOutcome extends $pb.GeneratedMessage {
  factory ServerSetupOutcome({
    $core.Iterable<ServerSetupStep>? completed,
    ServerAudit? audit,
  }) {
    final result = ServerSetupOutcome._();
    if (completed != null) result.completed.addAll(completed);
    if (audit != null) result.audit = audit;
    return result;
  }

  ServerSetupOutcome._();

  factory ServerSetupOutcome.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupOutcome()..mergeFromBuffer(data, registry);
  factory ServerSetupOutcome.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerSetupOutcome()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerSetupOutcome',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerSetupOutcome.$_createMessage)
    ..pc<ServerSetupStep>(
        1, _omitFieldNames ? '' : 'completed', $pb.PbFieldType.KE,
        valueOf: ServerSetupStep.valueOf,
        enumValues: ServerSetupStep.values,
        defaultEnumValue: ServerSetupStep.SERVER_SETUP_STEP_UNSPECIFIED)
    ..aOM<ServerAudit>(2, _omitFieldNames ? '' : 'audit',
        subBuilder: ServerAudit.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupOutcome clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerSetupOutcome copyWith(void Function(ServerSetupOutcome) updates) =>
      super.copyWith((message) => updates(message as ServerSetupOutcome))
          as ServerSetupOutcome;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerSetupOutcome() / ServerSetupOutcome.new instead')
  static ServerSetupOutcome create() => ServerSetupOutcome._();
  static $pb.GeneratedMessage $_createMessage() => ServerSetupOutcome._();
  @$core.override
  ServerSetupOutcome createEmptyInstance() => ServerSetupOutcome._();
  @$core.pragma('dart2js:noInline')
  static ServerSetupOutcome getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerSetupOutcome>(
          ServerSetupOutcome.$_createMessage);
  static ServerSetupOutcome? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ServerSetupStep> get completed => $_getList(0);

  @$pb.TagNumber(2)
  ServerAudit get audit => $_getN(1);
  @$pb.TagNumber(2)
  set audit(ServerAudit value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAudit() => $_has(1);
  @$pb.TagNumber(2)
  void clearAudit() => $_clearField(2);
  @$pb.TagNumber(2)
  ServerAudit ensureAudit() => $_ensure(1);
}

class ServerConnectionInfo extends $pb.GeneratedMessage {
  factory ServerConnectionInfo({
    $core.bool? connected,
    $fixnum.Int64? reusedSessions,
    $fixnum.Int64? maxPoolSize,
    $fixnum.Int64? connections,
    $fixnum.Int64? activeChannels,
    $fixnum.Int64? maxChannelsPerSession,
  }) {
    final result = ServerConnectionInfo._();
    if (connected != null) result.connected = connected;
    if (reusedSessions != null) result.reusedSessions = reusedSessions;
    if (maxPoolSize != null) result.maxPoolSize = maxPoolSize;
    if (connections != null) result.connections = connections;
    if (activeChannels != null) result.activeChannels = activeChannels;
    if (maxChannelsPerSession != null)
      result.maxChannelsPerSession = maxChannelsPerSession;
    return result;
  }

  ServerConnectionInfo._();

  factory ServerConnectionInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerConnectionInfo()..mergeFromBuffer(data, registry);
  factory ServerConnectionInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerConnectionInfo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerConnectionInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerConnectionInfo.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'connected')
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'reusedSessions', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        3, _omitFieldNames ? '' : 'maxPoolSize', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        4, _omitFieldNames ? '' : 'connections', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        5, _omitFieldNames ? '' : 'activeChannels', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        6, _omitFieldNames ? '' : 'maxChannelsPerSession', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerConnectionInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerConnectionInfo copyWith(void Function(ServerConnectionInfo) updates) =>
      super.copyWith((message) => updates(message as ServerConnectionInfo))
          as ServerConnectionInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerConnectionInfo() / ServerConnectionInfo.new instead')
  static ServerConnectionInfo create() => ServerConnectionInfo._();
  static $pb.GeneratedMessage $_createMessage() => ServerConnectionInfo._();
  @$core.override
  ServerConnectionInfo createEmptyInstance() => ServerConnectionInfo._();
  @$core.pragma('dart2js:noInline')
  static ServerConnectionInfo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerConnectionInfo>(
          ServerConnectionInfo.$_createMessage);
  static ServerConnectionInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get connected => $_getBF(0);
  @$pb.TagNumber(1)
  set connected($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConnected() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnected() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get reusedSessions => $_getI64(1);
  @$pb.TagNumber(2)
  set reusedSessions($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReusedSessions() => $_has(1);
  @$pb.TagNumber(2)
  void clearReusedSessions() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get maxPoolSize => $_getI64(2);
  @$pb.TagNumber(3)
  set maxPoolSize($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMaxPoolSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearMaxPoolSize() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get connections => $_getI64(3);
  @$pb.TagNumber(4)
  set connections($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasConnections() => $_has(3);
  @$pb.TagNumber(4)
  void clearConnections() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get activeChannels => $_getI64(4);
  @$pb.TagNumber(5)
  set activeChannels($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasActiveChannels() => $_has(4);
  @$pb.TagNumber(5)
  void clearActiveChannels() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get maxChannelsPerSession => $_getI64(5);
  @$pb.TagNumber(6)
  set maxChannelsPerSession($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasMaxChannelsPerSession() => $_has(5);
  @$pb.TagNumber(6)
  void clearMaxChannelsPerSession() => $_clearField(6);
}

/// Probes an unsaved endpoint before a server row exists.
class ServerControllerTestDirectConnectionRequest extends $pb.GeneratedMessage {
  factory ServerControllerTestDirectConnectionRequest({
    $core.String? ipAddress,
    $core.int? port,
    $core.String? username,
    $fixnum.Int64? sshKeyId,
  }) {
    final result = ServerControllerTestDirectConnectionRequest._();
    if (ipAddress != null) result.ipAddress = ipAddress;
    if (port != null) result.port = port;
    if (username != null) result.username = username;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    return result;
  }

  ServerControllerTestDirectConnectionRequest._();

  factory ServerControllerTestDirectConnectionRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestDirectConnectionRequest()
        ..mergeFromBuffer(data, registry);
  factory ServerControllerTestDirectConnectionRequest.fromJson(
          $core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestDirectConnectionRequest()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerTestDirectConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerTestDirectConnectionRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'ipAddress')
    ..aI(2, _omitFieldNames ? '' : 'port', fieldType: $pb.PbFieldType.OU3)
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aInt64(4, _omitFieldNames ? '' : 'sshKeyId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestDirectConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestDirectConnectionRequest copyWith(
          void Function(ServerControllerTestDirectConnectionRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerTestDirectConnectionRequest))
          as ServerControllerTestDirectConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerTestDirectConnectionRequest() / ServerControllerTestDirectConnectionRequest.new instead')
  static ServerControllerTestDirectConnectionRequest create() =>
      ServerControllerTestDirectConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerTestDirectConnectionRequest._();
  @$core.override
  ServerControllerTestDirectConnectionRequest createEmptyInstance() =>
      ServerControllerTestDirectConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerTestDirectConnectionRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerTestDirectConnectionRequest>(
          ServerControllerTestDirectConnectionRequest.$_createMessage);
  static ServerControllerTestDirectConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get ipAddress => $_getSZ(0);
  @$pb.TagNumber(1)
  set ipAddress($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIpAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearIpAddress() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get port => $_getIZ(1);
  @$pb.TagNumber(2)
  set port($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get sshKeyId => $_getI64(3);
  @$pb.TagNumber(4)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSshKeyId() => $_has(3);
  @$pb.TagNumber(4)
  void clearSshKeyId() => $_clearField(4);
}

class ServerControllerTestDirectConnectionResponse
    extends $pb.GeneratedMessage {
  factory ServerControllerTestDirectConnectionResponse({
    ServerConnectionInfo? connection,
  }) {
    final result = ServerControllerTestDirectConnectionResponse._();
    if (connection != null) result.connection = connection;
    return result;
  }

  ServerControllerTestDirectConnectionResponse._();

  factory ServerControllerTestDirectConnectionResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestDirectConnectionResponse()
        ..mergeFromBuffer(data, registry);
  factory ServerControllerTestDirectConnectionResponse.fromJson(
          $core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestDirectConnectionResponse()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerTestDirectConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerTestDirectConnectionResponse.$_createMessage)
    ..aOM<ServerConnectionInfo>(1, _omitFieldNames ? '' : 'connection',
        subBuilder: ServerConnectionInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestDirectConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestDirectConnectionResponse copyWith(
          void Function(ServerControllerTestDirectConnectionResponse)
              updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerTestDirectConnectionResponse))
          as ServerControllerTestDirectConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerTestDirectConnectionResponse() / ServerControllerTestDirectConnectionResponse.new instead')
  static ServerControllerTestDirectConnectionResponse create() =>
      ServerControllerTestDirectConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerTestDirectConnectionResponse._();
  @$core.override
  ServerControllerTestDirectConnectionResponse createEmptyInstance() =>
      ServerControllerTestDirectConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerTestDirectConnectionResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerTestDirectConnectionResponse>(
          ServerControllerTestDirectConnectionResponse.$_createMessage);
  static ServerControllerTestDirectConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerConnectionInfo get connection => $_getN(0);
  @$pb.TagNumber(1)
  set connection(ServerConnectionInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasConnection() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnection() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerConnectionInfo ensureConnection() => $_ensure(0);
}

class ServerControllerTestConnectionRequest extends $pb.GeneratedMessage {
  factory ServerControllerTestConnectionRequest({
    $fixnum.Int64? id,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerControllerTestConnectionRequest._();
    if (id != null) result.id = id;
    if (hostKeyFingerprint != null)
      result.hostKeyFingerprint = hostKeyFingerprint;
    if (sudoPassword != null) result.sudoPassword = sudoPassword;
    if (poolSize != null) result.poolSize = poolSize;
    return result;
  }

  ServerControllerTestConnectionRequest._();

  factory ServerControllerTestConnectionRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestConnectionRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerTestConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerTestConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerTestConnectionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(3, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(4, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestConnectionRequest copyWith(
          void Function(ServerControllerTestConnectionRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerTestConnectionRequest))
          as ServerControllerTestConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerTestConnectionRequest() / ServerControllerTestConnectionRequest.new instead')
  static ServerControllerTestConnectionRequest create() =>
      ServerControllerTestConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerTestConnectionRequest._();
  @$core.override
  ServerControllerTestConnectionRequest createEmptyInstance() =>
      ServerControllerTestConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerTestConnectionRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerTestConnectionRequest>(
          ServerControllerTestConnectionRequest.$_createMessage);
  static ServerControllerTestConnectionRequest? _defaultInstance;

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

  /// Sudo credential document; only sent for privileged setup steps.
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

class ServerControllerTestConnectionResponse extends $pb.GeneratedMessage {
  factory ServerControllerTestConnectionResponse({
    ServerConnectionInfo? connection,
  }) {
    final result = ServerControllerTestConnectionResponse._();
    if (connection != null) result.connection = connection;
    return result;
  }

  ServerControllerTestConnectionResponse._();

  factory ServerControllerTestConnectionResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestConnectionResponse()..mergeFromBuffer(data, registry);
  factory ServerControllerTestConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerTestConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerTestConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerTestConnectionResponse.$_createMessage)
    ..aOM<ServerConnectionInfo>(1, _omitFieldNames ? '' : 'connection',
        subBuilder: ServerConnectionInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerTestConnectionResponse copyWith(
          void Function(ServerControllerTestConnectionResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerTestConnectionResponse))
          as ServerControllerTestConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerTestConnectionResponse() / ServerControllerTestConnectionResponse.new instead')
  static ServerControllerTestConnectionResponse create() =>
      ServerControllerTestConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerTestConnectionResponse._();
  @$core.override
  ServerControllerTestConnectionResponse createEmptyInstance() =>
      ServerControllerTestConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerTestConnectionResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerTestConnectionResponse>(
          ServerControllerTestConnectionResponse.$_createMessage);
  static ServerControllerTestConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerConnectionInfo get connection => $_getN(0);
  @$pb.TagNumber(1)
  set connection(ServerConnectionInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasConnection() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnection() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerConnectionInfo ensureConnection() => $_ensure(0);
}

class ServerControllerAuditRequest extends $pb.GeneratedMessage {
  factory ServerControllerAuditRequest({
    $fixnum.Int64? id,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerControllerAuditRequest._();
    if (id != null) result.id = id;
    if (hostKeyFingerprint != null)
      result.hostKeyFingerprint = hostKeyFingerprint;
    if (sudoPassword != null) result.sudoPassword = sudoPassword;
    if (poolSize != null) result.poolSize = poolSize;
    return result;
  }

  ServerControllerAuditRequest._();

  factory ServerControllerAuditRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerAuditRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerAuditRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerAuditRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerAuditRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerAuditRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(3, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(4, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerAuditRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerAuditRequest copyWith(
          void Function(ServerControllerAuditRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerControllerAuditRequest))
          as ServerControllerAuditRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerAuditRequest() / ServerControllerAuditRequest.new instead')
  static ServerControllerAuditRequest create() =>
      ServerControllerAuditRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerAuditRequest._();
  @$core.override
  ServerControllerAuditRequest createEmptyInstance() =>
      ServerControllerAuditRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerAuditRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerControllerAuditRequest>(
          ServerControllerAuditRequest.$_createMessage);
  static ServerControllerAuditRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get hostKeyFingerprint => $_getSZ(1);
  @$pb.TagNumber(2)
  set hostKeyFingerprint($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHostKeyFingerprint() => $_has(1);
  @$pb.TagNumber(2)
  void clearHostKeyFingerprint() => $_clearField(2);

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

class ServerControllerAuditResponse extends $pb.GeneratedMessage {
  factory ServerControllerAuditResponse({
    ServerAudit? audit,
  }) {
    final result = ServerControllerAuditResponse._();
    if (audit != null) result.audit = audit;
    return result;
  }

  ServerControllerAuditResponse._();

  factory ServerControllerAuditResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerAuditResponse()..mergeFromBuffer(data, registry);
  factory ServerControllerAuditResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerAuditResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerAuditResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerAuditResponse.$_createMessage)
    ..aOM<ServerAudit>(1, _omitFieldNames ? '' : 'audit',
        subBuilder: ServerAudit.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerAuditResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerAuditResponse copyWith(
          void Function(ServerControllerAuditResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerControllerAuditResponse))
          as ServerControllerAuditResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerAuditResponse() / ServerControllerAuditResponse.new instead')
  static ServerControllerAuditResponse create() =>
      ServerControllerAuditResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerAuditResponse._();
  @$core.override
  ServerControllerAuditResponse createEmptyInstance() =>
      ServerControllerAuditResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerAuditResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerControllerAuditResponse>(
          ServerControllerAuditResponse.$_createMessage);
  static ServerControllerAuditResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerAudit get audit => $_getN(0);
  @$pb.TagNumber(1)
  set audit(ServerAudit value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAudit() => $_has(0);
  @$pb.TagNumber(1)
  void clearAudit() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerAudit ensureAudit() => $_ensure(0);
}

class ServerControllerSetupRequest extends $pb.GeneratedMessage {
  factory ServerControllerSetupRequest({
    $fixnum.Int64? id,
    $core.bool? installDependencies,
    $core.String? advertiseAddr,
    $core.String? acmeEmail,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerControllerSetupRequest._();
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

  ServerControllerSetupRequest._();

  factory ServerControllerSetupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerSetupRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerSetupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerSetupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerSetupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerSetupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'installDependencies')
    ..aOS(3, _omitFieldNames ? '' : 'advertiseAddr')
    ..aOS(4, _omitFieldNames ? '' : 'acmeEmail')
    ..aOS(5, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(6, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(7, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerSetupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerSetupRequest copyWith(
          void Function(ServerControllerSetupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerControllerSetupRequest))
          as ServerControllerSetupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerSetupRequest() / ServerControllerSetupRequest.new instead')
  static ServerControllerSetupRequest create() =>
      ServerControllerSetupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerSetupRequest._();
  @$core.override
  ServerControllerSetupRequest createEmptyInstance() =>
      ServerControllerSetupRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerSetupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerControllerSetupRequest>(
          ServerControllerSetupRequest.$_createMessage);
  static ServerControllerSetupRequest? _defaultInstance;

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

class ServerControllerSetupResponse extends $pb.GeneratedMessage {
  factory ServerControllerSetupResponse({
    ServerSetupOutcome? outcome,
  }) {
    final result = ServerControllerSetupResponse._();
    if (outcome != null) result.outcome = outcome;
    return result;
  }

  ServerControllerSetupResponse._();

  factory ServerControllerSetupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerSetupResponse()..mergeFromBuffer(data, registry);
  factory ServerControllerSetupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerSetupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerSetupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerSetupResponse.$_createMessage)
    ..aOM<ServerSetupOutcome>(1, _omitFieldNames ? '' : 'outcome',
        subBuilder: ServerSetupOutcome.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerSetupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerSetupResponse copyWith(
          void Function(ServerControllerSetupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerControllerSetupResponse))
          as ServerControllerSetupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerSetupResponse() / ServerControllerSetupResponse.new instead')
  static ServerControllerSetupResponse create() =>
      ServerControllerSetupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerSetupResponse._();
  @$core.override
  ServerControllerSetupResponse createEmptyInstance() =>
      ServerControllerSetupResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerSetupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerControllerSetupResponse>(
          ServerControllerSetupResponse.$_createMessage);
  static ServerControllerSetupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerSetupOutcome get outcome => $_getN(0);
  @$pb.TagNumber(1)
  set outcome(ServerSetupOutcome value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasOutcome() => $_has(0);
  @$pb.TagNumber(1)
  void clearOutcome() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerSetupOutcome ensureOutcome() => $_ensure(0);
}

class ServerControllerStreamSetupLogsRequest extends $pb.GeneratedMessage {
  factory ServerControllerStreamSetupLogsRequest({
    $fixnum.Int64? id,
    $core.bool? installDependencies,
    $core.String? advertiseAddr,
    $core.String? acmeEmail,
    $core.String? hostKeyFingerprint,
    $core.String? sudoPassword,
    $core.int? poolSize,
  }) {
    final result = ServerControllerStreamSetupLogsRequest._();
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

  ServerControllerStreamSetupLogsRequest._();

  factory ServerControllerStreamSetupLogsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerStreamSetupLogsRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerStreamSetupLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerStreamSetupLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerStreamSetupLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerStreamSetupLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'installDependencies')
    ..aOS(3, _omitFieldNames ? '' : 'advertiseAddr')
    ..aOS(4, _omitFieldNames ? '' : 'acmeEmail')
    ..aOS(5, _omitFieldNames ? '' : 'hostKeyFingerprint')
    ..aOS(6, _omitFieldNames ? '' : 'sudoPassword')
    ..aI(7, _omitFieldNames ? '' : 'poolSize', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerStreamSetupLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerStreamSetupLogsRequest copyWith(
          void Function(ServerControllerStreamSetupLogsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerStreamSetupLogsRequest))
          as ServerControllerStreamSetupLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerStreamSetupLogsRequest() / ServerControllerStreamSetupLogsRequest.new instead')
  static ServerControllerStreamSetupLogsRequest create() =>
      ServerControllerStreamSetupLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerStreamSetupLogsRequest._();
  @$core.override
  ServerControllerStreamSetupLogsRequest createEmptyInstance() =>
      ServerControllerStreamSetupLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerStreamSetupLogsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerStreamSetupLogsRequest>(
          ServerControllerStreamSetupLogsRequest.$_createMessage);
  static ServerControllerStreamSetupLogsRequest? _defaultInstance;

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

/// One completion event is emitted when the streamed setup finishes.
class ServerControllerStreamSetupLogsResponse extends $pb.GeneratedMessage {
  factory ServerControllerStreamSetupLogsResponse({
    $fixnum.Int64? serverId,
    ServerSetupOutcome? outcome,
    $core.Iterable<$core.String>? lines,
  }) {
    final result = ServerControllerStreamSetupLogsResponse._();
    if (serverId != null) result.serverId = serverId;
    if (outcome != null) result.outcome = outcome;
    if (lines != null) result.lines.addAll(lines);
    return result;
  }

  ServerControllerStreamSetupLogsResponse._();

  factory ServerControllerStreamSetupLogsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerStreamSetupLogsResponse()
        ..mergeFromBuffer(data, registry);
  factory ServerControllerStreamSetupLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerStreamSetupLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerStreamSetupLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerControllerStreamSetupLogsResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOM<ServerSetupOutcome>(2, _omitFieldNames ? '' : 'outcome',
        subBuilder: ServerSetupOutcome.$_createMessage)
    ..pPS(3, _omitFieldNames ? '' : 'lines')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerStreamSetupLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerStreamSetupLogsResponse copyWith(
          void Function(ServerControllerStreamSetupLogsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerStreamSetupLogsResponse))
          as ServerControllerStreamSetupLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerStreamSetupLogsResponse() / ServerControllerStreamSetupLogsResponse.new instead')
  static ServerControllerStreamSetupLogsResponse create() =>
      ServerControllerStreamSetupLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerStreamSetupLogsResponse._();
  @$core.override
  ServerControllerStreamSetupLogsResponse createEmptyInstance() =>
      ServerControllerStreamSetupLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerStreamSetupLogsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerStreamSetupLogsResponse>(
          ServerControllerStreamSetupLogsResponse.$_createMessage);
  static ServerControllerStreamSetupLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  ServerSetupOutcome get outcome => $_getN(1);
  @$pb.TagNumber(2)
  set outcome(ServerSetupOutcome value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOutcome() => $_has(1);
  @$pb.TagNumber(2)
  void clearOutcome() => $_clearField(2);
  @$pb.TagNumber(2)
  ServerSetupOutcome ensureOutcome() => $_ensure(1);

  /// Live log lines streamed while the setup runs; the final event carries
  /// `outcome` instead.
  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get lines => $_getList(2);
}

class ServerControllerListSessionsRequest extends $pb.GeneratedMessage {
  factory ServerControllerListSessionsRequest() =>
      ServerControllerListSessionsRequest._();

  ServerControllerListSessionsRequest._();

  factory ServerControllerListSessionsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerListSessionsRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerListSessionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerListSessionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerListSessionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerListSessionsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerListSessionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerListSessionsRequest copyWith(
          void Function(ServerControllerListSessionsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerListSessionsRequest))
          as ServerControllerListSessionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerListSessionsRequest() / ServerControllerListSessionsRequest.new instead')
  static ServerControllerListSessionsRequest create() =>
      ServerControllerListSessionsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerListSessionsRequest._();
  @$core.override
  ServerControllerListSessionsRequest createEmptyInstance() =>
      ServerControllerListSessionsRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerListSessionsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerListSessionsRequest>(
          ServerControllerListSessionsRequest.$_createMessage);
  static ServerControllerListSessionsRequest? _defaultInstance;
}

class ServerControllerListSessionsResponse extends $pb.GeneratedMessage {
  factory ServerControllerListSessionsResponse({
    $core.Iterable<ServerConnectionInfo>? sessions,
  }) {
    final result = ServerControllerListSessionsResponse._();
    if (sessions != null) result.sessions.addAll(sessions);
    return result;
  }

  ServerControllerListSessionsResponse._();

  factory ServerControllerListSessionsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerListSessionsResponse()..mergeFromBuffer(data, registry);
  factory ServerControllerListSessionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerListSessionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerListSessionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerListSessionsResponse.$_createMessage)
    ..pPM<ServerConnectionInfo>(1, _omitFieldNames ? '' : 'sessions',
        subBuilder: ServerConnectionInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerListSessionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerListSessionsResponse copyWith(
          void Function(ServerControllerListSessionsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerListSessionsResponse))
          as ServerControllerListSessionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerListSessionsResponse() / ServerControllerListSessionsResponse.new instead')
  static ServerControllerListSessionsResponse create() =>
      ServerControllerListSessionsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerListSessionsResponse._();
  @$core.override
  ServerControllerListSessionsResponse createEmptyInstance() =>
      ServerControllerListSessionsResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerListSessionsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerListSessionsResponse>(
          ServerControllerListSessionsResponse.$_createMessage);
  static ServerControllerListSessionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ServerConnectionInfo> get sessions => $_getList(0);
}

class ServerControllerDeleteSessionRequest extends $pb.GeneratedMessage {
  factory ServerControllerDeleteSessionRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerControllerDeleteSessionRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerControllerDeleteSessionRequest._();

  factory ServerControllerDeleteSessionRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerDeleteSessionRequest()..mergeFromBuffer(data, registry);
  factory ServerControllerDeleteSessionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerControllerDeleteSessionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerControllerDeleteSessionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerControllerDeleteSessionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerDeleteSessionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerControllerDeleteSessionRequest copyWith(
          void Function(ServerControllerDeleteSessionRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerControllerDeleteSessionRequest))
          as ServerControllerDeleteSessionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerControllerDeleteSessionRequest() / ServerControllerDeleteSessionRequest.new instead')
  static ServerControllerDeleteSessionRequest create() =>
      ServerControllerDeleteSessionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerControllerDeleteSessionRequest._();
  @$core.override
  ServerControllerDeleteSessionRequest createEmptyInstance() =>
      ServerControllerDeleteSessionRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerControllerDeleteSessionRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerControllerDeleteSessionRequest>(
          ServerControllerDeleteSessionRequest.$_createMessage);
  static ServerControllerDeleteSessionRequest? _defaultInstance;

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

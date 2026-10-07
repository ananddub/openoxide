// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/swarm.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'swarm.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'swarm.pbenum.dart';

/// kind "swarm_info": `docker swarm inspect` summary for one engine.
class SwarmInfo extends $pb.GeneratedMessage {
  factory SwarmInfo({
    $core.String? nodeId,
    $core.String? nodeAddr,
    $core.String? localNodeState,
    $core.bool? controlAvailable,
    $fixnum.Int64? nodes,
    $fixnum.Int64? managers,
  }) {
    final result = SwarmInfo._();
    if (nodeId != null) result.nodeId = nodeId;
    if (nodeAddr != null) result.nodeAddr = nodeAddr;
    if (localNodeState != null) result.localNodeState = localNodeState;
    if (controlAvailable != null) result.controlAvailable = controlAvailable;
    if (nodes != null) result.nodes = nodes;
    if (managers != null) result.managers = managers;
    return result;
  }

  SwarmInfo._();

  factory SwarmInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfo()..mergeFromBuffer(data, registry);
  factory SwarmInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmInfo.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'nodeId')
    ..aOS(2, _omitFieldNames ? '' : 'nodeAddr')
    ..aOS(3, _omitFieldNames ? '' : 'localNodeState')
    ..aOB(4, _omitFieldNames ? '' : 'controlAvailable')
    ..aInt64(5, _omitFieldNames ? '' : 'nodes')
    ..aInt64(6, _omitFieldNames ? '' : 'managers')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfo copyWith(void Function(SwarmInfo) updates) =>
      super.copyWith((message) => updates(message as SwarmInfo)) as SwarmInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmInfo() / SwarmInfo.new instead')
  static SwarmInfo create() => SwarmInfo._();
  static $pb.GeneratedMessage $_createMessage() => SwarmInfo._();
  @$core.override
  SwarmInfo createEmptyInstance() => SwarmInfo._();
  @$core.pragma('dart2js:noInline')
  static SwarmInfo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmInfo>(SwarmInfo.$_createMessage);
  static SwarmInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get nodeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set nodeId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNodeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNodeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get nodeAddr => $_getSZ(1);
  @$pb.TagNumber(2)
  set nodeAddr($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNodeAddr() => $_has(1);
  @$pb.TagNumber(2)
  void clearNodeAddr() => $_clearField(2);

  /// Swarm state of the local node, e.g. "active"/"inactive"/"pending".
  @$pb.TagNumber(3)
  $core.String get localNodeState => $_getSZ(2);
  @$pb.TagNumber(3)
  set localNodeState($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLocalNodeState() => $_has(2);
  @$pb.TagNumber(3)
  void clearLocalNodeState() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get controlAvailable => $_getBF(3);
  @$pb.TagNumber(4)
  set controlAvailable($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasControlAvailable() => $_has(3);
  @$pb.TagNumber(4)
  void clearControlAvailable() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get nodes => $_getI64(4);
  @$pb.TagNumber(5)
  set nodes($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasNodes() => $_has(4);
  @$pb.TagNumber(5)
  void clearNodes() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get managers => $_getI64(5);
  @$pb.TagNumber(6)
  set managers($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasManagers() => $_has(5);
  @$pb.TagNumber(6)
  void clearManagers() => $_clearField(6);
}

/// kind "swarm_tokens": join tokens for one cluster.
class SwarmJoinTokens extends $pb.GeneratedMessage {
  factory SwarmJoinTokens({
    $core.String? worker,
    $core.String? manager,
  }) {
    final result = SwarmJoinTokens._();
    if (worker != null) result.worker = worker;
    if (manager != null) result.manager = manager;
    return result;
  }

  SwarmJoinTokens._();

  factory SwarmJoinTokens.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmJoinTokens()..mergeFromBuffer(data, registry);
  factory SwarmJoinTokens.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmJoinTokens()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmJoinTokens',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmJoinTokens.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'worker')
    ..aOS(2, _omitFieldNames ? '' : 'manager')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmJoinTokens clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmJoinTokens copyWith(void Function(SwarmJoinTokens) updates) =>
      super.copyWith((message) => updates(message as SwarmJoinTokens))
          as SwarmJoinTokens;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmJoinTokens() / SwarmJoinTokens.new instead')
  static SwarmJoinTokens create() => SwarmJoinTokens._();
  static $pb.GeneratedMessage $_createMessage() => SwarmJoinTokens._();
  @$core.override
  SwarmJoinTokens createEmptyInstance() => SwarmJoinTokens._();
  @$core.pragma('dart2js:noInline')
  static SwarmJoinTokens getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmJoinTokens>(
          SwarmJoinTokens.$_createMessage);
  static SwarmJoinTokens? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get worker => $_getSZ(0);
  @$pb.TagNumber(1)
  set worker($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWorker() => $_has(0);
  @$pb.TagNumber(1)
  void clearWorker() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get manager => $_getSZ(1);
  @$pb.TagNumber(2)
  set manager($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasManager() => $_has(1);
  @$pb.TagNumber(2)
  void clearManager() => $_clearField(2);
}

/// kind "swarm_node": one `docker node inspect` summary.
class SwarmNode extends $pb.GeneratedMessage {
  factory SwarmNode({
    $core.String? id,
    $core.String? hostname,
    $core.String? status,
    $core.String? availability,
    SwarmNodeRole? role,
    $core.bool? isLeader,
    $core.String? reachability,
    $core.String? ipAddress,
    $core.String? engineVersion,
  }) {
    final result = SwarmNode._();
    if (id != null) result.id = id;
    if (hostname != null) result.hostname = hostname;
    if (status != null) result.status = status;
    if (availability != null) result.availability = availability;
    if (role != null) result.role = role;
    if (isLeader != null) result.isLeader = isLeader;
    if (reachability != null) result.reachability = reachability;
    if (ipAddress != null) result.ipAddress = ipAddress;
    if (engineVersion != null) result.engineVersion = engineVersion;
    return result;
  }

  SwarmNode._();

  factory SwarmNode.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmNode()..mergeFromBuffer(data, registry);
  factory SwarmNode.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmNode()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmNode',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmNode.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'hostname')
    ..aOS(3, _omitFieldNames ? '' : 'status')
    ..aOS(4, _omitFieldNames ? '' : 'availability')
    ..aE<SwarmNodeRole>(5, _omitFieldNames ? '' : 'role',
        enumValues: SwarmNodeRole.values)
    ..aOB(6, _omitFieldNames ? '' : 'isLeader')
    ..aOS(7, _omitFieldNames ? '' : 'reachability')
    ..aOS(8, _omitFieldNames ? '' : 'ipAddress')
    ..aOS(9, _omitFieldNames ? '' : 'engineVersion')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmNode clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmNode copyWith(void Function(SwarmNode) updates) =>
      super.copyWith((message) => updates(message as SwarmNode)) as SwarmNode;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmNode() / SwarmNode.new instead')
  static SwarmNode create() => SwarmNode._();
  static $pb.GeneratedMessage $_createMessage() => SwarmNode._();
  @$core.override
  SwarmNode createEmptyInstance() => SwarmNode._();
  @$core.pragma('dart2js:noInline')
  static SwarmNode getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmNode>(SwarmNode.$_createMessage);
  static SwarmNode? _defaultInstance;

  /// Node id as issued by Docker (not a panel row id).
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get hostname => $_getSZ(1);
  @$pb.TagNumber(2)
  set hostname($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHostname() => $_has(1);
  @$pb.TagNumber(2)
  void clearHostname() => $_clearField(2);

  /// Node status state, e.g. "ready"/"down".
  @$pb.TagNumber(3)
  $core.String get status => $_getSZ(2);
  @$pb.TagNumber(3)
  set status($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  /// Node availability as reported by Docker ("active"/"pause"/"drain").
  @$pb.TagNumber(4)
  $core.String get availability => $_getSZ(3);
  @$pb.TagNumber(4)
  set availability($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAvailability() => $_has(3);
  @$pb.TagNumber(4)
  void clearAvailability() => $_clearField(4);

  /// From `Spec.Role`, the authoritative config value.
  @$pb.TagNumber(5)
  SwarmNodeRole get role => $_getN(4);
  @$pb.TagNumber(5)
  set role(SwarmNodeRole value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasRole() => $_has(4);
  @$pb.TagNumber(5)
  void clearRole() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get isLeader => $_getBF(5);
  @$pb.TagNumber(6)
  set isLeader($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasIsLeader() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsLeader() => $_clearField(6);

  /// "reachable" | "unreachable" | "" (empty for workers).
  @$pb.TagNumber(7)
  $core.String get reachability => $_getSZ(6);
  @$pb.TagNumber(7)
  set reachability($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasReachability() => $_has(6);
  @$pb.TagNumber(7)
  void clearReachability() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get ipAddress => $_getSZ(7);
  @$pb.TagNumber(8)
  set ipAddress($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasIpAddress() => $_has(7);
  @$pb.TagNumber(8)
  void clearIpAddress() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get engineVersion => $_getSZ(8);
  @$pb.TagNumber(9)
  set engineVersion($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEngineVersion() => $_has(8);
  @$pb.TagNumber(9)
  void clearEngineVersion() => $_clearField(9);
}

/// Panel-level swarm settings document (not yet served by any controller; per-service swarm templates live on DatabaseService instead).
class SwarmSettingsDocument extends $pb.GeneratedMessage {
  factory SwarmSettingsDocument({
    $core.String? settingsJson,
  }) {
    final result = SwarmSettingsDocument._();
    if (settingsJson != null) result.settingsJson = settingsJson;
    return result;
  }

  SwarmSettingsDocument._();

  factory SwarmSettingsDocument.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettingsDocument()..mergeFromBuffer(data, registry);
  factory SwarmSettingsDocument.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmSettingsDocument()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmSettingsDocument',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmSettingsDocument.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'settingsJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettingsDocument clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmSettingsDocument copyWith(
          void Function(SwarmSettingsDocument) updates) =>
      super.copyWith((message) => updates(message as SwarmSettingsDocument))
          as SwarmSettingsDocument;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmSettingsDocument() / SwarmSettingsDocument.new instead')
  static SwarmSettingsDocument create() => SwarmSettingsDocument._();
  static $pb.GeneratedMessage $_createMessage() => SwarmSettingsDocument._();
  @$core.override
  SwarmSettingsDocument createEmptyInstance() => SwarmSettingsDocument._();
  @$core.pragma('dart2js:noInline')
  static SwarmSettingsDocument getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmSettingsDocument>(
          SwarmSettingsDocument.$_createMessage);
  static SwarmSettingsDocument? _defaultInstance;

  /// Opaque JSON document of cluster-level swarm defaults.
  @$pb.TagNumber(1)
  $core.String get settingsJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set settingsJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSettingsJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettingsJson() => $_clearField(1);
}

/// Opaque `docker network` record (raw CLI JSON; fields are engine-dependent).
class SwarmNetwork extends $pb.GeneratedMessage {
  factory SwarmNetwork({
    $core.String? id,
    $core.String? name,
    $core.String? networkJson,
  }) {
    final result = SwarmNetwork._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (networkJson != null) result.networkJson = networkJson;
    return result;
  }

  SwarmNetwork._();

  factory SwarmNetwork.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmNetwork()..mergeFromBuffer(data, registry);
  factory SwarmNetwork.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmNetwork()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmNetwork',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmNetwork.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'networkJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmNetwork clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmNetwork copyWith(void Function(SwarmNetwork) updates) =>
      super.copyWith((message) => updates(message as SwarmNetwork))
          as SwarmNetwork;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmNetwork() / SwarmNetwork.new instead')
  static SwarmNetwork create() => SwarmNetwork._();
  static $pb.GeneratedMessage $_createMessage() => SwarmNetwork._();
  @$core.override
  SwarmNetwork createEmptyInstance() => SwarmNetwork._();
  @$core.pragma('dart2js:noInline')
  static SwarmNetwork getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmNetwork>(
          SwarmNetwork.$_createMessage);
  static SwarmNetwork? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
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
  $core.String get networkJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set networkJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasNetworkJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearNetworkJson() => $_clearField(3);
}

/// Opaque `docker service` record (raw CLI JSON; fields are engine-dependent).
class SwarmServiceInfo extends $pb.GeneratedMessage {
  factory SwarmServiceInfo({
    $core.String? id,
    $core.String? name,
    $core.String? serviceJson,
  }) {
    final result = SwarmServiceInfo._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (serviceJson != null) result.serviceJson = serviceJson;
    return result;
  }

  SwarmServiceInfo._();

  factory SwarmServiceInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmServiceInfo()..mergeFromBuffer(data, registry);
  factory SwarmServiceInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmServiceInfo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmServiceInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmServiceInfo.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'serviceJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmServiceInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmServiceInfo copyWith(void Function(SwarmServiceInfo) updates) =>
      super.copyWith((message) => updates(message as SwarmServiceInfo))
          as SwarmServiceInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmServiceInfo() / SwarmServiceInfo.new instead')
  static SwarmServiceInfo create() => SwarmServiceInfo._();
  static $pb.GeneratedMessage $_createMessage() => SwarmServiceInfo._();
  @$core.override
  SwarmServiceInfo createEmptyInstance() => SwarmServiceInfo._();
  @$core.pragma('dart2js:noInline')
  static SwarmServiceInfo getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmServiceInfo>(
          SwarmServiceInfo.$_createMessage);
  static SwarmServiceInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
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
  $core.String get serviceJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set serviceJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasServiceJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearServiceJson() => $_clearField(3);
}

class SwarmGetSettingsRequest extends $pb.GeneratedMessage {
  factory SwarmGetSettingsRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmGetSettingsRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmGetSettingsRequest._();

  factory SwarmGetSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetSettingsRequest()..mergeFromBuffer(data, registry);
  factory SwarmGetSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetSettingsRequest copyWith(
          void Function(SwarmGetSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmGetSettingsRequest))
          as SwarmGetSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetSettingsRequest() / SwarmGetSettingsRequest.new instead')
  static SwarmGetSettingsRequest create() => SwarmGetSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmGetSettingsRequest._();
  @$core.override
  SwarmGetSettingsRequest createEmptyInstance() => SwarmGetSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetSettingsRequest>(
          SwarmGetSettingsRequest.$_createMessage);
  static SwarmGetSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmGetSettingsResponse extends $pb.GeneratedMessage {
  factory SwarmGetSettingsResponse({
    SwarmSettingsDocument? settings,
  }) {
    final result = SwarmGetSettingsResponse._();
    if (settings != null) result.settings = settings;
    return result;
  }

  SwarmGetSettingsResponse._();

  factory SwarmGetSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetSettingsResponse()..mergeFromBuffer(data, registry);
  factory SwarmGetSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetSettingsResponse.$_createMessage)
    ..aOM<SwarmSettingsDocument>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: SwarmSettingsDocument.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetSettingsResponse copyWith(
          void Function(SwarmGetSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmGetSettingsResponse))
          as SwarmGetSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetSettingsResponse() / SwarmGetSettingsResponse.new instead')
  static SwarmGetSettingsResponse create() => SwarmGetSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => SwarmGetSettingsResponse._();
  @$core.override
  SwarmGetSettingsResponse createEmptyInstance() =>
      SwarmGetSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetSettingsResponse>(
          SwarmGetSettingsResponse.$_createMessage);
  static SwarmGetSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmSettingsDocument get settings => $_getN(0);
  @$pb.TagNumber(1)
  set settings(SwarmSettingsDocument value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSettings() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettings() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmSettingsDocument ensureSettings() => $_ensure(0);
}

class SwarmUpdateSettingsRequest extends $pb.GeneratedMessage {
  factory SwarmUpdateSettingsRequest({
    $fixnum.Int64? serverId,
    SwarmSettingsDocument? settings,
  }) {
    final result = SwarmUpdateSettingsRequest._();
    if (serverId != null) result.serverId = serverId;
    if (settings != null) result.settings = settings;
    return result;
  }

  SwarmUpdateSettingsRequest._();

  factory SwarmUpdateSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateSettingsRequest()..mergeFromBuffer(data, registry);
  factory SwarmUpdateSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOM<SwarmSettingsDocument>(2, _omitFieldNames ? '' : 'settings',
        subBuilder: SwarmSettingsDocument.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateSettingsRequest copyWith(
          void Function(SwarmUpdateSettingsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmUpdateSettingsRequest))
          as SwarmUpdateSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateSettingsRequest() / SwarmUpdateSettingsRequest.new instead')
  static SwarmUpdateSettingsRequest create() => SwarmUpdateSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateSettingsRequest._();
  @$core.override
  SwarmUpdateSettingsRequest createEmptyInstance() =>
      SwarmUpdateSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateSettingsRequest>(
          SwarmUpdateSettingsRequest.$_createMessage);
  static SwarmUpdateSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  SwarmSettingsDocument get settings => $_getN(1);
  @$pb.TagNumber(2)
  set settings(SwarmSettingsDocument value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSettings() => $_has(1);
  @$pb.TagNumber(2)
  void clearSettings() => $_clearField(2);
  @$pb.TagNumber(2)
  SwarmSettingsDocument ensureSettings() => $_ensure(1);
}

class SwarmUpdateSettingsResponse extends $pb.GeneratedMessage {
  factory SwarmUpdateSettingsResponse({
    SwarmSettingsDocument? settings,
  }) {
    final result = SwarmUpdateSettingsResponse._();
    if (settings != null) result.settings = settings;
    return result;
  }

  SwarmUpdateSettingsResponse._();

  factory SwarmUpdateSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateSettingsResponse()..mergeFromBuffer(data, registry);
  factory SwarmUpdateSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateSettingsResponse.$_createMessage)
    ..aOM<SwarmSettingsDocument>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: SwarmSettingsDocument.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateSettingsResponse copyWith(
          void Function(SwarmUpdateSettingsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmUpdateSettingsResponse))
          as SwarmUpdateSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateSettingsResponse() / SwarmUpdateSettingsResponse.new instead')
  static SwarmUpdateSettingsResponse create() =>
      SwarmUpdateSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateSettingsResponse._();
  @$core.override
  SwarmUpdateSettingsResponse createEmptyInstance() =>
      SwarmUpdateSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateSettingsResponse>(
          SwarmUpdateSettingsResponse.$_createMessage);
  static SwarmUpdateSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmSettingsDocument get settings => $_getN(0);
  @$pb.TagNumber(1)
  set settings(SwarmSettingsDocument value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSettings() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettings() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmSettingsDocument ensureSettings() => $_ensure(0);
}

class SwarmListNetworksRequest extends $pb.GeneratedMessage {
  factory SwarmListNetworksRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmListNetworksRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmListNetworksRequest._();

  factory SwarmListNetworksRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNetworksRequest()..mergeFromBuffer(data, registry);
  factory SwarmListNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListNetworksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNetworksRequest copyWith(
          void Function(SwarmListNetworksRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmListNetworksRequest))
          as SwarmListNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListNetworksRequest() / SwarmListNetworksRequest.new instead')
  static SwarmListNetworksRequest create() => SwarmListNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmListNetworksRequest._();
  @$core.override
  SwarmListNetworksRequest createEmptyInstance() =>
      SwarmListNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmListNetworksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListNetworksRequest>(
          SwarmListNetworksRequest.$_createMessage);
  static SwarmListNetworksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmListNetworksResponse extends $pb.GeneratedMessage {
  factory SwarmListNetworksResponse({
    $core.Iterable<SwarmNetwork>? networks,
  }) {
    final result = SwarmListNetworksResponse._();
    if (networks != null) result.networks.addAll(networks);
    return result;
  }

  SwarmListNetworksResponse._();

  factory SwarmListNetworksResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNetworksResponse()..mergeFromBuffer(data, registry);
  factory SwarmListNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNetworksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListNetworksResponse.$_createMessage)
    ..pPM<SwarmNetwork>(1, _omitFieldNames ? '' : 'networks',
        subBuilder: SwarmNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNetworksResponse copyWith(
          void Function(SwarmListNetworksResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmListNetworksResponse))
          as SwarmListNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListNetworksResponse() / SwarmListNetworksResponse.new instead')
  static SwarmListNetworksResponse create() => SwarmListNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmListNetworksResponse._();
  @$core.override
  SwarmListNetworksResponse createEmptyInstance() =>
      SwarmListNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmListNetworksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListNetworksResponse>(
          SwarmListNetworksResponse.$_createMessage);
  static SwarmListNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SwarmNetwork> get networks => $_getList(0);
}

class SwarmCreateNetworkRequest extends $pb.GeneratedMessage {
  factory SwarmCreateNetworkRequest({
    $fixnum.Int64? serverId,
    $core.String? name,
    $core.String? configJson,
  }) {
    final result = SwarmCreateNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    if (name != null) result.name = name;
    if (configJson != null) result.configJson = configJson;
    return result;
  }

  SwarmCreateNetworkRequest._();

  factory SwarmCreateNetworkRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmCreateNetworkRequest()..mergeFromBuffer(data, registry);
  factory SwarmCreateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmCreateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmCreateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmCreateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'configJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmCreateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmCreateNetworkRequest copyWith(
          void Function(SwarmCreateNetworkRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmCreateNetworkRequest))
          as SwarmCreateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmCreateNetworkRequest() / SwarmCreateNetworkRequest.new instead')
  static SwarmCreateNetworkRequest create() => SwarmCreateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmCreateNetworkRequest._();
  @$core.override
  SwarmCreateNetworkRequest createEmptyInstance() =>
      SwarmCreateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmCreateNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmCreateNetworkRequest>(
          SwarmCreateNetworkRequest.$_createMessage);
  static SwarmCreateNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// Opaque JSON document of network options (driver, labels, ...).
  @$pb.TagNumber(3)
  $core.String get configJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set configJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasConfigJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearConfigJson() => $_clearField(3);
}

class SwarmCreateNetworkResponse extends $pb.GeneratedMessage {
  factory SwarmCreateNetworkResponse({
    SwarmNetwork? network,
  }) {
    final result = SwarmCreateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  SwarmCreateNetworkResponse._();

  factory SwarmCreateNetworkResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmCreateNetworkResponse()..mergeFromBuffer(data, registry);
  factory SwarmCreateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmCreateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmCreateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmCreateNetworkResponse.$_createMessage)
    ..aOM<SwarmNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: SwarmNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmCreateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmCreateNetworkResponse copyWith(
          void Function(SwarmCreateNetworkResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmCreateNetworkResponse))
          as SwarmCreateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmCreateNetworkResponse() / SwarmCreateNetworkResponse.new instead')
  static SwarmCreateNetworkResponse create() => SwarmCreateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmCreateNetworkResponse._();
  @$core.override
  SwarmCreateNetworkResponse createEmptyInstance() =>
      SwarmCreateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmCreateNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmCreateNetworkResponse>(
          SwarmCreateNetworkResponse.$_createMessage);
  static SwarmCreateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(SwarmNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmNetwork ensureNetwork() => $_ensure(0);
}

class SwarmUpdateNetworkRequest extends $pb.GeneratedMessage {
  factory SwarmUpdateNetworkRequest({
    $fixnum.Int64? serverId,
    $core.String? networkId,
    $core.String? configJson,
  }) {
    final result = SwarmUpdateNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    if (networkId != null) result.networkId = networkId;
    if (configJson != null) result.configJson = configJson;
    return result;
  }

  SwarmUpdateNetworkRequest._();

  factory SwarmUpdateNetworkRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNetworkRequest()..mergeFromBuffer(data, registry);
  factory SwarmUpdateNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'networkId')
    ..aOS(3, _omitFieldNames ? '' : 'configJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNetworkRequest copyWith(
          void Function(SwarmUpdateNetworkRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmUpdateNetworkRequest))
          as SwarmUpdateNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateNetworkRequest() / SwarmUpdateNetworkRequest.new instead')
  static SwarmUpdateNetworkRequest create() => SwarmUpdateNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateNetworkRequest._();
  @$core.override
  SwarmUpdateNetworkRequest createEmptyInstance() =>
      SwarmUpdateNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateNetworkRequest>(
          SwarmUpdateNetworkRequest.$_createMessage);
  static SwarmUpdateNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get networkId => $_getSZ(1);
  @$pb.TagNumber(2)
  set networkId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNetworkId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNetworkId() => $_clearField(2);

  /// Opaque JSON document of the network fields to replace.
  @$pb.TagNumber(3)
  $core.String get configJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set configJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasConfigJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearConfigJson() => $_clearField(3);
}

class SwarmUpdateNetworkResponse extends $pb.GeneratedMessage {
  factory SwarmUpdateNetworkResponse({
    SwarmNetwork? network,
  }) {
    final result = SwarmUpdateNetworkResponse._();
    if (network != null) result.network = network;
    return result;
  }

  SwarmUpdateNetworkResponse._();

  factory SwarmUpdateNetworkResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNetworkResponse()..mergeFromBuffer(data, registry);
  factory SwarmUpdateNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateNetworkResponse.$_createMessage)
    ..aOM<SwarmNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: SwarmNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNetworkResponse copyWith(
          void Function(SwarmUpdateNetworkResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmUpdateNetworkResponse))
          as SwarmUpdateNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateNetworkResponse() / SwarmUpdateNetworkResponse.new instead')
  static SwarmUpdateNetworkResponse create() => SwarmUpdateNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateNetworkResponse._();
  @$core.override
  SwarmUpdateNetworkResponse createEmptyInstance() =>
      SwarmUpdateNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateNetworkResponse>(
          SwarmUpdateNetworkResponse.$_createMessage);
  static SwarmUpdateNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(SwarmNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmNetwork ensureNetwork() => $_ensure(0);
}

class SwarmDeleteNetworkRequest extends $pb.GeneratedMessage {
  factory SwarmDeleteNetworkRequest({
    $fixnum.Int64? serverId,
    $core.String? networkId,
  }) {
    final result = SwarmDeleteNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    if (networkId != null) result.networkId = networkId;
    return result;
  }

  SwarmDeleteNetworkRequest._();

  factory SwarmDeleteNetworkRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmDeleteNetworkRequest()..mergeFromBuffer(data, registry);
  factory SwarmDeleteNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmDeleteNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmDeleteNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmDeleteNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'networkId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmDeleteNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmDeleteNetworkRequest copyWith(
          void Function(SwarmDeleteNetworkRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmDeleteNetworkRequest))
          as SwarmDeleteNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmDeleteNetworkRequest() / SwarmDeleteNetworkRequest.new instead')
  static SwarmDeleteNetworkRequest create() => SwarmDeleteNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmDeleteNetworkRequest._();
  @$core.override
  SwarmDeleteNetworkRequest createEmptyInstance() =>
      SwarmDeleteNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmDeleteNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmDeleteNetworkRequest>(
          SwarmDeleteNetworkRequest.$_createMessage);
  static SwarmDeleteNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get networkId => $_getSZ(1);
  @$pb.TagNumber(2)
  set networkId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNetworkId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNetworkId() => $_clearField(2);
}

class SwarmListServicesRequest extends $pb.GeneratedMessage {
  factory SwarmListServicesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmListServicesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmListServicesRequest._();

  factory SwarmListServicesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListServicesRequest()..mergeFromBuffer(data, registry);
  factory SwarmListServicesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListServicesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListServicesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListServicesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListServicesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListServicesRequest copyWith(
          void Function(SwarmListServicesRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmListServicesRequest))
          as SwarmListServicesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListServicesRequest() / SwarmListServicesRequest.new instead')
  static SwarmListServicesRequest create() => SwarmListServicesRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmListServicesRequest._();
  @$core.override
  SwarmListServicesRequest createEmptyInstance() =>
      SwarmListServicesRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmListServicesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListServicesRequest>(
          SwarmListServicesRequest.$_createMessage);
  static SwarmListServicesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmListServicesResponse extends $pb.GeneratedMessage {
  factory SwarmListServicesResponse({
    $core.Iterable<SwarmServiceInfo>? services,
  }) {
    final result = SwarmListServicesResponse._();
    if (services != null) result.services.addAll(services);
    return result;
  }

  SwarmListServicesResponse._();

  factory SwarmListServicesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListServicesResponse()..mergeFromBuffer(data, registry);
  factory SwarmListServicesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListServicesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListServicesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListServicesResponse.$_createMessage)
    ..pPM<SwarmServiceInfo>(1, _omitFieldNames ? '' : 'services',
        subBuilder: SwarmServiceInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListServicesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListServicesResponse copyWith(
          void Function(SwarmListServicesResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmListServicesResponse))
          as SwarmListServicesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListServicesResponse() / SwarmListServicesResponse.new instead')
  static SwarmListServicesResponse create() => SwarmListServicesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmListServicesResponse._();
  @$core.override
  SwarmListServicesResponse createEmptyInstance() =>
      SwarmListServicesResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmListServicesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListServicesResponse>(
          SwarmListServicesResponse.$_createMessage);
  static SwarmListServicesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SwarmServiceInfo> get services => $_getList(0);
}

class SwarmGetServiceRequest extends $pb.GeneratedMessage {
  factory SwarmGetServiceRequest({
    $fixnum.Int64? serverId,
    $core.String? serviceId,
  }) {
    final result = SwarmGetServiceRequest._();
    if (serverId != null) result.serverId = serverId;
    if (serviceId != null) result.serviceId = serviceId;
    return result;
  }

  SwarmGetServiceRequest._();

  factory SwarmGetServiceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetServiceRequest()..mergeFromBuffer(data, registry);
  factory SwarmGetServiceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetServiceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetServiceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetServiceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'serviceId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetServiceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetServiceRequest copyWith(
          void Function(SwarmGetServiceRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmGetServiceRequest))
          as SwarmGetServiceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetServiceRequest() / SwarmGetServiceRequest.new instead')
  static SwarmGetServiceRequest create() => SwarmGetServiceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmGetServiceRequest._();
  @$core.override
  SwarmGetServiceRequest createEmptyInstance() => SwarmGetServiceRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetServiceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetServiceRequest>(
          SwarmGetServiceRequest.$_createMessage);
  static SwarmGetServiceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get serviceId => $_getSZ(1);
  @$pb.TagNumber(2)
  set serviceId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServiceId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServiceId() => $_clearField(2);
}

class SwarmGetServiceResponse extends $pb.GeneratedMessage {
  factory SwarmGetServiceResponse({
    SwarmServiceInfo? service,
  }) {
    final result = SwarmGetServiceResponse._();
    if (service != null) result.service = service;
    return result;
  }

  SwarmGetServiceResponse._();

  factory SwarmGetServiceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetServiceResponse()..mergeFromBuffer(data, registry);
  factory SwarmGetServiceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetServiceResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetServiceResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetServiceResponse.$_createMessage)
    ..aOM<SwarmServiceInfo>(1, _omitFieldNames ? '' : 'service',
        subBuilder: SwarmServiceInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetServiceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetServiceResponse copyWith(
          void Function(SwarmGetServiceResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmGetServiceResponse))
          as SwarmGetServiceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetServiceResponse() / SwarmGetServiceResponse.new instead')
  static SwarmGetServiceResponse create() => SwarmGetServiceResponse._();
  static $pb.GeneratedMessage $_createMessage() => SwarmGetServiceResponse._();
  @$core.override
  SwarmGetServiceResponse createEmptyInstance() => SwarmGetServiceResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetServiceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetServiceResponse>(
          SwarmGetServiceResponse.$_createMessage);
  static SwarmGetServiceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmServiceInfo get service => $_getN(0);
  @$pb.TagNumber(1)
  set service(SwarmServiceInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasService() => $_has(0);
  @$pb.TagNumber(1)
  void clearService() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmServiceInfo ensureService() => $_ensure(0);
}

class SwarmUpdateServiceRequest extends $pb.GeneratedMessage {
  factory SwarmUpdateServiceRequest({
    $fixnum.Int64? serverId,
    $core.String? serviceId,
    $core.String? specJson,
  }) {
    final result = SwarmUpdateServiceRequest._();
    if (serverId != null) result.serverId = serverId;
    if (serviceId != null) result.serviceId = serviceId;
    if (specJson != null) result.specJson = specJson;
    return result;
  }

  SwarmUpdateServiceRequest._();

  factory SwarmUpdateServiceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateServiceRequest()..mergeFromBuffer(data, registry);
  factory SwarmUpdateServiceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateServiceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateServiceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateServiceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'serviceId')
    ..aOS(3, _omitFieldNames ? '' : 'specJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateServiceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateServiceRequest copyWith(
          void Function(SwarmUpdateServiceRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmUpdateServiceRequest))
          as SwarmUpdateServiceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateServiceRequest() / SwarmUpdateServiceRequest.new instead')
  static SwarmUpdateServiceRequest create() => SwarmUpdateServiceRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateServiceRequest._();
  @$core.override
  SwarmUpdateServiceRequest createEmptyInstance() =>
      SwarmUpdateServiceRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateServiceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateServiceRequest>(
          SwarmUpdateServiceRequest.$_createMessage);
  static SwarmUpdateServiceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get serviceId => $_getSZ(1);
  @$pb.TagNumber(2)
  set serviceId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServiceId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServiceId() => $_clearField(2);

  /// Opaque JSON document of the service spec fields to replace.
  @$pb.TagNumber(3)
  $core.String get specJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set specJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSpecJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearSpecJson() => $_clearField(3);
}

class SwarmUpdateServiceResponse extends $pb.GeneratedMessage {
  factory SwarmUpdateServiceResponse({
    SwarmServiceInfo? service,
  }) {
    final result = SwarmUpdateServiceResponse._();
    if (service != null) result.service = service;
    return result;
  }

  SwarmUpdateServiceResponse._();

  factory SwarmUpdateServiceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateServiceResponse()..mergeFromBuffer(data, registry);
  factory SwarmUpdateServiceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateServiceResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateServiceResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateServiceResponse.$_createMessage)
    ..aOM<SwarmServiceInfo>(1, _omitFieldNames ? '' : 'service',
        subBuilder: SwarmServiceInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateServiceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateServiceResponse copyWith(
          void Function(SwarmUpdateServiceResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmUpdateServiceResponse))
          as SwarmUpdateServiceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateServiceResponse() / SwarmUpdateServiceResponse.new instead')
  static SwarmUpdateServiceResponse create() => SwarmUpdateServiceResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateServiceResponse._();
  @$core.override
  SwarmUpdateServiceResponse createEmptyInstance() =>
      SwarmUpdateServiceResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateServiceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateServiceResponse>(
          SwarmUpdateServiceResponse.$_createMessage);
  static SwarmUpdateServiceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmServiceInfo get service => $_getN(0);
  @$pb.TagNumber(1)
  set service(SwarmServiceInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasService() => $_has(0);
  @$pb.TagNumber(1)
  void clearService() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmServiceInfo ensureService() => $_ensure(0);
}

class SwarmInfoRequest extends $pb.GeneratedMessage {
  factory SwarmInfoRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmInfoRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmInfoRequest._();

  factory SwarmInfoRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfoRequest()..mergeFromBuffer(data, registry);
  factory SwarmInfoRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfoRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmInfoRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmInfoRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfoRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfoRequest copyWith(void Function(SwarmInfoRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmInfoRequest))
          as SwarmInfoRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmInfoRequest() / SwarmInfoRequest.new instead')
  static SwarmInfoRequest create() => SwarmInfoRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmInfoRequest._();
  @$core.override
  SwarmInfoRequest createEmptyInstance() => SwarmInfoRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmInfoRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmInfoRequest>(
          SwarmInfoRequest.$_createMessage);
  static SwarmInfoRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmInfoResponse extends $pb.GeneratedMessage {
  factory SwarmInfoResponse({
    SwarmInfo? info,
  }) {
    final result = SwarmInfoResponse._();
    if (info != null) result.info = info;
    return result;
  }

  SwarmInfoResponse._();

  factory SwarmInfoResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfoResponse()..mergeFromBuffer(data, registry);
  factory SwarmInfoResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmInfoResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmInfoResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmInfoResponse.$_createMessage)
    ..aOM<SwarmInfo>(1, _omitFieldNames ? '' : 'info',
        subBuilder: SwarmInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfoResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmInfoResponse copyWith(void Function(SwarmInfoResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmInfoResponse))
          as SwarmInfoResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmInfoResponse() / SwarmInfoResponse.new instead')
  static SwarmInfoResponse create() => SwarmInfoResponse._();
  static $pb.GeneratedMessage $_createMessage() => SwarmInfoResponse._();
  @$core.override
  SwarmInfoResponse createEmptyInstance() => SwarmInfoResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmInfoResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmInfoResponse>(
          SwarmInfoResponse.$_createMessage);
  static SwarmInfoResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmInfo get info => $_getN(0);
  @$pb.TagNumber(1)
  set info(SwarmInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmInfo ensureInfo() => $_ensure(0);
}

class SwarmGetJoinTokensRequest extends $pb.GeneratedMessage {
  factory SwarmGetJoinTokensRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmGetJoinTokensRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmGetJoinTokensRequest._();

  factory SwarmGetJoinTokensRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetJoinTokensRequest()..mergeFromBuffer(data, registry);
  factory SwarmGetJoinTokensRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetJoinTokensRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetJoinTokensRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetJoinTokensRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetJoinTokensRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetJoinTokensRequest copyWith(
          void Function(SwarmGetJoinTokensRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmGetJoinTokensRequest))
          as SwarmGetJoinTokensRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetJoinTokensRequest() / SwarmGetJoinTokensRequest.new instead')
  static SwarmGetJoinTokensRequest create() => SwarmGetJoinTokensRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmGetJoinTokensRequest._();
  @$core.override
  SwarmGetJoinTokensRequest createEmptyInstance() =>
      SwarmGetJoinTokensRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetJoinTokensRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetJoinTokensRequest>(
          SwarmGetJoinTokensRequest.$_createMessage);
  static SwarmGetJoinTokensRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmGetJoinTokensResponse extends $pb.GeneratedMessage {
  factory SwarmGetJoinTokensResponse({
    SwarmJoinTokens? tokens,
  }) {
    final result = SwarmGetJoinTokensResponse._();
    if (tokens != null) result.tokens = tokens;
    return result;
  }

  SwarmGetJoinTokensResponse._();

  factory SwarmGetJoinTokensResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetJoinTokensResponse()..mergeFromBuffer(data, registry);
  factory SwarmGetJoinTokensResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmGetJoinTokensResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmGetJoinTokensResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmGetJoinTokensResponse.$_createMessage)
    ..aOM<SwarmJoinTokens>(1, _omitFieldNames ? '' : 'tokens',
        subBuilder: SwarmJoinTokens.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetJoinTokensResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmGetJoinTokensResponse copyWith(
          void Function(SwarmGetJoinTokensResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SwarmGetJoinTokensResponse))
          as SwarmGetJoinTokensResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmGetJoinTokensResponse() / SwarmGetJoinTokensResponse.new instead')
  static SwarmGetJoinTokensResponse create() => SwarmGetJoinTokensResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmGetJoinTokensResponse._();
  @$core.override
  SwarmGetJoinTokensResponse createEmptyInstance() =>
      SwarmGetJoinTokensResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmGetJoinTokensResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmGetJoinTokensResponse>(
          SwarmGetJoinTokensResponse.$_createMessage);
  static SwarmGetJoinTokensResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SwarmJoinTokens get tokens => $_getN(0);
  @$pb.TagNumber(1)
  set tokens(SwarmJoinTokens value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasTokens() => $_has(0);
  @$pb.TagNumber(1)
  void clearTokens() => $_clearField(1);
  @$pb.TagNumber(1)
  SwarmJoinTokens ensureTokens() => $_ensure(0);
}

class SwarmListNodesRequest extends $pb.GeneratedMessage {
  factory SwarmListNodesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = SwarmListNodesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  SwarmListNodesRequest._();

  factory SwarmListNodesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNodesRequest()..mergeFromBuffer(data, registry);
  factory SwarmListNodesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNodesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListNodesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListNodesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNodesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNodesRequest copyWith(
          void Function(SwarmListNodesRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmListNodesRequest))
          as SwarmListNodesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListNodesRequest() / SwarmListNodesRequest.new instead')
  static SwarmListNodesRequest create() => SwarmListNodesRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmListNodesRequest._();
  @$core.override
  SwarmListNodesRequest createEmptyInstance() => SwarmListNodesRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmListNodesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListNodesRequest>(
          SwarmListNodesRequest.$_createMessage);
  static SwarmListNodesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class SwarmListNodesResponse extends $pb.GeneratedMessage {
  factory SwarmListNodesResponse({
    $core.Iterable<SwarmNode>? nodes,
  }) {
    final result = SwarmListNodesResponse._();
    if (nodes != null) result.nodes.addAll(nodes);
    return result;
  }

  SwarmListNodesResponse._();

  factory SwarmListNodesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNodesResponse()..mergeFromBuffer(data, registry);
  factory SwarmListNodesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmListNodesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmListNodesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmListNodesResponse.$_createMessage)
    ..pPM<SwarmNode>(1, _omitFieldNames ? '' : 'nodes',
        subBuilder: SwarmNode.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNodesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmListNodesResponse copyWith(
          void Function(SwarmListNodesResponse) updates) =>
      super.copyWith((message) => updates(message as SwarmListNodesResponse))
          as SwarmListNodesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmListNodesResponse() / SwarmListNodesResponse.new instead')
  static SwarmListNodesResponse create() => SwarmListNodesResponse._();
  static $pb.GeneratedMessage $_createMessage() => SwarmListNodesResponse._();
  @$core.override
  SwarmListNodesResponse createEmptyInstance() => SwarmListNodesResponse._();
  @$core.pragma('dart2js:noInline')
  static SwarmListNodesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmListNodesResponse>(
          SwarmListNodesResponse.$_createMessage);
  static SwarmListNodesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SwarmNode> get nodes => $_getList(0);
}

class SwarmPromoteNodeRequest extends $pb.GeneratedMessage {
  factory SwarmPromoteNodeRequest({
    $fixnum.Int64? serverId,
    $core.String? nodeId,
  }) {
    final result = SwarmPromoteNodeRequest._();
    if (serverId != null) result.serverId = serverId;
    if (nodeId != null) result.nodeId = nodeId;
    return result;
  }

  SwarmPromoteNodeRequest._();

  factory SwarmPromoteNodeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmPromoteNodeRequest()..mergeFromBuffer(data, registry);
  factory SwarmPromoteNodeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmPromoteNodeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmPromoteNodeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmPromoteNodeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'nodeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmPromoteNodeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmPromoteNodeRequest copyWith(
          void Function(SwarmPromoteNodeRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmPromoteNodeRequest))
          as SwarmPromoteNodeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmPromoteNodeRequest() / SwarmPromoteNodeRequest.new instead')
  static SwarmPromoteNodeRequest create() => SwarmPromoteNodeRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmPromoteNodeRequest._();
  @$core.override
  SwarmPromoteNodeRequest createEmptyInstance() => SwarmPromoteNodeRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmPromoteNodeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmPromoteNodeRequest>(
          SwarmPromoteNodeRequest.$_createMessage);
  static SwarmPromoteNodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get nodeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set nodeId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNodeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNodeId() => $_clearField(2);
}

class SwarmDemoteNodeRequest extends $pb.GeneratedMessage {
  factory SwarmDemoteNodeRequest({
    $fixnum.Int64? serverId,
    $core.String? nodeId,
  }) {
    final result = SwarmDemoteNodeRequest._();
    if (serverId != null) result.serverId = serverId;
    if (nodeId != null) result.nodeId = nodeId;
    return result;
  }

  SwarmDemoteNodeRequest._();

  factory SwarmDemoteNodeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmDemoteNodeRequest()..mergeFromBuffer(data, registry);
  factory SwarmDemoteNodeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmDemoteNodeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmDemoteNodeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmDemoteNodeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'nodeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmDemoteNodeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmDemoteNodeRequest copyWith(
          void Function(SwarmDemoteNodeRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmDemoteNodeRequest))
          as SwarmDemoteNodeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmDemoteNodeRequest() / SwarmDemoteNodeRequest.new instead')
  static SwarmDemoteNodeRequest create() => SwarmDemoteNodeRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmDemoteNodeRequest._();
  @$core.override
  SwarmDemoteNodeRequest createEmptyInstance() => SwarmDemoteNodeRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmDemoteNodeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmDemoteNodeRequest>(
          SwarmDemoteNodeRequest.$_createMessage);
  static SwarmDemoteNodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get nodeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set nodeId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNodeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNodeId() => $_clearField(2);
}

class SwarmUpdateNodeAvailabilityRequest extends $pb.GeneratedMessage {
  factory SwarmUpdateNodeAvailabilityRequest({
    $fixnum.Int64? serverId,
    $core.String? nodeId,
    SwarmNodeAvailability? availability,
  }) {
    final result = SwarmUpdateNodeAvailabilityRequest._();
    if (serverId != null) result.serverId = serverId;
    if (nodeId != null) result.nodeId = nodeId;
    if (availability != null) result.availability = availability;
    return result;
  }

  SwarmUpdateNodeAvailabilityRequest._();

  factory SwarmUpdateNodeAvailabilityRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNodeAvailabilityRequest()..mergeFromBuffer(data, registry);
  factory SwarmUpdateNodeAvailabilityRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmUpdateNodeAvailabilityRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmUpdateNodeAvailabilityRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmUpdateNodeAvailabilityRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'nodeId')
    ..aE<SwarmNodeAvailability>(3, _omitFieldNames ? '' : 'availability',
        enumValues: SwarmNodeAvailability.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNodeAvailabilityRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmUpdateNodeAvailabilityRequest copyWith(
          void Function(SwarmUpdateNodeAvailabilityRequest) updates) =>
      super.copyWith((message) =>
              updates(message as SwarmUpdateNodeAvailabilityRequest))
          as SwarmUpdateNodeAvailabilityRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmUpdateNodeAvailabilityRequest() / SwarmUpdateNodeAvailabilityRequest.new instead')
  static SwarmUpdateNodeAvailabilityRequest create() =>
      SwarmUpdateNodeAvailabilityRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SwarmUpdateNodeAvailabilityRequest._();
  @$core.override
  SwarmUpdateNodeAvailabilityRequest createEmptyInstance() =>
      SwarmUpdateNodeAvailabilityRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmUpdateNodeAvailabilityRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmUpdateNodeAvailabilityRequest>(
          SwarmUpdateNodeAvailabilityRequest.$_createMessage);
  static SwarmUpdateNodeAvailabilityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get nodeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set nodeId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNodeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNodeId() => $_clearField(2);

  @$pb.TagNumber(3)
  SwarmNodeAvailability get availability => $_getN(2);
  @$pb.TagNumber(3)
  set availability(SwarmNodeAvailability value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasAvailability() => $_has(2);
  @$pb.TagNumber(3)
  void clearAvailability() => $_clearField(3);
}

class SwarmRemoveNodeRequest extends $pb.GeneratedMessage {
  factory SwarmRemoveNodeRequest({
    $fixnum.Int64? serverId,
    $core.String? nodeId,
  }) {
    final result = SwarmRemoveNodeRequest._();
    if (serverId != null) result.serverId = serverId;
    if (nodeId != null) result.nodeId = nodeId;
    return result;
  }

  SwarmRemoveNodeRequest._();

  factory SwarmRemoveNodeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmRemoveNodeRequest()..mergeFromBuffer(data, registry);
  factory SwarmRemoveNodeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmRemoveNodeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmRemoveNodeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmRemoveNodeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'nodeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmRemoveNodeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmRemoveNodeRequest copyWith(
          void Function(SwarmRemoveNodeRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmRemoveNodeRequest))
          as SwarmRemoveNodeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SwarmRemoveNodeRequest() / SwarmRemoveNodeRequest.new instead')
  static SwarmRemoveNodeRequest create() => SwarmRemoveNodeRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmRemoveNodeRequest._();
  @$core.override
  SwarmRemoveNodeRequest createEmptyInstance() => SwarmRemoveNodeRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmRemoveNodeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SwarmRemoveNodeRequest>(
          SwarmRemoveNodeRequest.$_createMessage);
  static SwarmRemoveNodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get nodeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set nodeId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNodeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNodeId() => $_clearField(2);
}

/// Force is required when a manager leaves; it resets the node so it can join another cluster.
class SwarmLeaveRequest extends $pb.GeneratedMessage {
  factory SwarmLeaveRequest({
    $fixnum.Int64? serverId,
    $core.bool? force,
  }) {
    final result = SwarmLeaveRequest._();
    if (serverId != null) result.serverId = serverId;
    if (force != null) result.force = force;
    return result;
  }

  SwarmLeaveRequest._();

  factory SwarmLeaveRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmLeaveRequest()..mergeFromBuffer(data, registry);
  factory SwarmLeaveRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmLeaveRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmLeaveRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmLeaveRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOB(2, _omitFieldNames ? '' : 'force')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmLeaveRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmLeaveRequest copyWith(void Function(SwarmLeaveRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmLeaveRequest))
          as SwarmLeaveRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmLeaveRequest() / SwarmLeaveRequest.new instead')
  static SwarmLeaveRequest create() => SwarmLeaveRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmLeaveRequest._();
  @$core.override
  SwarmLeaveRequest createEmptyInstance() => SwarmLeaveRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmLeaveRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmLeaveRequest>(
          SwarmLeaveRequest.$_createMessage);
  static SwarmLeaveRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get force => $_getBF(1);
  @$pb.TagNumber(2)
  set force($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasForce() => $_has(1);
  @$pb.TagNumber(2)
  void clearForce() => $_clearField(2);
}

/// Joins one server into another server's cluster in one shot: the target force-leaves any standalone swarm first, then joins with a VPN-aware advertise address.
class SwarmJoinRequest extends $pb.GeneratedMessage {
  factory SwarmJoinRequest({
    $fixnum.Int64? targetServerId,
    $fixnum.Int64? managerServerId,
    SwarmRole? role,
  }) {
    final result = SwarmJoinRequest._();
    if (targetServerId != null) result.targetServerId = targetServerId;
    if (managerServerId != null) result.managerServerId = managerServerId;
    if (role != null) result.role = role;
    return result;
  }

  SwarmJoinRequest._();

  factory SwarmJoinRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmJoinRequest()..mergeFromBuffer(data, registry);
  factory SwarmJoinRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SwarmJoinRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SwarmJoinRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SwarmJoinRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'targetServerId')
    ..aInt64(2, _omitFieldNames ? '' : 'managerServerId')
    ..aE<SwarmRole>(3, _omitFieldNames ? '' : 'role',
        enumValues: SwarmRole.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmJoinRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SwarmJoinRequest copyWith(void Function(SwarmJoinRequest) updates) =>
      super.copyWith((message) => updates(message as SwarmJoinRequest))
          as SwarmJoinRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SwarmJoinRequest() / SwarmJoinRequest.new instead')
  static SwarmJoinRequest create() => SwarmJoinRequest._();
  static $pb.GeneratedMessage $_createMessage() => SwarmJoinRequest._();
  @$core.override
  SwarmJoinRequest createEmptyInstance() => SwarmJoinRequest._();
  @$core.pragma('dart2js:noInline')
  static SwarmJoinRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SwarmJoinRequest>(
          SwarmJoinRequest.$_createMessage);
  static SwarmJoinRequest? _defaultInstance;

  /// Server that should join the cluster.
  @$pb.TagNumber(1)
  $fixnum.Int64 get targetServerId => $_getI64(0);
  @$pb.TagNumber(1)
  set targetServerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTargetServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetServerId() => $_clearField(1);

  /// Server whose cluster to join; absent means the local engine's cluster.
  @$pb.TagNumber(2)
  $fixnum.Int64 get managerServerId => $_getI64(1);
  @$pb.TagNumber(2)
  set managerServerId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasManagerServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearManagerServerId() => $_clearField(2);

  @$pb.TagNumber(3)
  SwarmRole get role => $_getN(2);
  @$pb.TagNumber(3)
  set role(SwarmRole value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearRole() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

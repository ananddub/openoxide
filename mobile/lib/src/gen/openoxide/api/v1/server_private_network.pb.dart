// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_private_network.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'server_private_network.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'server_private_network.pbenum.dart';

class ServerPrivateNetwork extends $pb.GeneratedMessage {
  factory ServerPrivateNetwork({
    $fixnum.Int64? serverId,
    ServerConnectionMode? connectionMode,
    ServerPrivateNetworkProvider? provider,
    $core.String? privateHost,
    $core.String? tunnelAddress,
    $core.String? publicKey,
    $core.String? endpoint,
    $core.int? listenPort,
    $core.int? persistentKeepalive,
    ServerPrivateNetworkStatus? status,
    $fixnum.Int64? lastHandshakeAt,
    $fixnum.Int64? configVersion,
    $core.String? dnsName,
    $core.Iterable<$core.String>? routes,
    ServerPrivateNetworkHealthStatus? healthStatus,
    $core.String? healthError,
    $fixnum.Int64? lastHealthCheckAt,
    $fixnum.Int64? consecutiveFailures,
    ServerPrivateNetworkRotationState? rotationState,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ServerPrivateNetwork._();
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
    if (status != null) result.status = status;
    if (lastHandshakeAt != null) result.lastHandshakeAt = lastHandshakeAt;
    if (configVersion != null) result.configVersion = configVersion;
    if (dnsName != null) result.dnsName = dnsName;
    if (routes != null) result.routes.addAll(routes);
    if (healthStatus != null) result.healthStatus = healthStatus;
    if (healthError != null) result.healthError = healthError;
    if (lastHealthCheckAt != null) result.lastHealthCheckAt = lastHealthCheckAt;
    if (consecutiveFailures != null)
      result.consecutiveFailures = consecutiveFailures;
    if (rotationState != null) result.rotationState = rotationState;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ServerPrivateNetwork._();

  factory ServerPrivateNetwork.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetwork()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetwork.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetwork()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetwork',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetwork.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aE<ServerConnectionMode>(2, _omitFieldNames ? '' : 'connectionMode',
        enumValues: ServerConnectionMode.values)
    ..aE<ServerPrivateNetworkProvider>(3, _omitFieldNames ? '' : 'provider',
        enumValues: ServerPrivateNetworkProvider.values)
    ..aOS(4, _omitFieldNames ? '' : 'privateHost')
    ..aOS(5, _omitFieldNames ? '' : 'tunnelAddress')
    ..aOS(6, _omitFieldNames ? '' : 'publicKey')
    ..aOS(7, _omitFieldNames ? '' : 'endpoint')
    ..aI(8, _omitFieldNames ? '' : 'listenPort', fieldType: $pb.PbFieldType.OU3)
    ..aI(9, _omitFieldNames ? '' : 'persistentKeepalive',
        fieldType: $pb.PbFieldType.OU3)
    ..aE<ServerPrivateNetworkStatus>(10, _omitFieldNames ? '' : 'status',
        enumValues: ServerPrivateNetworkStatus.values)
    ..aInt64(11, _omitFieldNames ? '' : 'lastHandshakeAt')
    ..aInt64(12, _omitFieldNames ? '' : 'configVersion')
    ..aOS(13, _omitFieldNames ? '' : 'dnsName')
    ..pPS(14, _omitFieldNames ? '' : 'routes')
    ..aE<ServerPrivateNetworkHealthStatus>(
        15, _omitFieldNames ? '' : 'healthStatus',
        enumValues: ServerPrivateNetworkHealthStatus.values)
    ..aOS(16, _omitFieldNames ? '' : 'healthError')
    ..aInt64(17, _omitFieldNames ? '' : 'lastHealthCheckAt')
    ..aInt64(18, _omitFieldNames ? '' : 'consecutiveFailures')
    ..aE<ServerPrivateNetworkRotationState>(
        19, _omitFieldNames ? '' : 'rotationState',
        enumValues: ServerPrivateNetworkRotationState.values)
    ..aInt64(20, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetwork clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetwork copyWith(void Function(ServerPrivateNetwork) updates) =>
      super.copyWith((message) => updates(message as ServerPrivateNetwork))
          as ServerPrivateNetwork;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetwork() / ServerPrivateNetwork.new instead')
  static ServerPrivateNetwork create() => ServerPrivateNetwork._();
  static $pb.GeneratedMessage $_createMessage() => ServerPrivateNetwork._();
  @$core.override
  ServerPrivateNetwork createEmptyInstance() => ServerPrivateNetwork._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetwork getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetwork>(
          ServerPrivateNetwork.$_createMessage);
  static ServerPrivateNetwork? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  ServerConnectionMode get connectionMode => $_getN(1);
  @$pb.TagNumber(2)
  set connectionMode(ServerConnectionMode value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasConnectionMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearConnectionMode() => $_clearField(2);

  @$pb.TagNumber(3)
  ServerPrivateNetworkProvider get provider => $_getN(2);
  @$pb.TagNumber(3)
  set provider(ServerPrivateNetworkProvider value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  /// Routable private address the panel connects to when the tunnel is active.
  @$pb.TagNumber(4)
  $core.String get privateHost => $_getSZ(3);
  @$pb.TagNumber(4)
  set privateHost($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivateHost() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivateHost() => $_clearField(4);

  /// Tunnel interface address in CIDR notation.
  @$pb.TagNumber(5)
  $core.String get tunnelAddress => $_getSZ(4);
  @$pb.TagNumber(5)
  set tunnelAddress($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTunnelAddress() => $_has(4);
  @$pb.TagNumber(5)
  void clearTunnelAddress() => $_clearField(5);

  /// WireGuard public key (base64 key material); private keys never leave the panel.
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
  ServerPrivateNetworkStatus get status => $_getN(9);
  @$pb.TagNumber(10)
  set status(ServerPrivateNetworkStatus value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasStatus() => $_has(9);
  @$pb.TagNumber(10)
  void clearStatus() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get lastHandshakeAt => $_getI64(10);
  @$pb.TagNumber(11)
  set lastHandshakeAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLastHandshakeAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearLastHandshakeAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get configVersion => $_getI64(11);
  @$pb.TagNumber(12)
  set configVersion($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasConfigVersion() => $_has(11);
  @$pb.TagNumber(12)
  void clearConfigVersion() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get dnsName => $_getSZ(12);
  @$pb.TagNumber(13)
  set dnsName($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasDnsName() => $_has(12);
  @$pb.TagNumber(13)
  void clearDnsName() => $_clearField(13);

  /// Extra CIDR routes pushed through the tunnel.
  @$pb.TagNumber(14)
  $pb.PbList<$core.String> get routes => $_getList(13);

  @$pb.TagNumber(15)
  ServerPrivateNetworkHealthStatus get healthStatus => $_getN(14);
  @$pb.TagNumber(15)
  set healthStatus(ServerPrivateNetworkHealthStatus value) =>
      $_setField(15, value);
  @$pb.TagNumber(15)
  $core.bool hasHealthStatus() => $_has(14);
  @$pb.TagNumber(15)
  void clearHealthStatus() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get healthError => $_getSZ(15);
  @$pb.TagNumber(16)
  set healthError($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasHealthError() => $_has(15);
  @$pb.TagNumber(16)
  void clearHealthError() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get lastHealthCheckAt => $_getI64(16);
  @$pb.TagNumber(17)
  set lastHealthCheckAt($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasLastHealthCheckAt() => $_has(16);
  @$pb.TagNumber(17)
  void clearLastHealthCheckAt() => $_clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get consecutiveFailures => $_getI64(17);
  @$pb.TagNumber(18)
  set consecutiveFailures($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasConsecutiveFailures() => $_has(17);
  @$pb.TagNumber(18)
  void clearConsecutiveFailures() => $_clearField(18);

  @$pb.TagNumber(19)
  ServerPrivateNetworkRotationState get rotationState => $_getN(18);
  @$pb.TagNumber(19)
  set rotationState(ServerPrivateNetworkRotationState value) =>
      $_setField(19, value);
  @$pb.TagNumber(19)
  $core.bool hasRotationState() => $_has(18);
  @$pb.TagNumber(19)
  void clearRotationState() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get updatedAt => $_getI64(19);
  @$pb.TagNumber(20)
  set updatedAt($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasUpdatedAt() => $_has(19);
  @$pb.TagNumber(20)
  void clearUpdatedAt() => $_clearField(20);
}

class ServerPrivateNetworkHealth extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkHealth({
    ServerPrivateNetworkHealthStatus? status,
    $core.bool? interfaceExists,
    $core.bool? privateSshReachable,
    $fixnum.Int64? latestHandshakeAt,
    $fixnum.Int64? checkedAt,
    $core.String? error,
  }) {
    final result = ServerPrivateNetworkHealth._();
    if (status != null) result.status = status;
    if (interfaceExists != null) result.interfaceExists = interfaceExists;
    if (privateSshReachable != null)
      result.privateSshReachable = privateSshReachable;
    if (latestHandshakeAt != null) result.latestHandshakeAt = latestHandshakeAt;
    if (checkedAt != null) result.checkedAt = checkedAt;
    if (error != null) result.error = error;
    return result;
  }

  ServerPrivateNetworkHealth._();

  factory ServerPrivateNetworkHealth.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealth()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkHealth.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealth()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkHealth',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkHealth.$_createMessage)
    ..aE<ServerPrivateNetworkHealthStatus>(1, _omitFieldNames ? '' : 'status',
        enumValues: ServerPrivateNetworkHealthStatus.values)
    ..aOB(2, _omitFieldNames ? '' : 'interfaceExists')
    ..aOB(3, _omitFieldNames ? '' : 'privateSshReachable')
    ..aInt64(4, _omitFieldNames ? '' : 'latestHandshakeAt')
    ..aInt64(5, _omitFieldNames ? '' : 'checkedAt')
    ..aOS(6, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealth clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealth copyWith(
          void Function(ServerPrivateNetworkHealth) updates) =>
      super.copyWith(
              (message) => updates(message as ServerPrivateNetworkHealth))
          as ServerPrivateNetworkHealth;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkHealth() / ServerPrivateNetworkHealth.new instead')
  static ServerPrivateNetworkHealth create() => ServerPrivateNetworkHealth._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkHealth._();
  @$core.override
  ServerPrivateNetworkHealth createEmptyInstance() =>
      ServerPrivateNetworkHealth._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkHealth getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkHealth>(
          ServerPrivateNetworkHealth.$_createMessage);
  static ServerPrivateNetworkHealth? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetworkHealthStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(ServerPrivateNetworkHealthStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get interfaceExists => $_getBF(1);
  @$pb.TagNumber(2)
  set interfaceExists($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasInterfaceExists() => $_has(1);
  @$pb.TagNumber(2)
  void clearInterfaceExists() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get privateSshReachable => $_getBF(2);
  @$pb.TagNumber(3)
  set privateSshReachable($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPrivateSshReachable() => $_has(2);
  @$pb.TagNumber(3)
  void clearPrivateSshReachable() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get latestHandshakeAt => $_getI64(3);
  @$pb.TagNumber(4)
  set latestHandshakeAt($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLatestHandshakeAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearLatestHandshakeAt() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get checkedAt => $_getI64(4);
  @$pb.TagNumber(5)
  set checkedAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCheckedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCheckedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get error => $_getSZ(5);
  @$pb.TagNumber(6)
  set error($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasError() => $_has(5);
  @$pb.TagNumber(6)
  void clearError() => $_clearField(6);
}

class ServerPrivateNetworkGetRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkGetRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkGetRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkGetRequest._();

  factory ServerPrivateNetworkGetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkGetRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkGetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkGetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkGetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkGetRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkGetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkGetRequest copyWith(
          void Function(ServerPrivateNetworkGetRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerPrivateNetworkGetRequest))
          as ServerPrivateNetworkGetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkGetRequest() / ServerPrivateNetworkGetRequest.new instead')
  static ServerPrivateNetworkGetRequest create() =>
      ServerPrivateNetworkGetRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkGetRequest._();
  @$core.override
  ServerPrivateNetworkGetRequest createEmptyInstance() =>
      ServerPrivateNetworkGetRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkGetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkGetRequest>(
          ServerPrivateNetworkGetRequest.$_createMessage);
  static ServerPrivateNetworkGetRequest? _defaultInstance;

  /// Server id; the network is keyed by its owning server.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// `network` is absent when the server has no private-network configuration.
class ServerPrivateNetworkGetResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkGetResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkGetResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkGetResponse._();

  factory ServerPrivateNetworkGetResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkGetResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkGetResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkGetResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkGetResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkGetResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkGetResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkGetResponse copyWith(
          void Function(ServerPrivateNetworkGetResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ServerPrivateNetworkGetResponse))
          as ServerPrivateNetworkGetResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkGetResponse() / ServerPrivateNetworkGetResponse.new instead')
  static ServerPrivateNetworkGetResponse create() =>
      ServerPrivateNetworkGetResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkGetResponse._();
  @$core.override
  ServerPrivateNetworkGetResponse createEmptyInstance() =>
      ServerPrivateNetworkGetResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkGetResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkGetResponse>(
          ServerPrivateNetworkGetResponse.$_createMessage);
  static ServerPrivateNetworkGetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkUpdateRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkUpdateRequest({
    $fixnum.Int64? id,
    ServerConnectionMode? connectionMode,
    ServerPrivateNetworkProvider? provider,
    $core.String? privateHost,
    $core.String? tunnelAddress,
    $core.String? publicKey,
    $core.String? endpoint,
    $core.int? listenPort,
    $core.int? persistentKeepalive,
    $core.String? dnsName,
    $core.Iterable<$core.String>? routes,
  }) {
    final result = ServerPrivateNetworkUpdateRequest._();
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

  ServerPrivateNetworkUpdateRequest._();

  factory ServerPrivateNetworkUpdateRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkUpdateRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkUpdateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkUpdateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkUpdateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkUpdateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<ServerConnectionMode>(2, _omitFieldNames ? '' : 'connectionMode',
        enumValues: ServerConnectionMode.values)
    ..aE<ServerPrivateNetworkProvider>(3, _omitFieldNames ? '' : 'provider',
        enumValues: ServerPrivateNetworkProvider.values)
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
  ServerPrivateNetworkUpdateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkUpdateRequest copyWith(
          void Function(ServerPrivateNetworkUpdateRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkUpdateRequest))
          as ServerPrivateNetworkUpdateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkUpdateRequest() / ServerPrivateNetworkUpdateRequest.new instead')
  static ServerPrivateNetworkUpdateRequest create() =>
      ServerPrivateNetworkUpdateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkUpdateRequest._();
  @$core.override
  ServerPrivateNetworkUpdateRequest createEmptyInstance() =>
      ServerPrivateNetworkUpdateRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkUpdateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkUpdateRequest>(
          ServerPrivateNetworkUpdateRequest.$_createMessage);
  static ServerPrivateNetworkUpdateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ServerConnectionMode get connectionMode => $_getN(1);
  @$pb.TagNumber(2)
  set connectionMode(ServerConnectionMode value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasConnectionMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearConnectionMode() => $_clearField(2);

  @$pb.TagNumber(3)
  ServerPrivateNetworkProvider get provider => $_getN(2);
  @$pb.TagNumber(3)
  set provider(ServerPrivateNetworkProvider value) => $_setField(3, value);
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

class ServerPrivateNetworkUpdateResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkUpdateResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkUpdateResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkUpdateResponse._();

  factory ServerPrivateNetworkUpdateResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkUpdateResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkUpdateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkUpdateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkUpdateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkUpdateResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkUpdateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkUpdateResponse copyWith(
          void Function(ServerPrivateNetworkUpdateResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkUpdateResponse))
          as ServerPrivateNetworkUpdateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkUpdateResponse() / ServerPrivateNetworkUpdateResponse.new instead')
  static ServerPrivateNetworkUpdateResponse create() =>
      ServerPrivateNetworkUpdateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkUpdateResponse._();
  @$core.override
  ServerPrivateNetworkUpdateResponse createEmptyInstance() =>
      ServerPrivateNetworkUpdateResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkUpdateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkUpdateResponse>(
          ServerPrivateNetworkUpdateResponse.$_createMessage);
  static ServerPrivateNetworkUpdateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkDeleteRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkDeleteRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkDeleteRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkDeleteRequest._();

  factory ServerPrivateNetworkDeleteRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkDeleteRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkDeleteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkDeleteRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkDeleteRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkDeleteRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkDeleteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkDeleteRequest copyWith(
          void Function(ServerPrivateNetworkDeleteRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkDeleteRequest))
          as ServerPrivateNetworkDeleteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkDeleteRequest() / ServerPrivateNetworkDeleteRequest.new instead')
  static ServerPrivateNetworkDeleteRequest create() =>
      ServerPrivateNetworkDeleteRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkDeleteRequest._();
  @$core.override
  ServerPrivateNetworkDeleteRequest createEmptyInstance() =>
      ServerPrivateNetworkDeleteRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkDeleteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkDeleteRequest>(
          ServerPrivateNetworkDeleteRequest.$_createMessage);
  static ServerPrivateNetworkDeleteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkSetupRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkSetupRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkSetupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkSetupRequest._();

  factory ServerPrivateNetworkSetupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkSetupRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkSetupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkSetupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkSetupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkSetupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkSetupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkSetupRequest copyWith(
          void Function(ServerPrivateNetworkSetupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ServerPrivateNetworkSetupRequest))
          as ServerPrivateNetworkSetupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkSetupRequest() / ServerPrivateNetworkSetupRequest.new instead')
  static ServerPrivateNetworkSetupRequest create() =>
      ServerPrivateNetworkSetupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkSetupRequest._();
  @$core.override
  ServerPrivateNetworkSetupRequest createEmptyInstance() =>
      ServerPrivateNetworkSetupRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkSetupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkSetupRequest>(
          ServerPrivateNetworkSetupRequest.$_createMessage);
  static ServerPrivateNetworkSetupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkSetupResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkSetupResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkSetupResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkSetupResponse._();

  factory ServerPrivateNetworkSetupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkSetupResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkSetupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkSetupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkSetupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkSetupResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkSetupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkSetupResponse copyWith(
          void Function(ServerPrivateNetworkSetupResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkSetupResponse))
          as ServerPrivateNetworkSetupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkSetupResponse() / ServerPrivateNetworkSetupResponse.new instead')
  static ServerPrivateNetworkSetupResponse create() =>
      ServerPrivateNetworkSetupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkSetupResponse._();
  @$core.override
  ServerPrivateNetworkSetupResponse createEmptyInstance() =>
      ServerPrivateNetworkSetupResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkSetupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkSetupResponse>(
          ServerPrivateNetworkSetupResponse.$_createMessage);
  static ServerPrivateNetworkSetupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkHealthRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkHealthRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkHealthRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkHealthRequest._();

  factory ServerPrivateNetworkHealthRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealthRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkHealthRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealthRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkHealthRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkHealthRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealthRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealthRequest copyWith(
          void Function(ServerPrivateNetworkHealthRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkHealthRequest))
          as ServerPrivateNetworkHealthRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkHealthRequest() / ServerPrivateNetworkHealthRequest.new instead')
  static ServerPrivateNetworkHealthRequest create() =>
      ServerPrivateNetworkHealthRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkHealthRequest._();
  @$core.override
  ServerPrivateNetworkHealthRequest createEmptyInstance() =>
      ServerPrivateNetworkHealthRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkHealthRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkHealthRequest>(
          ServerPrivateNetworkHealthRequest.$_createMessage);
  static ServerPrivateNetworkHealthRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkHealthResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkHealthResponse({
    ServerPrivateNetworkHealth? health,
  }) {
    final result = ServerPrivateNetworkHealthResponse._();
    if (health != null) result.health = health;
    return result;
  }

  ServerPrivateNetworkHealthResponse._();

  factory ServerPrivateNetworkHealthResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealthResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkHealthResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkHealthResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkHealthResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkHealthResponse.$_createMessage)
    ..aOM<ServerPrivateNetworkHealth>(1, _omitFieldNames ? '' : 'health',
        subBuilder: ServerPrivateNetworkHealth.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealthResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkHealthResponse copyWith(
          void Function(ServerPrivateNetworkHealthResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkHealthResponse))
          as ServerPrivateNetworkHealthResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkHealthResponse() / ServerPrivateNetworkHealthResponse.new instead')
  static ServerPrivateNetworkHealthResponse create() =>
      ServerPrivateNetworkHealthResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkHealthResponse._();
  @$core.override
  ServerPrivateNetworkHealthResponse createEmptyInstance() =>
      ServerPrivateNetworkHealthResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkHealthResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkHealthResponse>(
          ServerPrivateNetworkHealthResponse.$_createMessage);
  static ServerPrivateNetworkHealthResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetworkHealth get health => $_getN(0);
  @$pb.TagNumber(1)
  set health(ServerPrivateNetworkHealth value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHealth() => $_has(0);
  @$pb.TagNumber(1)
  void clearHealth() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetworkHealth ensureHealth() => $_ensure(0);
}

class ServerPrivateNetworkRepairRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkRepairRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkRepairRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkRepairRequest._();

  factory ServerPrivateNetworkRepairRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRepairRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkRepairRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRepairRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkRepairRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkRepairRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRepairRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRepairRequest copyWith(
          void Function(ServerPrivateNetworkRepairRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkRepairRequest))
          as ServerPrivateNetworkRepairRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkRepairRequest() / ServerPrivateNetworkRepairRequest.new instead')
  static ServerPrivateNetworkRepairRequest create() =>
      ServerPrivateNetworkRepairRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkRepairRequest._();
  @$core.override
  ServerPrivateNetworkRepairRequest createEmptyInstance() =>
      ServerPrivateNetworkRepairRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkRepairRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkRepairRequest>(
          ServerPrivateNetworkRepairRequest.$_createMessage);
  static ServerPrivateNetworkRepairRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkRepairResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkRepairResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkRepairResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkRepairResponse._();

  factory ServerPrivateNetworkRepairResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRepairResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkRepairResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRepairResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkRepairResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkRepairResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRepairResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRepairResponse copyWith(
          void Function(ServerPrivateNetworkRepairResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkRepairResponse))
          as ServerPrivateNetworkRepairResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkRepairResponse() / ServerPrivateNetworkRepairResponse.new instead')
  static ServerPrivateNetworkRepairResponse create() =>
      ServerPrivateNetworkRepairResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkRepairResponse._();
  @$core.override
  ServerPrivateNetworkRepairResponse createEmptyInstance() =>
      ServerPrivateNetworkRepairResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkRepairResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkRepairResponse>(
          ServerPrivateNetworkRepairResponse.$_createMessage);
  static ServerPrivateNetworkRepairResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkReSetupRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkReSetupRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkReSetupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkReSetupRequest._();

  factory ServerPrivateNetworkReSetupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkReSetupRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkReSetupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkReSetupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkReSetupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkReSetupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkReSetupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkReSetupRequest copyWith(
          void Function(ServerPrivateNetworkReSetupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkReSetupRequest))
          as ServerPrivateNetworkReSetupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkReSetupRequest() / ServerPrivateNetworkReSetupRequest.new instead')
  static ServerPrivateNetworkReSetupRequest create() =>
      ServerPrivateNetworkReSetupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkReSetupRequest._();
  @$core.override
  ServerPrivateNetworkReSetupRequest createEmptyInstance() =>
      ServerPrivateNetworkReSetupRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkReSetupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerPrivateNetworkReSetupRequest>(
          ServerPrivateNetworkReSetupRequest.$_createMessage);
  static ServerPrivateNetworkReSetupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkReSetupResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkReSetupResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkReSetupResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkReSetupResponse._();

  factory ServerPrivateNetworkReSetupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkReSetupResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkReSetupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkReSetupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkReSetupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkReSetupResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkReSetupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkReSetupResponse copyWith(
          void Function(ServerPrivateNetworkReSetupResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkReSetupResponse))
          as ServerPrivateNetworkReSetupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkReSetupResponse() / ServerPrivateNetworkReSetupResponse.new instead')
  static ServerPrivateNetworkReSetupResponse create() =>
      ServerPrivateNetworkReSetupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkReSetupResponse._();
  @$core.override
  ServerPrivateNetworkReSetupResponse createEmptyInstance() =>
      ServerPrivateNetworkReSetupResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkReSetupResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerPrivateNetworkReSetupResponse>(
          ServerPrivateNetworkReSetupResponse.$_createMessage);
  static ServerPrivateNetworkReSetupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkRotateKeysRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkRotateKeysRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkRotateKeysRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkRotateKeysRequest._();

  factory ServerPrivateNetworkRotateKeysRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRotateKeysRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkRotateKeysRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRotateKeysRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkRotateKeysRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerPrivateNetworkRotateKeysRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRotateKeysRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRotateKeysRequest copyWith(
          void Function(ServerPrivateNetworkRotateKeysRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkRotateKeysRequest))
          as ServerPrivateNetworkRotateKeysRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkRotateKeysRequest() / ServerPrivateNetworkRotateKeysRequest.new instead')
  static ServerPrivateNetworkRotateKeysRequest create() =>
      ServerPrivateNetworkRotateKeysRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkRotateKeysRequest._();
  @$core.override
  ServerPrivateNetworkRotateKeysRequest createEmptyInstance() =>
      ServerPrivateNetworkRotateKeysRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkRotateKeysRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerPrivateNetworkRotateKeysRequest>(
          ServerPrivateNetworkRotateKeysRequest.$_createMessage);
  static ServerPrivateNetworkRotateKeysRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ServerPrivateNetworkRotateKeysResponse extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkRotateKeysResponse({
    ServerPrivateNetwork? network,
  }) {
    final result = ServerPrivateNetworkRotateKeysResponse._();
    if (network != null) result.network = network;
    return result;
  }

  ServerPrivateNetworkRotateKeysResponse._();

  factory ServerPrivateNetworkRotateKeysResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRotateKeysResponse()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkRotateKeysResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkRotateKeysResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkRotateKeysResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ServerPrivateNetworkRotateKeysResponse.$_createMessage)
    ..aOM<ServerPrivateNetwork>(1, _omitFieldNames ? '' : 'network',
        subBuilder: ServerPrivateNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRotateKeysResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkRotateKeysResponse copyWith(
          void Function(ServerPrivateNetworkRotateKeysResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkRotateKeysResponse))
          as ServerPrivateNetworkRotateKeysResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkRotateKeysResponse() / ServerPrivateNetworkRotateKeysResponse.new instead')
  static ServerPrivateNetworkRotateKeysResponse create() =>
      ServerPrivateNetworkRotateKeysResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkRotateKeysResponse._();
  @$core.override
  ServerPrivateNetworkRotateKeysResponse createEmptyInstance() =>
      ServerPrivateNetworkRotateKeysResponse._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkRotateKeysResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerPrivateNetworkRotateKeysResponse>(
          ServerPrivateNetworkRotateKeysResponse.$_createMessage);
  static ServerPrivateNetworkRotateKeysResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ServerPrivateNetwork get network => $_getN(0);
  @$pb.TagNumber(1)
  set network(ServerPrivateNetwork value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNetwork() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetwork() => $_clearField(1);
  @$pb.TagNumber(1)
  ServerPrivateNetwork ensureNetwork() => $_ensure(0);
}

class ServerPrivateNetworkTeardownRequest extends $pb.GeneratedMessage {
  factory ServerPrivateNetworkTeardownRequest({
    $fixnum.Int64? id,
  }) {
    final result = ServerPrivateNetworkTeardownRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ServerPrivateNetworkTeardownRequest._();

  factory ServerPrivateNetworkTeardownRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkTeardownRequest()..mergeFromBuffer(data, registry);
  factory ServerPrivateNetworkTeardownRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerPrivateNetworkTeardownRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerPrivateNetworkTeardownRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServerPrivateNetworkTeardownRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkTeardownRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerPrivateNetworkTeardownRequest copyWith(
          void Function(ServerPrivateNetworkTeardownRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ServerPrivateNetworkTeardownRequest))
          as ServerPrivateNetworkTeardownRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServerPrivateNetworkTeardownRequest() / ServerPrivateNetworkTeardownRequest.new instead')
  static ServerPrivateNetworkTeardownRequest create() =>
      ServerPrivateNetworkTeardownRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ServerPrivateNetworkTeardownRequest._();
  @$core.override
  ServerPrivateNetworkTeardownRequest createEmptyInstance() =>
      ServerPrivateNetworkTeardownRequest._();
  @$core.pragma('dart2js:noInline')
  static ServerPrivateNetworkTeardownRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ServerPrivateNetworkTeardownRequest>(
          ServerPrivateNetworkTeardownRequest.$_createMessage);
  static ServerPrivateNetworkTeardownRequest? _defaultInstance;

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

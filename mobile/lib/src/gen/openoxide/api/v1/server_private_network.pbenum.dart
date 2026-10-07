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

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set: `string_enum!` PrivateNetworkConnectionMode.
class ServerConnectionMode extends $pb.ProtobufEnum {
  static const ServerConnectionMode SERVER_CONNECTION_MODE_UNSPECIFIED =
      ServerConnectionMode._(
          0, _omitEnumNames ? '' : 'SERVER_CONNECTION_MODE_UNSPECIFIED');
  static const ServerConnectionMode SERVER_CONNECTION_MODE_DIRECT_SSH =
      ServerConnectionMode._(
          1, _omitEnumNames ? '' : 'SERVER_CONNECTION_MODE_DIRECT_SSH');
  static const ServerConnectionMode SERVER_CONNECTION_MODE_MANAGED_WIREGUARD =
      ServerConnectionMode._(
          2, _omitEnumNames ? '' : 'SERVER_CONNECTION_MODE_MANAGED_WIREGUARD');
  static const ServerConnectionMode
      SERVER_CONNECTION_MODE_EXTERNAL_PRIVATE_NETWORK = ServerConnectionMode._(
          3,
          _omitEnumNames
              ? ''
              : 'SERVER_CONNECTION_MODE_EXTERNAL_PRIVATE_NETWORK');

  static const $core.List<ServerConnectionMode> values = <ServerConnectionMode>[
    SERVER_CONNECTION_MODE_UNSPECIFIED,
    SERVER_CONNECTION_MODE_DIRECT_SSH,
    SERVER_CONNECTION_MODE_MANAGED_WIREGUARD,
    SERVER_CONNECTION_MODE_EXTERNAL_PRIVATE_NETWORK,
  ];

  static final $core.List<ServerConnectionMode?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static ServerConnectionMode? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerConnectionMode._(super.value, super.name);
}

/// Closed set: `string_enum!` PrivateNetworkProvider.
class ServerPrivateNetworkProvider extends $pb.ProtobufEnum {
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_UNSPECIFIED =
      ServerPrivateNetworkProvider._(0,
          _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_UNSPECIFIED');
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_WIREGUARD =
      ServerPrivateNetworkProvider._(
          1, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_WIREGUARD');
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_TAILSCALE =
      ServerPrivateNetworkProvider._(
          2, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_TAILSCALE');
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_ZEROTIER = ServerPrivateNetworkProvider._(
          3, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_ZEROTIER');
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_NETBIRD = ServerPrivateNetworkProvider._(
          4, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_NETBIRD');
  static const ServerPrivateNetworkProvider
      SERVER_PRIVATE_NETWORK_PROVIDER_CUSTOM = ServerPrivateNetworkProvider._(
          5, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_PROVIDER_CUSTOM');

  static const $core.List<ServerPrivateNetworkProvider> values =
      <ServerPrivateNetworkProvider>[
    SERVER_PRIVATE_NETWORK_PROVIDER_UNSPECIFIED,
    SERVER_PRIVATE_NETWORK_PROVIDER_WIREGUARD,
    SERVER_PRIVATE_NETWORK_PROVIDER_TAILSCALE,
    SERVER_PRIVATE_NETWORK_PROVIDER_ZEROTIER,
    SERVER_PRIVATE_NETWORK_PROVIDER_NETBIRD,
    SERVER_PRIVATE_NETWORK_PROVIDER_CUSTOM,
  ];

  static final $core.List<ServerPrivateNetworkProvider?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static ServerPrivateNetworkProvider? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerPrivateNetworkProvider._(super.value, super.name);
}

/// Closed set: `string_enum!` PrivateNetworkStatus.
class ServerPrivateNetworkStatus extends $pb.ProtobufEnum {
  static const ServerPrivateNetworkStatus
      SERVER_PRIVATE_NETWORK_STATUS_UNSPECIFIED = ServerPrivateNetworkStatus._(
          0, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_STATUS_UNSPECIFIED');
  static const ServerPrivateNetworkStatus
      SERVER_PRIVATE_NETWORK_STATUS_DISABLED = ServerPrivateNetworkStatus._(
          1, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_STATUS_DISABLED');
  static const ServerPrivateNetworkStatus
      SERVER_PRIVATE_NETWORK_STATUS_CONFIGURING = ServerPrivateNetworkStatus._(
          2, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_STATUS_CONFIGURING');
  static const ServerPrivateNetworkStatus SERVER_PRIVATE_NETWORK_STATUS_ACTIVE =
      ServerPrivateNetworkStatus._(
          3, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_STATUS_ACTIVE');
  static const ServerPrivateNetworkStatus SERVER_PRIVATE_NETWORK_STATUS_FAILED =
      ServerPrivateNetworkStatus._(
          4, _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_STATUS_FAILED');

  static const $core.List<ServerPrivateNetworkStatus> values =
      <ServerPrivateNetworkStatus>[
    SERVER_PRIVATE_NETWORK_STATUS_UNSPECIFIED,
    SERVER_PRIVATE_NETWORK_STATUS_DISABLED,
    SERVER_PRIVATE_NETWORK_STATUS_CONFIGURING,
    SERVER_PRIVATE_NETWORK_STATUS_ACTIVE,
    SERVER_PRIVATE_NETWORK_STATUS_FAILED,
  ];

  static final $core.List<ServerPrivateNetworkStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ServerPrivateNetworkStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerPrivateNetworkStatus._(super.value, super.name);
}

/// Closed set: `string_enum!` PrivateNetworkHealthStatus.
class ServerPrivateNetworkHealthStatus extends $pb.ProtobufEnum {
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNSPECIFIED =
      ServerPrivateNetworkHealthStatus._(
          0,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNSPECIFIED');
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNKNOWN =
      ServerPrivateNetworkHealthStatus._(1,
          _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNKNOWN');
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_HEALTHY =
      ServerPrivateNetworkHealthStatus._(2,
          _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_HEALTHY');
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_DEGRADED =
      ServerPrivateNetworkHealthStatus._(
          3,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_DEGRADED');
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNREACHABLE =
      ServerPrivateNetworkHealthStatus._(
          4,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNREACHABLE');
  static const ServerPrivateNetworkHealthStatus
      SERVER_PRIVATE_NETWORK_HEALTH_STATUS_CONFIG_DRIFT =
      ServerPrivateNetworkHealthStatus._(
          5,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_HEALTH_STATUS_CONFIG_DRIFT');

  static const $core.List<ServerPrivateNetworkHealthStatus> values =
      <ServerPrivateNetworkHealthStatus>[
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNSPECIFIED,
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNKNOWN,
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_HEALTHY,
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_DEGRADED,
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_UNREACHABLE,
    SERVER_PRIVATE_NETWORK_HEALTH_STATUS_CONFIG_DRIFT,
  ];

  static final $core.List<ServerPrivateNetworkHealthStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static ServerPrivateNetworkHealthStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerPrivateNetworkHealthStatus._(super.value, super.name);
}

/// Closed set: `string_enum!` PrivateNetworkRotationState.
class ServerPrivateNetworkRotationState extends $pb.ProtobufEnum {
  static const ServerPrivateNetworkRotationState
      SERVER_PRIVATE_NETWORK_ROTATION_STATE_UNSPECIFIED =
      ServerPrivateNetworkRotationState._(
          0,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_UNSPECIFIED');
  static const ServerPrivateNetworkRotationState
      SERVER_PRIVATE_NETWORK_ROTATION_STATE_IDLE =
      ServerPrivateNetworkRotationState._(1,
          _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_IDLE');
  static const ServerPrivateNetworkRotationState
      SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROTATING =
      ServerPrivateNetworkRotationState._(
          2,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROTATING');
  static const ServerPrivateNetworkRotationState
      SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROLLING_BACK =
      ServerPrivateNetworkRotationState._(
          3,
          _omitEnumNames
              ? ''
              : 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROLLING_BACK');
  static const ServerPrivateNetworkRotationState
      SERVER_PRIVATE_NETWORK_ROTATION_STATE_FAILED =
      ServerPrivateNetworkRotationState._(4,
          _omitEnumNames ? '' : 'SERVER_PRIVATE_NETWORK_ROTATION_STATE_FAILED');

  static const $core.List<ServerPrivateNetworkRotationState> values =
      <ServerPrivateNetworkRotationState>[
    SERVER_PRIVATE_NETWORK_ROTATION_STATE_UNSPECIFIED,
    SERVER_PRIVATE_NETWORK_ROTATION_STATE_IDLE,
    SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROTATING,
    SERVER_PRIVATE_NETWORK_ROTATION_STATE_ROLLING_BACK,
    SERVER_PRIVATE_NETWORK_ROTATION_STATE_FAILED,
  ];

  static final $core.List<ServerPrivateNetworkRotationState?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ServerPrivateNetworkRotationState? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerPrivateNetworkRotationState._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

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

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set: `SetupStep` match arms in the setup outcome mapper.
class ServerSetupStep extends $pb.ProtobufEnum {
  static const ServerSetupStep SERVER_SETUP_STEP_UNSPECIFIED =
      ServerSetupStep._(
          0, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_UNSPECIFIED');
  static const ServerSetupStep SERVER_SETUP_STEP_DEPENDENCIES =
      ServerSetupStep._(
          1, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_DEPENDENCIES');
  static const ServerSetupStep SERVER_SETUP_STEP_BUILD_TOOLS =
      ServerSetupStep._(
          2, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_BUILD_TOOLS');
  static const ServerSetupStep SERVER_SETUP_STEP_DIRECTORIES =
      ServerSetupStep._(
          3, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_DIRECTORIES');
  static const ServerSetupStep SERVER_SETUP_STEP_SWARM =
      ServerSetupStep._(4, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_SWARM');
  static const ServerSetupStep SERVER_SETUP_STEP_NETWORK =
      ServerSetupStep._(5, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_NETWORK');
  static const ServerSetupStep SERVER_SETUP_STEP_TRAEFIK_CONFIG =
      ServerSetupStep._(
          6, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_TRAEFIK_CONFIG');
  static const ServerSetupStep SERVER_SETUP_STEP_TRAEFIK =
      ServerSetupStep._(7, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_TRAEFIK');
  static const ServerSetupStep SERVER_SETUP_STEP_AGENT =
      ServerSetupStep._(8, _omitEnumNames ? '' : 'SERVER_SETUP_STEP_AGENT');

  static const $core.List<ServerSetupStep> values = <ServerSetupStep>[
    SERVER_SETUP_STEP_UNSPECIFIED,
    SERVER_SETUP_STEP_DEPENDENCIES,
    SERVER_SETUP_STEP_BUILD_TOOLS,
    SERVER_SETUP_STEP_DIRECTORIES,
    SERVER_SETUP_STEP_SWARM,
    SERVER_SETUP_STEP_NETWORK,
    SERVER_SETUP_STEP_TRAEFIK_CONFIG,
    SERVER_SETUP_STEP_TRAEFIK,
    SERVER_SETUP_STEP_AGENT,
  ];

  static final $core.List<ServerSetupStep?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 8);
  static ServerSetupStep? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerSetupStep._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

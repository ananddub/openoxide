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

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set: only ACTIVE/INACTIVE are ever written to `servers.server_status`.
class ServerStatus extends $pb.ProtobufEnum {
  static const ServerStatus SERVER_STATUS_UNSPECIFIED =
      ServerStatus._(0, _omitEnumNames ? '' : 'SERVER_STATUS_UNSPECIFIED');
  static const ServerStatus SERVER_STATUS_ACTIVE =
      ServerStatus._(1, _omitEnumNames ? '' : 'SERVER_STATUS_ACTIVE');
  static const ServerStatus SERVER_STATUS_INACTIVE =
      ServerStatus._(2, _omitEnumNames ? '' : 'SERVER_STATUS_INACTIVE');

  static const $core.List<ServerStatus> values = <ServerStatus>[
    SERVER_STATUS_UNSPECIFIED,
    SERVER_STATUS_ACTIVE,
    SERVER_STATUS_INACTIVE,
  ];

  static final $core.List<ServerStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ServerStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerStatus._(super.value, super.name);
}

/// Closed set: `RemoteServerTypeDto` (DEPLOY/BUILD).
class ServerType extends $pb.ProtobufEnum {
  static const ServerType SERVER_TYPE_UNSPECIFIED =
      ServerType._(0, _omitEnumNames ? '' : 'SERVER_TYPE_UNSPECIFIED');
  static const ServerType SERVER_TYPE_DEPLOY =
      ServerType._(1, _omitEnumNames ? '' : 'SERVER_TYPE_DEPLOY');
  static const ServerType SERVER_TYPE_BUILD =
      ServerType._(2, _omitEnumNames ? '' : 'SERVER_TYPE_BUILD');

  static const $core.List<ServerType> values = <ServerType>[
    SERVER_TYPE_UNSPECIFIED,
    SERVER_TYPE_DEPLOY,
    SERVER_TYPE_BUILD,
  ];

  static final $core.List<ServerType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ServerType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

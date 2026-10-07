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

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set: cleanup executions are finished with exactly one of the two literals written by `ServerCleanupService::run`.
class ServerCleanupStatus extends $pb.ProtobufEnum {
  static const ServerCleanupStatus SERVER_CLEANUP_STATUS_UNSPECIFIED =
      ServerCleanupStatus._(
          0, _omitEnumNames ? '' : 'SERVER_CLEANUP_STATUS_UNSPECIFIED');
  static const ServerCleanupStatus SERVER_CLEANUP_STATUS_SUCCEEDED =
      ServerCleanupStatus._(
          1, _omitEnumNames ? '' : 'SERVER_CLEANUP_STATUS_SUCCEEDED');
  static const ServerCleanupStatus SERVER_CLEANUP_STATUS_FAILED =
      ServerCleanupStatus._(
          2, _omitEnumNames ? '' : 'SERVER_CLEANUP_STATUS_FAILED');

  static const $core.List<ServerCleanupStatus> values = <ServerCleanupStatus>[
    SERVER_CLEANUP_STATUS_UNSPECIFIED,
    SERVER_CLEANUP_STATUS_SUCCEEDED,
    SERVER_CLEANUP_STATUS_FAILED,
  ];

  static final $core.List<ServerCleanupStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ServerCleanupStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ServerCleanupStatus._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

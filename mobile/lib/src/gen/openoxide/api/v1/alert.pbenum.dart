// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/alert.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Event lifecycle state (Rust AlertEventState).
class AlertEventState extends $pb.ProtobufEnum {
  static const AlertEventState ALERT_EVENT_STATE_UNSPECIFIED =
      AlertEventState._(
          0, _omitEnumNames ? '' : 'ALERT_EVENT_STATE_UNSPECIFIED');
  static const AlertEventState ALERT_EVENT_STATE_FIRING =
      AlertEventState._(1, _omitEnumNames ? '' : 'ALERT_EVENT_STATE_FIRING');
  static const AlertEventState ALERT_EVENT_STATE_RESOLVED =
      AlertEventState._(2, _omitEnumNames ? '' : 'ALERT_EVENT_STATE_RESOLVED');
  static const AlertEventState ALERT_EVENT_STATE_NO_DATA =
      AlertEventState._(3, _omitEnumNames ? '' : 'ALERT_EVENT_STATE_NO_DATA');

  static const $core.List<AlertEventState> values = <AlertEventState>[
    ALERT_EVENT_STATE_UNSPECIFIED,
    ALERT_EVENT_STATE_FIRING,
    ALERT_EVENT_STATE_RESOLVED,
    ALERT_EVENT_STATE_NO_DATA,
  ];

  static final $core.List<AlertEventState?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static AlertEventState? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AlertEventState._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

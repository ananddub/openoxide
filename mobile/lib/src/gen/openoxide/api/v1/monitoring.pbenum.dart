// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/monitoring.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Derived from the agent heartbeat: ONLINE within 30s, OFFLINE once seen, NEVER_SEEN when no heartbeat was ever recorded.
class MonitoringAgentState extends $pb.ProtobufEnum {
  static const MonitoringAgentState MONITORING_AGENT_STATE_UNSPECIFIED =
      MonitoringAgentState._(
          0, _omitEnumNames ? '' : 'MONITORING_AGENT_STATE_UNSPECIFIED');
  static const MonitoringAgentState MONITORING_AGENT_STATE_ONLINE =
      MonitoringAgentState._(
          1, _omitEnumNames ? '' : 'MONITORING_AGENT_STATE_ONLINE');
  static const MonitoringAgentState MONITORING_AGENT_STATE_OFFLINE =
      MonitoringAgentState._(
          2, _omitEnumNames ? '' : 'MONITORING_AGENT_STATE_OFFLINE');
  static const MonitoringAgentState MONITORING_AGENT_STATE_NEVER_SEEN =
      MonitoringAgentState._(
          3, _omitEnumNames ? '' : 'MONITORING_AGENT_STATE_NEVER_SEEN');

  static const $core.List<MonitoringAgentState> values = <MonitoringAgentState>[
    MONITORING_AGENT_STATE_UNSPECIFIED,
    MONITORING_AGENT_STATE_ONLINE,
    MONITORING_AGENT_STATE_OFFLINE,
    MONITORING_AGENT_STATE_NEVER_SEEN,
  ];

  static final $core.List<MonitoringAgentState?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static MonitoringAgentState? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const MonitoringAgentState._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

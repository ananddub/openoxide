// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/alert_rule.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Watched resource kind (Rust TargetKind).
class AlertTargetKind extends $pb.ProtobufEnum {
  static const AlertTargetKind ALERT_TARGET_KIND_UNSPECIFIED =
      AlertTargetKind._(
          0, _omitEnumNames ? '' : 'ALERT_TARGET_KIND_UNSPECIFIED');
  static const AlertTargetKind ALERT_TARGET_KIND_SERVER =
      AlertTargetKind._(1, _omitEnumNames ? '' : 'ALERT_TARGET_KIND_SERVER');
  static const AlertTargetKind ALERT_TARGET_KIND_APPLICATION =
      AlertTargetKind._(
          2, _omitEnumNames ? '' : 'ALERT_TARGET_KIND_APPLICATION');
  static const AlertTargetKind ALERT_TARGET_KIND_DATABASE =
      AlertTargetKind._(3, _omitEnumNames ? '' : 'ALERT_TARGET_KIND_DATABASE');
  static const AlertTargetKind ALERT_TARGET_KIND_COMPOSE =
      AlertTargetKind._(4, _omitEnumNames ? '' : 'ALERT_TARGET_KIND_COMPOSE');

  static const $core.List<AlertTargetKind> values = <AlertTargetKind>[
    ALERT_TARGET_KIND_UNSPECIFIED,
    ALERT_TARGET_KIND_SERVER,
    ALERT_TARGET_KIND_APPLICATION,
    ALERT_TARGET_KIND_DATABASE,
    ALERT_TARGET_KIND_COMPOSE,
  ];

  static final $core.List<AlertTargetKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static AlertTargetKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AlertTargetKind._(super.value, super.name);
}

/// Watched metric (Rust MetricKind).
class AlertMetricKind extends $pb.ProtobufEnum {
  static const AlertMetricKind ALERT_METRIC_KIND_UNSPECIFIED =
      AlertMetricKind._(
          0, _omitEnumNames ? '' : 'ALERT_METRIC_KIND_UNSPECIFIED');
  static const AlertMetricKind ALERT_METRIC_KIND_CPU =
      AlertMetricKind._(1, _omitEnumNames ? '' : 'ALERT_METRIC_KIND_CPU');
  static const AlertMetricKind ALERT_METRIC_KIND_MEMORY =
      AlertMetricKind._(2, _omitEnumNames ? '' : 'ALERT_METRIC_KIND_MEMORY');
  static const AlertMetricKind ALERT_METRIC_KIND_DISK =
      AlertMetricKind._(3, _omitEnumNames ? '' : 'ALERT_METRIC_KIND_DISK');

  static const $core.List<AlertMetricKind> values = <AlertMetricKind>[
    ALERT_METRIC_KIND_UNSPECIFIED,
    ALERT_METRIC_KIND_CPU,
    ALERT_METRIC_KIND_MEMORY,
    ALERT_METRIC_KIND_DISK,
  ];

  static final $core.List<AlertMetricKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static AlertMetricKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AlertMetricKind._(super.value, super.name);
}

/// Comparison operator (Rust Operator).
class AlertOperator extends $pb.ProtobufEnum {
  static const AlertOperator ALERT_OPERATOR_UNSPECIFIED =
      AlertOperator._(0, _omitEnumNames ? '' : 'ALERT_OPERATOR_UNSPECIFIED');
  static const AlertOperator ALERT_OPERATOR_GREATER_THAN =
      AlertOperator._(1, _omitEnumNames ? '' : 'ALERT_OPERATOR_GREATER_THAN');
  static const AlertOperator ALERT_OPERATOR_GREATER_OR_EQUAL = AlertOperator._(
      2, _omitEnumNames ? '' : 'ALERT_OPERATOR_GREATER_OR_EQUAL');
  static const AlertOperator ALERT_OPERATOR_LESS_THAN =
      AlertOperator._(3, _omitEnumNames ? '' : 'ALERT_OPERATOR_LESS_THAN');
  static const AlertOperator ALERT_OPERATOR_LESS_OR_EQUAL =
      AlertOperator._(4, _omitEnumNames ? '' : 'ALERT_OPERATOR_LESS_OR_EQUAL');
  static const AlertOperator ALERT_OPERATOR_EQUAL =
      AlertOperator._(5, _omitEnumNames ? '' : 'ALERT_OPERATOR_EQUAL');

  static const $core.List<AlertOperator> values = <AlertOperator>[
    ALERT_OPERATOR_UNSPECIFIED,
    ALERT_OPERATOR_GREATER_THAN,
    ALERT_OPERATOR_GREATER_OR_EQUAL,
    ALERT_OPERATOR_LESS_THAN,
    ALERT_OPERATOR_LESS_OR_EQUAL,
    ALERT_OPERATOR_EQUAL,
  ];

  static final $core.List<AlertOperator?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static AlertOperator? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AlertOperator._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

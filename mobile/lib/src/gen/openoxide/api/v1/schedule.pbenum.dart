// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/schedule.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ScheduleShellType extends $pb.ProtobufEnum {
  static const ScheduleShellType SCHEDULE_SHELL_TYPE_UNSPECIFIED =
      ScheduleShellType._(
          0, _omitEnumNames ? '' : 'SCHEDULE_SHELL_TYPE_UNSPECIFIED');
  static const ScheduleShellType SCHEDULE_SHELL_TYPE_BASH =
      ScheduleShellType._(1, _omitEnumNames ? '' : 'SCHEDULE_SHELL_TYPE_BASH');
  static const ScheduleShellType SCHEDULE_SHELL_TYPE_SH =
      ScheduleShellType._(2, _omitEnumNames ? '' : 'SCHEDULE_SHELL_TYPE_SH');

  static const $core.List<ScheduleShellType> values = <ScheduleShellType>[
    SCHEDULE_SHELL_TYPE_UNSPECIFIED,
    SCHEDULE_SHELL_TYPE_BASH,
    SCHEDULE_SHELL_TYPE_SH,
  ];

  static final $core.List<ScheduleShellType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ScheduleShellType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleShellType._(super.value, super.name);
}

/// `DATABASE` is a legacy stored value kept for rows predating the APPLICATION/COMPOSE/SERVER split.
class ScheduleType extends $pb.ProtobufEnum {
  static const ScheduleType SCHEDULE_TYPE_UNSPECIFIED =
      ScheduleType._(0, _omitEnumNames ? '' : 'SCHEDULE_TYPE_UNSPECIFIED');
  static const ScheduleType SCHEDULE_TYPE_APPLICATION =
      ScheduleType._(1, _omitEnumNames ? '' : 'SCHEDULE_TYPE_APPLICATION');
  static const ScheduleType SCHEDULE_TYPE_COMPOSE =
      ScheduleType._(2, _omitEnumNames ? '' : 'SCHEDULE_TYPE_COMPOSE');
  static const ScheduleType SCHEDULE_TYPE_DATABASE =
      ScheduleType._(3, _omitEnumNames ? '' : 'SCHEDULE_TYPE_DATABASE');
  static const ScheduleType SCHEDULE_TYPE_SERVER =
      ScheduleType._(4, _omitEnumNames ? '' : 'SCHEDULE_TYPE_SERVER');
  static const ScheduleType SCHEDULE_TYPE_DOKPANEL_SERVER =
      ScheduleType._(5, _omitEnumNames ? '' : 'SCHEDULE_TYPE_DOKPANEL_SERVER');

  static const $core.List<ScheduleType> values = <ScheduleType>[
    SCHEDULE_TYPE_UNSPECIFIED,
    SCHEDULE_TYPE_APPLICATION,
    SCHEDULE_TYPE_COMPOSE,
    SCHEDULE_TYPE_DATABASE,
    SCHEDULE_TYPE_SERVER,
    SCHEDULE_TYPE_DOKPANEL_SERVER,
  ];

  static final $core.List<ScheduleType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static ScheduleType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleType._(super.value, super.name);
}

class ScheduleAction extends $pb.ProtobufEnum {
  static const ScheduleAction SCHEDULE_ACTION_UNSPECIFIED =
      ScheduleAction._(0, _omitEnumNames ? '' : 'SCHEDULE_ACTION_UNSPECIFIED');
  static const ScheduleAction SCHEDULE_ACTION_EXEC =
      ScheduleAction._(1, _omitEnumNames ? '' : 'SCHEDULE_ACTION_EXEC');
  static const ScheduleAction SCHEDULE_ACTION_DEPLOY =
      ScheduleAction._(2, _omitEnumNames ? '' : 'SCHEDULE_ACTION_DEPLOY');
  static const ScheduleAction SCHEDULE_ACTION_REDEPLOY =
      ScheduleAction._(3, _omitEnumNames ? '' : 'SCHEDULE_ACTION_REDEPLOY');
  static const ScheduleAction SCHEDULE_ACTION_REBUILD =
      ScheduleAction._(4, _omitEnumNames ? '' : 'SCHEDULE_ACTION_REBUILD');
  static const ScheduleAction SCHEDULE_ACTION_RELOAD =
      ScheduleAction._(5, _omitEnumNames ? '' : 'SCHEDULE_ACTION_RELOAD');
  static const ScheduleAction SCHEDULE_ACTION_START =
      ScheduleAction._(6, _omitEnumNames ? '' : 'SCHEDULE_ACTION_START');
  static const ScheduleAction SCHEDULE_ACTION_STOP =
      ScheduleAction._(7, _omitEnumNames ? '' : 'SCHEDULE_ACTION_STOP');

  static const $core.List<ScheduleAction> values = <ScheduleAction>[
    SCHEDULE_ACTION_UNSPECIFIED,
    SCHEDULE_ACTION_EXEC,
    SCHEDULE_ACTION_DEPLOY,
    SCHEDULE_ACTION_REDEPLOY,
    SCHEDULE_ACTION_REBUILD,
    SCHEDULE_ACTION_RELOAD,
    SCHEDULE_ACTION_START,
    SCHEDULE_ACTION_STOP,
  ];

  static final $core.List<ScheduleAction?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static ScheduleAction? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleAction._(super.value, super.name);
}

class ScheduleTriggerKind extends $pb.ProtobufEnum {
  static const ScheduleTriggerKind SCHEDULE_TRIGGER_KIND_UNSPECIFIED =
      ScheduleTriggerKind._(
          0, _omitEnumNames ? '' : 'SCHEDULE_TRIGGER_KIND_UNSPECIFIED');
  static const ScheduleTriggerKind SCHEDULE_TRIGGER_KIND_CRON =
      ScheduleTriggerKind._(
          1, _omitEnumNames ? '' : 'SCHEDULE_TRIGGER_KIND_CRON');
  static const ScheduleTriggerKind SCHEDULE_TRIGGER_KIND_MANUAL =
      ScheduleTriggerKind._(
          2, _omitEnumNames ? '' : 'SCHEDULE_TRIGGER_KIND_MANUAL');
  static const ScheduleTriggerKind SCHEDULE_TRIGGER_KIND_MISSED =
      ScheduleTriggerKind._(
          3, _omitEnumNames ? '' : 'SCHEDULE_TRIGGER_KIND_MISSED');
  static const ScheduleTriggerKind SCHEDULE_TRIGGER_KIND_RETRY =
      ScheduleTriggerKind._(
          4, _omitEnumNames ? '' : 'SCHEDULE_TRIGGER_KIND_RETRY');

  static const $core.List<ScheduleTriggerKind> values = <ScheduleTriggerKind>[
    SCHEDULE_TRIGGER_KIND_UNSPECIFIED,
    SCHEDULE_TRIGGER_KIND_CRON,
    SCHEDULE_TRIGGER_KIND_MANUAL,
    SCHEDULE_TRIGGER_KIND_MISSED,
    SCHEDULE_TRIGGER_KIND_RETRY,
  ];

  static final $core.List<ScheduleTriggerKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ScheduleTriggerKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleTriggerKind._(super.value, super.name);
}

class ScheduleExecutionStatus extends $pb.ProtobufEnum {
  static const ScheduleExecutionStatus SCHEDULE_EXECUTION_STATUS_UNSPECIFIED =
      ScheduleExecutionStatus._(
          0, _omitEnumNames ? '' : 'SCHEDULE_EXECUTION_STATUS_UNSPECIFIED');
  static const ScheduleExecutionStatus SCHEDULE_EXECUTION_STATUS_RUNNING =
      ScheduleExecutionStatus._(
          1, _omitEnumNames ? '' : 'SCHEDULE_EXECUTION_STATUS_RUNNING');
  static const ScheduleExecutionStatus SCHEDULE_EXECUTION_STATUS_SUCCEEDED =
      ScheduleExecutionStatus._(
          2, _omitEnumNames ? '' : 'SCHEDULE_EXECUTION_STATUS_SUCCEEDED');
  static const ScheduleExecutionStatus SCHEDULE_EXECUTION_STATUS_FAILED =
      ScheduleExecutionStatus._(
          3, _omitEnumNames ? '' : 'SCHEDULE_EXECUTION_STATUS_FAILED');
  static const ScheduleExecutionStatus SCHEDULE_EXECUTION_STATUS_SKIPPED =
      ScheduleExecutionStatus._(
          4, _omitEnumNames ? '' : 'SCHEDULE_EXECUTION_STATUS_SKIPPED');

  static const $core.List<ScheduleExecutionStatus> values =
      <ScheduleExecutionStatus>[
    SCHEDULE_EXECUTION_STATUS_UNSPECIFIED,
    SCHEDULE_EXECUTION_STATUS_RUNNING,
    SCHEDULE_EXECUTION_STATUS_SUCCEEDED,
    SCHEDULE_EXECUTION_STATUS_FAILED,
    SCHEDULE_EXECUTION_STATUS_SKIPPED,
  ];

  static final $core.List<ScheduleExecutionStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ScheduleExecutionStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleExecutionStatus._(super.value, super.name);
}

class ScheduleMissedRunPolicy extends $pb.ProtobufEnum {
  static const ScheduleMissedRunPolicy SCHEDULE_MISSED_RUN_POLICY_UNSPECIFIED =
      ScheduleMissedRunPolicy._(
          0, _omitEnumNames ? '' : 'SCHEDULE_MISSED_RUN_POLICY_UNSPECIFIED');
  static const ScheduleMissedRunPolicy SCHEDULE_MISSED_RUN_POLICY_SKIP =
      ScheduleMissedRunPolicy._(
          1, _omitEnumNames ? '' : 'SCHEDULE_MISSED_RUN_POLICY_SKIP');
  static const ScheduleMissedRunPolicy SCHEDULE_MISSED_RUN_POLICY_RUN_ONCE =
      ScheduleMissedRunPolicy._(
          2, _omitEnumNames ? '' : 'SCHEDULE_MISSED_RUN_POLICY_RUN_ONCE');

  static const $core.List<ScheduleMissedRunPolicy> values =
      <ScheduleMissedRunPolicy>[
    SCHEDULE_MISSED_RUN_POLICY_UNSPECIFIED,
    SCHEDULE_MISSED_RUN_POLICY_SKIP,
    SCHEDULE_MISSED_RUN_POLICY_RUN_ONCE,
  ];

  static final $core.List<ScheduleMissedRunPolicy?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ScheduleMissedRunPolicy? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleMissedRunPolicy._(super.value, super.name);
}

class ScheduleConcurrencyPolicy extends $pb.ProtobufEnum {
  static const ScheduleConcurrencyPolicy
      SCHEDULE_CONCURRENCY_POLICY_UNSPECIFIED = ScheduleConcurrencyPolicy._(
          0, _omitEnumNames ? '' : 'SCHEDULE_CONCURRENCY_POLICY_UNSPECIFIED');
  static const ScheduleConcurrencyPolicy SCHEDULE_CONCURRENCY_POLICY_SKIP =
      ScheduleConcurrencyPolicy._(
          1, _omitEnumNames ? '' : 'SCHEDULE_CONCURRENCY_POLICY_SKIP');
  static const ScheduleConcurrencyPolicy SCHEDULE_CONCURRENCY_POLICY_QUEUE =
      ScheduleConcurrencyPolicy._(
          2, _omitEnumNames ? '' : 'SCHEDULE_CONCURRENCY_POLICY_QUEUE');
  static const ScheduleConcurrencyPolicy SCHEDULE_CONCURRENCY_POLICY_ALLOW =
      ScheduleConcurrencyPolicy._(
          3, _omitEnumNames ? '' : 'SCHEDULE_CONCURRENCY_POLICY_ALLOW');

  static const $core.List<ScheduleConcurrencyPolicy> values =
      <ScheduleConcurrencyPolicy>[
    SCHEDULE_CONCURRENCY_POLICY_UNSPECIFIED,
    SCHEDULE_CONCURRENCY_POLICY_SKIP,
    SCHEDULE_CONCURRENCY_POLICY_QUEUE,
    SCHEDULE_CONCURRENCY_POLICY_ALLOW,
  ];

  static final $core.List<ScheduleConcurrencyPolicy?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static ScheduleConcurrencyPolicy? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ScheduleConcurrencyPolicy._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

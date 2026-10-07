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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'schedule.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'schedule.pbenum.dart';

/// kind "schedule": a cron-driven automation row.
class Schedule extends $pb.GeneratedMessage {
  factory Schedule({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? cronExpression,
    $core.String? appName,
    $core.String? serviceName,
    ScheduleShellType? shellType,
    ScheduleType? scheduleType,
    ScheduleAction? scheduleAction,
    $core.String? command,
    $core.String? script,
    $core.String? timezone,
    $core.bool? enabled,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? databaseId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Schedule._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (shellType != null) result.shellType = shellType;
    if (scheduleType != null) result.scheduleType = scheduleType;
    if (scheduleAction != null) result.scheduleAction = scheduleAction;
    if (command != null) result.command = command;
    if (script != null) result.script = script;
    if (timezone != null) result.timezone = timezone;
    if (enabled != null) result.enabled = enabled;
    if (serviceId != null) result.serviceId = serviceId;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (databaseId != null) result.databaseId = databaseId;
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Schedule._();

  factory Schedule.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Schedule()..mergeFromBuffer(data, registry);
  factory Schedule.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Schedule()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Schedule',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Schedule.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'cronExpression')
    ..aOS(5, _omitFieldNames ? '' : 'appName')
    ..aOS(6, _omitFieldNames ? '' : 'serviceName')
    ..aE<ScheduleShellType>(7, _omitFieldNames ? '' : 'shellType',
        enumValues: ScheduleShellType.values)
    ..aE<ScheduleType>(8, _omitFieldNames ? '' : 'scheduleType',
        enumValues: ScheduleType.values)
    ..aE<ScheduleAction>(9, _omitFieldNames ? '' : 'scheduleAction',
        enumValues: ScheduleAction.values)
    ..aOS(10, _omitFieldNames ? '' : 'command')
    ..aOS(11, _omitFieldNames ? '' : 'script')
    ..aOS(12, _omitFieldNames ? '' : 'timezone')
    ..aOB(13, _omitFieldNames ? '' : 'enabled')
    ..aInt64(14, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(15, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(16, _omitFieldNames ? '' : 'composeId')
    ..aInt64(17, _omitFieldNames ? '' : 'databaseId')
    ..aInt64(18, _omitFieldNames ? '' : 'serverId')
    ..aInt64(19, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(20, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(21, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Schedule clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Schedule copyWith(void Function(Schedule) updates) =>
      super.copyWith((message) => updates(message as Schedule)) as Schedule;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Schedule() / Schedule.new instead')
  static Schedule create() => Schedule._();
  static $pb.GeneratedMessage $_createMessage() => Schedule._();
  @$core.override
  Schedule createEmptyInstance() => Schedule._();
  @$core.pragma('dart2js:noInline')
  static Schedule getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Schedule>(Schedule.$_createMessage);
  static Schedule? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get cronExpression => $_getSZ(3);
  @$pb.TagNumber(4)
  set cronExpression($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCronExpression() => $_has(3);
  @$pb.TagNumber(4)
  void clearCronExpression() => $_clearField(4);

  /// Generated slug identifying the schedule's service row.
  @$pb.TagNumber(5)
  $core.String get appName => $_getSZ(4);
  @$pb.TagNumber(5)
  set appName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAppName() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serviceName => $_getSZ(5);
  @$pb.TagNumber(6)
  set serviceName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServiceName() => $_has(5);
  @$pb.TagNumber(6)
  void clearServiceName() => $_clearField(6);

  @$pb.TagNumber(7)
  ScheduleShellType get shellType => $_getN(6);
  @$pb.TagNumber(7)
  set shellType(ScheduleShellType value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasShellType() => $_has(6);
  @$pb.TagNumber(7)
  void clearShellType() => $_clearField(7);

  @$pb.TagNumber(8)
  ScheduleType get scheduleType => $_getN(7);
  @$pb.TagNumber(8)
  set scheduleType(ScheduleType value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasScheduleType() => $_has(7);
  @$pb.TagNumber(8)
  void clearScheduleType() => $_clearField(8);

  @$pb.TagNumber(9)
  ScheduleAction get scheduleAction => $_getN(8);
  @$pb.TagNumber(9)
  set scheduleAction(ScheduleAction value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasScheduleAction() => $_has(8);
  @$pb.TagNumber(9)
  void clearScheduleAction() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get command => $_getSZ(9);
  @$pb.TagNumber(10)
  set command($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCommand() => $_has(9);
  @$pb.TagNumber(10)
  void clearCommand() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get script => $_getSZ(10);
  @$pb.TagNumber(11)
  set script($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasScript() => $_has(10);
  @$pb.TagNumber(11)
  void clearScript() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get timezone => $_getSZ(11);
  @$pb.TagNumber(12)
  set timezone($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasTimezone() => $_has(11);
  @$pb.TagNumber(12)
  void clearTimezone() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get enabled => $_getBF(12);
  @$pb.TagNumber(13)
  set enabled($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasEnabled() => $_has(12);
  @$pb.TagNumber(13)
  void clearEnabled() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get serviceId => $_getI64(13);
  @$pb.TagNumber(14)
  set serviceId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasServiceId() => $_has(13);
  @$pb.TagNumber(14)
  void clearServiceId() => $_clearField(14);

  /// Typed linkage mirrors derived from (schedule_type, service_id); exactly one is set.
  @$pb.TagNumber(15)
  $fixnum.Int64 get applicationId => $_getI64(14);
  @$pb.TagNumber(15)
  set applicationId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasApplicationId() => $_has(14);
  @$pb.TagNumber(15)
  void clearApplicationId() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get composeId => $_getI64(15);
  @$pb.TagNumber(16)
  set composeId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasComposeId() => $_has(15);
  @$pb.TagNumber(16)
  void clearComposeId() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get databaseId => $_getI64(16);
  @$pb.TagNumber(17)
  set databaseId($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasDatabaseId() => $_has(16);
  @$pb.TagNumber(17)
  void clearDatabaseId() => $_clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get serverId => $_getI64(17);
  @$pb.TagNumber(18)
  set serverId($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasServerId() => $_has(17);
  @$pb.TagNumber(18)
  void clearServerId() => $_clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get organizationId => $_getI64(18);
  @$pb.TagNumber(19)
  set organizationId($fixnum.Int64 value) => $_setInt64(18, value);
  @$pb.TagNumber(19)
  $core.bool hasOrganizationId() => $_has(18);
  @$pb.TagNumber(19)
  void clearOrganizationId() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get createdAt => $_getI64(19);
  @$pb.TagNumber(20)
  set createdAt($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasCreatedAt() => $_has(19);
  @$pb.TagNumber(20)
  void clearCreatedAt() => $_clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get updatedAt => $_getI64(20);
  @$pb.TagNumber(21)
  set updatedAt($fixnum.Int64 value) => $_setInt64(20, value);
  @$pb.TagNumber(21)
  $core.bool hasUpdatedAt() => $_has(20);
  @$pb.TagNumber(21)
  void clearUpdatedAt() => $_clearField(21);
}

/// Outcome of running a schedule right now.
class ScheduleRun extends $pb.GeneratedMessage {
  factory ScheduleRun({
    Schedule? schedule,
    $core.String? action,
    $fixnum.Int64? deploymentId,
    $core.String? message,
    $core.String? stdout,
    $core.String? stderr,
  }) {
    final result = ScheduleRun._();
    if (schedule != null) result.schedule = schedule;
    if (action != null) result.action = action;
    if (deploymentId != null) result.deploymentId = deploymentId;
    if (message != null) result.message = message;
    if (stdout != null) result.stdout = stdout;
    if (stderr != null) result.stderr = stderr;
    return result;
  }

  ScheduleRun._();

  factory ScheduleRun.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRun()..mergeFromBuffer(data, registry);
  factory ScheduleRun.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRun()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleRun',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleRun.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'action')
    ..aInt64(3, _omitFieldNames ? '' : 'deploymentId')
    ..aOS(4, _omitFieldNames ? '' : 'message')
    ..aOS(5, _omitFieldNames ? '' : 'stdout')
    ..aOS(6, _omitFieldNames ? '' : 'stderr')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRun clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRun copyWith(void Function(ScheduleRun) updates) =>
      super.copyWith((message) => updates(message as ScheduleRun))
          as ScheduleRun;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ScheduleRun() / ScheduleRun.new instead')
  static ScheduleRun create() => ScheduleRun._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleRun._();
  @$core.override
  ScheduleRun createEmptyInstance() => ScheduleRun._();
  @$core.pragma('dart2js:noInline')
  static ScheduleRun getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ScheduleRun>(
          ScheduleRun.$_createMessage);
  static ScheduleRun? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);

  /// Action actually performed (echo of schedule_action, free-form).
  @$pb.TagNumber(2)
  $core.String get action => $_getSZ(1);
  @$pb.TagNumber(2)
  set action($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get deploymentId => $_getI64(2);
  @$pb.TagNumber(3)
  set deploymentId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDeploymentId() => $_has(2);
  @$pb.TagNumber(3)
  void clearDeploymentId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get message => $_getSZ(3);
  @$pb.TagNumber(4)
  set message($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMessage() => $_has(3);
  @$pb.TagNumber(4)
  void clearMessage() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get stdout => $_getSZ(4);
  @$pb.TagNumber(5)
  set stdout($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStdout() => $_has(4);
  @$pb.TagNumber(5)
  void clearStdout() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get stderr => $_getSZ(5);
  @$pb.TagNumber(6)
  set stderr($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasStderr() => $_has(5);
  @$pb.TagNumber(6)
  void clearStderr() => $_clearField(6);
}

/// kind "schedule_execution": one recorded run of a schedule.
class ScheduleExecution extends $pb.GeneratedMessage {
  factory ScheduleExecution({
    $fixnum.Int64? id,
    $fixnum.Int64? scheduleId,
    $fixnum.Int64? organizationId,
    ScheduleTriggerKind? triggerKind,
    ScheduleExecutionStatus? status,
    $fixnum.Int64? attempt,
    $fixnum.Int64? scheduledAt,
    $fixnum.Int64? startedAt,
    $fixnum.Int64? finishedAt,
    $core.String? message,
    $core.String? stdout,
    $core.String? stderr,
  }) {
    final result = ScheduleExecution._();
    if (id != null) result.id = id;
    if (scheduleId != null) result.scheduleId = scheduleId;
    if (organizationId != null) result.organizationId = organizationId;
    if (triggerKind != null) result.triggerKind = triggerKind;
    if (status != null) result.status = status;
    if (attempt != null) result.attempt = attempt;
    if (scheduledAt != null) result.scheduledAt = scheduledAt;
    if (startedAt != null) result.startedAt = startedAt;
    if (finishedAt != null) result.finishedAt = finishedAt;
    if (message != null) result.message = message;
    if (stdout != null) result.stdout = stdout;
    if (stderr != null) result.stderr = stderr;
    return result;
  }

  ScheduleExecution._();

  factory ScheduleExecution.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleExecution()..mergeFromBuffer(data, registry);
  factory ScheduleExecution.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleExecution()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleExecution',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleExecution.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'scheduleId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aE<ScheduleTriggerKind>(4, _omitFieldNames ? '' : 'triggerKind',
        enumValues: ScheduleTriggerKind.values)
    ..aE<ScheduleExecutionStatus>(5, _omitFieldNames ? '' : 'status',
        enumValues: ScheduleExecutionStatus.values)
    ..aInt64(6, _omitFieldNames ? '' : 'attempt')
    ..aInt64(7, _omitFieldNames ? '' : 'scheduledAt')
    ..aInt64(8, _omitFieldNames ? '' : 'startedAt')
    ..aInt64(9, _omitFieldNames ? '' : 'finishedAt')
    ..aOS(10, _omitFieldNames ? '' : 'message')
    ..aOS(11, _omitFieldNames ? '' : 'stdout')
    ..aOS(12, _omitFieldNames ? '' : 'stderr')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleExecution clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleExecution copyWith(void Function(ScheduleExecution) updates) =>
      super.copyWith((message) => updates(message as ScheduleExecution))
          as ScheduleExecution;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ScheduleExecution() / ScheduleExecution.new instead')
  static ScheduleExecution create() => ScheduleExecution._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleExecution._();
  @$core.override
  ScheduleExecution createEmptyInstance() => ScheduleExecution._();
  @$core.pragma('dart2js:noInline')
  static ScheduleExecution getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ScheduleExecution>(
          ScheduleExecution.$_createMessage);
  static ScheduleExecution? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get scheduleId => $_getI64(1);
  @$pb.TagNumber(2)
  set scheduleId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasScheduleId() => $_has(1);
  @$pb.TagNumber(2)
  void clearScheduleId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  @$pb.TagNumber(4)
  ScheduleTriggerKind get triggerKind => $_getN(3);
  @$pb.TagNumber(4)
  set triggerKind(ScheduleTriggerKind value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasTriggerKind() => $_has(3);
  @$pb.TagNumber(4)
  void clearTriggerKind() => $_clearField(4);

  @$pb.TagNumber(5)
  ScheduleExecutionStatus get status => $_getN(4);
  @$pb.TagNumber(5)
  set status(ScheduleExecutionStatus value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(5)
  void clearStatus() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get attempt => $_getI64(5);
  @$pb.TagNumber(6)
  set attempt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAttempt() => $_has(5);
  @$pb.TagNumber(6)
  void clearAttempt() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get scheduledAt => $_getI64(6);
  @$pb.TagNumber(7)
  set scheduledAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasScheduledAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearScheduledAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get startedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set startedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasStartedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearStartedAt() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get finishedAt => $_getI64(8);
  @$pb.TagNumber(9)
  set finishedAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasFinishedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearFinishedAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get message => $_getSZ(9);
  @$pb.TagNumber(10)
  set message($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasMessage() => $_has(9);
  @$pb.TagNumber(10)
  void clearMessage() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get stdout => $_getSZ(10);
  @$pb.TagNumber(11)
  set stdout($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasStdout() => $_has(10);
  @$pb.TagNumber(11)
  void clearStdout() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get stderr => $_getSZ(11);
  @$pb.TagNumber(12)
  set stderr($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasStderr() => $_has(11);
  @$pb.TagNumber(12)
  void clearStderr() => $_clearField(12);
}

/// kind "schedule_runtime_policy": per-schedule runner tuning.
class ScheduleRuntimePolicy extends $pb.GeneratedMessage {
  factory ScheduleRuntimePolicy({
    $fixnum.Int64? scheduleId,
    $fixnum.Int64? retryCount,
    $fixnum.Int64? retryDelaySeconds,
    ScheduleMissedRunPolicy? missedRunPolicy,
    ScheduleConcurrencyPolicy? concurrencyPolicy,
    $fixnum.Int64? leaseSeconds,
    $core.bool? notifyOnSuccess,
    $core.bool? notifyOnFailure,
    $fixnum.Int64? lastScheduledAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ScheduleRuntimePolicy._();
    if (scheduleId != null) result.scheduleId = scheduleId;
    if (retryCount != null) result.retryCount = retryCount;
    if (retryDelaySeconds != null) result.retryDelaySeconds = retryDelaySeconds;
    if (missedRunPolicy != null) result.missedRunPolicy = missedRunPolicy;
    if (concurrencyPolicy != null) result.concurrencyPolicy = concurrencyPolicy;
    if (leaseSeconds != null) result.leaseSeconds = leaseSeconds;
    if (notifyOnSuccess != null) result.notifyOnSuccess = notifyOnSuccess;
    if (notifyOnFailure != null) result.notifyOnFailure = notifyOnFailure;
    if (lastScheduledAt != null) result.lastScheduledAt = lastScheduledAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ScheduleRuntimePolicy._();

  factory ScheduleRuntimePolicy.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRuntimePolicy()..mergeFromBuffer(data, registry);
  factory ScheduleRuntimePolicy.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRuntimePolicy()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleRuntimePolicy',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleRuntimePolicy.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'scheduleId')
    ..aInt64(2, _omitFieldNames ? '' : 'retryCount')
    ..aInt64(3, _omitFieldNames ? '' : 'retryDelaySeconds')
    ..aE<ScheduleMissedRunPolicy>(4, _omitFieldNames ? '' : 'missedRunPolicy',
        enumValues: ScheduleMissedRunPolicy.values)
    ..aE<ScheduleConcurrencyPolicy>(
        5, _omitFieldNames ? '' : 'concurrencyPolicy',
        enumValues: ScheduleConcurrencyPolicy.values)
    ..aInt64(6, _omitFieldNames ? '' : 'leaseSeconds')
    ..aOB(7, _omitFieldNames ? '' : 'notifyOnSuccess')
    ..aOB(8, _omitFieldNames ? '' : 'notifyOnFailure')
    ..aInt64(9, _omitFieldNames ? '' : 'lastScheduledAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRuntimePolicy clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRuntimePolicy copyWith(
          void Function(ScheduleRuntimePolicy) updates) =>
      super.copyWith((message) => updates(message as ScheduleRuntimePolicy))
          as ScheduleRuntimePolicy;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleRuntimePolicy() / ScheduleRuntimePolicy.new instead')
  static ScheduleRuntimePolicy create() => ScheduleRuntimePolicy._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleRuntimePolicy._();
  @$core.override
  ScheduleRuntimePolicy createEmptyInstance() => ScheduleRuntimePolicy._();
  @$core.pragma('dart2js:noInline')
  static ScheduleRuntimePolicy getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleRuntimePolicy>(
          ScheduleRuntimePolicy.$_createMessage);
  static ScheduleRuntimePolicy? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get scheduleId => $_getI64(0);
  @$pb.TagNumber(1)
  set scheduleId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasScheduleId() => $_has(0);
  @$pb.TagNumber(1)
  void clearScheduleId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get retryCount => $_getI64(1);
  @$pb.TagNumber(2)
  set retryCount($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRetryCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearRetryCount() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get retryDelaySeconds => $_getI64(2);
  @$pb.TagNumber(3)
  set retryDelaySeconds($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRetryDelaySeconds() => $_has(2);
  @$pb.TagNumber(3)
  void clearRetryDelaySeconds() => $_clearField(3);

  @$pb.TagNumber(4)
  ScheduleMissedRunPolicy get missedRunPolicy => $_getN(3);
  @$pb.TagNumber(4)
  set missedRunPolicy(ScheduleMissedRunPolicy value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMissedRunPolicy() => $_has(3);
  @$pb.TagNumber(4)
  void clearMissedRunPolicy() => $_clearField(4);

  @$pb.TagNumber(5)
  ScheduleConcurrencyPolicy get concurrencyPolicy => $_getN(4);
  @$pb.TagNumber(5)
  set concurrencyPolicy(ScheduleConcurrencyPolicy value) =>
      $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasConcurrencyPolicy() => $_has(4);
  @$pb.TagNumber(5)
  void clearConcurrencyPolicy() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get leaseSeconds => $_getI64(5);
  @$pb.TagNumber(6)
  set leaseSeconds($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLeaseSeconds() => $_has(5);
  @$pb.TagNumber(6)
  void clearLeaseSeconds() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get notifyOnSuccess => $_getBF(6);
  @$pb.TagNumber(7)
  set notifyOnSuccess($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasNotifyOnSuccess() => $_has(6);
  @$pb.TagNumber(7)
  void clearNotifyOnSuccess() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get notifyOnFailure => $_getBF(7);
  @$pb.TagNumber(8)
  set notifyOnFailure($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasNotifyOnFailure() => $_has(7);
  @$pb.TagNumber(8)
  void clearNotifyOnFailure() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get lastScheduledAt => $_getI64(8);
  @$pb.TagNumber(9)
  set lastScheduledAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasLastScheduledAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearLastScheduledAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get updatedAt => $_getI64(9);
  @$pb.TagNumber(10)
  set updatedAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUpdatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearUpdatedAt() => $_clearField(10);
}

/// kind "schedule_log" / "schedule_execution_log": raw file-log contents.
class ScheduleLog extends $pb.GeneratedMessage {
  factory ScheduleLog({
    $core.String? content,
  }) {
    final result = ScheduleLog._();
    if (content != null) result.content = content;
    return result;
  }

  ScheduleLog._();

  factory ScheduleLog.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleLog()..mergeFromBuffer(data, registry);
  factory ScheduleLog.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleLog()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleLog',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleLog.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleLog clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleLog copyWith(void Function(ScheduleLog) updates) =>
      super.copyWith((message) => updates(message as ScheduleLog))
          as ScheduleLog;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ScheduleLog() / ScheduleLog.new instead')
  static ScheduleLog create() => ScheduleLog._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleLog._();
  @$core.override
  ScheduleLog createEmptyInstance() => ScheduleLog._();
  @$core.pragma('dart2js:noInline')
  static ScheduleLog getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ScheduleLog>(
          ScheduleLog.$_createMessage);
  static ScheduleLog? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

class ScheduleListSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListSchedulesRequest() => ScheduleListSchedulesRequest._();

  ScheduleListSchedulesRequest._();

  factory ScheduleListSchedulesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListSchedulesRequest()..mergeFromBuffer(data, registry);
  factory ScheduleListSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListSchedulesRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListSchedulesRequest copyWith(
          void Function(ScheduleListSchedulesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleListSchedulesRequest))
          as ScheduleListSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListSchedulesRequest() / ScheduleListSchedulesRequest.new instead')
  static ScheduleListSchedulesRequest create() =>
      ScheduleListSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListSchedulesRequest._();
  @$core.override
  ScheduleListSchedulesRequest createEmptyInstance() =>
      ScheduleListSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListSchedulesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleListSchedulesRequest>(
          ScheduleListSchedulesRequest.$_createMessage);
  static ScheduleListSchedulesRequest? _defaultInstance;
}

class ScheduleListSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListSchedulesResponse._();

  factory ScheduleListSchedulesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListSchedulesResponse()..mergeFromBuffer(data, registry);
  factory ScheduleListSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListSchedulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListSchedulesResponse copyWith(
          void Function(ScheduleListSchedulesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleListSchedulesResponse))
          as ScheduleListSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListSchedulesResponse() / ScheduleListSchedulesResponse.new instead')
  static ScheduleListSchedulesResponse create() =>
      ScheduleListSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListSchedulesResponse._();
  @$core.override
  ScheduleListSchedulesResponse createEmptyInstance() =>
      ScheduleListSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListSchedulesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleListSchedulesResponse>(
          ScheduleListSchedulesResponse.$_createMessage);
  static ScheduleListSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleGetScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleGetScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleGetScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleGetScheduleRequest._();

  factory ScheduleGetScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleGetScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetScheduleRequest copyWith(
          void Function(ScheduleGetScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetScheduleRequest))
          as ScheduleGetScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetScheduleRequest() / ScheduleGetScheduleRequest.new instead')
  static ScheduleGetScheduleRequest create() => ScheduleGetScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetScheduleRequest._();
  @$core.override
  ScheduleGetScheduleRequest createEmptyInstance() =>
      ScheduleGetScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetScheduleRequest>(
          ScheduleGetScheduleRequest.$_createMessage);
  static ScheduleGetScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleGetScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleGetScheduleResponse({
    Schedule? schedule,
  }) {
    final result = ScheduleGetScheduleResponse._();
    if (schedule != null) result.schedule = schedule;
    return result;
  }

  ScheduleGetScheduleResponse._();

  factory ScheduleGetScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleGetScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetScheduleResponse.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetScheduleResponse copyWith(
          void Function(ScheduleGetScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetScheduleResponse))
          as ScheduleGetScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetScheduleResponse() / ScheduleGetScheduleResponse.new instead')
  static ScheduleGetScheduleResponse create() =>
      ScheduleGetScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetScheduleResponse._();
  @$core.override
  ScheduleGetScheduleResponse createEmptyInstance() =>
      ScheduleGetScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetScheduleResponse>(
          ScheduleGetScheduleResponse.$_createMessage);
  static ScheduleGetScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);
}

/// Absent fields fall back to server defaults: shell_type BASH, schedule_type APPLICATION, schedule_action EXEC, enabled true.
class ScheduleCreateScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleCreateScheduleRequest({
    $core.String? name,
    $core.String? description,
    $core.String? cronExpression,
    $core.String? appName,
    $core.String? serviceName,
    ScheduleShellType? shellType,
    ScheduleType? scheduleType,
    ScheduleAction? scheduleAction,
    $core.String? command,
    $core.String? script,
    $core.String? timezone,
    $core.bool? enabled,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
  }) {
    final result = ScheduleCreateScheduleRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (shellType != null) result.shellType = shellType;
    if (scheduleType != null) result.scheduleType = scheduleType;
    if (scheduleAction != null) result.scheduleAction = scheduleAction;
    if (command != null) result.command = command;
    if (script != null) result.script = script;
    if (timezone != null) result.timezone = timezone;
    if (enabled != null) result.enabled = enabled;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ScheduleCreateScheduleRequest._();

  factory ScheduleCreateScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleCreateScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleCreateScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleCreateScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleCreateScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleCreateScheduleRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aOS(3, _omitFieldNames ? '' : 'cronExpression')
    ..aOS(4, _omitFieldNames ? '' : 'appName')
    ..aOS(5, _omitFieldNames ? '' : 'serviceName')
    ..aE<ScheduleShellType>(6, _omitFieldNames ? '' : 'shellType',
        enumValues: ScheduleShellType.values)
    ..aE<ScheduleType>(7, _omitFieldNames ? '' : 'scheduleType',
        enumValues: ScheduleType.values)
    ..aE<ScheduleAction>(8, _omitFieldNames ? '' : 'scheduleAction',
        enumValues: ScheduleAction.values)
    ..aOS(9, _omitFieldNames ? '' : 'command')
    ..aOS(10, _omitFieldNames ? '' : 'script')
    ..aOS(11, _omitFieldNames ? '' : 'timezone')
    ..aOB(12, _omitFieldNames ? '' : 'enabled')
    ..aInt64(13, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(14, _omitFieldNames ? '' : 'composeId')
    ..aInt64(15, _omitFieldNames ? '' : 'serverId')
    ..aInt64(16, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleCreateScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleCreateScheduleRequest copyWith(
          void Function(ScheduleCreateScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleCreateScheduleRequest))
          as ScheduleCreateScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleCreateScheduleRequest() / ScheduleCreateScheduleRequest.new instead')
  static ScheduleCreateScheduleRequest create() =>
      ScheduleCreateScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleCreateScheduleRequest._();
  @$core.override
  ScheduleCreateScheduleRequest createEmptyInstance() =>
      ScheduleCreateScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleCreateScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleCreateScheduleRequest>(
          ScheduleCreateScheduleRequest.$_createMessage);
  static ScheduleCreateScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get cronExpression => $_getSZ(2);
  @$pb.TagNumber(3)
  set cronExpression($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCronExpression() => $_has(2);
  @$pb.TagNumber(3)
  void clearCronExpression() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get appName => $_getSZ(3);
  @$pb.TagNumber(4)
  set appName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAppName() => $_has(3);
  @$pb.TagNumber(4)
  void clearAppName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get serviceName => $_getSZ(4);
  @$pb.TagNumber(5)
  set serviceName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServiceName() => $_has(4);
  @$pb.TagNumber(5)
  void clearServiceName() => $_clearField(5);

  @$pb.TagNumber(6)
  ScheduleShellType get shellType => $_getN(5);
  @$pb.TagNumber(6)
  set shellType(ScheduleShellType value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasShellType() => $_has(5);
  @$pb.TagNumber(6)
  void clearShellType() => $_clearField(6);

  @$pb.TagNumber(7)
  ScheduleType get scheduleType => $_getN(6);
  @$pb.TagNumber(7)
  set scheduleType(ScheduleType value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasScheduleType() => $_has(6);
  @$pb.TagNumber(7)
  void clearScheduleType() => $_clearField(7);

  @$pb.TagNumber(8)
  ScheduleAction get scheduleAction => $_getN(7);
  @$pb.TagNumber(8)
  set scheduleAction(ScheduleAction value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasScheduleAction() => $_has(7);
  @$pb.TagNumber(8)
  void clearScheduleAction() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get command => $_getSZ(8);
  @$pb.TagNumber(9)
  set command($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCommand() => $_has(8);
  @$pb.TagNumber(9)
  void clearCommand() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get script => $_getSZ(9);
  @$pb.TagNumber(10)
  set script($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasScript() => $_has(9);
  @$pb.TagNumber(10)
  void clearScript() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get timezone => $_getSZ(10);
  @$pb.TagNumber(11)
  set timezone($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasTimezone() => $_has(10);
  @$pb.TagNumber(11)
  void clearTimezone() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get enabled => $_getBF(11);
  @$pb.TagNumber(12)
  set enabled($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnabled() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnabled() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get applicationId => $_getI64(12);
  @$pb.TagNumber(13)
  set applicationId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasApplicationId() => $_has(12);
  @$pb.TagNumber(13)
  void clearApplicationId() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get composeId => $_getI64(13);
  @$pb.TagNumber(14)
  set composeId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasComposeId() => $_has(13);
  @$pb.TagNumber(14)
  void clearComposeId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get serverId => $_getI64(14);
  @$pb.TagNumber(15)
  set serverId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasServerId() => $_has(14);
  @$pb.TagNumber(15)
  void clearServerId() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get organizationId => $_getI64(15);
  @$pb.TagNumber(16)
  set organizationId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasOrganizationId() => $_has(15);
  @$pb.TagNumber(16)
  void clearOrganizationId() => $_clearField(16);
}

class ScheduleCreateScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleCreateScheduleResponse({
    Schedule? schedule,
  }) {
    final result = ScheduleCreateScheduleResponse._();
    if (schedule != null) result.schedule = schedule;
    return result;
  }

  ScheduleCreateScheduleResponse._();

  factory ScheduleCreateScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleCreateScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleCreateScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleCreateScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleCreateScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleCreateScheduleResponse.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleCreateScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleCreateScheduleResponse copyWith(
          void Function(ScheduleCreateScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleCreateScheduleResponse))
          as ScheduleCreateScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleCreateScheduleResponse() / ScheduleCreateScheduleResponse.new instead')
  static ScheduleCreateScheduleResponse create() =>
      ScheduleCreateScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleCreateScheduleResponse._();
  @$core.override
  ScheduleCreateScheduleResponse createEmptyInstance() =>
      ScheduleCreateScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleCreateScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleCreateScheduleResponse>(
          ScheduleCreateScheduleResponse.$_createMessage);
  static ScheduleCreateScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);
}

/// An absent field keeps its stored value.
class ScheduleUpdateScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleUpdateScheduleRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? cronExpression,
    $core.String? appName,
    $core.String? serviceName,
    ScheduleShellType? shellType,
    ScheduleType? scheduleType,
    ScheduleAction? scheduleAction,
    $core.String? command,
    $core.String? script,
    $core.String? timezone,
    $core.bool? enabled,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
  }) {
    final result = ScheduleUpdateScheduleRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (shellType != null) result.shellType = shellType;
    if (scheduleType != null) result.scheduleType = scheduleType;
    if (scheduleAction != null) result.scheduleAction = scheduleAction;
    if (command != null) result.command = command;
    if (script != null) result.script = script;
    if (timezone != null) result.timezone = timezone;
    if (enabled != null) result.enabled = enabled;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ScheduleUpdateScheduleRequest._();

  factory ScheduleUpdateScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleUpdateScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleUpdateScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleUpdateScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'cronExpression')
    ..aOS(5, _omitFieldNames ? '' : 'appName')
    ..aOS(6, _omitFieldNames ? '' : 'serviceName')
    ..aE<ScheduleShellType>(7, _omitFieldNames ? '' : 'shellType',
        enumValues: ScheduleShellType.values)
    ..aE<ScheduleType>(8, _omitFieldNames ? '' : 'scheduleType',
        enumValues: ScheduleType.values)
    ..aE<ScheduleAction>(9, _omitFieldNames ? '' : 'scheduleAction',
        enumValues: ScheduleAction.values)
    ..aOS(10, _omitFieldNames ? '' : 'command')
    ..aOS(11, _omitFieldNames ? '' : 'script')
    ..aOS(12, _omitFieldNames ? '' : 'timezone')
    ..aOB(13, _omitFieldNames ? '' : 'enabled')
    ..aInt64(14, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(15, _omitFieldNames ? '' : 'composeId')
    ..aInt64(16, _omitFieldNames ? '' : 'serverId')
    ..aInt64(17, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateScheduleRequest copyWith(
          void Function(ScheduleUpdateScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleUpdateScheduleRequest))
          as ScheduleUpdateScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleUpdateScheduleRequest() / ScheduleUpdateScheduleRequest.new instead')
  static ScheduleUpdateScheduleRequest create() =>
      ScheduleUpdateScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleUpdateScheduleRequest._();
  @$core.override
  ScheduleUpdateScheduleRequest createEmptyInstance() =>
      ScheduleUpdateScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleUpdateScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleUpdateScheduleRequest>(
          ScheduleUpdateScheduleRequest.$_createMessage);
  static ScheduleUpdateScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get cronExpression => $_getSZ(3);
  @$pb.TagNumber(4)
  set cronExpression($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCronExpression() => $_has(3);
  @$pb.TagNumber(4)
  void clearCronExpression() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get appName => $_getSZ(4);
  @$pb.TagNumber(5)
  set appName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAppName() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serviceName => $_getSZ(5);
  @$pb.TagNumber(6)
  set serviceName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServiceName() => $_has(5);
  @$pb.TagNumber(6)
  void clearServiceName() => $_clearField(6);

  @$pb.TagNumber(7)
  ScheduleShellType get shellType => $_getN(6);
  @$pb.TagNumber(7)
  set shellType(ScheduleShellType value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasShellType() => $_has(6);
  @$pb.TagNumber(7)
  void clearShellType() => $_clearField(7);

  @$pb.TagNumber(8)
  ScheduleType get scheduleType => $_getN(7);
  @$pb.TagNumber(8)
  set scheduleType(ScheduleType value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasScheduleType() => $_has(7);
  @$pb.TagNumber(8)
  void clearScheduleType() => $_clearField(8);

  @$pb.TagNumber(9)
  ScheduleAction get scheduleAction => $_getN(8);
  @$pb.TagNumber(9)
  set scheduleAction(ScheduleAction value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasScheduleAction() => $_has(8);
  @$pb.TagNumber(9)
  void clearScheduleAction() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get command => $_getSZ(9);
  @$pb.TagNumber(10)
  set command($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCommand() => $_has(9);
  @$pb.TagNumber(10)
  void clearCommand() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get script => $_getSZ(10);
  @$pb.TagNumber(11)
  set script($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasScript() => $_has(10);
  @$pb.TagNumber(11)
  void clearScript() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get timezone => $_getSZ(11);
  @$pb.TagNumber(12)
  set timezone($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasTimezone() => $_has(11);
  @$pb.TagNumber(12)
  void clearTimezone() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get enabled => $_getBF(12);
  @$pb.TagNumber(13)
  set enabled($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasEnabled() => $_has(12);
  @$pb.TagNumber(13)
  void clearEnabled() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get applicationId => $_getI64(13);
  @$pb.TagNumber(14)
  set applicationId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasApplicationId() => $_has(13);
  @$pb.TagNumber(14)
  void clearApplicationId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get composeId => $_getI64(14);
  @$pb.TagNumber(15)
  set composeId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasComposeId() => $_has(14);
  @$pb.TagNumber(15)
  void clearComposeId() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get serverId => $_getI64(15);
  @$pb.TagNumber(16)
  set serverId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasServerId() => $_has(15);
  @$pb.TagNumber(16)
  void clearServerId() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get organizationId => $_getI64(16);
  @$pb.TagNumber(17)
  set organizationId($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasOrganizationId() => $_has(16);
  @$pb.TagNumber(17)
  void clearOrganizationId() => $_clearField(17);
}

class ScheduleUpdateScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleUpdateScheduleResponse({
    Schedule? schedule,
  }) {
    final result = ScheduleUpdateScheduleResponse._();
    if (schedule != null) result.schedule = schedule;
    return result;
  }

  ScheduleUpdateScheduleResponse._();

  factory ScheduleUpdateScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleUpdateScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleUpdateScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleUpdateScheduleResponse.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateScheduleResponse copyWith(
          void Function(ScheduleUpdateScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleUpdateScheduleResponse))
          as ScheduleUpdateScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleUpdateScheduleResponse() / ScheduleUpdateScheduleResponse.new instead')
  static ScheduleUpdateScheduleResponse create() =>
      ScheduleUpdateScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleUpdateScheduleResponse._();
  @$core.override
  ScheduleUpdateScheduleResponse createEmptyInstance() =>
      ScheduleUpdateScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleUpdateScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleUpdateScheduleResponse>(
          ScheduleUpdateScheduleResponse.$_createMessage);
  static ScheduleUpdateScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);
}

class ScheduleDeleteScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleDeleteScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleDeleteScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleDeleteScheduleRequest._();

  factory ScheduleDeleteScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleDeleteScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleDeleteScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleDeleteScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleDeleteScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleDeleteScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleDeleteScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleDeleteScheduleRequest copyWith(
          void Function(ScheduleDeleteScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleDeleteScheduleRequest))
          as ScheduleDeleteScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleDeleteScheduleRequest() / ScheduleDeleteScheduleRequest.new instead')
  static ScheduleDeleteScheduleRequest create() =>
      ScheduleDeleteScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleDeleteScheduleRequest._();
  @$core.override
  ScheduleDeleteScheduleRequest createEmptyInstance() =>
      ScheduleDeleteScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleDeleteScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleDeleteScheduleRequest>(
          ScheduleDeleteScheduleRequest.$_createMessage);
  static ScheduleDeleteScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleRunScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleRunScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleRunScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleRunScheduleRequest._();

  factory ScheduleRunScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRunScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleRunScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRunScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleRunScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleRunScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRunScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRunScheduleRequest copyWith(
          void Function(ScheduleRunScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleRunScheduleRequest))
          as ScheduleRunScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleRunScheduleRequest() / ScheduleRunScheduleRequest.new instead')
  static ScheduleRunScheduleRequest create() => ScheduleRunScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleRunScheduleRequest._();
  @$core.override
  ScheduleRunScheduleRequest createEmptyInstance() =>
      ScheduleRunScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleRunScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleRunScheduleRequest>(
          ScheduleRunScheduleRequest.$_createMessage);
  static ScheduleRunScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleRunScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleRunScheduleResponse({
    ScheduleRun? result,
  }) {
    final result$ = ScheduleRunScheduleResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ScheduleRunScheduleResponse._();

  factory ScheduleRunScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRunScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleRunScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleRunScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleRunScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleRunScheduleResponse.$_createMessage)
    ..aOM<ScheduleRun>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ScheduleRun.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRunScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleRunScheduleResponse copyWith(
          void Function(ScheduleRunScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleRunScheduleResponse))
          as ScheduleRunScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleRunScheduleResponse() / ScheduleRunScheduleResponse.new instead')
  static ScheduleRunScheduleResponse create() =>
      ScheduleRunScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleRunScheduleResponse._();
  @$core.override
  ScheduleRunScheduleResponse createEmptyInstance() =>
      ScheduleRunScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleRunScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleRunScheduleResponse>(
          ScheduleRunScheduleResponse.$_createMessage);
  static ScheduleRunScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleRun get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ScheduleRun value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleRun ensureResult() => $_ensure(0);
}

class SchedulePauseScheduleRequest extends $pb.GeneratedMessage {
  factory SchedulePauseScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = SchedulePauseScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  SchedulePauseScheduleRequest._();

  factory SchedulePauseScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchedulePauseScheduleRequest()..mergeFromBuffer(data, registry);
  factory SchedulePauseScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchedulePauseScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SchedulePauseScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SchedulePauseScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchedulePauseScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchedulePauseScheduleRequest copyWith(
          void Function(SchedulePauseScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SchedulePauseScheduleRequest))
          as SchedulePauseScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SchedulePauseScheduleRequest() / SchedulePauseScheduleRequest.new instead')
  static SchedulePauseScheduleRequest create() =>
      SchedulePauseScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SchedulePauseScheduleRequest._();
  @$core.override
  SchedulePauseScheduleRequest createEmptyInstance() =>
      SchedulePauseScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static SchedulePauseScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SchedulePauseScheduleRequest>(
          SchedulePauseScheduleRequest.$_createMessage);
  static SchedulePauseScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class SchedulePauseScheduleResponse extends $pb.GeneratedMessage {
  factory SchedulePauseScheduleResponse({
    Schedule? schedule,
  }) {
    final result = SchedulePauseScheduleResponse._();
    if (schedule != null) result.schedule = schedule;
    return result;
  }

  SchedulePauseScheduleResponse._();

  factory SchedulePauseScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchedulePauseScheduleResponse()..mergeFromBuffer(data, registry);
  factory SchedulePauseScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchedulePauseScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SchedulePauseScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SchedulePauseScheduleResponse.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchedulePauseScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchedulePauseScheduleResponse copyWith(
          void Function(SchedulePauseScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SchedulePauseScheduleResponse))
          as SchedulePauseScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SchedulePauseScheduleResponse() / SchedulePauseScheduleResponse.new instead')
  static SchedulePauseScheduleResponse create() =>
      SchedulePauseScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SchedulePauseScheduleResponse._();
  @$core.override
  SchedulePauseScheduleResponse createEmptyInstance() =>
      SchedulePauseScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static SchedulePauseScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SchedulePauseScheduleResponse>(
          SchedulePauseScheduleResponse.$_createMessage);
  static SchedulePauseScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);
}

class ScheduleResumeScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleResumeScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleResumeScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleResumeScheduleRequest._();

  factory ScheduleResumeScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleResumeScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleResumeScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleResumeScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleResumeScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleResumeScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleResumeScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleResumeScheduleRequest copyWith(
          void Function(ScheduleResumeScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleResumeScheduleRequest))
          as ScheduleResumeScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleResumeScheduleRequest() / ScheduleResumeScheduleRequest.new instead')
  static ScheduleResumeScheduleRequest create() =>
      ScheduleResumeScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleResumeScheduleRequest._();
  @$core.override
  ScheduleResumeScheduleRequest createEmptyInstance() =>
      ScheduleResumeScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleResumeScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleResumeScheduleRequest>(
          ScheduleResumeScheduleRequest.$_createMessage);
  static ScheduleResumeScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleResumeScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleResumeScheduleResponse({
    Schedule? schedule,
  }) {
    final result = ScheduleResumeScheduleResponse._();
    if (schedule != null) result.schedule = schedule;
    return result;
  }

  ScheduleResumeScheduleResponse._();

  factory ScheduleResumeScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleResumeScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleResumeScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleResumeScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleResumeScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleResumeScheduleResponse.$_createMessage)
    ..aOM<Schedule>(1, _omitFieldNames ? '' : 'schedule',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleResumeScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleResumeScheduleResponse copyWith(
          void Function(ScheduleResumeScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleResumeScheduleResponse))
          as ScheduleResumeScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleResumeScheduleResponse() / ScheduleResumeScheduleResponse.new instead')
  static ScheduleResumeScheduleResponse create() =>
      ScheduleResumeScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleResumeScheduleResponse._();
  @$core.override
  ScheduleResumeScheduleResponse createEmptyInstance() =>
      ScheduleResumeScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleResumeScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleResumeScheduleResponse>(
          ScheduleResumeScheduleResponse.$_createMessage);
  static ScheduleResumeScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Schedule get schedule => $_getN(0);
  @$pb.TagNumber(1)
  set schedule(Schedule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSchedule() => $_has(0);
  @$pb.TagNumber(1)
  void clearSchedule() => $_clearField(1);
  @$pb.TagNumber(1)
  Schedule ensureSchedule() => $_ensure(0);
}

class ScheduleListApplicationSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListApplicationSchedulesRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ScheduleListApplicationSchedulesRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ScheduleListApplicationSchedulesRequest._();

  factory ScheduleListApplicationSchedulesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListApplicationSchedulesRequest()
        ..mergeFromBuffer(data, registry);
  factory ScheduleListApplicationSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListApplicationSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListApplicationSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ScheduleListApplicationSchedulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListApplicationSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListApplicationSchedulesRequest copyWith(
          void Function(ScheduleListApplicationSchedulesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListApplicationSchedulesRequest))
          as ScheduleListApplicationSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListApplicationSchedulesRequest() / ScheduleListApplicationSchedulesRequest.new instead')
  static ScheduleListApplicationSchedulesRequest create() =>
      ScheduleListApplicationSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListApplicationSchedulesRequest._();
  @$core.override
  ScheduleListApplicationSchedulesRequest createEmptyInstance() =>
      ScheduleListApplicationSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListApplicationSchedulesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListApplicationSchedulesRequest>(
          ScheduleListApplicationSchedulesRequest.$_createMessage);
  static ScheduleListApplicationSchedulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ScheduleListApplicationSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListApplicationSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListApplicationSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListApplicationSchedulesResponse._();

  factory ScheduleListApplicationSchedulesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListApplicationSchedulesResponse()
        ..mergeFromBuffer(data, registry);
  factory ScheduleListApplicationSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListApplicationSchedulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListApplicationSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ScheduleListApplicationSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListApplicationSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListApplicationSchedulesResponse copyWith(
          void Function(ScheduleListApplicationSchedulesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListApplicationSchedulesResponse))
          as ScheduleListApplicationSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListApplicationSchedulesResponse() / ScheduleListApplicationSchedulesResponse.new instead')
  static ScheduleListApplicationSchedulesResponse create() =>
      ScheduleListApplicationSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListApplicationSchedulesResponse._();
  @$core.override
  ScheduleListApplicationSchedulesResponse createEmptyInstance() =>
      ScheduleListApplicationSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListApplicationSchedulesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListApplicationSchedulesResponse>(
          ScheduleListApplicationSchedulesResponse.$_createMessage);
  static ScheduleListApplicationSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleListComposeSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListComposeSchedulesRequest({
    $fixnum.Int64? composeId,
  }) {
    final result = ScheduleListComposeSchedulesRequest._();
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  ScheduleListComposeSchedulesRequest._();

  factory ScheduleListComposeSchedulesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListComposeSchedulesRequest()..mergeFromBuffer(data, registry);
  factory ScheduleListComposeSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListComposeSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListComposeSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListComposeSchedulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListComposeSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListComposeSchedulesRequest copyWith(
          void Function(ScheduleListComposeSchedulesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListComposeSchedulesRequest))
          as ScheduleListComposeSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListComposeSchedulesRequest() / ScheduleListComposeSchedulesRequest.new instead')
  static ScheduleListComposeSchedulesRequest create() =>
      ScheduleListComposeSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListComposeSchedulesRequest._();
  @$core.override
  ScheduleListComposeSchedulesRequest createEmptyInstance() =>
      ScheduleListComposeSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListComposeSchedulesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListComposeSchedulesRequest>(
          ScheduleListComposeSchedulesRequest.$_createMessage);
  static ScheduleListComposeSchedulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);
}

class ScheduleListComposeSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListComposeSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListComposeSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListComposeSchedulesResponse._();

  factory ScheduleListComposeSchedulesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListComposeSchedulesResponse()..mergeFromBuffer(data, registry);
  factory ScheduleListComposeSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListComposeSchedulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListComposeSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListComposeSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListComposeSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListComposeSchedulesResponse copyWith(
          void Function(ScheduleListComposeSchedulesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListComposeSchedulesResponse))
          as ScheduleListComposeSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListComposeSchedulesResponse() / ScheduleListComposeSchedulesResponse.new instead')
  static ScheduleListComposeSchedulesResponse create() =>
      ScheduleListComposeSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListComposeSchedulesResponse._();
  @$core.override
  ScheduleListComposeSchedulesResponse createEmptyInstance() =>
      ScheduleListComposeSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListComposeSchedulesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListComposeSchedulesResponse>(
          ScheduleListComposeSchedulesResponse.$_createMessage);
  static ScheduleListComposeSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleListDatabaseSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListDatabaseSchedulesRequest({
    $fixnum.Int64? databaseId,
  }) {
    final result = ScheduleListDatabaseSchedulesRequest._();
    if (databaseId != null) result.databaseId = databaseId;
    return result;
  }

  ScheduleListDatabaseSchedulesRequest._();

  factory ScheduleListDatabaseSchedulesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListDatabaseSchedulesRequest()..mergeFromBuffer(data, registry);
  factory ScheduleListDatabaseSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListDatabaseSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListDatabaseSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListDatabaseSchedulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'databaseId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListDatabaseSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListDatabaseSchedulesRequest copyWith(
          void Function(ScheduleListDatabaseSchedulesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListDatabaseSchedulesRequest))
          as ScheduleListDatabaseSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListDatabaseSchedulesRequest() / ScheduleListDatabaseSchedulesRequest.new instead')
  static ScheduleListDatabaseSchedulesRequest create() =>
      ScheduleListDatabaseSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListDatabaseSchedulesRequest._();
  @$core.override
  ScheduleListDatabaseSchedulesRequest createEmptyInstance() =>
      ScheduleListDatabaseSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListDatabaseSchedulesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListDatabaseSchedulesRequest>(
          ScheduleListDatabaseSchedulesRequest.$_createMessage);
  static ScheduleListDatabaseSchedulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get databaseId => $_getI64(0);
  @$pb.TagNumber(1)
  set databaseId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDatabaseId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDatabaseId() => $_clearField(1);
}

class ScheduleListDatabaseSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListDatabaseSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListDatabaseSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListDatabaseSchedulesResponse._();

  factory ScheduleListDatabaseSchedulesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListDatabaseSchedulesResponse()..mergeFromBuffer(data, registry);
  factory ScheduleListDatabaseSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListDatabaseSchedulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListDatabaseSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ScheduleListDatabaseSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListDatabaseSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListDatabaseSchedulesResponse copyWith(
          void Function(ScheduleListDatabaseSchedulesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListDatabaseSchedulesResponse))
          as ScheduleListDatabaseSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListDatabaseSchedulesResponse() / ScheduleListDatabaseSchedulesResponse.new instead')
  static ScheduleListDatabaseSchedulesResponse create() =>
      ScheduleListDatabaseSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListDatabaseSchedulesResponse._();
  @$core.override
  ScheduleListDatabaseSchedulesResponse createEmptyInstance() =>
      ScheduleListDatabaseSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListDatabaseSchedulesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListDatabaseSchedulesResponse>(
          ScheduleListDatabaseSchedulesResponse.$_createMessage);
  static ScheduleListDatabaseSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleListServerSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListServerSchedulesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = ScheduleListServerSchedulesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  ScheduleListServerSchedulesRequest._();

  factory ScheduleListServerSchedulesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListServerSchedulesRequest()..mergeFromBuffer(data, registry);
  factory ScheduleListServerSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListServerSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListServerSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListServerSchedulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListServerSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListServerSchedulesRequest copyWith(
          void Function(ScheduleListServerSchedulesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListServerSchedulesRequest))
          as ScheduleListServerSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListServerSchedulesRequest() / ScheduleListServerSchedulesRequest.new instead')
  static ScheduleListServerSchedulesRequest create() =>
      ScheduleListServerSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListServerSchedulesRequest._();
  @$core.override
  ScheduleListServerSchedulesRequest createEmptyInstance() =>
      ScheduleListServerSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListServerSchedulesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleListServerSchedulesRequest>(
          ScheduleListServerSchedulesRequest.$_createMessage);
  static ScheduleListServerSchedulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class ScheduleListServerSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListServerSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListServerSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListServerSchedulesResponse._();

  factory ScheduleListServerSchedulesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListServerSchedulesResponse()..mergeFromBuffer(data, registry);
  factory ScheduleListServerSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListServerSchedulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListServerSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListServerSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListServerSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListServerSchedulesResponse copyWith(
          void Function(ScheduleListServerSchedulesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListServerSchedulesResponse))
          as ScheduleListServerSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListServerSchedulesResponse() / ScheduleListServerSchedulesResponse.new instead')
  static ScheduleListServerSchedulesResponse create() =>
      ScheduleListServerSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListServerSchedulesResponse._();
  @$core.override
  ScheduleListServerSchedulesResponse createEmptyInstance() =>
      ScheduleListServerSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListServerSchedulesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListServerSchedulesResponse>(
          ScheduleListServerSchedulesResponse.$_createMessage);
  static ScheduleListServerSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleListOrganizationSchedulesRequest extends $pb.GeneratedMessage {
  factory ScheduleListOrganizationSchedulesRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ScheduleListOrganizationSchedulesRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ScheduleListOrganizationSchedulesRequest._();

  factory ScheduleListOrganizationSchedulesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListOrganizationSchedulesRequest()
        ..mergeFromBuffer(data, registry);
  factory ScheduleListOrganizationSchedulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListOrganizationSchedulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListOrganizationSchedulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ScheduleListOrganizationSchedulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListOrganizationSchedulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListOrganizationSchedulesRequest copyWith(
          void Function(ScheduleListOrganizationSchedulesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListOrganizationSchedulesRequest))
          as ScheduleListOrganizationSchedulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListOrganizationSchedulesRequest() / ScheduleListOrganizationSchedulesRequest.new instead')
  static ScheduleListOrganizationSchedulesRequest create() =>
      ScheduleListOrganizationSchedulesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListOrganizationSchedulesRequest._();
  @$core.override
  ScheduleListOrganizationSchedulesRequest createEmptyInstance() =>
      ScheduleListOrganizationSchedulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListOrganizationSchedulesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListOrganizationSchedulesRequest>(
          ScheduleListOrganizationSchedulesRequest.$_createMessage);
  static ScheduleListOrganizationSchedulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ScheduleListOrganizationSchedulesResponse extends $pb.GeneratedMessage {
  factory ScheduleListOrganizationSchedulesResponse({
    $core.Iterable<Schedule>? schedules,
  }) {
    final result = ScheduleListOrganizationSchedulesResponse._();
    if (schedules != null) result.schedules.addAll(schedules);
    return result;
  }

  ScheduleListOrganizationSchedulesResponse._();

  factory ScheduleListOrganizationSchedulesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListOrganizationSchedulesResponse()
        ..mergeFromBuffer(data, registry);
  factory ScheduleListOrganizationSchedulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListOrganizationSchedulesResponse()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListOrganizationSchedulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ScheduleListOrganizationSchedulesResponse.$_createMessage)
    ..pPM<Schedule>(1, _omitFieldNames ? '' : 'schedules',
        subBuilder: Schedule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListOrganizationSchedulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListOrganizationSchedulesResponse copyWith(
          void Function(ScheduleListOrganizationSchedulesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleListOrganizationSchedulesResponse))
          as ScheduleListOrganizationSchedulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListOrganizationSchedulesResponse() / ScheduleListOrganizationSchedulesResponse.new instead')
  static ScheduleListOrganizationSchedulesResponse create() =>
      ScheduleListOrganizationSchedulesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListOrganizationSchedulesResponse._();
  @$core.override
  ScheduleListOrganizationSchedulesResponse createEmptyInstance() =>
      ScheduleListOrganizationSchedulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListOrganizationSchedulesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleListOrganizationSchedulesResponse>(
          ScheduleListOrganizationSchedulesResponse.$_createMessage);
  static ScheduleListOrganizationSchedulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Schedule> get schedules => $_getList(0);
}

class ScheduleTriggerScheduleRequest extends $pb.GeneratedMessage {
  factory ScheduleTriggerScheduleRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleTriggerScheduleRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleTriggerScheduleRequest._();

  factory ScheduleTriggerScheduleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleTriggerScheduleRequest()..mergeFromBuffer(data, registry);
  factory ScheduleTriggerScheduleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleTriggerScheduleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleTriggerScheduleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleTriggerScheduleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleTriggerScheduleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleTriggerScheduleRequest copyWith(
          void Function(ScheduleTriggerScheduleRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleTriggerScheduleRequest))
          as ScheduleTriggerScheduleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleTriggerScheduleRequest() / ScheduleTriggerScheduleRequest.new instead')
  static ScheduleTriggerScheduleRequest create() =>
      ScheduleTriggerScheduleRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleTriggerScheduleRequest._();
  @$core.override
  ScheduleTriggerScheduleRequest createEmptyInstance() =>
      ScheduleTriggerScheduleRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleTriggerScheduleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleTriggerScheduleRequest>(
          ScheduleTriggerScheduleRequest.$_createMessage);
  static ScheduleTriggerScheduleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleTriggerScheduleResponse extends $pb.GeneratedMessage {
  factory ScheduleTriggerScheduleResponse({
    ScheduleRun? result,
  }) {
    final result$ = ScheduleTriggerScheduleResponse._();
    if (result != null) result$.result = result;
    return result$;
  }

  ScheduleTriggerScheduleResponse._();

  factory ScheduleTriggerScheduleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleTriggerScheduleResponse()..mergeFromBuffer(data, registry);
  factory ScheduleTriggerScheduleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleTriggerScheduleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleTriggerScheduleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleTriggerScheduleResponse.$_createMessage)
    ..aOM<ScheduleRun>(1, _omitFieldNames ? '' : 'result',
        subBuilder: ScheduleRun.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleTriggerScheduleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleTriggerScheduleResponse copyWith(
          void Function(ScheduleTriggerScheduleResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleTriggerScheduleResponse))
          as ScheduleTriggerScheduleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleTriggerScheduleResponse() / ScheduleTriggerScheduleResponse.new instead')
  static ScheduleTriggerScheduleResponse create() =>
      ScheduleTriggerScheduleResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleTriggerScheduleResponse._();
  @$core.override
  ScheduleTriggerScheduleResponse createEmptyInstance() =>
      ScheduleTriggerScheduleResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleTriggerScheduleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleTriggerScheduleResponse>(
          ScheduleTriggerScheduleResponse.$_createMessage);
  static ScheduleTriggerScheduleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleRun get result => $_getN(0);
  @$pb.TagNumber(1)
  set result(ScheduleRun value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleRun ensureResult() => $_ensure(0);
}

class ScheduleGetRuntimePolicyRequest extends $pb.GeneratedMessage {
  factory ScheduleGetRuntimePolicyRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleGetRuntimePolicyRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleGetRuntimePolicyRequest._();

  factory ScheduleGetRuntimePolicyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetRuntimePolicyRequest()..mergeFromBuffer(data, registry);
  factory ScheduleGetRuntimePolicyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetRuntimePolicyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetRuntimePolicyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetRuntimePolicyRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetRuntimePolicyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetRuntimePolicyRequest copyWith(
          void Function(ScheduleGetRuntimePolicyRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetRuntimePolicyRequest))
          as ScheduleGetRuntimePolicyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetRuntimePolicyRequest() / ScheduleGetRuntimePolicyRequest.new instead')
  static ScheduleGetRuntimePolicyRequest create() =>
      ScheduleGetRuntimePolicyRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetRuntimePolicyRequest._();
  @$core.override
  ScheduleGetRuntimePolicyRequest createEmptyInstance() =>
      ScheduleGetRuntimePolicyRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetRuntimePolicyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetRuntimePolicyRequest>(
          ScheduleGetRuntimePolicyRequest.$_createMessage);
  static ScheduleGetRuntimePolicyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleGetRuntimePolicyResponse extends $pb.GeneratedMessage {
  factory ScheduleGetRuntimePolicyResponse({
    ScheduleRuntimePolicy? policy,
  }) {
    final result = ScheduleGetRuntimePolicyResponse._();
    if (policy != null) result.policy = policy;
    return result;
  }

  ScheduleGetRuntimePolicyResponse._();

  factory ScheduleGetRuntimePolicyResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetRuntimePolicyResponse()..mergeFromBuffer(data, registry);
  factory ScheduleGetRuntimePolicyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetRuntimePolicyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetRuntimePolicyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetRuntimePolicyResponse.$_createMessage)
    ..aOM<ScheduleRuntimePolicy>(1, _omitFieldNames ? '' : 'policy',
        subBuilder: ScheduleRuntimePolicy.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetRuntimePolicyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetRuntimePolicyResponse copyWith(
          void Function(ScheduleGetRuntimePolicyResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetRuntimePolicyResponse))
          as ScheduleGetRuntimePolicyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetRuntimePolicyResponse() / ScheduleGetRuntimePolicyResponse.new instead')
  static ScheduleGetRuntimePolicyResponse create() =>
      ScheduleGetRuntimePolicyResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetRuntimePolicyResponse._();
  @$core.override
  ScheduleGetRuntimePolicyResponse createEmptyInstance() =>
      ScheduleGetRuntimePolicyResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetRuntimePolicyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetRuntimePolicyResponse>(
          ScheduleGetRuntimePolicyResponse.$_createMessage);
  static ScheduleGetRuntimePolicyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleRuntimePolicy get policy => $_getN(0);
  @$pb.TagNumber(1)
  set policy(ScheduleRuntimePolicy value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPolicy() => $_has(0);
  @$pb.TagNumber(1)
  void clearPolicy() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleRuntimePolicy ensurePolicy() => $_ensure(0);
}

class ScheduleUpdateRuntimePolicyRequest extends $pb.GeneratedMessage {
  factory ScheduleUpdateRuntimePolicyRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? retryCount,
    $fixnum.Int64? retryDelaySeconds,
    ScheduleMissedRunPolicy? missedRunPolicy,
    ScheduleConcurrencyPolicy? concurrencyPolicy,
    $fixnum.Int64? leaseSeconds,
    $core.bool? notifyOnSuccess,
    $core.bool? notifyOnFailure,
  }) {
    final result = ScheduleUpdateRuntimePolicyRequest._();
    if (id != null) result.id = id;
    if (retryCount != null) result.retryCount = retryCount;
    if (retryDelaySeconds != null) result.retryDelaySeconds = retryDelaySeconds;
    if (missedRunPolicy != null) result.missedRunPolicy = missedRunPolicy;
    if (concurrencyPolicy != null) result.concurrencyPolicy = concurrencyPolicy;
    if (leaseSeconds != null) result.leaseSeconds = leaseSeconds;
    if (notifyOnSuccess != null) result.notifyOnSuccess = notifyOnSuccess;
    if (notifyOnFailure != null) result.notifyOnFailure = notifyOnFailure;
    return result;
  }

  ScheduleUpdateRuntimePolicyRequest._();

  factory ScheduleUpdateRuntimePolicyRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateRuntimePolicyRequest()..mergeFromBuffer(data, registry);
  factory ScheduleUpdateRuntimePolicyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateRuntimePolicyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleUpdateRuntimePolicyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleUpdateRuntimePolicyRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'retryCount')
    ..aInt64(3, _omitFieldNames ? '' : 'retryDelaySeconds')
    ..aE<ScheduleMissedRunPolicy>(4, _omitFieldNames ? '' : 'missedRunPolicy',
        enumValues: ScheduleMissedRunPolicy.values)
    ..aE<ScheduleConcurrencyPolicy>(
        5, _omitFieldNames ? '' : 'concurrencyPolicy',
        enumValues: ScheduleConcurrencyPolicy.values)
    ..aInt64(6, _omitFieldNames ? '' : 'leaseSeconds')
    ..aOB(7, _omitFieldNames ? '' : 'notifyOnSuccess')
    ..aOB(8, _omitFieldNames ? '' : 'notifyOnFailure')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateRuntimePolicyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateRuntimePolicyRequest copyWith(
          void Function(ScheduleUpdateRuntimePolicyRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleUpdateRuntimePolicyRequest))
          as ScheduleUpdateRuntimePolicyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleUpdateRuntimePolicyRequest() / ScheduleUpdateRuntimePolicyRequest.new instead')
  static ScheduleUpdateRuntimePolicyRequest create() =>
      ScheduleUpdateRuntimePolicyRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleUpdateRuntimePolicyRequest._();
  @$core.override
  ScheduleUpdateRuntimePolicyRequest createEmptyInstance() =>
      ScheduleUpdateRuntimePolicyRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleUpdateRuntimePolicyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleUpdateRuntimePolicyRequest>(
          ScheduleUpdateRuntimePolicyRequest.$_createMessage);
  static ScheduleUpdateRuntimePolicyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get retryCount => $_getI64(1);
  @$pb.TagNumber(2)
  set retryCount($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRetryCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearRetryCount() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get retryDelaySeconds => $_getI64(2);
  @$pb.TagNumber(3)
  set retryDelaySeconds($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRetryDelaySeconds() => $_has(2);
  @$pb.TagNumber(3)
  void clearRetryDelaySeconds() => $_clearField(3);

  @$pb.TagNumber(4)
  ScheduleMissedRunPolicy get missedRunPolicy => $_getN(3);
  @$pb.TagNumber(4)
  set missedRunPolicy(ScheduleMissedRunPolicy value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMissedRunPolicy() => $_has(3);
  @$pb.TagNumber(4)
  void clearMissedRunPolicy() => $_clearField(4);

  @$pb.TagNumber(5)
  ScheduleConcurrencyPolicy get concurrencyPolicy => $_getN(4);
  @$pb.TagNumber(5)
  set concurrencyPolicy(ScheduleConcurrencyPolicy value) =>
      $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasConcurrencyPolicy() => $_has(4);
  @$pb.TagNumber(5)
  void clearConcurrencyPolicy() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get leaseSeconds => $_getI64(5);
  @$pb.TagNumber(6)
  set leaseSeconds($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLeaseSeconds() => $_has(5);
  @$pb.TagNumber(6)
  void clearLeaseSeconds() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get notifyOnSuccess => $_getBF(6);
  @$pb.TagNumber(7)
  set notifyOnSuccess($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasNotifyOnSuccess() => $_has(6);
  @$pb.TagNumber(7)
  void clearNotifyOnSuccess() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get notifyOnFailure => $_getBF(7);
  @$pb.TagNumber(8)
  set notifyOnFailure($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasNotifyOnFailure() => $_has(7);
  @$pb.TagNumber(8)
  void clearNotifyOnFailure() => $_clearField(8);
}

class ScheduleUpdateRuntimePolicyResponse extends $pb.GeneratedMessage {
  factory ScheduleUpdateRuntimePolicyResponse({
    ScheduleRuntimePolicy? policy,
  }) {
    final result = ScheduleUpdateRuntimePolicyResponse._();
    if (policy != null) result.policy = policy;
    return result;
  }

  ScheduleUpdateRuntimePolicyResponse._();

  factory ScheduleUpdateRuntimePolicyResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateRuntimePolicyResponse()..mergeFromBuffer(data, registry);
  factory ScheduleUpdateRuntimePolicyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleUpdateRuntimePolicyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleUpdateRuntimePolicyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleUpdateRuntimePolicyResponse.$_createMessage)
    ..aOM<ScheduleRuntimePolicy>(1, _omitFieldNames ? '' : 'policy',
        subBuilder: ScheduleRuntimePolicy.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateRuntimePolicyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleUpdateRuntimePolicyResponse copyWith(
          void Function(ScheduleUpdateRuntimePolicyResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ScheduleUpdateRuntimePolicyResponse))
          as ScheduleUpdateRuntimePolicyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleUpdateRuntimePolicyResponse() / ScheduleUpdateRuntimePolicyResponse.new instead')
  static ScheduleUpdateRuntimePolicyResponse create() =>
      ScheduleUpdateRuntimePolicyResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleUpdateRuntimePolicyResponse._();
  @$core.override
  ScheduleUpdateRuntimePolicyResponse createEmptyInstance() =>
      ScheduleUpdateRuntimePolicyResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleUpdateRuntimePolicyResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ScheduleUpdateRuntimePolicyResponse>(
          ScheduleUpdateRuntimePolicyResponse.$_createMessage);
  static ScheduleUpdateRuntimePolicyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleRuntimePolicy get policy => $_getN(0);
  @$pb.TagNumber(1)
  set policy(ScheduleRuntimePolicy value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPolicy() => $_has(0);
  @$pb.TagNumber(1)
  void clearPolicy() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleRuntimePolicy ensurePolicy() => $_ensure(0);
}

class ScheduleListExecutionsRequest extends $pb.GeneratedMessage {
  factory ScheduleListExecutionsRequest({
    $fixnum.Int64? scheduleId,
  }) {
    final result = ScheduleListExecutionsRequest._();
    if (scheduleId != null) result.scheduleId = scheduleId;
    return result;
  }

  ScheduleListExecutionsRequest._();

  factory ScheduleListExecutionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListExecutionsRequest()..mergeFromBuffer(data, registry);
  factory ScheduleListExecutionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListExecutionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListExecutionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListExecutionsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'scheduleId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListExecutionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListExecutionsRequest copyWith(
          void Function(ScheduleListExecutionsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleListExecutionsRequest))
          as ScheduleListExecutionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListExecutionsRequest() / ScheduleListExecutionsRequest.new instead')
  static ScheduleListExecutionsRequest create() =>
      ScheduleListExecutionsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListExecutionsRequest._();
  @$core.override
  ScheduleListExecutionsRequest createEmptyInstance() =>
      ScheduleListExecutionsRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListExecutionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleListExecutionsRequest>(
          ScheduleListExecutionsRequest.$_createMessage);
  static ScheduleListExecutionsRequest? _defaultInstance;

  /// Owner schedule id (former envelope `organization_id` slot).
  @$pb.TagNumber(1)
  $fixnum.Int64 get scheduleId => $_getI64(0);
  @$pb.TagNumber(1)
  set scheduleId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasScheduleId() => $_has(0);
  @$pb.TagNumber(1)
  void clearScheduleId() => $_clearField(1);
}

class ScheduleListExecutionsResponse extends $pb.GeneratedMessage {
  factory ScheduleListExecutionsResponse({
    $core.Iterable<ScheduleExecution>? executions,
  }) {
    final result = ScheduleListExecutionsResponse._();
    if (executions != null) result.executions.addAll(executions);
    return result;
  }

  ScheduleListExecutionsResponse._();

  factory ScheduleListExecutionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListExecutionsResponse()..mergeFromBuffer(data, registry);
  factory ScheduleListExecutionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleListExecutionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleListExecutionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleListExecutionsResponse.$_createMessage)
    ..pPM<ScheduleExecution>(1, _omitFieldNames ? '' : 'executions',
        subBuilder: ScheduleExecution.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListExecutionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleListExecutionsResponse copyWith(
          void Function(ScheduleListExecutionsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleListExecutionsResponse))
          as ScheduleListExecutionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleListExecutionsResponse() / ScheduleListExecutionsResponse.new instead')
  static ScheduleListExecutionsResponse create() =>
      ScheduleListExecutionsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleListExecutionsResponse._();
  @$core.override
  ScheduleListExecutionsResponse createEmptyInstance() =>
      ScheduleListExecutionsResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleListExecutionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleListExecutionsResponse>(
          ScheduleListExecutionsResponse.$_createMessage);
  static ScheduleListExecutionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ScheduleExecution> get executions => $_getList(0);
}

class ScheduleGetLogsRequest extends $pb.GeneratedMessage {
  factory ScheduleGetLogsRequest({
    $fixnum.Int64? id,
  }) {
    final result = ScheduleGetLogsRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ScheduleGetLogsRequest._();

  factory ScheduleGetLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetLogsRequest()..mergeFromBuffer(data, registry);
  factory ScheduleGetLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetLogsRequest copyWith(
          void Function(ScheduleGetLogsRequest) updates) =>
      super.copyWith((message) => updates(message as ScheduleGetLogsRequest))
          as ScheduleGetLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetLogsRequest() / ScheduleGetLogsRequest.new instead')
  static ScheduleGetLogsRequest create() => ScheduleGetLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleGetLogsRequest._();
  @$core.override
  ScheduleGetLogsRequest createEmptyInstance() => ScheduleGetLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetLogsRequest>(
          ScheduleGetLogsRequest.$_createMessage);
  static ScheduleGetLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ScheduleGetLogsResponse extends $pb.GeneratedMessage {
  factory ScheduleGetLogsResponse({
    ScheduleLog? log,
  }) {
    final result = ScheduleGetLogsResponse._();
    if (log != null) result.log = log;
    return result;
  }

  ScheduleGetLogsResponse._();

  factory ScheduleGetLogsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetLogsResponse()..mergeFromBuffer(data, registry);
  factory ScheduleGetLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetLogsResponse.$_createMessage)
    ..aOM<ScheduleLog>(1, _omitFieldNames ? '' : 'log',
        subBuilder: ScheduleLog.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetLogsResponse copyWith(
          void Function(ScheduleGetLogsResponse) updates) =>
      super.copyWith((message) => updates(message as ScheduleGetLogsResponse))
          as ScheduleGetLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetLogsResponse() / ScheduleGetLogsResponse.new instead')
  static ScheduleGetLogsResponse create() => ScheduleGetLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ScheduleGetLogsResponse._();
  @$core.override
  ScheduleGetLogsResponse createEmptyInstance() => ScheduleGetLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetLogsResponse>(
          ScheduleGetLogsResponse.$_createMessage);
  static ScheduleGetLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleLog get log => $_getN(0);
  @$pb.TagNumber(1)
  set log(ScheduleLog value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasLog() => $_has(0);
  @$pb.TagNumber(1)
  void clearLog() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleLog ensureLog() => $_ensure(0);
}

class ScheduleGetExecutionLogsRequest extends $pb.GeneratedMessage {
  factory ScheduleGetExecutionLogsRequest({
    $fixnum.Int64? scheduleId,
    $fixnum.Int64? executionId,
  }) {
    final result = ScheduleGetExecutionLogsRequest._();
    if (scheduleId != null) result.scheduleId = scheduleId;
    if (executionId != null) result.executionId = executionId;
    return result;
  }

  ScheduleGetExecutionLogsRequest._();

  factory ScheduleGetExecutionLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetExecutionLogsRequest()..mergeFromBuffer(data, registry);
  factory ScheduleGetExecutionLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetExecutionLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetExecutionLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetExecutionLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'scheduleId')
    ..aInt64(2, _omitFieldNames ? '' : 'executionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetExecutionLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetExecutionLogsRequest copyWith(
          void Function(ScheduleGetExecutionLogsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetExecutionLogsRequest))
          as ScheduleGetExecutionLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetExecutionLogsRequest() / ScheduleGetExecutionLogsRequest.new instead')
  static ScheduleGetExecutionLogsRequest create() =>
      ScheduleGetExecutionLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetExecutionLogsRequest._();
  @$core.override
  ScheduleGetExecutionLogsRequest createEmptyInstance() =>
      ScheduleGetExecutionLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetExecutionLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetExecutionLogsRequest>(
          ScheduleGetExecutionLogsRequest.$_createMessage);
  static ScheduleGetExecutionLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get scheduleId => $_getI64(0);
  @$pb.TagNumber(1)
  set scheduleId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasScheduleId() => $_has(0);
  @$pb.TagNumber(1)
  void clearScheduleId() => $_clearField(1);

  /// Execution row whose log file is read (former envelope `organization_id` slot).
  @$pb.TagNumber(2)
  $fixnum.Int64 get executionId => $_getI64(1);
  @$pb.TagNumber(2)
  set executionId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExecutionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearExecutionId() => $_clearField(2);
}

class ScheduleGetExecutionLogsResponse extends $pb.GeneratedMessage {
  factory ScheduleGetExecutionLogsResponse({
    ScheduleLog? log,
  }) {
    final result = ScheduleGetExecutionLogsResponse._();
    if (log != null) result.log = log;
    return result;
  }

  ScheduleGetExecutionLogsResponse._();

  factory ScheduleGetExecutionLogsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetExecutionLogsResponse()..mergeFromBuffer(data, registry);
  factory ScheduleGetExecutionLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ScheduleGetExecutionLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ScheduleGetExecutionLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ScheduleGetExecutionLogsResponse.$_createMessage)
    ..aOM<ScheduleLog>(1, _omitFieldNames ? '' : 'log',
        subBuilder: ScheduleLog.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetExecutionLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ScheduleGetExecutionLogsResponse copyWith(
          void Function(ScheduleGetExecutionLogsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ScheduleGetExecutionLogsResponse))
          as ScheduleGetExecutionLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ScheduleGetExecutionLogsResponse() / ScheduleGetExecutionLogsResponse.new instead')
  static ScheduleGetExecutionLogsResponse create() =>
      ScheduleGetExecutionLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ScheduleGetExecutionLogsResponse._();
  @$core.override
  ScheduleGetExecutionLogsResponse createEmptyInstance() =>
      ScheduleGetExecutionLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static ScheduleGetExecutionLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ScheduleGetExecutionLogsResponse>(
          ScheduleGetExecutionLogsResponse.$_createMessage);
  static ScheduleGetExecutionLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ScheduleLog get log => $_getN(0);
  @$pb.TagNumber(1)
  set log(ScheduleLog value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasLog() => $_has(0);
  @$pb.TagNumber(1)
  void clearLog() => $_clearField(1);
  @$pb.TagNumber(1)
  ScheduleLog ensureLog() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

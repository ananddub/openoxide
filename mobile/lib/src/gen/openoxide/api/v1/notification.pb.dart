// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/notification.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'notification.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'notification.pbenum.dart';

/// Notification rows never echo provider credentials; config is write-only.
class Notification extends $pb.GeneratedMessage {
  factory Notification({
    $fixnum.Int64? id,
    $core.String? name,
    NotificationProviderType? notificationType,
    $core.bool? onAppDeploy,
    $core.bool? onAppBuildError,
    $core.bool? onDatabaseBackup,
    $core.bool? onVolumeBackup,
    $core.bool? onPanelRestart,
    $core.bool? onDockerCleanup,
    $core.bool? onServerThreshold,
    $core.bool? onPanelBackup,
    $core.bool? onScheduleSuccess,
    $core.bool? onScheduleFailure,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Notification._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (notificationType != null) result.notificationType = notificationType;
    if (onAppDeploy != null) result.onAppDeploy = onAppDeploy;
    if (onAppBuildError != null) result.onAppBuildError = onAppBuildError;
    if (onDatabaseBackup != null) result.onDatabaseBackup = onDatabaseBackup;
    if (onVolumeBackup != null) result.onVolumeBackup = onVolumeBackup;
    if (onPanelRestart != null) result.onPanelRestart = onPanelRestart;
    if (onDockerCleanup != null) result.onDockerCleanup = onDockerCleanup;
    if (onServerThreshold != null) result.onServerThreshold = onServerThreshold;
    if (onPanelBackup != null) result.onPanelBackup = onPanelBackup;
    if (onScheduleSuccess != null) result.onScheduleSuccess = onScheduleSuccess;
    if (onScheduleFailure != null) result.onScheduleFailure = onScheduleFailure;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Notification._();

  factory Notification.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Notification()..mergeFromBuffer(data, registry);
  factory Notification.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Notification()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Notification',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Notification.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<NotificationProviderType>(3, _omitFieldNames ? '' : 'notificationType',
        enumValues: NotificationProviderType.values)
    ..aOB(4, _omitFieldNames ? '' : 'onAppDeploy')
    ..aOB(5, _omitFieldNames ? '' : 'onAppBuildError')
    ..aOB(6, _omitFieldNames ? '' : 'onDatabaseBackup')
    ..aOB(7, _omitFieldNames ? '' : 'onVolumeBackup')
    ..aOB(8, _omitFieldNames ? '' : 'onPanelRestart')
    ..aOB(9, _omitFieldNames ? '' : 'onDockerCleanup')
    ..aOB(10, _omitFieldNames ? '' : 'onServerThreshold')
    ..aOB(11, _omitFieldNames ? '' : 'onPanelBackup')
    ..aOB(12, _omitFieldNames ? '' : 'onScheduleSuccess')
    ..aOB(13, _omitFieldNames ? '' : 'onScheduleFailure')
    ..aInt64(14, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(15, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(16, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Notification clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Notification copyWith(void Function(Notification) updates) =>
      super.copyWith((message) => updates(message as Notification))
          as Notification;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Notification() / Notification.new instead')
  static Notification create() => Notification._();
  static $pb.GeneratedMessage $_createMessage() => Notification._();
  @$core.override
  Notification createEmptyInstance() => Notification._();
  @$core.pragma('dart2js:noInline')
  static Notification getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Notification>(
          Notification.$_createMessage);
  static Notification? _defaultInstance;

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
  NotificationProviderType get notificationType => $_getN(2);
  @$pb.TagNumber(3)
  set notificationType(NotificationProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasNotificationType() => $_has(2);
  @$pb.TagNumber(3)
  void clearNotificationType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get onAppDeploy => $_getBF(3);
  @$pb.TagNumber(4)
  set onAppDeploy($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOnAppDeploy() => $_has(3);
  @$pb.TagNumber(4)
  void clearOnAppDeploy() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get onAppBuildError => $_getBF(4);
  @$pb.TagNumber(5)
  set onAppBuildError($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOnAppBuildError() => $_has(4);
  @$pb.TagNumber(5)
  void clearOnAppBuildError() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get onDatabaseBackup => $_getBF(5);
  @$pb.TagNumber(6)
  set onDatabaseBackup($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOnDatabaseBackup() => $_has(5);
  @$pb.TagNumber(6)
  void clearOnDatabaseBackup() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get onVolumeBackup => $_getBF(6);
  @$pb.TagNumber(7)
  set onVolumeBackup($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasOnVolumeBackup() => $_has(6);
  @$pb.TagNumber(7)
  void clearOnVolumeBackup() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get onPanelRestart => $_getBF(7);
  @$pb.TagNumber(8)
  set onPanelRestart($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOnPanelRestart() => $_has(7);
  @$pb.TagNumber(8)
  void clearOnPanelRestart() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get onDockerCleanup => $_getBF(8);
  @$pb.TagNumber(9)
  set onDockerCleanup($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOnDockerCleanup() => $_has(8);
  @$pb.TagNumber(9)
  void clearOnDockerCleanup() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get onServerThreshold => $_getBF(9);
  @$pb.TagNumber(10)
  set onServerThreshold($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasOnServerThreshold() => $_has(9);
  @$pb.TagNumber(10)
  void clearOnServerThreshold() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get onPanelBackup => $_getBF(10);
  @$pb.TagNumber(11)
  set onPanelBackup($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasOnPanelBackup() => $_has(10);
  @$pb.TagNumber(11)
  void clearOnPanelBackup() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get onScheduleSuccess => $_getBF(11);
  @$pb.TagNumber(12)
  set onScheduleSuccess($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOnScheduleSuccess() => $_has(11);
  @$pb.TagNumber(12)
  void clearOnScheduleSuccess() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get onScheduleFailure => $_getBF(12);
  @$pb.TagNumber(13)
  set onScheduleFailure($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasOnScheduleFailure() => $_has(12);
  @$pb.TagNumber(13)
  void clearOnScheduleFailure() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get organizationId => $_getI64(13);
  @$pb.TagNumber(14)
  set organizationId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasOrganizationId() => $_has(13);
  @$pb.TagNumber(14)
  void clearOrganizationId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get createdAt => $_getI64(14);
  @$pb.TagNumber(15)
  set createdAt($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasCreatedAt() => $_has(14);
  @$pb.TagNumber(15)
  void clearCreatedAt() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get updatedAt => $_getI64(15);
  @$pb.TagNumber(16)
  set updatedAt($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasUpdatedAt() => $_has(15);
  @$pb.TagNumber(16)
  void clearUpdatedAt() => $_clearField(16);
}

class NotificationDeliveryAttempt extends $pb.GeneratedMessage {
  factory NotificationDeliveryAttempt({
    $fixnum.Int64? id,
    $fixnum.Int64? notificationId,
    $fixnum.Int64? organizationId,
    $core.String? triggerName,
    $core.String? correlationId,
    $core.String? status,
    $fixnum.Int64? attempt,
    $core.String? title,
    $core.String? body,
    $core.String? error,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? finishedAt,
  }) {
    final result = NotificationDeliveryAttempt._();
    if (id != null) result.id = id;
    if (notificationId != null) result.notificationId = notificationId;
    if (organizationId != null) result.organizationId = organizationId;
    if (triggerName != null) result.triggerName = triggerName;
    if (correlationId != null) result.correlationId = correlationId;
    if (status != null) result.status = status;
    if (attempt != null) result.attempt = attempt;
    if (title != null) result.title = title;
    if (body != null) result.body = body;
    if (error != null) result.error = error;
    if (createdAt != null) result.createdAt = createdAt;
    if (finishedAt != null) result.finishedAt = finishedAt;
    return result;
  }

  NotificationDeliveryAttempt._();

  factory NotificationDeliveryAttempt.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationDeliveryAttempt()..mergeFromBuffer(data, registry);
  factory NotificationDeliveryAttempt.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationDeliveryAttempt()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationDeliveryAttempt',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationDeliveryAttempt.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'notificationId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aOS(4, _omitFieldNames ? '' : 'triggerName')
    ..aOS(5, _omitFieldNames ? '' : 'correlationId')
    ..aOS(6, _omitFieldNames ? '' : 'status')
    ..aInt64(7, _omitFieldNames ? '' : 'attempt')
    ..aOS(8, _omitFieldNames ? '' : 'title')
    ..aOS(9, _omitFieldNames ? '' : 'body')
    ..aOS(10, _omitFieldNames ? '' : 'error')
    ..aInt64(11, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(12, _omitFieldNames ? '' : 'finishedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationDeliveryAttempt clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationDeliveryAttempt copyWith(
          void Function(NotificationDeliveryAttempt) updates) =>
      super.copyWith(
              (message) => updates(message as NotificationDeliveryAttempt))
          as NotificationDeliveryAttempt;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationDeliveryAttempt() / NotificationDeliveryAttempt.new instead')
  static NotificationDeliveryAttempt create() =>
      NotificationDeliveryAttempt._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationDeliveryAttempt._();
  @$core.override
  NotificationDeliveryAttempt createEmptyInstance() =>
      NotificationDeliveryAttempt._();
  @$core.pragma('dart2js:noInline')
  static NotificationDeliveryAttempt getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationDeliveryAttempt>(
          NotificationDeliveryAttempt.$_createMessage);
  static NotificationDeliveryAttempt? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get notificationId => $_getI64(1);
  @$pb.TagNumber(2)
  set notificationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNotificationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNotificationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get triggerName => $_getSZ(3);
  @$pb.TagNumber(4)
  set triggerName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTriggerName() => $_has(3);
  @$pb.TagNumber(4)
  void clearTriggerName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get correlationId => $_getSZ(4);
  @$pb.TagNumber(5)
  set correlationId($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCorrelationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearCorrelationId() => $_clearField(5);

  /// Delivery lifecycle status as recorded by the dispatcher (free-form string).
  @$pb.TagNumber(6)
  $core.String get status => $_getSZ(5);
  @$pb.TagNumber(6)
  set status($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearStatus() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get attempt => $_getI64(6);
  @$pb.TagNumber(7)
  set attempt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAttempt() => $_has(6);
  @$pb.TagNumber(7)
  void clearAttempt() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get title => $_getSZ(7);
  @$pb.TagNumber(8)
  set title($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasTitle() => $_has(7);
  @$pb.TagNumber(8)
  void clearTitle() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get body => $_getSZ(8);
  @$pb.TagNumber(9)
  set body($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasBody() => $_has(8);
  @$pb.TagNumber(9)
  void clearBody() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get error => $_getSZ(9);
  @$pb.TagNumber(10)
  set error($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasError() => $_has(9);
  @$pb.TagNumber(10)
  void clearError() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get createdAt => $_getI64(10);
  @$pb.TagNumber(11)
  set createdAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCreatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCreatedAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get finishedAt => $_getI64(11);
  @$pb.TagNumber(12)
  set finishedAt($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasFinishedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearFinishedAt() => $_clearField(12);
}

class NotificationBinding extends $pb.GeneratedMessage {
  factory NotificationBinding({
    $fixnum.Int64? id,
    $fixnum.Int64? notificationId,
    $fixnum.Int64? organizationId,
    NotificationResourceType? resourceType,
    $fixnum.Int64? resourceId,
    $fixnum.Int64? createdAt,
  }) {
    final result = NotificationBinding._();
    if (id != null) result.id = id;
    if (notificationId != null) result.notificationId = notificationId;
    if (organizationId != null) result.organizationId = organizationId;
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  NotificationBinding._();

  factory NotificationBinding.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationBinding()..mergeFromBuffer(data, registry);
  factory NotificationBinding.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationBinding()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationBinding',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationBinding.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'notificationId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aE<NotificationResourceType>(4, _omitFieldNames ? '' : 'resourceType',
        enumValues: NotificationResourceType.values)
    ..aInt64(5, _omitFieldNames ? '' : 'resourceId')
    ..aInt64(6, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationBinding clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationBinding copyWith(void Function(NotificationBinding) updates) =>
      super.copyWith((message) => updates(message as NotificationBinding))
          as NotificationBinding;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use NotificationBinding() / NotificationBinding.new instead')
  static NotificationBinding create() => NotificationBinding._();
  static $pb.GeneratedMessage $_createMessage() => NotificationBinding._();
  @$core.override
  NotificationBinding createEmptyInstance() => NotificationBinding._();
  @$core.pragma('dart2js:noInline')
  static NotificationBinding getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationBinding>(
          NotificationBinding.$_createMessage);
  static NotificationBinding? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get notificationId => $_getI64(1);
  @$pb.TagNumber(2)
  set notificationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNotificationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNotificationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  @$pb.TagNumber(4)
  NotificationResourceType get resourceType => $_getN(3);
  @$pb.TagNumber(4)
  set resourceType(NotificationResourceType value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasResourceType() => $_has(3);
  @$pb.TagNumber(4)
  void clearResourceType() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get resourceId => $_getI64(4);
  @$pb.TagNumber(5)
  set resourceId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasResourceId() => $_has(4);
  @$pb.TagNumber(5)
  void clearResourceId() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get createdAt => $_getI64(5);
  @$pb.TagNumber(6)
  set createdAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCreatedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedAt() => $_clearField(6);
}

class NotificationSlackConfig extends $pb.GeneratedMessage {
  factory NotificationSlackConfig({
    $core.String? webhookUrl,
    $core.String? channel,
  }) {
    final result = NotificationSlackConfig._();
    if (webhookUrl != null) result.webhookUrl = webhookUrl;
    if (channel != null) result.channel = channel;
    return result;
  }

  NotificationSlackConfig._();

  factory NotificationSlackConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationSlackConfig()..mergeFromBuffer(data, registry);
  factory NotificationSlackConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationSlackConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationSlackConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationSlackConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'webhookUrl')
    ..aOS(2, _omitFieldNames ? '' : 'channel')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationSlackConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationSlackConfig copyWith(
          void Function(NotificationSlackConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationSlackConfig))
          as NotificationSlackConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationSlackConfig() / NotificationSlackConfig.new instead')
  static NotificationSlackConfig create() => NotificationSlackConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationSlackConfig._();
  @$core.override
  NotificationSlackConfig createEmptyInstance() => NotificationSlackConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationSlackConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationSlackConfig>(
          NotificationSlackConfig.$_createMessage);
  static NotificationSlackConfig? _defaultInstance;

  /// Opaque Slack incoming-webhook URL embedding its secret token.
  @$pb.TagNumber(1)
  $core.String get webhookUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set webhookUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhookUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhookUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get channel => $_getSZ(1);
  @$pb.TagNumber(2)
  set channel($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChannel() => $_has(1);
  @$pb.TagNumber(2)
  void clearChannel() => $_clearField(2);
}

class NotificationTelegramConfig extends $pb.GeneratedMessage {
  factory NotificationTelegramConfig({
    $core.String? botToken,
    $core.String? chatId,
    $core.String? messageThreadId,
  }) {
    final result = NotificationTelegramConfig._();
    if (botToken != null) result.botToken = botToken;
    if (chatId != null) result.chatId = chatId;
    if (messageThreadId != null) result.messageThreadId = messageThreadId;
    return result;
  }

  NotificationTelegramConfig._();

  factory NotificationTelegramConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationTelegramConfig()..mergeFromBuffer(data, registry);
  factory NotificationTelegramConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationTelegramConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationTelegramConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationTelegramConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'botToken')
    ..aOS(2, _omitFieldNames ? '' : 'chatId')
    ..aOS(3, _omitFieldNames ? '' : 'messageThreadId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationTelegramConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationTelegramConfig copyWith(
          void Function(NotificationTelegramConfig) updates) =>
      super.copyWith(
              (message) => updates(message as NotificationTelegramConfig))
          as NotificationTelegramConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationTelegramConfig() / NotificationTelegramConfig.new instead')
  static NotificationTelegramConfig create() => NotificationTelegramConfig._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationTelegramConfig._();
  @$core.override
  NotificationTelegramConfig createEmptyInstance() =>
      NotificationTelegramConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationTelegramConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationTelegramConfig>(
          NotificationTelegramConfig.$_createMessage);
  static NotificationTelegramConfig? _defaultInstance;

  /// Opaque Telegram bot token (secret).
  @$pb.TagNumber(1)
  $core.String get botToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set botToken($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBotToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearBotToken() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get chatId => $_getSZ(1);
  @$pb.TagNumber(2)
  set chatId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChatId() => $_has(1);
  @$pb.TagNumber(2)
  void clearChatId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get messageThreadId => $_getSZ(2);
  @$pb.TagNumber(3)
  set messageThreadId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessageThreadId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageThreadId() => $_clearField(3);
}

class NotificationDiscordConfig extends $pb.GeneratedMessage {
  factory NotificationDiscordConfig({
    $core.String? webhookUrl,
    $core.bool? decoration,
  }) {
    final result = NotificationDiscordConfig._();
    if (webhookUrl != null) result.webhookUrl = webhookUrl;
    if (decoration != null) result.decoration = decoration;
    return result;
  }

  NotificationDiscordConfig._();

  factory NotificationDiscordConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationDiscordConfig()..mergeFromBuffer(data, registry);
  factory NotificationDiscordConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationDiscordConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationDiscordConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationDiscordConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'webhookUrl')
    ..aOB(2, _omitFieldNames ? '' : 'decoration')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationDiscordConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationDiscordConfig copyWith(
          void Function(NotificationDiscordConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationDiscordConfig))
          as NotificationDiscordConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationDiscordConfig() / NotificationDiscordConfig.new instead')
  static NotificationDiscordConfig create() => NotificationDiscordConfig._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationDiscordConfig._();
  @$core.override
  NotificationDiscordConfig createEmptyInstance() =>
      NotificationDiscordConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationDiscordConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationDiscordConfig>(
          NotificationDiscordConfig.$_createMessage);
  static NotificationDiscordConfig? _defaultInstance;

  /// Opaque Discord webhook URL embedding its secret token.
  @$pb.TagNumber(1)
  $core.String get webhookUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set webhookUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhookUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhookUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get decoration => $_getBF(1);
  @$pb.TagNumber(2)
  set decoration($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDecoration() => $_has(1);
  @$pb.TagNumber(2)
  void clearDecoration() => $_clearField(2);
}

class NotificationEmailConfig extends $pb.GeneratedMessage {
  factory NotificationEmailConfig({
    $core.String? smtpServer,
    $fixnum.Int64? smtpPort,
    $core.String? username,
    $core.String? password,
    $core.String? fromAddress,
    $core.Iterable<$core.String>? toAddresses,
  }) {
    final result = NotificationEmailConfig._();
    if (smtpServer != null) result.smtpServer = smtpServer;
    if (smtpPort != null) result.smtpPort = smtpPort;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    if (fromAddress != null) result.fromAddress = fromAddress;
    if (toAddresses != null) result.toAddresses.addAll(toAddresses);
    return result;
  }

  NotificationEmailConfig._();

  factory NotificationEmailConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationEmailConfig()..mergeFromBuffer(data, registry);
  factory NotificationEmailConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationEmailConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationEmailConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationEmailConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'smtpServer')
    ..aInt64(2, _omitFieldNames ? '' : 'smtpPort')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'password')
    ..aOS(5, _omitFieldNames ? '' : 'fromAddress')
    ..pPS(6, _omitFieldNames ? '' : 'toAddresses')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationEmailConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationEmailConfig copyWith(
          void Function(NotificationEmailConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationEmailConfig))
          as NotificationEmailConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationEmailConfig() / NotificationEmailConfig.new instead')
  static NotificationEmailConfig create() => NotificationEmailConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationEmailConfig._();
  @$core.override
  NotificationEmailConfig createEmptyInstance() => NotificationEmailConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationEmailConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationEmailConfig>(
          NotificationEmailConfig.$_createMessage);
  static NotificationEmailConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get smtpServer => $_getSZ(0);
  @$pb.TagNumber(1)
  set smtpServer($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSmtpServer() => $_has(0);
  @$pb.TagNumber(1)
  void clearSmtpServer() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get smtpPort => $_getI64(1);
  @$pb.TagNumber(2)
  set smtpPort($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSmtpPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearSmtpPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);

  /// SMTP account password (secret).
  @$pb.TagNumber(4)
  $core.String get password => $_getSZ(3);
  @$pb.TagNumber(4)
  set password($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPassword() => $_has(3);
  @$pb.TagNumber(4)
  void clearPassword() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get fromAddress => $_getSZ(4);
  @$pb.TagNumber(5)
  set fromAddress($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFromAddress() => $_has(4);
  @$pb.TagNumber(5)
  void clearFromAddress() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<$core.String> get toAddresses => $_getList(5);
}

class NotificationResendConfig extends $pb.GeneratedMessage {
  factory NotificationResendConfig({
    $core.String? apiKey,
    $core.String? fromAddress,
    $core.Iterable<$core.String>? toAddresses,
  }) {
    final result = NotificationResendConfig._();
    if (apiKey != null) result.apiKey = apiKey;
    if (fromAddress != null) result.fromAddress = fromAddress;
    if (toAddresses != null) result.toAddresses.addAll(toAddresses);
    return result;
  }

  NotificationResendConfig._();

  factory NotificationResendConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationResendConfig()..mergeFromBuffer(data, registry);
  factory NotificationResendConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationResendConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationResendConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationResendConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'apiKey')
    ..aOS(2, _omitFieldNames ? '' : 'fromAddress')
    ..pPS(3, _omitFieldNames ? '' : 'toAddresses')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationResendConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationResendConfig copyWith(
          void Function(NotificationResendConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationResendConfig))
          as NotificationResendConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationResendConfig() / NotificationResendConfig.new instead')
  static NotificationResendConfig create() => NotificationResendConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationResendConfig._();
  @$core.override
  NotificationResendConfig createEmptyInstance() =>
      NotificationResendConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationResendConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationResendConfig>(
          NotificationResendConfig.$_createMessage);
  static NotificationResendConfig? _defaultInstance;

  /// Resend API key (secret).
  @$pb.TagNumber(1)
  $core.String get apiKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set apiKey($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApiKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearApiKey() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get fromAddress => $_getSZ(1);
  @$pb.TagNumber(2)
  set fromAddress($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFromAddress() => $_has(1);
  @$pb.TagNumber(2)
  void clearFromAddress() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get toAddresses => $_getList(2);
}

class NotificationGotifyConfig extends $pb.GeneratedMessage {
  factory NotificationGotifyConfig({
    $core.String? serverUrl,
    $core.String? appToken,
    $fixnum.Int64? priority,
    $core.bool? decoration,
  }) {
    final result = NotificationGotifyConfig._();
    if (serverUrl != null) result.serverUrl = serverUrl;
    if (appToken != null) result.appToken = appToken;
    if (priority != null) result.priority = priority;
    if (decoration != null) result.decoration = decoration;
    return result;
  }

  NotificationGotifyConfig._();

  factory NotificationGotifyConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationGotifyConfig()..mergeFromBuffer(data, registry);
  factory NotificationGotifyConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationGotifyConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationGotifyConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationGotifyConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'serverUrl')
    ..aOS(2, _omitFieldNames ? '' : 'appToken')
    ..aInt64(3, _omitFieldNames ? '' : 'priority')
    ..aOB(4, _omitFieldNames ? '' : 'decoration')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationGotifyConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationGotifyConfig copyWith(
          void Function(NotificationGotifyConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationGotifyConfig))
          as NotificationGotifyConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationGotifyConfig() / NotificationGotifyConfig.new instead')
  static NotificationGotifyConfig create() => NotificationGotifyConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationGotifyConfig._();
  @$core.override
  NotificationGotifyConfig createEmptyInstance() =>
      NotificationGotifyConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationGotifyConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationGotifyConfig>(
          NotificationGotifyConfig.$_createMessage);
  static NotificationGotifyConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get serverUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerUrl() => $_clearField(1);

  /// Gotify application token (secret).
  @$pb.TagNumber(2)
  $core.String get appToken => $_getSZ(1);
  @$pb.TagNumber(2)
  set appToken($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAppToken() => $_has(1);
  @$pb.TagNumber(2)
  void clearAppToken() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get priority => $_getI64(2);
  @$pb.TagNumber(3)
  set priority($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPriority() => $_has(2);
  @$pb.TagNumber(3)
  void clearPriority() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get decoration => $_getBF(3);
  @$pb.TagNumber(4)
  set decoration($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDecoration() => $_has(3);
  @$pb.TagNumber(4)
  void clearDecoration() => $_clearField(4);
}

class NotificationNtfyConfig extends $pb.GeneratedMessage {
  factory NotificationNtfyConfig({
    $core.String? serverUrl,
    $core.String? topic,
    $core.String? accessToken,
    $fixnum.Int64? priority,
  }) {
    final result = NotificationNtfyConfig._();
    if (serverUrl != null) result.serverUrl = serverUrl;
    if (topic != null) result.topic = topic;
    if (accessToken != null) result.accessToken = accessToken;
    if (priority != null) result.priority = priority;
    return result;
  }

  NotificationNtfyConfig._();

  factory NotificationNtfyConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationNtfyConfig()..mergeFromBuffer(data, registry);
  factory NotificationNtfyConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationNtfyConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationNtfyConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationNtfyConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'serverUrl')
    ..aOS(2, _omitFieldNames ? '' : 'topic')
    ..aOS(3, _omitFieldNames ? '' : 'accessToken')
    ..aInt64(4, _omitFieldNames ? '' : 'priority')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationNtfyConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationNtfyConfig copyWith(
          void Function(NotificationNtfyConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationNtfyConfig))
          as NotificationNtfyConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationNtfyConfig() / NotificationNtfyConfig.new instead')
  static NotificationNtfyConfig create() => NotificationNtfyConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationNtfyConfig._();
  @$core.override
  NotificationNtfyConfig createEmptyInstance() => NotificationNtfyConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationNtfyConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationNtfyConfig>(
          NotificationNtfyConfig.$_createMessage);
  static NotificationNtfyConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get serverUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get topic => $_getSZ(1);
  @$pb.TagNumber(2)
  set topic($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTopic() => $_has(1);
  @$pb.TagNumber(2)
  void clearTopic() => $_clearField(2);

  /// Optional ntfy access token (secret).
  @$pb.TagNumber(3)
  $core.String get accessToken => $_getSZ(2);
  @$pb.TagNumber(3)
  set accessToken($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAccessToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccessToken() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get priority => $_getI64(3);
  @$pb.TagNumber(4)
  set priority($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPriority() => $_has(3);
  @$pb.TagNumber(4)
  void clearPriority() => $_clearField(4);
}

class NotificationMattermostConfig extends $pb.GeneratedMessage {
  factory NotificationMattermostConfig({
    $core.String? webhookUrl,
    $core.String? channel,
    $core.String? username,
  }) {
    final result = NotificationMattermostConfig._();
    if (webhookUrl != null) result.webhookUrl = webhookUrl;
    if (channel != null) result.channel = channel;
    if (username != null) result.username = username;
    return result;
  }

  NotificationMattermostConfig._();

  factory NotificationMattermostConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationMattermostConfig()..mergeFromBuffer(data, registry);
  factory NotificationMattermostConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationMattermostConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationMattermostConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationMattermostConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'webhookUrl')
    ..aOS(2, _omitFieldNames ? '' : 'channel')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationMattermostConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationMattermostConfig copyWith(
          void Function(NotificationMattermostConfig) updates) =>
      super.copyWith(
              (message) => updates(message as NotificationMattermostConfig))
          as NotificationMattermostConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationMattermostConfig() / NotificationMattermostConfig.new instead')
  static NotificationMattermostConfig create() =>
      NotificationMattermostConfig._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationMattermostConfig._();
  @$core.override
  NotificationMattermostConfig createEmptyInstance() =>
      NotificationMattermostConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationMattermostConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationMattermostConfig>(
          NotificationMattermostConfig.$_createMessage);
  static NotificationMattermostConfig? _defaultInstance;

  /// Opaque Mattermost webhook URL embedding its secret token.
  @$pb.TagNumber(1)
  $core.String get webhookUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set webhookUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhookUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhookUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get channel => $_getSZ(1);
  @$pb.TagNumber(2)
  set channel($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChannel() => $_has(1);
  @$pb.TagNumber(2)
  void clearChannel() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);
}

class NotificationCustomConfig extends $pb.GeneratedMessage {
  factory NotificationCustomConfig({
    $core.String? endpoint,
    $core.String? headers,
  }) {
    final result = NotificationCustomConfig._();
    if (endpoint != null) result.endpoint = endpoint;
    if (headers != null) result.headers = headers;
    return result;
  }

  NotificationCustomConfig._();

  factory NotificationCustomConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationCustomConfig()..mergeFromBuffer(data, registry);
  factory NotificationCustomConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationCustomConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationCustomConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationCustomConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'endpoint')
    ..aOS(2, _omitFieldNames ? '' : 'headers')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationCustomConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationCustomConfig copyWith(
          void Function(NotificationCustomConfig) updates) =>
      super.copyWith((message) => updates(message as NotificationCustomConfig))
          as NotificationCustomConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationCustomConfig() / NotificationCustomConfig.new instead')
  static NotificationCustomConfig create() => NotificationCustomConfig._();
  static $pb.GeneratedMessage $_createMessage() => NotificationCustomConfig._();
  @$core.override
  NotificationCustomConfig createEmptyInstance() =>
      NotificationCustomConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationCustomConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationCustomConfig>(
          NotificationCustomConfig.$_createMessage);
  static NotificationCustomConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get endpoint => $_getSZ(0);
  @$pb.TagNumber(1)
  set endpoint($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEndpoint() => $_has(0);
  @$pb.TagNumber(1)
  void clearEndpoint() => $_clearField(1);

  /// Extra HTTP headers as a JSON object document.
  @$pb.TagNumber(2)
  $core.String get headers => $_getSZ(1);
  @$pb.TagNumber(2)
  set headers($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHeaders() => $_has(1);
  @$pb.TagNumber(2)
  void clearHeaders() => $_clearField(2);
}

/// Shared by the LARK and TEAMS webhook-url providers.
class NotificationWebhookUrlConfig extends $pb.GeneratedMessage {
  factory NotificationWebhookUrlConfig({
    $core.String? webhookUrl,
  }) {
    final result = NotificationWebhookUrlConfig._();
    if (webhookUrl != null) result.webhookUrl = webhookUrl;
    return result;
  }

  NotificationWebhookUrlConfig._();

  factory NotificationWebhookUrlConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationWebhookUrlConfig()..mergeFromBuffer(data, registry);
  factory NotificationWebhookUrlConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationWebhookUrlConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationWebhookUrlConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationWebhookUrlConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'webhookUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationWebhookUrlConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationWebhookUrlConfig copyWith(
          void Function(NotificationWebhookUrlConfig) updates) =>
      super.copyWith(
              (message) => updates(message as NotificationWebhookUrlConfig))
          as NotificationWebhookUrlConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationWebhookUrlConfig() / NotificationWebhookUrlConfig.new instead')
  static NotificationWebhookUrlConfig create() =>
      NotificationWebhookUrlConfig._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationWebhookUrlConfig._();
  @$core.override
  NotificationWebhookUrlConfig createEmptyInstance() =>
      NotificationWebhookUrlConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationWebhookUrlConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationWebhookUrlConfig>(
          NotificationWebhookUrlConfig.$_createMessage);
  static NotificationWebhookUrlConfig? _defaultInstance;

  /// Opaque provider webhook URL embedding its secret token.
  @$pb.TagNumber(1)
  $core.String get webhookUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set webhookUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhookUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhookUrl() => $_clearField(1);
}

class NotificationPushoverConfig extends $pb.GeneratedMessage {
  factory NotificationPushoverConfig({
    $core.String? userKey,
    $core.String? apiToken,
    $fixnum.Int64? priority,
    $fixnum.Int64? retry,
    $fixnum.Int64? expire,
  }) {
    final result = NotificationPushoverConfig._();
    if (userKey != null) result.userKey = userKey;
    if (apiToken != null) result.apiToken = apiToken;
    if (priority != null) result.priority = priority;
    if (retry != null) result.retry = retry;
    if (expire != null) result.expire = expire;
    return result;
  }

  NotificationPushoverConfig._();

  factory NotificationPushoverConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationPushoverConfig()..mergeFromBuffer(data, registry);
  factory NotificationPushoverConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NotificationPushoverConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NotificationPushoverConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NotificationPushoverConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'userKey')
    ..aOS(2, _omitFieldNames ? '' : 'apiToken')
    ..aInt64(3, _omitFieldNames ? '' : 'priority')
    ..aInt64(4, _omitFieldNames ? '' : 'retry')
    ..aInt64(5, _omitFieldNames ? '' : 'expire')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationPushoverConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NotificationPushoverConfig copyWith(
          void Function(NotificationPushoverConfig) updates) =>
      super.copyWith(
              (message) => updates(message as NotificationPushoverConfig))
          as NotificationPushoverConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NotificationPushoverConfig() / NotificationPushoverConfig.new instead')
  static NotificationPushoverConfig create() => NotificationPushoverConfig._();
  static $pb.GeneratedMessage $_createMessage() =>
      NotificationPushoverConfig._();
  @$core.override
  NotificationPushoverConfig createEmptyInstance() =>
      NotificationPushoverConfig._();
  @$core.pragma('dart2js:noInline')
  static NotificationPushoverConfig getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NotificationPushoverConfig>(
          NotificationPushoverConfig.$_createMessage);
  static NotificationPushoverConfig? _defaultInstance;

  /// Pushover user key (secret).
  @$pb.TagNumber(1)
  $core.String get userKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set userKey($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserKey() => $_clearField(1);

  /// Pushover application API token (secret).
  @$pb.TagNumber(2)
  $core.String get apiToken => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiToken($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiToken() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiToken() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get priority => $_getI64(2);
  @$pb.TagNumber(3)
  set priority($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPriority() => $_has(2);
  @$pb.TagNumber(3)
  void clearPriority() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get retry => $_getI64(3);
  @$pb.TagNumber(4)
  set retry($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRetry() => $_has(3);
  @$pb.TagNumber(4)
  void clearRetry() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get expire => $_getI64(4);
  @$pb.TagNumber(5)
  set expire($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasExpire() => $_has(4);
  @$pb.TagNumber(5)
  void clearExpire() => $_clearField(5);
}

class ListNotificationsRequest extends $pb.GeneratedMessage {
  factory ListNotificationsRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListNotificationsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListNotificationsRequest._();

  factory ListNotificationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationsRequest()..mergeFromBuffer(data, registry);
  factory ListNotificationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListNotificationsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsRequest copyWith(
          void Function(ListNotificationsRequest) updates) =>
      super.copyWith((message) => updates(message as ListNotificationsRequest))
          as ListNotificationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationsRequest() / ListNotificationsRequest.new instead')
  static ListNotificationsRequest create() => ListNotificationsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListNotificationsRequest._();
  @$core.override
  ListNotificationsRequest createEmptyInstance() =>
      ListNotificationsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationsRequest>(
          ListNotificationsRequest.$_createMessage);
  static ListNotificationsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListNotificationsResponse extends $pb.GeneratedMessage {
  factory ListNotificationsResponse({
    $core.Iterable<Notification>? notifications,
  }) {
    final result = ListNotificationsResponse._();
    if (notifications != null) result.notifications.addAll(notifications);
    return result;
  }

  ListNotificationsResponse._();

  factory ListNotificationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationsResponse()..mergeFromBuffer(data, registry);
  factory ListNotificationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListNotificationsResponse.$_createMessage)
    ..pPM<Notification>(1, _omitFieldNames ? '' : 'notifications',
        subBuilder: Notification.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsResponse copyWith(
          void Function(ListNotificationsResponse) updates) =>
      super.copyWith((message) => updates(message as ListNotificationsResponse))
          as ListNotificationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationsResponse() / ListNotificationsResponse.new instead')
  static ListNotificationsResponse create() => ListNotificationsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListNotificationsResponse._();
  @$core.override
  ListNotificationsResponse createEmptyInstance() =>
      ListNotificationsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationsResponse>(
          ListNotificationsResponse.$_createMessage);
  static ListNotificationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Notification> get notifications => $_getList(0);
}

class GetNotificationRequest extends $pb.GeneratedMessage {
  factory GetNotificationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = GetNotificationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  GetNotificationRequest._();

  factory GetNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetNotificationRequest()..mergeFromBuffer(data, registry);
  factory GetNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetNotificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetNotificationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetNotificationRequest copyWith(
          void Function(GetNotificationRequest) updates) =>
      super.copyWith((message) => updates(message as GetNotificationRequest))
          as GetNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetNotificationRequest() / GetNotificationRequest.new instead')
  static GetNotificationRequest create() => GetNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetNotificationRequest._();
  @$core.override
  GetNotificationRequest createEmptyInstance() => GetNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static GetNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetNotificationRequest>(
          GetNotificationRequest.$_createMessage);
  static GetNotificationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class GetNotificationResponse extends $pb.GeneratedMessage {
  factory GetNotificationResponse({
    Notification? notification,
  }) {
    final result = GetNotificationResponse._();
    if (notification != null) result.notification = notification;
    return result;
  }

  GetNotificationResponse._();

  factory GetNotificationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetNotificationResponse()..mergeFromBuffer(data, registry);
  factory GetNotificationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetNotificationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetNotificationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetNotificationResponse.$_createMessage)
    ..aOM<Notification>(1, _omitFieldNames ? '' : 'notification',
        subBuilder: Notification.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetNotificationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetNotificationResponse copyWith(
          void Function(GetNotificationResponse) updates) =>
      super.copyWith((message) => updates(message as GetNotificationResponse))
          as GetNotificationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetNotificationResponse() / GetNotificationResponse.new instead')
  static GetNotificationResponse create() => GetNotificationResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetNotificationResponse._();
  @$core.override
  GetNotificationResponse createEmptyInstance() => GetNotificationResponse._();
  @$core.pragma('dart2js:noInline')
  static GetNotificationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetNotificationResponse>(
          GetNotificationResponse.$_createMessage);
  static GetNotificationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Notification get notification => $_getN(0);
  @$pb.TagNumber(1)
  set notification(Notification value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNotification() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotification() => $_clearField(1);
  @$pb.TagNumber(1)
  Notification ensureNotification() => $_ensure(0);
}

enum CreateNotificationRequest_Provider {
  slack,
  telegram,
  discord,
  email,
  resend,
  gotify,
  ntfy,
  mattermost,
  custom,
  lark,
  pushover,
  teams,
  notSet
}

class CreateNotificationRequest extends $pb.GeneratedMessage {
  factory CreateNotificationRequest({
    $core.String? name,
    $fixnum.Int64? organizationId,
    $core.bool? onAppDeploy,
    $core.bool? onAppBuildError,
    $core.bool? onDatabaseBackup,
    $core.bool? onVolumeBackup,
    $core.bool? onPanelRestart,
    $core.bool? onDockerCleanup,
    $core.bool? onServerThreshold,
    $core.bool? onPanelBackup,
    $core.bool? onScheduleSuccess,
    $core.bool? onScheduleFailure,
    NotificationSlackConfig? slack,
    NotificationTelegramConfig? telegram,
    NotificationDiscordConfig? discord,
    NotificationEmailConfig? email,
    NotificationResendConfig? resend,
    NotificationGotifyConfig? gotify,
    NotificationNtfyConfig? ntfy,
    NotificationMattermostConfig? mattermost,
    NotificationCustomConfig? custom,
    NotificationWebhookUrlConfig? lark,
    NotificationPushoverConfig? pushover,
    NotificationWebhookUrlConfig? teams,
  }) {
    final result = CreateNotificationRequest._();
    if (name != null) result.name = name;
    if (organizationId != null) result.organizationId = organizationId;
    if (onAppDeploy != null) result.onAppDeploy = onAppDeploy;
    if (onAppBuildError != null) result.onAppBuildError = onAppBuildError;
    if (onDatabaseBackup != null) result.onDatabaseBackup = onDatabaseBackup;
    if (onVolumeBackup != null) result.onVolumeBackup = onVolumeBackup;
    if (onPanelRestart != null) result.onPanelRestart = onPanelRestart;
    if (onDockerCleanup != null) result.onDockerCleanup = onDockerCleanup;
    if (onServerThreshold != null) result.onServerThreshold = onServerThreshold;
    if (onPanelBackup != null) result.onPanelBackup = onPanelBackup;
    if (onScheduleSuccess != null) result.onScheduleSuccess = onScheduleSuccess;
    if (onScheduleFailure != null) result.onScheduleFailure = onScheduleFailure;
    if (slack != null) result.slack = slack;
    if (telegram != null) result.telegram = telegram;
    if (discord != null) result.discord = discord;
    if (email != null) result.email = email;
    if (resend != null) result.resend = resend;
    if (gotify != null) result.gotify = gotify;
    if (ntfy != null) result.ntfy = ntfy;
    if (mattermost != null) result.mattermost = mattermost;
    if (custom != null) result.custom = custom;
    if (lark != null) result.lark = lark;
    if (pushover != null) result.pushover = pushover;
    if (teams != null) result.teams = teams;
    return result;
  }

  CreateNotificationRequest._();

  factory CreateNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationRequest()..mergeFromBuffer(data, registry);
  factory CreateNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, CreateNotificationRequest_Provider>
      _CreateNotificationRequest_ProviderByTag = {
    13: CreateNotificationRequest_Provider.slack,
    14: CreateNotificationRequest_Provider.telegram,
    15: CreateNotificationRequest_Provider.discord,
    16: CreateNotificationRequest_Provider.email,
    17: CreateNotificationRequest_Provider.resend,
    18: CreateNotificationRequest_Provider.gotify,
    19: CreateNotificationRequest_Provider.ntfy,
    20: CreateNotificationRequest_Provider.mattermost,
    21: CreateNotificationRequest_Provider.custom,
    22: CreateNotificationRequest_Provider.lark,
    23: CreateNotificationRequest_Provider.pushover,
    24: CreateNotificationRequest_Provider.teams,
    0: CreateNotificationRequest_Provider.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateNotificationRequest.$_createMessage)
    ..oo(0, [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24])
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOB(3, _omitFieldNames ? '' : 'onAppDeploy')
    ..aOB(4, _omitFieldNames ? '' : 'onAppBuildError')
    ..aOB(5, _omitFieldNames ? '' : 'onDatabaseBackup')
    ..aOB(6, _omitFieldNames ? '' : 'onVolumeBackup')
    ..aOB(7, _omitFieldNames ? '' : 'onPanelRestart')
    ..aOB(8, _omitFieldNames ? '' : 'onDockerCleanup')
    ..aOB(9, _omitFieldNames ? '' : 'onServerThreshold')
    ..aOB(10, _omitFieldNames ? '' : 'onPanelBackup')
    ..aOB(11, _omitFieldNames ? '' : 'onScheduleSuccess')
    ..aOB(12, _omitFieldNames ? '' : 'onScheduleFailure')
    ..aOM<NotificationSlackConfig>(13, _omitFieldNames ? '' : 'slack',
        subBuilder: NotificationSlackConfig.$_createMessage)
    ..aOM<NotificationTelegramConfig>(14, _omitFieldNames ? '' : 'telegram',
        subBuilder: NotificationTelegramConfig.$_createMessage)
    ..aOM<NotificationDiscordConfig>(15, _omitFieldNames ? '' : 'discord',
        subBuilder: NotificationDiscordConfig.$_createMessage)
    ..aOM<NotificationEmailConfig>(16, _omitFieldNames ? '' : 'email',
        subBuilder: NotificationEmailConfig.$_createMessage)
    ..aOM<NotificationResendConfig>(17, _omitFieldNames ? '' : 'resend',
        subBuilder: NotificationResendConfig.$_createMessage)
    ..aOM<NotificationGotifyConfig>(18, _omitFieldNames ? '' : 'gotify',
        subBuilder: NotificationGotifyConfig.$_createMessage)
    ..aOM<NotificationNtfyConfig>(19, _omitFieldNames ? '' : 'ntfy',
        subBuilder: NotificationNtfyConfig.$_createMessage)
    ..aOM<NotificationMattermostConfig>(20, _omitFieldNames ? '' : 'mattermost',
        subBuilder: NotificationMattermostConfig.$_createMessage)
    ..aOM<NotificationCustomConfig>(21, _omitFieldNames ? '' : 'custom',
        subBuilder: NotificationCustomConfig.$_createMessage)
    ..aOM<NotificationWebhookUrlConfig>(22, _omitFieldNames ? '' : 'lark',
        subBuilder: NotificationWebhookUrlConfig.$_createMessage)
    ..aOM<NotificationPushoverConfig>(23, _omitFieldNames ? '' : 'pushover',
        subBuilder: NotificationPushoverConfig.$_createMessage)
    ..aOM<NotificationWebhookUrlConfig>(24, _omitFieldNames ? '' : 'teams',
        subBuilder: NotificationWebhookUrlConfig.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationRequest copyWith(
          void Function(CreateNotificationRequest) updates) =>
      super.copyWith((message) => updates(message as CreateNotificationRequest))
          as CreateNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateNotificationRequest() / CreateNotificationRequest.new instead')
  static CreateNotificationRequest create() => CreateNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateNotificationRequest._();
  @$core.override
  CreateNotificationRequest createEmptyInstance() =>
      CreateNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateNotificationRequest>(
          CreateNotificationRequest.$_createMessage);
  static CreateNotificationRequest? _defaultInstance;

  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  @$pb.TagNumber(17)
  @$pb.TagNumber(18)
  @$pb.TagNumber(19)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  CreateNotificationRequest_Provider whichProvider() =>
      _CreateNotificationRequest_ProviderByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  @$pb.TagNumber(17)
  @$pb.TagNumber(18)
  @$pb.TagNumber(19)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  void clearProvider() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get onAppDeploy => $_getBF(2);
  @$pb.TagNumber(3)
  set onAppDeploy($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOnAppDeploy() => $_has(2);
  @$pb.TagNumber(3)
  void clearOnAppDeploy() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get onAppBuildError => $_getBF(3);
  @$pb.TagNumber(4)
  set onAppBuildError($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOnAppBuildError() => $_has(3);
  @$pb.TagNumber(4)
  void clearOnAppBuildError() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get onDatabaseBackup => $_getBF(4);
  @$pb.TagNumber(5)
  set onDatabaseBackup($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOnDatabaseBackup() => $_has(4);
  @$pb.TagNumber(5)
  void clearOnDatabaseBackup() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get onVolumeBackup => $_getBF(5);
  @$pb.TagNumber(6)
  set onVolumeBackup($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOnVolumeBackup() => $_has(5);
  @$pb.TagNumber(6)
  void clearOnVolumeBackup() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get onPanelRestart => $_getBF(6);
  @$pb.TagNumber(7)
  set onPanelRestart($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasOnPanelRestart() => $_has(6);
  @$pb.TagNumber(7)
  void clearOnPanelRestart() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get onDockerCleanup => $_getBF(7);
  @$pb.TagNumber(8)
  set onDockerCleanup($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOnDockerCleanup() => $_has(7);
  @$pb.TagNumber(8)
  void clearOnDockerCleanup() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get onServerThreshold => $_getBF(8);
  @$pb.TagNumber(9)
  set onServerThreshold($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOnServerThreshold() => $_has(8);
  @$pb.TagNumber(9)
  void clearOnServerThreshold() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get onPanelBackup => $_getBF(9);
  @$pb.TagNumber(10)
  set onPanelBackup($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasOnPanelBackup() => $_has(9);
  @$pb.TagNumber(10)
  void clearOnPanelBackup() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get onScheduleSuccess => $_getBF(10);
  @$pb.TagNumber(11)
  set onScheduleSuccess($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasOnScheduleSuccess() => $_has(10);
  @$pb.TagNumber(11)
  void clearOnScheduleSuccess() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get onScheduleFailure => $_getBF(11);
  @$pb.TagNumber(12)
  set onScheduleFailure($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOnScheduleFailure() => $_has(11);
  @$pb.TagNumber(12)
  void clearOnScheduleFailure() => $_clearField(12);

  @$pb.TagNumber(13)
  NotificationSlackConfig get slack => $_getN(12);
  @$pb.TagNumber(13)
  set slack(NotificationSlackConfig value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasSlack() => $_has(12);
  @$pb.TagNumber(13)
  void clearSlack() => $_clearField(13);
  @$pb.TagNumber(13)
  NotificationSlackConfig ensureSlack() => $_ensure(12);

  @$pb.TagNumber(14)
  NotificationTelegramConfig get telegram => $_getN(13);
  @$pb.TagNumber(14)
  set telegram(NotificationTelegramConfig value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasTelegram() => $_has(13);
  @$pb.TagNumber(14)
  void clearTelegram() => $_clearField(14);
  @$pb.TagNumber(14)
  NotificationTelegramConfig ensureTelegram() => $_ensure(13);

  @$pb.TagNumber(15)
  NotificationDiscordConfig get discord => $_getN(14);
  @$pb.TagNumber(15)
  set discord(NotificationDiscordConfig value) => $_setField(15, value);
  @$pb.TagNumber(15)
  $core.bool hasDiscord() => $_has(14);
  @$pb.TagNumber(15)
  void clearDiscord() => $_clearField(15);
  @$pb.TagNumber(15)
  NotificationDiscordConfig ensureDiscord() => $_ensure(14);

  @$pb.TagNumber(16)
  NotificationEmailConfig get email => $_getN(15);
  @$pb.TagNumber(16)
  set email(NotificationEmailConfig value) => $_setField(16, value);
  @$pb.TagNumber(16)
  $core.bool hasEmail() => $_has(15);
  @$pb.TagNumber(16)
  void clearEmail() => $_clearField(16);
  @$pb.TagNumber(16)
  NotificationEmailConfig ensureEmail() => $_ensure(15);

  @$pb.TagNumber(17)
  NotificationResendConfig get resend => $_getN(16);
  @$pb.TagNumber(17)
  set resend(NotificationResendConfig value) => $_setField(17, value);
  @$pb.TagNumber(17)
  $core.bool hasResend() => $_has(16);
  @$pb.TagNumber(17)
  void clearResend() => $_clearField(17);
  @$pb.TagNumber(17)
  NotificationResendConfig ensureResend() => $_ensure(16);

  @$pb.TagNumber(18)
  NotificationGotifyConfig get gotify => $_getN(17);
  @$pb.TagNumber(18)
  set gotify(NotificationGotifyConfig value) => $_setField(18, value);
  @$pb.TagNumber(18)
  $core.bool hasGotify() => $_has(17);
  @$pb.TagNumber(18)
  void clearGotify() => $_clearField(18);
  @$pb.TagNumber(18)
  NotificationGotifyConfig ensureGotify() => $_ensure(17);

  @$pb.TagNumber(19)
  NotificationNtfyConfig get ntfy => $_getN(18);
  @$pb.TagNumber(19)
  set ntfy(NotificationNtfyConfig value) => $_setField(19, value);
  @$pb.TagNumber(19)
  $core.bool hasNtfy() => $_has(18);
  @$pb.TagNumber(19)
  void clearNtfy() => $_clearField(19);
  @$pb.TagNumber(19)
  NotificationNtfyConfig ensureNtfy() => $_ensure(18);

  @$pb.TagNumber(20)
  NotificationMattermostConfig get mattermost => $_getN(19);
  @$pb.TagNumber(20)
  set mattermost(NotificationMattermostConfig value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasMattermost() => $_has(19);
  @$pb.TagNumber(20)
  void clearMattermost() => $_clearField(20);
  @$pb.TagNumber(20)
  NotificationMattermostConfig ensureMattermost() => $_ensure(19);

  @$pb.TagNumber(21)
  NotificationCustomConfig get custom => $_getN(20);
  @$pb.TagNumber(21)
  set custom(NotificationCustomConfig value) => $_setField(21, value);
  @$pb.TagNumber(21)
  $core.bool hasCustom() => $_has(20);
  @$pb.TagNumber(21)
  void clearCustom() => $_clearField(21);
  @$pb.TagNumber(21)
  NotificationCustomConfig ensureCustom() => $_ensure(20);

  @$pb.TagNumber(22)
  NotificationWebhookUrlConfig get lark => $_getN(21);
  @$pb.TagNumber(22)
  set lark(NotificationWebhookUrlConfig value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasLark() => $_has(21);
  @$pb.TagNumber(22)
  void clearLark() => $_clearField(22);
  @$pb.TagNumber(22)
  NotificationWebhookUrlConfig ensureLark() => $_ensure(21);

  @$pb.TagNumber(23)
  NotificationPushoverConfig get pushover => $_getN(22);
  @$pb.TagNumber(23)
  set pushover(NotificationPushoverConfig value) => $_setField(23, value);
  @$pb.TagNumber(23)
  $core.bool hasPushover() => $_has(22);
  @$pb.TagNumber(23)
  void clearPushover() => $_clearField(23);
  @$pb.TagNumber(23)
  NotificationPushoverConfig ensurePushover() => $_ensure(22);

  @$pb.TagNumber(24)
  NotificationWebhookUrlConfig get teams => $_getN(23);
  @$pb.TagNumber(24)
  set teams(NotificationWebhookUrlConfig value) => $_setField(24, value);
  @$pb.TagNumber(24)
  $core.bool hasTeams() => $_has(23);
  @$pb.TagNumber(24)
  void clearTeams() => $_clearField(24);
  @$pb.TagNumber(24)
  NotificationWebhookUrlConfig ensureTeams() => $_ensure(23);
}

class CreateNotificationResponse extends $pb.GeneratedMessage {
  factory CreateNotificationResponse({
    Notification? notification,
  }) {
    final result = CreateNotificationResponse._();
    if (notification != null) result.notification = notification;
    return result;
  }

  CreateNotificationResponse._();

  factory CreateNotificationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationResponse()..mergeFromBuffer(data, registry);
  factory CreateNotificationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateNotificationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateNotificationResponse.$_createMessage)
    ..aOM<Notification>(1, _omitFieldNames ? '' : 'notification',
        subBuilder: Notification.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationResponse copyWith(
          void Function(CreateNotificationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateNotificationResponse))
          as CreateNotificationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateNotificationResponse() / CreateNotificationResponse.new instead')
  static CreateNotificationResponse create() => CreateNotificationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateNotificationResponse._();
  @$core.override
  CreateNotificationResponse createEmptyInstance() =>
      CreateNotificationResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateNotificationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateNotificationResponse>(
          CreateNotificationResponse.$_createMessage);
  static CreateNotificationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Notification get notification => $_getN(0);
  @$pb.TagNumber(1)
  set notification(Notification value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNotification() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotification() => $_clearField(1);
  @$pb.TagNumber(1)
  Notification ensureNotification() => $_ensure(0);
}

/// PATCH semantics: absent fields keep their stored values. Provider config fields update the stored channel config in place; the provider kind itself is immutable here (use ReplaceNotification to change it).
class UpdateNotificationRequest extends $pb.GeneratedMessage {
  factory UpdateNotificationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $core.String? name,
    $core.bool? onAppDeploy,
    $core.bool? onAppBuildError,
    $core.bool? onDatabaseBackup,
    $core.bool? onVolumeBackup,
    $core.bool? onPanelRestart,
    $core.bool? onDockerCleanup,
    $core.bool? onServerThreshold,
    $core.bool? onPanelBackup,
    $core.bool? onScheduleSuccess,
    $core.bool? onScheduleFailure,
    $core.String? webhookUrl,
    $core.String? channel,
    $core.String? username,
    $fixnum.Int64? decoration,
    $core.String? botToken,
    $core.String? chatId,
    $core.String? messageThreadId,
    $core.String? smtpServer,
    $fixnum.Int64? smtpPort,
    $core.String? password,
    $core.String? fromAddress,
    $core.Iterable<$core.String>? toAddresses,
    $core.String? apiKey,
    $core.String? serverUrl,
    $core.String? appToken,
    $fixnum.Int64? priority,
    $core.String? topic,
    $core.String? accessToken,
    $core.String? endpoint,
    $core.String? headers,
    $core.String? userKey,
    $core.String? apiToken,
    $fixnum.Int64? retry,
    $fixnum.Int64? expire,
  }) {
    final result = UpdateNotificationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (name != null) result.name = name;
    if (onAppDeploy != null) result.onAppDeploy = onAppDeploy;
    if (onAppBuildError != null) result.onAppBuildError = onAppBuildError;
    if (onDatabaseBackup != null) result.onDatabaseBackup = onDatabaseBackup;
    if (onVolumeBackup != null) result.onVolumeBackup = onVolumeBackup;
    if (onPanelRestart != null) result.onPanelRestart = onPanelRestart;
    if (onDockerCleanup != null) result.onDockerCleanup = onDockerCleanup;
    if (onServerThreshold != null) result.onServerThreshold = onServerThreshold;
    if (onPanelBackup != null) result.onPanelBackup = onPanelBackup;
    if (onScheduleSuccess != null) result.onScheduleSuccess = onScheduleSuccess;
    if (onScheduleFailure != null) result.onScheduleFailure = onScheduleFailure;
    if (webhookUrl != null) result.webhookUrl = webhookUrl;
    if (channel != null) result.channel = channel;
    if (username != null) result.username = username;
    if (decoration != null) result.decoration = decoration;
    if (botToken != null) result.botToken = botToken;
    if (chatId != null) result.chatId = chatId;
    if (messageThreadId != null) result.messageThreadId = messageThreadId;
    if (smtpServer != null) result.smtpServer = smtpServer;
    if (smtpPort != null) result.smtpPort = smtpPort;
    if (password != null) result.password = password;
    if (fromAddress != null) result.fromAddress = fromAddress;
    if (toAddresses != null) result.toAddresses.addAll(toAddresses);
    if (apiKey != null) result.apiKey = apiKey;
    if (serverUrl != null) result.serverUrl = serverUrl;
    if (appToken != null) result.appToken = appToken;
    if (priority != null) result.priority = priority;
    if (topic != null) result.topic = topic;
    if (accessToken != null) result.accessToken = accessToken;
    if (endpoint != null) result.endpoint = endpoint;
    if (headers != null) result.headers = headers;
    if (userKey != null) result.userKey = userKey;
    if (apiToken != null) result.apiToken = apiToken;
    if (retry != null) result.retry = retry;
    if (expire != null) result.expire = expire;
    return result;
  }

  UpdateNotificationRequest._();

  factory UpdateNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateNotificationRequest()..mergeFromBuffer(data, registry);
  factory UpdateNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateNotificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateNotificationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOB(4, _omitFieldNames ? '' : 'onAppDeploy')
    ..aOB(5, _omitFieldNames ? '' : 'onAppBuildError')
    ..aOB(6, _omitFieldNames ? '' : 'onDatabaseBackup')
    ..aOB(7, _omitFieldNames ? '' : 'onVolumeBackup')
    ..aOB(8, _omitFieldNames ? '' : 'onPanelRestart')
    ..aOB(9, _omitFieldNames ? '' : 'onDockerCleanup')
    ..aOB(10, _omitFieldNames ? '' : 'onServerThreshold')
    ..aOB(11, _omitFieldNames ? '' : 'onPanelBackup')
    ..aOB(12, _omitFieldNames ? '' : 'onScheduleSuccess')
    ..aOB(13, _omitFieldNames ? '' : 'onScheduleFailure')
    ..aOS(14, _omitFieldNames ? '' : 'webhookUrl')
    ..aOS(15, _omitFieldNames ? '' : 'channel')
    ..aOS(16, _omitFieldNames ? '' : 'username')
    ..aInt64(17, _omitFieldNames ? '' : 'decoration')
    ..aOS(18, _omitFieldNames ? '' : 'botToken')
    ..aOS(19, _omitFieldNames ? '' : 'chatId')
    ..aOS(20, _omitFieldNames ? '' : 'messageThreadId')
    ..aOS(21, _omitFieldNames ? '' : 'smtpServer')
    ..aInt64(22, _omitFieldNames ? '' : 'smtpPort')
    ..aOS(23, _omitFieldNames ? '' : 'password')
    ..aOS(24, _omitFieldNames ? '' : 'fromAddress')
    ..pPS(25, _omitFieldNames ? '' : 'toAddresses')
    ..aOS(26, _omitFieldNames ? '' : 'apiKey')
    ..aOS(27, _omitFieldNames ? '' : 'serverUrl')
    ..aOS(28, _omitFieldNames ? '' : 'appToken')
    ..aInt64(29, _omitFieldNames ? '' : 'priority')
    ..aOS(30, _omitFieldNames ? '' : 'topic')
    ..aOS(31, _omitFieldNames ? '' : 'accessToken')
    ..aOS(32, _omitFieldNames ? '' : 'endpoint')
    ..aOS(33, _omitFieldNames ? '' : 'headers')
    ..aOS(34, _omitFieldNames ? '' : 'userKey')
    ..aOS(35, _omitFieldNames ? '' : 'apiToken')
    ..aInt64(36, _omitFieldNames ? '' : 'retry')
    ..aInt64(37, _omitFieldNames ? '' : 'expire')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateNotificationRequest copyWith(
          void Function(UpdateNotificationRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateNotificationRequest))
          as UpdateNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateNotificationRequest() / UpdateNotificationRequest.new instead')
  static UpdateNotificationRequest create() => UpdateNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateNotificationRequest._();
  @$core.override
  UpdateNotificationRequest createEmptyInstance() =>
      UpdateNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateNotificationRequest>(
          UpdateNotificationRequest.$_createMessage);
  static UpdateNotificationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get onAppDeploy => $_getBF(3);
  @$pb.TagNumber(4)
  set onAppDeploy($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOnAppDeploy() => $_has(3);
  @$pb.TagNumber(4)
  void clearOnAppDeploy() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get onAppBuildError => $_getBF(4);
  @$pb.TagNumber(5)
  set onAppBuildError($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOnAppBuildError() => $_has(4);
  @$pb.TagNumber(5)
  void clearOnAppBuildError() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get onDatabaseBackup => $_getBF(5);
  @$pb.TagNumber(6)
  set onDatabaseBackup($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOnDatabaseBackup() => $_has(5);
  @$pb.TagNumber(6)
  void clearOnDatabaseBackup() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get onVolumeBackup => $_getBF(6);
  @$pb.TagNumber(7)
  set onVolumeBackup($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasOnVolumeBackup() => $_has(6);
  @$pb.TagNumber(7)
  void clearOnVolumeBackup() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get onPanelRestart => $_getBF(7);
  @$pb.TagNumber(8)
  set onPanelRestart($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOnPanelRestart() => $_has(7);
  @$pb.TagNumber(8)
  void clearOnPanelRestart() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get onDockerCleanup => $_getBF(8);
  @$pb.TagNumber(9)
  set onDockerCleanup($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOnDockerCleanup() => $_has(8);
  @$pb.TagNumber(9)
  void clearOnDockerCleanup() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get onServerThreshold => $_getBF(9);
  @$pb.TagNumber(10)
  set onServerThreshold($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasOnServerThreshold() => $_has(9);
  @$pb.TagNumber(10)
  void clearOnServerThreshold() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get onPanelBackup => $_getBF(10);
  @$pb.TagNumber(11)
  set onPanelBackup($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasOnPanelBackup() => $_has(10);
  @$pb.TagNumber(11)
  void clearOnPanelBackup() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get onScheduleSuccess => $_getBF(11);
  @$pb.TagNumber(12)
  set onScheduleSuccess($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOnScheduleSuccess() => $_has(11);
  @$pb.TagNumber(12)
  void clearOnScheduleSuccess() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get onScheduleFailure => $_getBF(12);
  @$pb.TagNumber(13)
  set onScheduleFailure($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasOnScheduleFailure() => $_has(12);
  @$pb.TagNumber(13)
  void clearOnScheduleFailure() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get webhookUrl => $_getSZ(13);
  @$pb.TagNumber(14)
  set webhookUrl($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasWebhookUrl() => $_has(13);
  @$pb.TagNumber(14)
  void clearWebhookUrl() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get channel => $_getSZ(14);
  @$pb.TagNumber(15)
  set channel($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasChannel() => $_has(14);
  @$pb.TagNumber(15)
  void clearChannel() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get username => $_getSZ(15);
  @$pb.TagNumber(16)
  set username($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasUsername() => $_has(15);
  @$pb.TagNumber(16)
  void clearUsername() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get decoration => $_getI64(16);
  @$pb.TagNumber(17)
  set decoration($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasDecoration() => $_has(16);
  @$pb.TagNumber(17)
  void clearDecoration() => $_clearField(17);

  /// Opaque Telegram bot token (secret).
  @$pb.TagNumber(18)
  $core.String get botToken => $_getSZ(17);
  @$pb.TagNumber(18)
  set botToken($core.String value) => $_setString(17, value);
  @$pb.TagNumber(18)
  $core.bool hasBotToken() => $_has(17);
  @$pb.TagNumber(18)
  void clearBotToken() => $_clearField(18);

  @$pb.TagNumber(19)
  $core.String get chatId => $_getSZ(18);
  @$pb.TagNumber(19)
  set chatId($core.String value) => $_setString(18, value);
  @$pb.TagNumber(19)
  $core.bool hasChatId() => $_has(18);
  @$pb.TagNumber(19)
  void clearChatId() => $_clearField(19);

  @$pb.TagNumber(20)
  $core.String get messageThreadId => $_getSZ(19);
  @$pb.TagNumber(20)
  set messageThreadId($core.String value) => $_setString(19, value);
  @$pb.TagNumber(20)
  $core.bool hasMessageThreadId() => $_has(19);
  @$pb.TagNumber(20)
  void clearMessageThreadId() => $_clearField(20);

  @$pb.TagNumber(21)
  $core.String get smtpServer => $_getSZ(20);
  @$pb.TagNumber(21)
  set smtpServer($core.String value) => $_setString(20, value);
  @$pb.TagNumber(21)
  $core.bool hasSmtpServer() => $_has(20);
  @$pb.TagNumber(21)
  void clearSmtpServer() => $_clearField(21);

  @$pb.TagNumber(22)
  $fixnum.Int64 get smtpPort => $_getI64(21);
  @$pb.TagNumber(22)
  set smtpPort($fixnum.Int64 value) => $_setInt64(21, value);
  @$pb.TagNumber(22)
  $core.bool hasSmtpPort() => $_has(21);
  @$pb.TagNumber(22)
  void clearSmtpPort() => $_clearField(22);

  /// SMTP account password (secret).
  @$pb.TagNumber(23)
  $core.String get password => $_getSZ(22);
  @$pb.TagNumber(23)
  set password($core.String value) => $_setString(22, value);
  @$pb.TagNumber(23)
  $core.bool hasPassword() => $_has(22);
  @$pb.TagNumber(23)
  void clearPassword() => $_clearField(23);

  @$pb.TagNumber(24)
  $core.String get fromAddress => $_getSZ(23);
  @$pb.TagNumber(24)
  set fromAddress($core.String value) => $_setString(23, value);
  @$pb.TagNumber(24)
  $core.bool hasFromAddress() => $_has(23);
  @$pb.TagNumber(24)
  void clearFromAddress() => $_clearField(24);

  /// Present-and-nonempty replaces the stored recipient list.
  @$pb.TagNumber(25)
  $pb.PbList<$core.String> get toAddresses => $_getList(24);

  /// Resend API key (secret).
  @$pb.TagNumber(26)
  $core.String get apiKey => $_getSZ(25);
  @$pb.TagNumber(26)
  set apiKey($core.String value) => $_setString(25, value);
  @$pb.TagNumber(26)
  $core.bool hasApiKey() => $_has(25);
  @$pb.TagNumber(26)
  void clearApiKey() => $_clearField(26);

  @$pb.TagNumber(27)
  $core.String get serverUrl => $_getSZ(26);
  @$pb.TagNumber(27)
  set serverUrl($core.String value) => $_setString(26, value);
  @$pb.TagNumber(27)
  $core.bool hasServerUrl() => $_has(26);
  @$pb.TagNumber(27)
  void clearServerUrl() => $_clearField(27);

  /// Gotify application token (secret).
  @$pb.TagNumber(28)
  $core.String get appToken => $_getSZ(27);
  @$pb.TagNumber(28)
  set appToken($core.String value) => $_setString(27, value);
  @$pb.TagNumber(28)
  $core.bool hasAppToken() => $_has(27);
  @$pb.TagNumber(28)
  void clearAppToken() => $_clearField(28);

  @$pb.TagNumber(29)
  $fixnum.Int64 get priority => $_getI64(28);
  @$pb.TagNumber(29)
  set priority($fixnum.Int64 value) => $_setInt64(28, value);
  @$pb.TagNumber(29)
  $core.bool hasPriority() => $_has(28);
  @$pb.TagNumber(29)
  void clearPriority() => $_clearField(29);

  @$pb.TagNumber(30)
  $core.String get topic => $_getSZ(29);
  @$pb.TagNumber(30)
  set topic($core.String value) => $_setString(29, value);
  @$pb.TagNumber(30)
  $core.bool hasTopic() => $_has(29);
  @$pb.TagNumber(30)
  void clearTopic() => $_clearField(30);

  /// Optional ntfy access token (secret).
  @$pb.TagNumber(31)
  $core.String get accessToken => $_getSZ(30);
  @$pb.TagNumber(31)
  set accessToken($core.String value) => $_setString(30, value);
  @$pb.TagNumber(31)
  $core.bool hasAccessToken() => $_has(30);
  @$pb.TagNumber(31)
  void clearAccessToken() => $_clearField(31);

  @$pb.TagNumber(32)
  $core.String get endpoint => $_getSZ(31);
  @$pb.TagNumber(32)
  set endpoint($core.String value) => $_setString(31, value);
  @$pb.TagNumber(32)
  $core.bool hasEndpoint() => $_has(31);
  @$pb.TagNumber(32)
  void clearEndpoint() => $_clearField(32);

  /// Extra HTTP headers as a JSON object document.
  @$pb.TagNumber(33)
  $core.String get headers => $_getSZ(32);
  @$pb.TagNumber(33)
  set headers($core.String value) => $_setString(32, value);
  @$pb.TagNumber(33)
  $core.bool hasHeaders() => $_has(32);
  @$pb.TagNumber(33)
  void clearHeaders() => $_clearField(33);

  /// Pushover user key (secret).
  @$pb.TagNumber(34)
  $core.String get userKey => $_getSZ(33);
  @$pb.TagNumber(34)
  set userKey($core.String value) => $_setString(33, value);
  @$pb.TagNumber(34)
  $core.bool hasUserKey() => $_has(33);
  @$pb.TagNumber(34)
  void clearUserKey() => $_clearField(34);

  /// Pushover application API token (secret).
  @$pb.TagNumber(35)
  $core.String get apiToken => $_getSZ(34);
  @$pb.TagNumber(35)
  set apiToken($core.String value) => $_setString(34, value);
  @$pb.TagNumber(35)
  $core.bool hasApiToken() => $_has(34);
  @$pb.TagNumber(35)
  void clearApiToken() => $_clearField(35);

  @$pb.TagNumber(36)
  $fixnum.Int64 get retry => $_getI64(35);
  @$pb.TagNumber(36)
  set retry($fixnum.Int64 value) => $_setInt64(35, value);
  @$pb.TagNumber(36)
  $core.bool hasRetry() => $_has(35);
  @$pb.TagNumber(36)
  void clearRetry() => $_clearField(36);

  @$pb.TagNumber(37)
  $fixnum.Int64 get expire => $_getI64(36);
  @$pb.TagNumber(37)
  set expire($fixnum.Int64 value) => $_setInt64(36, value);
  @$pb.TagNumber(37)
  $core.bool hasExpire() => $_has(36);
  @$pb.TagNumber(37)
  void clearExpire() => $_clearField(37);
}

class UpdateNotificationResponse extends $pb.GeneratedMessage {
  factory UpdateNotificationResponse({
    Notification? notification,
  }) {
    final result = UpdateNotificationResponse._();
    if (notification != null) result.notification = notification;
    return result;
  }

  UpdateNotificationResponse._();

  factory UpdateNotificationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateNotificationResponse()..mergeFromBuffer(data, registry);
  factory UpdateNotificationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateNotificationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateNotificationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateNotificationResponse.$_createMessage)
    ..aOM<Notification>(1, _omitFieldNames ? '' : 'notification',
        subBuilder: Notification.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateNotificationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateNotificationResponse copyWith(
          void Function(UpdateNotificationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateNotificationResponse))
          as UpdateNotificationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateNotificationResponse() / UpdateNotificationResponse.new instead')
  static UpdateNotificationResponse create() => UpdateNotificationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateNotificationResponse._();
  @$core.override
  UpdateNotificationResponse createEmptyInstance() =>
      UpdateNotificationResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateNotificationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateNotificationResponse>(
          UpdateNotificationResponse.$_createMessage);
  static UpdateNotificationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Notification get notification => $_getN(0);
  @$pb.TagNumber(1)
  set notification(Notification value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNotification() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotification() => $_clearField(1);
  @$pb.TagNumber(1)
  Notification ensureNotification() => $_ensure(0);
}

enum ReplaceNotificationRequest_Provider {
  slack,
  telegram,
  discord,
  email,
  resend,
  gotify,
  ntfy,
  mattermost,
  custom,
  lark,
  pushover,
  teams,
  notSet
}

/// Full replacement of the notification, including its provider kind.
class ReplaceNotificationRequest extends $pb.GeneratedMessage {
  factory ReplaceNotificationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $core.String? name,
    $core.bool? onAppDeploy,
    $core.bool? onAppBuildError,
    $core.bool? onDatabaseBackup,
    $core.bool? onVolumeBackup,
    $core.bool? onPanelRestart,
    $core.bool? onDockerCleanup,
    $core.bool? onServerThreshold,
    $core.bool? onPanelBackup,
    $core.bool? onScheduleSuccess,
    $core.bool? onScheduleFailure,
    NotificationSlackConfig? slack,
    NotificationTelegramConfig? telegram,
    NotificationDiscordConfig? discord,
    NotificationEmailConfig? email,
    NotificationResendConfig? resend,
    NotificationGotifyConfig? gotify,
    NotificationNtfyConfig? ntfy,
    NotificationMattermostConfig? mattermost,
    NotificationCustomConfig? custom,
    NotificationWebhookUrlConfig? lark,
    NotificationPushoverConfig? pushover,
    NotificationWebhookUrlConfig? teams,
  }) {
    final result = ReplaceNotificationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (name != null) result.name = name;
    if (onAppDeploy != null) result.onAppDeploy = onAppDeploy;
    if (onAppBuildError != null) result.onAppBuildError = onAppBuildError;
    if (onDatabaseBackup != null) result.onDatabaseBackup = onDatabaseBackup;
    if (onVolumeBackup != null) result.onVolumeBackup = onVolumeBackup;
    if (onPanelRestart != null) result.onPanelRestart = onPanelRestart;
    if (onDockerCleanup != null) result.onDockerCleanup = onDockerCleanup;
    if (onServerThreshold != null) result.onServerThreshold = onServerThreshold;
    if (onPanelBackup != null) result.onPanelBackup = onPanelBackup;
    if (onScheduleSuccess != null) result.onScheduleSuccess = onScheduleSuccess;
    if (onScheduleFailure != null) result.onScheduleFailure = onScheduleFailure;
    if (slack != null) result.slack = slack;
    if (telegram != null) result.telegram = telegram;
    if (discord != null) result.discord = discord;
    if (email != null) result.email = email;
    if (resend != null) result.resend = resend;
    if (gotify != null) result.gotify = gotify;
    if (ntfy != null) result.ntfy = ntfy;
    if (mattermost != null) result.mattermost = mattermost;
    if (custom != null) result.custom = custom;
    if (lark != null) result.lark = lark;
    if (pushover != null) result.pushover = pushover;
    if (teams != null) result.teams = teams;
    return result;
  }

  ReplaceNotificationRequest._();

  factory ReplaceNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ReplaceNotificationRequest()..mergeFromBuffer(data, registry);
  factory ReplaceNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ReplaceNotificationRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ReplaceNotificationRequest_Provider>
      _ReplaceNotificationRequest_ProviderByTag = {
    14: ReplaceNotificationRequest_Provider.slack,
    15: ReplaceNotificationRequest_Provider.telegram,
    16: ReplaceNotificationRequest_Provider.discord,
    17: ReplaceNotificationRequest_Provider.email,
    18: ReplaceNotificationRequest_Provider.resend,
    19: ReplaceNotificationRequest_Provider.gotify,
    20: ReplaceNotificationRequest_Provider.ntfy,
    21: ReplaceNotificationRequest_Provider.mattermost,
    22: ReplaceNotificationRequest_Provider.custom,
    23: ReplaceNotificationRequest_Provider.lark,
    24: ReplaceNotificationRequest_Provider.pushover,
    25: ReplaceNotificationRequest_Provider.teams,
    0: ReplaceNotificationRequest_Provider.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ReplaceNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ReplaceNotificationRequest.$_createMessage)
    ..oo(0, [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25])
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOB(4, _omitFieldNames ? '' : 'onAppDeploy')
    ..aOB(5, _omitFieldNames ? '' : 'onAppBuildError')
    ..aOB(6, _omitFieldNames ? '' : 'onDatabaseBackup')
    ..aOB(7, _omitFieldNames ? '' : 'onVolumeBackup')
    ..aOB(8, _omitFieldNames ? '' : 'onPanelRestart')
    ..aOB(9, _omitFieldNames ? '' : 'onDockerCleanup')
    ..aOB(10, _omitFieldNames ? '' : 'onServerThreshold')
    ..aOB(11, _omitFieldNames ? '' : 'onPanelBackup')
    ..aOB(12, _omitFieldNames ? '' : 'onScheduleSuccess')
    ..aOB(13, _omitFieldNames ? '' : 'onScheduleFailure')
    ..aOM<NotificationSlackConfig>(14, _omitFieldNames ? '' : 'slack',
        subBuilder: NotificationSlackConfig.$_createMessage)
    ..aOM<NotificationTelegramConfig>(15, _omitFieldNames ? '' : 'telegram',
        subBuilder: NotificationTelegramConfig.$_createMessage)
    ..aOM<NotificationDiscordConfig>(16, _omitFieldNames ? '' : 'discord',
        subBuilder: NotificationDiscordConfig.$_createMessage)
    ..aOM<NotificationEmailConfig>(17, _omitFieldNames ? '' : 'email',
        subBuilder: NotificationEmailConfig.$_createMessage)
    ..aOM<NotificationResendConfig>(18, _omitFieldNames ? '' : 'resend',
        subBuilder: NotificationResendConfig.$_createMessage)
    ..aOM<NotificationGotifyConfig>(19, _omitFieldNames ? '' : 'gotify',
        subBuilder: NotificationGotifyConfig.$_createMessage)
    ..aOM<NotificationNtfyConfig>(20, _omitFieldNames ? '' : 'ntfy',
        subBuilder: NotificationNtfyConfig.$_createMessage)
    ..aOM<NotificationMattermostConfig>(21, _omitFieldNames ? '' : 'mattermost',
        subBuilder: NotificationMattermostConfig.$_createMessage)
    ..aOM<NotificationCustomConfig>(22, _omitFieldNames ? '' : 'custom',
        subBuilder: NotificationCustomConfig.$_createMessage)
    ..aOM<NotificationWebhookUrlConfig>(23, _omitFieldNames ? '' : 'lark',
        subBuilder: NotificationWebhookUrlConfig.$_createMessage)
    ..aOM<NotificationPushoverConfig>(24, _omitFieldNames ? '' : 'pushover',
        subBuilder: NotificationPushoverConfig.$_createMessage)
    ..aOM<NotificationWebhookUrlConfig>(25, _omitFieldNames ? '' : 'teams',
        subBuilder: NotificationWebhookUrlConfig.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplaceNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplaceNotificationRequest copyWith(
          void Function(ReplaceNotificationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ReplaceNotificationRequest))
          as ReplaceNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ReplaceNotificationRequest() / ReplaceNotificationRequest.new instead')
  static ReplaceNotificationRequest create() => ReplaceNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ReplaceNotificationRequest._();
  @$core.override
  ReplaceNotificationRequest createEmptyInstance() =>
      ReplaceNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static ReplaceNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ReplaceNotificationRequest>(
          ReplaceNotificationRequest.$_createMessage);
  static ReplaceNotificationRequest? _defaultInstance;

  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  @$pb.TagNumber(17)
  @$pb.TagNumber(18)
  @$pb.TagNumber(19)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  @$pb.TagNumber(25)
  ReplaceNotificationRequest_Provider whichProvider() =>
      _ReplaceNotificationRequest_ProviderByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  @$pb.TagNumber(17)
  @$pb.TagNumber(18)
  @$pb.TagNumber(19)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  @$pb.TagNumber(25)
  void clearProvider() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get onAppDeploy => $_getBF(3);
  @$pb.TagNumber(4)
  set onAppDeploy($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOnAppDeploy() => $_has(3);
  @$pb.TagNumber(4)
  void clearOnAppDeploy() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get onAppBuildError => $_getBF(4);
  @$pb.TagNumber(5)
  set onAppBuildError($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOnAppBuildError() => $_has(4);
  @$pb.TagNumber(5)
  void clearOnAppBuildError() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get onDatabaseBackup => $_getBF(5);
  @$pb.TagNumber(6)
  set onDatabaseBackup($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOnDatabaseBackup() => $_has(5);
  @$pb.TagNumber(6)
  void clearOnDatabaseBackup() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get onVolumeBackup => $_getBF(6);
  @$pb.TagNumber(7)
  set onVolumeBackup($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasOnVolumeBackup() => $_has(6);
  @$pb.TagNumber(7)
  void clearOnVolumeBackup() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get onPanelRestart => $_getBF(7);
  @$pb.TagNumber(8)
  set onPanelRestart($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOnPanelRestart() => $_has(7);
  @$pb.TagNumber(8)
  void clearOnPanelRestart() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get onDockerCleanup => $_getBF(8);
  @$pb.TagNumber(9)
  set onDockerCleanup($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOnDockerCleanup() => $_has(8);
  @$pb.TagNumber(9)
  void clearOnDockerCleanup() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get onServerThreshold => $_getBF(9);
  @$pb.TagNumber(10)
  set onServerThreshold($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasOnServerThreshold() => $_has(9);
  @$pb.TagNumber(10)
  void clearOnServerThreshold() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get onPanelBackup => $_getBF(10);
  @$pb.TagNumber(11)
  set onPanelBackup($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasOnPanelBackup() => $_has(10);
  @$pb.TagNumber(11)
  void clearOnPanelBackup() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get onScheduleSuccess => $_getBF(11);
  @$pb.TagNumber(12)
  set onScheduleSuccess($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOnScheduleSuccess() => $_has(11);
  @$pb.TagNumber(12)
  void clearOnScheduleSuccess() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get onScheduleFailure => $_getBF(12);
  @$pb.TagNumber(13)
  set onScheduleFailure($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasOnScheduleFailure() => $_has(12);
  @$pb.TagNumber(13)
  void clearOnScheduleFailure() => $_clearField(13);

  @$pb.TagNumber(14)
  NotificationSlackConfig get slack => $_getN(13);
  @$pb.TagNumber(14)
  set slack(NotificationSlackConfig value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasSlack() => $_has(13);
  @$pb.TagNumber(14)
  void clearSlack() => $_clearField(14);
  @$pb.TagNumber(14)
  NotificationSlackConfig ensureSlack() => $_ensure(13);

  @$pb.TagNumber(15)
  NotificationTelegramConfig get telegram => $_getN(14);
  @$pb.TagNumber(15)
  set telegram(NotificationTelegramConfig value) => $_setField(15, value);
  @$pb.TagNumber(15)
  $core.bool hasTelegram() => $_has(14);
  @$pb.TagNumber(15)
  void clearTelegram() => $_clearField(15);
  @$pb.TagNumber(15)
  NotificationTelegramConfig ensureTelegram() => $_ensure(14);

  @$pb.TagNumber(16)
  NotificationDiscordConfig get discord => $_getN(15);
  @$pb.TagNumber(16)
  set discord(NotificationDiscordConfig value) => $_setField(16, value);
  @$pb.TagNumber(16)
  $core.bool hasDiscord() => $_has(15);
  @$pb.TagNumber(16)
  void clearDiscord() => $_clearField(16);
  @$pb.TagNumber(16)
  NotificationDiscordConfig ensureDiscord() => $_ensure(15);

  @$pb.TagNumber(17)
  NotificationEmailConfig get email => $_getN(16);
  @$pb.TagNumber(17)
  set email(NotificationEmailConfig value) => $_setField(17, value);
  @$pb.TagNumber(17)
  $core.bool hasEmail() => $_has(16);
  @$pb.TagNumber(17)
  void clearEmail() => $_clearField(17);
  @$pb.TagNumber(17)
  NotificationEmailConfig ensureEmail() => $_ensure(16);

  @$pb.TagNumber(18)
  NotificationResendConfig get resend => $_getN(17);
  @$pb.TagNumber(18)
  set resend(NotificationResendConfig value) => $_setField(18, value);
  @$pb.TagNumber(18)
  $core.bool hasResend() => $_has(17);
  @$pb.TagNumber(18)
  void clearResend() => $_clearField(18);
  @$pb.TagNumber(18)
  NotificationResendConfig ensureResend() => $_ensure(17);

  @$pb.TagNumber(19)
  NotificationGotifyConfig get gotify => $_getN(18);
  @$pb.TagNumber(19)
  set gotify(NotificationGotifyConfig value) => $_setField(19, value);
  @$pb.TagNumber(19)
  $core.bool hasGotify() => $_has(18);
  @$pb.TagNumber(19)
  void clearGotify() => $_clearField(19);
  @$pb.TagNumber(19)
  NotificationGotifyConfig ensureGotify() => $_ensure(18);

  @$pb.TagNumber(20)
  NotificationNtfyConfig get ntfy => $_getN(19);
  @$pb.TagNumber(20)
  set ntfy(NotificationNtfyConfig value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasNtfy() => $_has(19);
  @$pb.TagNumber(20)
  void clearNtfy() => $_clearField(20);
  @$pb.TagNumber(20)
  NotificationNtfyConfig ensureNtfy() => $_ensure(19);

  @$pb.TagNumber(21)
  NotificationMattermostConfig get mattermost => $_getN(20);
  @$pb.TagNumber(21)
  set mattermost(NotificationMattermostConfig value) => $_setField(21, value);
  @$pb.TagNumber(21)
  $core.bool hasMattermost() => $_has(20);
  @$pb.TagNumber(21)
  void clearMattermost() => $_clearField(21);
  @$pb.TagNumber(21)
  NotificationMattermostConfig ensureMattermost() => $_ensure(20);

  @$pb.TagNumber(22)
  NotificationCustomConfig get custom => $_getN(21);
  @$pb.TagNumber(22)
  set custom(NotificationCustomConfig value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasCustom() => $_has(21);
  @$pb.TagNumber(22)
  void clearCustom() => $_clearField(22);
  @$pb.TagNumber(22)
  NotificationCustomConfig ensureCustom() => $_ensure(21);

  @$pb.TagNumber(23)
  NotificationWebhookUrlConfig get lark => $_getN(22);
  @$pb.TagNumber(23)
  set lark(NotificationWebhookUrlConfig value) => $_setField(23, value);
  @$pb.TagNumber(23)
  $core.bool hasLark() => $_has(22);
  @$pb.TagNumber(23)
  void clearLark() => $_clearField(23);
  @$pb.TagNumber(23)
  NotificationWebhookUrlConfig ensureLark() => $_ensure(22);

  @$pb.TagNumber(24)
  NotificationPushoverConfig get pushover => $_getN(23);
  @$pb.TagNumber(24)
  set pushover(NotificationPushoverConfig value) => $_setField(24, value);
  @$pb.TagNumber(24)
  $core.bool hasPushover() => $_has(23);
  @$pb.TagNumber(24)
  void clearPushover() => $_clearField(24);
  @$pb.TagNumber(24)
  NotificationPushoverConfig ensurePushover() => $_ensure(23);

  @$pb.TagNumber(25)
  NotificationWebhookUrlConfig get teams => $_getN(24);
  @$pb.TagNumber(25)
  set teams(NotificationWebhookUrlConfig value) => $_setField(25, value);
  @$pb.TagNumber(25)
  $core.bool hasTeams() => $_has(24);
  @$pb.TagNumber(25)
  void clearTeams() => $_clearField(25);
  @$pb.TagNumber(25)
  NotificationWebhookUrlConfig ensureTeams() => $_ensure(24);
}

class ReplaceNotificationResponse extends $pb.GeneratedMessage {
  factory ReplaceNotificationResponse({
    Notification? notification,
  }) {
    final result = ReplaceNotificationResponse._();
    if (notification != null) result.notification = notification;
    return result;
  }

  ReplaceNotificationResponse._();

  factory ReplaceNotificationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ReplaceNotificationResponse()..mergeFromBuffer(data, registry);
  factory ReplaceNotificationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ReplaceNotificationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ReplaceNotificationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ReplaceNotificationResponse.$_createMessage)
    ..aOM<Notification>(1, _omitFieldNames ? '' : 'notification',
        subBuilder: Notification.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplaceNotificationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplaceNotificationResponse copyWith(
          void Function(ReplaceNotificationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ReplaceNotificationResponse))
          as ReplaceNotificationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ReplaceNotificationResponse() / ReplaceNotificationResponse.new instead')
  static ReplaceNotificationResponse create() =>
      ReplaceNotificationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ReplaceNotificationResponse._();
  @$core.override
  ReplaceNotificationResponse createEmptyInstance() =>
      ReplaceNotificationResponse._();
  @$core.pragma('dart2js:noInline')
  static ReplaceNotificationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ReplaceNotificationResponse>(
          ReplaceNotificationResponse.$_createMessage);
  static ReplaceNotificationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Notification get notification => $_getN(0);
  @$pb.TagNumber(1)
  set notification(Notification value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasNotification() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotification() => $_clearField(1);
  @$pb.TagNumber(1)
  Notification ensureNotification() => $_ensure(0);
}

class DeleteNotificationRequest extends $pb.GeneratedMessage {
  factory DeleteNotificationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = DeleteNotificationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  DeleteNotificationRequest._();

  factory DeleteNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteNotificationRequest()..mergeFromBuffer(data, registry);
  factory DeleteNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteNotificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteNotificationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteNotificationRequest copyWith(
          void Function(DeleteNotificationRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteNotificationRequest))
          as DeleteNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteNotificationRequest() / DeleteNotificationRequest.new instead')
  static DeleteNotificationRequest create() => DeleteNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DeleteNotificationRequest._();
  @$core.override
  DeleteNotificationRequest createEmptyInstance() =>
      DeleteNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteNotificationRequest>(
          DeleteNotificationRequest.$_createMessage);
  static DeleteNotificationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class TestNotificationRequest extends $pb.GeneratedMessage {
  factory TestNotificationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = TestNotificationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  TestNotificationRequest._();

  factory TestNotificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestNotificationRequest()..mergeFromBuffer(data, registry);
  factory TestNotificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestNotificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestNotificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestNotificationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestNotificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestNotificationRequest copyWith(
          void Function(TestNotificationRequest) updates) =>
      super.copyWith((message) => updates(message as TestNotificationRequest))
          as TestNotificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestNotificationRequest() / TestNotificationRequest.new instead')
  static TestNotificationRequest create() => TestNotificationRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestNotificationRequest._();
  @$core.override
  TestNotificationRequest createEmptyInstance() => TestNotificationRequest._();
  @$core.pragma('dart2js:noInline')
  static TestNotificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestNotificationRequest>(
          TestNotificationRequest.$_createMessage);
  static TestNotificationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class TestNotificationResponse extends $pb.GeneratedMessage {
  factory TestNotificationResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestNotificationResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestNotificationResponse._();

  factory TestNotificationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestNotificationResponse()..mergeFromBuffer(data, registry);
  factory TestNotificationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestNotificationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestNotificationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestNotificationResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestNotificationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestNotificationResponse copyWith(
          void Function(TestNotificationResponse) updates) =>
      super.copyWith((message) => updates(message as TestNotificationResponse))
          as TestNotificationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestNotificationResponse() / TestNotificationResponse.new instead')
  static TestNotificationResponse create() => TestNotificationResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestNotificationResponse._();
  @$core.override
  TestNotificationResponse createEmptyInstance() =>
      TestNotificationResponse._();
  @$core.pragma('dart2js:noInline')
  static TestNotificationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestNotificationResponse>(
          TestNotificationResponse.$_createMessage);
  static TestNotificationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

class ListNotificationDeliveryHistoryRequest extends $pb.GeneratedMessage {
  factory ListNotificationDeliveryHistoryRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListNotificationDeliveryHistoryRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListNotificationDeliveryHistoryRequest._();

  factory ListNotificationDeliveryHistoryRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationDeliveryHistoryRequest()..mergeFromBuffer(data, registry);
  factory ListNotificationDeliveryHistoryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationDeliveryHistoryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationDeliveryHistoryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ListNotificationDeliveryHistoryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationDeliveryHistoryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationDeliveryHistoryRequest copyWith(
          void Function(ListNotificationDeliveryHistoryRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ListNotificationDeliveryHistoryRequest))
          as ListNotificationDeliveryHistoryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationDeliveryHistoryRequest() / ListNotificationDeliveryHistoryRequest.new instead')
  static ListNotificationDeliveryHistoryRequest create() =>
      ListNotificationDeliveryHistoryRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListNotificationDeliveryHistoryRequest._();
  @$core.override
  ListNotificationDeliveryHistoryRequest createEmptyInstance() =>
      ListNotificationDeliveryHistoryRequest._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationDeliveryHistoryRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ListNotificationDeliveryHistoryRequest>(
          ListNotificationDeliveryHistoryRequest.$_createMessage);
  static ListNotificationDeliveryHistoryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListNotificationDeliveryHistoryResponse extends $pb.GeneratedMessage {
  factory ListNotificationDeliveryHistoryResponse({
    $core.Iterable<NotificationDeliveryAttempt>? attempts,
  }) {
    final result = ListNotificationDeliveryHistoryResponse._();
    if (attempts != null) result.attempts.addAll(attempts);
    return result;
  }

  ListNotificationDeliveryHistoryResponse._();

  factory ListNotificationDeliveryHistoryResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationDeliveryHistoryResponse()
        ..mergeFromBuffer(data, registry);
  factory ListNotificationDeliveryHistoryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationDeliveryHistoryResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationDeliveryHistoryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          ListNotificationDeliveryHistoryResponse.$_createMessage)
    ..pPM<NotificationDeliveryAttempt>(1, _omitFieldNames ? '' : 'attempts',
        subBuilder: NotificationDeliveryAttempt.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationDeliveryHistoryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationDeliveryHistoryResponse copyWith(
          void Function(ListNotificationDeliveryHistoryResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ListNotificationDeliveryHistoryResponse))
          as ListNotificationDeliveryHistoryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationDeliveryHistoryResponse() / ListNotificationDeliveryHistoryResponse.new instead')
  static ListNotificationDeliveryHistoryResponse create() =>
      ListNotificationDeliveryHistoryResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListNotificationDeliveryHistoryResponse._();
  @$core.override
  ListNotificationDeliveryHistoryResponse createEmptyInstance() =>
      ListNotificationDeliveryHistoryResponse._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationDeliveryHistoryResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ListNotificationDeliveryHistoryResponse>(
          ListNotificationDeliveryHistoryResponse.$_createMessage);
  static ListNotificationDeliveryHistoryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<NotificationDeliveryAttempt> get attempts => $_getList(0);
}

class CreateNotificationBindingRequest extends $pb.GeneratedMessage {
  factory CreateNotificationBindingRequest({
    $fixnum.Int64? notificationId,
    NotificationResourceType? resourceType,
    $fixnum.Int64? resourceId,
    $fixnum.Int64? organizationId,
  }) {
    final result = CreateNotificationBindingRequest._();
    if (notificationId != null) result.notificationId = notificationId;
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  CreateNotificationBindingRequest._();

  factory CreateNotificationBindingRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationBindingRequest()..mergeFromBuffer(data, registry);
  factory CreateNotificationBindingRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationBindingRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateNotificationBindingRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateNotificationBindingRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'notificationId')
    ..aE<NotificationResourceType>(2, _omitFieldNames ? '' : 'resourceType',
        enumValues: NotificationResourceType.values)
    ..aInt64(3, _omitFieldNames ? '' : 'resourceId')
    ..aInt64(4, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationBindingRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationBindingRequest copyWith(
          void Function(CreateNotificationBindingRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateNotificationBindingRequest))
          as CreateNotificationBindingRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateNotificationBindingRequest() / CreateNotificationBindingRequest.new instead')
  static CreateNotificationBindingRequest create() =>
      CreateNotificationBindingRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateNotificationBindingRequest._();
  @$core.override
  CreateNotificationBindingRequest createEmptyInstance() =>
      CreateNotificationBindingRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateNotificationBindingRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateNotificationBindingRequest>(
          CreateNotificationBindingRequest.$_createMessage);
  static CreateNotificationBindingRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get notificationId => $_getI64(0);
  @$pb.TagNumber(1)
  set notificationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNotificationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotificationId() => $_clearField(1);

  @$pb.TagNumber(2)
  NotificationResourceType get resourceType => $_getN(1);
  @$pb.TagNumber(2)
  set resourceType(NotificationResourceType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasResourceType() => $_has(1);
  @$pb.TagNumber(2)
  void clearResourceType() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get resourceId => $_getI64(2);
  @$pb.TagNumber(3)
  set resourceId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasResourceId() => $_has(2);
  @$pb.TagNumber(3)
  void clearResourceId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get organizationId => $_getI64(3);
  @$pb.TagNumber(4)
  set organizationId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOrganizationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearOrganizationId() => $_clearField(4);
}

class CreateNotificationBindingResponse extends $pb.GeneratedMessage {
  factory CreateNotificationBindingResponse({
    $fixnum.Int64? id,
  }) {
    final result = CreateNotificationBindingResponse._();
    if (id != null) result.id = id;
    return result;
  }

  CreateNotificationBindingResponse._();

  factory CreateNotificationBindingResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationBindingResponse()..mergeFromBuffer(data, registry);
  factory CreateNotificationBindingResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateNotificationBindingResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateNotificationBindingResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateNotificationBindingResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationBindingResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateNotificationBindingResponse copyWith(
          void Function(CreateNotificationBindingResponse) updates) =>
      super.copyWith((message) =>
              updates(message as CreateNotificationBindingResponse))
          as CreateNotificationBindingResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateNotificationBindingResponse() / CreateNotificationBindingResponse.new instead')
  static CreateNotificationBindingResponse create() =>
      CreateNotificationBindingResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateNotificationBindingResponse._();
  @$core.override
  CreateNotificationBindingResponse createEmptyInstance() =>
      CreateNotificationBindingResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateNotificationBindingResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateNotificationBindingResponse>(
          CreateNotificationBindingResponse.$_createMessage);
  static CreateNotificationBindingResponse? _defaultInstance;

  /// The create path only returns the new binding id; the row is re-read via ListBindings.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListNotificationBindingsRequest extends $pb.GeneratedMessage {
  factory ListNotificationBindingsRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListNotificationBindingsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListNotificationBindingsRequest._();

  factory ListNotificationBindingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationBindingsRequest()..mergeFromBuffer(data, registry);
  factory ListNotificationBindingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationBindingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationBindingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListNotificationBindingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationBindingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationBindingsRequest copyWith(
          void Function(ListNotificationBindingsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListNotificationBindingsRequest))
          as ListNotificationBindingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationBindingsRequest() / ListNotificationBindingsRequest.new instead')
  static ListNotificationBindingsRequest create() =>
      ListNotificationBindingsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListNotificationBindingsRequest._();
  @$core.override
  ListNotificationBindingsRequest createEmptyInstance() =>
      ListNotificationBindingsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationBindingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationBindingsRequest>(
          ListNotificationBindingsRequest.$_createMessage);
  static ListNotificationBindingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListNotificationBindingsResponse extends $pb.GeneratedMessage {
  factory ListNotificationBindingsResponse({
    $core.Iterable<NotificationBinding>? bindings,
  }) {
    final result = ListNotificationBindingsResponse._();
    if (bindings != null) result.bindings.addAll(bindings);
    return result;
  }

  ListNotificationBindingsResponse._();

  factory ListNotificationBindingsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationBindingsResponse()..mergeFromBuffer(data, registry);
  factory ListNotificationBindingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListNotificationBindingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationBindingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListNotificationBindingsResponse.$_createMessage)
    ..pPM<NotificationBinding>(1, _omitFieldNames ? '' : 'bindings',
        subBuilder: NotificationBinding.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationBindingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationBindingsResponse copyWith(
          void Function(ListNotificationBindingsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListNotificationBindingsResponse))
          as ListNotificationBindingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListNotificationBindingsResponse() / ListNotificationBindingsResponse.new instead')
  static ListNotificationBindingsResponse create() =>
      ListNotificationBindingsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListNotificationBindingsResponse._();
  @$core.override
  ListNotificationBindingsResponse createEmptyInstance() =>
      ListNotificationBindingsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListNotificationBindingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationBindingsResponse>(
          ListNotificationBindingsResponse.$_createMessage);
  static ListNotificationBindingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<NotificationBinding> get bindings => $_getList(0);
}

class DeleteNotificationBindingRequest extends $pb.GeneratedMessage {
  factory DeleteNotificationBindingRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = DeleteNotificationBindingRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  DeleteNotificationBindingRequest._();

  factory DeleteNotificationBindingRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteNotificationBindingRequest()..mergeFromBuffer(data, registry);
  factory DeleteNotificationBindingRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteNotificationBindingRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteNotificationBindingRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteNotificationBindingRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteNotificationBindingRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteNotificationBindingRequest copyWith(
          void Function(DeleteNotificationBindingRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeleteNotificationBindingRequest))
          as DeleteNotificationBindingRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteNotificationBindingRequest() / DeleteNotificationBindingRequest.new instead')
  static DeleteNotificationBindingRequest create() =>
      DeleteNotificationBindingRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DeleteNotificationBindingRequest._();
  @$core.override
  DeleteNotificationBindingRequest createEmptyInstance() =>
      DeleteNotificationBindingRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteNotificationBindingRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteNotificationBindingRequest>(
          DeleteNotificationBindingRequest.$_createMessage);
  static DeleteNotificationBindingRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

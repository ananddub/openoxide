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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'alert.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'alert.pbenum.dart';

class AlertEvent extends $pb.GeneratedMessage {
  factory AlertEvent({
    $fixnum.Int64? id,
    $fixnum.Int64? alertRuleId,
    $fixnum.Int64? organizationId,
    $core.String? targetKey,
    AlertEventState? state,
    $core.double? value,
    $core.double? threshold,
    $core.String? message,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? acknowledgedAt,
    $fixnum.Int64? acknowledgedBy,
    $fixnum.Int64? silencedUntil,
    $fixnum.Int64? resolvedAt,
    $core.String? notificationCorrelationId,
  }) {
    final result = AlertEvent._();
    if (id != null) result.id = id;
    if (alertRuleId != null) result.alertRuleId = alertRuleId;
    if (organizationId != null) result.organizationId = organizationId;
    if (targetKey != null) result.targetKey = targetKey;
    if (state != null) result.state = state;
    if (value != null) result.value = value;
    if (threshold != null) result.threshold = threshold;
    if (message != null) result.message = message;
    if (createdAt != null) result.createdAt = createdAt;
    if (acknowledgedAt != null) result.acknowledgedAt = acknowledgedAt;
    if (acknowledgedBy != null) result.acknowledgedBy = acknowledgedBy;
    if (silencedUntil != null) result.silencedUntil = silencedUntil;
    if (resolvedAt != null) result.resolvedAt = resolvedAt;
    if (notificationCorrelationId != null)
      result.notificationCorrelationId = notificationCorrelationId;
    return result;
  }

  AlertEvent._();

  factory AlertEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AlertEvent()..mergeFromBuffer(data, registry);
  factory AlertEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AlertEvent()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AlertEvent',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AlertEvent.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'alertRuleId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aOS(4, _omitFieldNames ? '' : 'targetKey')
    ..aE<AlertEventState>(5, _omitFieldNames ? '' : 'state',
        enumValues: AlertEventState.values)
    ..aD(6, _omitFieldNames ? '' : 'value')
    ..aD(7, _omitFieldNames ? '' : 'threshold')
    ..aOS(8, _omitFieldNames ? '' : 'message')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'acknowledgedAt')
    ..aInt64(11, _omitFieldNames ? '' : 'acknowledgedBy')
    ..aInt64(12, _omitFieldNames ? '' : 'silencedUntil')
    ..aInt64(13, _omitFieldNames ? '' : 'resolvedAt')
    ..aOS(14, _omitFieldNames ? '' : 'notificationCorrelationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AlertEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AlertEvent copyWith(void Function(AlertEvent) updates) =>
      super.copyWith((message) => updates(message as AlertEvent)) as AlertEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AlertEvent() / AlertEvent.new instead')
  static AlertEvent create() => AlertEvent._();
  static $pb.GeneratedMessage $_createMessage() => AlertEvent._();
  @$core.override
  AlertEvent createEmptyInstance() => AlertEvent._();
  @$core.pragma('dart2js:noInline')
  static AlertEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AlertEvent>(AlertEvent.$_createMessage);
  static AlertEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get alertRuleId => $_getI64(1);
  @$pb.TagNumber(2)
  set alertRuleId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAlertRuleId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAlertRuleId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  /// Stable target identity key recorded when the rule fired.
  @$pb.TagNumber(4)
  $core.String get targetKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set targetKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTargetKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearTargetKey() => $_clearField(4);

  @$pb.TagNumber(5)
  AlertEventState get state => $_getN(4);
  @$pb.TagNumber(5)
  set state(AlertEventState value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasState() => $_has(4);
  @$pb.TagNumber(5)
  void clearState() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.double get value => $_getN(5);
  @$pb.TagNumber(6)
  set value($core.double value) => $_setDouble(5, value);
  @$pb.TagNumber(6)
  $core.bool hasValue() => $_has(5);
  @$pb.TagNumber(6)
  void clearValue() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get threshold => $_getN(6);
  @$pb.TagNumber(7)
  set threshold($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasThreshold() => $_has(6);
  @$pb.TagNumber(7)
  void clearThreshold() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get message => $_getSZ(7);
  @$pb.TagNumber(8)
  set message($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasMessage() => $_has(7);
  @$pb.TagNumber(8)
  void clearMessage() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get createdAt => $_getI64(8);
  @$pb.TagNumber(9)
  set createdAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCreatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearCreatedAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get acknowledgedAt => $_getI64(9);
  @$pb.TagNumber(10)
  set acknowledgedAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasAcknowledgedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearAcknowledgedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get acknowledgedBy => $_getI64(10);
  @$pb.TagNumber(11)
  set acknowledgedBy($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasAcknowledgedBy() => $_has(10);
  @$pb.TagNumber(11)
  void clearAcknowledgedBy() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get silencedUntil => $_getI64(11);
  @$pb.TagNumber(12)
  set silencedUntil($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasSilencedUntil() => $_has(11);
  @$pb.TagNumber(12)
  void clearSilencedUntil() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get resolvedAt => $_getI64(12);
  @$pb.TagNumber(13)
  set resolvedAt($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasResolvedAt() => $_has(12);
  @$pb.TagNumber(13)
  void clearResolvedAt() => $_clearField(13);

  /// Correlation id linking the event to notification delivery attempts.
  @$pb.TagNumber(14)
  $core.String get notificationCorrelationId => $_getSZ(13);
  @$pb.TagNumber(14)
  set notificationCorrelationId($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasNotificationCorrelationId() => $_has(13);
  @$pb.TagNumber(14)
  void clearNotificationCorrelationId() => $_clearField(14);
}

class ListAlertEventsRequest extends $pb.GeneratedMessage {
  factory ListAlertEventsRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListAlertEventsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListAlertEventsRequest._();

  factory ListAlertEventsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertEventsRequest()..mergeFromBuffer(data, registry);
  factory ListAlertEventsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertEventsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListAlertEventsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListAlertEventsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertEventsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertEventsRequest copyWith(
          void Function(ListAlertEventsRequest) updates) =>
      super.copyWith((message) => updates(message as ListAlertEventsRequest))
          as ListAlertEventsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListAlertEventsRequest() / ListAlertEventsRequest.new instead')
  static ListAlertEventsRequest create() => ListAlertEventsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListAlertEventsRequest._();
  @$core.override
  ListAlertEventsRequest createEmptyInstance() => ListAlertEventsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListAlertEventsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListAlertEventsRequest>(
          ListAlertEventsRequest.$_createMessage);
  static ListAlertEventsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListAlertEventsResponse extends $pb.GeneratedMessage {
  factory ListAlertEventsResponse({
    $core.Iterable<AlertEvent>? events,
  }) {
    final result = ListAlertEventsResponse._();
    if (events != null) result.events.addAll(events);
    return result;
  }

  ListAlertEventsResponse._();

  factory ListAlertEventsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertEventsResponse()..mergeFromBuffer(data, registry);
  factory ListAlertEventsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertEventsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListAlertEventsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListAlertEventsResponse.$_createMessage)
    ..pPM<AlertEvent>(1, _omitFieldNames ? '' : 'events',
        subBuilder: AlertEvent.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertEventsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertEventsResponse copyWith(
          void Function(ListAlertEventsResponse) updates) =>
      super.copyWith((message) => updates(message as ListAlertEventsResponse))
          as ListAlertEventsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListAlertEventsResponse() / ListAlertEventsResponse.new instead')
  static ListAlertEventsResponse create() => ListAlertEventsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListAlertEventsResponse._();
  @$core.override
  ListAlertEventsResponse createEmptyInstance() => ListAlertEventsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListAlertEventsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListAlertEventsResponse>(
          ListAlertEventsResponse.$_createMessage);
  static ListAlertEventsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AlertEvent> get events => $_getList(0);
}

class AcknowledgeAlertEventRequest extends $pb.GeneratedMessage {
  factory AcknowledgeAlertEventRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = AcknowledgeAlertEventRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AcknowledgeAlertEventRequest._();

  factory AcknowledgeAlertEventRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AcknowledgeAlertEventRequest()..mergeFromBuffer(data, registry);
  factory AcknowledgeAlertEventRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AcknowledgeAlertEventRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AcknowledgeAlertEventRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AcknowledgeAlertEventRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AcknowledgeAlertEventRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AcknowledgeAlertEventRequest copyWith(
          void Function(AcknowledgeAlertEventRequest) updates) =>
      super.copyWith(
              (message) => updates(message as AcknowledgeAlertEventRequest))
          as AcknowledgeAlertEventRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AcknowledgeAlertEventRequest() / AcknowledgeAlertEventRequest.new instead')
  static AcknowledgeAlertEventRequest create() =>
      AcknowledgeAlertEventRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AcknowledgeAlertEventRequest._();
  @$core.override
  AcknowledgeAlertEventRequest createEmptyInstance() =>
      AcknowledgeAlertEventRequest._();
  @$core.pragma('dart2js:noInline')
  static AcknowledgeAlertEventRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AcknowledgeAlertEventRequest>(
          AcknowledgeAlertEventRequest.$_createMessage);
  static AcknowledgeAlertEventRequest? _defaultInstance;

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

class SilenceAlertEventRequest extends $pb.GeneratedMessage {
  factory SilenceAlertEventRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? until,
  }) {
    final result = SilenceAlertEventRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (until != null) result.until = until;
    return result;
  }

  SilenceAlertEventRequest._();

  factory SilenceAlertEventRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SilenceAlertEventRequest()..mergeFromBuffer(data, registry);
  factory SilenceAlertEventRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SilenceAlertEventRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SilenceAlertEventRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SilenceAlertEventRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(3, _omitFieldNames ? '' : 'until')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SilenceAlertEventRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SilenceAlertEventRequest copyWith(
          void Function(SilenceAlertEventRequest) updates) =>
      super.copyWith((message) => updates(message as SilenceAlertEventRequest))
          as SilenceAlertEventRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SilenceAlertEventRequest() / SilenceAlertEventRequest.new instead')
  static SilenceAlertEventRequest create() => SilenceAlertEventRequest._();
  static $pb.GeneratedMessage $_createMessage() => SilenceAlertEventRequest._();
  @$core.override
  SilenceAlertEventRequest createEmptyInstance() =>
      SilenceAlertEventRequest._();
  @$core.pragma('dart2js:noInline')
  static SilenceAlertEventRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SilenceAlertEventRequest>(
          SilenceAlertEventRequest.$_createMessage);
  static SilenceAlertEventRequest? _defaultInstance;

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

  /// Unix seconds until which further notifications for the event are muted.
  @$pb.TagNumber(3)
  $fixnum.Int64 get until => $_getI64(2);
  @$pb.TagNumber(3)
  set until($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUntil() => $_has(2);
  @$pb.TagNumber(3)
  void clearUntil() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

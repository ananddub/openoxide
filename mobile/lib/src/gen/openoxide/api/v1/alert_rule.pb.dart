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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'alert_rule.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'alert_rule.pbenum.dart';

class AlertRule extends $pb.GeneratedMessage {
  factory AlertRule({
    $fixnum.Int64? id,
    $core.String? name,
    AlertTargetKind? targetKind,
    $fixnum.Int64? targetId,
    AlertMetricKind? metric,
    AlertOperator? operator,
    $core.double? threshold,
    $fixnum.Int64? durationSeconds,
    $core.bool? enabled,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = AlertRule._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (targetKind != null) result.targetKind = targetKind;
    if (targetId != null) result.targetId = targetId;
    if (metric != null) result.metric = metric;
    if (operator != null) result.operator = operator;
    if (threshold != null) result.threshold = threshold;
    if (durationSeconds != null) result.durationSeconds = durationSeconds;
    if (enabled != null) result.enabled = enabled;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  AlertRule._();

  factory AlertRule.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AlertRule()..mergeFromBuffer(data, registry);
  factory AlertRule.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AlertRule()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AlertRule',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AlertRule.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<AlertTargetKind>(3, _omitFieldNames ? '' : 'targetKind',
        enumValues: AlertTargetKind.values)
    ..aInt64(4, _omitFieldNames ? '' : 'targetId')
    ..aE<AlertMetricKind>(5, _omitFieldNames ? '' : 'metric',
        enumValues: AlertMetricKind.values)
    ..aE<AlertOperator>(6, _omitFieldNames ? '' : 'operator',
        enumValues: AlertOperator.values)
    ..aD(7, _omitFieldNames ? '' : 'threshold')
    ..aInt64(8, _omitFieldNames ? '' : 'durationSeconds')
    ..aOB(9, _omitFieldNames ? '' : 'enabled')
    ..aInt64(10, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(11, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(12, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AlertRule clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AlertRule copyWith(void Function(AlertRule) updates) =>
      super.copyWith((message) => updates(message as AlertRule)) as AlertRule;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AlertRule() / AlertRule.new instead')
  static AlertRule create() => AlertRule._();
  static $pb.GeneratedMessage $_createMessage() => AlertRule._();
  @$core.override
  AlertRule createEmptyInstance() => AlertRule._();
  @$core.pragma('dart2js:noInline')
  static AlertRule getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AlertRule>(AlertRule.$_createMessage);
  static AlertRule? _defaultInstance;

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
  AlertTargetKind get targetKind => $_getN(2);
  @$pb.TagNumber(3)
  set targetKind(AlertTargetKind value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasTargetKind() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetKind() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get targetId => $_getI64(3);
  @$pb.TagNumber(4)
  set targetId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTargetId() => $_has(3);
  @$pb.TagNumber(4)
  void clearTargetId() => $_clearField(4);

  @$pb.TagNumber(5)
  AlertMetricKind get metric => $_getN(4);
  @$pb.TagNumber(5)
  set metric(AlertMetricKind value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasMetric() => $_has(4);
  @$pb.TagNumber(5)
  void clearMetric() => $_clearField(5);

  @$pb.TagNumber(6)
  AlertOperator get operator => $_getN(5);
  @$pb.TagNumber(6)
  set operator(AlertOperator value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasOperator() => $_has(5);
  @$pb.TagNumber(6)
  void clearOperator() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get threshold => $_getN(6);
  @$pb.TagNumber(7)
  set threshold($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasThreshold() => $_has(6);
  @$pb.TagNumber(7)
  void clearThreshold() => $_clearField(7);

  /// Seconds a breach must persist; also drives the post-fire cooldown.
  @$pb.TagNumber(8)
  $fixnum.Int64 get durationSeconds => $_getI64(7);
  @$pb.TagNumber(8)
  set durationSeconds($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDurationSeconds() => $_has(7);
  @$pb.TagNumber(8)
  void clearDurationSeconds() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get enabled => $_getBF(8);
  @$pb.TagNumber(9)
  set enabled($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEnabled() => $_has(8);
  @$pb.TagNumber(9)
  void clearEnabled() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get organizationId => $_getI64(9);
  @$pb.TagNumber(10)
  set organizationId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasOrganizationId() => $_has(9);
  @$pb.TagNumber(10)
  void clearOrganizationId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get createdAt => $_getI64(10);
  @$pb.TagNumber(11)
  set createdAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCreatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCreatedAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get updatedAt => $_getI64(11);
  @$pb.TagNumber(12)
  set updatedAt($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasUpdatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearUpdatedAt() => $_clearField(12);
}

class ListAlertRulesRequest extends $pb.GeneratedMessage {
  factory ListAlertRulesRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListAlertRulesRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListAlertRulesRequest._();

  factory ListAlertRulesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertRulesRequest()..mergeFromBuffer(data, registry);
  factory ListAlertRulesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertRulesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListAlertRulesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListAlertRulesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertRulesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertRulesRequest copyWith(
          void Function(ListAlertRulesRequest) updates) =>
      super.copyWith((message) => updates(message as ListAlertRulesRequest))
          as ListAlertRulesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListAlertRulesRequest() / ListAlertRulesRequest.new instead')
  static ListAlertRulesRequest create() => ListAlertRulesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListAlertRulesRequest._();
  @$core.override
  ListAlertRulesRequest createEmptyInstance() => ListAlertRulesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListAlertRulesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListAlertRulesRequest>(
          ListAlertRulesRequest.$_createMessage);
  static ListAlertRulesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListAlertRulesResponse extends $pb.GeneratedMessage {
  factory ListAlertRulesResponse({
    $core.Iterable<AlertRule>? rules,
  }) {
    final result = ListAlertRulesResponse._();
    if (rules != null) result.rules.addAll(rules);
    return result;
  }

  ListAlertRulesResponse._();

  factory ListAlertRulesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertRulesResponse()..mergeFromBuffer(data, registry);
  factory ListAlertRulesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListAlertRulesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListAlertRulesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListAlertRulesResponse.$_createMessage)
    ..pPM<AlertRule>(1, _omitFieldNames ? '' : 'rules',
        subBuilder: AlertRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertRulesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListAlertRulesResponse copyWith(
          void Function(ListAlertRulesResponse) updates) =>
      super.copyWith((message) => updates(message as ListAlertRulesResponse))
          as ListAlertRulesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListAlertRulesResponse() / ListAlertRulesResponse.new instead')
  static ListAlertRulesResponse create() => ListAlertRulesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListAlertRulesResponse._();
  @$core.override
  ListAlertRulesResponse createEmptyInstance() => ListAlertRulesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListAlertRulesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListAlertRulesResponse>(
          ListAlertRulesResponse.$_createMessage);
  static ListAlertRulesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AlertRule> get rules => $_getList(0);
}

class GetAlertRuleRequest extends $pb.GeneratedMessage {
  factory GetAlertRuleRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = GetAlertRuleRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  GetAlertRuleRequest._();

  factory GetAlertRuleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetAlertRuleRequest()..mergeFromBuffer(data, registry);
  factory GetAlertRuleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetAlertRuleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetAlertRuleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetAlertRuleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetAlertRuleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetAlertRuleRequest copyWith(void Function(GetAlertRuleRequest) updates) =>
      super.copyWith((message) => updates(message as GetAlertRuleRequest))
          as GetAlertRuleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use GetAlertRuleRequest() / GetAlertRuleRequest.new instead')
  static GetAlertRuleRequest create() => GetAlertRuleRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetAlertRuleRequest._();
  @$core.override
  GetAlertRuleRequest createEmptyInstance() => GetAlertRuleRequest._();
  @$core.pragma('dart2js:noInline')
  static GetAlertRuleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetAlertRuleRequest>(
          GetAlertRuleRequest.$_createMessage);
  static GetAlertRuleRequest? _defaultInstance;

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

class GetAlertRuleResponse extends $pb.GeneratedMessage {
  factory GetAlertRuleResponse({
    AlertRule? rule,
  }) {
    final result = GetAlertRuleResponse._();
    if (rule != null) result.rule = rule;
    return result;
  }

  GetAlertRuleResponse._();

  factory GetAlertRuleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetAlertRuleResponse()..mergeFromBuffer(data, registry);
  factory GetAlertRuleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetAlertRuleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetAlertRuleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetAlertRuleResponse.$_createMessage)
    ..aOM<AlertRule>(1, _omitFieldNames ? '' : 'rule',
        subBuilder: AlertRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetAlertRuleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetAlertRuleResponse copyWith(void Function(GetAlertRuleResponse) updates) =>
      super.copyWith((message) => updates(message as GetAlertRuleResponse))
          as GetAlertRuleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetAlertRuleResponse() / GetAlertRuleResponse.new instead')
  static GetAlertRuleResponse create() => GetAlertRuleResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetAlertRuleResponse._();
  @$core.override
  GetAlertRuleResponse createEmptyInstance() => GetAlertRuleResponse._();
  @$core.pragma('dart2js:noInline')
  static GetAlertRuleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetAlertRuleResponse>(
          GetAlertRuleResponse.$_createMessage);
  static GetAlertRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AlertRule get rule => $_getN(0);
  @$pb.TagNumber(1)
  set rule(AlertRule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRule() => $_has(0);
  @$pb.TagNumber(1)
  void clearRule() => $_clearField(1);
  @$pb.TagNumber(1)
  AlertRule ensureRule() => $_ensure(0);
}

class CreateAlertRuleRequest extends $pb.GeneratedMessage {
  factory CreateAlertRuleRequest({
    $core.String? name,
    AlertTargetKind? targetKind,
    $fixnum.Int64? targetId,
    AlertMetricKind? metric,
    AlertOperator? operator,
    $core.double? threshold,
    $fixnum.Int64? durationSeconds,
    $core.bool? enabled,
    $fixnum.Int64? organizationId,
  }) {
    final result = CreateAlertRuleRequest._();
    if (name != null) result.name = name;
    if (targetKind != null) result.targetKind = targetKind;
    if (targetId != null) result.targetId = targetId;
    if (metric != null) result.metric = metric;
    if (operator != null) result.operator = operator;
    if (threshold != null) result.threshold = threshold;
    if (durationSeconds != null) result.durationSeconds = durationSeconds;
    if (enabled != null) result.enabled = enabled;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  CreateAlertRuleRequest._();

  factory CreateAlertRuleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateAlertRuleRequest()..mergeFromBuffer(data, registry);
  factory CreateAlertRuleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateAlertRuleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateAlertRuleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateAlertRuleRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aE<AlertTargetKind>(2, _omitFieldNames ? '' : 'targetKind',
        enumValues: AlertTargetKind.values)
    ..aInt64(3, _omitFieldNames ? '' : 'targetId')
    ..aE<AlertMetricKind>(4, _omitFieldNames ? '' : 'metric',
        enumValues: AlertMetricKind.values)
    ..aE<AlertOperator>(5, _omitFieldNames ? '' : 'operator',
        enumValues: AlertOperator.values)
    ..aD(6, _omitFieldNames ? '' : 'threshold')
    ..aInt64(7, _omitFieldNames ? '' : 'durationSeconds')
    ..aOB(8, _omitFieldNames ? '' : 'enabled')
    ..aInt64(9, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateAlertRuleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateAlertRuleRequest copyWith(
          void Function(CreateAlertRuleRequest) updates) =>
      super.copyWith((message) => updates(message as CreateAlertRuleRequest))
          as CreateAlertRuleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateAlertRuleRequest() / CreateAlertRuleRequest.new instead')
  static CreateAlertRuleRequest create() => CreateAlertRuleRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateAlertRuleRequest._();
  @$core.override
  CreateAlertRuleRequest createEmptyInstance() => CreateAlertRuleRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateAlertRuleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateAlertRuleRequest>(
          CreateAlertRuleRequest.$_createMessage);
  static CreateAlertRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  AlertTargetKind get targetKind => $_getN(1);
  @$pb.TagNumber(2)
  set targetKind(AlertTargetKind value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetKind() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetKind() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get targetId => $_getI64(2);
  @$pb.TagNumber(3)
  set targetId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTargetId() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetId() => $_clearField(3);

  @$pb.TagNumber(4)
  AlertMetricKind get metric => $_getN(3);
  @$pb.TagNumber(4)
  set metric(AlertMetricKind value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMetric() => $_has(3);
  @$pb.TagNumber(4)
  void clearMetric() => $_clearField(4);

  @$pb.TagNumber(5)
  AlertOperator get operator => $_getN(4);
  @$pb.TagNumber(5)
  set operator(AlertOperator value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasOperator() => $_has(4);
  @$pb.TagNumber(5)
  void clearOperator() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.double get threshold => $_getN(5);
  @$pb.TagNumber(6)
  set threshold($core.double value) => $_setDouble(5, value);
  @$pb.TagNumber(6)
  $core.bool hasThreshold() => $_has(5);
  @$pb.TagNumber(6)
  void clearThreshold() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get durationSeconds => $_getI64(6);
  @$pb.TagNumber(7)
  set durationSeconds($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDurationSeconds() => $_has(6);
  @$pb.TagNumber(7)
  void clearDurationSeconds() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get enabled => $_getBF(7);
  @$pb.TagNumber(8)
  set enabled($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEnabled() => $_has(7);
  @$pb.TagNumber(8)
  void clearEnabled() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get organizationId => $_getI64(8);
  @$pb.TagNumber(9)
  set organizationId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOrganizationId() => $_has(8);
  @$pb.TagNumber(9)
  void clearOrganizationId() => $_clearField(9);
}

class CreateAlertRuleResponse extends $pb.GeneratedMessage {
  factory CreateAlertRuleResponse({
    AlertRule? rule,
  }) {
    final result = CreateAlertRuleResponse._();
    if (rule != null) result.rule = rule;
    return result;
  }

  CreateAlertRuleResponse._();

  factory CreateAlertRuleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateAlertRuleResponse()..mergeFromBuffer(data, registry);
  factory CreateAlertRuleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateAlertRuleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateAlertRuleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateAlertRuleResponse.$_createMessage)
    ..aOM<AlertRule>(1, _omitFieldNames ? '' : 'rule',
        subBuilder: AlertRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateAlertRuleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateAlertRuleResponse copyWith(
          void Function(CreateAlertRuleResponse) updates) =>
      super.copyWith((message) => updates(message as CreateAlertRuleResponse))
          as CreateAlertRuleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateAlertRuleResponse() / CreateAlertRuleResponse.new instead')
  static CreateAlertRuleResponse create() => CreateAlertRuleResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateAlertRuleResponse._();
  @$core.override
  CreateAlertRuleResponse createEmptyInstance() => CreateAlertRuleResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateAlertRuleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateAlertRuleResponse>(
          CreateAlertRuleResponse.$_createMessage);
  static CreateAlertRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AlertRule get rule => $_getN(0);
  @$pb.TagNumber(1)
  set rule(AlertRule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRule() => $_has(0);
  @$pb.TagNumber(1)
  void clearRule() => $_clearField(1);
  @$pb.TagNumber(1)
  AlertRule ensureRule() => $_ensure(0);
}

/// Full replacement of the rule definition; organization is fixed to the stored row and cannot be moved between tenants.
class UpdateAlertRuleRequest extends $pb.GeneratedMessage {
  factory UpdateAlertRuleRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $core.String? name,
    AlertTargetKind? targetKind,
    $fixnum.Int64? targetId,
    AlertMetricKind? metric,
    AlertOperator? operator,
    $core.double? threshold,
    $fixnum.Int64? durationSeconds,
    $core.bool? enabled,
  }) {
    final result = UpdateAlertRuleRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (name != null) result.name = name;
    if (targetKind != null) result.targetKind = targetKind;
    if (targetId != null) result.targetId = targetId;
    if (metric != null) result.metric = metric;
    if (operator != null) result.operator = operator;
    if (threshold != null) result.threshold = threshold;
    if (durationSeconds != null) result.durationSeconds = durationSeconds;
    if (enabled != null) result.enabled = enabled;
    return result;
  }

  UpdateAlertRuleRequest._();

  factory UpdateAlertRuleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAlertRuleRequest()..mergeFromBuffer(data, registry);
  factory UpdateAlertRuleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAlertRuleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateAlertRuleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateAlertRuleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aE<AlertTargetKind>(4, _omitFieldNames ? '' : 'targetKind',
        enumValues: AlertTargetKind.values)
    ..aInt64(5, _omitFieldNames ? '' : 'targetId')
    ..aE<AlertMetricKind>(6, _omitFieldNames ? '' : 'metric',
        enumValues: AlertMetricKind.values)
    ..aE<AlertOperator>(7, _omitFieldNames ? '' : 'operator',
        enumValues: AlertOperator.values)
    ..aD(8, _omitFieldNames ? '' : 'threshold')
    ..aInt64(9, _omitFieldNames ? '' : 'durationSeconds')
    ..aOB(10, _omitFieldNames ? '' : 'enabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAlertRuleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAlertRuleRequest copyWith(
          void Function(UpdateAlertRuleRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateAlertRuleRequest))
          as UpdateAlertRuleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateAlertRuleRequest() / UpdateAlertRuleRequest.new instead')
  static UpdateAlertRuleRequest create() => UpdateAlertRuleRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateAlertRuleRequest._();
  @$core.override
  UpdateAlertRuleRequest createEmptyInstance() => UpdateAlertRuleRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateAlertRuleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateAlertRuleRequest>(
          UpdateAlertRuleRequest.$_createMessage);
  static UpdateAlertRuleRequest? _defaultInstance;

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
  AlertTargetKind get targetKind => $_getN(3);
  @$pb.TagNumber(4)
  set targetKind(AlertTargetKind value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasTargetKind() => $_has(3);
  @$pb.TagNumber(4)
  void clearTargetKind() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get targetId => $_getI64(4);
  @$pb.TagNumber(5)
  set targetId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTargetId() => $_has(4);
  @$pb.TagNumber(5)
  void clearTargetId() => $_clearField(5);

  @$pb.TagNumber(6)
  AlertMetricKind get metric => $_getN(5);
  @$pb.TagNumber(6)
  set metric(AlertMetricKind value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasMetric() => $_has(5);
  @$pb.TagNumber(6)
  void clearMetric() => $_clearField(6);

  @$pb.TagNumber(7)
  AlertOperator get operator => $_getN(6);
  @$pb.TagNumber(7)
  set operator(AlertOperator value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasOperator() => $_has(6);
  @$pb.TagNumber(7)
  void clearOperator() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.double get threshold => $_getN(7);
  @$pb.TagNumber(8)
  set threshold($core.double value) => $_setDouble(7, value);
  @$pb.TagNumber(8)
  $core.bool hasThreshold() => $_has(7);
  @$pb.TagNumber(8)
  void clearThreshold() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get durationSeconds => $_getI64(8);
  @$pb.TagNumber(9)
  set durationSeconds($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasDurationSeconds() => $_has(8);
  @$pb.TagNumber(9)
  void clearDurationSeconds() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get enabled => $_getBF(9);
  @$pb.TagNumber(10)
  set enabled($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasEnabled() => $_has(9);
  @$pb.TagNumber(10)
  void clearEnabled() => $_clearField(10);
}

class UpdateAlertRuleResponse extends $pb.GeneratedMessage {
  factory UpdateAlertRuleResponse({
    AlertRule? rule,
  }) {
    final result = UpdateAlertRuleResponse._();
    if (rule != null) result.rule = rule;
    return result;
  }

  UpdateAlertRuleResponse._();

  factory UpdateAlertRuleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAlertRuleResponse()..mergeFromBuffer(data, registry);
  factory UpdateAlertRuleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateAlertRuleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateAlertRuleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateAlertRuleResponse.$_createMessage)
    ..aOM<AlertRule>(1, _omitFieldNames ? '' : 'rule',
        subBuilder: AlertRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAlertRuleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateAlertRuleResponse copyWith(
          void Function(UpdateAlertRuleResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateAlertRuleResponse))
          as UpdateAlertRuleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateAlertRuleResponse() / UpdateAlertRuleResponse.new instead')
  static UpdateAlertRuleResponse create() => UpdateAlertRuleResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateAlertRuleResponse._();
  @$core.override
  UpdateAlertRuleResponse createEmptyInstance() => UpdateAlertRuleResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateAlertRuleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateAlertRuleResponse>(
          UpdateAlertRuleResponse.$_createMessage);
  static UpdateAlertRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AlertRule get rule => $_getN(0);
  @$pb.TagNumber(1)
  set rule(AlertRule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRule() => $_has(0);
  @$pb.TagNumber(1)
  void clearRule() => $_clearField(1);
  @$pb.TagNumber(1)
  AlertRule ensureRule() => $_ensure(0);
}

class DeleteAlertRuleRequest extends $pb.GeneratedMessage {
  factory DeleteAlertRuleRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = DeleteAlertRuleRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  DeleteAlertRuleRequest._();

  factory DeleteAlertRuleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteAlertRuleRequest()..mergeFromBuffer(data, registry);
  factory DeleteAlertRuleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteAlertRuleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteAlertRuleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteAlertRuleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteAlertRuleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteAlertRuleRequest copyWith(
          void Function(DeleteAlertRuleRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteAlertRuleRequest))
          as DeleteAlertRuleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteAlertRuleRequest() / DeleteAlertRuleRequest.new instead')
  static DeleteAlertRuleRequest create() => DeleteAlertRuleRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteAlertRuleRequest._();
  @$core.override
  DeleteAlertRuleRequest createEmptyInstance() => DeleteAlertRuleRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteAlertRuleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteAlertRuleRequest>(
          DeleteAlertRuleRequest.$_createMessage);
  static DeleteAlertRuleRequest? _defaultInstance;

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

/// Fires a canned test message through an existing notification channel.
class TestAlertRuleRequest extends $pb.GeneratedMessage {
  factory TestAlertRuleRequest({
    $fixnum.Int64? notificationId,
  }) {
    final result = TestAlertRuleRequest._();
    if (notificationId != null) result.notificationId = notificationId;
    return result;
  }

  TestAlertRuleRequest._();

  factory TestAlertRuleRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestAlertRuleRequest()..mergeFromBuffer(data, registry);
  factory TestAlertRuleRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestAlertRuleRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestAlertRuleRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestAlertRuleRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'notificationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestAlertRuleRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestAlertRuleRequest copyWith(void Function(TestAlertRuleRequest) updates) =>
      super.copyWith((message) => updates(message as TestAlertRuleRequest))
          as TestAlertRuleRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestAlertRuleRequest() / TestAlertRuleRequest.new instead')
  static TestAlertRuleRequest create() => TestAlertRuleRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestAlertRuleRequest._();
  @$core.override
  TestAlertRuleRequest createEmptyInstance() => TestAlertRuleRequest._();
  @$core.pragma('dart2js:noInline')
  static TestAlertRuleRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestAlertRuleRequest>(
          TestAlertRuleRequest.$_createMessage);
  static TestAlertRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get notificationId => $_getI64(0);
  @$pb.TagNumber(1)
  set notificationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNotificationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotificationId() => $_clearField(1);
}

class TestAlertRuleResponse extends $pb.GeneratedMessage {
  factory TestAlertRuleResponse({
    $core.bool? delivered,
    $core.String? message,
  }) {
    final result = TestAlertRuleResponse._();
    if (delivered != null) result.delivered = delivered;
    if (message != null) result.message = message;
    return result;
  }

  TestAlertRuleResponse._();

  factory TestAlertRuleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestAlertRuleResponse()..mergeFromBuffer(data, registry);
  factory TestAlertRuleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestAlertRuleResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestAlertRuleResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestAlertRuleResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'delivered')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestAlertRuleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestAlertRuleResponse copyWith(
          void Function(TestAlertRuleResponse) updates) =>
      super.copyWith((message) => updates(message as TestAlertRuleResponse))
          as TestAlertRuleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestAlertRuleResponse() / TestAlertRuleResponse.new instead')
  static TestAlertRuleResponse create() => TestAlertRuleResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestAlertRuleResponse._();
  @$core.override
  TestAlertRuleResponse createEmptyInstance() => TestAlertRuleResponse._();
  @$core.pragma('dart2js:noInline')
  static TestAlertRuleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestAlertRuleResponse>(
          TestAlertRuleResponse.$_createMessage);
  static TestAlertRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get delivered => $_getBF(0);
  @$pb.TagNumber(1)
  set delivered($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDelivered() => $_has(0);
  @$pb.TagNumber(1)
  void clearDelivered() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

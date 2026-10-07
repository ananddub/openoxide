// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/alert.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'alert.pb.dart' as $1;
import 'alert_rule.pb.dart' as $0;
import 'common.pb.dart' as $2;

export 'alert.pb.dart';

/// Operations surface: rule CRUD (shared messages with AlertRuleService) plus the fired-event stream and its lifecycle actions.
@$pb.GrpcServiceName('openoxide.api.v1.AlertOperationsService')
class AlertOperationsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AlertOperationsServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListAlertRulesResponse> listRules(
    $0.ListAlertRulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRules, request, options: options);
  }

  $grpc.ResponseFuture<$1.ListAlertEventsResponse> listEvents(
    $1.ListAlertEventsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listEvents, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetAlertRuleResponse> getRule(
    $0.GetAlertRuleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRule, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateAlertRuleResponse> createRule(
    $0.CreateAlertRuleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createRule, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateAlertRuleResponse> updateRule(
    $0.UpdateAlertRuleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRule, request, options: options);
  }

  $grpc.ResponseFuture<$2.Empty> deleteRule(
    $0.DeleteAlertRuleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRule, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestAlertRuleResponse> testRule(
    $0.TestAlertRuleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testRule, request, options: options);
  }

  $grpc.ResponseFuture<$2.Empty> acknowledgeEvent(
    $1.AcknowledgeAlertEventRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acknowledgeEvent, request, options: options);
  }

  $grpc.ResponseFuture<$2.Empty> silenceEvent(
    $1.SilenceAlertEventRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$silenceEvent, request, options: options);
  }

  // method descriptors

  static final _$listRules =
      $grpc.ClientMethod<$0.ListAlertRulesRequest, $0.ListAlertRulesResponse>(
          '/openoxide.api.v1.AlertOperationsService/ListRules',
          ($0.ListAlertRulesRequest value) => value.writeToBuffer(),
          $0.ListAlertRulesResponse.fromBuffer);
  static final _$listEvents =
      $grpc.ClientMethod<$1.ListAlertEventsRequest, $1.ListAlertEventsResponse>(
          '/openoxide.api.v1.AlertOperationsService/ListEvents',
          ($1.ListAlertEventsRequest value) => value.writeToBuffer(),
          $1.ListAlertEventsResponse.fromBuffer);
  static final _$getRule =
      $grpc.ClientMethod<$0.GetAlertRuleRequest, $0.GetAlertRuleResponse>(
          '/openoxide.api.v1.AlertOperationsService/GetRule',
          ($0.GetAlertRuleRequest value) => value.writeToBuffer(),
          $0.GetAlertRuleResponse.fromBuffer);
  static final _$createRule =
      $grpc.ClientMethod<$0.CreateAlertRuleRequest, $0.CreateAlertRuleResponse>(
          '/openoxide.api.v1.AlertOperationsService/CreateRule',
          ($0.CreateAlertRuleRequest value) => value.writeToBuffer(),
          $0.CreateAlertRuleResponse.fromBuffer);
  static final _$updateRule =
      $grpc.ClientMethod<$0.UpdateAlertRuleRequest, $0.UpdateAlertRuleResponse>(
          '/openoxide.api.v1.AlertOperationsService/UpdateRule',
          ($0.UpdateAlertRuleRequest value) => value.writeToBuffer(),
          $0.UpdateAlertRuleResponse.fromBuffer);
  static final _$deleteRule =
      $grpc.ClientMethod<$0.DeleteAlertRuleRequest, $2.Empty>(
          '/openoxide.api.v1.AlertOperationsService/DeleteRule',
          ($0.DeleteAlertRuleRequest value) => value.writeToBuffer(),
          $2.Empty.fromBuffer);
  static final _$testRule =
      $grpc.ClientMethod<$0.TestAlertRuleRequest, $0.TestAlertRuleResponse>(
          '/openoxide.api.v1.AlertOperationsService/TestRule',
          ($0.TestAlertRuleRequest value) => value.writeToBuffer(),
          $0.TestAlertRuleResponse.fromBuffer);
  static final _$acknowledgeEvent =
      $grpc.ClientMethod<$1.AcknowledgeAlertEventRequest, $2.Empty>(
          '/openoxide.api.v1.AlertOperationsService/AcknowledgeEvent',
          ($1.AcknowledgeAlertEventRequest value) => value.writeToBuffer(),
          $2.Empty.fromBuffer);
  static final _$silenceEvent =
      $grpc.ClientMethod<$1.SilenceAlertEventRequest, $2.Empty>(
          '/openoxide.api.v1.AlertOperationsService/SilenceEvent',
          ($1.SilenceAlertEventRequest value) => value.writeToBuffer(),
          $2.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.AlertOperationsService')
abstract class AlertOperationsServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.AlertOperationsService';

  AlertOperationsServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListAlertRulesRequest,
            $0.ListAlertRulesResponse>(
        'ListRules',
        listRules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListAlertRulesRequest.fromBuffer(value),
        ($0.ListAlertRulesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.ListAlertEventsRequest,
            $1.ListAlertEventsResponse>(
        'ListEvents',
        listEvents_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.ListAlertEventsRequest.fromBuffer(value),
        ($1.ListAlertEventsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetAlertRuleRequest, $0.GetAlertRuleResponse>(
            'GetRule',
            getRule_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetAlertRuleRequest.fromBuffer(value),
            ($0.GetAlertRuleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateAlertRuleRequest,
            $0.CreateAlertRuleResponse>(
        'CreateRule',
        createRule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateAlertRuleRequest.fromBuffer(value),
        ($0.CreateAlertRuleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateAlertRuleRequest,
            $0.UpdateAlertRuleResponse>(
        'UpdateRule',
        updateRule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateAlertRuleRequest.fromBuffer(value),
        ($0.UpdateAlertRuleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteAlertRuleRequest, $2.Empty>(
        'DeleteRule',
        deleteRule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteAlertRuleRequest.fromBuffer(value),
        ($2.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TestAlertRuleRequest, $0.TestAlertRuleResponse>(
            'TestRule',
            testRule_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TestAlertRuleRequest.fromBuffer(value),
            ($0.TestAlertRuleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.AcknowledgeAlertEventRequest, $2.Empty>(
        'AcknowledgeEvent',
        acknowledgeEvent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.AcknowledgeAlertEventRequest.fromBuffer(value),
        ($2.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.SilenceAlertEventRequest, $2.Empty>(
        'SilenceEvent',
        silenceEvent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.SilenceAlertEventRequest.fromBuffer(value),
        ($2.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListAlertRulesResponse> listRules_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListAlertRulesRequest> $request) async {
    return listRules($call, await $request);
  }

  $async.Future<$0.ListAlertRulesResponse> listRules(
      $grpc.ServiceCall call, $0.ListAlertRulesRequest request);

  $async.Future<$1.ListAlertEventsResponse> listEvents_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$1.ListAlertEventsRequest> $request) async {
    return listEvents($call, await $request);
  }

  $async.Future<$1.ListAlertEventsResponse> listEvents(
      $grpc.ServiceCall call, $1.ListAlertEventsRequest request);

  $async.Future<$0.GetAlertRuleResponse> getRule_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetAlertRuleRequest> $request) async {
    return getRule($call, await $request);
  }

  $async.Future<$0.GetAlertRuleResponse> getRule(
      $grpc.ServiceCall call, $0.GetAlertRuleRequest request);

  $async.Future<$0.CreateAlertRuleResponse> createRule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateAlertRuleRequest> $request) async {
    return createRule($call, await $request);
  }

  $async.Future<$0.CreateAlertRuleResponse> createRule(
      $grpc.ServiceCall call, $0.CreateAlertRuleRequest request);

  $async.Future<$0.UpdateAlertRuleResponse> updateRule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateAlertRuleRequest> $request) async {
    return updateRule($call, await $request);
  }

  $async.Future<$0.UpdateAlertRuleResponse> updateRule(
      $grpc.ServiceCall call, $0.UpdateAlertRuleRequest request);

  $async.Future<$2.Empty> deleteRule_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteAlertRuleRequest> $request) async {
    return deleteRule($call, await $request);
  }

  $async.Future<$2.Empty> deleteRule(
      $grpc.ServiceCall call, $0.DeleteAlertRuleRequest request);

  $async.Future<$0.TestAlertRuleResponse> testRule_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TestAlertRuleRequest> $request) async {
    return testRule($call, await $request);
  }

  $async.Future<$0.TestAlertRuleResponse> testRule(
      $grpc.ServiceCall call, $0.TestAlertRuleRequest request);

  $async.Future<$2.Empty> acknowledgeEvent_Pre($grpc.ServiceCall $call,
      $async.Future<$1.AcknowledgeAlertEventRequest> $request) async {
    return acknowledgeEvent($call, await $request);
  }

  $async.Future<$2.Empty> acknowledgeEvent(
      $grpc.ServiceCall call, $1.AcknowledgeAlertEventRequest request);

  $async.Future<$2.Empty> silenceEvent_Pre($grpc.ServiceCall $call,
      $async.Future<$1.SilenceAlertEventRequest> $request) async {
    return silenceEvent($call, await $request);
  }

  $async.Future<$2.Empty> silenceEvent(
      $grpc.ServiceCall call, $1.SilenceAlertEventRequest request);
}

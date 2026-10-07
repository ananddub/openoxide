// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/schedule.proto.

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

import 'common.pb.dart' as $1;
import 'schedule.pb.dart' as $0;

export 'schedule.pb.dart';

/// Sole schedule owner service; absorbs the former ScheduleOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.ScheduleService')
class ScheduleServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ScheduleServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ScheduleListSchedulesResponse> listSchedules(
    $0.ScheduleListSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listSchedules, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleGetScheduleResponse> getSchedule(
    $0.ScheduleGetScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleCreateScheduleResponse> createSchedule(
    $0.ScheduleCreateScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleUpdateScheduleResponse> updateSchedule(
    $0.ScheduleUpdateScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteSchedule(
    $0.ScheduleDeleteScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleRunScheduleResponse> runSchedule(
    $0.ScheduleRunScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.SchedulePauseScheduleResponse> pauseSchedule(
    $0.SchedulePauseScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$pauseSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleResumeScheduleResponse> resumeSchedule(
    $0.ScheduleResumeScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$resumeSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListApplicationSchedulesResponse>
      listApplicationSchedules(
    $0.ScheduleListApplicationSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationSchedules, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListComposeSchedulesResponse>
      listComposeSchedules(
    $0.ScheduleListComposeSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listComposeSchedules, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListDatabaseSchedulesResponse>
      listDatabaseSchedules(
    $0.ScheduleListDatabaseSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDatabaseSchedules, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListServerSchedulesResponse>
      listServerSchedules(
    $0.ScheduleListServerSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listServerSchedules, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListOrganizationSchedulesResponse>
      listOrganizationSchedules(
    $0.ScheduleListOrganizationSchedulesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listOrganizationSchedules, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleTriggerScheduleResponse> triggerSchedule(
    $0.ScheduleTriggerScheduleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$triggerSchedule, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleGetRuntimePolicyResponse> getRuntimePolicy(
    $0.ScheduleGetRuntimePolicyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRuntimePolicy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleUpdateRuntimePolicyResponse>
      updateRuntimePolicy(
    $0.ScheduleUpdateRuntimePolicyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRuntimePolicy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleListExecutionsResponse> listExecutions(
    $0.ScheduleListExecutionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listExecutions, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleGetLogsResponse> getLogs(
    $0.ScheduleGetLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getLogs, request, options: options);
  }

  $grpc.ResponseFuture<$0.ScheduleGetExecutionLogsResponse> getExecutionLogs(
    $0.ScheduleGetExecutionLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getExecutionLogs, request, options: options);
  }

  // method descriptors

  static final _$listSchedules = $grpc.ClientMethod<
          $0.ScheduleListSchedulesRequest, $0.ScheduleListSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListSchedules',
      ($0.ScheduleListSchedulesRequest value) => value.writeToBuffer(),
      $0.ScheduleListSchedulesResponse.fromBuffer);
  static final _$getSchedule = $grpc.ClientMethod<$0.ScheduleGetScheduleRequest,
          $0.ScheduleGetScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/GetSchedule',
      ($0.ScheduleGetScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleGetScheduleResponse.fromBuffer);
  static final _$createSchedule = $grpc.ClientMethod<
          $0.ScheduleCreateScheduleRequest, $0.ScheduleCreateScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/CreateSchedule',
      ($0.ScheduleCreateScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleCreateScheduleResponse.fromBuffer);
  static final _$updateSchedule = $grpc.ClientMethod<
          $0.ScheduleUpdateScheduleRequest, $0.ScheduleUpdateScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/UpdateSchedule',
      ($0.ScheduleUpdateScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleUpdateScheduleResponse.fromBuffer);
  static final _$deleteSchedule =
      $grpc.ClientMethod<$0.ScheduleDeleteScheduleRequest, $1.Empty>(
          '/openoxide.api.v1.ScheduleService/DeleteSchedule',
          ($0.ScheduleDeleteScheduleRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$runSchedule = $grpc.ClientMethod<$0.ScheduleRunScheduleRequest,
          $0.ScheduleRunScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/RunSchedule',
      ($0.ScheduleRunScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleRunScheduleResponse.fromBuffer);
  static final _$pauseSchedule = $grpc.ClientMethod<
          $0.SchedulePauseScheduleRequest, $0.SchedulePauseScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/PauseSchedule',
      ($0.SchedulePauseScheduleRequest value) => value.writeToBuffer(),
      $0.SchedulePauseScheduleResponse.fromBuffer);
  static final _$resumeSchedule = $grpc.ClientMethod<
          $0.ScheduleResumeScheduleRequest, $0.ScheduleResumeScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/ResumeSchedule',
      ($0.ScheduleResumeScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleResumeScheduleResponse.fromBuffer);
  static final _$listApplicationSchedules = $grpc.ClientMethod<
          $0.ScheduleListApplicationSchedulesRequest,
          $0.ScheduleListApplicationSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListApplicationSchedules',
      ($0.ScheduleListApplicationSchedulesRequest value) =>
          value.writeToBuffer(),
      $0.ScheduleListApplicationSchedulesResponse.fromBuffer);
  static final _$listComposeSchedules = $grpc.ClientMethod<
          $0.ScheduleListComposeSchedulesRequest,
          $0.ScheduleListComposeSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListComposeSchedules',
      ($0.ScheduleListComposeSchedulesRequest value) => value.writeToBuffer(),
      $0.ScheduleListComposeSchedulesResponse.fromBuffer);
  static final _$listDatabaseSchedules = $grpc.ClientMethod<
          $0.ScheduleListDatabaseSchedulesRequest,
          $0.ScheduleListDatabaseSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListDatabaseSchedules',
      ($0.ScheduleListDatabaseSchedulesRequest value) => value.writeToBuffer(),
      $0.ScheduleListDatabaseSchedulesResponse.fromBuffer);
  static final _$listServerSchedules = $grpc.ClientMethod<
          $0.ScheduleListServerSchedulesRequest,
          $0.ScheduleListServerSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListServerSchedules',
      ($0.ScheduleListServerSchedulesRequest value) => value.writeToBuffer(),
      $0.ScheduleListServerSchedulesResponse.fromBuffer);
  static final _$listOrganizationSchedules = $grpc.ClientMethod<
          $0.ScheduleListOrganizationSchedulesRequest,
          $0.ScheduleListOrganizationSchedulesResponse>(
      '/openoxide.api.v1.ScheduleService/ListOrganizationSchedules',
      ($0.ScheduleListOrganizationSchedulesRequest value) =>
          value.writeToBuffer(),
      $0.ScheduleListOrganizationSchedulesResponse.fromBuffer);
  static final _$triggerSchedule = $grpc.ClientMethod<
          $0.ScheduleTriggerScheduleRequest,
          $0.ScheduleTriggerScheduleResponse>(
      '/openoxide.api.v1.ScheduleService/TriggerSchedule',
      ($0.ScheduleTriggerScheduleRequest value) => value.writeToBuffer(),
      $0.ScheduleTriggerScheduleResponse.fromBuffer);
  static final _$getRuntimePolicy = $grpc.ClientMethod<
          $0.ScheduleGetRuntimePolicyRequest,
          $0.ScheduleGetRuntimePolicyResponse>(
      '/openoxide.api.v1.ScheduleService/GetRuntimePolicy',
      ($0.ScheduleGetRuntimePolicyRequest value) => value.writeToBuffer(),
      $0.ScheduleGetRuntimePolicyResponse.fromBuffer);
  static final _$updateRuntimePolicy = $grpc.ClientMethod<
          $0.ScheduleUpdateRuntimePolicyRequest,
          $0.ScheduleUpdateRuntimePolicyResponse>(
      '/openoxide.api.v1.ScheduleService/UpdateRuntimePolicy',
      ($0.ScheduleUpdateRuntimePolicyRequest value) => value.writeToBuffer(),
      $0.ScheduleUpdateRuntimePolicyResponse.fromBuffer);
  static final _$listExecutions = $grpc.ClientMethod<
          $0.ScheduleListExecutionsRequest, $0.ScheduleListExecutionsResponse>(
      '/openoxide.api.v1.ScheduleService/ListExecutions',
      ($0.ScheduleListExecutionsRequest value) => value.writeToBuffer(),
      $0.ScheduleListExecutionsResponse.fromBuffer);
  static final _$getLogs =
      $grpc.ClientMethod<$0.ScheduleGetLogsRequest, $0.ScheduleGetLogsResponse>(
          '/openoxide.api.v1.ScheduleService/GetLogs',
          ($0.ScheduleGetLogsRequest value) => value.writeToBuffer(),
          $0.ScheduleGetLogsResponse.fromBuffer);
  static final _$getExecutionLogs = $grpc.ClientMethod<
          $0.ScheduleGetExecutionLogsRequest,
          $0.ScheduleGetExecutionLogsResponse>(
      '/openoxide.api.v1.ScheduleService/GetExecutionLogs',
      ($0.ScheduleGetExecutionLogsRequest value) => value.writeToBuffer(),
      $0.ScheduleGetExecutionLogsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ScheduleService')
abstract class ScheduleServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ScheduleService';

  ScheduleServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ScheduleListSchedulesRequest,
            $0.ScheduleListSchedulesResponse>(
        'ListSchedules',
        listSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListSchedulesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleGetScheduleRequest,
            $0.ScheduleGetScheduleResponse>(
        'GetSchedule',
        getSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleGetScheduleRequest.fromBuffer(value),
        ($0.ScheduleGetScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleCreateScheduleRequest,
            $0.ScheduleCreateScheduleResponse>(
        'CreateSchedule',
        createSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleCreateScheduleRequest.fromBuffer(value),
        ($0.ScheduleCreateScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleUpdateScheduleRequest,
            $0.ScheduleUpdateScheduleResponse>(
        'UpdateSchedule',
        updateSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleUpdateScheduleRequest.fromBuffer(value),
        ($0.ScheduleUpdateScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleDeleteScheduleRequest, $1.Empty>(
        'DeleteSchedule',
        deleteSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleDeleteScheduleRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleRunScheduleRequest,
            $0.ScheduleRunScheduleResponse>(
        'RunSchedule',
        runSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleRunScheduleRequest.fromBuffer(value),
        ($0.ScheduleRunScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SchedulePauseScheduleRequest,
            $0.SchedulePauseScheduleResponse>(
        'PauseSchedule',
        pauseSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SchedulePauseScheduleRequest.fromBuffer(value),
        ($0.SchedulePauseScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleResumeScheduleRequest,
            $0.ScheduleResumeScheduleResponse>(
        'ResumeSchedule',
        resumeSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleResumeScheduleRequest.fromBuffer(value),
        ($0.ScheduleResumeScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListApplicationSchedulesRequest,
            $0.ScheduleListApplicationSchedulesResponse>(
        'ListApplicationSchedules',
        listApplicationSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListApplicationSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListApplicationSchedulesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListComposeSchedulesRequest,
            $0.ScheduleListComposeSchedulesResponse>(
        'ListComposeSchedules',
        listComposeSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListComposeSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListComposeSchedulesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListDatabaseSchedulesRequest,
            $0.ScheduleListDatabaseSchedulesResponse>(
        'ListDatabaseSchedules',
        listDatabaseSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListDatabaseSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListDatabaseSchedulesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListServerSchedulesRequest,
            $0.ScheduleListServerSchedulesResponse>(
        'ListServerSchedules',
        listServerSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListServerSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListServerSchedulesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListOrganizationSchedulesRequest,
            $0.ScheduleListOrganizationSchedulesResponse>(
        'ListOrganizationSchedules',
        listOrganizationSchedules_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListOrganizationSchedulesRequest.fromBuffer(value),
        ($0.ScheduleListOrganizationSchedulesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleTriggerScheduleRequest,
            $0.ScheduleTriggerScheduleResponse>(
        'TriggerSchedule',
        triggerSchedule_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleTriggerScheduleRequest.fromBuffer(value),
        ($0.ScheduleTriggerScheduleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleGetRuntimePolicyRequest,
            $0.ScheduleGetRuntimePolicyResponse>(
        'GetRuntimePolicy',
        getRuntimePolicy_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleGetRuntimePolicyRequest.fromBuffer(value),
        ($0.ScheduleGetRuntimePolicyResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleUpdateRuntimePolicyRequest,
            $0.ScheduleUpdateRuntimePolicyResponse>(
        'UpdateRuntimePolicy',
        updateRuntimePolicy_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleUpdateRuntimePolicyRequest.fromBuffer(value),
        ($0.ScheduleUpdateRuntimePolicyResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleListExecutionsRequest,
            $0.ScheduleListExecutionsResponse>(
        'ListExecutions',
        listExecutions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleListExecutionsRequest.fromBuffer(value),
        ($0.ScheduleListExecutionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleGetLogsRequest,
            $0.ScheduleGetLogsResponse>(
        'GetLogs',
        getLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleGetLogsRequest.fromBuffer(value),
        ($0.ScheduleGetLogsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ScheduleGetExecutionLogsRequest,
            $0.ScheduleGetExecutionLogsResponse>(
        'GetExecutionLogs',
        getExecutionLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ScheduleGetExecutionLogsRequest.fromBuffer(value),
        ($0.ScheduleGetExecutionLogsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ScheduleListSchedulesResponse> listSchedules_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleListSchedulesRequest> $request) async {
    return listSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListSchedulesResponse> listSchedules(
      $grpc.ServiceCall call, $0.ScheduleListSchedulesRequest request);

  $async.Future<$0.ScheduleGetScheduleResponse> getSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleGetScheduleRequest> $request) async {
    return getSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleGetScheduleResponse> getSchedule(
      $grpc.ServiceCall call, $0.ScheduleGetScheduleRequest request);

  $async.Future<$0.ScheduleCreateScheduleResponse> createSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleCreateScheduleRequest> $request) async {
    return createSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleCreateScheduleResponse> createSchedule(
      $grpc.ServiceCall call, $0.ScheduleCreateScheduleRequest request);

  $async.Future<$0.ScheduleUpdateScheduleResponse> updateSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleUpdateScheduleRequest> $request) async {
    return updateSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleUpdateScheduleResponse> updateSchedule(
      $grpc.ServiceCall call, $0.ScheduleUpdateScheduleRequest request);

  $async.Future<$1.Empty> deleteSchedule_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ScheduleDeleteScheduleRequest> $request) async {
    return deleteSchedule($call, await $request);
  }

  $async.Future<$1.Empty> deleteSchedule(
      $grpc.ServiceCall call, $0.ScheduleDeleteScheduleRequest request);

  $async.Future<$0.ScheduleRunScheduleResponse> runSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleRunScheduleRequest> $request) async {
    return runSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleRunScheduleResponse> runSchedule(
      $grpc.ServiceCall call, $0.ScheduleRunScheduleRequest request);

  $async.Future<$0.SchedulePauseScheduleResponse> pauseSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SchedulePauseScheduleRequest> $request) async {
    return pauseSchedule($call, await $request);
  }

  $async.Future<$0.SchedulePauseScheduleResponse> pauseSchedule(
      $grpc.ServiceCall call, $0.SchedulePauseScheduleRequest request);

  $async.Future<$0.ScheduleResumeScheduleResponse> resumeSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleResumeScheduleRequest> $request) async {
    return resumeSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleResumeScheduleResponse> resumeSchedule(
      $grpc.ServiceCall call, $0.ScheduleResumeScheduleRequest request);

  $async.Future<$0.ScheduleListApplicationSchedulesResponse>
      listApplicationSchedules_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ScheduleListApplicationSchedulesRequest>
              $request) async {
    return listApplicationSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListApplicationSchedulesResponse>
      listApplicationSchedules($grpc.ServiceCall call,
          $0.ScheduleListApplicationSchedulesRequest request);

  $async.Future<$0.ScheduleListComposeSchedulesResponse>
      listComposeSchedules_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ScheduleListComposeSchedulesRequest>
              $request) async {
    return listComposeSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListComposeSchedulesResponse> listComposeSchedules(
      $grpc.ServiceCall call, $0.ScheduleListComposeSchedulesRequest request);

  $async.Future<$0.ScheduleListDatabaseSchedulesResponse>
      listDatabaseSchedules_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ScheduleListDatabaseSchedulesRequest>
              $request) async {
    return listDatabaseSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListDatabaseSchedulesResponse> listDatabaseSchedules(
      $grpc.ServiceCall call, $0.ScheduleListDatabaseSchedulesRequest request);

  $async.Future<$0.ScheduleListServerSchedulesResponse> listServerSchedules_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleListServerSchedulesRequest> $request) async {
    return listServerSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListServerSchedulesResponse> listServerSchedules(
      $grpc.ServiceCall call, $0.ScheduleListServerSchedulesRequest request);

  $async.Future<$0.ScheduleListOrganizationSchedulesResponse>
      listOrganizationSchedules_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ScheduleListOrganizationSchedulesRequest>
              $request) async {
    return listOrganizationSchedules($call, await $request);
  }

  $async.Future<$0.ScheduleListOrganizationSchedulesResponse>
      listOrganizationSchedules($grpc.ServiceCall call,
          $0.ScheduleListOrganizationSchedulesRequest request);

  $async.Future<$0.ScheduleTriggerScheduleResponse> triggerSchedule_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleTriggerScheduleRequest> $request) async {
    return triggerSchedule($call, await $request);
  }

  $async.Future<$0.ScheduleTriggerScheduleResponse> triggerSchedule(
      $grpc.ServiceCall call, $0.ScheduleTriggerScheduleRequest request);

  $async.Future<$0.ScheduleGetRuntimePolicyResponse> getRuntimePolicy_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleGetRuntimePolicyRequest> $request) async {
    return getRuntimePolicy($call, await $request);
  }

  $async.Future<$0.ScheduleGetRuntimePolicyResponse> getRuntimePolicy(
      $grpc.ServiceCall call, $0.ScheduleGetRuntimePolicyRequest request);

  $async.Future<$0.ScheduleUpdateRuntimePolicyResponse> updateRuntimePolicy_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleUpdateRuntimePolicyRequest> $request) async {
    return updateRuntimePolicy($call, await $request);
  }

  $async.Future<$0.ScheduleUpdateRuntimePolicyResponse> updateRuntimePolicy(
      $grpc.ServiceCall call, $0.ScheduleUpdateRuntimePolicyRequest request);

  $async.Future<$0.ScheduleListExecutionsResponse> listExecutions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleListExecutionsRequest> $request) async {
    return listExecutions($call, await $request);
  }

  $async.Future<$0.ScheduleListExecutionsResponse> listExecutions(
      $grpc.ServiceCall call, $0.ScheduleListExecutionsRequest request);

  $async.Future<$0.ScheduleGetLogsResponse> getLogs_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ScheduleGetLogsRequest> $request) async {
    return getLogs($call, await $request);
  }

  $async.Future<$0.ScheduleGetLogsResponse> getLogs(
      $grpc.ServiceCall call, $0.ScheduleGetLogsRequest request);

  $async.Future<$0.ScheduleGetExecutionLogsResponse> getExecutionLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ScheduleGetExecutionLogsRequest> $request) async {
    return getExecutionLogs($call, await $request);
  }

  $async.Future<$0.ScheduleGetExecutionLogsResponse> getExecutionLogs(
      $grpc.ServiceCall call, $0.ScheduleGetExecutionLogsRequest request);
}

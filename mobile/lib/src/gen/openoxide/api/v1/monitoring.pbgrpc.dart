// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/monitoring.proto.

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
import 'monitoring.pb.dart' as $0;

export 'monitoring.pb.dart';

/// Sole monitoring owner service; absorbs the former MonitoringOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.MonitoringService')
class MonitoringServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  MonitoringServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$1.Empty> getOverview(
    $0.MonitoringGetOverviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getOverview, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> getMetrics(
    $0.MonitoringGetMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> getContainerState(
    $0.MonitoringGetContainerStateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getContainerState, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetLocalMetricsResponse> getLocalMetrics(
    $0.MonitoringGetLocalMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getLocalMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetServerMetricsResponse> getServerMetrics(
    $0.MonitoringGetServerMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getServerMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetContainerMetricsResponse>
      getContainerMetrics(
    $0.MonitoringGetContainerMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getContainerMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetContainerStatesResponse>
      getContainerStates(
    $0.MonitoringGetContainerStatesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getContainerStates, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringCreateAgentTokenResponse> createAgentToken(
    $0.MonitoringCreateAgentTokenRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createAgentToken, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetAgentStatusResponse> getAgentStatus(
    $0.MonitoringGetAgentStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getAgentStatus, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringStartAgentResponse> startAgent(
    $0.MonitoringStartAgentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$startAgent, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringStopAgentResponse> stopAgent(
    $0.MonitoringStopAgentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stopAgent, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringRestartAgentResponse> restartAgent(
    $0.MonitoringRestartAgentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$restartAgent, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringReinstallAgentResponse> reinstallAgent(
    $0.MonitoringReinstallAgentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reinstallAgent, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringExportServerMetricsResponse>
      exportServerMetrics(
    $0.MonitoringExportServerMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportServerMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringGetPrometheusMetricsResponse>
      getPrometheusMetrics(
    $0.MonitoringGetPrometheusMetricsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPrometheusMetrics, request, options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringCreateMaintenanceWindowResponse>
      createMaintenanceWindow(
    $0.MonitoringCreateMaintenanceWindowRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createMaintenanceWindow, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.MonitoringListMaintenanceWindowsResponse>
      listMaintenanceWindows(
    $0.MonitoringListMaintenanceWindowsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMaintenanceWindows, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteMaintenanceWindow(
    $0.MonitoringDeleteMaintenanceWindowRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteMaintenanceWindow, request,
        options: options);
  }

  // method descriptors

  static final _$getOverview =
      $grpc.ClientMethod<$0.MonitoringGetOverviewRequest, $1.Empty>(
          '/openoxide.api.v1.MonitoringService/GetOverview',
          ($0.MonitoringGetOverviewRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getMetrics =
      $grpc.ClientMethod<$0.MonitoringGetMetricsRequest, $1.Empty>(
          '/openoxide.api.v1.MonitoringService/GetMetrics',
          ($0.MonitoringGetMetricsRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getContainerState =
      $grpc.ClientMethod<$0.MonitoringGetContainerStateRequest, $1.Empty>(
          '/openoxide.api.v1.MonitoringService/GetContainerState',
          ($0.MonitoringGetContainerStateRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getLocalMetrics = $grpc.ClientMethod<
          $0.MonitoringGetLocalMetricsRequest,
          $0.MonitoringGetLocalMetricsResponse>(
      '/openoxide.api.v1.MonitoringService/GetLocalMetrics',
      ($0.MonitoringGetLocalMetricsRequest value) => value.writeToBuffer(),
      $0.MonitoringGetLocalMetricsResponse.fromBuffer);
  static final _$getServerMetrics = $grpc.ClientMethod<
          $0.MonitoringGetServerMetricsRequest,
          $0.MonitoringGetServerMetricsResponse>(
      '/openoxide.api.v1.MonitoringService/GetServerMetrics',
      ($0.MonitoringGetServerMetricsRequest value) => value.writeToBuffer(),
      $0.MonitoringGetServerMetricsResponse.fromBuffer);
  static final _$getContainerMetrics = $grpc.ClientMethod<
          $0.MonitoringGetContainerMetricsRequest,
          $0.MonitoringGetContainerMetricsResponse>(
      '/openoxide.api.v1.MonitoringService/GetContainerMetrics',
      ($0.MonitoringGetContainerMetricsRequest value) => value.writeToBuffer(),
      $0.MonitoringGetContainerMetricsResponse.fromBuffer);
  static final _$getContainerStates = $grpc.ClientMethod<
          $0.MonitoringGetContainerStatesRequest,
          $0.MonitoringGetContainerStatesResponse>(
      '/openoxide.api.v1.MonitoringService/GetContainerStates',
      ($0.MonitoringGetContainerStatesRequest value) => value.writeToBuffer(),
      $0.MonitoringGetContainerStatesResponse.fromBuffer);
  static final _$createAgentToken = $grpc.ClientMethod<
          $0.MonitoringCreateAgentTokenRequest,
          $0.MonitoringCreateAgentTokenResponse>(
      '/openoxide.api.v1.MonitoringService/CreateAgentToken',
      ($0.MonitoringCreateAgentTokenRequest value) => value.writeToBuffer(),
      $0.MonitoringCreateAgentTokenResponse.fromBuffer);
  static final _$getAgentStatus = $grpc.ClientMethod<
          $0.MonitoringGetAgentStatusRequest,
          $0.MonitoringGetAgentStatusResponse>(
      '/openoxide.api.v1.MonitoringService/GetAgentStatus',
      ($0.MonitoringGetAgentStatusRequest value) => value.writeToBuffer(),
      $0.MonitoringGetAgentStatusResponse.fromBuffer);
  static final _$startAgent = $grpc.ClientMethod<$0.MonitoringStartAgentRequest,
          $0.MonitoringStartAgentResponse>(
      '/openoxide.api.v1.MonitoringService/StartAgent',
      ($0.MonitoringStartAgentRequest value) => value.writeToBuffer(),
      $0.MonitoringStartAgentResponse.fromBuffer);
  static final _$stopAgent = $grpc.ClientMethod<$0.MonitoringStopAgentRequest,
          $0.MonitoringStopAgentResponse>(
      '/openoxide.api.v1.MonitoringService/StopAgent',
      ($0.MonitoringStopAgentRequest value) => value.writeToBuffer(),
      $0.MonitoringStopAgentResponse.fromBuffer);
  static final _$restartAgent = $grpc.ClientMethod<
          $0.MonitoringRestartAgentRequest, $0.MonitoringRestartAgentResponse>(
      '/openoxide.api.v1.MonitoringService/RestartAgent',
      ($0.MonitoringRestartAgentRequest value) => value.writeToBuffer(),
      $0.MonitoringRestartAgentResponse.fromBuffer);
  static final _$reinstallAgent = $grpc.ClientMethod<
          $0.MonitoringReinstallAgentRequest,
          $0.MonitoringReinstallAgentResponse>(
      '/openoxide.api.v1.MonitoringService/ReinstallAgent',
      ($0.MonitoringReinstallAgentRequest value) => value.writeToBuffer(),
      $0.MonitoringReinstallAgentResponse.fromBuffer);
  static final _$exportServerMetrics = $grpc.ClientMethod<
          $0.MonitoringExportServerMetricsRequest,
          $0.MonitoringExportServerMetricsResponse>(
      '/openoxide.api.v1.MonitoringService/ExportServerMetrics',
      ($0.MonitoringExportServerMetricsRequest value) => value.writeToBuffer(),
      $0.MonitoringExportServerMetricsResponse.fromBuffer);
  static final _$getPrometheusMetrics = $grpc.ClientMethod<
          $0.MonitoringGetPrometheusMetricsRequest,
          $0.MonitoringGetPrometheusMetricsResponse>(
      '/openoxide.api.v1.MonitoringService/GetPrometheusMetrics',
      ($0.MonitoringGetPrometheusMetricsRequest value) => value.writeToBuffer(),
      $0.MonitoringGetPrometheusMetricsResponse.fromBuffer);
  static final _$createMaintenanceWindow = $grpc.ClientMethod<
          $0.MonitoringCreateMaintenanceWindowRequest,
          $0.MonitoringCreateMaintenanceWindowResponse>(
      '/openoxide.api.v1.MonitoringService/CreateMaintenanceWindow',
      ($0.MonitoringCreateMaintenanceWindowRequest value) =>
          value.writeToBuffer(),
      $0.MonitoringCreateMaintenanceWindowResponse.fromBuffer);
  static final _$listMaintenanceWindows = $grpc.ClientMethod<
          $0.MonitoringListMaintenanceWindowsRequest,
          $0.MonitoringListMaintenanceWindowsResponse>(
      '/openoxide.api.v1.MonitoringService/ListMaintenanceWindows',
      ($0.MonitoringListMaintenanceWindowsRequest value) =>
          value.writeToBuffer(),
      $0.MonitoringListMaintenanceWindowsResponse.fromBuffer);
  static final _$deleteMaintenanceWindow =
      $grpc.ClientMethod<$0.MonitoringDeleteMaintenanceWindowRequest, $1.Empty>(
          '/openoxide.api.v1.MonitoringService/DeleteMaintenanceWindow',
          ($0.MonitoringDeleteMaintenanceWindowRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.MonitoringService')
abstract class MonitoringServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.MonitoringService';

  MonitoringServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetOverviewRequest, $1.Empty>(
        'GetOverview',
        getOverview_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetOverviewRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetMetricsRequest, $1.Empty>(
        'GetMetrics',
        getMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetMetricsRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.MonitoringGetContainerStateRequest, $1.Empty>(
            'GetContainerState',
            getContainerState_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.MonitoringGetContainerStateRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetLocalMetricsRequest,
            $0.MonitoringGetLocalMetricsResponse>(
        'GetLocalMetrics',
        getLocalMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetLocalMetricsRequest.fromBuffer(value),
        ($0.MonitoringGetLocalMetricsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetServerMetricsRequest,
            $0.MonitoringGetServerMetricsResponse>(
        'GetServerMetrics',
        getServerMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetServerMetricsRequest.fromBuffer(value),
        ($0.MonitoringGetServerMetricsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetContainerMetricsRequest,
            $0.MonitoringGetContainerMetricsResponse>(
        'GetContainerMetrics',
        getContainerMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetContainerMetricsRequest.fromBuffer(value),
        ($0.MonitoringGetContainerMetricsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetContainerStatesRequest,
            $0.MonitoringGetContainerStatesResponse>(
        'GetContainerStates',
        getContainerStates_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetContainerStatesRequest.fromBuffer(value),
        ($0.MonitoringGetContainerStatesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringCreateAgentTokenRequest,
            $0.MonitoringCreateAgentTokenResponse>(
        'CreateAgentToken',
        createAgentToken_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringCreateAgentTokenRequest.fromBuffer(value),
        ($0.MonitoringCreateAgentTokenResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetAgentStatusRequest,
            $0.MonitoringGetAgentStatusResponse>(
        'GetAgentStatus',
        getAgentStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetAgentStatusRequest.fromBuffer(value),
        ($0.MonitoringGetAgentStatusResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringStartAgentRequest,
            $0.MonitoringStartAgentResponse>(
        'StartAgent',
        startAgent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringStartAgentRequest.fromBuffer(value),
        ($0.MonitoringStartAgentResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringStopAgentRequest,
            $0.MonitoringStopAgentResponse>(
        'StopAgent',
        stopAgent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringStopAgentRequest.fromBuffer(value),
        ($0.MonitoringStopAgentResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringRestartAgentRequest,
            $0.MonitoringRestartAgentResponse>(
        'RestartAgent',
        restartAgent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringRestartAgentRequest.fromBuffer(value),
        ($0.MonitoringRestartAgentResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringReinstallAgentRequest,
            $0.MonitoringReinstallAgentResponse>(
        'ReinstallAgent',
        reinstallAgent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringReinstallAgentRequest.fromBuffer(value),
        ($0.MonitoringReinstallAgentResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringExportServerMetricsRequest,
            $0.MonitoringExportServerMetricsResponse>(
        'ExportServerMetrics',
        exportServerMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringExportServerMetricsRequest.fromBuffer(value),
        ($0.MonitoringExportServerMetricsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringGetPrometheusMetricsRequest,
            $0.MonitoringGetPrometheusMetricsResponse>(
        'GetPrometheusMetrics',
        getPrometheusMetrics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringGetPrometheusMetricsRequest.fromBuffer(value),
        ($0.MonitoringGetPrometheusMetricsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringCreateMaintenanceWindowRequest,
            $0.MonitoringCreateMaintenanceWindowResponse>(
        'CreateMaintenanceWindow',
        createMaintenanceWindow_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringCreateMaintenanceWindowRequest.fromBuffer(value),
        ($0.MonitoringCreateMaintenanceWindowResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringListMaintenanceWindowsRequest,
            $0.MonitoringListMaintenanceWindowsResponse>(
        'ListMaintenanceWindows',
        listMaintenanceWindows_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringListMaintenanceWindowsRequest.fromBuffer(value),
        ($0.MonitoringListMaintenanceWindowsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MonitoringDeleteMaintenanceWindowRequest,
            $1.Empty>(
        'DeleteMaintenanceWindow',
        deleteMaintenanceWindow_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MonitoringDeleteMaintenanceWindowRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$1.Empty> getOverview_Pre($grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetOverviewRequest> $request) async {
    return getOverview($call, await $request);
  }

  $async.Future<$1.Empty> getOverview(
      $grpc.ServiceCall call, $0.MonitoringGetOverviewRequest request);

  $async.Future<$1.Empty> getMetrics_Pre($grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetMetricsRequest> $request) async {
    return getMetrics($call, await $request);
  }

  $async.Future<$1.Empty> getMetrics(
      $grpc.ServiceCall call, $0.MonitoringGetMetricsRequest request);

  $async.Future<$1.Empty> getContainerState_Pre($grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetContainerStateRequest> $request) async {
    return getContainerState($call, await $request);
  }

  $async.Future<$1.Empty> getContainerState(
      $grpc.ServiceCall call, $0.MonitoringGetContainerStateRequest request);

  $async.Future<$0.MonitoringGetLocalMetricsResponse> getLocalMetrics_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetLocalMetricsRequest> $request) async {
    return getLocalMetrics($call, await $request);
  }

  $async.Future<$0.MonitoringGetLocalMetricsResponse> getLocalMetrics(
      $grpc.ServiceCall call, $0.MonitoringGetLocalMetricsRequest request);

  $async.Future<$0.MonitoringGetServerMetricsResponse> getServerMetrics_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetServerMetricsRequest> $request) async {
    return getServerMetrics($call, await $request);
  }

  $async.Future<$0.MonitoringGetServerMetricsResponse> getServerMetrics(
      $grpc.ServiceCall call, $0.MonitoringGetServerMetricsRequest request);

  $async.Future<$0.MonitoringGetContainerMetricsResponse>
      getContainerMetrics_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.MonitoringGetContainerMetricsRequest>
              $request) async {
    return getContainerMetrics($call, await $request);
  }

  $async.Future<$0.MonitoringGetContainerMetricsResponse> getContainerMetrics(
      $grpc.ServiceCall call, $0.MonitoringGetContainerMetricsRequest request);

  $async.Future<$0.MonitoringGetContainerStatesResponse> getContainerStates_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetContainerStatesRequest> $request) async {
    return getContainerStates($call, await $request);
  }

  $async.Future<$0.MonitoringGetContainerStatesResponse> getContainerStates(
      $grpc.ServiceCall call, $0.MonitoringGetContainerStatesRequest request);

  $async.Future<$0.MonitoringCreateAgentTokenResponse> createAgentToken_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringCreateAgentTokenRequest> $request) async {
    return createAgentToken($call, await $request);
  }

  $async.Future<$0.MonitoringCreateAgentTokenResponse> createAgentToken(
      $grpc.ServiceCall call, $0.MonitoringCreateAgentTokenRequest request);

  $async.Future<$0.MonitoringGetAgentStatusResponse> getAgentStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringGetAgentStatusRequest> $request) async {
    return getAgentStatus($call, await $request);
  }

  $async.Future<$0.MonitoringGetAgentStatusResponse> getAgentStatus(
      $grpc.ServiceCall call, $0.MonitoringGetAgentStatusRequest request);

  $async.Future<$0.MonitoringStartAgentResponse> startAgent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringStartAgentRequest> $request) async {
    return startAgent($call, await $request);
  }

  $async.Future<$0.MonitoringStartAgentResponse> startAgent(
      $grpc.ServiceCall call, $0.MonitoringStartAgentRequest request);

  $async.Future<$0.MonitoringStopAgentResponse> stopAgent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringStopAgentRequest> $request) async {
    return stopAgent($call, await $request);
  }

  $async.Future<$0.MonitoringStopAgentResponse> stopAgent(
      $grpc.ServiceCall call, $0.MonitoringStopAgentRequest request);

  $async.Future<$0.MonitoringRestartAgentResponse> restartAgent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringRestartAgentRequest> $request) async {
    return restartAgent($call, await $request);
  }

  $async.Future<$0.MonitoringRestartAgentResponse> restartAgent(
      $grpc.ServiceCall call, $0.MonitoringRestartAgentRequest request);

  $async.Future<$0.MonitoringReinstallAgentResponse> reinstallAgent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringReinstallAgentRequest> $request) async {
    return reinstallAgent($call, await $request);
  }

  $async.Future<$0.MonitoringReinstallAgentResponse> reinstallAgent(
      $grpc.ServiceCall call, $0.MonitoringReinstallAgentRequest request);

  $async.Future<$0.MonitoringExportServerMetricsResponse>
      exportServerMetrics_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.MonitoringExportServerMetricsRequest>
              $request) async {
    return exportServerMetrics($call, await $request);
  }

  $async.Future<$0.MonitoringExportServerMetricsResponse> exportServerMetrics(
      $grpc.ServiceCall call, $0.MonitoringExportServerMetricsRequest request);

  $async.Future<$0.MonitoringGetPrometheusMetricsResponse>
      getPrometheusMetrics_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.MonitoringGetPrometheusMetricsRequest>
              $request) async {
    return getPrometheusMetrics($call, await $request);
  }

  $async.Future<$0.MonitoringGetPrometheusMetricsResponse> getPrometheusMetrics(
      $grpc.ServiceCall call, $0.MonitoringGetPrometheusMetricsRequest request);

  $async.Future<$0.MonitoringCreateMaintenanceWindowResponse>
      createMaintenanceWindow_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.MonitoringCreateMaintenanceWindowRequest>
              $request) async {
    return createMaintenanceWindow($call, await $request);
  }

  $async.Future<$0.MonitoringCreateMaintenanceWindowResponse>
      createMaintenanceWindow($grpc.ServiceCall call,
          $0.MonitoringCreateMaintenanceWindowRequest request);

  $async.Future<$0.MonitoringListMaintenanceWindowsResponse>
      listMaintenanceWindows_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.MonitoringListMaintenanceWindowsRequest>
              $request) async {
    return listMaintenanceWindows($call, await $request);
  }

  $async.Future<$0.MonitoringListMaintenanceWindowsResponse>
      listMaintenanceWindows($grpc.ServiceCall call,
          $0.MonitoringListMaintenanceWindowsRequest request);

  $async.Future<$1.Empty> deleteMaintenanceWindow_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MonitoringDeleteMaintenanceWindowRequest>
          $request) async {
    return deleteMaintenanceWindow($call, await $request);
  }

  $async.Future<$1.Empty> deleteMaintenanceWindow($grpc.ServiceCall call,
      $0.MonitoringDeleteMaintenanceWindowRequest request);
}

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/deployment.proto.

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
import 'deployment.pb.dart' as $0;

export 'deployment.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.DeploymentService')
class DeploymentServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DeploymentServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListDeploymentsResponse> listDeployments(
    $0.ListDeploymentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDeployments, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDeploymentsResponse> searchLogs(
    $0.SearchLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$searchLogs, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDeploymentsResponse> listActiveDeployments(
    $1.Empty request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listActiveDeployments, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDeploymentsResponse> listRunningDeployments(
    $1.Empty request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRunningDeployments, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.Deployment> getDeployment(
    $0.GetDeploymentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDeployment, request, options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamApplicationStats(
    $0.StreamTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamApplicationStats, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamComposeStats(
    $0.StreamTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamComposeStats, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamDatabaseStats(
    $0.StreamTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamDatabaseStats, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamContainerLogs(
    $0.LogTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamContainerLogs, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.LogExport> exportContainerLogs(
    $0.LogTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportContainerLogs, request, options: options);
  }

  $grpc.ResponseFuture<$0.ContainerList> listContainers(
    $1.Empty request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listContainers, request, options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamServiceLogs(
    $0.LogTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamServiceLogs, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.LogExport> exportServiceLogs(
    $0.LogTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportServiceLogs, request, options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamComposeLogs(
    $0.ComposeLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamComposeLogs, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.LogExport> exportComposeLogs(
    $0.ComposeLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportComposeLogs, request, options: options);
  }

  $grpc.ResponseStream<$0.DeploymentEvent> streamDeploymentLogs(
    $0.GetDeploymentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamDeploymentLogs, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cancelDeployment(
    $0.CancelDeploymentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelDeployment, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDeployment(
    $0.GetDeploymentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDeployment, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteApplicationDeployments(
    $0.ResourceTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteApplicationDeployments, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteComposeDeployments(
    $0.ResourceTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteComposeDeployments, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDatabaseDeployments(
    $0.ResourceTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDatabaseDeployments, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> clearDeployments(
    $1.Empty request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$clearDeployments, request, options: options);
  }

  // method descriptors

  static final _$listDeployments =
      $grpc.ClientMethod<$0.ListDeploymentsRequest, $0.ListDeploymentsResponse>(
          '/openoxide.api.v1.DeploymentService/ListDeployments',
          ($0.ListDeploymentsRequest value) => value.writeToBuffer(),
          $0.ListDeploymentsResponse.fromBuffer);
  static final _$searchLogs =
      $grpc.ClientMethod<$0.SearchLogsRequest, $0.ListDeploymentsResponse>(
          '/openoxide.api.v1.DeploymentService/SearchLogs',
          ($0.SearchLogsRequest value) => value.writeToBuffer(),
          $0.ListDeploymentsResponse.fromBuffer);
  static final _$listActiveDeployments =
      $grpc.ClientMethod<$1.Empty, $0.ListDeploymentsResponse>(
          '/openoxide.api.v1.DeploymentService/ListActiveDeployments',
          ($1.Empty value) => value.writeToBuffer(),
          $0.ListDeploymentsResponse.fromBuffer);
  static final _$listRunningDeployments =
      $grpc.ClientMethod<$1.Empty, $0.ListDeploymentsResponse>(
          '/openoxide.api.v1.DeploymentService/ListRunningDeployments',
          ($1.Empty value) => value.writeToBuffer(),
          $0.ListDeploymentsResponse.fromBuffer);
  static final _$getDeployment =
      $grpc.ClientMethod<$0.GetDeploymentRequest, $0.Deployment>(
          '/openoxide.api.v1.DeploymentService/GetDeployment',
          ($0.GetDeploymentRequest value) => value.writeToBuffer(),
          $0.Deployment.fromBuffer);
  static final _$streamApplicationStats =
      $grpc.ClientMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamApplicationStats',
          ($0.StreamTargetRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$streamComposeStats =
      $grpc.ClientMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamComposeStats',
          ($0.StreamTargetRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$streamDatabaseStats =
      $grpc.ClientMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamDatabaseStats',
          ($0.StreamTargetRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$streamContainerLogs =
      $grpc.ClientMethod<$0.LogTargetRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamContainerLogs',
          ($0.LogTargetRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$exportContainerLogs =
      $grpc.ClientMethod<$0.LogTargetRequest, $0.LogExport>(
          '/openoxide.api.v1.DeploymentService/ExportContainerLogs',
          ($0.LogTargetRequest value) => value.writeToBuffer(),
          $0.LogExport.fromBuffer);
  static final _$listContainers =
      $grpc.ClientMethod<$1.Empty, $0.ContainerList>(
          '/openoxide.api.v1.DeploymentService/ListContainers',
          ($1.Empty value) => value.writeToBuffer(),
          $0.ContainerList.fromBuffer);
  static final _$streamServiceLogs =
      $grpc.ClientMethod<$0.LogTargetRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamServiceLogs',
          ($0.LogTargetRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$exportServiceLogs =
      $grpc.ClientMethod<$0.LogTargetRequest, $0.LogExport>(
          '/openoxide.api.v1.DeploymentService/ExportServiceLogs',
          ($0.LogTargetRequest value) => value.writeToBuffer(),
          $0.LogExport.fromBuffer);
  static final _$streamComposeLogs =
      $grpc.ClientMethod<$0.ComposeLogsRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamComposeLogs',
          ($0.ComposeLogsRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$exportComposeLogs =
      $grpc.ClientMethod<$0.ComposeLogsRequest, $0.LogExport>(
          '/openoxide.api.v1.DeploymentService/ExportComposeLogs',
          ($0.ComposeLogsRequest value) => value.writeToBuffer(),
          $0.LogExport.fromBuffer);
  static final _$streamDeploymentLogs =
      $grpc.ClientMethod<$0.GetDeploymentRequest, $0.DeploymentEvent>(
          '/openoxide.api.v1.DeploymentService/StreamDeploymentLogs',
          ($0.GetDeploymentRequest value) => value.writeToBuffer(),
          $0.DeploymentEvent.fromBuffer);
  static final _$cancelDeployment =
      $grpc.ClientMethod<$0.CancelDeploymentRequest, $1.Empty>(
          '/openoxide.api.v1.DeploymentService/CancelDeployment',
          ($0.CancelDeploymentRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteDeployment =
      $grpc.ClientMethod<$0.GetDeploymentRequest, $1.Empty>(
          '/openoxide.api.v1.DeploymentService/DeleteDeployment',
          ($0.GetDeploymentRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteApplicationDeployments =
      $grpc.ClientMethod<$0.ResourceTargetRequest, $1.Empty>(
          '/openoxide.api.v1.DeploymentService/DeleteApplicationDeployments',
          ($0.ResourceTargetRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteComposeDeployments =
      $grpc.ClientMethod<$0.ResourceTargetRequest, $1.Empty>(
          '/openoxide.api.v1.DeploymentService/DeleteComposeDeployments',
          ($0.ResourceTargetRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteDatabaseDeployments =
      $grpc.ClientMethod<$0.ResourceTargetRequest, $1.Empty>(
          '/openoxide.api.v1.DeploymentService/DeleteDatabaseDeployments',
          ($0.ResourceTargetRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$clearDeployments = $grpc.ClientMethod<$1.Empty, $1.Empty>(
      '/openoxide.api.v1.DeploymentService/ClearDeployments',
      ($1.Empty value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DeploymentService')
abstract class DeploymentServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DeploymentService';

  DeploymentServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListDeploymentsRequest,
            $0.ListDeploymentsResponse>(
        'ListDeployments',
        listDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListDeploymentsRequest.fromBuffer(value),
        ($0.ListDeploymentsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SearchLogsRequest, $0.ListDeploymentsResponse>(
            'SearchLogs',
            searchLogs_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SearchLogsRequest.fromBuffer(value),
            ($0.ListDeploymentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $0.ListDeploymentsResponse>(
        'ListActiveDeployments',
        listActiveDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($0.ListDeploymentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $0.ListDeploymentsResponse>(
        'ListRunningDeployments',
        listRunningDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($0.ListDeploymentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDeploymentRequest, $0.Deployment>(
        'GetDeployment',
        getDeployment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDeploymentRequest.fromBuffer(value),
        ($0.Deployment value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
        'StreamApplicationStats',
        streamApplicationStats_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.StreamTargetRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
        'StreamComposeStats',
        streamComposeStats_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.StreamTargetRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.StreamTargetRequest, $0.DeploymentEvent>(
        'StreamDatabaseStats',
        streamDatabaseStats_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.StreamTargetRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogTargetRequest, $0.DeploymentEvent>(
        'StreamContainerLogs',
        streamContainerLogs_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.LogTargetRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogTargetRequest, $0.LogExport>(
        'ExportContainerLogs',
        exportContainerLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LogTargetRequest.fromBuffer(value),
        ($0.LogExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $0.ContainerList>(
        'ListContainers',
        listContainers_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($0.ContainerList value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogTargetRequest, $0.DeploymentEvent>(
        'StreamServiceLogs',
        streamServiceLogs_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.LogTargetRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogTargetRequest, $0.LogExport>(
        'ExportServiceLogs',
        exportServiceLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LogTargetRequest.fromBuffer(value),
        ($0.LogExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeLogsRequest, $0.DeploymentEvent>(
        'StreamComposeLogs',
        streamComposeLogs_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.ComposeLogsRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeLogsRequest, $0.LogExport>(
        'ExportComposeLogs',
        exportComposeLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeLogsRequest.fromBuffer(value),
        ($0.LogExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDeploymentRequest, $0.DeploymentEvent>(
        'StreamDeploymentLogs',
        streamDeploymentLogs_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.GetDeploymentRequest.fromBuffer(value),
        ($0.DeploymentEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CancelDeploymentRequest, $1.Empty>(
        'CancelDeployment',
        cancelDeployment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CancelDeploymentRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDeploymentRequest, $1.Empty>(
        'DeleteDeployment',
        deleteDeployment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDeploymentRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ResourceTargetRequest, $1.Empty>(
        'DeleteApplicationDeployments',
        deleteApplicationDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ResourceTargetRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ResourceTargetRequest, $1.Empty>(
        'DeleteComposeDeployments',
        deleteComposeDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ResourceTargetRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ResourceTargetRequest, $1.Empty>(
        'DeleteDatabaseDeployments',
        deleteDatabaseDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ResourceTargetRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $1.Empty>(
        'ClearDeployments',
        clearDeployments_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListDeploymentsResponse> listDeployments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDeploymentsRequest> $request) async {
    return listDeployments($call, await $request);
  }

  $async.Future<$0.ListDeploymentsResponse> listDeployments(
      $grpc.ServiceCall call, $0.ListDeploymentsRequest request);

  $async.Future<$0.ListDeploymentsResponse> searchLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SearchLogsRequest> $request) async {
    return searchLogs($call, await $request);
  }

  $async.Future<$0.ListDeploymentsResponse> searchLogs(
      $grpc.ServiceCall call, $0.SearchLogsRequest request);

  $async.Future<$0.ListDeploymentsResponse> listActiveDeployments_Pre(
      $grpc.ServiceCall $call, $async.Future<$1.Empty> $request) async {
    return listActiveDeployments($call, await $request);
  }

  $async.Future<$0.ListDeploymentsResponse> listActiveDeployments(
      $grpc.ServiceCall call, $1.Empty request);

  $async.Future<$0.ListDeploymentsResponse> listRunningDeployments_Pre(
      $grpc.ServiceCall $call, $async.Future<$1.Empty> $request) async {
    return listRunningDeployments($call, await $request);
  }

  $async.Future<$0.ListDeploymentsResponse> listRunningDeployments(
      $grpc.ServiceCall call, $1.Empty request);

  $async.Future<$0.Deployment> getDeployment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetDeploymentRequest> $request) async {
    return getDeployment($call, await $request);
  }

  $async.Future<$0.Deployment> getDeployment(
      $grpc.ServiceCall call, $0.GetDeploymentRequest request);

  $async.Stream<$0.DeploymentEvent> streamApplicationStats_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.StreamTargetRequest> $request) async* {
    yield* streamApplicationStats($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamApplicationStats(
      $grpc.ServiceCall call, $0.StreamTargetRequest request);

  $async.Stream<$0.DeploymentEvent> streamComposeStats_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.StreamTargetRequest> $request) async* {
    yield* streamComposeStats($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamComposeStats(
      $grpc.ServiceCall call, $0.StreamTargetRequest request);

  $async.Stream<$0.DeploymentEvent> streamDatabaseStats_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.StreamTargetRequest> $request) async* {
    yield* streamDatabaseStats($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamDatabaseStats(
      $grpc.ServiceCall call, $0.StreamTargetRequest request);

  $async.Stream<$0.DeploymentEvent> streamContainerLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.LogTargetRequest> $request) async* {
    yield* streamContainerLogs($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamContainerLogs(
      $grpc.ServiceCall call, $0.LogTargetRequest request);

  $async.Future<$0.LogExport> exportContainerLogs_Pre($grpc.ServiceCall $call,
      $async.Future<$0.LogTargetRequest> $request) async {
    return exportContainerLogs($call, await $request);
  }

  $async.Future<$0.LogExport> exportContainerLogs(
      $grpc.ServiceCall call, $0.LogTargetRequest request);

  $async.Future<$0.ContainerList> listContainers_Pre(
      $grpc.ServiceCall $call, $async.Future<$1.Empty> $request) async {
    return listContainers($call, await $request);
  }

  $async.Future<$0.ContainerList> listContainers(
      $grpc.ServiceCall call, $1.Empty request);

  $async.Stream<$0.DeploymentEvent> streamServiceLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.LogTargetRequest> $request) async* {
    yield* streamServiceLogs($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamServiceLogs(
      $grpc.ServiceCall call, $0.LogTargetRequest request);

  $async.Future<$0.LogExport> exportServiceLogs_Pre($grpc.ServiceCall $call,
      $async.Future<$0.LogTargetRequest> $request) async {
    return exportServiceLogs($call, await $request);
  }

  $async.Future<$0.LogExport> exportServiceLogs(
      $grpc.ServiceCall call, $0.LogTargetRequest request);

  $async.Stream<$0.DeploymentEvent> streamComposeLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeLogsRequest> $request) async* {
    yield* streamComposeLogs($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamComposeLogs(
      $grpc.ServiceCall call, $0.ComposeLogsRequest request);

  $async.Future<$0.LogExport> exportComposeLogs_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeLogsRequest> $request) async {
    return exportComposeLogs($call, await $request);
  }

  $async.Future<$0.LogExport> exportComposeLogs(
      $grpc.ServiceCall call, $0.ComposeLogsRequest request);

  $async.Stream<$0.DeploymentEvent> streamDeploymentLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetDeploymentRequest> $request) async* {
    yield* streamDeploymentLogs($call, await $request);
  }

  $async.Stream<$0.DeploymentEvent> streamDeploymentLogs(
      $grpc.ServiceCall call, $0.GetDeploymentRequest request);

  $async.Future<$1.Empty> cancelDeployment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CancelDeploymentRequest> $request) async {
    return cancelDeployment($call, await $request);
  }

  $async.Future<$1.Empty> cancelDeployment(
      $grpc.ServiceCall call, $0.CancelDeploymentRequest request);

  $async.Future<$1.Empty> deleteDeployment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetDeploymentRequest> $request) async {
    return deleteDeployment($call, await $request);
  }

  $async.Future<$1.Empty> deleteDeployment(
      $grpc.ServiceCall call, $0.GetDeploymentRequest request);

  $async.Future<$1.Empty> deleteApplicationDeployments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceTargetRequest> $request) async {
    return deleteApplicationDeployments($call, await $request);
  }

  $async.Future<$1.Empty> deleteApplicationDeployments(
      $grpc.ServiceCall call, $0.ResourceTargetRequest request);

  $async.Future<$1.Empty> deleteComposeDeployments_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ResourceTargetRequest> $request) async {
    return deleteComposeDeployments($call, await $request);
  }

  $async.Future<$1.Empty> deleteComposeDeployments(
      $grpc.ServiceCall call, $0.ResourceTargetRequest request);

  $async.Future<$1.Empty> deleteDatabaseDeployments_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ResourceTargetRequest> $request) async {
    return deleteDatabaseDeployments($call, await $request);
  }

  $async.Future<$1.Empty> deleteDatabaseDeployments(
      $grpc.ServiceCall call, $0.ResourceTargetRequest request);

  $async.Future<$1.Empty> clearDeployments_Pre(
      $grpc.ServiceCall $call, $async.Future<$1.Empty> $request) async {
    return clearDeployments($call, await $request);
  }

  $async.Future<$1.Empty> clearDeployments(
      $grpc.ServiceCall call, $1.Empty request);
}

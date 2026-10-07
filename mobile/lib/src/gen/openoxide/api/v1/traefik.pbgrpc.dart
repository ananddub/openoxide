// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/traefik.proto.

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
import 'traefik.pb.dart' as $0;

export 'traefik.pb.dart';

/// Sole Traefik owner service; absorbs the former TraefikOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.TraefikService')
class TraefikServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  TraefikServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.TraefikListFilesResponse> listFiles(
    $0.TraefikListFilesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listFiles, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikGetFileTreeResponse> getFileTree(
    $0.TraefikGetFileTreeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFileTree, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikGetFileContentResponse> getFileContent(
    $0.TraefikGetFileContentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFileContent, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> updateFileContent(
    $0.TraefikUpdateFileContentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateFileContent, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikHealthResponse> health(
    $0.TraefikHealthRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$health, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikGetRequestStatusResponse> getRequestStatus(
    $0.TraefikGetRequestStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRequestStatus, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikToggleRequestsResponse> toggleRequests(
    $0.TraefikToggleRequestsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$toggleRequests, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikGetRequestLogsResponse> getRequestLogs(
    $0.TraefikGetRequestLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRequestLogs, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikGetVersionResponse> getVersion(
    $0.TraefikGetVersionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getVersion, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikUpdateVersionResponse> updateVersion(
    $0.TraefikUpdateVersionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateVersion, request, options: options);
  }

  $grpc.ResponseFuture<$0.TraefikRenderMiddlewareResponse> renderMiddleware(
    $0.TraefikRenderMiddlewareRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$renderMiddleware, request, options: options);
  }

  // method descriptors

  static final _$listFiles = $grpc.ClientMethod<$0.TraefikListFilesRequest,
          $0.TraefikListFilesResponse>(
      '/openoxide.api.v1.TraefikService/ListFiles',
      ($0.TraefikListFilesRequest value) => value.writeToBuffer(),
      $0.TraefikListFilesResponse.fromBuffer);
  static final _$getFileTree = $grpc.ClientMethod<$0.TraefikGetFileTreeRequest,
          $0.TraefikGetFileTreeResponse>(
      '/openoxide.api.v1.TraefikService/GetFileTree',
      ($0.TraefikGetFileTreeRequest value) => value.writeToBuffer(),
      $0.TraefikGetFileTreeResponse.fromBuffer);
  static final _$getFileContent = $grpc.ClientMethod<
          $0.TraefikGetFileContentRequest, $0.TraefikGetFileContentResponse>(
      '/openoxide.api.v1.TraefikService/GetFileContent',
      ($0.TraefikGetFileContentRequest value) => value.writeToBuffer(),
      $0.TraefikGetFileContentResponse.fromBuffer);
  static final _$updateFileContent =
      $grpc.ClientMethod<$0.TraefikUpdateFileContentRequest, $1.Empty>(
          '/openoxide.api.v1.TraefikService/UpdateFileContent',
          ($0.TraefikUpdateFileContentRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$health =
      $grpc.ClientMethod<$0.TraefikHealthRequest, $0.TraefikHealthResponse>(
          '/openoxide.api.v1.TraefikService/Health',
          ($0.TraefikHealthRequest value) => value.writeToBuffer(),
          $0.TraefikHealthResponse.fromBuffer);
  static final _$getRequestStatus = $grpc.ClientMethod<
          $0.TraefikGetRequestStatusRequest,
          $0.TraefikGetRequestStatusResponse>(
      '/openoxide.api.v1.TraefikService/GetRequestStatus',
      ($0.TraefikGetRequestStatusRequest value) => value.writeToBuffer(),
      $0.TraefikGetRequestStatusResponse.fromBuffer);
  static final _$toggleRequests = $grpc.ClientMethod<
          $0.TraefikToggleRequestsRequest, $0.TraefikToggleRequestsResponse>(
      '/openoxide.api.v1.TraefikService/ToggleRequests',
      ($0.TraefikToggleRequestsRequest value) => value.writeToBuffer(),
      $0.TraefikToggleRequestsResponse.fromBuffer);
  static final _$getRequestLogs = $grpc.ClientMethod<
          $0.TraefikGetRequestLogsRequest, $0.TraefikGetRequestLogsResponse>(
      '/openoxide.api.v1.TraefikService/GetRequestLogs',
      ($0.TraefikGetRequestLogsRequest value) => value.writeToBuffer(),
      $0.TraefikGetRequestLogsResponse.fromBuffer);
  static final _$getVersion = $grpc.ClientMethod<$0.TraefikGetVersionRequest,
          $0.TraefikGetVersionResponse>(
      '/openoxide.api.v1.TraefikService/GetVersion',
      ($0.TraefikGetVersionRequest value) => value.writeToBuffer(),
      $0.TraefikGetVersionResponse.fromBuffer);
  static final _$updateVersion = $grpc.ClientMethod<
          $0.TraefikUpdateVersionRequest, $0.TraefikUpdateVersionResponse>(
      '/openoxide.api.v1.TraefikService/UpdateVersion',
      ($0.TraefikUpdateVersionRequest value) => value.writeToBuffer(),
      $0.TraefikUpdateVersionResponse.fromBuffer);
  static final _$renderMiddleware = $grpc.ClientMethod<
          $0.TraefikRenderMiddlewareRequest,
          $0.TraefikRenderMiddlewareResponse>(
      '/openoxide.api.v1.TraefikService/RenderMiddleware',
      ($0.TraefikRenderMiddlewareRequest value) => value.writeToBuffer(),
      $0.TraefikRenderMiddlewareResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.TraefikService')
abstract class TraefikServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.TraefikService';

  TraefikServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.TraefikListFilesRequest,
            $0.TraefikListFilesResponse>(
        'ListFiles',
        listFiles_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikListFilesRequest.fromBuffer(value),
        ($0.TraefikListFilesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikGetFileTreeRequest,
            $0.TraefikGetFileTreeResponse>(
        'GetFileTree',
        getFileTree_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikGetFileTreeRequest.fromBuffer(value),
        ($0.TraefikGetFileTreeResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikGetFileContentRequest,
            $0.TraefikGetFileContentResponse>(
        'GetFileContent',
        getFileContent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikGetFileContentRequest.fromBuffer(value),
        ($0.TraefikGetFileContentResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TraefikUpdateFileContentRequest, $1.Empty>(
            'UpdateFileContent',
            updateFileContent_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TraefikUpdateFileContentRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TraefikHealthRequest, $0.TraefikHealthResponse>(
            'Health',
            health_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TraefikHealthRequest.fromBuffer(value),
            ($0.TraefikHealthResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikGetRequestStatusRequest,
            $0.TraefikGetRequestStatusResponse>(
        'GetRequestStatus',
        getRequestStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikGetRequestStatusRequest.fromBuffer(value),
        ($0.TraefikGetRequestStatusResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikToggleRequestsRequest,
            $0.TraefikToggleRequestsResponse>(
        'ToggleRequests',
        toggleRequests_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikToggleRequestsRequest.fromBuffer(value),
        ($0.TraefikToggleRequestsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikGetRequestLogsRequest,
            $0.TraefikGetRequestLogsResponse>(
        'GetRequestLogs',
        getRequestLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikGetRequestLogsRequest.fromBuffer(value),
        ($0.TraefikGetRequestLogsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikGetVersionRequest,
            $0.TraefikGetVersionResponse>(
        'GetVersion',
        getVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikGetVersionRequest.fromBuffer(value),
        ($0.TraefikGetVersionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikUpdateVersionRequest,
            $0.TraefikUpdateVersionResponse>(
        'UpdateVersion',
        updateVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikUpdateVersionRequest.fromBuffer(value),
        ($0.TraefikUpdateVersionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TraefikRenderMiddlewareRequest,
            $0.TraefikRenderMiddlewareResponse>(
        'RenderMiddleware',
        renderMiddleware_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TraefikRenderMiddlewareRequest.fromBuffer(value),
        ($0.TraefikRenderMiddlewareResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.TraefikListFilesResponse> listFiles_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikListFilesRequest> $request) async {
    return listFiles($call, await $request);
  }

  $async.Future<$0.TraefikListFilesResponse> listFiles(
      $grpc.ServiceCall call, $0.TraefikListFilesRequest request);

  $async.Future<$0.TraefikGetFileTreeResponse> getFileTree_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikGetFileTreeRequest> $request) async {
    return getFileTree($call, await $request);
  }

  $async.Future<$0.TraefikGetFileTreeResponse> getFileTree(
      $grpc.ServiceCall call, $0.TraefikGetFileTreeRequest request);

  $async.Future<$0.TraefikGetFileContentResponse> getFileContent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikGetFileContentRequest> $request) async {
    return getFileContent($call, await $request);
  }

  $async.Future<$0.TraefikGetFileContentResponse> getFileContent(
      $grpc.ServiceCall call, $0.TraefikGetFileContentRequest request);

  $async.Future<$1.Empty> updateFileContent_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TraefikUpdateFileContentRequest> $request) async {
    return updateFileContent($call, await $request);
  }

  $async.Future<$1.Empty> updateFileContent(
      $grpc.ServiceCall call, $0.TraefikUpdateFileContentRequest request);

  $async.Future<$0.TraefikHealthResponse> health_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TraefikHealthRequest> $request) async {
    return health($call, await $request);
  }

  $async.Future<$0.TraefikHealthResponse> health(
      $grpc.ServiceCall call, $0.TraefikHealthRequest request);

  $async.Future<$0.TraefikGetRequestStatusResponse> getRequestStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikGetRequestStatusRequest> $request) async {
    return getRequestStatus($call, await $request);
  }

  $async.Future<$0.TraefikGetRequestStatusResponse> getRequestStatus(
      $grpc.ServiceCall call, $0.TraefikGetRequestStatusRequest request);

  $async.Future<$0.TraefikToggleRequestsResponse> toggleRequests_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikToggleRequestsRequest> $request) async {
    return toggleRequests($call, await $request);
  }

  $async.Future<$0.TraefikToggleRequestsResponse> toggleRequests(
      $grpc.ServiceCall call, $0.TraefikToggleRequestsRequest request);

  $async.Future<$0.TraefikGetRequestLogsResponse> getRequestLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikGetRequestLogsRequest> $request) async {
    return getRequestLogs($call, await $request);
  }

  $async.Future<$0.TraefikGetRequestLogsResponse> getRequestLogs(
      $grpc.ServiceCall call, $0.TraefikGetRequestLogsRequest request);

  $async.Future<$0.TraefikGetVersionResponse> getVersion_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikGetVersionRequest> $request) async {
    return getVersion($call, await $request);
  }

  $async.Future<$0.TraefikGetVersionResponse> getVersion(
      $grpc.ServiceCall call, $0.TraefikGetVersionRequest request);

  $async.Future<$0.TraefikUpdateVersionResponse> updateVersion_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikUpdateVersionRequest> $request) async {
    return updateVersion($call, await $request);
  }

  $async.Future<$0.TraefikUpdateVersionResponse> updateVersion(
      $grpc.ServiceCall call, $0.TraefikUpdateVersionRequest request);

  $async.Future<$0.TraefikRenderMiddlewareResponse> renderMiddleware_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TraefikRenderMiddlewareRequest> $request) async {
    return renderMiddleware($call, await $request);
  }

  $async.Future<$0.TraefikRenderMiddlewareResponse> renderMiddleware(
      $grpc.ServiceCall call, $0.TraefikRenderMiddlewareRequest request);
}

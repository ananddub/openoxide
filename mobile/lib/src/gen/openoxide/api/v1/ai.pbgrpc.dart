// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ai.proto.

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

import 'ai.pb.dart' as $0;
import 'common.pb.dart' as $1;

export 'ai.pb.dart';

/// AI provider settings, model discovery, compose generation and log analysis.
@$pb.GrpcServiceName('openoxide.api.v1.AiService')
class AiServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AiServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.AiListSettingsResponse> listSettings(
    $0.AiListSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiGetSettingsResponse> getSettings(
    $0.AiGetSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiCreateSettingsResponse> createSettings(
    $0.AiCreateSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiUpdateSettingsResponse> updateSettings(
    $0.AiUpdateSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSettings, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteSettings(
    $0.AiDeleteSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiListModelsResponse> listModels(
    $0.AiListModelsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listModels, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiTestSettingsResponse> testSettings(
    $0.AiTestSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiDiscoverModelsResponse> discoverModels(
    $0.AiDiscoverModelsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$discoverModels, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiTestConnectionResponse> testConnection(
    $0.AiTestConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiListGenerationsResponse> listGenerations(
    $0.AiListGenerationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listGenerations, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiGetGenerationResponse> getGeneration(
    $0.AiGetGenerationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getGeneration, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiCreateGenerationResponse> createGeneration(
    $0.AiCreateGenerationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createGeneration, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiReviewGenerationResponse> reviewGeneration(
    $0.AiReviewGenerationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reviewGeneration, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiDeployGenerationResponse> deployGeneration(
    $0.AiDeployGenerationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deployGeneration, request, options: options);
  }

  $grpc.ResponseFuture<$0.AiAnalyzeLogsResponse> analyzeLogs(
    $0.AiAnalyzeLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$analyzeLogs, request, options: options);
  }

  /// Agentic assistant: natural-language request → permission-checked actions.
  $grpc.ResponseFuture<$0.AiAssistantResponse> assistant(
    $0.AiAssistantRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$assistant, request, options: options);
  }

  // method descriptors

  static final _$listSettings =
      $grpc.ClientMethod<$0.AiListSettingsRequest, $0.AiListSettingsResponse>(
          '/openoxide.api.v1.AiService/ListSettings',
          ($0.AiListSettingsRequest value) => value.writeToBuffer(),
          $0.AiListSettingsResponse.fromBuffer);
  static final _$getSettings =
      $grpc.ClientMethod<$0.AiGetSettingsRequest, $0.AiGetSettingsResponse>(
          '/openoxide.api.v1.AiService/GetSettings',
          ($0.AiGetSettingsRequest value) => value.writeToBuffer(),
          $0.AiGetSettingsResponse.fromBuffer);
  static final _$createSettings = $grpc.ClientMethod<$0.AiCreateSettingsRequest,
          $0.AiCreateSettingsResponse>(
      '/openoxide.api.v1.AiService/CreateSettings',
      ($0.AiCreateSettingsRequest value) => value.writeToBuffer(),
      $0.AiCreateSettingsResponse.fromBuffer);
  static final _$updateSettings = $grpc.ClientMethod<$0.AiUpdateSettingsRequest,
          $0.AiUpdateSettingsResponse>(
      '/openoxide.api.v1.AiService/UpdateSettings',
      ($0.AiUpdateSettingsRequest value) => value.writeToBuffer(),
      $0.AiUpdateSettingsResponse.fromBuffer);
  static final _$deleteSettings =
      $grpc.ClientMethod<$0.AiDeleteSettingsRequest, $1.Empty>(
          '/openoxide.api.v1.AiService/DeleteSettings',
          ($0.AiDeleteSettingsRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listModels =
      $grpc.ClientMethod<$0.AiListModelsRequest, $0.AiListModelsResponse>(
          '/openoxide.api.v1.AiService/ListModels',
          ($0.AiListModelsRequest value) => value.writeToBuffer(),
          $0.AiListModelsResponse.fromBuffer);
  static final _$testSettings =
      $grpc.ClientMethod<$0.AiTestSettingsRequest, $0.AiTestSettingsResponse>(
          '/openoxide.api.v1.AiService/TestSettings',
          ($0.AiTestSettingsRequest value) => value.writeToBuffer(),
          $0.AiTestSettingsResponse.fromBuffer);
  static final _$discoverModels = $grpc.ClientMethod<$0.AiDiscoverModelsRequest,
          $0.AiDiscoverModelsResponse>(
      '/openoxide.api.v1.AiService/DiscoverModels',
      ($0.AiDiscoverModelsRequest value) => value.writeToBuffer(),
      $0.AiDiscoverModelsResponse.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<$0.AiTestConnectionRequest,
          $0.AiTestConnectionResponse>(
      '/openoxide.api.v1.AiService/TestConnection',
      ($0.AiTestConnectionRequest value) => value.writeToBuffer(),
      $0.AiTestConnectionResponse.fromBuffer);
  static final _$listGenerations = $grpc.ClientMethod<
          $0.AiListGenerationsRequest, $0.AiListGenerationsResponse>(
      '/openoxide.api.v1.AiService/ListGenerations',
      ($0.AiListGenerationsRequest value) => value.writeToBuffer(),
      $0.AiListGenerationsResponse.fromBuffer);
  static final _$getGeneration =
      $grpc.ClientMethod<$0.AiGetGenerationRequest, $0.AiGetGenerationResponse>(
          '/openoxide.api.v1.AiService/GetGeneration',
          ($0.AiGetGenerationRequest value) => value.writeToBuffer(),
          $0.AiGetGenerationResponse.fromBuffer);
  static final _$createGeneration = $grpc.ClientMethod<
          $0.AiCreateGenerationRequest, $0.AiCreateGenerationResponse>(
      '/openoxide.api.v1.AiService/CreateGeneration',
      ($0.AiCreateGenerationRequest value) => value.writeToBuffer(),
      $0.AiCreateGenerationResponse.fromBuffer);
  static final _$reviewGeneration = $grpc.ClientMethod<
          $0.AiReviewGenerationRequest, $0.AiReviewGenerationResponse>(
      '/openoxide.api.v1.AiService/ReviewGeneration',
      ($0.AiReviewGenerationRequest value) => value.writeToBuffer(),
      $0.AiReviewGenerationResponse.fromBuffer);
  static final _$deployGeneration = $grpc.ClientMethod<
          $0.AiDeployGenerationRequest, $0.AiDeployGenerationResponse>(
      '/openoxide.api.v1.AiService/DeployGeneration',
      ($0.AiDeployGenerationRequest value) => value.writeToBuffer(),
      $0.AiDeployGenerationResponse.fromBuffer);
  static final _$analyzeLogs =
      $grpc.ClientMethod<$0.AiAnalyzeLogsRequest, $0.AiAnalyzeLogsResponse>(
          '/openoxide.api.v1.AiService/AnalyzeLogs',
          ($0.AiAnalyzeLogsRequest value) => value.writeToBuffer(),
          $0.AiAnalyzeLogsResponse.fromBuffer);
  static final _$assistant =
      $grpc.ClientMethod<$0.AiAssistantRequest, $0.AiAssistantResponse>(
          '/openoxide.api.v1.AiService/Assistant',
          ($0.AiAssistantRequest value) => value.writeToBuffer(),
          $0.AiAssistantResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.AiService')
abstract class AiServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.AiService';

  AiServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.AiListSettingsRequest,
            $0.AiListSettingsResponse>(
        'ListSettings',
        listSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiListSettingsRequest.fromBuffer(value),
        ($0.AiListSettingsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.AiGetSettingsRequest, $0.AiGetSettingsResponse>(
            'GetSettings',
            getSettings_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AiGetSettingsRequest.fromBuffer(value),
            ($0.AiGetSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiCreateSettingsRequest,
            $0.AiCreateSettingsResponse>(
        'CreateSettings',
        createSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiCreateSettingsRequest.fromBuffer(value),
        ($0.AiCreateSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiUpdateSettingsRequest,
            $0.AiUpdateSettingsResponse>(
        'UpdateSettings',
        updateSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiUpdateSettingsRequest.fromBuffer(value),
        ($0.AiUpdateSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiDeleteSettingsRequest, $1.Empty>(
        'DeleteSettings',
        deleteSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiDeleteSettingsRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.AiListModelsRequest, $0.AiListModelsResponse>(
            'ListModels',
            listModels_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AiListModelsRequest.fromBuffer(value),
            ($0.AiListModelsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiTestSettingsRequest,
            $0.AiTestSettingsResponse>(
        'TestSettings',
        testSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiTestSettingsRequest.fromBuffer(value),
        ($0.AiTestSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiDiscoverModelsRequest,
            $0.AiDiscoverModelsResponse>(
        'DiscoverModels',
        discoverModels_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiDiscoverModelsRequest.fromBuffer(value),
        ($0.AiDiscoverModelsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiTestConnectionRequest,
            $0.AiTestConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiTestConnectionRequest.fromBuffer(value),
        ($0.AiTestConnectionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiListGenerationsRequest,
            $0.AiListGenerationsResponse>(
        'ListGenerations',
        listGenerations_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiListGenerationsRequest.fromBuffer(value),
        ($0.AiListGenerationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiGetGenerationRequest,
            $0.AiGetGenerationResponse>(
        'GetGeneration',
        getGeneration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiGetGenerationRequest.fromBuffer(value),
        ($0.AiGetGenerationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiCreateGenerationRequest,
            $0.AiCreateGenerationResponse>(
        'CreateGeneration',
        createGeneration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiCreateGenerationRequest.fromBuffer(value),
        ($0.AiCreateGenerationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiReviewGenerationRequest,
            $0.AiReviewGenerationResponse>(
        'ReviewGeneration',
        reviewGeneration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiReviewGenerationRequest.fromBuffer(value),
        ($0.AiReviewGenerationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AiDeployGenerationRequest,
            $0.AiDeployGenerationResponse>(
        'DeployGeneration',
        deployGeneration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AiDeployGenerationRequest.fromBuffer(value),
        ($0.AiDeployGenerationResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.AiAnalyzeLogsRequest, $0.AiAnalyzeLogsResponse>(
            'AnalyzeLogs',
            analyzeLogs_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AiAnalyzeLogsRequest.fromBuffer(value),
            ($0.AiAnalyzeLogsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.AiAssistantRequest, $0.AiAssistantResponse>(
            'Assistant',
            assistant_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AiAssistantRequest.fromBuffer(value),
            ($0.AiAssistantResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.AiListSettingsResponse> listSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiListSettingsRequest> $request) async {
    return listSettings($call, await $request);
  }

  $async.Future<$0.AiListSettingsResponse> listSettings(
      $grpc.ServiceCall call, $0.AiListSettingsRequest request);

  $async.Future<$0.AiGetSettingsResponse> getSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiGetSettingsRequest> $request) async {
    return getSettings($call, await $request);
  }

  $async.Future<$0.AiGetSettingsResponse> getSettings(
      $grpc.ServiceCall call, $0.AiGetSettingsRequest request);

  $async.Future<$0.AiCreateSettingsResponse> createSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiCreateSettingsRequest> $request) async {
    return createSettings($call, await $request);
  }

  $async.Future<$0.AiCreateSettingsResponse> createSettings(
      $grpc.ServiceCall call, $0.AiCreateSettingsRequest request);

  $async.Future<$0.AiUpdateSettingsResponse> updateSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiUpdateSettingsRequest> $request) async {
    return updateSettings($call, await $request);
  }

  $async.Future<$0.AiUpdateSettingsResponse> updateSettings(
      $grpc.ServiceCall call, $0.AiUpdateSettingsRequest request);

  $async.Future<$1.Empty> deleteSettings_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AiDeleteSettingsRequest> $request) async {
    return deleteSettings($call, await $request);
  }

  $async.Future<$1.Empty> deleteSettings(
      $grpc.ServiceCall call, $0.AiDeleteSettingsRequest request);

  $async.Future<$0.AiListModelsResponse> listModels_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AiListModelsRequest> $request) async {
    return listModels($call, await $request);
  }

  $async.Future<$0.AiListModelsResponse> listModels(
      $grpc.ServiceCall call, $0.AiListModelsRequest request);

  $async.Future<$0.AiTestSettingsResponse> testSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiTestSettingsRequest> $request) async {
    return testSettings($call, await $request);
  }

  $async.Future<$0.AiTestSettingsResponse> testSettings(
      $grpc.ServiceCall call, $0.AiTestSettingsRequest request);

  $async.Future<$0.AiDiscoverModelsResponse> discoverModels_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiDiscoverModelsRequest> $request) async {
    return discoverModels($call, await $request);
  }

  $async.Future<$0.AiDiscoverModelsResponse> discoverModels(
      $grpc.ServiceCall call, $0.AiDiscoverModelsRequest request);

  $async.Future<$0.AiTestConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiTestConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.AiTestConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.AiTestConnectionRequest request);

  $async.Future<$0.AiListGenerationsResponse> listGenerations_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiListGenerationsRequest> $request) async {
    return listGenerations($call, await $request);
  }

  $async.Future<$0.AiListGenerationsResponse> listGenerations(
      $grpc.ServiceCall call, $0.AiListGenerationsRequest request);

  $async.Future<$0.AiGetGenerationResponse> getGeneration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiGetGenerationRequest> $request) async {
    return getGeneration($call, await $request);
  }

  $async.Future<$0.AiGetGenerationResponse> getGeneration(
      $grpc.ServiceCall call, $0.AiGetGenerationRequest request);

  $async.Future<$0.AiCreateGenerationResponse> createGeneration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiCreateGenerationRequest> $request) async {
    return createGeneration($call, await $request);
  }

  $async.Future<$0.AiCreateGenerationResponse> createGeneration(
      $grpc.ServiceCall call, $0.AiCreateGenerationRequest request);

  $async.Future<$0.AiReviewGenerationResponse> reviewGeneration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiReviewGenerationRequest> $request) async {
    return reviewGeneration($call, await $request);
  }

  $async.Future<$0.AiReviewGenerationResponse> reviewGeneration(
      $grpc.ServiceCall call, $0.AiReviewGenerationRequest request);

  $async.Future<$0.AiDeployGenerationResponse> deployGeneration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiDeployGenerationRequest> $request) async {
    return deployGeneration($call, await $request);
  }

  $async.Future<$0.AiDeployGenerationResponse> deployGeneration(
      $grpc.ServiceCall call, $0.AiDeployGenerationRequest request);

  $async.Future<$0.AiAnalyzeLogsResponse> analyzeLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AiAnalyzeLogsRequest> $request) async {
    return analyzeLogs($call, await $request);
  }

  $async.Future<$0.AiAnalyzeLogsResponse> analyzeLogs(
      $grpc.ServiceCall call, $0.AiAnalyzeLogsRequest request);

  $async.Future<$0.AiAssistantResponse> assistant_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AiAssistantRequest> $request) async {
    return assistant($call, await $request);
  }

  $async.Future<$0.AiAssistantResponse> assistant(
      $grpc.ServiceCall call, $0.AiAssistantRequest request);
}

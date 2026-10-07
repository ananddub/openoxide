// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/git_provider.proto.

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
import 'git_provider.pb.dart' as $0;

export 'git_provider.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.GitProviderService')
class GitProviderServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  GitProviderServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListGitProvidersResponse> listGitProviders(
    $0.ListGitProvidersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listGitProviders, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetGitProviderResponse> getGitProvider(
    $0.GetGitProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getGitProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateGithubProviderResponse> createGithubProvider(
    $0.CreateGithubProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createGithubProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateGitlabProviderResponse> createGitlabProvider(
    $0.CreateGitlabProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createGitlabProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateGiteaProviderResponse> createGiteaProvider(
    $0.CreateGiteaProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createGiteaProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateBitbucketProviderResponse>
      createBitbucketProvider(
    $0.CreateBitbucketProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createBitbucketProvider, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.UpdateGithubProviderResponse> updateGithubProvider(
    $0.UpdateGithubProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateGithubProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateGitlabProviderResponse> updateGitlabProvider(
    $0.UpdateGitlabProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateGitlabProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateGiteaProviderResponse> updateGiteaProvider(
    $0.UpdateGiteaProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateGiteaProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateBitbucketProviderResponse>
      updateBitbucketProvider(
    $0.UpdateBitbucketProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateBitbucketProvider, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteGitProvider(
    $0.DeleteGitProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteGitProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestGitProviderResponse> testGitProvider(
    $0.TestGitProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testGitProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitRotateWebhookSecretResponse> rotateWebhookSecret(
    $0.GitRotateWebhookSecretRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateWebhookSecret, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitListRepositoriesResponse> listRepositories(
    $0.GitListRepositoriesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRepositories, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitListRepositoryBranchesResponse> listBranches(
    $0.GitListRepositoryBranchesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listBranches, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitListTagsResponse> listTags(
    $0.GitListTagsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listTags, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitAuthorizeResponse> authorize(
    $0.GitAuthorizeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$authorize, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitOAuthCallbackResponse> oAuthCallback(
    $0.GitOAuthCallbackRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$oAuthCallback, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> disconnect(
    $0.GitDisconnectRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$disconnect, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitGetCollaboratorPermissionResponse>
      getCollaboratorPermission(
    $0.GitGetCollaboratorPermissionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getCollaboratorPermission, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.GitGetWebhookStatusResponse> getWebhookStatus(
    $0.GitGetWebhookStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWebhookStatus, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitInstallWebhookResponse> installWebhook(
    $0.GitInstallWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$installWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$0.GitRecreateWebhookResponse> recreateWebhook(
    $0.GitRecreateWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$recreateWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteWebhook(
    $0.GitDeleteWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteWebhook, request, options: options);
  }

  // method descriptors

  static final _$listGitProviders = $grpc.ClientMethod<
          $0.ListGitProvidersRequest, $0.ListGitProvidersResponse>(
      '/openoxide.api.v1.GitProviderService/ListGitProviders',
      ($0.ListGitProvidersRequest value) => value.writeToBuffer(),
      $0.ListGitProvidersResponse.fromBuffer);
  static final _$getGitProvider =
      $grpc.ClientMethod<$0.GetGitProviderRequest, $0.GetGitProviderResponse>(
          '/openoxide.api.v1.GitProviderService/GetGitProvider',
          ($0.GetGitProviderRequest value) => value.writeToBuffer(),
          $0.GetGitProviderResponse.fromBuffer);
  static final _$createGithubProvider = $grpc.ClientMethod<
          $0.CreateGithubProviderRequest, $0.CreateGithubProviderResponse>(
      '/openoxide.api.v1.GitProviderService/CreateGithubProvider',
      ($0.CreateGithubProviderRequest value) => value.writeToBuffer(),
      $0.CreateGithubProviderResponse.fromBuffer);
  static final _$createGitlabProvider = $grpc.ClientMethod<
          $0.CreateGitlabProviderRequest, $0.CreateGitlabProviderResponse>(
      '/openoxide.api.v1.GitProviderService/CreateGitlabProvider',
      ($0.CreateGitlabProviderRequest value) => value.writeToBuffer(),
      $0.CreateGitlabProviderResponse.fromBuffer);
  static final _$createGiteaProvider = $grpc.ClientMethod<
          $0.CreateGiteaProviderRequest, $0.CreateGiteaProviderResponse>(
      '/openoxide.api.v1.GitProviderService/CreateGiteaProvider',
      ($0.CreateGiteaProviderRequest value) => value.writeToBuffer(),
      $0.CreateGiteaProviderResponse.fromBuffer);
  static final _$createBitbucketProvider = $grpc.ClientMethod<
          $0.CreateBitbucketProviderRequest,
          $0.CreateBitbucketProviderResponse>(
      '/openoxide.api.v1.GitProviderService/CreateBitbucketProvider',
      ($0.CreateBitbucketProviderRequest value) => value.writeToBuffer(),
      $0.CreateBitbucketProviderResponse.fromBuffer);
  static final _$updateGithubProvider = $grpc.ClientMethod<
          $0.UpdateGithubProviderRequest, $0.UpdateGithubProviderResponse>(
      '/openoxide.api.v1.GitProviderService/UpdateGithubProvider',
      ($0.UpdateGithubProviderRequest value) => value.writeToBuffer(),
      $0.UpdateGithubProviderResponse.fromBuffer);
  static final _$updateGitlabProvider = $grpc.ClientMethod<
          $0.UpdateGitlabProviderRequest, $0.UpdateGitlabProviderResponse>(
      '/openoxide.api.v1.GitProviderService/UpdateGitlabProvider',
      ($0.UpdateGitlabProviderRequest value) => value.writeToBuffer(),
      $0.UpdateGitlabProviderResponse.fromBuffer);
  static final _$updateGiteaProvider = $grpc.ClientMethod<
          $0.UpdateGiteaProviderRequest, $0.UpdateGiteaProviderResponse>(
      '/openoxide.api.v1.GitProviderService/UpdateGiteaProvider',
      ($0.UpdateGiteaProviderRequest value) => value.writeToBuffer(),
      $0.UpdateGiteaProviderResponse.fromBuffer);
  static final _$updateBitbucketProvider = $grpc.ClientMethod<
          $0.UpdateBitbucketProviderRequest,
          $0.UpdateBitbucketProviderResponse>(
      '/openoxide.api.v1.GitProviderService/UpdateBitbucketProvider',
      ($0.UpdateBitbucketProviderRequest value) => value.writeToBuffer(),
      $0.UpdateBitbucketProviderResponse.fromBuffer);
  static final _$deleteGitProvider =
      $grpc.ClientMethod<$0.DeleteGitProviderRequest, $1.Empty>(
          '/openoxide.api.v1.GitProviderService/DeleteGitProvider',
          ($0.DeleteGitProviderRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testGitProvider =
      $grpc.ClientMethod<$0.TestGitProviderRequest, $0.TestGitProviderResponse>(
          '/openoxide.api.v1.GitProviderService/TestGitProvider',
          ($0.TestGitProviderRequest value) => value.writeToBuffer(),
          $0.TestGitProviderResponse.fromBuffer);
  static final _$rotateWebhookSecret = $grpc.ClientMethod<
          $0.GitRotateWebhookSecretRequest, $0.GitRotateWebhookSecretResponse>(
      '/openoxide.api.v1.GitProviderService/RotateWebhookSecret',
      ($0.GitRotateWebhookSecretRequest value) => value.writeToBuffer(),
      $0.GitRotateWebhookSecretResponse.fromBuffer);
  static final _$listRepositories = $grpc.ClientMethod<
          $0.GitListRepositoriesRequest, $0.GitListRepositoriesResponse>(
      '/openoxide.api.v1.GitProviderService/ListRepositories',
      ($0.GitListRepositoriesRequest value) => value.writeToBuffer(),
      $0.GitListRepositoriesResponse.fromBuffer);
  static final _$listBranches = $grpc.ClientMethod<
          $0.GitListRepositoryBranchesRequest,
          $0.GitListRepositoryBranchesResponse>(
      '/openoxide.api.v1.GitProviderService/ListBranches',
      ($0.GitListRepositoryBranchesRequest value) => value.writeToBuffer(),
      $0.GitListRepositoryBranchesResponse.fromBuffer);
  static final _$listTags =
      $grpc.ClientMethod<$0.GitListTagsRequest, $0.GitListTagsResponse>(
          '/openoxide.api.v1.GitProviderService/ListTags',
          ($0.GitListTagsRequest value) => value.writeToBuffer(),
          $0.GitListTagsResponse.fromBuffer);
  static final _$authorize =
      $grpc.ClientMethod<$0.GitAuthorizeRequest, $0.GitAuthorizeResponse>(
          '/openoxide.api.v1.GitProviderService/Authorize',
          ($0.GitAuthorizeRequest value) => value.writeToBuffer(),
          $0.GitAuthorizeResponse.fromBuffer);
  static final _$oAuthCallback = $grpc.ClientMethod<$0.GitOAuthCallbackRequest,
          $0.GitOAuthCallbackResponse>(
      '/openoxide.api.v1.GitProviderService/OAuthCallback',
      ($0.GitOAuthCallbackRequest value) => value.writeToBuffer(),
      $0.GitOAuthCallbackResponse.fromBuffer);
  static final _$disconnect =
      $grpc.ClientMethod<$0.GitDisconnectRequest, $1.Empty>(
          '/openoxide.api.v1.GitProviderService/Disconnect',
          ($0.GitDisconnectRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getCollaboratorPermission = $grpc.ClientMethod<
          $0.GitGetCollaboratorPermissionRequest,
          $0.GitGetCollaboratorPermissionResponse>(
      '/openoxide.api.v1.GitProviderService/GetCollaboratorPermission',
      ($0.GitGetCollaboratorPermissionRequest value) => value.writeToBuffer(),
      $0.GitGetCollaboratorPermissionResponse.fromBuffer);
  static final _$getWebhookStatus = $grpc.ClientMethod<
          $0.GitGetWebhookStatusRequest, $0.GitGetWebhookStatusResponse>(
      '/openoxide.api.v1.GitProviderService/GetWebhookStatus',
      ($0.GitGetWebhookStatusRequest value) => value.writeToBuffer(),
      $0.GitGetWebhookStatusResponse.fromBuffer);
  static final _$installWebhook = $grpc.ClientMethod<
          $0.GitInstallWebhookRequest, $0.GitInstallWebhookResponse>(
      '/openoxide.api.v1.GitProviderService/InstallWebhook',
      ($0.GitInstallWebhookRequest value) => value.writeToBuffer(),
      $0.GitInstallWebhookResponse.fromBuffer);
  static final _$recreateWebhook = $grpc.ClientMethod<
          $0.GitRecreateWebhookRequest, $0.GitRecreateWebhookResponse>(
      '/openoxide.api.v1.GitProviderService/RecreateWebhook',
      ($0.GitRecreateWebhookRequest value) => value.writeToBuffer(),
      $0.GitRecreateWebhookResponse.fromBuffer);
  static final _$deleteWebhook =
      $grpc.ClientMethod<$0.GitDeleteWebhookRequest, $1.Empty>(
          '/openoxide.api.v1.GitProviderService/DeleteWebhook',
          ($0.GitDeleteWebhookRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.GitProviderService')
abstract class GitProviderServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.GitProviderService';

  GitProviderServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListGitProvidersRequest,
            $0.ListGitProvidersResponse>(
        'ListGitProviders',
        listGitProviders_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListGitProvidersRequest.fromBuffer(value),
        ($0.ListGitProvidersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetGitProviderRequest,
            $0.GetGitProviderResponse>(
        'GetGitProvider',
        getGitProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetGitProviderRequest.fromBuffer(value),
        ($0.GetGitProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateGithubProviderRequest,
            $0.CreateGithubProviderResponse>(
        'CreateGithubProvider',
        createGithubProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateGithubProviderRequest.fromBuffer(value),
        ($0.CreateGithubProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateGitlabProviderRequest,
            $0.CreateGitlabProviderResponse>(
        'CreateGitlabProvider',
        createGitlabProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateGitlabProviderRequest.fromBuffer(value),
        ($0.CreateGitlabProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateGiteaProviderRequest,
            $0.CreateGiteaProviderResponse>(
        'CreateGiteaProvider',
        createGiteaProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateGiteaProviderRequest.fromBuffer(value),
        ($0.CreateGiteaProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateBitbucketProviderRequest,
            $0.CreateBitbucketProviderResponse>(
        'CreateBitbucketProvider',
        createBitbucketProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateBitbucketProviderRequest.fromBuffer(value),
        ($0.CreateBitbucketProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateGithubProviderRequest,
            $0.UpdateGithubProviderResponse>(
        'UpdateGithubProvider',
        updateGithubProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateGithubProviderRequest.fromBuffer(value),
        ($0.UpdateGithubProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateGitlabProviderRequest,
            $0.UpdateGitlabProviderResponse>(
        'UpdateGitlabProvider',
        updateGitlabProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateGitlabProviderRequest.fromBuffer(value),
        ($0.UpdateGitlabProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateGiteaProviderRequest,
            $0.UpdateGiteaProviderResponse>(
        'UpdateGiteaProvider',
        updateGiteaProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateGiteaProviderRequest.fromBuffer(value),
        ($0.UpdateGiteaProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateBitbucketProviderRequest,
            $0.UpdateBitbucketProviderResponse>(
        'UpdateBitbucketProvider',
        updateBitbucketProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateBitbucketProviderRequest.fromBuffer(value),
        ($0.UpdateBitbucketProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteGitProviderRequest, $1.Empty>(
        'DeleteGitProvider',
        deleteGitProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteGitProviderRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestGitProviderRequest,
            $0.TestGitProviderResponse>(
        'TestGitProvider',
        testGitProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestGitProviderRequest.fromBuffer(value),
        ($0.TestGitProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitRotateWebhookSecretRequest,
            $0.GitRotateWebhookSecretResponse>(
        'RotateWebhookSecret',
        rotateWebhookSecret_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitRotateWebhookSecretRequest.fromBuffer(value),
        ($0.GitRotateWebhookSecretResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitListRepositoriesRequest,
            $0.GitListRepositoriesResponse>(
        'ListRepositories',
        listRepositories_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitListRepositoriesRequest.fromBuffer(value),
        ($0.GitListRepositoriesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitListRepositoryBranchesRequest,
            $0.GitListRepositoryBranchesResponse>(
        'ListBranches',
        listBranches_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitListRepositoryBranchesRequest.fromBuffer(value),
        ($0.GitListRepositoryBranchesResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GitListTagsRequest, $0.GitListTagsResponse>(
            'ListTags',
            listTags_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GitListTagsRequest.fromBuffer(value),
            ($0.GitListTagsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GitAuthorizeRequest, $0.GitAuthorizeResponse>(
            'Authorize',
            authorize_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GitAuthorizeRequest.fromBuffer(value),
            ($0.GitAuthorizeResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitOAuthCallbackRequest,
            $0.GitOAuthCallbackResponse>(
        'OAuthCallback',
        oAuthCallback_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitOAuthCallbackRequest.fromBuffer(value),
        ($0.GitOAuthCallbackResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitDisconnectRequest, $1.Empty>(
        'Disconnect',
        disconnect_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitDisconnectRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitGetCollaboratorPermissionRequest,
            $0.GitGetCollaboratorPermissionResponse>(
        'GetCollaboratorPermission',
        getCollaboratorPermission_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitGetCollaboratorPermissionRequest.fromBuffer(value),
        ($0.GitGetCollaboratorPermissionResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitGetWebhookStatusRequest,
            $0.GitGetWebhookStatusResponse>(
        'GetWebhookStatus',
        getWebhookStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitGetWebhookStatusRequest.fromBuffer(value),
        ($0.GitGetWebhookStatusResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitInstallWebhookRequest,
            $0.GitInstallWebhookResponse>(
        'InstallWebhook',
        installWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitInstallWebhookRequest.fromBuffer(value),
        ($0.GitInstallWebhookResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitRecreateWebhookRequest,
            $0.GitRecreateWebhookResponse>(
        'RecreateWebhook',
        recreateWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitRecreateWebhookRequest.fromBuffer(value),
        ($0.GitRecreateWebhookResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GitDeleteWebhookRequest, $1.Empty>(
        'DeleteWebhook',
        deleteWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GitDeleteWebhookRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListGitProvidersResponse> listGitProviders_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListGitProvidersRequest> $request) async {
    return listGitProviders($call, await $request);
  }

  $async.Future<$0.ListGitProvidersResponse> listGitProviders(
      $grpc.ServiceCall call, $0.ListGitProvidersRequest request);

  $async.Future<$0.GetGitProviderResponse> getGitProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetGitProviderRequest> $request) async {
    return getGitProvider($call, await $request);
  }

  $async.Future<$0.GetGitProviderResponse> getGitProvider(
      $grpc.ServiceCall call, $0.GetGitProviderRequest request);

  $async.Future<$0.CreateGithubProviderResponse> createGithubProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateGithubProviderRequest> $request) async {
    return createGithubProvider($call, await $request);
  }

  $async.Future<$0.CreateGithubProviderResponse> createGithubProvider(
      $grpc.ServiceCall call, $0.CreateGithubProviderRequest request);

  $async.Future<$0.CreateGitlabProviderResponse> createGitlabProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateGitlabProviderRequest> $request) async {
    return createGitlabProvider($call, await $request);
  }

  $async.Future<$0.CreateGitlabProviderResponse> createGitlabProvider(
      $grpc.ServiceCall call, $0.CreateGitlabProviderRequest request);

  $async.Future<$0.CreateGiteaProviderResponse> createGiteaProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateGiteaProviderRequest> $request) async {
    return createGiteaProvider($call, await $request);
  }

  $async.Future<$0.CreateGiteaProviderResponse> createGiteaProvider(
      $grpc.ServiceCall call, $0.CreateGiteaProviderRequest request);

  $async.Future<$0.CreateBitbucketProviderResponse> createBitbucketProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateBitbucketProviderRequest> $request) async {
    return createBitbucketProvider($call, await $request);
  }

  $async.Future<$0.CreateBitbucketProviderResponse> createBitbucketProvider(
      $grpc.ServiceCall call, $0.CreateBitbucketProviderRequest request);

  $async.Future<$0.UpdateGithubProviderResponse> updateGithubProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateGithubProviderRequest> $request) async {
    return updateGithubProvider($call, await $request);
  }

  $async.Future<$0.UpdateGithubProviderResponse> updateGithubProvider(
      $grpc.ServiceCall call, $0.UpdateGithubProviderRequest request);

  $async.Future<$0.UpdateGitlabProviderResponse> updateGitlabProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateGitlabProviderRequest> $request) async {
    return updateGitlabProvider($call, await $request);
  }

  $async.Future<$0.UpdateGitlabProviderResponse> updateGitlabProvider(
      $grpc.ServiceCall call, $0.UpdateGitlabProviderRequest request);

  $async.Future<$0.UpdateGiteaProviderResponse> updateGiteaProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateGiteaProviderRequest> $request) async {
    return updateGiteaProvider($call, await $request);
  }

  $async.Future<$0.UpdateGiteaProviderResponse> updateGiteaProvider(
      $grpc.ServiceCall call, $0.UpdateGiteaProviderRequest request);

  $async.Future<$0.UpdateBitbucketProviderResponse> updateBitbucketProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateBitbucketProviderRequest> $request) async {
    return updateBitbucketProvider($call, await $request);
  }

  $async.Future<$0.UpdateBitbucketProviderResponse> updateBitbucketProvider(
      $grpc.ServiceCall call, $0.UpdateBitbucketProviderRequest request);

  $async.Future<$1.Empty> deleteGitProvider_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteGitProviderRequest> $request) async {
    return deleteGitProvider($call, await $request);
  }

  $async.Future<$1.Empty> deleteGitProvider(
      $grpc.ServiceCall call, $0.DeleteGitProviderRequest request);

  $async.Future<$0.TestGitProviderResponse> testGitProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestGitProviderRequest> $request) async {
    return testGitProvider($call, await $request);
  }

  $async.Future<$0.TestGitProviderResponse> testGitProvider(
      $grpc.ServiceCall call, $0.TestGitProviderRequest request);

  $async.Future<$0.GitRotateWebhookSecretResponse> rotateWebhookSecret_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitRotateWebhookSecretRequest> $request) async {
    return rotateWebhookSecret($call, await $request);
  }

  $async.Future<$0.GitRotateWebhookSecretResponse> rotateWebhookSecret(
      $grpc.ServiceCall call, $0.GitRotateWebhookSecretRequest request);

  $async.Future<$0.GitListRepositoriesResponse> listRepositories_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitListRepositoriesRequest> $request) async {
    return listRepositories($call, await $request);
  }

  $async.Future<$0.GitListRepositoriesResponse> listRepositories(
      $grpc.ServiceCall call, $0.GitListRepositoriesRequest request);

  $async.Future<$0.GitListRepositoryBranchesResponse> listBranches_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitListRepositoryBranchesRequest> $request) async {
    return listBranches($call, await $request);
  }

  $async.Future<$0.GitListRepositoryBranchesResponse> listBranches(
      $grpc.ServiceCall call, $0.GitListRepositoryBranchesRequest request);

  $async.Future<$0.GitListTagsResponse> listTags_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GitListTagsRequest> $request) async {
    return listTags($call, await $request);
  }

  $async.Future<$0.GitListTagsResponse> listTags(
      $grpc.ServiceCall call, $0.GitListTagsRequest request);

  $async.Future<$0.GitAuthorizeResponse> authorize_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GitAuthorizeRequest> $request) async {
    return authorize($call, await $request);
  }

  $async.Future<$0.GitAuthorizeResponse> authorize(
      $grpc.ServiceCall call, $0.GitAuthorizeRequest request);

  $async.Future<$0.GitOAuthCallbackResponse> oAuthCallback_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitOAuthCallbackRequest> $request) async {
    return oAuthCallback($call, await $request);
  }

  $async.Future<$0.GitOAuthCallbackResponse> oAuthCallback(
      $grpc.ServiceCall call, $0.GitOAuthCallbackRequest request);

  $async.Future<$1.Empty> disconnect_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GitDisconnectRequest> $request) async {
    return disconnect($call, await $request);
  }

  $async.Future<$1.Empty> disconnect(
      $grpc.ServiceCall call, $0.GitDisconnectRequest request);

  $async.Future<$0.GitGetCollaboratorPermissionResponse>
      getCollaboratorPermission_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.GitGetCollaboratorPermissionRequest>
              $request) async {
    return getCollaboratorPermission($call, await $request);
  }

  $async.Future<$0.GitGetCollaboratorPermissionResponse>
      getCollaboratorPermission($grpc.ServiceCall call,
          $0.GitGetCollaboratorPermissionRequest request);

  $async.Future<$0.GitGetWebhookStatusResponse> getWebhookStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitGetWebhookStatusRequest> $request) async {
    return getWebhookStatus($call, await $request);
  }

  $async.Future<$0.GitGetWebhookStatusResponse> getWebhookStatus(
      $grpc.ServiceCall call, $0.GitGetWebhookStatusRequest request);

  $async.Future<$0.GitInstallWebhookResponse> installWebhook_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitInstallWebhookRequest> $request) async {
    return installWebhook($call, await $request);
  }

  $async.Future<$0.GitInstallWebhookResponse> installWebhook(
      $grpc.ServiceCall call, $0.GitInstallWebhookRequest request);

  $async.Future<$0.GitRecreateWebhookResponse> recreateWebhook_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GitRecreateWebhookRequest> $request) async {
    return recreateWebhook($call, await $request);
  }

  $async.Future<$0.GitRecreateWebhookResponse> recreateWebhook(
      $grpc.ServiceCall call, $0.GitRecreateWebhookRequest request);

  $async.Future<$1.Empty> deleteWebhook_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GitDeleteWebhookRequest> $request) async {
    return deleteWebhook($call, await $request);
  }

  $async.Future<$1.Empty> deleteWebhook(
      $grpc.ServiceCall call, $0.GitDeleteWebhookRequest request);
}

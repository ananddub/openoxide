// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/application.proto.

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

import 'application.pb.dart' as $0;
import 'common.pb.dart' as $1;

export 'application.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ApplicationService')
class ApplicationServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ApplicationServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Application> getApplication(
    $0.GetApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getApplication, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListApplicationsResponse> listApplications(
    $0.ListApplicationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplications, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> createApplication(
    $0.CreateApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createApplication, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> updateApplication(
    $0.UpdateApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateApplication, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteApplication(
    $0.DeleteApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteApplication, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> importApplication(
    $0.ImportApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$importApplication, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationExport> exportApplication(
    $0.ExportApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportApplication, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setGithubSource(
    $0.SetGithubSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGithubSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setGitlabSource(
    $0.SetGitlabSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGitlabSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setBitbucketSource(
    $0.SetBitbucketSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setBitbucketSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setGiteaSource(
    $0.SetGiteaSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGiteaSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setCustomGitSource(
    $0.SetCustomGitSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setCustomGitSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setDockerSource(
    $0.SetDockerSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setDockerSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> setDropSource(
    $0.SetDropSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setDropSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> uploadSource(
    $async.Stream<$0.UploadSourceRequest> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$uploadSource, request, options: options)
        .single;
  }

  $grpc.ResponseFuture<$0.Application> updateBuildConfig(
    $0.UpdateBuildConfigRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateBuildConfig, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> updateResourceConfig(
    $0.UpdateResourceConfigRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateResourceConfig, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> updatePreviewConfig(
    $0.UpdatePreviewConfigRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePreviewConfig, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> deploy(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> redeploy(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$redeploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> rebuild(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rebuild, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> reload(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reload, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> start(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$start, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationOperation> stop(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stop, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cancel(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancel, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationWebhookToken> rotateWebhookToken(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateWebhookToken, request, options: options);
  }

  $grpc.ResponseFuture<$0.Application> moveApplication(
    $0.MoveApplicationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$moveApplication, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> clearDeploymentHistory(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$clearDeploymentHistory, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cleanupDeploymentQueue(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cleanupDeploymentQueue, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> forceKillDeployment(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$forceKillDeployment, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListRollbacksResponse> listRollbacks(
    $0.ListRollbacksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRollbacks, request, options: options);
  }

  $grpc.ResponseFuture<$0.RollbackTrigger> triggerRollback(
    $0.TriggerRollbackRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$triggerRollback, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteRollback(
    $0.DeleteRollbackRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRollback, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationDependencies> listDependencies(
    $0.ApplicationOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDependencies, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationPorts> listApplicationPorts(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationPorts, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationMounts> listApplicationMounts(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationMounts, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationNetworks> listApplicationNetworks(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationNetworks, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationPatches> listApplicationPatches(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationPatches, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationMiddlewares> listApplicationMiddlewares(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationMiddlewares, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationRedirects> listApplicationRedirects(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationRedirects, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationSecurity> listApplicationSecurity(
    $0.ResourceListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationSecurity, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationListPortsResponse> listPorts(
    $0.ApplicationListPortsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPorts, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreatePortResponse> createPort(
    $0.ApplicationCreatePortRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPort, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdatePortResponse> updatePort(
    $0.ApplicationUpdatePortRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePort, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePort(
    $0.ApplicationDeletePortRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePort, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationListMountsResponse> listMounts(
    $0.ApplicationListMountsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMounts, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreateMountResponse> createMount(
    $0.ApplicationCreateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdateMountResponse> updateMount(
    $0.ApplicationUpdateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateMount, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteMount(
    $0.ApplicationDeleteMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationListPatchesResponse> listPatches(
    $0.ApplicationListPatchesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPatches, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreatePatchResponse> createPatch(
    $0.ApplicationCreatePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPatch, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdatePatchResponse> updatePatch(
    $0.ApplicationUpdatePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePatch, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePatch(
    $0.ApplicationDeletePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePatch, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationListMiddlewaresResponse> listMiddlewares(
    $0.ApplicationListMiddlewaresRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMiddlewares, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreateMiddlewareResponse> createMiddleware(
    $0.ApplicationCreateMiddlewareRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createMiddleware, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdateMiddlewareResponse> updateMiddleware(
    $0.ApplicationUpdateMiddlewareRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateMiddleware, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteMiddleware(
    $0.ApplicationDeleteMiddlewareRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteMiddleware, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationListRedirectsResponse> listRedirects(
    $0.ApplicationListRedirectsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRedirects, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreateRedirectResponse> createRedirect(
    $0.ApplicationCreateRedirectRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createRedirect, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdateRedirectResponse> updateRedirect(
    $0.ApplicationUpdateRedirectRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRedirect, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteRedirect(
    $0.ApplicationDeleteRedirectRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRedirect, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationGetSecurityResponse> getSecurity(
    $0.ApplicationGetSecurityRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSecurity, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationCreateSecurityResponse> createSecurity(
    $0.ApplicationCreateSecurityRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createSecurity, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdateSecurityResponse> updateSecurity(
    $0.ApplicationUpdateSecurityRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSecurity, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteSecurity(
    $0.ApplicationDeleteSecurityRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteSecurity, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplicationUpdateNetworksResponse> updateNetworks(
    $0.ApplicationUpdateNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNetworks, request, options: options);
  }

  // method descriptors

  static final _$getApplication =
      $grpc.ClientMethod<$0.GetApplicationRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/GetApplication',
          ($0.GetApplicationRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$listApplications = $grpc.ClientMethod<
          $0.ListApplicationsRequest, $0.ListApplicationsResponse>(
      '/openoxide.api.v1.ApplicationService/ListApplications',
      ($0.ListApplicationsRequest value) => value.writeToBuffer(),
      $0.ListApplicationsResponse.fromBuffer);
  static final _$createApplication =
      $grpc.ClientMethod<$0.CreateApplicationRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/CreateApplication',
          ($0.CreateApplicationRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$updateApplication =
      $grpc.ClientMethod<$0.UpdateApplicationRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/UpdateApplication',
          ($0.UpdateApplicationRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$deleteApplication =
      $grpc.ClientMethod<$0.DeleteApplicationRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteApplication',
          ($0.DeleteApplicationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$importApplication =
      $grpc.ClientMethod<$0.ImportApplicationRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/ImportApplication',
          ($0.ImportApplicationRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$exportApplication =
      $grpc.ClientMethod<$0.ExportApplicationRequest, $0.ApplicationExport>(
          '/openoxide.api.v1.ApplicationService/ExportApplication',
          ($0.ExportApplicationRequest value) => value.writeToBuffer(),
          $0.ApplicationExport.fromBuffer);
  static final _$setGithubSource =
      $grpc.ClientMethod<$0.SetGithubSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetGithubSource',
          ($0.SetGithubSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setGitlabSource =
      $grpc.ClientMethod<$0.SetGitlabSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetGitlabSource',
          ($0.SetGitlabSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setBitbucketSource =
      $grpc.ClientMethod<$0.SetBitbucketSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetBitbucketSource',
          ($0.SetBitbucketSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setGiteaSource =
      $grpc.ClientMethod<$0.SetGiteaSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetGiteaSource',
          ($0.SetGiteaSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setCustomGitSource =
      $grpc.ClientMethod<$0.SetCustomGitSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetCustomGitSource',
          ($0.SetCustomGitSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setDockerSource =
      $grpc.ClientMethod<$0.SetDockerSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetDockerSource',
          ($0.SetDockerSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$setDropSource =
      $grpc.ClientMethod<$0.SetDropSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/SetDropSource',
          ($0.SetDropSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$uploadSource =
      $grpc.ClientMethod<$0.UploadSourceRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/UploadSource',
          ($0.UploadSourceRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$updateBuildConfig =
      $grpc.ClientMethod<$0.UpdateBuildConfigRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/UpdateBuildConfig',
          ($0.UpdateBuildConfigRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$updateResourceConfig =
      $grpc.ClientMethod<$0.UpdateResourceConfigRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/UpdateResourceConfig',
          ($0.UpdateResourceConfigRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$updatePreviewConfig =
      $grpc.ClientMethod<$0.UpdatePreviewConfigRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/UpdatePreviewConfig',
          ($0.UpdatePreviewConfigRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$deploy = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Deploy',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$redeploy = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Redeploy',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$rebuild = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Rebuild',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$reload = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Reload',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$start = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Start',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$stop = $grpc.ClientMethod<$0.ApplicationOperationRequest,
          $0.ApplicationOperation>(
      '/openoxide.api.v1.ApplicationService/Stop',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationOperation.fromBuffer);
  static final _$cancel =
      $grpc.ClientMethod<$0.ApplicationOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/Cancel',
          ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$rotateWebhookToken = $grpc.ClientMethod<
          $0.ApplicationOperationRequest, $0.ApplicationWebhookToken>(
      '/openoxide.api.v1.ApplicationService/RotateWebhookToken',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationWebhookToken.fromBuffer);
  static final _$moveApplication =
      $grpc.ClientMethod<$0.MoveApplicationRequest, $0.Application>(
          '/openoxide.api.v1.ApplicationService/MoveApplication',
          ($0.MoveApplicationRequest value) => value.writeToBuffer(),
          $0.Application.fromBuffer);
  static final _$clearDeploymentHistory =
      $grpc.ClientMethod<$0.ApplicationOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/ClearDeploymentHistory',
          ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$cleanupDeploymentQueue =
      $grpc.ClientMethod<$0.ApplicationOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/CleanupDeploymentQueue',
          ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$forceKillDeployment =
      $grpc.ClientMethod<$0.ApplicationOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/ForceKillDeployment',
          ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listRollbacks =
      $grpc.ClientMethod<$0.ListRollbacksRequest, $0.ListRollbacksResponse>(
          '/openoxide.api.v1.ApplicationService/ListRollbacks',
          ($0.ListRollbacksRequest value) => value.writeToBuffer(),
          $0.ListRollbacksResponse.fromBuffer);
  static final _$triggerRollback =
      $grpc.ClientMethod<$0.TriggerRollbackRequest, $0.RollbackTrigger>(
          '/openoxide.api.v1.ApplicationService/TriggerRollback',
          ($0.TriggerRollbackRequest value) => value.writeToBuffer(),
          $0.RollbackTrigger.fromBuffer);
  static final _$deleteRollback =
      $grpc.ClientMethod<$0.DeleteRollbackRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteRollback',
          ($0.DeleteRollbackRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listDependencies = $grpc.ClientMethod<
          $0.ApplicationOperationRequest, $0.ApplicationDependencies>(
      '/openoxide.api.v1.ApplicationService/ListDependencies',
      ($0.ApplicationOperationRequest value) => value.writeToBuffer(),
      $0.ApplicationDependencies.fromBuffer);
  static final _$listApplicationPorts =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationPorts>(
          '/openoxide.api.v1.ApplicationService/ListApplicationPorts',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationPorts.fromBuffer);
  static final _$listApplicationMounts =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationMounts>(
          '/openoxide.api.v1.ApplicationService/ListApplicationMounts',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationMounts.fromBuffer);
  static final _$listApplicationNetworks =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationNetworks>(
          '/openoxide.api.v1.ApplicationService/ListApplicationNetworks',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationNetworks.fromBuffer);
  static final _$listApplicationPatches =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationPatches>(
          '/openoxide.api.v1.ApplicationService/ListApplicationPatches',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationPatches.fromBuffer);
  static final _$listApplicationMiddlewares =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationMiddlewares>(
          '/openoxide.api.v1.ApplicationService/ListApplicationMiddlewares',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationMiddlewares.fromBuffer);
  static final _$listApplicationRedirects =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationRedirects>(
          '/openoxide.api.v1.ApplicationService/ListApplicationRedirects',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationRedirects.fromBuffer);
  static final _$listApplicationSecurity =
      $grpc.ClientMethod<$0.ResourceListRequest, $0.ApplicationSecurity>(
          '/openoxide.api.v1.ApplicationService/ListApplicationSecurity',
          ($0.ResourceListRequest value) => value.writeToBuffer(),
          $0.ApplicationSecurity.fromBuffer);
  static final _$listPorts = $grpc.ClientMethod<$0.ApplicationListPortsRequest,
          $0.ApplicationListPortsResponse>(
      '/openoxide.api.v1.ApplicationService/ListPorts',
      ($0.ApplicationListPortsRequest value) => value.writeToBuffer(),
      $0.ApplicationListPortsResponse.fromBuffer);
  static final _$createPort = $grpc.ClientMethod<
          $0.ApplicationCreatePortRequest, $0.ApplicationCreatePortResponse>(
      '/openoxide.api.v1.ApplicationService/CreatePort',
      ($0.ApplicationCreatePortRequest value) => value.writeToBuffer(),
      $0.ApplicationCreatePortResponse.fromBuffer);
  static final _$updatePort = $grpc.ClientMethod<
          $0.ApplicationUpdatePortRequest, $0.ApplicationUpdatePortResponse>(
      '/openoxide.api.v1.ApplicationService/UpdatePort',
      ($0.ApplicationUpdatePortRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdatePortResponse.fromBuffer);
  static final _$deletePort =
      $grpc.ClientMethod<$0.ApplicationDeletePortRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeletePort',
          ($0.ApplicationDeletePortRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listMounts = $grpc.ClientMethod<
          $0.ApplicationListMountsRequest, $0.ApplicationListMountsResponse>(
      '/openoxide.api.v1.ApplicationService/ListMounts',
      ($0.ApplicationListMountsRequest value) => value.writeToBuffer(),
      $0.ApplicationListMountsResponse.fromBuffer);
  static final _$createMount = $grpc.ClientMethod<
          $0.ApplicationCreateMountRequest, $0.ApplicationCreateMountResponse>(
      '/openoxide.api.v1.ApplicationService/CreateMount',
      ($0.ApplicationCreateMountRequest value) => value.writeToBuffer(),
      $0.ApplicationCreateMountResponse.fromBuffer);
  static final _$updateMount = $grpc.ClientMethod<
          $0.ApplicationUpdateMountRequest, $0.ApplicationUpdateMountResponse>(
      '/openoxide.api.v1.ApplicationService/UpdateMount',
      ($0.ApplicationUpdateMountRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdateMountResponse.fromBuffer);
  static final _$deleteMount =
      $grpc.ClientMethod<$0.ApplicationDeleteMountRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteMount',
          ($0.ApplicationDeleteMountRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listPatches = $grpc.ClientMethod<
          $0.ApplicationListPatchesRequest, $0.ApplicationListPatchesResponse>(
      '/openoxide.api.v1.ApplicationService/ListPatches',
      ($0.ApplicationListPatchesRequest value) => value.writeToBuffer(),
      $0.ApplicationListPatchesResponse.fromBuffer);
  static final _$createPatch = $grpc.ClientMethod<
          $0.ApplicationCreatePatchRequest, $0.ApplicationCreatePatchResponse>(
      '/openoxide.api.v1.ApplicationService/CreatePatch',
      ($0.ApplicationCreatePatchRequest value) => value.writeToBuffer(),
      $0.ApplicationCreatePatchResponse.fromBuffer);
  static final _$updatePatch = $grpc.ClientMethod<
          $0.ApplicationUpdatePatchRequest, $0.ApplicationUpdatePatchResponse>(
      '/openoxide.api.v1.ApplicationService/UpdatePatch',
      ($0.ApplicationUpdatePatchRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdatePatchResponse.fromBuffer);
  static final _$deletePatch =
      $grpc.ClientMethod<$0.ApplicationDeletePatchRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeletePatch',
          ($0.ApplicationDeletePatchRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listMiddlewares = $grpc.ClientMethod<
          $0.ApplicationListMiddlewaresRequest,
          $0.ApplicationListMiddlewaresResponse>(
      '/openoxide.api.v1.ApplicationService/ListMiddlewares',
      ($0.ApplicationListMiddlewaresRequest value) => value.writeToBuffer(),
      $0.ApplicationListMiddlewaresResponse.fromBuffer);
  static final _$createMiddleware = $grpc.ClientMethod<
          $0.ApplicationCreateMiddlewareRequest,
          $0.ApplicationCreateMiddlewareResponse>(
      '/openoxide.api.v1.ApplicationService/CreateMiddleware',
      ($0.ApplicationCreateMiddlewareRequest value) => value.writeToBuffer(),
      $0.ApplicationCreateMiddlewareResponse.fromBuffer);
  static final _$updateMiddleware = $grpc.ClientMethod<
          $0.ApplicationUpdateMiddlewareRequest,
          $0.ApplicationUpdateMiddlewareResponse>(
      '/openoxide.api.v1.ApplicationService/UpdateMiddleware',
      ($0.ApplicationUpdateMiddlewareRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdateMiddlewareResponse.fromBuffer);
  static final _$deleteMiddleware =
      $grpc.ClientMethod<$0.ApplicationDeleteMiddlewareRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteMiddleware',
          ($0.ApplicationDeleteMiddlewareRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listRedirects = $grpc.ClientMethod<
          $0.ApplicationListRedirectsRequest,
          $0.ApplicationListRedirectsResponse>(
      '/openoxide.api.v1.ApplicationService/ListRedirects',
      ($0.ApplicationListRedirectsRequest value) => value.writeToBuffer(),
      $0.ApplicationListRedirectsResponse.fromBuffer);
  static final _$createRedirect = $grpc.ClientMethod<
          $0.ApplicationCreateRedirectRequest,
          $0.ApplicationCreateRedirectResponse>(
      '/openoxide.api.v1.ApplicationService/CreateRedirect',
      ($0.ApplicationCreateRedirectRequest value) => value.writeToBuffer(),
      $0.ApplicationCreateRedirectResponse.fromBuffer);
  static final _$updateRedirect = $grpc.ClientMethod<
          $0.ApplicationUpdateRedirectRequest,
          $0.ApplicationUpdateRedirectResponse>(
      '/openoxide.api.v1.ApplicationService/UpdateRedirect',
      ($0.ApplicationUpdateRedirectRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdateRedirectResponse.fromBuffer);
  static final _$deleteRedirect =
      $grpc.ClientMethod<$0.ApplicationDeleteRedirectRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteRedirect',
          ($0.ApplicationDeleteRedirectRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getSecurity = $grpc.ClientMethod<
          $0.ApplicationGetSecurityRequest, $0.ApplicationGetSecurityResponse>(
      '/openoxide.api.v1.ApplicationService/GetSecurity',
      ($0.ApplicationGetSecurityRequest value) => value.writeToBuffer(),
      $0.ApplicationGetSecurityResponse.fromBuffer);
  static final _$createSecurity = $grpc.ClientMethod<
          $0.ApplicationCreateSecurityRequest,
          $0.ApplicationCreateSecurityResponse>(
      '/openoxide.api.v1.ApplicationService/CreateSecurity',
      ($0.ApplicationCreateSecurityRequest value) => value.writeToBuffer(),
      $0.ApplicationCreateSecurityResponse.fromBuffer);
  static final _$updateSecurity = $grpc.ClientMethod<
          $0.ApplicationUpdateSecurityRequest,
          $0.ApplicationUpdateSecurityResponse>(
      '/openoxide.api.v1.ApplicationService/UpdateSecurity',
      ($0.ApplicationUpdateSecurityRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdateSecurityResponse.fromBuffer);
  static final _$deleteSecurity =
      $grpc.ClientMethod<$0.ApplicationDeleteSecurityRequest, $1.Empty>(
          '/openoxide.api.v1.ApplicationService/DeleteSecurity',
          ($0.ApplicationDeleteSecurityRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$updateNetworks = $grpc.ClientMethod<
          $0.ApplicationUpdateNetworksRequest,
          $0.ApplicationUpdateNetworksResponse>(
      '/openoxide.api.v1.ApplicationService/UpdateNetworks',
      ($0.ApplicationUpdateNetworksRequest value) => value.writeToBuffer(),
      $0.ApplicationUpdateNetworksResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ApplicationService')
abstract class ApplicationServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ApplicationService';

  ApplicationServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetApplicationRequest, $0.Application>(
        'GetApplication',
        getApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetApplicationRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListApplicationsRequest,
            $0.ListApplicationsResponse>(
        'ListApplications',
        listApplications_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListApplicationsRequest.fromBuffer(value),
        ($0.ListApplicationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateApplicationRequest, $0.Application>(
        'CreateApplication',
        createApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateApplicationRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateApplicationRequest, $0.Application>(
        'UpdateApplication',
        updateApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateApplicationRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteApplicationRequest, $1.Empty>(
        'DeleteApplication',
        deleteApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteApplicationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ImportApplicationRequest, $0.Application>(
        'ImportApplication',
        importApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ImportApplicationRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ExportApplicationRequest, $0.ApplicationExport>(
            'ExportApplication',
            exportApplication_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ExportApplicationRequest.fromBuffer(value),
            ($0.ApplicationExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetGithubSourceRequest, $0.Application>(
        'SetGithubSource',
        setGithubSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetGithubSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetGitlabSourceRequest, $0.Application>(
        'SetGitlabSource',
        setGitlabSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetGitlabSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SetBitbucketSourceRequest, $0.Application>(
            'SetBitbucketSource',
            setBitbucketSource_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SetBitbucketSourceRequest.fromBuffer(value),
            ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetGiteaSourceRequest, $0.Application>(
        'SetGiteaSource',
        setGiteaSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetGiteaSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SetCustomGitSourceRequest, $0.Application>(
            'SetCustomGitSource',
            setCustomGitSource_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SetCustomGitSourceRequest.fromBuffer(value),
            ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetDockerSourceRequest, $0.Application>(
        'SetDockerSource',
        setDockerSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetDockerSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetDropSourceRequest, $0.Application>(
        'SetDropSource',
        setDropSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetDropSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UploadSourceRequest, $0.Application>(
        'UploadSource',
        uploadSource,
        true,
        false,
        ($core.List<$core.int> value) =>
            $0.UploadSourceRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateBuildConfigRequest, $0.Application>(
        'UpdateBuildConfig',
        updateBuildConfig_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateBuildConfigRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateResourceConfigRequest, $0.Application>(
            'UpdateResourceConfig',
            updateResourceConfig_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateResourceConfigRequest.fromBuffer(value),
            ($0.Application value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdatePreviewConfigRequest, $0.Application>(
            'UpdatePreviewConfig',
            updatePreviewConfig_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdatePreviewConfigRequest.fromBuffer(value),
            ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Deploy',
        deploy_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Redeploy',
        redeploy_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Rebuild',
        rebuild_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Reload',
        reload_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Start',
        start_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationOperation>(
        'Stop',
        stop_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest, $1.Empty>(
        'Cancel',
        cancel_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationWebhookToken>(
        'RotateWebhookToken',
        rotateWebhookToken_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationWebhookToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MoveApplicationRequest, $0.Application>(
        'MoveApplication',
        moveApplication_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MoveApplicationRequest.fromBuffer(value),
        ($0.Application value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest, $1.Empty>(
        'ClearDeploymentHistory',
        clearDeploymentHistory_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest, $1.Empty>(
        'CleanupDeploymentQueue',
        cleanupDeploymentQueue_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest, $1.Empty>(
        'ForceKillDeployment',
        forceKillDeployment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListRollbacksRequest, $0.ListRollbacksResponse>(
            'ListRollbacks',
            listRollbacks_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListRollbacksRequest.fromBuffer(value),
            ($0.ListRollbacksResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TriggerRollbackRequest, $0.RollbackTrigger>(
            'TriggerRollback',
            triggerRollback_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TriggerRollbackRequest.fromBuffer(value),
            ($0.RollbackTrigger value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteRollbackRequest, $1.Empty>(
        'DeleteRollback',
        deleteRollback_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteRollbackRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationOperationRequest,
            $0.ApplicationDependencies>(
        'ListDependencies',
        listDependencies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationOperationRequest.fromBuffer(value),
        ($0.ApplicationDependencies value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationPorts>(
        'ListApplicationPorts',
        listApplicationPorts_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ResourceListRequest.fromBuffer(value),
        ($0.ApplicationPorts value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationMounts>(
            'ListApplicationMounts',
            listApplicationMounts_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationMounts value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationNetworks>(
            'ListApplicationNetworks',
            listApplicationNetworks_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationNetworks value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationPatches>(
            'ListApplicationPatches',
            listApplicationPatches_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationPatches value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationMiddlewares>(
            'ListApplicationMiddlewares',
            listApplicationMiddlewares_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationMiddlewares value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationRedirects>(
            'ListApplicationRedirects',
            listApplicationRedirects_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationRedirects value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ResourceListRequest, $0.ApplicationSecurity>(
            'ListApplicationSecurity',
            listApplicationSecurity_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ResourceListRequest.fromBuffer(value),
            ($0.ApplicationSecurity value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationListPortsRequest,
            $0.ApplicationListPortsResponse>(
        'ListPorts',
        listPorts_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationListPortsRequest.fromBuffer(value),
        ($0.ApplicationListPortsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreatePortRequest,
            $0.ApplicationCreatePortResponse>(
        'CreatePort',
        createPort_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreatePortRequest.fromBuffer(value),
        ($0.ApplicationCreatePortResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdatePortRequest,
            $0.ApplicationUpdatePortResponse>(
        'UpdatePort',
        updatePort_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdatePortRequest.fromBuffer(value),
        ($0.ApplicationUpdatePortResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationDeletePortRequest, $1.Empty>(
        'DeletePort',
        deletePort_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationDeletePortRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationListMountsRequest,
            $0.ApplicationListMountsResponse>(
        'ListMounts',
        listMounts_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationListMountsRequest.fromBuffer(value),
        ($0.ApplicationListMountsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreateMountRequest,
            $0.ApplicationCreateMountResponse>(
        'CreateMount',
        createMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreateMountRequest.fromBuffer(value),
        ($0.ApplicationCreateMountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdateMountRequest,
            $0.ApplicationUpdateMountResponse>(
        'UpdateMount',
        updateMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdateMountRequest.fromBuffer(value),
        ($0.ApplicationUpdateMountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationDeleteMountRequest, $1.Empty>(
        'DeleteMount',
        deleteMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationDeleteMountRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationListPatchesRequest,
            $0.ApplicationListPatchesResponse>(
        'ListPatches',
        listPatches_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationListPatchesRequest.fromBuffer(value),
        ($0.ApplicationListPatchesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreatePatchRequest,
            $0.ApplicationCreatePatchResponse>(
        'CreatePatch',
        createPatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreatePatchRequest.fromBuffer(value),
        ($0.ApplicationCreatePatchResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdatePatchRequest,
            $0.ApplicationUpdatePatchResponse>(
        'UpdatePatch',
        updatePatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdatePatchRequest.fromBuffer(value),
        ($0.ApplicationUpdatePatchResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationDeletePatchRequest, $1.Empty>(
        'DeletePatch',
        deletePatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationDeletePatchRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationListMiddlewaresRequest,
            $0.ApplicationListMiddlewaresResponse>(
        'ListMiddlewares',
        listMiddlewares_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationListMiddlewaresRequest.fromBuffer(value),
        ($0.ApplicationListMiddlewaresResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreateMiddlewareRequest,
            $0.ApplicationCreateMiddlewareResponse>(
        'CreateMiddleware',
        createMiddleware_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreateMiddlewareRequest.fromBuffer(value),
        ($0.ApplicationCreateMiddlewareResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdateMiddlewareRequest,
            $0.ApplicationUpdateMiddlewareResponse>(
        'UpdateMiddleware',
        updateMiddleware_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdateMiddlewareRequest.fromBuffer(value),
        ($0.ApplicationUpdateMiddlewareResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ApplicationDeleteMiddlewareRequest, $1.Empty>(
            'DeleteMiddleware',
            deleteMiddleware_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ApplicationDeleteMiddlewareRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationListRedirectsRequest,
            $0.ApplicationListRedirectsResponse>(
        'ListRedirects',
        listRedirects_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationListRedirectsRequest.fromBuffer(value),
        ($0.ApplicationListRedirectsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreateRedirectRequest,
            $0.ApplicationCreateRedirectResponse>(
        'CreateRedirect',
        createRedirect_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreateRedirectRequest.fromBuffer(value),
        ($0.ApplicationCreateRedirectResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdateRedirectRequest,
            $0.ApplicationUpdateRedirectResponse>(
        'UpdateRedirect',
        updateRedirect_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdateRedirectRequest.fromBuffer(value),
        ($0.ApplicationUpdateRedirectResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ApplicationDeleteRedirectRequest, $1.Empty>(
            'DeleteRedirect',
            deleteRedirect_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ApplicationDeleteRedirectRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationGetSecurityRequest,
            $0.ApplicationGetSecurityResponse>(
        'GetSecurity',
        getSecurity_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationGetSecurityRequest.fromBuffer(value),
        ($0.ApplicationGetSecurityResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationCreateSecurityRequest,
            $0.ApplicationCreateSecurityResponse>(
        'CreateSecurity',
        createSecurity_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationCreateSecurityRequest.fromBuffer(value),
        ($0.ApplicationCreateSecurityResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdateSecurityRequest,
            $0.ApplicationUpdateSecurityResponse>(
        'UpdateSecurity',
        updateSecurity_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdateSecurityRequest.fromBuffer(value),
        ($0.ApplicationUpdateSecurityResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ApplicationDeleteSecurityRequest, $1.Empty>(
            'DeleteSecurity',
            deleteSecurity_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ApplicationDeleteSecurityRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplicationUpdateNetworksRequest,
            $0.ApplicationUpdateNetworksResponse>(
        'UpdateNetworks',
        updateNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ApplicationUpdateNetworksRequest.fromBuffer(value),
        ($0.ApplicationUpdateNetworksResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.Application> getApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetApplicationRequest> $request) async {
    return getApplication($call, await $request);
  }

  $async.Future<$0.Application> getApplication(
      $grpc.ServiceCall call, $0.GetApplicationRequest request);

  $async.Future<$0.ListApplicationsResponse> listApplications_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListApplicationsRequest> $request) async {
    return listApplications($call, await $request);
  }

  $async.Future<$0.ListApplicationsResponse> listApplications(
      $grpc.ServiceCall call, $0.ListApplicationsRequest request);

  $async.Future<$0.Application> createApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateApplicationRequest> $request) async {
    return createApplication($call, await $request);
  }

  $async.Future<$0.Application> createApplication(
      $grpc.ServiceCall call, $0.CreateApplicationRequest request);

  $async.Future<$0.Application> updateApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateApplicationRequest> $request) async {
    return updateApplication($call, await $request);
  }

  $async.Future<$0.Application> updateApplication(
      $grpc.ServiceCall call, $0.UpdateApplicationRequest request);

  $async.Future<$1.Empty> deleteApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteApplicationRequest> $request) async {
    return deleteApplication($call, await $request);
  }

  $async.Future<$1.Empty> deleteApplication(
      $grpc.ServiceCall call, $0.DeleteApplicationRequest request);

  $async.Future<$0.Application> importApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ImportApplicationRequest> $request) async {
    return importApplication($call, await $request);
  }

  $async.Future<$0.Application> importApplication(
      $grpc.ServiceCall call, $0.ImportApplicationRequest request);

  $async.Future<$0.ApplicationExport> exportApplication_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ExportApplicationRequest> $request) async {
    return exportApplication($call, await $request);
  }

  $async.Future<$0.ApplicationExport> exportApplication(
      $grpc.ServiceCall call, $0.ExportApplicationRequest request);

  $async.Future<$0.Application> setGithubSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetGithubSourceRequest> $request) async {
    return setGithubSource($call, await $request);
  }

  $async.Future<$0.Application> setGithubSource(
      $grpc.ServiceCall call, $0.SetGithubSourceRequest request);

  $async.Future<$0.Application> setGitlabSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetGitlabSourceRequest> $request) async {
    return setGitlabSource($call, await $request);
  }

  $async.Future<$0.Application> setGitlabSource(
      $grpc.ServiceCall call, $0.SetGitlabSourceRequest request);

  $async.Future<$0.Application> setBitbucketSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetBitbucketSourceRequest> $request) async {
    return setBitbucketSource($call, await $request);
  }

  $async.Future<$0.Application> setBitbucketSource(
      $grpc.ServiceCall call, $0.SetBitbucketSourceRequest request);

  $async.Future<$0.Application> setGiteaSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetGiteaSourceRequest> $request) async {
    return setGiteaSource($call, await $request);
  }

  $async.Future<$0.Application> setGiteaSource(
      $grpc.ServiceCall call, $0.SetGiteaSourceRequest request);

  $async.Future<$0.Application> setCustomGitSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetCustomGitSourceRequest> $request) async {
    return setCustomGitSource($call, await $request);
  }

  $async.Future<$0.Application> setCustomGitSource(
      $grpc.ServiceCall call, $0.SetCustomGitSourceRequest request);

  $async.Future<$0.Application> setDockerSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetDockerSourceRequest> $request) async {
    return setDockerSource($call, await $request);
  }

  $async.Future<$0.Application> setDockerSource(
      $grpc.ServiceCall call, $0.SetDockerSourceRequest request);

  $async.Future<$0.Application> setDropSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetDropSourceRequest> $request) async {
    return setDropSource($call, await $request);
  }

  $async.Future<$0.Application> setDropSource(
      $grpc.ServiceCall call, $0.SetDropSourceRequest request);

  $async.Future<$0.Application> uploadSource(
      $grpc.ServiceCall call, $async.Stream<$0.UploadSourceRequest> request);

  $async.Future<$0.Application> updateBuildConfig_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateBuildConfigRequest> $request) async {
    return updateBuildConfig($call, await $request);
  }

  $async.Future<$0.Application> updateBuildConfig(
      $grpc.ServiceCall call, $0.UpdateBuildConfigRequest request);

  $async.Future<$0.Application> updateResourceConfig_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateResourceConfigRequest> $request) async {
    return updateResourceConfig($call, await $request);
  }

  $async.Future<$0.Application> updateResourceConfig(
      $grpc.ServiceCall call, $0.UpdateResourceConfigRequest request);

  $async.Future<$0.Application> updatePreviewConfig_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdatePreviewConfigRequest> $request) async {
    return updatePreviewConfig($call, await $request);
  }

  $async.Future<$0.Application> updatePreviewConfig(
      $grpc.ServiceCall call, $0.UpdatePreviewConfigRequest request);

  $async.Future<$0.ApplicationOperation> deploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return deploy($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> deploy(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationOperation> redeploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return redeploy($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> redeploy(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationOperation> rebuild_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return rebuild($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> rebuild(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationOperation> reload_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return reload($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> reload(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationOperation> start_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return start($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> start(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationOperation> stop_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return stop($call, await $request);
  }

  $async.Future<$0.ApplicationOperation> stop(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$1.Empty> cancel_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return cancel($call, await $request);
  }

  $async.Future<$1.Empty> cancel(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationWebhookToken> rotateWebhookToken_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return rotateWebhookToken($call, await $request);
  }

  $async.Future<$0.ApplicationWebhookToken> rotateWebhookToken(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.Application> moveApplication_Pre($grpc.ServiceCall $call,
      $async.Future<$0.MoveApplicationRequest> $request) async {
    return moveApplication($call, await $request);
  }

  $async.Future<$0.Application> moveApplication(
      $grpc.ServiceCall call, $0.MoveApplicationRequest request);

  $async.Future<$1.Empty> clearDeploymentHistory_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return clearDeploymentHistory($call, await $request);
  }

  $async.Future<$1.Empty> clearDeploymentHistory(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$1.Empty> cleanupDeploymentQueue_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return cleanupDeploymentQueue($call, await $request);
  }

  $async.Future<$1.Empty> cleanupDeploymentQueue(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$1.Empty> forceKillDeployment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return forceKillDeployment($call, await $request);
  }

  $async.Future<$1.Empty> forceKillDeployment(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ListRollbacksResponse> listRollbacks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListRollbacksRequest> $request) async {
    return listRollbacks($call, await $request);
  }

  $async.Future<$0.ListRollbacksResponse> listRollbacks(
      $grpc.ServiceCall call, $0.ListRollbacksRequest request);

  $async.Future<$0.RollbackTrigger> triggerRollback_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TriggerRollbackRequest> $request) async {
    return triggerRollback($call, await $request);
  }

  $async.Future<$0.RollbackTrigger> triggerRollback(
      $grpc.ServiceCall call, $0.TriggerRollbackRequest request);

  $async.Future<$1.Empty> deleteRollback_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteRollbackRequest> $request) async {
    return deleteRollback($call, await $request);
  }

  $async.Future<$1.Empty> deleteRollback(
      $grpc.ServiceCall call, $0.DeleteRollbackRequest request);

  $async.Future<$0.ApplicationDependencies> listDependencies_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationOperationRequest> $request) async {
    return listDependencies($call, await $request);
  }

  $async.Future<$0.ApplicationDependencies> listDependencies(
      $grpc.ServiceCall call, $0.ApplicationOperationRequest request);

  $async.Future<$0.ApplicationPorts> listApplicationPorts_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationPorts($call, await $request);
  }

  $async.Future<$0.ApplicationPorts> listApplicationPorts(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationMounts> listApplicationMounts_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationMounts($call, await $request);
  }

  $async.Future<$0.ApplicationMounts> listApplicationMounts(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationNetworks> listApplicationNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationNetworks($call, await $request);
  }

  $async.Future<$0.ApplicationNetworks> listApplicationNetworks(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationPatches> listApplicationPatches_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationPatches($call, await $request);
  }

  $async.Future<$0.ApplicationPatches> listApplicationPatches(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationMiddlewares> listApplicationMiddlewares_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationMiddlewares($call, await $request);
  }

  $async.Future<$0.ApplicationMiddlewares> listApplicationMiddlewares(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationRedirects> listApplicationRedirects_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationRedirects($call, await $request);
  }

  $async.Future<$0.ApplicationRedirects> listApplicationRedirects(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationSecurity> listApplicationSecurity_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ResourceListRequest> $request) async {
    return listApplicationSecurity($call, await $request);
  }

  $async.Future<$0.ApplicationSecurity> listApplicationSecurity(
      $grpc.ServiceCall call, $0.ResourceListRequest request);

  $async.Future<$0.ApplicationListPortsResponse> listPorts_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationListPortsRequest> $request) async {
    return listPorts($call, await $request);
  }

  $async.Future<$0.ApplicationListPortsResponse> listPorts(
      $grpc.ServiceCall call, $0.ApplicationListPortsRequest request);

  $async.Future<$0.ApplicationCreatePortResponse> createPort_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreatePortRequest> $request) async {
    return createPort($call, await $request);
  }

  $async.Future<$0.ApplicationCreatePortResponse> createPort(
      $grpc.ServiceCall call, $0.ApplicationCreatePortRequest request);

  $async.Future<$0.ApplicationUpdatePortResponse> updatePort_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdatePortRequest> $request) async {
    return updatePort($call, await $request);
  }

  $async.Future<$0.ApplicationUpdatePortResponse> updatePort(
      $grpc.ServiceCall call, $0.ApplicationUpdatePortRequest request);

  $async.Future<$1.Empty> deletePort_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeletePortRequest> $request) async {
    return deletePort($call, await $request);
  }

  $async.Future<$1.Empty> deletePort(
      $grpc.ServiceCall call, $0.ApplicationDeletePortRequest request);

  $async.Future<$0.ApplicationListMountsResponse> listMounts_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationListMountsRequest> $request) async {
    return listMounts($call, await $request);
  }

  $async.Future<$0.ApplicationListMountsResponse> listMounts(
      $grpc.ServiceCall call, $0.ApplicationListMountsRequest request);

  $async.Future<$0.ApplicationCreateMountResponse> createMount_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreateMountRequest> $request) async {
    return createMount($call, await $request);
  }

  $async.Future<$0.ApplicationCreateMountResponse> createMount(
      $grpc.ServiceCall call, $0.ApplicationCreateMountRequest request);

  $async.Future<$0.ApplicationUpdateMountResponse> updateMount_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdateMountRequest> $request) async {
    return updateMount($call, await $request);
  }

  $async.Future<$0.ApplicationUpdateMountResponse> updateMount(
      $grpc.ServiceCall call, $0.ApplicationUpdateMountRequest request);

  $async.Future<$1.Empty> deleteMount_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeleteMountRequest> $request) async {
    return deleteMount($call, await $request);
  }

  $async.Future<$1.Empty> deleteMount(
      $grpc.ServiceCall call, $0.ApplicationDeleteMountRequest request);

  $async.Future<$0.ApplicationListPatchesResponse> listPatches_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationListPatchesRequest> $request) async {
    return listPatches($call, await $request);
  }

  $async.Future<$0.ApplicationListPatchesResponse> listPatches(
      $grpc.ServiceCall call, $0.ApplicationListPatchesRequest request);

  $async.Future<$0.ApplicationCreatePatchResponse> createPatch_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreatePatchRequest> $request) async {
    return createPatch($call, await $request);
  }

  $async.Future<$0.ApplicationCreatePatchResponse> createPatch(
      $grpc.ServiceCall call, $0.ApplicationCreatePatchRequest request);

  $async.Future<$0.ApplicationUpdatePatchResponse> updatePatch_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdatePatchRequest> $request) async {
    return updatePatch($call, await $request);
  }

  $async.Future<$0.ApplicationUpdatePatchResponse> updatePatch(
      $grpc.ServiceCall call, $0.ApplicationUpdatePatchRequest request);

  $async.Future<$1.Empty> deletePatch_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeletePatchRequest> $request) async {
    return deletePatch($call, await $request);
  }

  $async.Future<$1.Empty> deletePatch(
      $grpc.ServiceCall call, $0.ApplicationDeletePatchRequest request);

  $async.Future<$0.ApplicationListMiddlewaresResponse> listMiddlewares_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationListMiddlewaresRequest> $request) async {
    return listMiddlewares($call, await $request);
  }

  $async.Future<$0.ApplicationListMiddlewaresResponse> listMiddlewares(
      $grpc.ServiceCall call, $0.ApplicationListMiddlewaresRequest request);

  $async.Future<$0.ApplicationCreateMiddlewareResponse> createMiddleware_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreateMiddlewareRequest> $request) async {
    return createMiddleware($call, await $request);
  }

  $async.Future<$0.ApplicationCreateMiddlewareResponse> createMiddleware(
      $grpc.ServiceCall call, $0.ApplicationCreateMiddlewareRequest request);

  $async.Future<$0.ApplicationUpdateMiddlewareResponse> updateMiddleware_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdateMiddlewareRequest> $request) async {
    return updateMiddleware($call, await $request);
  }

  $async.Future<$0.ApplicationUpdateMiddlewareResponse> updateMiddleware(
      $grpc.ServiceCall call, $0.ApplicationUpdateMiddlewareRequest request);

  $async.Future<$1.Empty> deleteMiddleware_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeleteMiddlewareRequest> $request) async {
    return deleteMiddleware($call, await $request);
  }

  $async.Future<$1.Empty> deleteMiddleware(
      $grpc.ServiceCall call, $0.ApplicationDeleteMiddlewareRequest request);

  $async.Future<$0.ApplicationListRedirectsResponse> listRedirects_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationListRedirectsRequest> $request) async {
    return listRedirects($call, await $request);
  }

  $async.Future<$0.ApplicationListRedirectsResponse> listRedirects(
      $grpc.ServiceCall call, $0.ApplicationListRedirectsRequest request);

  $async.Future<$0.ApplicationCreateRedirectResponse> createRedirect_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreateRedirectRequest> $request) async {
    return createRedirect($call, await $request);
  }

  $async.Future<$0.ApplicationCreateRedirectResponse> createRedirect(
      $grpc.ServiceCall call, $0.ApplicationCreateRedirectRequest request);

  $async.Future<$0.ApplicationUpdateRedirectResponse> updateRedirect_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdateRedirectRequest> $request) async {
    return updateRedirect($call, await $request);
  }

  $async.Future<$0.ApplicationUpdateRedirectResponse> updateRedirect(
      $grpc.ServiceCall call, $0.ApplicationUpdateRedirectRequest request);

  $async.Future<$1.Empty> deleteRedirect_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeleteRedirectRequest> $request) async {
    return deleteRedirect($call, await $request);
  }

  $async.Future<$1.Empty> deleteRedirect(
      $grpc.ServiceCall call, $0.ApplicationDeleteRedirectRequest request);

  $async.Future<$0.ApplicationGetSecurityResponse> getSecurity_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationGetSecurityRequest> $request) async {
    return getSecurity($call, await $request);
  }

  $async.Future<$0.ApplicationGetSecurityResponse> getSecurity(
      $grpc.ServiceCall call, $0.ApplicationGetSecurityRequest request);

  $async.Future<$0.ApplicationCreateSecurityResponse> createSecurity_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationCreateSecurityRequest> $request) async {
    return createSecurity($call, await $request);
  }

  $async.Future<$0.ApplicationCreateSecurityResponse> createSecurity(
      $grpc.ServiceCall call, $0.ApplicationCreateSecurityRequest request);

  $async.Future<$0.ApplicationUpdateSecurityResponse> updateSecurity_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdateSecurityRequest> $request) async {
    return updateSecurity($call, await $request);
  }

  $async.Future<$0.ApplicationUpdateSecurityResponse> updateSecurity(
      $grpc.ServiceCall call, $0.ApplicationUpdateSecurityRequest request);

  $async.Future<$1.Empty> deleteSecurity_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ApplicationDeleteSecurityRequest> $request) async {
    return deleteSecurity($call, await $request);
  }

  $async.Future<$1.Empty> deleteSecurity(
      $grpc.ServiceCall call, $0.ApplicationDeleteSecurityRequest request);

  $async.Future<$0.ApplicationUpdateNetworksResponse> updateNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ApplicationUpdateNetworksRequest> $request) async {
    return updateNetworks($call, await $request);
  }

  $async.Future<$0.ApplicationUpdateNetworksResponse> updateNetworks(
      $grpc.ServiceCall call, $0.ApplicationUpdateNetworksRequest request);
}

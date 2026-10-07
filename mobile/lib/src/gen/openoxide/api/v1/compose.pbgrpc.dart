// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/compose.proto.

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
import 'compose.pb.dart' as $0;

export 'compose.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ComposeService')
class ComposeServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ComposeServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Compose> getCompose(
    $0.GetComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListComposesResponse> listComposes(
    $0.ListComposesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listComposes, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> createCompose(
    $0.CreateComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> updateCompose(
    $0.UpdateComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateCompose, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteCompose(
    $0.DeleteComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setGithubSource(
    $0.SetComposeSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGithubSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setGitlabSource(
    $0.SetComposeSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGitlabSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setBitbucketSource(
    $0.SetComposeSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setBitbucketSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setGiteaSource(
    $0.SetComposeSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGiteaSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setCustomGitSource(
    $0.SetComposeSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setCustomGitSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> setRawSource(
    $0.SetRawSourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setRawSource, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> uploadSource(
    $async.Stream<$0.UploadComposeRequest> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$uploadSource, request, options: options)
        .single;
  }

  $grpc.ResponseFuture<$0.ComposeOperation> deploy(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeOperation> redeploy(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$redeploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeOperation> reload(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reload, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeOperation> start(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$start, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeOperation> stop(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stop, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cancel(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancel, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> moveCompose(
    $0.MoveComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$moveCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeWebhookToken> rotateWebhookToken(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateWebhookToken, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeDependencies> listDependencies(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDependencies, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposePreview> previewCompose(
    $0.PreviewComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$previewCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> installTemplate(
    $0.InstallTemplateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$installTemplate, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeExport> exportCompose(
    $0.ExportComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportCompose, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> importCompose(
    $0.ImportComposeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$importCompose, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeServices(
    $0.ServiceMutationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeServices, request, options: options);
  }

  $grpc.ResponseFuture<$0.Compose> updateResources(
    $0.UpdateComposeResourcesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateResources, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> clearDeploymentHistory(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$clearDeploymentHistory, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cleanupDeploymentQueue(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cleanupDeploymentQueue, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> forceKillDeployment(
    $0.ComposeOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$forceKillDeployment, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeListMountsResponse> listMounts(
    $0.ComposeListMountsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMounts, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeCreateMountResponse> createMount(
    $0.ComposeCreateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeUpdateMountResponse> updateMount(
    $0.ComposeUpdateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateMount, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteMount(
    $0.ComposeMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeListPatchesResponse> listPatches(
    $0.ComposeListPatchesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPatches, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeCreatePatchResponse> createPatch(
    $0.ComposeCreatePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPatch, request, options: options);
  }

  $grpc.ResponseFuture<$0.ComposeUpdatePatchResponse> updatePatch(
    $0.ComposeUpdatePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePatch, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePatch(
    $0.ComposePatchRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePatch, request, options: options);
  }

  // method descriptors

  static final _$getCompose =
      $grpc.ClientMethod<$0.GetComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/GetCompose',
          ($0.GetComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$listComposes =
      $grpc.ClientMethod<$0.ListComposesRequest, $0.ListComposesResponse>(
          '/openoxide.api.v1.ComposeService/ListComposes',
          ($0.ListComposesRequest value) => value.writeToBuffer(),
          $0.ListComposesResponse.fromBuffer);
  static final _$createCompose =
      $grpc.ClientMethod<$0.CreateComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/CreateCompose',
          ($0.CreateComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$updateCompose =
      $grpc.ClientMethod<$0.UpdateComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/UpdateCompose',
          ($0.UpdateComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$deleteCompose =
      $grpc.ClientMethod<$0.DeleteComposeRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/DeleteCompose',
          ($0.DeleteComposeRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$setGithubSource =
      $grpc.ClientMethod<$0.SetComposeSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetGithubSource',
          ($0.SetComposeSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$setGitlabSource =
      $grpc.ClientMethod<$0.SetComposeSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetGitlabSource',
          ($0.SetComposeSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$setBitbucketSource =
      $grpc.ClientMethod<$0.SetComposeSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetBitbucketSource',
          ($0.SetComposeSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$setGiteaSource =
      $grpc.ClientMethod<$0.SetComposeSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetGiteaSource',
          ($0.SetComposeSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$setCustomGitSource =
      $grpc.ClientMethod<$0.SetComposeSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetCustomGitSource',
          ($0.SetComposeSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$setRawSource =
      $grpc.ClientMethod<$0.SetRawSourceRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/SetRawSource',
          ($0.SetRawSourceRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$uploadSource =
      $grpc.ClientMethod<$0.UploadComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/UploadSource',
          ($0.UploadComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$deploy =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
          '/openoxide.api.v1.ComposeService/Deploy',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeOperation.fromBuffer);
  static final _$redeploy =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
          '/openoxide.api.v1.ComposeService/Redeploy',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeOperation.fromBuffer);
  static final _$reload =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
          '/openoxide.api.v1.ComposeService/Reload',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeOperation.fromBuffer);
  static final _$start =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
          '/openoxide.api.v1.ComposeService/Start',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeOperation.fromBuffer);
  static final _$stop =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
          '/openoxide.api.v1.ComposeService/Stop',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeOperation.fromBuffer);
  static final _$cancel =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/Cancel',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$moveCompose =
      $grpc.ClientMethod<$0.MoveComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/MoveCompose',
          ($0.MoveComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$rotateWebhookToken =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeWebhookToken>(
          '/openoxide.api.v1.ComposeService/RotateWebhookToken',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeWebhookToken.fromBuffer);
  static final _$listDependencies =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $0.ComposeDependencies>(
          '/openoxide.api.v1.ComposeService/ListDependencies',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $0.ComposeDependencies.fromBuffer);
  static final _$previewCompose =
      $grpc.ClientMethod<$0.PreviewComposeRequest, $0.ComposePreview>(
          '/openoxide.api.v1.ComposeService/PreviewCompose',
          ($0.PreviewComposeRequest value) => value.writeToBuffer(),
          $0.ComposePreview.fromBuffer);
  static final _$installTemplate =
      $grpc.ClientMethod<$0.InstallTemplateRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/InstallTemplate',
          ($0.InstallTemplateRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$exportCompose =
      $grpc.ClientMethod<$0.ExportComposeRequest, $0.ComposeExport>(
          '/openoxide.api.v1.ComposeService/ExportCompose',
          ($0.ExportComposeRequest value) => value.writeToBuffer(),
          $0.ComposeExport.fromBuffer);
  static final _$importCompose =
      $grpc.ClientMethod<$0.ImportComposeRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/ImportCompose',
          ($0.ImportComposeRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$removeServices =
      $grpc.ClientMethod<$0.ServiceMutationRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/RemoveServices',
          ($0.ServiceMutationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$updateResources =
      $grpc.ClientMethod<$0.UpdateComposeResourcesRequest, $0.Compose>(
          '/openoxide.api.v1.ComposeService/UpdateResources',
          ($0.UpdateComposeResourcesRequest value) => value.writeToBuffer(),
          $0.Compose.fromBuffer);
  static final _$clearDeploymentHistory =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/ClearDeploymentHistory',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$cleanupDeploymentQueue =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/CleanupDeploymentQueue',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$forceKillDeployment =
      $grpc.ClientMethod<$0.ComposeOperationRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/ForceKillDeployment',
          ($0.ComposeOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listMounts = $grpc.ClientMethod<$0.ComposeListMountsRequest,
          $0.ComposeListMountsResponse>(
      '/openoxide.api.v1.ComposeService/ListMounts',
      ($0.ComposeListMountsRequest value) => value.writeToBuffer(),
      $0.ComposeListMountsResponse.fromBuffer);
  static final _$createMount = $grpc.ClientMethod<$0.ComposeCreateMountRequest,
          $0.ComposeCreateMountResponse>(
      '/openoxide.api.v1.ComposeService/CreateMount',
      ($0.ComposeCreateMountRequest value) => value.writeToBuffer(),
      $0.ComposeCreateMountResponse.fromBuffer);
  static final _$updateMount = $grpc.ClientMethod<$0.ComposeUpdateMountRequest,
          $0.ComposeUpdateMountResponse>(
      '/openoxide.api.v1.ComposeService/UpdateMount',
      ($0.ComposeUpdateMountRequest value) => value.writeToBuffer(),
      $0.ComposeUpdateMountResponse.fromBuffer);
  static final _$deleteMount =
      $grpc.ClientMethod<$0.ComposeMountRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/DeleteMount',
          ($0.ComposeMountRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listPatches = $grpc.ClientMethod<$0.ComposeListPatchesRequest,
          $0.ComposeListPatchesResponse>(
      '/openoxide.api.v1.ComposeService/ListPatches',
      ($0.ComposeListPatchesRequest value) => value.writeToBuffer(),
      $0.ComposeListPatchesResponse.fromBuffer);
  static final _$createPatch = $grpc.ClientMethod<$0.ComposeCreatePatchRequest,
          $0.ComposeCreatePatchResponse>(
      '/openoxide.api.v1.ComposeService/CreatePatch',
      ($0.ComposeCreatePatchRequest value) => value.writeToBuffer(),
      $0.ComposeCreatePatchResponse.fromBuffer);
  static final _$updatePatch = $grpc.ClientMethod<$0.ComposeUpdatePatchRequest,
          $0.ComposeUpdatePatchResponse>(
      '/openoxide.api.v1.ComposeService/UpdatePatch',
      ($0.ComposeUpdatePatchRequest value) => value.writeToBuffer(),
      $0.ComposeUpdatePatchResponse.fromBuffer);
  static final _$deletePatch =
      $grpc.ClientMethod<$0.ComposePatchRequest, $1.Empty>(
          '/openoxide.api.v1.ComposeService/DeletePatch',
          ($0.ComposePatchRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ComposeService')
abstract class ComposeServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ComposeService';

  ComposeServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetComposeRequest, $0.Compose>(
        'GetCompose',
        getCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListComposesRequest, $0.ListComposesResponse>(
            'ListComposes',
            listComposes_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListComposesRequest.fromBuffer(value),
            ($0.ListComposesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateComposeRequest, $0.Compose>(
        'CreateCompose',
        createCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateComposeRequest, $0.Compose>(
        'UpdateCompose',
        updateCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteComposeRequest, $1.Empty>(
        'DeleteCompose',
        deleteCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteComposeRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetComposeSourceRequest, $0.Compose>(
        'SetGithubSource',
        setGithubSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetComposeSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetComposeSourceRequest, $0.Compose>(
        'SetGitlabSource',
        setGitlabSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetComposeSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetComposeSourceRequest, $0.Compose>(
        'SetBitbucketSource',
        setBitbucketSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetComposeSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetComposeSourceRequest, $0.Compose>(
        'SetGiteaSource',
        setGiteaSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetComposeSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetComposeSourceRequest, $0.Compose>(
        'SetCustomGitSource',
        setCustomGitSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetComposeSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetRawSourceRequest, $0.Compose>(
        'SetRawSource',
        setRawSource_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetRawSourceRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UploadComposeRequest, $0.Compose>(
        'UploadSource',
        uploadSource,
        true,
        false,
        ($core.List<$core.int> value) =>
            $0.UploadComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
            'Deploy',
            deploy_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
            'Redeploy',
            redeploy_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
            'Reload',
            reload_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
            'Start',
            start_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeOperation>(
            'Stop',
            stop_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeOperationRequest, $1.Empty>(
        'Cancel',
        cancel_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MoveComposeRequest, $0.Compose>(
        'MoveCompose',
        moveCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MoveComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeWebhookToken>(
            'RotateWebhookToken',
            rotateWebhookToken_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeWebhookToken value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ComposeOperationRequest, $0.ComposeDependencies>(
            'ListDependencies',
            listDependencies_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ComposeOperationRequest.fromBuffer(value),
            ($0.ComposeDependencies value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PreviewComposeRequest, $0.ComposePreview>(
        'PreviewCompose',
        previewCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PreviewComposeRequest.fromBuffer(value),
        ($0.ComposePreview value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.InstallTemplateRequest, $0.Compose>(
        'InstallTemplate',
        installTemplate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.InstallTemplateRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ExportComposeRequest, $0.ComposeExport>(
        'ExportCompose',
        exportCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ExportComposeRequest.fromBuffer(value),
        ($0.ComposeExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ImportComposeRequest, $0.Compose>(
        'ImportCompose',
        importCompose_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ImportComposeRequest.fromBuffer(value),
        ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServiceMutationRequest, $1.Empty>(
        'RemoveServices',
        removeServices_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServiceMutationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateComposeResourcesRequest, $0.Compose>(
            'UpdateResources',
            updateResources_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateComposeResourcesRequest.fromBuffer(value),
            ($0.Compose value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeOperationRequest, $1.Empty>(
        'ClearDeploymentHistory',
        clearDeploymentHistory_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeOperationRequest, $1.Empty>(
        'CleanupDeploymentQueue',
        cleanupDeploymentQueue_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeOperationRequest, $1.Empty>(
        'ForceKillDeployment',
        forceKillDeployment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeListMountsRequest,
            $0.ComposeListMountsResponse>(
        'ListMounts',
        listMounts_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeListMountsRequest.fromBuffer(value),
        ($0.ComposeListMountsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeCreateMountRequest,
            $0.ComposeCreateMountResponse>(
        'CreateMount',
        createMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeCreateMountRequest.fromBuffer(value),
        ($0.ComposeCreateMountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeUpdateMountRequest,
            $0.ComposeUpdateMountResponse>(
        'UpdateMount',
        updateMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeUpdateMountRequest.fromBuffer(value),
        ($0.ComposeUpdateMountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeMountRequest, $1.Empty>(
        'DeleteMount',
        deleteMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeMountRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeListPatchesRequest,
            $0.ComposeListPatchesResponse>(
        'ListPatches',
        listPatches_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeListPatchesRequest.fromBuffer(value),
        ($0.ComposeListPatchesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeCreatePatchRequest,
            $0.ComposeCreatePatchResponse>(
        'CreatePatch',
        createPatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeCreatePatchRequest.fromBuffer(value),
        ($0.ComposeCreatePatchResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposeUpdatePatchRequest,
            $0.ComposeUpdatePatchResponse>(
        'UpdatePatch',
        updatePatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposeUpdatePatchRequest.fromBuffer(value),
        ($0.ComposeUpdatePatchResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ComposePatchRequest, $1.Empty>(
        'DeletePatch',
        deletePatch_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ComposePatchRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.Compose> getCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetComposeRequest> $request) async {
    return getCompose($call, await $request);
  }

  $async.Future<$0.Compose> getCompose(
      $grpc.ServiceCall call, $0.GetComposeRequest request);

  $async.Future<$0.ListComposesResponse> listComposes_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListComposesRequest> $request) async {
    return listComposes($call, await $request);
  }

  $async.Future<$0.ListComposesResponse> listComposes(
      $grpc.ServiceCall call, $0.ListComposesRequest request);

  $async.Future<$0.Compose> createCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateComposeRequest> $request) async {
    return createCompose($call, await $request);
  }

  $async.Future<$0.Compose> createCompose(
      $grpc.ServiceCall call, $0.CreateComposeRequest request);

  $async.Future<$0.Compose> updateCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateComposeRequest> $request) async {
    return updateCompose($call, await $request);
  }

  $async.Future<$0.Compose> updateCompose(
      $grpc.ServiceCall call, $0.UpdateComposeRequest request);

  $async.Future<$1.Empty> deleteCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteComposeRequest> $request) async {
    return deleteCompose($call, await $request);
  }

  $async.Future<$1.Empty> deleteCompose(
      $grpc.ServiceCall call, $0.DeleteComposeRequest request);

  $async.Future<$0.Compose> setGithubSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetComposeSourceRequest> $request) async {
    return setGithubSource($call, await $request);
  }

  $async.Future<$0.Compose> setGithubSource(
      $grpc.ServiceCall call, $0.SetComposeSourceRequest request);

  $async.Future<$0.Compose> setGitlabSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetComposeSourceRequest> $request) async {
    return setGitlabSource($call, await $request);
  }

  $async.Future<$0.Compose> setGitlabSource(
      $grpc.ServiceCall call, $0.SetComposeSourceRequest request);

  $async.Future<$0.Compose> setBitbucketSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetComposeSourceRequest> $request) async {
    return setBitbucketSource($call, await $request);
  }

  $async.Future<$0.Compose> setBitbucketSource(
      $grpc.ServiceCall call, $0.SetComposeSourceRequest request);

  $async.Future<$0.Compose> setGiteaSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetComposeSourceRequest> $request) async {
    return setGiteaSource($call, await $request);
  }

  $async.Future<$0.Compose> setGiteaSource(
      $grpc.ServiceCall call, $0.SetComposeSourceRequest request);

  $async.Future<$0.Compose> setCustomGitSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetComposeSourceRequest> $request) async {
    return setCustomGitSource($call, await $request);
  }

  $async.Future<$0.Compose> setCustomGitSource(
      $grpc.ServiceCall call, $0.SetComposeSourceRequest request);

  $async.Future<$0.Compose> setRawSource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetRawSourceRequest> $request) async {
    return setRawSource($call, await $request);
  }

  $async.Future<$0.Compose> setRawSource(
      $grpc.ServiceCall call, $0.SetRawSourceRequest request);

  $async.Future<$0.Compose> uploadSource(
      $grpc.ServiceCall call, $async.Stream<$0.UploadComposeRequest> request);

  $async.Future<$0.ComposeOperation> deploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return deploy($call, await $request);
  }

  $async.Future<$0.ComposeOperation> deploy(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeOperation> redeploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return redeploy($call, await $request);
  }

  $async.Future<$0.ComposeOperation> redeploy(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeOperation> reload_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return reload($call, await $request);
  }

  $async.Future<$0.ComposeOperation> reload(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeOperation> start_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return start($call, await $request);
  }

  $async.Future<$0.ComposeOperation> start(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeOperation> stop_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return stop($call, await $request);
  }

  $async.Future<$0.ComposeOperation> stop(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$1.Empty> cancel_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return cancel($call, await $request);
  }

  $async.Future<$1.Empty> cancel(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.Compose> moveCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.MoveComposeRequest> $request) async {
    return moveCompose($call, await $request);
  }

  $async.Future<$0.Compose> moveCompose(
      $grpc.ServiceCall call, $0.MoveComposeRequest request);

  $async.Future<$0.ComposeWebhookToken> rotateWebhookToken_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return rotateWebhookToken($call, await $request);
  }

  $async.Future<$0.ComposeWebhookToken> rotateWebhookToken(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeDependencies> listDependencies_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return listDependencies($call, await $request);
  }

  $async.Future<$0.ComposeDependencies> listDependencies(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposePreview> previewCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PreviewComposeRequest> $request) async {
    return previewCompose($call, await $request);
  }

  $async.Future<$0.ComposePreview> previewCompose(
      $grpc.ServiceCall call, $0.PreviewComposeRequest request);

  $async.Future<$0.Compose> installTemplate_Pre($grpc.ServiceCall $call,
      $async.Future<$0.InstallTemplateRequest> $request) async {
    return installTemplate($call, await $request);
  }

  $async.Future<$0.Compose> installTemplate(
      $grpc.ServiceCall call, $0.InstallTemplateRequest request);

  $async.Future<$0.ComposeExport> exportCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExportComposeRequest> $request) async {
    return exportCompose($call, await $request);
  }

  $async.Future<$0.ComposeExport> exportCompose(
      $grpc.ServiceCall call, $0.ExportComposeRequest request);

  $async.Future<$0.Compose> importCompose_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ImportComposeRequest> $request) async {
    return importCompose($call, await $request);
  }

  $async.Future<$0.Compose> importCompose(
      $grpc.ServiceCall call, $0.ImportComposeRequest request);

  $async.Future<$1.Empty> removeServices_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServiceMutationRequest> $request) async {
    return removeServices($call, await $request);
  }

  $async.Future<$1.Empty> removeServices(
      $grpc.ServiceCall call, $0.ServiceMutationRequest request);

  $async.Future<$0.Compose> updateResources_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateComposeResourcesRequest> $request) async {
    return updateResources($call, await $request);
  }

  $async.Future<$0.Compose> updateResources(
      $grpc.ServiceCall call, $0.UpdateComposeResourcesRequest request);

  $async.Future<$1.Empty> clearDeploymentHistory_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return clearDeploymentHistory($call, await $request);
  }

  $async.Future<$1.Empty> clearDeploymentHistory(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$1.Empty> cleanupDeploymentQueue_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return cleanupDeploymentQueue($call, await $request);
  }

  $async.Future<$1.Empty> cleanupDeploymentQueue(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$1.Empty> forceKillDeployment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeOperationRequest> $request) async {
    return forceKillDeployment($call, await $request);
  }

  $async.Future<$1.Empty> forceKillDeployment(
      $grpc.ServiceCall call, $0.ComposeOperationRequest request);

  $async.Future<$0.ComposeListMountsResponse> listMounts_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeListMountsRequest> $request) async {
    return listMounts($call, await $request);
  }

  $async.Future<$0.ComposeListMountsResponse> listMounts(
      $grpc.ServiceCall call, $0.ComposeListMountsRequest request);

  $async.Future<$0.ComposeCreateMountResponse> createMount_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeCreateMountRequest> $request) async {
    return createMount($call, await $request);
  }

  $async.Future<$0.ComposeCreateMountResponse> createMount(
      $grpc.ServiceCall call, $0.ComposeCreateMountRequest request);

  $async.Future<$0.ComposeUpdateMountResponse> updateMount_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeUpdateMountRequest> $request) async {
    return updateMount($call, await $request);
  }

  $async.Future<$0.ComposeUpdateMountResponse> updateMount(
      $grpc.ServiceCall call, $0.ComposeUpdateMountRequest request);

  $async.Future<$1.Empty> deleteMount_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposeMountRequest> $request) async {
    return deleteMount($call, await $request);
  }

  $async.Future<$1.Empty> deleteMount(
      $grpc.ServiceCall call, $0.ComposeMountRequest request);

  $async.Future<$0.ComposeListPatchesResponse> listPatches_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeListPatchesRequest> $request) async {
    return listPatches($call, await $request);
  }

  $async.Future<$0.ComposeListPatchesResponse> listPatches(
      $grpc.ServiceCall call, $0.ComposeListPatchesRequest request);

  $async.Future<$0.ComposeCreatePatchResponse> createPatch_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeCreatePatchRequest> $request) async {
    return createPatch($call, await $request);
  }

  $async.Future<$0.ComposeCreatePatchResponse> createPatch(
      $grpc.ServiceCall call, $0.ComposeCreatePatchRequest request);

  $async.Future<$0.ComposeUpdatePatchResponse> updatePatch_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ComposeUpdatePatchRequest> $request) async {
    return updatePatch($call, await $request);
  }

  $async.Future<$0.ComposeUpdatePatchResponse> updatePatch(
      $grpc.ServiceCall call, $0.ComposeUpdatePatchRequest request);

  $async.Future<$1.Empty> deletePatch_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ComposePatchRequest> $request) async {
    return deletePatch($call, await $request);
  }

  $async.Future<$1.Empty> deletePatch(
      $grpc.ServiceCall call, $0.ComposePatchRequest request);
}

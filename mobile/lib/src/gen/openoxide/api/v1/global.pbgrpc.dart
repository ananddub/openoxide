// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/global.proto.

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
import 'global.pb.dart' as $0;

export 'global.pb.dart';

/// Sole Docker-management owner service; absorbs the former GlobalOperationsApiService and GlobalOperationsService surface; `server_id` selects the target engine, absent means the local Agent.
@$pb.GrpcServiceName('openoxide.api.v1.GlobalService')
class GlobalServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  GlobalServiceClient(super.channel, {super.options, super.interceptors});

  /// Client-streaming upload: exactly one header chunk first, then byte chunks.
  $grpc.ResponseFuture<$0.GlobalUploadFileResponse> uploadFile(
    $async.Stream<$0.GlobalUploadFileRequest> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$uploadFile, request, options: options).single;
  }

  $grpc.ResponseFuture<$0.GlobalDownloadFileResponse> downloadFile(
    $0.GlobalDownloadFileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$downloadFile, request, options: options);
  }

  /// Twin of UploadFile kept for method-name compatibility with the former GlobalOperationsService.
  $grpc.ResponseFuture<$0.GlobalUploadContainerFileResponse>
      uploadContainerFile(
    $async.Stream<$0.GlobalUploadContainerFileRequest> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$uploadContainerFile, request,
            options: options)
        .single;
  }

  $grpc.ResponseFuture<$0.GlobalDownloadContainerFileResponse>
      downloadContainerFile(
    $0.GlobalDownloadContainerFileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$downloadContainerFile, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalListContainersResponse> listContainers(
    $0.GlobalListContainersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listContainers, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalListImagesResponse> listImages(
    $0.GlobalListImagesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listImages, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalListNetworksResponse> listNetworks(
    $0.GlobalListNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNetworks, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalListVolumesResponse> listVolumes(
    $0.GlobalListVolumesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listVolumes, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalInspectContainerResponse> inspectContainer(
    $0.GlobalInspectContainerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$inspectContainer, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalInspectImageResponse> inspectImage(
    $0.GlobalInspectImageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$inspectImage, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalInspectNetworkResponse> inspectNetwork(
    $0.GlobalInspectNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$inspectNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalInspectVolumeResponse> inspectVolume(
    $0.GlobalInspectVolumeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$inspectVolume, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalContainerActionResponse> containerAction(
    $0.GlobalContainerActionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$containerAction, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeContainer(
    $0.GlobalRemoveContainerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeContainer, request, options: options);
  }

  /// Twin of RemoveContainer kept for method-name compatibility with the former GlobalOperationsService.
  $grpc.ResponseFuture<$1.Empty> deleteContainer(
    $0.GlobalDeleteContainerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteContainer, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalDiskUsageResponse> diskUsage(
    $0.GlobalDiskUsageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$diskUsage, request, options: options);
  }

  $grpc.ResponseFuture<$0.GlobalPruneResponse> prune(
    $0.GlobalPruneRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$prune, request, options: options);
  }

  // method descriptors

  static final _$uploadFile = $grpc.ClientMethod<$0.GlobalUploadFileRequest,
          $0.GlobalUploadFileResponse>(
      '/openoxide.api.v1.GlobalService/UploadFile',
      ($0.GlobalUploadFileRequest value) => value.writeToBuffer(),
      $0.GlobalUploadFileResponse.fromBuffer);
  static final _$downloadFile = $grpc.ClientMethod<$0.GlobalDownloadFileRequest,
          $0.GlobalDownloadFileResponse>(
      '/openoxide.api.v1.GlobalService/DownloadFile',
      ($0.GlobalDownloadFileRequest value) => value.writeToBuffer(),
      $0.GlobalDownloadFileResponse.fromBuffer);
  static final _$uploadContainerFile = $grpc.ClientMethod<
          $0.GlobalUploadContainerFileRequest,
          $0.GlobalUploadContainerFileResponse>(
      '/openoxide.api.v1.GlobalService/UploadContainerFile',
      ($0.GlobalUploadContainerFileRequest value) => value.writeToBuffer(),
      $0.GlobalUploadContainerFileResponse.fromBuffer);
  static final _$downloadContainerFile = $grpc.ClientMethod<
          $0.GlobalDownloadContainerFileRequest,
          $0.GlobalDownloadContainerFileResponse>(
      '/openoxide.api.v1.GlobalService/DownloadContainerFile',
      ($0.GlobalDownloadContainerFileRequest value) => value.writeToBuffer(),
      $0.GlobalDownloadContainerFileResponse.fromBuffer);
  static final _$listContainers = $grpc.ClientMethod<
          $0.GlobalListContainersRequest, $0.GlobalListContainersResponse>(
      '/openoxide.api.v1.GlobalService/ListContainers',
      ($0.GlobalListContainersRequest value) => value.writeToBuffer(),
      $0.GlobalListContainersResponse.fromBuffer);
  static final _$listImages = $grpc.ClientMethod<$0.GlobalListImagesRequest,
          $0.GlobalListImagesResponse>(
      '/openoxide.api.v1.GlobalService/ListImages',
      ($0.GlobalListImagesRequest value) => value.writeToBuffer(),
      $0.GlobalListImagesResponse.fromBuffer);
  static final _$listNetworks = $grpc.ClientMethod<$0.GlobalListNetworksRequest,
          $0.GlobalListNetworksResponse>(
      '/openoxide.api.v1.GlobalService/ListNetworks',
      ($0.GlobalListNetworksRequest value) => value.writeToBuffer(),
      $0.GlobalListNetworksResponse.fromBuffer);
  static final _$listVolumes = $grpc.ClientMethod<$0.GlobalListVolumesRequest,
          $0.GlobalListVolumesResponse>(
      '/openoxide.api.v1.GlobalService/ListVolumes',
      ($0.GlobalListVolumesRequest value) => value.writeToBuffer(),
      $0.GlobalListVolumesResponse.fromBuffer);
  static final _$inspectContainer = $grpc.ClientMethod<
          $0.GlobalInspectContainerRequest, $0.GlobalInspectContainerResponse>(
      '/openoxide.api.v1.GlobalService/InspectContainer',
      ($0.GlobalInspectContainerRequest value) => value.writeToBuffer(),
      $0.GlobalInspectContainerResponse.fromBuffer);
  static final _$inspectImage = $grpc.ClientMethod<$0.GlobalInspectImageRequest,
          $0.GlobalInspectImageResponse>(
      '/openoxide.api.v1.GlobalService/InspectImage',
      ($0.GlobalInspectImageRequest value) => value.writeToBuffer(),
      $0.GlobalInspectImageResponse.fromBuffer);
  static final _$inspectNetwork = $grpc.ClientMethod<
          $0.GlobalInspectNetworkRequest, $0.GlobalInspectNetworkResponse>(
      '/openoxide.api.v1.GlobalService/InspectNetwork',
      ($0.GlobalInspectNetworkRequest value) => value.writeToBuffer(),
      $0.GlobalInspectNetworkResponse.fromBuffer);
  static final _$inspectVolume = $grpc.ClientMethod<
          $0.GlobalInspectVolumeRequest, $0.GlobalInspectVolumeResponse>(
      '/openoxide.api.v1.GlobalService/InspectVolume',
      ($0.GlobalInspectVolumeRequest value) => value.writeToBuffer(),
      $0.GlobalInspectVolumeResponse.fromBuffer);
  static final _$containerAction = $grpc.ClientMethod<
          $0.GlobalContainerActionRequest, $0.GlobalContainerActionResponse>(
      '/openoxide.api.v1.GlobalService/ContainerAction',
      ($0.GlobalContainerActionRequest value) => value.writeToBuffer(),
      $0.GlobalContainerActionResponse.fromBuffer);
  static final _$removeContainer =
      $grpc.ClientMethod<$0.GlobalRemoveContainerRequest, $1.Empty>(
          '/openoxide.api.v1.GlobalService/RemoveContainer',
          ($0.GlobalRemoveContainerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteContainer =
      $grpc.ClientMethod<$0.GlobalDeleteContainerRequest, $1.Empty>(
          '/openoxide.api.v1.GlobalService/DeleteContainer',
          ($0.GlobalDeleteContainerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$diskUsage =
      $grpc.ClientMethod<$0.GlobalDiskUsageRequest, $0.GlobalDiskUsageResponse>(
          '/openoxide.api.v1.GlobalService/DiskUsage',
          ($0.GlobalDiskUsageRequest value) => value.writeToBuffer(),
          $0.GlobalDiskUsageResponse.fromBuffer);
  static final _$prune =
      $grpc.ClientMethod<$0.GlobalPruneRequest, $0.GlobalPruneResponse>(
          '/openoxide.api.v1.GlobalService/Prune',
          ($0.GlobalPruneRequest value) => value.writeToBuffer(),
          $0.GlobalPruneResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.GlobalService')
abstract class GlobalServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.GlobalService';

  GlobalServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GlobalUploadFileRequest,
            $0.GlobalUploadFileResponse>(
        'UploadFile',
        uploadFile,
        true,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalUploadFileRequest.fromBuffer(value),
        ($0.GlobalUploadFileResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalDownloadFileRequest,
            $0.GlobalDownloadFileResponse>(
        'DownloadFile',
        downloadFile_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalDownloadFileRequest.fromBuffer(value),
        ($0.GlobalDownloadFileResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalUploadContainerFileRequest,
            $0.GlobalUploadContainerFileResponse>(
        'UploadContainerFile',
        uploadContainerFile,
        true,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalUploadContainerFileRequest.fromBuffer(value),
        ($0.GlobalUploadContainerFileResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalDownloadContainerFileRequest,
            $0.GlobalDownloadContainerFileResponse>(
        'DownloadContainerFile',
        downloadContainerFile_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalDownloadContainerFileRequest.fromBuffer(value),
        ($0.GlobalDownloadContainerFileResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalListContainersRequest,
            $0.GlobalListContainersResponse>(
        'ListContainers',
        listContainers_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalListContainersRequest.fromBuffer(value),
        ($0.GlobalListContainersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalListImagesRequest,
            $0.GlobalListImagesResponse>(
        'ListImages',
        listImages_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalListImagesRequest.fromBuffer(value),
        ($0.GlobalListImagesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalListNetworksRequest,
            $0.GlobalListNetworksResponse>(
        'ListNetworks',
        listNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalListNetworksRequest.fromBuffer(value),
        ($0.GlobalListNetworksResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalListVolumesRequest,
            $0.GlobalListVolumesResponse>(
        'ListVolumes',
        listVolumes_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalListVolumesRequest.fromBuffer(value),
        ($0.GlobalListVolumesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalInspectContainerRequest,
            $0.GlobalInspectContainerResponse>(
        'InspectContainer',
        inspectContainer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalInspectContainerRequest.fromBuffer(value),
        ($0.GlobalInspectContainerResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalInspectImageRequest,
            $0.GlobalInspectImageResponse>(
        'InspectImage',
        inspectImage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalInspectImageRequest.fromBuffer(value),
        ($0.GlobalInspectImageResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalInspectNetworkRequest,
            $0.GlobalInspectNetworkResponse>(
        'InspectNetwork',
        inspectNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalInspectNetworkRequest.fromBuffer(value),
        ($0.GlobalInspectNetworkResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalInspectVolumeRequest,
            $0.GlobalInspectVolumeResponse>(
        'InspectVolume',
        inspectVolume_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalInspectVolumeRequest.fromBuffer(value),
        ($0.GlobalInspectVolumeResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalContainerActionRequest,
            $0.GlobalContainerActionResponse>(
        'ContainerAction',
        containerAction_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalContainerActionRequest.fromBuffer(value),
        ($0.GlobalContainerActionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalRemoveContainerRequest, $1.Empty>(
        'RemoveContainer',
        removeContainer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalRemoveContainerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalDeleteContainerRequest, $1.Empty>(
        'DeleteContainer',
        deleteContainer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalDeleteContainerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GlobalDiskUsageRequest,
            $0.GlobalDiskUsageResponse>(
        'DiskUsage',
        diskUsage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GlobalDiskUsageRequest.fromBuffer(value),
        ($0.GlobalDiskUsageResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GlobalPruneRequest, $0.GlobalPruneResponse>(
            'Prune',
            prune_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GlobalPruneRequest.fromBuffer(value),
            ($0.GlobalPruneResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.GlobalUploadFileResponse> uploadFile($grpc.ServiceCall call,
      $async.Stream<$0.GlobalUploadFileRequest> request);

  $async.Future<$0.GlobalDownloadFileResponse> downloadFile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalDownloadFileRequest> $request) async {
    return downloadFile($call, await $request);
  }

  $async.Future<$0.GlobalDownloadFileResponse> downloadFile(
      $grpc.ServiceCall call, $0.GlobalDownloadFileRequest request);

  $async.Future<$0.GlobalUploadContainerFileResponse> uploadContainerFile(
      $grpc.ServiceCall call,
      $async.Stream<$0.GlobalUploadContainerFileRequest> request);

  $async.Future<$0.GlobalDownloadContainerFileResponse>
      downloadContainerFile_Pre($grpc.ServiceCall $call,
          $async.Future<$0.GlobalDownloadContainerFileRequest> $request) async {
    return downloadContainerFile($call, await $request);
  }

  $async.Future<$0.GlobalDownloadContainerFileResponse> downloadContainerFile(
      $grpc.ServiceCall call, $0.GlobalDownloadContainerFileRequest request);

  $async.Future<$0.GlobalListContainersResponse> listContainers_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalListContainersRequest> $request) async {
    return listContainers($call, await $request);
  }

  $async.Future<$0.GlobalListContainersResponse> listContainers(
      $grpc.ServiceCall call, $0.GlobalListContainersRequest request);

  $async.Future<$0.GlobalListImagesResponse> listImages_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalListImagesRequest> $request) async {
    return listImages($call, await $request);
  }

  $async.Future<$0.GlobalListImagesResponse> listImages(
      $grpc.ServiceCall call, $0.GlobalListImagesRequest request);

  $async.Future<$0.GlobalListNetworksResponse> listNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalListNetworksRequest> $request) async {
    return listNetworks($call, await $request);
  }

  $async.Future<$0.GlobalListNetworksResponse> listNetworks(
      $grpc.ServiceCall call, $0.GlobalListNetworksRequest request);

  $async.Future<$0.GlobalListVolumesResponse> listVolumes_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalListVolumesRequest> $request) async {
    return listVolumes($call, await $request);
  }

  $async.Future<$0.GlobalListVolumesResponse> listVolumes(
      $grpc.ServiceCall call, $0.GlobalListVolumesRequest request);

  $async.Future<$0.GlobalInspectContainerResponse> inspectContainer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalInspectContainerRequest> $request) async {
    return inspectContainer($call, await $request);
  }

  $async.Future<$0.GlobalInspectContainerResponse> inspectContainer(
      $grpc.ServiceCall call, $0.GlobalInspectContainerRequest request);

  $async.Future<$0.GlobalInspectImageResponse> inspectImage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalInspectImageRequest> $request) async {
    return inspectImage($call, await $request);
  }

  $async.Future<$0.GlobalInspectImageResponse> inspectImage(
      $grpc.ServiceCall call, $0.GlobalInspectImageRequest request);

  $async.Future<$0.GlobalInspectNetworkResponse> inspectNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalInspectNetworkRequest> $request) async {
    return inspectNetwork($call, await $request);
  }

  $async.Future<$0.GlobalInspectNetworkResponse> inspectNetwork(
      $grpc.ServiceCall call, $0.GlobalInspectNetworkRequest request);

  $async.Future<$0.GlobalInspectVolumeResponse> inspectVolume_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalInspectVolumeRequest> $request) async {
    return inspectVolume($call, await $request);
  }

  $async.Future<$0.GlobalInspectVolumeResponse> inspectVolume(
      $grpc.ServiceCall call, $0.GlobalInspectVolumeRequest request);

  $async.Future<$0.GlobalContainerActionResponse> containerAction_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalContainerActionRequest> $request) async {
    return containerAction($call, await $request);
  }

  $async.Future<$0.GlobalContainerActionResponse> containerAction(
      $grpc.ServiceCall call, $0.GlobalContainerActionRequest request);

  $async.Future<$1.Empty> removeContainer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GlobalRemoveContainerRequest> $request) async {
    return removeContainer($call, await $request);
  }

  $async.Future<$1.Empty> removeContainer(
      $grpc.ServiceCall call, $0.GlobalRemoveContainerRequest request);

  $async.Future<$1.Empty> deleteContainer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GlobalDeleteContainerRequest> $request) async {
    return deleteContainer($call, await $request);
  }

  $async.Future<$1.Empty> deleteContainer(
      $grpc.ServiceCall call, $0.GlobalDeleteContainerRequest request);

  $async.Future<$0.GlobalDiskUsageResponse> diskUsage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GlobalDiskUsageRequest> $request) async {
    return diskUsage($call, await $request);
  }

  $async.Future<$0.GlobalDiskUsageResponse> diskUsage(
      $grpc.ServiceCall call, $0.GlobalDiskUsageRequest request);

  $async.Future<$0.GlobalPruneResponse> prune_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GlobalPruneRequest> $request) async {
    return prune($call, await $request);
  }

  $async.Future<$0.GlobalPruneResponse> prune(
      $grpc.ServiceCall call, $0.GlobalPruneRequest request);
}

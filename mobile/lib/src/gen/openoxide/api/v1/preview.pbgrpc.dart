// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/preview.proto.

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
import 'preview.pb.dart' as $0;

export 'preview.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.PreviewDeploymentService')
class PreviewDeploymentServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PreviewDeploymentServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListPreviewsResponse> listPreviews(
    $0.ListPreviewsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPreviews, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetPreviewResponse> getPreview(
    $0.GetPreviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPreview, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreatePreviewResponse> createPreview(
    $0.CreatePreviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPreview, request, options: options);
  }

  $grpc.ResponseFuture<$0.RedeployPreviewResponse> redeployPreview(
    $0.RedeployPreviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$redeployPreview, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePreview(
    $0.DeletePreviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePreview, request, options: options);
  }

  // method descriptors

  static final _$listPreviews =
      $grpc.ClientMethod<$0.ListPreviewsRequest, $0.ListPreviewsResponse>(
          '/openoxide.api.v1.PreviewDeploymentService/ListPreviews',
          ($0.ListPreviewsRequest value) => value.writeToBuffer(),
          $0.ListPreviewsResponse.fromBuffer);
  static final _$getPreview =
      $grpc.ClientMethod<$0.GetPreviewRequest, $0.GetPreviewResponse>(
          '/openoxide.api.v1.PreviewDeploymentService/GetPreview',
          ($0.GetPreviewRequest value) => value.writeToBuffer(),
          $0.GetPreviewResponse.fromBuffer);
  static final _$createPreview =
      $grpc.ClientMethod<$0.CreatePreviewRequest, $0.CreatePreviewResponse>(
          '/openoxide.api.v1.PreviewDeploymentService/CreatePreview',
          ($0.CreatePreviewRequest value) => value.writeToBuffer(),
          $0.CreatePreviewResponse.fromBuffer);
  static final _$redeployPreview =
      $grpc.ClientMethod<$0.RedeployPreviewRequest, $0.RedeployPreviewResponse>(
          '/openoxide.api.v1.PreviewDeploymentService/RedeployPreview',
          ($0.RedeployPreviewRequest value) => value.writeToBuffer(),
          $0.RedeployPreviewResponse.fromBuffer);
  static final _$deletePreview =
      $grpc.ClientMethod<$0.DeletePreviewRequest, $1.Empty>(
          '/openoxide.api.v1.PreviewDeploymentService/DeletePreview',
          ($0.DeletePreviewRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.PreviewDeploymentService')
abstract class PreviewDeploymentServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.PreviewDeploymentService';

  PreviewDeploymentServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.ListPreviewsRequest, $0.ListPreviewsResponse>(
            'ListPreviews',
            listPreviews_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListPreviewsRequest.fromBuffer(value),
            ($0.ListPreviewsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetPreviewRequest, $0.GetPreviewResponse>(
        'GetPreview',
        getPreview_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetPreviewRequest.fromBuffer(value),
        ($0.GetPreviewResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreatePreviewRequest, $0.CreatePreviewResponse>(
            'CreatePreview',
            createPreview_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreatePreviewRequest.fromBuffer(value),
            ($0.CreatePreviewResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RedeployPreviewRequest,
            $0.RedeployPreviewResponse>(
        'RedeployPreview',
        redeployPreview_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RedeployPreviewRequest.fromBuffer(value),
        ($0.RedeployPreviewResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeletePreviewRequest, $1.Empty>(
        'DeletePreview',
        deletePreview_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeletePreviewRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListPreviewsResponse> listPreviews_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListPreviewsRequest> $request) async {
    return listPreviews($call, await $request);
  }

  $async.Future<$0.ListPreviewsResponse> listPreviews(
      $grpc.ServiceCall call, $0.ListPreviewsRequest request);

  $async.Future<$0.GetPreviewResponse> getPreview_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetPreviewRequest> $request) async {
    return getPreview($call, await $request);
  }

  $async.Future<$0.GetPreviewResponse> getPreview(
      $grpc.ServiceCall call, $0.GetPreviewRequest request);

  $async.Future<$0.CreatePreviewResponse> createPreview_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreatePreviewRequest> $request) async {
    return createPreview($call, await $request);
  }

  $async.Future<$0.CreatePreviewResponse> createPreview(
      $grpc.ServiceCall call, $0.CreatePreviewRequest request);

  $async.Future<$0.RedeployPreviewResponse> redeployPreview_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RedeployPreviewRequest> $request) async {
    return redeployPreview($call, await $request);
  }

  $async.Future<$0.RedeployPreviewResponse> redeployPreview(
      $grpc.ServiceCall call, $0.RedeployPreviewRequest request);

  $async.Future<$1.Empty> deletePreview_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeletePreviewRequest> $request) async {
    return deletePreview($call, await $request);
  }

  $async.Future<$1.Empty> deletePreview(
      $grpc.ServiceCall call, $0.DeletePreviewRequest request);
}

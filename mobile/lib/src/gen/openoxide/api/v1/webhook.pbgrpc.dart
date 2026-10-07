// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/webhook.proto.

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
import 'webhook.pb.dart' as $0;

export 'webhook.pb.dart';

/// Management surface for repository webhooks (the receive side lives in IncomingWebhookService.HandleWebhook).
@$pb.GrpcServiceName('openoxide.api.v1.WebhookService')
class WebhookServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  WebhookServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListWebhooksResponse> listWebhooks(
    $0.ListWebhooksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listWebhooks, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetWebhookResponse> getWebhook(
    $0.GetWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateWebhookResponse> createWebhook(
    $0.CreateWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateWebhookResponse> updateWebhook(
    $0.UpdateWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteWebhook(
    $0.DeleteWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteWebhook, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestWebhookResponse> testWebhook(
    $0.TestWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testWebhook, request, options: options);
  }

  // method descriptors

  static final _$listWebhooks =
      $grpc.ClientMethod<$0.ListWebhooksRequest, $0.ListWebhooksResponse>(
          '/openoxide.api.v1.WebhookService/ListWebhooks',
          ($0.ListWebhooksRequest value) => value.writeToBuffer(),
          $0.ListWebhooksResponse.fromBuffer);
  static final _$getWebhook =
      $grpc.ClientMethod<$0.GetWebhookRequest, $0.GetWebhookResponse>(
          '/openoxide.api.v1.WebhookService/GetWebhook',
          ($0.GetWebhookRequest value) => value.writeToBuffer(),
          $0.GetWebhookResponse.fromBuffer);
  static final _$createWebhook =
      $grpc.ClientMethod<$0.CreateWebhookRequest, $0.CreateWebhookResponse>(
          '/openoxide.api.v1.WebhookService/CreateWebhook',
          ($0.CreateWebhookRequest value) => value.writeToBuffer(),
          $0.CreateWebhookResponse.fromBuffer);
  static final _$updateWebhook =
      $grpc.ClientMethod<$0.UpdateWebhookRequest, $0.UpdateWebhookResponse>(
          '/openoxide.api.v1.WebhookService/UpdateWebhook',
          ($0.UpdateWebhookRequest value) => value.writeToBuffer(),
          $0.UpdateWebhookResponse.fromBuffer);
  static final _$deleteWebhook =
      $grpc.ClientMethod<$0.DeleteWebhookRequest, $1.Empty>(
          '/openoxide.api.v1.WebhookService/DeleteWebhook',
          ($0.DeleteWebhookRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testWebhook =
      $grpc.ClientMethod<$0.TestWebhookRequest, $0.TestWebhookResponse>(
          '/openoxide.api.v1.WebhookService/TestWebhook',
          ($0.TestWebhookRequest value) => value.writeToBuffer(),
          $0.TestWebhookResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.WebhookService')
abstract class WebhookServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.WebhookService';

  WebhookServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.ListWebhooksRequest, $0.ListWebhooksResponse>(
            'ListWebhooks',
            listWebhooks_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListWebhooksRequest.fromBuffer(value),
            ($0.ListWebhooksResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetWebhookRequest, $0.GetWebhookResponse>(
        'GetWebhook',
        getWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetWebhookRequest.fromBuffer(value),
        ($0.GetWebhookResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreateWebhookRequest, $0.CreateWebhookResponse>(
            'CreateWebhook',
            createWebhook_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateWebhookRequest.fromBuffer(value),
            ($0.CreateWebhookResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateWebhookRequest, $0.UpdateWebhookResponse>(
            'UpdateWebhook',
            updateWebhook_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateWebhookRequest.fromBuffer(value),
            ($0.UpdateWebhookResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteWebhookRequest, $1.Empty>(
        'DeleteWebhook',
        deleteWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteWebhookRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TestWebhookRequest, $0.TestWebhookResponse>(
            'TestWebhook',
            testWebhook_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TestWebhookRequest.fromBuffer(value),
            ($0.TestWebhookResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListWebhooksResponse> listWebhooks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListWebhooksRequest> $request) async {
    return listWebhooks($call, await $request);
  }

  $async.Future<$0.ListWebhooksResponse> listWebhooks(
      $grpc.ServiceCall call, $0.ListWebhooksRequest request);

  $async.Future<$0.GetWebhookResponse> getWebhook_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetWebhookRequest> $request) async {
    return getWebhook($call, await $request);
  }

  $async.Future<$0.GetWebhookResponse> getWebhook(
      $grpc.ServiceCall call, $0.GetWebhookRequest request);

  $async.Future<$0.CreateWebhookResponse> createWebhook_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateWebhookRequest> $request) async {
    return createWebhook($call, await $request);
  }

  $async.Future<$0.CreateWebhookResponse> createWebhook(
      $grpc.ServiceCall call, $0.CreateWebhookRequest request);

  $async.Future<$0.UpdateWebhookResponse> updateWebhook_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateWebhookRequest> $request) async {
    return updateWebhook($call, await $request);
  }

  $async.Future<$0.UpdateWebhookResponse> updateWebhook(
      $grpc.ServiceCall call, $0.UpdateWebhookRequest request);

  $async.Future<$1.Empty> deleteWebhook_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteWebhookRequest> $request) async {
    return deleteWebhook($call, await $request);
  }

  $async.Future<$1.Empty> deleteWebhook(
      $grpc.ServiceCall call, $0.DeleteWebhookRequest request);

  $async.Future<$0.TestWebhookResponse> testWebhook_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TestWebhookRequest> $request) async {
    return testWebhook($call, await $request);
  }

  $async.Future<$0.TestWebhookResponse> testWebhook(
      $grpc.ServiceCall call, $0.TestWebhookRequest request);
}

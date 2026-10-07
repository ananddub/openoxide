// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/incoming_webhook.proto.

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

import 'incoming_webhook.pb.dart' as $0;

export 'incoming_webhook.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.IncomingWebhookService')
class IncomingWebhookServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  IncomingWebhookServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.IncomingWebhookResponse> handleWebhook(
    $0.IncomingWebhookRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$handleWebhook, request, options: options);
  }

  // method descriptors

  static final _$handleWebhook =
      $grpc.ClientMethod<$0.IncomingWebhookRequest, $0.IncomingWebhookResponse>(
          '/openoxide.api.v1.IncomingWebhookService/HandleWebhook',
          ($0.IncomingWebhookRequest value) => value.writeToBuffer(),
          $0.IncomingWebhookResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.IncomingWebhookService')
abstract class IncomingWebhookServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.IncomingWebhookService';

  IncomingWebhookServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.IncomingWebhookRequest,
            $0.IncomingWebhookResponse>(
        'HandleWebhook',
        handleWebhook_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.IncomingWebhookRequest.fromBuffer(value),
        ($0.IncomingWebhookResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.IncomingWebhookResponse> handleWebhook_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.IncomingWebhookRequest> $request) async {
    return handleWebhook($call, await $request);
  }

  $async.Future<$0.IncomingWebhookResponse> handleWebhook(
      $grpc.ServiceCall call, $0.IncomingWebhookRequest request);
}

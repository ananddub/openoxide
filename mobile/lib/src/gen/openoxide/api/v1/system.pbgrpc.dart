// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/system.proto.

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

import 'system.pb.dart' as $0;

export 'system.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.SystemService')
class SystemServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SystemServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.SystemHealthResponse> health(
    $0.SystemHealthRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$health, request, options: options);
  }

  $grpc.ResponseFuture<$0.SystemTestConfigResponse> testConfig(
    $0.SystemTestConfigRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConfig, request, options: options);
  }

  // method descriptors

  static final _$health =
      $grpc.ClientMethod<$0.SystemHealthRequest, $0.SystemHealthResponse>(
          '/openoxide.api.v1.SystemService/Health',
          ($0.SystemHealthRequest value) => value.writeToBuffer(),
          $0.SystemHealthResponse.fromBuffer);
  static final _$testConfig = $grpc.ClientMethod<$0.SystemTestConfigRequest,
          $0.SystemTestConfigResponse>(
      '/openoxide.api.v1.SystemService/TestConfig',
      ($0.SystemTestConfigRequest value) => value.writeToBuffer(),
      $0.SystemTestConfigResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.SystemService')
abstract class SystemServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.SystemService';

  SystemServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.SystemHealthRequest, $0.SystemHealthResponse>(
            'Health',
            health_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SystemHealthRequest.fromBuffer(value),
            ($0.SystemHealthResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SystemTestConfigRequest,
            $0.SystemTestConfigResponse>(
        'TestConfig',
        testConfig_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SystemTestConfigRequest.fromBuffer(value),
        ($0.SystemTestConfigResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.SystemHealthResponse> health_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SystemHealthRequest> $request) async {
    return health($call, await $request);
  }

  $async.Future<$0.SystemHealthResponse> health(
      $grpc.ServiceCall call, $0.SystemHealthRequest request);

  $async.Future<$0.SystemTestConfigResponse> testConfig_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SystemTestConfigRequest> $request) async {
    return testConfig($call, await $request);
  }

  $async.Future<$0.SystemTestConfigResponse> testConfig(
      $grpc.ServiceCall call, $0.SystemTestConfigRequest request);
}

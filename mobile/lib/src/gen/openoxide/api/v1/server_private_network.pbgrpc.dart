// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_private_network.proto.

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
import 'server_private_network.pb.dart' as $0;

export 'server_private_network.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ServerPrivateNetworkService')
class ServerPrivateNetworkServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ServerPrivateNetworkServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ServerPrivateNetworkGetResponse> getNetwork(
    $0.ServerPrivateNetworkGetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkUpdateResponse> updateNetwork(
    $0.ServerPrivateNetworkUpdateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteNetwork(
    $0.ServerPrivateNetworkDeleteRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkSetupResponse> setup(
    $0.ServerPrivateNetworkSetupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setup, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkHealthResponse> health(
    $0.ServerPrivateNetworkHealthRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$health, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkRepairResponse> repair(
    $0.ServerPrivateNetworkRepairRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$repair, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkReSetupResponse> reSetup(
    $0.ServerPrivateNetworkReSetupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reSetup, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerPrivateNetworkRotateKeysResponse> rotateKeys(
    $0.ServerPrivateNetworkRotateKeysRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateKeys, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> teardown(
    $0.ServerPrivateNetworkTeardownRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$teardown, request, options: options);
  }

  // method descriptors

  static final _$getNetwork = $grpc.ClientMethod<
          $0.ServerPrivateNetworkGetRequest,
          $0.ServerPrivateNetworkGetResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/GetNetwork',
      ($0.ServerPrivateNetworkGetRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkGetResponse.fromBuffer);
  static final _$updateNetwork = $grpc.ClientMethod<
          $0.ServerPrivateNetworkUpdateRequest,
          $0.ServerPrivateNetworkUpdateResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/UpdateNetwork',
      ($0.ServerPrivateNetworkUpdateRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkUpdateResponse.fromBuffer);
  static final _$deleteNetwork =
      $grpc.ClientMethod<$0.ServerPrivateNetworkDeleteRequest, $1.Empty>(
          '/openoxide.api.v1.ServerPrivateNetworkService/DeleteNetwork',
          ($0.ServerPrivateNetworkDeleteRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$setup = $grpc.ClientMethod<$0.ServerPrivateNetworkSetupRequest,
          $0.ServerPrivateNetworkSetupResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/Setup',
      ($0.ServerPrivateNetworkSetupRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkSetupResponse.fromBuffer);
  static final _$health = $grpc.ClientMethod<
          $0.ServerPrivateNetworkHealthRequest,
          $0.ServerPrivateNetworkHealthResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/Health',
      ($0.ServerPrivateNetworkHealthRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkHealthResponse.fromBuffer);
  static final _$repair = $grpc.ClientMethod<
          $0.ServerPrivateNetworkRepairRequest,
          $0.ServerPrivateNetworkRepairResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/Repair',
      ($0.ServerPrivateNetworkRepairRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkRepairResponse.fromBuffer);
  static final _$reSetup = $grpc.ClientMethod<
          $0.ServerPrivateNetworkReSetupRequest,
          $0.ServerPrivateNetworkReSetupResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/ReSetup',
      ($0.ServerPrivateNetworkReSetupRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkReSetupResponse.fromBuffer);
  static final _$rotateKeys = $grpc.ClientMethod<
          $0.ServerPrivateNetworkRotateKeysRequest,
          $0.ServerPrivateNetworkRotateKeysResponse>(
      '/openoxide.api.v1.ServerPrivateNetworkService/RotateKeys',
      ($0.ServerPrivateNetworkRotateKeysRequest value) => value.writeToBuffer(),
      $0.ServerPrivateNetworkRotateKeysResponse.fromBuffer);
  static final _$teardown =
      $grpc.ClientMethod<$0.ServerPrivateNetworkTeardownRequest, $1.Empty>(
          '/openoxide.api.v1.ServerPrivateNetworkService/Teardown',
          ($0.ServerPrivateNetworkTeardownRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ServerPrivateNetworkService')
abstract class ServerPrivateNetworkServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ServerPrivateNetworkService';

  ServerPrivateNetworkServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkGetRequest,
            $0.ServerPrivateNetworkGetResponse>(
        'GetNetwork',
        getNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkGetRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkGetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkUpdateRequest,
            $0.ServerPrivateNetworkUpdateResponse>(
        'UpdateNetwork',
        updateNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkUpdateRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkUpdateResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ServerPrivateNetworkDeleteRequest, $1.Empty>(
            'DeleteNetwork',
            deleteNetwork_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ServerPrivateNetworkDeleteRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkSetupRequest,
            $0.ServerPrivateNetworkSetupResponse>(
        'Setup',
        setup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkSetupRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkSetupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkHealthRequest,
            $0.ServerPrivateNetworkHealthResponse>(
        'Health',
        health_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkHealthRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkHealthResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkRepairRequest,
            $0.ServerPrivateNetworkRepairResponse>(
        'Repair',
        repair_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkRepairRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkRepairResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkReSetupRequest,
            $0.ServerPrivateNetworkReSetupResponse>(
        'ReSetup',
        reSetup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkReSetupRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkReSetupResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerPrivateNetworkRotateKeysRequest,
            $0.ServerPrivateNetworkRotateKeysResponse>(
        'RotateKeys',
        rotateKeys_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerPrivateNetworkRotateKeysRequest.fromBuffer(value),
        ($0.ServerPrivateNetworkRotateKeysResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ServerPrivateNetworkTeardownRequest, $1.Empty>(
            'Teardown',
            teardown_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ServerPrivateNetworkTeardownRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ServerPrivateNetworkGetResponse> getNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkGetRequest> $request) async {
    return getNetwork($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkGetResponse> getNetwork(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkGetRequest request);

  $async.Future<$0.ServerPrivateNetworkUpdateResponse> updateNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkUpdateRequest> $request) async {
    return updateNetwork($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkUpdateResponse> updateNetwork(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkUpdateRequest request);

  $async.Future<$1.Empty> deleteNetwork_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkDeleteRequest> $request) async {
    return deleteNetwork($call, await $request);
  }

  $async.Future<$1.Empty> deleteNetwork(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkDeleteRequest request);

  $async.Future<$0.ServerPrivateNetworkSetupResponse> setup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkSetupRequest> $request) async {
    return setup($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkSetupResponse> setup(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkSetupRequest request);

  $async.Future<$0.ServerPrivateNetworkHealthResponse> health_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkHealthRequest> $request) async {
    return health($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkHealthResponse> health(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkHealthRequest request);

  $async.Future<$0.ServerPrivateNetworkRepairResponse> repair_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkRepairRequest> $request) async {
    return repair($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkRepairResponse> repair(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkRepairRequest request);

  $async.Future<$0.ServerPrivateNetworkReSetupResponse> reSetup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkReSetupRequest> $request) async {
    return reSetup($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkReSetupResponse> reSetup(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkReSetupRequest request);

  $async.Future<$0.ServerPrivateNetworkRotateKeysResponse> rotateKeys_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkRotateKeysRequest> $request) async {
    return rotateKeys($call, await $request);
  }

  $async.Future<$0.ServerPrivateNetworkRotateKeysResponse> rotateKeys(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkRotateKeysRequest request);

  $async.Future<$1.Empty> teardown_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerPrivateNetworkTeardownRequest> $request) async {
    return teardown($call, await $request);
  }

  $async.Future<$1.Empty> teardown(
      $grpc.ServiceCall call, $0.ServerPrivateNetworkTeardownRequest request);
}

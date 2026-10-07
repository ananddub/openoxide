// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/environment.proto.

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
import 'environment.pb.dart' as $0;

export 'environment.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.EnvironmentService')
class EnvironmentServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  EnvironmentServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Environment> getEnvironment(
    $0.GetEnvironmentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getEnvironment, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListEnvironmentsResponse> listEnvironments(
    $0.ListEnvironmentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listEnvironments, request, options: options);
  }

  $grpc.ResponseFuture<$0.Environment> createEnvironment(
    $0.CreateEnvironmentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createEnvironment, request, options: options);
  }

  $grpc.ResponseFuture<$0.Environment> updateEnvironment(
    $0.UpdateEnvironmentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateEnvironment, request, options: options);
  }

  $grpc.ResponseFuture<$0.Environment> setDefaultEnvironment(
    $0.SetDefaultEnvironmentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setDefaultEnvironment, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteEnvironment(
    $0.DeleteEnvironmentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteEnvironment, request, options: options);
  }

  // method descriptors

  static final _$getEnvironment =
      $grpc.ClientMethod<$0.GetEnvironmentRequest, $0.Environment>(
          '/openoxide.api.v1.EnvironmentService/GetEnvironment',
          ($0.GetEnvironmentRequest value) => value.writeToBuffer(),
          $0.Environment.fromBuffer);
  static final _$listEnvironments = $grpc.ClientMethod<
          $0.ListEnvironmentsRequest, $0.ListEnvironmentsResponse>(
      '/openoxide.api.v1.EnvironmentService/ListEnvironments',
      ($0.ListEnvironmentsRequest value) => value.writeToBuffer(),
      $0.ListEnvironmentsResponse.fromBuffer);
  static final _$createEnvironment =
      $grpc.ClientMethod<$0.CreateEnvironmentRequest, $0.Environment>(
          '/openoxide.api.v1.EnvironmentService/CreateEnvironment',
          ($0.CreateEnvironmentRequest value) => value.writeToBuffer(),
          $0.Environment.fromBuffer);
  static final _$updateEnvironment =
      $grpc.ClientMethod<$0.UpdateEnvironmentRequest, $0.Environment>(
          '/openoxide.api.v1.EnvironmentService/UpdateEnvironment',
          ($0.UpdateEnvironmentRequest value) => value.writeToBuffer(),
          $0.Environment.fromBuffer);
  static final _$setDefaultEnvironment =
      $grpc.ClientMethod<$0.SetDefaultEnvironmentRequest, $0.Environment>(
          '/openoxide.api.v1.EnvironmentService/SetDefaultEnvironment',
          ($0.SetDefaultEnvironmentRequest value) => value.writeToBuffer(),
          $0.Environment.fromBuffer);
  static final _$deleteEnvironment =
      $grpc.ClientMethod<$0.DeleteEnvironmentRequest, $1.Empty>(
          '/openoxide.api.v1.EnvironmentService/DeleteEnvironment',
          ($0.DeleteEnvironmentRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.EnvironmentService')
abstract class EnvironmentServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.EnvironmentService';

  EnvironmentServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetEnvironmentRequest, $0.Environment>(
        'GetEnvironment',
        getEnvironment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetEnvironmentRequest.fromBuffer(value),
        ($0.Environment value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListEnvironmentsRequest,
            $0.ListEnvironmentsResponse>(
        'ListEnvironments',
        listEnvironments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListEnvironmentsRequest.fromBuffer(value),
        ($0.ListEnvironmentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateEnvironmentRequest, $0.Environment>(
        'CreateEnvironment',
        createEnvironment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateEnvironmentRequest.fromBuffer(value),
        ($0.Environment value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateEnvironmentRequest, $0.Environment>(
        'UpdateEnvironment',
        updateEnvironment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateEnvironmentRequest.fromBuffer(value),
        ($0.Environment value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SetDefaultEnvironmentRequest, $0.Environment>(
            'SetDefaultEnvironment',
            setDefaultEnvironment_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SetDefaultEnvironmentRequest.fromBuffer(value),
            ($0.Environment value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteEnvironmentRequest, $1.Empty>(
        'DeleteEnvironment',
        deleteEnvironment_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteEnvironmentRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.Environment> getEnvironment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetEnvironmentRequest> $request) async {
    return getEnvironment($call, await $request);
  }

  $async.Future<$0.Environment> getEnvironment(
      $grpc.ServiceCall call, $0.GetEnvironmentRequest request);

  $async.Future<$0.ListEnvironmentsResponse> listEnvironments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListEnvironmentsRequest> $request) async {
    return listEnvironments($call, await $request);
  }

  $async.Future<$0.ListEnvironmentsResponse> listEnvironments(
      $grpc.ServiceCall call, $0.ListEnvironmentsRequest request);

  $async.Future<$0.Environment> createEnvironment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateEnvironmentRequest> $request) async {
    return createEnvironment($call, await $request);
  }

  $async.Future<$0.Environment> createEnvironment(
      $grpc.ServiceCall call, $0.CreateEnvironmentRequest request);

  $async.Future<$0.Environment> updateEnvironment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateEnvironmentRequest> $request) async {
    return updateEnvironment($call, await $request);
  }

  $async.Future<$0.Environment> updateEnvironment(
      $grpc.ServiceCall call, $0.UpdateEnvironmentRequest request);

  $async.Future<$0.Environment> setDefaultEnvironment_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SetDefaultEnvironmentRequest> $request) async {
    return setDefaultEnvironment($call, await $request);
  }

  $async.Future<$0.Environment> setDefaultEnvironment(
      $grpc.ServiceCall call, $0.SetDefaultEnvironmentRequest request);

  $async.Future<$1.Empty> deleteEnvironment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteEnvironmentRequest> $request) async {
    return deleteEnvironment($call, await $request);
  }

  $async.Future<$1.Empty> deleteEnvironment(
      $grpc.ServiceCall call, $0.DeleteEnvironmentRequest request);
}

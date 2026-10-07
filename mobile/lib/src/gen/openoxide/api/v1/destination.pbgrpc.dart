// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/destination.proto.

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
import 'destination.pb.dart' as $0;

export 'destination.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.DestinationService')
class DestinationServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DestinationServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListDestinationsResponse> listDestinations(
    $0.ListDestinationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDestinations, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetDestinationResponse> getDestination(
    $0.GetDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDestination, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateDestinationResponse> createDestination(
    $0.CreateDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDestination, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateDestinationResponse> updateDestination(
    $0.UpdateDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateDestination, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDestination(
    $0.DeleteDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDestination, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestDestinationResponse> testDestination(
    $0.TestDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testDestination, request, options: options);
  }

  // method descriptors

  static final _$listDestinations = $grpc.ClientMethod<
          $0.ListDestinationsRequest, $0.ListDestinationsResponse>(
      '/openoxide.api.v1.DestinationService/ListDestinations',
      ($0.ListDestinationsRequest value) => value.writeToBuffer(),
      $0.ListDestinationsResponse.fromBuffer);
  static final _$getDestination =
      $grpc.ClientMethod<$0.GetDestinationRequest, $0.GetDestinationResponse>(
          '/openoxide.api.v1.DestinationService/GetDestination',
          ($0.GetDestinationRequest value) => value.writeToBuffer(),
          $0.GetDestinationResponse.fromBuffer);
  static final _$createDestination = $grpc.ClientMethod<
          $0.CreateDestinationRequest, $0.CreateDestinationResponse>(
      '/openoxide.api.v1.DestinationService/CreateDestination',
      ($0.CreateDestinationRequest value) => value.writeToBuffer(),
      $0.CreateDestinationResponse.fromBuffer);
  static final _$updateDestination = $grpc.ClientMethod<
          $0.UpdateDestinationRequest, $0.UpdateDestinationResponse>(
      '/openoxide.api.v1.DestinationService/UpdateDestination',
      ($0.UpdateDestinationRequest value) => value.writeToBuffer(),
      $0.UpdateDestinationResponse.fromBuffer);
  static final _$deleteDestination =
      $grpc.ClientMethod<$0.DeleteDestinationRequest, $1.Empty>(
          '/openoxide.api.v1.DestinationService/DeleteDestination',
          ($0.DeleteDestinationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testDestination =
      $grpc.ClientMethod<$0.TestDestinationRequest, $0.TestDestinationResponse>(
          '/openoxide.api.v1.DestinationService/TestDestination',
          ($0.TestDestinationRequest value) => value.writeToBuffer(),
          $0.TestDestinationResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DestinationService')
abstract class DestinationServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DestinationService';

  DestinationServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListDestinationsRequest,
            $0.ListDestinationsResponse>(
        'ListDestinations',
        listDestinations_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListDestinationsRequest.fromBuffer(value),
        ($0.ListDestinationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDestinationRequest,
            $0.GetDestinationResponse>(
        'GetDestination',
        getDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDestinationRequest.fromBuffer(value),
        ($0.GetDestinationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDestinationRequest,
            $0.CreateDestinationResponse>(
        'CreateDestination',
        createDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDestinationRequest.fromBuffer(value),
        ($0.CreateDestinationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateDestinationRequest,
            $0.UpdateDestinationResponse>(
        'UpdateDestination',
        updateDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateDestinationRequest.fromBuffer(value),
        ($0.UpdateDestinationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteDestinationRequest, $1.Empty>(
        'DeleteDestination',
        deleteDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteDestinationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestDestinationRequest,
            $0.TestDestinationResponse>(
        'TestDestination',
        testDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestDestinationRequest.fromBuffer(value),
        ($0.TestDestinationResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListDestinationsResponse> listDestinations_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDestinationsRequest> $request) async {
    return listDestinations($call, await $request);
  }

  $async.Future<$0.ListDestinationsResponse> listDestinations(
      $grpc.ServiceCall call, $0.ListDestinationsRequest request);

  $async.Future<$0.GetDestinationResponse> getDestination_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetDestinationRequest> $request) async {
    return getDestination($call, await $request);
  }

  $async.Future<$0.GetDestinationResponse> getDestination(
      $grpc.ServiceCall call, $0.GetDestinationRequest request);

  $async.Future<$0.CreateDestinationResponse> createDestination_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateDestinationRequest> $request) async {
    return createDestination($call, await $request);
  }

  $async.Future<$0.CreateDestinationResponse> createDestination(
      $grpc.ServiceCall call, $0.CreateDestinationRequest request);

  $async.Future<$0.UpdateDestinationResponse> updateDestination_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateDestinationRequest> $request) async {
    return updateDestination($call, await $request);
  }

  $async.Future<$0.UpdateDestinationResponse> updateDestination(
      $grpc.ServiceCall call, $0.UpdateDestinationRequest request);

  $async.Future<$1.Empty> deleteDestination_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteDestinationRequest> $request) async {
    return deleteDestination($call, await $request);
  }

  $async.Future<$1.Empty> deleteDestination(
      $grpc.ServiceCall call, $0.DeleteDestinationRequest request);

  $async.Future<$0.TestDestinationResponse> testDestination_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestDestinationRequest> $request) async {
    return testDestination($call, await $request);
  }

  $async.Future<$0.TestDestinationResponse> testDestination(
      $grpc.ServiceCall call, $0.TestDestinationRequest request);
}

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database_network.proto.

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
import 'database_network.pb.dart' as $0;

export 'database_network.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.DatabaseNetworkService')
class DatabaseNetworkServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DatabaseNetworkServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.DatabaseNetworkListNetworksResponse> listNetworks(
    $0.DatabaseNetworkListNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNetworks, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseNetworkListServerNetworksResponse>
      listServerNetworks(
    $0.DatabaseNetworkListServerNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listServerNetworks, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseNetworkGetNetworkResponse> getNetwork(
    $0.DatabaseNetworkGetNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseNetworkCreateNetworkResponse> createNetwork(
    $0.DatabaseNetworkCreateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseNetworkUpdateNetworkResponse> updateNetwork(
    $0.DatabaseNetworkUpdateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteNetwork(
    $0.DatabaseNetworkDeleteNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseNetworkListDependenciesResponse>
      listDependencies(
    $0.DatabaseNetworkListDependenciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDependencies, request, options: options);
  }

  // method descriptors

  static final _$listNetworks = $grpc.ClientMethod<
          $0.DatabaseNetworkListNetworksRequest,
          $0.DatabaseNetworkListNetworksResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/ListNetworks',
      ($0.DatabaseNetworkListNetworksRequest value) => value.writeToBuffer(),
      $0.DatabaseNetworkListNetworksResponse.fromBuffer);
  static final _$listServerNetworks = $grpc.ClientMethod<
          $0.DatabaseNetworkListServerNetworksRequest,
          $0.DatabaseNetworkListServerNetworksResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/ListServerNetworks',
      ($0.DatabaseNetworkListServerNetworksRequest value) =>
          value.writeToBuffer(),
      $0.DatabaseNetworkListServerNetworksResponse.fromBuffer);
  static final _$getNetwork = $grpc.ClientMethod<
          $0.DatabaseNetworkGetNetworkRequest,
          $0.DatabaseNetworkGetNetworkResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/GetNetwork',
      ($0.DatabaseNetworkGetNetworkRequest value) => value.writeToBuffer(),
      $0.DatabaseNetworkGetNetworkResponse.fromBuffer);
  static final _$createNetwork = $grpc.ClientMethod<
          $0.DatabaseNetworkCreateNetworkRequest,
          $0.DatabaseNetworkCreateNetworkResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/CreateNetwork',
      ($0.DatabaseNetworkCreateNetworkRequest value) => value.writeToBuffer(),
      $0.DatabaseNetworkCreateNetworkResponse.fromBuffer);
  static final _$updateNetwork = $grpc.ClientMethod<
          $0.DatabaseNetworkUpdateNetworkRequest,
          $0.DatabaseNetworkUpdateNetworkResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/UpdateNetwork',
      ($0.DatabaseNetworkUpdateNetworkRequest value) => value.writeToBuffer(),
      $0.DatabaseNetworkUpdateNetworkResponse.fromBuffer);
  static final _$deleteNetwork =
      $grpc.ClientMethod<$0.DatabaseNetworkDeleteNetworkRequest, $1.Empty>(
          '/openoxide.api.v1.DatabaseNetworkService/DeleteNetwork',
          ($0.DatabaseNetworkDeleteNetworkRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listDependencies = $grpc.ClientMethod<
          $0.DatabaseNetworkListDependenciesRequest,
          $0.DatabaseNetworkListDependenciesResponse>(
      '/openoxide.api.v1.DatabaseNetworkService/ListDependencies',
      ($0.DatabaseNetworkListDependenciesRequest value) =>
          value.writeToBuffer(),
      $0.DatabaseNetworkListDependenciesResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DatabaseNetworkService')
abstract class DatabaseNetworkServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DatabaseNetworkService';

  DatabaseNetworkServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkListNetworksRequest,
            $0.DatabaseNetworkListNetworksResponse>(
        'ListNetworks',
        listNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkListNetworksRequest.fromBuffer(value),
        ($0.DatabaseNetworkListNetworksResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkListServerNetworksRequest,
            $0.DatabaseNetworkListServerNetworksResponse>(
        'ListServerNetworks',
        listServerNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkListServerNetworksRequest.fromBuffer(value),
        ($0.DatabaseNetworkListServerNetworksResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkGetNetworkRequest,
            $0.DatabaseNetworkGetNetworkResponse>(
        'GetNetwork',
        getNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkGetNetworkRequest.fromBuffer(value),
        ($0.DatabaseNetworkGetNetworkResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkCreateNetworkRequest,
            $0.DatabaseNetworkCreateNetworkResponse>(
        'CreateNetwork',
        createNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkCreateNetworkRequest.fromBuffer(value),
        ($0.DatabaseNetworkCreateNetworkResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkUpdateNetworkRequest,
            $0.DatabaseNetworkUpdateNetworkResponse>(
        'UpdateNetwork',
        updateNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkUpdateNetworkRequest.fromBuffer(value),
        ($0.DatabaseNetworkUpdateNetworkResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseNetworkDeleteNetworkRequest, $1.Empty>(
            'DeleteNetwork',
            deleteNetwork_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseNetworkDeleteNetworkRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseNetworkListDependenciesRequest,
            $0.DatabaseNetworkListDependenciesResponse>(
        'ListDependencies',
        listDependencies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseNetworkListDependenciesRequest.fromBuffer(value),
        ($0.DatabaseNetworkListDependenciesResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.DatabaseNetworkListNetworksResponse> listNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseNetworkListNetworksRequest> $request) async {
    return listNetworks($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkListNetworksResponse> listNetworks(
      $grpc.ServiceCall call, $0.DatabaseNetworkListNetworksRequest request);

  $async.Future<$0.DatabaseNetworkListServerNetworksResponse>
      listServerNetworks_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.DatabaseNetworkListServerNetworksRequest>
              $request) async {
    return listServerNetworks($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkListServerNetworksResponse>
      listServerNetworks($grpc.ServiceCall call,
          $0.DatabaseNetworkListServerNetworksRequest request);

  $async.Future<$0.DatabaseNetworkGetNetworkResponse> getNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseNetworkGetNetworkRequest> $request) async {
    return getNetwork($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkGetNetworkResponse> getNetwork(
      $grpc.ServiceCall call, $0.DatabaseNetworkGetNetworkRequest request);

  $async.Future<$0.DatabaseNetworkCreateNetworkResponse> createNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseNetworkCreateNetworkRequest> $request) async {
    return createNetwork($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkCreateNetworkResponse> createNetwork(
      $grpc.ServiceCall call, $0.DatabaseNetworkCreateNetworkRequest request);

  $async.Future<$0.DatabaseNetworkUpdateNetworkResponse> updateNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseNetworkUpdateNetworkRequest> $request) async {
    return updateNetwork($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkUpdateNetworkResponse> updateNetwork(
      $grpc.ServiceCall call, $0.DatabaseNetworkUpdateNetworkRequest request);

  $async.Future<$1.Empty> deleteNetwork_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseNetworkDeleteNetworkRequest> $request) async {
    return deleteNetwork($call, await $request);
  }

  $async.Future<$1.Empty> deleteNetwork(
      $grpc.ServiceCall call, $0.DatabaseNetworkDeleteNetworkRequest request);

  $async.Future<$0.DatabaseNetworkListDependenciesResponse>
      listDependencies_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.DatabaseNetworkListDependenciesRequest>
              $request) async {
    return listDependencies($call, await $request);
  }

  $async.Future<$0.DatabaseNetworkListDependenciesResponse> listDependencies(
      $grpc.ServiceCall call,
      $0.DatabaseNetworkListDependenciesRequest request);
}

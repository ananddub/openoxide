// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server.proto.

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
import 'server.pb.dart' as $0;

export 'server.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ServerService')
class ServerServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ServerServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ServerListServersResponse> listServers(
    $0.ServerListServersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listServers, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerGetServerResponse> getServer(
    $0.ServerGetServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerCreateServerResponse> createServer(
    $0.ServerCreateServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerUpdateServerResponse> updateServer(
    $0.ServerUpdateServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateServer, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteServer(
    $0.ServerDeleteServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerTestConnectionResponse> testConnection(
    $0.ServerTestConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerSetupServerResponse> setupServer(
    $0.ServerSetupServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setupServer, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> startServer(
    $0.ServerStartServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$startServer, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> stopServer(
    $0.ServerStopServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stopServer, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> restartServer(
    $0.ServerRestartServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$restartServer, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeServer(
    $0.ServerRemoveServerRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerGetServerErrorsResponse> getServerErrors(
    $0.ServerGetServerErrorsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getServerErrors, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerListRemoteServersResponse> listRemoteServers(
    $0.ServerListRemoteServersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRemoteServers, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerListPrivateNetworksResponse>
      listPrivateNetworks(
    $0.ServerListPrivateNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPrivateNetworks, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerCreatePrivateNetworkResponse>
      createPrivateNetwork(
    $0.ServerCreatePrivateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPrivateNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerUpdatePrivateNetworkResponse>
      updatePrivateNetwork(
    $0.ServerUpdatePrivateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePrivateNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePrivateNetwork(
    $0.ServerDeletePrivateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePrivateNetwork, request, options: options);
  }

  // method descriptors

  static final _$listServers = $grpc.ClientMethod<$0.ServerListServersRequest,
          $0.ServerListServersResponse>(
      '/openoxide.api.v1.ServerService/ListServers',
      ($0.ServerListServersRequest value) => value.writeToBuffer(),
      $0.ServerListServersResponse.fromBuffer);
  static final _$getServer =
      $grpc.ClientMethod<$0.ServerGetServerRequest, $0.ServerGetServerResponse>(
          '/openoxide.api.v1.ServerService/GetServer',
          ($0.ServerGetServerRequest value) => value.writeToBuffer(),
          $0.ServerGetServerResponse.fromBuffer);
  static final _$createServer = $grpc.ClientMethod<$0.ServerCreateServerRequest,
          $0.ServerCreateServerResponse>(
      '/openoxide.api.v1.ServerService/CreateServer',
      ($0.ServerCreateServerRequest value) => value.writeToBuffer(),
      $0.ServerCreateServerResponse.fromBuffer);
  static final _$updateServer = $grpc.ClientMethod<$0.ServerUpdateServerRequest,
          $0.ServerUpdateServerResponse>(
      '/openoxide.api.v1.ServerService/UpdateServer',
      ($0.ServerUpdateServerRequest value) => value.writeToBuffer(),
      $0.ServerUpdateServerResponse.fromBuffer);
  static final _$deleteServer =
      $grpc.ClientMethod<$0.ServerDeleteServerRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/DeleteServer',
          ($0.ServerDeleteServerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<
          $0.ServerTestConnectionRequest, $0.ServerTestConnectionResponse>(
      '/openoxide.api.v1.ServerService/TestConnection',
      ($0.ServerTestConnectionRequest value) => value.writeToBuffer(),
      $0.ServerTestConnectionResponse.fromBuffer);
  static final _$setupServer = $grpc.ClientMethod<$0.ServerSetupServerRequest,
          $0.ServerSetupServerResponse>(
      '/openoxide.api.v1.ServerService/SetupServer',
      ($0.ServerSetupServerRequest value) => value.writeToBuffer(),
      $0.ServerSetupServerResponse.fromBuffer);
  static final _$startServer =
      $grpc.ClientMethod<$0.ServerStartServerRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/StartServer',
          ($0.ServerStartServerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$stopServer =
      $grpc.ClientMethod<$0.ServerStopServerRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/StopServer',
          ($0.ServerStopServerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$restartServer =
      $grpc.ClientMethod<$0.ServerRestartServerRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/RestartServer',
          ($0.ServerRestartServerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$removeServer =
      $grpc.ClientMethod<$0.ServerRemoveServerRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/RemoveServer',
          ($0.ServerRemoveServerRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getServerErrors = $grpc.ClientMethod<
          $0.ServerGetServerErrorsRequest, $0.ServerGetServerErrorsResponse>(
      '/openoxide.api.v1.ServerService/GetServerErrors',
      ($0.ServerGetServerErrorsRequest value) => value.writeToBuffer(),
      $0.ServerGetServerErrorsResponse.fromBuffer);
  static final _$listRemoteServers = $grpc.ClientMethod<
          $0.ServerListRemoteServersRequest,
          $0.ServerListRemoteServersResponse>(
      '/openoxide.api.v1.ServerService/ListRemoteServers',
      ($0.ServerListRemoteServersRequest value) => value.writeToBuffer(),
      $0.ServerListRemoteServersResponse.fromBuffer);
  static final _$listPrivateNetworks = $grpc.ClientMethod<
          $0.ServerListPrivateNetworksRequest,
          $0.ServerListPrivateNetworksResponse>(
      '/openoxide.api.v1.ServerService/ListPrivateNetworks',
      ($0.ServerListPrivateNetworksRequest value) => value.writeToBuffer(),
      $0.ServerListPrivateNetworksResponse.fromBuffer);
  static final _$createPrivateNetwork = $grpc.ClientMethod<
          $0.ServerCreatePrivateNetworkRequest,
          $0.ServerCreatePrivateNetworkResponse>(
      '/openoxide.api.v1.ServerService/CreatePrivateNetwork',
      ($0.ServerCreatePrivateNetworkRequest value) => value.writeToBuffer(),
      $0.ServerCreatePrivateNetworkResponse.fromBuffer);
  static final _$updatePrivateNetwork = $grpc.ClientMethod<
          $0.ServerUpdatePrivateNetworkRequest,
          $0.ServerUpdatePrivateNetworkResponse>(
      '/openoxide.api.v1.ServerService/UpdatePrivateNetwork',
      ($0.ServerUpdatePrivateNetworkRequest value) => value.writeToBuffer(),
      $0.ServerUpdatePrivateNetworkResponse.fromBuffer);
  static final _$deletePrivateNetwork =
      $grpc.ClientMethod<$0.ServerDeletePrivateNetworkRequest, $1.Empty>(
          '/openoxide.api.v1.ServerService/DeletePrivateNetwork',
          ($0.ServerDeletePrivateNetworkRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ServerService')
abstract class ServerServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ServerService';

  ServerServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ServerListServersRequest,
            $0.ServerListServersResponse>(
        'ListServers',
        listServers_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerListServersRequest.fromBuffer(value),
        ($0.ServerListServersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerGetServerRequest,
            $0.ServerGetServerResponse>(
        'GetServer',
        getServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerGetServerRequest.fromBuffer(value),
        ($0.ServerGetServerResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerCreateServerRequest,
            $0.ServerCreateServerResponse>(
        'CreateServer',
        createServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerCreateServerRequest.fromBuffer(value),
        ($0.ServerCreateServerResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerUpdateServerRequest,
            $0.ServerUpdateServerResponse>(
        'UpdateServer',
        updateServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerUpdateServerRequest.fromBuffer(value),
        ($0.ServerUpdateServerResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerDeleteServerRequest, $1.Empty>(
        'DeleteServer',
        deleteServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerDeleteServerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerTestConnectionRequest,
            $0.ServerTestConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerTestConnectionRequest.fromBuffer(value),
        ($0.ServerTestConnectionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerSetupServerRequest,
            $0.ServerSetupServerResponse>(
        'SetupServer',
        setupServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerSetupServerRequest.fromBuffer(value),
        ($0.ServerSetupServerResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerStartServerRequest, $1.Empty>(
        'StartServer',
        startServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerStartServerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerStopServerRequest, $1.Empty>(
        'StopServer',
        stopServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerStopServerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerRestartServerRequest, $1.Empty>(
        'RestartServer',
        restartServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerRestartServerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerRemoveServerRequest, $1.Empty>(
        'RemoveServer',
        removeServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerRemoveServerRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerGetServerErrorsRequest,
            $0.ServerGetServerErrorsResponse>(
        'GetServerErrors',
        getServerErrors_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerGetServerErrorsRequest.fromBuffer(value),
        ($0.ServerGetServerErrorsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerListRemoteServersRequest,
            $0.ServerListRemoteServersResponse>(
        'ListRemoteServers',
        listRemoteServers_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerListRemoteServersRequest.fromBuffer(value),
        ($0.ServerListRemoteServersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerListPrivateNetworksRequest,
            $0.ServerListPrivateNetworksResponse>(
        'ListPrivateNetworks',
        listPrivateNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerListPrivateNetworksRequest.fromBuffer(value),
        ($0.ServerListPrivateNetworksResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerCreatePrivateNetworkRequest,
            $0.ServerCreatePrivateNetworkResponse>(
        'CreatePrivateNetwork',
        createPrivateNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerCreatePrivateNetworkRequest.fromBuffer(value),
        ($0.ServerCreatePrivateNetworkResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerUpdatePrivateNetworkRequest,
            $0.ServerUpdatePrivateNetworkResponse>(
        'UpdatePrivateNetwork',
        updatePrivateNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerUpdatePrivateNetworkRequest.fromBuffer(value),
        ($0.ServerUpdatePrivateNetworkResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ServerDeletePrivateNetworkRequest, $1.Empty>(
            'DeletePrivateNetwork',
            deletePrivateNetwork_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ServerDeletePrivateNetworkRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ServerListServersResponse> listServers_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerListServersRequest> $request) async {
    return listServers($call, await $request);
  }

  $async.Future<$0.ServerListServersResponse> listServers(
      $grpc.ServiceCall call, $0.ServerListServersRequest request);

  $async.Future<$0.ServerGetServerResponse> getServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerGetServerRequest> $request) async {
    return getServer($call, await $request);
  }

  $async.Future<$0.ServerGetServerResponse> getServer(
      $grpc.ServiceCall call, $0.ServerGetServerRequest request);

  $async.Future<$0.ServerCreateServerResponse> createServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerCreateServerRequest> $request) async {
    return createServer($call, await $request);
  }

  $async.Future<$0.ServerCreateServerResponse> createServer(
      $grpc.ServiceCall call, $0.ServerCreateServerRequest request);

  $async.Future<$0.ServerUpdateServerResponse> updateServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerUpdateServerRequest> $request) async {
    return updateServer($call, await $request);
  }

  $async.Future<$0.ServerUpdateServerResponse> updateServer(
      $grpc.ServiceCall call, $0.ServerUpdateServerRequest request);

  $async.Future<$1.Empty> deleteServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerDeleteServerRequest> $request) async {
    return deleteServer($call, await $request);
  }

  $async.Future<$1.Empty> deleteServer(
      $grpc.ServiceCall call, $0.ServerDeleteServerRequest request);

  $async.Future<$0.ServerTestConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerTestConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.ServerTestConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.ServerTestConnectionRequest request);

  $async.Future<$0.ServerSetupServerResponse> setupServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerSetupServerRequest> $request) async {
    return setupServer($call, await $request);
  }

  $async.Future<$0.ServerSetupServerResponse> setupServer(
      $grpc.ServiceCall call, $0.ServerSetupServerRequest request);

  $async.Future<$1.Empty> startServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerStartServerRequest> $request) async {
    return startServer($call, await $request);
  }

  $async.Future<$1.Empty> startServer(
      $grpc.ServiceCall call, $0.ServerStartServerRequest request);

  $async.Future<$1.Empty> stopServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerStopServerRequest> $request) async {
    return stopServer($call, await $request);
  }

  $async.Future<$1.Empty> stopServer(
      $grpc.ServiceCall call, $0.ServerStopServerRequest request);

  $async.Future<$1.Empty> restartServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerRestartServerRequest> $request) async {
    return restartServer($call, await $request);
  }

  $async.Future<$1.Empty> restartServer(
      $grpc.ServiceCall call, $0.ServerRestartServerRequest request);

  $async.Future<$1.Empty> removeServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerRemoveServerRequest> $request) async {
    return removeServer($call, await $request);
  }

  $async.Future<$1.Empty> removeServer(
      $grpc.ServiceCall call, $0.ServerRemoveServerRequest request);

  $async.Future<$0.ServerGetServerErrorsResponse> getServerErrors_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerGetServerErrorsRequest> $request) async {
    return getServerErrors($call, await $request);
  }

  $async.Future<$0.ServerGetServerErrorsResponse> getServerErrors(
      $grpc.ServiceCall call, $0.ServerGetServerErrorsRequest request);

  $async.Future<$0.ServerListRemoteServersResponse> listRemoteServers_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerListRemoteServersRequest> $request) async {
    return listRemoteServers($call, await $request);
  }

  $async.Future<$0.ServerListRemoteServersResponse> listRemoteServers(
      $grpc.ServiceCall call, $0.ServerListRemoteServersRequest request);

  $async.Future<$0.ServerListPrivateNetworksResponse> listPrivateNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerListPrivateNetworksRequest> $request) async {
    return listPrivateNetworks($call, await $request);
  }

  $async.Future<$0.ServerListPrivateNetworksResponse> listPrivateNetworks(
      $grpc.ServiceCall call, $0.ServerListPrivateNetworksRequest request);

  $async.Future<$0.ServerCreatePrivateNetworkResponse> createPrivateNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerCreatePrivateNetworkRequest> $request) async {
    return createPrivateNetwork($call, await $request);
  }

  $async.Future<$0.ServerCreatePrivateNetworkResponse> createPrivateNetwork(
      $grpc.ServiceCall call, $0.ServerCreatePrivateNetworkRequest request);

  $async.Future<$0.ServerUpdatePrivateNetworkResponse> updatePrivateNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerUpdatePrivateNetworkRequest> $request) async {
    return updatePrivateNetwork($call, await $request);
  }

  $async.Future<$0.ServerUpdatePrivateNetworkResponse> updatePrivateNetwork(
      $grpc.ServiceCall call, $0.ServerUpdatePrivateNetworkRequest request);

  $async.Future<$1.Empty> deletePrivateNetwork_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerDeletePrivateNetworkRequest> $request) async {
    return deletePrivateNetwork($call, await $request);
  }

  $async.Future<$1.Empty> deletePrivateNetwork(
      $grpc.ServiceCall call, $0.ServerDeletePrivateNetworkRequest request);
}

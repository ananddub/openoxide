// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/remote_server.proto.

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
import 'remote_server.pb.dart' as $0;

export 'remote_server.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.RemoteServerService')
class RemoteServerServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  RemoteServerServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.RemoteServerListResponse> listRemoteServers(
    $0.RemoteServerListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRemoteServers, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerGetResponse> getRemoteServer(
    $0.RemoteServerGetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRemoteServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerCreateResponse> createRemoteServer(
    $0.RemoteServerCreateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createRemoteServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerUpdateResponse> updateRemoteServer(
    $0.RemoteServerUpdateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRemoteServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerActivateResponse> activate(
    $0.RemoteServerActivateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$activate, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerDeactivateResponse> deactivate(
    $0.RemoteServerDeactivateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deactivate, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerTestConnectionResponse> testConnection(
    $0.RemoteServerTestConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteRemoteServer(
    $0.RemoteServerDeleteRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRemoteServer, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerMigrateDependenciesResponse>
      migrateDependencies(
    $0.RemoteServerMigrateDependenciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$migrateDependencies, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerGetMigrationResponse> getMigration(
    $0.RemoteServerGetMigrationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getMigration, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoteServerRollbackMigrationResponse>
      rollbackMigration(
    $0.RemoteServerRollbackMigrationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rollbackMigration, request, options: options);
  }

  /// Flags a RUNNING application_migrations row for cooperative cancellation; the runner honours it at the next phase/object boundary.
  $grpc.ResponseFuture<$0.RemoteServerCancelMigrationResponse> cancelMigration(
    $0.RemoteServerCancelMigrationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelMigration, request, options: options);
  }

  /// Triggers (or resumes) a stateful single-service migration.
  $grpc.ResponseFuture<$0.RemoteServerMigrateServiceResponse> migrateService(
    $0.RemoteServerMigrateServiceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$migrateService, request, options: options);
  }

  /// Lists stateful migrations for a service, newest first.
  $grpc.ResponseFuture<$0.RemoteServerListMigrationsResponse> listMigrations(
    $0.RemoteServerListMigrationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMigrations, request, options: options);
  }

  /// Lists per-object checkpoints of one stateful migration.
  $grpc.ResponseFuture<$0.RemoteServerListMigrationObjectsResponse>
      listMigrationObjects(
    $0.RemoteServerListMigrationObjectsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMigrationObjects, request, options: options);
  }

  // method descriptors

  static final _$listRemoteServers = $grpc.ClientMethod<
          $0.RemoteServerListRequest, $0.RemoteServerListResponse>(
      '/openoxide.api.v1.RemoteServerService/ListRemoteServers',
      ($0.RemoteServerListRequest value) => value.writeToBuffer(),
      $0.RemoteServerListResponse.fromBuffer);
  static final _$getRemoteServer =
      $grpc.ClientMethod<$0.RemoteServerGetRequest, $0.RemoteServerGetResponse>(
          '/openoxide.api.v1.RemoteServerService/GetRemoteServer',
          ($0.RemoteServerGetRequest value) => value.writeToBuffer(),
          $0.RemoteServerGetResponse.fromBuffer);
  static final _$createRemoteServer = $grpc.ClientMethod<
          $0.RemoteServerCreateRequest, $0.RemoteServerCreateResponse>(
      '/openoxide.api.v1.RemoteServerService/CreateRemoteServer',
      ($0.RemoteServerCreateRequest value) => value.writeToBuffer(),
      $0.RemoteServerCreateResponse.fromBuffer);
  static final _$updateRemoteServer = $grpc.ClientMethod<
          $0.RemoteServerUpdateRequest, $0.RemoteServerUpdateResponse>(
      '/openoxide.api.v1.RemoteServerService/UpdateRemoteServer',
      ($0.RemoteServerUpdateRequest value) => value.writeToBuffer(),
      $0.RemoteServerUpdateResponse.fromBuffer);
  static final _$activate = $grpc.ClientMethod<$0.RemoteServerActivateRequest,
          $0.RemoteServerActivateResponse>(
      '/openoxide.api.v1.RemoteServerService/Activate',
      ($0.RemoteServerActivateRequest value) => value.writeToBuffer(),
      $0.RemoteServerActivateResponse.fromBuffer);
  static final _$deactivate = $grpc.ClientMethod<
          $0.RemoteServerDeactivateRequest, $0.RemoteServerDeactivateResponse>(
      '/openoxide.api.v1.RemoteServerService/Deactivate',
      ($0.RemoteServerDeactivateRequest value) => value.writeToBuffer(),
      $0.RemoteServerDeactivateResponse.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<
          $0.RemoteServerTestConnectionRequest,
          $0.RemoteServerTestConnectionResponse>(
      '/openoxide.api.v1.RemoteServerService/TestConnection',
      ($0.RemoteServerTestConnectionRequest value) => value.writeToBuffer(),
      $0.RemoteServerTestConnectionResponse.fromBuffer);
  static final _$deleteRemoteServer =
      $grpc.ClientMethod<$0.RemoteServerDeleteRequest, $1.Empty>(
          '/openoxide.api.v1.RemoteServerService/DeleteRemoteServer',
          ($0.RemoteServerDeleteRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$migrateDependencies = $grpc.ClientMethod<
          $0.RemoteServerMigrateDependenciesRequest,
          $0.RemoteServerMigrateDependenciesResponse>(
      '/openoxide.api.v1.RemoteServerService/MigrateDependencies',
      ($0.RemoteServerMigrateDependenciesRequest value) =>
          value.writeToBuffer(),
      $0.RemoteServerMigrateDependenciesResponse.fromBuffer);
  static final _$getMigration = $grpc.ClientMethod<
          $0.RemoteServerGetMigrationRequest,
          $0.RemoteServerGetMigrationResponse>(
      '/openoxide.api.v1.RemoteServerService/GetMigration',
      ($0.RemoteServerGetMigrationRequest value) => value.writeToBuffer(),
      $0.RemoteServerGetMigrationResponse.fromBuffer);
  static final _$rollbackMigration = $grpc.ClientMethod<
          $0.RemoteServerRollbackMigrationRequest,
          $0.RemoteServerRollbackMigrationResponse>(
      '/openoxide.api.v1.RemoteServerService/RollbackMigration',
      ($0.RemoteServerRollbackMigrationRequest value) => value.writeToBuffer(),
      $0.RemoteServerRollbackMigrationResponse.fromBuffer);
  static final _$cancelMigration = $grpc.ClientMethod<
          $0.RemoteServerCancelMigrationRequest,
          $0.RemoteServerCancelMigrationResponse>(
      '/openoxide.api.v1.RemoteServerService/CancelMigration',
      ($0.RemoteServerCancelMigrationRequest value) => value.writeToBuffer(),
      $0.RemoteServerCancelMigrationResponse.fromBuffer);
  static final _$migrateService = $grpc.ClientMethod<
          $0.RemoteServerMigrateServiceRequest,
          $0.RemoteServerMigrateServiceResponse>(
      '/openoxide.api.v1.RemoteServerService/MigrateService',
      ($0.RemoteServerMigrateServiceRequest value) => value.writeToBuffer(),
      $0.RemoteServerMigrateServiceResponse.fromBuffer);
  static final _$listMigrations = $grpc.ClientMethod<
          $0.RemoteServerListMigrationsRequest,
          $0.RemoteServerListMigrationsResponse>(
      '/openoxide.api.v1.RemoteServerService/ListMigrations',
      ($0.RemoteServerListMigrationsRequest value) => value.writeToBuffer(),
      $0.RemoteServerListMigrationsResponse.fromBuffer);
  static final _$listMigrationObjects = $grpc.ClientMethod<
          $0.RemoteServerListMigrationObjectsRequest,
          $0.RemoteServerListMigrationObjectsResponse>(
      '/openoxide.api.v1.RemoteServerService/ListMigrationObjects',
      ($0.RemoteServerListMigrationObjectsRequest value) =>
          value.writeToBuffer(),
      $0.RemoteServerListMigrationObjectsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.RemoteServerService')
abstract class RemoteServerServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.RemoteServerService';

  RemoteServerServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.RemoteServerListRequest,
            $0.RemoteServerListResponse>(
        'ListRemoteServers',
        listRemoteServers_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerListRequest.fromBuffer(value),
        ($0.RemoteServerListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerGetRequest,
            $0.RemoteServerGetResponse>(
        'GetRemoteServer',
        getRemoteServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerGetRequest.fromBuffer(value),
        ($0.RemoteServerGetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerCreateRequest,
            $0.RemoteServerCreateResponse>(
        'CreateRemoteServer',
        createRemoteServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerCreateRequest.fromBuffer(value),
        ($0.RemoteServerCreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerUpdateRequest,
            $0.RemoteServerUpdateResponse>(
        'UpdateRemoteServer',
        updateRemoteServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerUpdateRequest.fromBuffer(value),
        ($0.RemoteServerUpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerActivateRequest,
            $0.RemoteServerActivateResponse>(
        'Activate',
        activate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerActivateRequest.fromBuffer(value),
        ($0.RemoteServerActivateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerDeactivateRequest,
            $0.RemoteServerDeactivateResponse>(
        'Deactivate',
        deactivate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerDeactivateRequest.fromBuffer(value),
        ($0.RemoteServerDeactivateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerTestConnectionRequest,
            $0.RemoteServerTestConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerTestConnectionRequest.fromBuffer(value),
        ($0.RemoteServerTestConnectionResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerDeleteRequest, $1.Empty>(
        'DeleteRemoteServer',
        deleteRemoteServer_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerDeleteRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerMigrateDependenciesRequest,
            $0.RemoteServerMigrateDependenciesResponse>(
        'MigrateDependencies',
        migrateDependencies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerMigrateDependenciesRequest.fromBuffer(value),
        ($0.RemoteServerMigrateDependenciesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerGetMigrationRequest,
            $0.RemoteServerGetMigrationResponse>(
        'GetMigration',
        getMigration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerGetMigrationRequest.fromBuffer(value),
        ($0.RemoteServerGetMigrationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerRollbackMigrationRequest,
            $0.RemoteServerRollbackMigrationResponse>(
        'RollbackMigration',
        rollbackMigration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerRollbackMigrationRequest.fromBuffer(value),
        ($0.RemoteServerRollbackMigrationResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerCancelMigrationRequest,
            $0.RemoteServerCancelMigrationResponse>(
        'CancelMigration',
        cancelMigration_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerCancelMigrationRequest.fromBuffer(value),
        ($0.RemoteServerCancelMigrationResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerMigrateServiceRequest,
            $0.RemoteServerMigrateServiceResponse>(
        'MigrateService',
        migrateService_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerMigrateServiceRequest.fromBuffer(value),
        ($0.RemoteServerMigrateServiceResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerListMigrationsRequest,
            $0.RemoteServerListMigrationsResponse>(
        'ListMigrations',
        listMigrations_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerListMigrationsRequest.fromBuffer(value),
        ($0.RemoteServerListMigrationsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoteServerListMigrationObjectsRequest,
            $0.RemoteServerListMigrationObjectsResponse>(
        'ListMigrationObjects',
        listMigrationObjects_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoteServerListMigrationObjectsRequest.fromBuffer(value),
        ($0.RemoteServerListMigrationObjectsResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.RemoteServerListResponse> listRemoteServers_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerListRequest> $request) async {
    return listRemoteServers($call, await $request);
  }

  $async.Future<$0.RemoteServerListResponse> listRemoteServers(
      $grpc.ServiceCall call, $0.RemoteServerListRequest request);

  $async.Future<$0.RemoteServerGetResponse> getRemoteServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerGetRequest> $request) async {
    return getRemoteServer($call, await $request);
  }

  $async.Future<$0.RemoteServerGetResponse> getRemoteServer(
      $grpc.ServiceCall call, $0.RemoteServerGetRequest request);

  $async.Future<$0.RemoteServerCreateResponse> createRemoteServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerCreateRequest> $request) async {
    return createRemoteServer($call, await $request);
  }

  $async.Future<$0.RemoteServerCreateResponse> createRemoteServer(
      $grpc.ServiceCall call, $0.RemoteServerCreateRequest request);

  $async.Future<$0.RemoteServerUpdateResponse> updateRemoteServer_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerUpdateRequest> $request) async {
    return updateRemoteServer($call, await $request);
  }

  $async.Future<$0.RemoteServerUpdateResponse> updateRemoteServer(
      $grpc.ServiceCall call, $0.RemoteServerUpdateRequest request);

  $async.Future<$0.RemoteServerActivateResponse> activate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerActivateRequest> $request) async {
    return activate($call, await $request);
  }

  $async.Future<$0.RemoteServerActivateResponse> activate(
      $grpc.ServiceCall call, $0.RemoteServerActivateRequest request);

  $async.Future<$0.RemoteServerDeactivateResponse> deactivate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerDeactivateRequest> $request) async {
    return deactivate($call, await $request);
  }

  $async.Future<$0.RemoteServerDeactivateResponse> deactivate(
      $grpc.ServiceCall call, $0.RemoteServerDeactivateRequest request);

  $async.Future<$0.RemoteServerTestConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerTestConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.RemoteServerTestConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.RemoteServerTestConnectionRequest request);

  $async.Future<$1.Empty> deleteRemoteServer_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerDeleteRequest> $request) async {
    return deleteRemoteServer($call, await $request);
  }

  $async.Future<$1.Empty> deleteRemoteServer(
      $grpc.ServiceCall call, $0.RemoteServerDeleteRequest request);

  $async.Future<$0.RemoteServerMigrateDependenciesResponse>
      migrateDependencies_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.RemoteServerMigrateDependenciesRequest>
              $request) async {
    return migrateDependencies($call, await $request);
  }

  $async.Future<$0.RemoteServerMigrateDependenciesResponse> migrateDependencies(
      $grpc.ServiceCall call,
      $0.RemoteServerMigrateDependenciesRequest request);

  $async.Future<$0.RemoteServerGetMigrationResponse> getMigration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerGetMigrationRequest> $request) async {
    return getMigration($call, await $request);
  }

  $async.Future<$0.RemoteServerGetMigrationResponse> getMigration(
      $grpc.ServiceCall call, $0.RemoteServerGetMigrationRequest request);

  $async.Future<$0.RemoteServerRollbackMigrationResponse> rollbackMigration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerRollbackMigrationRequest> $request) async {
    return rollbackMigration($call, await $request);
  }

  $async.Future<$0.RemoteServerRollbackMigrationResponse> rollbackMigration(
      $grpc.ServiceCall call, $0.RemoteServerRollbackMigrationRequest request);

  $async.Future<$0.RemoteServerCancelMigrationResponse> cancelMigration_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerCancelMigrationRequest> $request) async {
    return cancelMigration($call, await $request);
  }

  $async.Future<$0.RemoteServerCancelMigrationResponse> cancelMigration(
      $grpc.ServiceCall call, $0.RemoteServerCancelMigrationRequest request);

  $async.Future<$0.RemoteServerMigrateServiceResponse> migrateService_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerMigrateServiceRequest> $request) async {
    return migrateService($call, await $request);
  }

  $async.Future<$0.RemoteServerMigrateServiceResponse> migrateService(
      $grpc.ServiceCall call, $0.RemoteServerMigrateServiceRequest request);

  $async.Future<$0.RemoteServerListMigrationsResponse> listMigrations_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemoteServerListMigrationsRequest> $request) async {
    return listMigrations($call, await $request);
  }

  $async.Future<$0.RemoteServerListMigrationsResponse> listMigrations(
      $grpc.ServiceCall call, $0.RemoteServerListMigrationsRequest request);

  $async.Future<$0.RemoteServerListMigrationObjectsResponse>
      listMigrationObjects_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.RemoteServerListMigrationObjectsRequest>
              $request) async {
    return listMigrationObjects($call, await $request);
  }

  $async.Future<$0.RemoteServerListMigrationObjectsResponse>
      listMigrationObjects($grpc.ServiceCall call,
          $0.RemoteServerListMigrationObjectsRequest request);
}

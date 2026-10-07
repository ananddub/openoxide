// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database.proto.

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
import 'database.pb.dart' as $0;

export 'database.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.DatabaseService')
class DatabaseServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DatabaseServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Database> getDatabase(
    $0.GetDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDatabasesResponse> listDatabases(
    $0.ListDatabasesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDatabases, request, options: options);
  }

  $grpc.ResponseFuture<$0.Database> createDatabase(
    $0.CreateDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.Database> updateDatabase(
    $0.UpdateDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDatabase(
    $0.DeleteDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseOperation> deploy(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseOperation> redeploy(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$redeploy, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseOperation> reload(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$reload, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseOperation> start(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$start, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseOperation> stop(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stop, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cancel(
    $0.DatabaseOperationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancel, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmSettings> getSwarmSettings(
    $0.SwarmSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSwarmSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmSettings> updateSwarmSettings(
    $0.UpdateSwarmSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSwarmSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.MountList> listMounts(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMounts, request, options: options);
  }

  $grpc.ResponseFuture<$0.Mount> createMount(
    $0.CreateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.Mount> updateMount(
    $0.UpdateMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateMount, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteMount(
    $0.DeleteMountRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteMount, request, options: options);
  }

  $grpc.ResponseFuture<$0.Dependencies> listDependencies(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDependencies, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseExport> exportDatabase(
    $0.ExportDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.Database> importDatabase(
    $0.ImportDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$importDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.DatabaseConnection> getConnection(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.Database> rotateCredentials(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateCredentials, request, options: options);
  }

  $grpc.ResponseFuture<$0.ValidationResult> validateDatabase(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$validateDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$0.AdvancedConfig> getPostgresAdvancedConfig(
    $0.DatabaseTargetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPostgresAdvancedConfig, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.AdvancedConfig> updatePostgresAdvancedConfig(
    $0.UpdateAdvancedConfigRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePostgresAdvancedConfig, request,
        options: options);
  }

  // method descriptors

  static final _$getDatabase =
      $grpc.ClientMethod<$0.GetDatabaseRequest, $0.Database>(
          '/openoxide.api.v1.DatabaseService/GetDatabase',
          ($0.GetDatabaseRequest value) => value.writeToBuffer(),
          $0.Database.fromBuffer);
  static final _$listDatabases =
      $grpc.ClientMethod<$0.ListDatabasesRequest, $0.ListDatabasesResponse>(
          '/openoxide.api.v1.DatabaseService/ListDatabases',
          ($0.ListDatabasesRequest value) => value.writeToBuffer(),
          $0.ListDatabasesResponse.fromBuffer);
  static final _$createDatabase =
      $grpc.ClientMethod<$0.CreateDatabaseRequest, $0.Database>(
          '/openoxide.api.v1.DatabaseService/CreateDatabase',
          ($0.CreateDatabaseRequest value) => value.writeToBuffer(),
          $0.Database.fromBuffer);
  static final _$updateDatabase =
      $grpc.ClientMethod<$0.UpdateDatabaseRequest, $0.Database>(
          '/openoxide.api.v1.DatabaseService/UpdateDatabase',
          ($0.UpdateDatabaseRequest value) => value.writeToBuffer(),
          $0.Database.fromBuffer);
  static final _$deleteDatabase =
      $grpc.ClientMethod<$0.DeleteDatabaseRequest, $1.Empty>(
          '/openoxide.api.v1.DatabaseService/DeleteDatabase',
          ($0.DeleteDatabaseRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deploy =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
          '/openoxide.api.v1.DatabaseService/Deploy',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $0.DatabaseOperation.fromBuffer);
  static final _$redeploy =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
          '/openoxide.api.v1.DatabaseService/Redeploy',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $0.DatabaseOperation.fromBuffer);
  static final _$reload =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
          '/openoxide.api.v1.DatabaseService/Reload',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $0.DatabaseOperation.fromBuffer);
  static final _$start =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
          '/openoxide.api.v1.DatabaseService/Start',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $0.DatabaseOperation.fromBuffer);
  static final _$stop =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
          '/openoxide.api.v1.DatabaseService/Stop',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $0.DatabaseOperation.fromBuffer);
  static final _$cancel =
      $grpc.ClientMethod<$0.DatabaseOperationRequest, $1.Empty>(
          '/openoxide.api.v1.DatabaseService/Cancel',
          ($0.DatabaseOperationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getSwarmSettings =
      $grpc.ClientMethod<$0.SwarmSettingsRequest, $0.SwarmSettings>(
          '/openoxide.api.v1.DatabaseService/GetSwarmSettings',
          ($0.SwarmSettingsRequest value) => value.writeToBuffer(),
          $0.SwarmSettings.fromBuffer);
  static final _$updateSwarmSettings =
      $grpc.ClientMethod<$0.UpdateSwarmSettingsRequest, $0.SwarmSettings>(
          '/openoxide.api.v1.DatabaseService/UpdateSwarmSettings',
          ($0.UpdateSwarmSettingsRequest value) => value.writeToBuffer(),
          $0.SwarmSettings.fromBuffer);
  static final _$listMounts =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.MountList>(
          '/openoxide.api.v1.DatabaseService/ListMounts',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.MountList.fromBuffer);
  static final _$createMount =
      $grpc.ClientMethod<$0.CreateMountRequest, $0.Mount>(
          '/openoxide.api.v1.DatabaseService/CreateMount',
          ($0.CreateMountRequest value) => value.writeToBuffer(),
          $0.Mount.fromBuffer);
  static final _$updateMount =
      $grpc.ClientMethod<$0.UpdateMountRequest, $0.Mount>(
          '/openoxide.api.v1.DatabaseService/UpdateMount',
          ($0.UpdateMountRequest value) => value.writeToBuffer(),
          $0.Mount.fromBuffer);
  static final _$deleteMount =
      $grpc.ClientMethod<$0.DeleteMountRequest, $1.Empty>(
          '/openoxide.api.v1.DatabaseService/DeleteMount',
          ($0.DeleteMountRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listDependencies =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.Dependencies>(
          '/openoxide.api.v1.DatabaseService/ListDependencies',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.Dependencies.fromBuffer);
  static final _$exportDatabase =
      $grpc.ClientMethod<$0.ExportDatabaseRequest, $0.DatabaseExport>(
          '/openoxide.api.v1.DatabaseService/ExportDatabase',
          ($0.ExportDatabaseRequest value) => value.writeToBuffer(),
          $0.DatabaseExport.fromBuffer);
  static final _$importDatabase =
      $grpc.ClientMethod<$0.ImportDatabaseRequest, $0.Database>(
          '/openoxide.api.v1.DatabaseService/ImportDatabase',
          ($0.ImportDatabaseRequest value) => value.writeToBuffer(),
          $0.Database.fromBuffer);
  static final _$getConnection =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.DatabaseConnection>(
          '/openoxide.api.v1.DatabaseService/GetConnection',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.DatabaseConnection.fromBuffer);
  static final _$rotateCredentials =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.Database>(
          '/openoxide.api.v1.DatabaseService/RotateCredentials',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.Database.fromBuffer);
  static final _$validateDatabase =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.ValidationResult>(
          '/openoxide.api.v1.DatabaseService/ValidateDatabase',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.ValidationResult.fromBuffer);
  static final _$getPostgresAdvancedConfig =
      $grpc.ClientMethod<$0.DatabaseTargetRequest, $0.AdvancedConfig>(
          '/openoxide.api.v1.DatabaseService/GetPostgresAdvancedConfig',
          ($0.DatabaseTargetRequest value) => value.writeToBuffer(),
          $0.AdvancedConfig.fromBuffer);
  static final _$updatePostgresAdvancedConfig =
      $grpc.ClientMethod<$0.UpdateAdvancedConfigRequest, $0.AdvancedConfig>(
          '/openoxide.api.v1.DatabaseService/UpdatePostgresAdvancedConfig',
          ($0.UpdateAdvancedConfigRequest value) => value.writeToBuffer(),
          $0.AdvancedConfig.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DatabaseService')
abstract class DatabaseServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DatabaseService';

  DatabaseServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetDatabaseRequest, $0.Database>(
        'GetDatabase',
        getDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDatabaseRequest.fromBuffer(value),
        ($0.Database value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListDatabasesRequest, $0.ListDatabasesResponse>(
            'ListDatabases',
            listDatabases_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListDatabasesRequest.fromBuffer(value),
            ($0.ListDatabasesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDatabaseRequest, $0.Database>(
        'CreateDatabase',
        createDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDatabaseRequest.fromBuffer(value),
        ($0.Database value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateDatabaseRequest, $0.Database>(
        'UpdateDatabase',
        updateDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateDatabaseRequest.fromBuffer(value),
        ($0.Database value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteDatabaseRequest, $1.Empty>(
        'DeleteDatabase',
        deleteDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteDatabaseRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
            'Deploy',
            deploy_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseOperationRequest.fromBuffer(value),
            ($0.DatabaseOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
            'Redeploy',
            redeploy_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseOperationRequest.fromBuffer(value),
            ($0.DatabaseOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
            'Reload',
            reload_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseOperationRequest.fromBuffer(value),
            ($0.DatabaseOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
            'Start',
            start_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseOperationRequest.fromBuffer(value),
            ($0.DatabaseOperation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseOperationRequest, $0.DatabaseOperation>(
            'Stop',
            stop_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseOperationRequest.fromBuffer(value),
            ($0.DatabaseOperation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseOperationRequest, $1.Empty>(
        'Cancel',
        cancel_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseOperationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmSettingsRequest, $0.SwarmSettings>(
        'GetSwarmSettings',
        getSwarmSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmSettingsRequest.fromBuffer(value),
        ($0.SwarmSettings value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateSwarmSettingsRequest, $0.SwarmSettings>(
            'UpdateSwarmSettings',
            updateSwarmSettings_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateSwarmSettingsRequest.fromBuffer(value),
            ($0.SwarmSettings value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.MountList>(
        'ListMounts',
        listMounts_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseTargetRequest.fromBuffer(value),
        ($0.MountList value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateMountRequest, $0.Mount>(
        'CreateMount',
        createMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateMountRequest.fromBuffer(value),
        ($0.Mount value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateMountRequest, $0.Mount>(
        'UpdateMount',
        updateMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateMountRequest.fromBuffer(value),
        ($0.Mount value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteMountRequest, $1.Empty>(
        'DeleteMount',
        deleteMount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteMountRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.Dependencies>(
        'ListDependencies',
        listDependencies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseTargetRequest.fromBuffer(value),
        ($0.Dependencies value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ExportDatabaseRequest, $0.DatabaseExport>(
        'ExportDatabase',
        exportDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ExportDatabaseRequest.fromBuffer(value),
        ($0.DatabaseExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ImportDatabaseRequest, $0.Database>(
        'ImportDatabase',
        importDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ImportDatabaseRequest.fromBuffer(value),
        ($0.Database value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.DatabaseConnection>(
            'GetConnection',
            getConnection_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseTargetRequest.fromBuffer(value),
            ($0.DatabaseConnection value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.Database>(
        'RotateCredentials',
        rotateCredentials_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseTargetRequest.fromBuffer(value),
        ($0.Database value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.ValidationResult>(
            'ValidateDatabase',
            validateDatabase_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DatabaseTargetRequest.fromBuffer(value),
            ($0.ValidationResult value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DatabaseTargetRequest, $0.AdvancedConfig>(
        'GetPostgresAdvancedConfig',
        getPostgresAdvancedConfig_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DatabaseTargetRequest.fromBuffer(value),
        ($0.AdvancedConfig value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateAdvancedConfigRequest, $0.AdvancedConfig>(
            'UpdatePostgresAdvancedConfig',
            updatePostgresAdvancedConfig_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateAdvancedConfigRequest.fromBuffer(value),
            ($0.AdvancedConfig value) => value.writeToBuffer()));
  }

  $async.Future<$0.Database> getDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetDatabaseRequest> $request) async {
    return getDatabase($call, await $request);
  }

  $async.Future<$0.Database> getDatabase(
      $grpc.ServiceCall call, $0.GetDatabaseRequest request);

  $async.Future<$0.ListDatabasesResponse> listDatabases_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDatabasesRequest> $request) async {
    return listDatabases($call, await $request);
  }

  $async.Future<$0.ListDatabasesResponse> listDatabases(
      $grpc.ServiceCall call, $0.ListDatabasesRequest request);

  $async.Future<$0.Database> createDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateDatabaseRequest> $request) async {
    return createDatabase($call, await $request);
  }

  $async.Future<$0.Database> createDatabase(
      $grpc.ServiceCall call, $0.CreateDatabaseRequest request);

  $async.Future<$0.Database> updateDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateDatabaseRequest> $request) async {
    return updateDatabase($call, await $request);
  }

  $async.Future<$0.Database> updateDatabase(
      $grpc.ServiceCall call, $0.UpdateDatabaseRequest request);

  $async.Future<$1.Empty> deleteDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteDatabaseRequest> $request) async {
    return deleteDatabase($call, await $request);
  }

  $async.Future<$1.Empty> deleteDatabase(
      $grpc.ServiceCall call, $0.DeleteDatabaseRequest request);

  $async.Future<$0.DatabaseOperation> deploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return deploy($call, await $request);
  }

  $async.Future<$0.DatabaseOperation> deploy(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$0.DatabaseOperation> redeploy_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return redeploy($call, await $request);
  }

  $async.Future<$0.DatabaseOperation> redeploy(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$0.DatabaseOperation> reload_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return reload($call, await $request);
  }

  $async.Future<$0.DatabaseOperation> reload(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$0.DatabaseOperation> start_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return start($call, await $request);
  }

  $async.Future<$0.DatabaseOperation> start(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$0.DatabaseOperation> stop_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return stop($call, await $request);
  }

  $async.Future<$0.DatabaseOperation> stop(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$1.Empty> cancel_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseOperationRequest> $request) async {
    return cancel($call, await $request);
  }

  $async.Future<$1.Empty> cancel(
      $grpc.ServiceCall call, $0.DatabaseOperationRequest request);

  $async.Future<$0.SwarmSettings> getSwarmSettings_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmSettingsRequest> $request) async {
    return getSwarmSettings($call, await $request);
  }

  $async.Future<$0.SwarmSettings> getSwarmSettings(
      $grpc.ServiceCall call, $0.SwarmSettingsRequest request);

  $async.Future<$0.SwarmSettings> updateSwarmSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateSwarmSettingsRequest> $request) async {
    return updateSwarmSettings($call, await $request);
  }

  $async.Future<$0.SwarmSettings> updateSwarmSettings(
      $grpc.ServiceCall call, $0.UpdateSwarmSettingsRequest request);

  $async.Future<$0.MountList> listMounts_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return listMounts($call, await $request);
  }

  $async.Future<$0.MountList> listMounts(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.Mount> createMount_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateMountRequest> $request) async {
    return createMount($call, await $request);
  }

  $async.Future<$0.Mount> createMount(
      $grpc.ServiceCall call, $0.CreateMountRequest request);

  $async.Future<$0.Mount> updateMount_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateMountRequest> $request) async {
    return updateMount($call, await $request);
  }

  $async.Future<$0.Mount> updateMount(
      $grpc.ServiceCall call, $0.UpdateMountRequest request);

  $async.Future<$1.Empty> deleteMount_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteMountRequest> $request) async {
    return deleteMount($call, await $request);
  }

  $async.Future<$1.Empty> deleteMount(
      $grpc.ServiceCall call, $0.DeleteMountRequest request);

  $async.Future<$0.Dependencies> listDependencies_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return listDependencies($call, await $request);
  }

  $async.Future<$0.Dependencies> listDependencies(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.DatabaseExport> exportDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExportDatabaseRequest> $request) async {
    return exportDatabase($call, await $request);
  }

  $async.Future<$0.DatabaseExport> exportDatabase(
      $grpc.ServiceCall call, $0.ExportDatabaseRequest request);

  $async.Future<$0.Database> importDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ImportDatabaseRequest> $request) async {
    return importDatabase($call, await $request);
  }

  $async.Future<$0.Database> importDatabase(
      $grpc.ServiceCall call, $0.ImportDatabaseRequest request);

  $async.Future<$0.DatabaseConnection> getConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return getConnection($call, await $request);
  }

  $async.Future<$0.DatabaseConnection> getConnection(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.Database> rotateCredentials_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return rotateCredentials($call, await $request);
  }

  $async.Future<$0.Database> rotateCredentials(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.ValidationResult> validateDatabase_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return validateDatabase($call, await $request);
  }

  $async.Future<$0.ValidationResult> validateDatabase(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.AdvancedConfig> getPostgresAdvancedConfig_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DatabaseTargetRequest> $request) async {
    return getPostgresAdvancedConfig($call, await $request);
  }

  $async.Future<$0.AdvancedConfig> getPostgresAdvancedConfig(
      $grpc.ServiceCall call, $0.DatabaseTargetRequest request);

  $async.Future<$0.AdvancedConfig> updatePostgresAdvancedConfig_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateAdvancedConfigRequest> $request) async {
    return updatePostgresAdvancedConfig($call, await $request);
  }

  $async.Future<$0.AdvancedConfig> updatePostgresAdvancedConfig(
      $grpc.ServiceCall call, $0.UpdateAdvancedConfigRequest request);
}

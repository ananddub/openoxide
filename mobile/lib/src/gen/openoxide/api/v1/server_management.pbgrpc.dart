// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_management.proto.

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

import 'server_management.pb.dart' as $0;

export 'server_management.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ServerManagementService')
class ServerManagementServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ServerManagementServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ServerManagementGetResponse> getManagement(
    $0.ServerManagementGetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getManagement, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementUpdateResponse> updateManagement(
    $0.ServerManagementUpdateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateManagement, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementRepairAuditResponse> repairAudit(
    $0.ServerManagementRepairAuditRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$repairAudit, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementConfigureGpuResponse> configureGpu(
    $0.ServerManagementConfigureGpuRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$configureGpu, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementRunCleanupResponse> runCleanup(
    $0.ServerManagementRunCleanupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runCleanup, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementListCleanupHistoryResponse>
      listCleanupHistory(
    $0.ServerManagementListCleanupHistoryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listCleanupHistory, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementUpgradeResponse> upgrade(
    $0.ServerManagementUpgradeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$upgrade, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementBackupResponse> backup(
    $0.ServerManagementBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$backup, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerManagementDiagnosticsResponse> diagnostics(
    $0.ServerManagementDiagnosticsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$diagnostics, request, options: options);
  }

  // method descriptors

  static final _$getManagement = $grpc.ClientMethod<
          $0.ServerManagementGetRequest, $0.ServerManagementGetResponse>(
      '/openoxide.api.v1.ServerManagementService/GetManagement',
      ($0.ServerManagementGetRequest value) => value.writeToBuffer(),
      $0.ServerManagementGetResponse.fromBuffer);
  static final _$updateManagement = $grpc.ClientMethod<
          $0.ServerManagementUpdateRequest, $0.ServerManagementUpdateResponse>(
      '/openoxide.api.v1.ServerManagementService/UpdateManagement',
      ($0.ServerManagementUpdateRequest value) => value.writeToBuffer(),
      $0.ServerManagementUpdateResponse.fromBuffer);
  static final _$repairAudit = $grpc.ClientMethod<
          $0.ServerManagementRepairAuditRequest,
          $0.ServerManagementRepairAuditResponse>(
      '/openoxide.api.v1.ServerManagementService/RepairAudit',
      ($0.ServerManagementRepairAuditRequest value) => value.writeToBuffer(),
      $0.ServerManagementRepairAuditResponse.fromBuffer);
  static final _$configureGpu = $grpc.ClientMethod<
          $0.ServerManagementConfigureGpuRequest,
          $0.ServerManagementConfigureGpuResponse>(
      '/openoxide.api.v1.ServerManagementService/ConfigureGpu',
      ($0.ServerManagementConfigureGpuRequest value) => value.writeToBuffer(),
      $0.ServerManagementConfigureGpuResponse.fromBuffer);
  static final _$runCleanup = $grpc.ClientMethod<
          $0.ServerManagementRunCleanupRequest,
          $0.ServerManagementRunCleanupResponse>(
      '/openoxide.api.v1.ServerManagementService/RunCleanup',
      ($0.ServerManagementRunCleanupRequest value) => value.writeToBuffer(),
      $0.ServerManagementRunCleanupResponse.fromBuffer);
  static final _$listCleanupHistory = $grpc.ClientMethod<
          $0.ServerManagementListCleanupHistoryRequest,
          $0.ServerManagementListCleanupHistoryResponse>(
      '/openoxide.api.v1.ServerManagementService/ListCleanupHistory',
      ($0.ServerManagementListCleanupHistoryRequest value) =>
          value.writeToBuffer(),
      $0.ServerManagementListCleanupHistoryResponse.fromBuffer);
  static final _$upgrade = $grpc.ClientMethod<$0.ServerManagementUpgradeRequest,
          $0.ServerManagementUpgradeResponse>(
      '/openoxide.api.v1.ServerManagementService/Upgrade',
      ($0.ServerManagementUpgradeRequest value) => value.writeToBuffer(),
      $0.ServerManagementUpgradeResponse.fromBuffer);
  static final _$backup = $grpc.ClientMethod<$0.ServerManagementBackupRequest,
          $0.ServerManagementBackupResponse>(
      '/openoxide.api.v1.ServerManagementService/Backup',
      ($0.ServerManagementBackupRequest value) => value.writeToBuffer(),
      $0.ServerManagementBackupResponse.fromBuffer);
  static final _$diagnostics = $grpc.ClientMethod<
          $0.ServerManagementDiagnosticsRequest,
          $0.ServerManagementDiagnosticsResponse>(
      '/openoxide.api.v1.ServerManagementService/Diagnostics',
      ($0.ServerManagementDiagnosticsRequest value) => value.writeToBuffer(),
      $0.ServerManagementDiagnosticsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ServerManagementService')
abstract class ServerManagementServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ServerManagementService';

  ServerManagementServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ServerManagementGetRequest,
            $0.ServerManagementGetResponse>(
        'GetManagement',
        getManagement_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementGetRequest.fromBuffer(value),
        ($0.ServerManagementGetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementUpdateRequest,
            $0.ServerManagementUpdateResponse>(
        'UpdateManagement',
        updateManagement_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementUpdateRequest.fromBuffer(value),
        ($0.ServerManagementUpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementRepairAuditRequest,
            $0.ServerManagementRepairAuditResponse>(
        'RepairAudit',
        repairAudit_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementRepairAuditRequest.fromBuffer(value),
        ($0.ServerManagementRepairAuditResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementConfigureGpuRequest,
            $0.ServerManagementConfigureGpuResponse>(
        'ConfigureGpu',
        configureGpu_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementConfigureGpuRequest.fromBuffer(value),
        ($0.ServerManagementConfigureGpuResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementRunCleanupRequest,
            $0.ServerManagementRunCleanupResponse>(
        'RunCleanup',
        runCleanup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementRunCleanupRequest.fromBuffer(value),
        ($0.ServerManagementRunCleanupResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementListCleanupHistoryRequest,
            $0.ServerManagementListCleanupHistoryResponse>(
        'ListCleanupHistory',
        listCleanupHistory_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementListCleanupHistoryRequest.fromBuffer(value),
        ($0.ServerManagementListCleanupHistoryResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementUpgradeRequest,
            $0.ServerManagementUpgradeResponse>(
        'Upgrade',
        upgrade_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementUpgradeRequest.fromBuffer(value),
        ($0.ServerManagementUpgradeResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementBackupRequest,
            $0.ServerManagementBackupResponse>(
        'Backup',
        backup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementBackupRequest.fromBuffer(value),
        ($0.ServerManagementBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerManagementDiagnosticsRequest,
            $0.ServerManagementDiagnosticsResponse>(
        'Diagnostics',
        diagnostics_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerManagementDiagnosticsRequest.fromBuffer(value),
        ($0.ServerManagementDiagnosticsResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.ServerManagementGetResponse> getManagement_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementGetRequest> $request) async {
    return getManagement($call, await $request);
  }

  $async.Future<$0.ServerManagementGetResponse> getManagement(
      $grpc.ServiceCall call, $0.ServerManagementGetRequest request);

  $async.Future<$0.ServerManagementUpdateResponse> updateManagement_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementUpdateRequest> $request) async {
    return updateManagement($call, await $request);
  }

  $async.Future<$0.ServerManagementUpdateResponse> updateManagement(
      $grpc.ServiceCall call, $0.ServerManagementUpdateRequest request);

  $async.Future<$0.ServerManagementRepairAuditResponse> repairAudit_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementRepairAuditRequest> $request) async {
    return repairAudit($call, await $request);
  }

  $async.Future<$0.ServerManagementRepairAuditResponse> repairAudit(
      $grpc.ServiceCall call, $0.ServerManagementRepairAuditRequest request);

  $async.Future<$0.ServerManagementConfigureGpuResponse> configureGpu_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementConfigureGpuRequest> $request) async {
    return configureGpu($call, await $request);
  }

  $async.Future<$0.ServerManagementConfigureGpuResponse> configureGpu(
      $grpc.ServiceCall call, $0.ServerManagementConfigureGpuRequest request);

  $async.Future<$0.ServerManagementRunCleanupResponse> runCleanup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementRunCleanupRequest> $request) async {
    return runCleanup($call, await $request);
  }

  $async.Future<$0.ServerManagementRunCleanupResponse> runCleanup(
      $grpc.ServiceCall call, $0.ServerManagementRunCleanupRequest request);

  $async.Future<$0.ServerManagementListCleanupHistoryResponse>
      listCleanupHistory_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ServerManagementListCleanupHistoryRequest>
              $request) async {
    return listCleanupHistory($call, await $request);
  }

  $async.Future<$0.ServerManagementListCleanupHistoryResponse>
      listCleanupHistory($grpc.ServiceCall call,
          $0.ServerManagementListCleanupHistoryRequest request);

  $async.Future<$0.ServerManagementUpgradeResponse> upgrade_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementUpgradeRequest> $request) async {
    return upgrade($call, await $request);
  }

  $async.Future<$0.ServerManagementUpgradeResponse> upgrade(
      $grpc.ServiceCall call, $0.ServerManagementUpgradeRequest request);

  $async.Future<$0.ServerManagementBackupResponse> backup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementBackupRequest> $request) async {
    return backup($call, await $request);
  }

  $async.Future<$0.ServerManagementBackupResponse> backup(
      $grpc.ServiceCall call, $0.ServerManagementBackupRequest request);

  $async.Future<$0.ServerManagementDiagnosticsResponse> diagnostics_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerManagementDiagnosticsRequest> $request) async {
    return diagnostics($call, await $request);
  }

  $async.Future<$0.ServerManagementDiagnosticsResponse> diagnostics(
      $grpc.ServiceCall call, $0.ServerManagementDiagnosticsRequest request);
}

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/backup.proto.

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

import 'backup.pb.dart' as $0;
import 'common.pb.dart' as $1;

export 'backup.pb.dart';

/// Sole backup owner service; absorbs the former BackupOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.BackupService')
class BackupServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  BackupServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.BackupListBackupsResponse> listBackups(
    $0.BackupListBackupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listBackups, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupGetBackupResponse> getBackup(
    $0.BackupGetBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupCreateBackupResponse> createBackup(
    $0.BackupCreateBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupUpdateBackupResponse> updateBackup(
    $0.BackupUpdateBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteBackup(
    $0.BackupDeleteBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> runBackup(
    $0.BackupRunBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> restoreBackup(
    $0.BackupRestoreBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$restoreBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupValidateDestinationResponse>
      validateDestination(
    $0.BackupValidateDestinationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$validateDestination, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupRunComposeBackupResponse> runComposeBackup(
    $0.BackupRunComposeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runComposeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupRunPanelBackupResponse> runPanelBackup(
    $0.BackupRunPanelBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runPanelBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupStageRestoreResponse> stageRestore(
    $0.BackupStageRestoreRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$stageRestore, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupGetRestoreStatusResponse> getRestoreStatus(
    $0.BackupGetRestoreStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRestoreStatus, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupRollbackRestoreResponse> rollbackRestore(
    $0.BackupRollbackRestoreRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rollbackRestore, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupListFilesResponse> listFiles(
    $0.BackupListFilesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listFiles, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupDownloadFileResponse> downloadFile(
    $0.BackupDownloadFileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$downloadFile, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupPreviewRetentionResponse> previewRetention(
    $0.BackupPreviewRetentionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$previewRetention, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupVerifyFilesResponse> verifyFiles(
    $0.BackupVerifyFilesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$verifyFiles, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupListExecutionsResponse> listExecutions(
    $0.BackupListExecutionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listExecutions, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> retryExecution(
    $0.BackupRetryExecutionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$retryExecution, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupListDatabaseBackupsResponse>
      listDatabaseBackups(
    $0.BackupListDatabaseBackupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDatabaseBackups, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupGetDatabaseBackupResponse> getDatabaseBackup(
    $0.BackupGetDatabaseBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDatabaseBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupCreateDatabaseBackupResponse>
      createDatabaseBackup(
    $0.BackupCreateDatabaseBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDatabaseBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupUpdateDatabaseBackupResponse>
      updateDatabaseBackup(
    $0.BackupUpdateDatabaseBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateDatabaseBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDatabaseBackup(
    $0.BackupDeleteDatabaseBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDatabaseBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> runDatabaseBackup(
    $0.BackupRunDatabaseBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runDatabaseBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupListVolumeBackupsResponse> listVolumeBackups(
    $0.BackupListVolumeBackupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listVolumeBackups, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupGetVolumeBackupResponse> getVolumeBackup(
    $0.BackupGetVolumeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getVolumeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupCreateVolumeBackupResponse> createVolumeBackup(
    $0.BackupCreateVolumeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createVolumeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$0.BackupUpdateVolumeBackupResponse> updateVolumeBackup(
    $0.BackupUpdateVolumeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateVolumeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteVolumeBackup(
    $0.BackupDeleteVolumeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteVolumeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> runVolumeBackup(
    $0.BackupRunVolumeBackupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$runVolumeBackup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> restoreDatabase(
    $0.BackupRestoreDatabaseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$restoreDatabase, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> restoreVolume(
    $0.BackupRestoreVolumeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$restoreVolume, request, options: options);
  }

  // method descriptors

  static final _$listBackups = $grpc.ClientMethod<$0.BackupListBackupsRequest,
          $0.BackupListBackupsResponse>(
      '/openoxide.api.v1.BackupService/ListBackups',
      ($0.BackupListBackupsRequest value) => value.writeToBuffer(),
      $0.BackupListBackupsResponse.fromBuffer);
  static final _$getBackup =
      $grpc.ClientMethod<$0.BackupGetBackupRequest, $0.BackupGetBackupResponse>(
          '/openoxide.api.v1.BackupService/GetBackup',
          ($0.BackupGetBackupRequest value) => value.writeToBuffer(),
          $0.BackupGetBackupResponse.fromBuffer);
  static final _$createBackup = $grpc.ClientMethod<$0.BackupCreateBackupRequest,
          $0.BackupCreateBackupResponse>(
      '/openoxide.api.v1.BackupService/CreateBackup',
      ($0.BackupCreateBackupRequest value) => value.writeToBuffer(),
      $0.BackupCreateBackupResponse.fromBuffer);
  static final _$updateBackup = $grpc.ClientMethod<$0.BackupUpdateBackupRequest,
          $0.BackupUpdateBackupResponse>(
      '/openoxide.api.v1.BackupService/UpdateBackup',
      ($0.BackupUpdateBackupRequest value) => value.writeToBuffer(),
      $0.BackupUpdateBackupResponse.fromBuffer);
  static final _$deleteBackup =
      $grpc.ClientMethod<$0.BackupDeleteBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/DeleteBackup',
          ($0.BackupDeleteBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$runBackup =
      $grpc.ClientMethod<$0.BackupRunBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RunBackup',
          ($0.BackupRunBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$restoreBackup =
      $grpc.ClientMethod<$0.BackupRestoreBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RestoreBackup',
          ($0.BackupRestoreBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$validateDestination = $grpc.ClientMethod<
          $0.BackupValidateDestinationRequest,
          $0.BackupValidateDestinationResponse>(
      '/openoxide.api.v1.BackupService/ValidateDestination',
      ($0.BackupValidateDestinationRequest value) => value.writeToBuffer(),
      $0.BackupValidateDestinationResponse.fromBuffer);
  static final _$runComposeBackup = $grpc.ClientMethod<
          $0.BackupRunComposeBackupRequest, $0.BackupRunComposeBackupResponse>(
      '/openoxide.api.v1.BackupService/RunComposeBackup',
      ($0.BackupRunComposeBackupRequest value) => value.writeToBuffer(),
      $0.BackupRunComposeBackupResponse.fromBuffer);
  static final _$runPanelBackup = $grpc.ClientMethod<
          $0.BackupRunPanelBackupRequest, $0.BackupRunPanelBackupResponse>(
      '/openoxide.api.v1.BackupService/RunPanelBackup',
      ($0.BackupRunPanelBackupRequest value) => value.writeToBuffer(),
      $0.BackupRunPanelBackupResponse.fromBuffer);
  static final _$stageRestore = $grpc.ClientMethod<$0.BackupStageRestoreRequest,
          $0.BackupStageRestoreResponse>(
      '/openoxide.api.v1.BackupService/StageRestore',
      ($0.BackupStageRestoreRequest value) => value.writeToBuffer(),
      $0.BackupStageRestoreResponse.fromBuffer);
  static final _$getRestoreStatus = $grpc.ClientMethod<
          $0.BackupGetRestoreStatusRequest, $0.BackupGetRestoreStatusResponse>(
      '/openoxide.api.v1.BackupService/GetRestoreStatus',
      ($0.BackupGetRestoreStatusRequest value) => value.writeToBuffer(),
      $0.BackupGetRestoreStatusResponse.fromBuffer);
  static final _$rollbackRestore = $grpc.ClientMethod<
          $0.BackupRollbackRestoreRequest, $0.BackupRollbackRestoreResponse>(
      '/openoxide.api.v1.BackupService/RollbackRestore',
      ($0.BackupRollbackRestoreRequest value) => value.writeToBuffer(),
      $0.BackupRollbackRestoreResponse.fromBuffer);
  static final _$listFiles =
      $grpc.ClientMethod<$0.BackupListFilesRequest, $0.BackupListFilesResponse>(
          '/openoxide.api.v1.BackupService/ListFiles',
          ($0.BackupListFilesRequest value) => value.writeToBuffer(),
          $0.BackupListFilesResponse.fromBuffer);
  static final _$downloadFile = $grpc.ClientMethod<$0.BackupDownloadFileRequest,
          $0.BackupDownloadFileResponse>(
      '/openoxide.api.v1.BackupService/DownloadFile',
      ($0.BackupDownloadFileRequest value) => value.writeToBuffer(),
      $0.BackupDownloadFileResponse.fromBuffer);
  static final _$previewRetention = $grpc.ClientMethod<
          $0.BackupPreviewRetentionRequest, $0.BackupPreviewRetentionResponse>(
      '/openoxide.api.v1.BackupService/PreviewRetention',
      ($0.BackupPreviewRetentionRequest value) => value.writeToBuffer(),
      $0.BackupPreviewRetentionResponse.fromBuffer);
  static final _$verifyFiles = $grpc.ClientMethod<$0.BackupVerifyFilesRequest,
          $0.BackupVerifyFilesResponse>(
      '/openoxide.api.v1.BackupService/VerifyFiles',
      ($0.BackupVerifyFilesRequest value) => value.writeToBuffer(),
      $0.BackupVerifyFilesResponse.fromBuffer);
  static final _$listExecutions = $grpc.ClientMethod<
          $0.BackupListExecutionsRequest, $0.BackupListExecutionsResponse>(
      '/openoxide.api.v1.BackupService/ListExecutions',
      ($0.BackupListExecutionsRequest value) => value.writeToBuffer(),
      $0.BackupListExecutionsResponse.fromBuffer);
  static final _$retryExecution =
      $grpc.ClientMethod<$0.BackupRetryExecutionRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RetryExecution',
          ($0.BackupRetryExecutionRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listDatabaseBackups = $grpc.ClientMethod<
          $0.BackupListDatabaseBackupsRequest,
          $0.BackupListDatabaseBackupsResponse>(
      '/openoxide.api.v1.BackupService/ListDatabaseBackups',
      ($0.BackupListDatabaseBackupsRequest value) => value.writeToBuffer(),
      $0.BackupListDatabaseBackupsResponse.fromBuffer);
  static final _$getDatabaseBackup = $grpc.ClientMethod<
          $0.BackupGetDatabaseBackupRequest,
          $0.BackupGetDatabaseBackupResponse>(
      '/openoxide.api.v1.BackupService/GetDatabaseBackup',
      ($0.BackupGetDatabaseBackupRequest value) => value.writeToBuffer(),
      $0.BackupGetDatabaseBackupResponse.fromBuffer);
  static final _$createDatabaseBackup = $grpc.ClientMethod<
          $0.BackupCreateDatabaseBackupRequest,
          $0.BackupCreateDatabaseBackupResponse>(
      '/openoxide.api.v1.BackupService/CreateDatabaseBackup',
      ($0.BackupCreateDatabaseBackupRequest value) => value.writeToBuffer(),
      $0.BackupCreateDatabaseBackupResponse.fromBuffer);
  static final _$updateDatabaseBackup = $grpc.ClientMethod<
          $0.BackupUpdateDatabaseBackupRequest,
          $0.BackupUpdateDatabaseBackupResponse>(
      '/openoxide.api.v1.BackupService/UpdateDatabaseBackup',
      ($0.BackupUpdateDatabaseBackupRequest value) => value.writeToBuffer(),
      $0.BackupUpdateDatabaseBackupResponse.fromBuffer);
  static final _$deleteDatabaseBackup =
      $grpc.ClientMethod<$0.BackupDeleteDatabaseBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/DeleteDatabaseBackup',
          ($0.BackupDeleteDatabaseBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$runDatabaseBackup =
      $grpc.ClientMethod<$0.BackupRunDatabaseBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RunDatabaseBackup',
          ($0.BackupRunDatabaseBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listVolumeBackups = $grpc.ClientMethod<
          $0.BackupListVolumeBackupsRequest,
          $0.BackupListVolumeBackupsResponse>(
      '/openoxide.api.v1.BackupService/ListVolumeBackups',
      ($0.BackupListVolumeBackupsRequest value) => value.writeToBuffer(),
      $0.BackupListVolumeBackupsResponse.fromBuffer);
  static final _$getVolumeBackup = $grpc.ClientMethod<
          $0.BackupGetVolumeBackupRequest, $0.BackupGetVolumeBackupResponse>(
      '/openoxide.api.v1.BackupService/GetVolumeBackup',
      ($0.BackupGetVolumeBackupRequest value) => value.writeToBuffer(),
      $0.BackupGetVolumeBackupResponse.fromBuffer);
  static final _$createVolumeBackup = $grpc.ClientMethod<
          $0.BackupCreateVolumeBackupRequest,
          $0.BackupCreateVolumeBackupResponse>(
      '/openoxide.api.v1.BackupService/CreateVolumeBackup',
      ($0.BackupCreateVolumeBackupRequest value) => value.writeToBuffer(),
      $0.BackupCreateVolumeBackupResponse.fromBuffer);
  static final _$updateVolumeBackup = $grpc.ClientMethod<
          $0.BackupUpdateVolumeBackupRequest,
          $0.BackupUpdateVolumeBackupResponse>(
      '/openoxide.api.v1.BackupService/UpdateVolumeBackup',
      ($0.BackupUpdateVolumeBackupRequest value) => value.writeToBuffer(),
      $0.BackupUpdateVolumeBackupResponse.fromBuffer);
  static final _$deleteVolumeBackup =
      $grpc.ClientMethod<$0.BackupDeleteVolumeBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/DeleteVolumeBackup',
          ($0.BackupDeleteVolumeBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$runVolumeBackup =
      $grpc.ClientMethod<$0.BackupRunVolumeBackupRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RunVolumeBackup',
          ($0.BackupRunVolumeBackupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$restoreDatabase =
      $grpc.ClientMethod<$0.BackupRestoreDatabaseRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RestoreDatabase',
          ($0.BackupRestoreDatabaseRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$restoreVolume =
      $grpc.ClientMethod<$0.BackupRestoreVolumeRequest, $1.Empty>(
          '/openoxide.api.v1.BackupService/RestoreVolume',
          ($0.BackupRestoreVolumeRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.BackupService')
abstract class BackupServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.BackupService';

  BackupServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.BackupListBackupsRequest,
            $0.BackupListBackupsResponse>(
        'ListBackups',
        listBackups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupListBackupsRequest.fromBuffer(value),
        ($0.BackupListBackupsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupGetBackupRequest,
            $0.BackupGetBackupResponse>(
        'GetBackup',
        getBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupGetBackupRequest.fromBuffer(value),
        ($0.BackupGetBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupCreateBackupRequest,
            $0.BackupCreateBackupResponse>(
        'CreateBackup',
        createBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupCreateBackupRequest.fromBuffer(value),
        ($0.BackupCreateBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupUpdateBackupRequest,
            $0.BackupUpdateBackupResponse>(
        'UpdateBackup',
        updateBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupUpdateBackupRequest.fromBuffer(value),
        ($0.BackupUpdateBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupDeleteBackupRequest, $1.Empty>(
        'DeleteBackup',
        deleteBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupDeleteBackupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRunBackupRequest, $1.Empty>(
        'RunBackup',
        runBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRunBackupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRestoreBackupRequest, $1.Empty>(
        'RestoreBackup',
        restoreBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRestoreBackupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupValidateDestinationRequest,
            $0.BackupValidateDestinationResponse>(
        'ValidateDestination',
        validateDestination_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupValidateDestinationRequest.fromBuffer(value),
        ($0.BackupValidateDestinationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRunComposeBackupRequest,
            $0.BackupRunComposeBackupResponse>(
        'RunComposeBackup',
        runComposeBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRunComposeBackupRequest.fromBuffer(value),
        ($0.BackupRunComposeBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRunPanelBackupRequest,
            $0.BackupRunPanelBackupResponse>(
        'RunPanelBackup',
        runPanelBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRunPanelBackupRequest.fromBuffer(value),
        ($0.BackupRunPanelBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupStageRestoreRequest,
            $0.BackupStageRestoreResponse>(
        'StageRestore',
        stageRestore_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupStageRestoreRequest.fromBuffer(value),
        ($0.BackupStageRestoreResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupGetRestoreStatusRequest,
            $0.BackupGetRestoreStatusResponse>(
        'GetRestoreStatus',
        getRestoreStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupGetRestoreStatusRequest.fromBuffer(value),
        ($0.BackupGetRestoreStatusResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRollbackRestoreRequest,
            $0.BackupRollbackRestoreResponse>(
        'RollbackRestore',
        rollbackRestore_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRollbackRestoreRequest.fromBuffer(value),
        ($0.BackupRollbackRestoreResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupListFilesRequest,
            $0.BackupListFilesResponse>(
        'ListFiles',
        listFiles_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupListFilesRequest.fromBuffer(value),
        ($0.BackupListFilesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupDownloadFileRequest,
            $0.BackupDownloadFileResponse>(
        'DownloadFile',
        downloadFile_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupDownloadFileRequest.fromBuffer(value),
        ($0.BackupDownloadFileResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupPreviewRetentionRequest,
            $0.BackupPreviewRetentionResponse>(
        'PreviewRetention',
        previewRetention_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupPreviewRetentionRequest.fromBuffer(value),
        ($0.BackupPreviewRetentionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupVerifyFilesRequest,
            $0.BackupVerifyFilesResponse>(
        'VerifyFiles',
        verifyFiles_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupVerifyFilesRequest.fromBuffer(value),
        ($0.BackupVerifyFilesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupListExecutionsRequest,
            $0.BackupListExecutionsResponse>(
        'ListExecutions',
        listExecutions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupListExecutionsRequest.fromBuffer(value),
        ($0.BackupListExecutionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRetryExecutionRequest, $1.Empty>(
        'RetryExecution',
        retryExecution_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRetryExecutionRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupListDatabaseBackupsRequest,
            $0.BackupListDatabaseBackupsResponse>(
        'ListDatabaseBackups',
        listDatabaseBackups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupListDatabaseBackupsRequest.fromBuffer(value),
        ($0.BackupListDatabaseBackupsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupGetDatabaseBackupRequest,
            $0.BackupGetDatabaseBackupResponse>(
        'GetDatabaseBackup',
        getDatabaseBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupGetDatabaseBackupRequest.fromBuffer(value),
        ($0.BackupGetDatabaseBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupCreateDatabaseBackupRequest,
            $0.BackupCreateDatabaseBackupResponse>(
        'CreateDatabaseBackup',
        createDatabaseBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupCreateDatabaseBackupRequest.fromBuffer(value),
        ($0.BackupCreateDatabaseBackupResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupUpdateDatabaseBackupRequest,
            $0.BackupUpdateDatabaseBackupResponse>(
        'UpdateDatabaseBackup',
        updateDatabaseBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupUpdateDatabaseBackupRequest.fromBuffer(value),
        ($0.BackupUpdateDatabaseBackupResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.BackupDeleteDatabaseBackupRequest, $1.Empty>(
            'DeleteDatabaseBackup',
            deleteDatabaseBackup_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.BackupDeleteDatabaseBackupRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRunDatabaseBackupRequest, $1.Empty>(
        'RunDatabaseBackup',
        runDatabaseBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRunDatabaseBackupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupListVolumeBackupsRequest,
            $0.BackupListVolumeBackupsResponse>(
        'ListVolumeBackups',
        listVolumeBackups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupListVolumeBackupsRequest.fromBuffer(value),
        ($0.BackupListVolumeBackupsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupGetVolumeBackupRequest,
            $0.BackupGetVolumeBackupResponse>(
        'GetVolumeBackup',
        getVolumeBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupGetVolumeBackupRequest.fromBuffer(value),
        ($0.BackupGetVolumeBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupCreateVolumeBackupRequest,
            $0.BackupCreateVolumeBackupResponse>(
        'CreateVolumeBackup',
        createVolumeBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupCreateVolumeBackupRequest.fromBuffer(value),
        ($0.BackupCreateVolumeBackupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupUpdateVolumeBackupRequest,
            $0.BackupUpdateVolumeBackupResponse>(
        'UpdateVolumeBackup',
        updateVolumeBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupUpdateVolumeBackupRequest.fromBuffer(value),
        ($0.BackupUpdateVolumeBackupResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.BackupDeleteVolumeBackupRequest, $1.Empty>(
            'DeleteVolumeBackup',
            deleteVolumeBackup_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.BackupDeleteVolumeBackupRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRunVolumeBackupRequest, $1.Empty>(
        'RunVolumeBackup',
        runVolumeBackup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRunVolumeBackupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRestoreDatabaseRequest, $1.Empty>(
        'RestoreDatabase',
        restoreDatabase_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRestoreDatabaseRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BackupRestoreVolumeRequest, $1.Empty>(
        'RestoreVolume',
        restoreVolume_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BackupRestoreVolumeRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.BackupListBackupsResponse> listBackups_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupListBackupsRequest> $request) async {
    return listBackups($call, await $request);
  }

  $async.Future<$0.BackupListBackupsResponse> listBackups(
      $grpc.ServiceCall call, $0.BackupListBackupsRequest request);

  $async.Future<$0.BackupGetBackupResponse> getBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupGetBackupRequest> $request) async {
    return getBackup($call, await $request);
  }

  $async.Future<$0.BackupGetBackupResponse> getBackup(
      $grpc.ServiceCall call, $0.BackupGetBackupRequest request);

  $async.Future<$0.BackupCreateBackupResponse> createBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupCreateBackupRequest> $request) async {
    return createBackup($call, await $request);
  }

  $async.Future<$0.BackupCreateBackupResponse> createBackup(
      $grpc.ServiceCall call, $0.BackupCreateBackupRequest request);

  $async.Future<$0.BackupUpdateBackupResponse> updateBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupUpdateBackupRequest> $request) async {
    return updateBackup($call, await $request);
  }

  $async.Future<$0.BackupUpdateBackupResponse> updateBackup(
      $grpc.ServiceCall call, $0.BackupUpdateBackupRequest request);

  $async.Future<$1.Empty> deleteBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupDeleteBackupRequest> $request) async {
    return deleteBackup($call, await $request);
  }

  $async.Future<$1.Empty> deleteBackup(
      $grpc.ServiceCall call, $0.BackupDeleteBackupRequest request);

  $async.Future<$1.Empty> runBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRunBackupRequest> $request) async {
    return runBackup($call, await $request);
  }

  $async.Future<$1.Empty> runBackup(
      $grpc.ServiceCall call, $0.BackupRunBackupRequest request);

  $async.Future<$1.Empty> restoreBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRestoreBackupRequest> $request) async {
    return restoreBackup($call, await $request);
  }

  $async.Future<$1.Empty> restoreBackup(
      $grpc.ServiceCall call, $0.BackupRestoreBackupRequest request);

  $async.Future<$0.BackupValidateDestinationResponse> validateDestination_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupValidateDestinationRequest> $request) async {
    return validateDestination($call, await $request);
  }

  $async.Future<$0.BackupValidateDestinationResponse> validateDestination(
      $grpc.ServiceCall call, $0.BackupValidateDestinationRequest request);

  $async.Future<$0.BackupRunComposeBackupResponse> runComposeBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupRunComposeBackupRequest> $request) async {
    return runComposeBackup($call, await $request);
  }

  $async.Future<$0.BackupRunComposeBackupResponse> runComposeBackup(
      $grpc.ServiceCall call, $0.BackupRunComposeBackupRequest request);

  $async.Future<$0.BackupRunPanelBackupResponse> runPanelBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupRunPanelBackupRequest> $request) async {
    return runPanelBackup($call, await $request);
  }

  $async.Future<$0.BackupRunPanelBackupResponse> runPanelBackup(
      $grpc.ServiceCall call, $0.BackupRunPanelBackupRequest request);

  $async.Future<$0.BackupStageRestoreResponse> stageRestore_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupStageRestoreRequest> $request) async {
    return stageRestore($call, await $request);
  }

  $async.Future<$0.BackupStageRestoreResponse> stageRestore(
      $grpc.ServiceCall call, $0.BackupStageRestoreRequest request);

  $async.Future<$0.BackupGetRestoreStatusResponse> getRestoreStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupGetRestoreStatusRequest> $request) async {
    return getRestoreStatus($call, await $request);
  }

  $async.Future<$0.BackupGetRestoreStatusResponse> getRestoreStatus(
      $grpc.ServiceCall call, $0.BackupGetRestoreStatusRequest request);

  $async.Future<$0.BackupRollbackRestoreResponse> rollbackRestore_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupRollbackRestoreRequest> $request) async {
    return rollbackRestore($call, await $request);
  }

  $async.Future<$0.BackupRollbackRestoreResponse> rollbackRestore(
      $grpc.ServiceCall call, $0.BackupRollbackRestoreRequest request);

  $async.Future<$0.BackupListFilesResponse> listFiles_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupListFilesRequest> $request) async {
    return listFiles($call, await $request);
  }

  $async.Future<$0.BackupListFilesResponse> listFiles(
      $grpc.ServiceCall call, $0.BackupListFilesRequest request);

  $async.Future<$0.BackupDownloadFileResponse> downloadFile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupDownloadFileRequest> $request) async {
    return downloadFile($call, await $request);
  }

  $async.Future<$0.BackupDownloadFileResponse> downloadFile(
      $grpc.ServiceCall call, $0.BackupDownloadFileRequest request);

  $async.Future<$0.BackupPreviewRetentionResponse> previewRetention_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupPreviewRetentionRequest> $request) async {
    return previewRetention($call, await $request);
  }

  $async.Future<$0.BackupPreviewRetentionResponse> previewRetention(
      $grpc.ServiceCall call, $0.BackupPreviewRetentionRequest request);

  $async.Future<$0.BackupVerifyFilesResponse> verifyFiles_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupVerifyFilesRequest> $request) async {
    return verifyFiles($call, await $request);
  }

  $async.Future<$0.BackupVerifyFilesResponse> verifyFiles(
      $grpc.ServiceCall call, $0.BackupVerifyFilesRequest request);

  $async.Future<$0.BackupListExecutionsResponse> listExecutions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupListExecutionsRequest> $request) async {
    return listExecutions($call, await $request);
  }

  $async.Future<$0.BackupListExecutionsResponse> listExecutions(
      $grpc.ServiceCall call, $0.BackupListExecutionsRequest request);

  $async.Future<$1.Empty> retryExecution_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRetryExecutionRequest> $request) async {
    return retryExecution($call, await $request);
  }

  $async.Future<$1.Empty> retryExecution(
      $grpc.ServiceCall call, $0.BackupRetryExecutionRequest request);

  $async.Future<$0.BackupListDatabaseBackupsResponse> listDatabaseBackups_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupListDatabaseBackupsRequest> $request) async {
    return listDatabaseBackups($call, await $request);
  }

  $async.Future<$0.BackupListDatabaseBackupsResponse> listDatabaseBackups(
      $grpc.ServiceCall call, $0.BackupListDatabaseBackupsRequest request);

  $async.Future<$0.BackupGetDatabaseBackupResponse> getDatabaseBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupGetDatabaseBackupRequest> $request) async {
    return getDatabaseBackup($call, await $request);
  }

  $async.Future<$0.BackupGetDatabaseBackupResponse> getDatabaseBackup(
      $grpc.ServiceCall call, $0.BackupGetDatabaseBackupRequest request);

  $async.Future<$0.BackupCreateDatabaseBackupResponse> createDatabaseBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupCreateDatabaseBackupRequest> $request) async {
    return createDatabaseBackup($call, await $request);
  }

  $async.Future<$0.BackupCreateDatabaseBackupResponse> createDatabaseBackup(
      $grpc.ServiceCall call, $0.BackupCreateDatabaseBackupRequest request);

  $async.Future<$0.BackupUpdateDatabaseBackupResponse> updateDatabaseBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupUpdateDatabaseBackupRequest> $request) async {
    return updateDatabaseBackup($call, await $request);
  }

  $async.Future<$0.BackupUpdateDatabaseBackupResponse> updateDatabaseBackup(
      $grpc.ServiceCall call, $0.BackupUpdateDatabaseBackupRequest request);

  $async.Future<$1.Empty> deleteDatabaseBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupDeleteDatabaseBackupRequest> $request) async {
    return deleteDatabaseBackup($call, await $request);
  }

  $async.Future<$1.Empty> deleteDatabaseBackup(
      $grpc.ServiceCall call, $0.BackupDeleteDatabaseBackupRequest request);

  $async.Future<$1.Empty> runDatabaseBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRunDatabaseBackupRequest> $request) async {
    return runDatabaseBackup($call, await $request);
  }

  $async.Future<$1.Empty> runDatabaseBackup(
      $grpc.ServiceCall call, $0.BackupRunDatabaseBackupRequest request);

  $async.Future<$0.BackupListVolumeBackupsResponse> listVolumeBackups_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupListVolumeBackupsRequest> $request) async {
    return listVolumeBackups($call, await $request);
  }

  $async.Future<$0.BackupListVolumeBackupsResponse> listVolumeBackups(
      $grpc.ServiceCall call, $0.BackupListVolumeBackupsRequest request);

  $async.Future<$0.BackupGetVolumeBackupResponse> getVolumeBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupGetVolumeBackupRequest> $request) async {
    return getVolumeBackup($call, await $request);
  }

  $async.Future<$0.BackupGetVolumeBackupResponse> getVolumeBackup(
      $grpc.ServiceCall call, $0.BackupGetVolumeBackupRequest request);

  $async.Future<$0.BackupCreateVolumeBackupResponse> createVolumeBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupCreateVolumeBackupRequest> $request) async {
    return createVolumeBackup($call, await $request);
  }

  $async.Future<$0.BackupCreateVolumeBackupResponse> createVolumeBackup(
      $grpc.ServiceCall call, $0.BackupCreateVolumeBackupRequest request);

  $async.Future<$0.BackupUpdateVolumeBackupResponse> updateVolumeBackup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BackupUpdateVolumeBackupRequest> $request) async {
    return updateVolumeBackup($call, await $request);
  }

  $async.Future<$0.BackupUpdateVolumeBackupResponse> updateVolumeBackup(
      $grpc.ServiceCall call, $0.BackupUpdateVolumeBackupRequest request);

  $async.Future<$1.Empty> deleteVolumeBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupDeleteVolumeBackupRequest> $request) async {
    return deleteVolumeBackup($call, await $request);
  }

  $async.Future<$1.Empty> deleteVolumeBackup(
      $grpc.ServiceCall call, $0.BackupDeleteVolumeBackupRequest request);

  $async.Future<$1.Empty> runVolumeBackup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRunVolumeBackupRequest> $request) async {
    return runVolumeBackup($call, await $request);
  }

  $async.Future<$1.Empty> runVolumeBackup(
      $grpc.ServiceCall call, $0.BackupRunVolumeBackupRequest request);

  $async.Future<$1.Empty> restoreDatabase_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRestoreDatabaseRequest> $request) async {
    return restoreDatabase($call, await $request);
  }

  $async.Future<$1.Empty> restoreDatabase(
      $grpc.ServiceCall call, $0.BackupRestoreDatabaseRequest request);

  $async.Future<$1.Empty> restoreVolume_Pre($grpc.ServiceCall $call,
      $async.Future<$0.BackupRestoreVolumeRequest> $request) async {
    return restoreVolume($call, await $request);
  }

  $async.Future<$1.Empty> restoreVolume(
      $grpc.ServiceCall call, $0.BackupRestoreVolumeRequest request);
}

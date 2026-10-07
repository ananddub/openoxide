// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/audit.proto.

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

import 'audit.pb.dart' as $0;

export 'audit.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.AuditService')
class AuditServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AuditServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.AuditListAuditLogsResponse> listAuditLogs(
    $0.AuditListAuditLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listAuditLogs, request, options: options);
  }

  $grpc.ResponseFuture<$0.AuditExportAuditLogsResponse> exportAuditLogs(
    $0.AuditExportAuditLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$exportAuditLogs, request, options: options);
  }

  // method descriptors

  static final _$listAuditLogs = $grpc.ClientMethod<
          $0.AuditListAuditLogsRequest, $0.AuditListAuditLogsResponse>(
      '/openoxide.api.v1.AuditService/ListAuditLogs',
      ($0.AuditListAuditLogsRequest value) => value.writeToBuffer(),
      $0.AuditListAuditLogsResponse.fromBuffer);
  static final _$exportAuditLogs = $grpc.ClientMethod<
          $0.AuditExportAuditLogsRequest, $0.AuditExportAuditLogsResponse>(
      '/openoxide.api.v1.AuditService/ExportAuditLogs',
      ($0.AuditExportAuditLogsRequest value) => value.writeToBuffer(),
      $0.AuditExportAuditLogsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.AuditService')
abstract class AuditServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.AuditService';

  AuditServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.AuditListAuditLogsRequest,
            $0.AuditListAuditLogsResponse>(
        'ListAuditLogs',
        listAuditLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AuditListAuditLogsRequest.fromBuffer(value),
        ($0.AuditListAuditLogsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AuditExportAuditLogsRequest,
            $0.AuditExportAuditLogsResponse>(
        'ExportAuditLogs',
        exportAuditLogs_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AuditExportAuditLogsRequest.fromBuffer(value),
        ($0.AuditExportAuditLogsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.AuditListAuditLogsResponse> listAuditLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AuditListAuditLogsRequest> $request) async {
    return listAuditLogs($call, await $request);
  }

  $async.Future<$0.AuditListAuditLogsResponse> listAuditLogs(
      $grpc.ServiceCall call, $0.AuditListAuditLogsRequest request);

  $async.Future<$0.AuditExportAuditLogsResponse> exportAuditLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AuditExportAuditLogsRequest> $request) async {
    return exportAuditLogs($call, await $request);
  }

  $async.Future<$0.AuditExportAuditLogsResponse> exportAuditLogs(
      $grpc.ServiceCall call, $0.AuditExportAuditLogsRequest request);
}

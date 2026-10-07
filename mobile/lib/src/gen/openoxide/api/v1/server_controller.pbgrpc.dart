// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/server_controller.proto.

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
import 'server_controller.pb.dart' as $0;

export 'server_controller.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.ServerControllerService')
class ServerControllerServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ServerControllerServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ServerControllerTestDirectConnectionResponse>
      testDirectConnection(
    $0.ServerControllerTestDirectConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testDirectConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerControllerTestConnectionResponse>
      testConnection(
    $0.ServerControllerTestConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerControllerAuditResponse> audit(
    $0.ServerControllerAuditRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$audit, request, options: options);
  }

  $grpc.ResponseFuture<$0.ServerControllerSetupResponse> setup(
    $0.ServerControllerSetupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setup, request, options: options);
  }

  $grpc.ResponseStream<$0.ServerControllerStreamSetupLogsResponse>
      streamSetupLogs(
    $0.ServerControllerStreamSetupLogsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$streamSetupLogs, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.ServerControllerListSessionsResponse> listSessions(
    $0.ServerControllerListSessionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listSessions, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteSession(
    $0.ServerControllerDeleteSessionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteSession, request, options: options);
  }

  // method descriptors

  static final _$testDirectConnection = $grpc.ClientMethod<
          $0.ServerControllerTestDirectConnectionRequest,
          $0.ServerControllerTestDirectConnectionResponse>(
      '/openoxide.api.v1.ServerControllerService/TestDirectConnection',
      ($0.ServerControllerTestDirectConnectionRequest value) =>
          value.writeToBuffer(),
      $0.ServerControllerTestDirectConnectionResponse.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<
          $0.ServerControllerTestConnectionRequest,
          $0.ServerControllerTestConnectionResponse>(
      '/openoxide.api.v1.ServerControllerService/TestConnection',
      ($0.ServerControllerTestConnectionRequest value) => value.writeToBuffer(),
      $0.ServerControllerTestConnectionResponse.fromBuffer);
  static final _$audit = $grpc.ClientMethod<$0.ServerControllerAuditRequest,
          $0.ServerControllerAuditResponse>(
      '/openoxide.api.v1.ServerControllerService/Audit',
      ($0.ServerControllerAuditRequest value) => value.writeToBuffer(),
      $0.ServerControllerAuditResponse.fromBuffer);
  static final _$setup = $grpc.ClientMethod<$0.ServerControllerSetupRequest,
          $0.ServerControllerSetupResponse>(
      '/openoxide.api.v1.ServerControllerService/Setup',
      ($0.ServerControllerSetupRequest value) => value.writeToBuffer(),
      $0.ServerControllerSetupResponse.fromBuffer);
  static final _$streamSetupLogs = $grpc.ClientMethod<
          $0.ServerControllerStreamSetupLogsRequest,
          $0.ServerControllerStreamSetupLogsResponse>(
      '/openoxide.api.v1.ServerControllerService/StreamSetupLogs',
      ($0.ServerControllerStreamSetupLogsRequest value) =>
          value.writeToBuffer(),
      $0.ServerControllerStreamSetupLogsResponse.fromBuffer);
  static final _$listSessions = $grpc.ClientMethod<
          $0.ServerControllerListSessionsRequest,
          $0.ServerControllerListSessionsResponse>(
      '/openoxide.api.v1.ServerControllerService/ListSessions',
      ($0.ServerControllerListSessionsRequest value) => value.writeToBuffer(),
      $0.ServerControllerListSessionsResponse.fromBuffer);
  static final _$deleteSession =
      $grpc.ClientMethod<$0.ServerControllerDeleteSessionRequest, $1.Empty>(
          '/openoxide.api.v1.ServerControllerService/DeleteSession',
          ($0.ServerControllerDeleteSessionRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.ServerControllerService')
abstract class ServerControllerServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.ServerControllerService';

  ServerControllerServiceBase() {
    $addMethod($grpc.ServiceMethod<
            $0.ServerControllerTestDirectConnectionRequest,
            $0.ServerControllerTestDirectConnectionResponse>(
        'TestDirectConnection',
        testDirectConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerControllerTestDirectConnectionRequest.fromBuffer(value),
        ($0.ServerControllerTestDirectConnectionResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerControllerTestConnectionRequest,
            $0.ServerControllerTestConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerControllerTestConnectionRequest.fromBuffer(value),
        ($0.ServerControllerTestConnectionResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerControllerAuditRequest,
            $0.ServerControllerAuditResponse>(
        'Audit',
        audit_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerControllerAuditRequest.fromBuffer(value),
        ($0.ServerControllerAuditResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerControllerSetupRequest,
            $0.ServerControllerSetupResponse>(
        'Setup',
        setup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerControllerSetupRequest.fromBuffer(value),
        ($0.ServerControllerSetupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerControllerStreamSetupLogsRequest,
            $0.ServerControllerStreamSetupLogsResponse>(
        'StreamSetupLogs',
        streamSetupLogs_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.ServerControllerStreamSetupLogsRequest.fromBuffer(value),
        ($0.ServerControllerStreamSetupLogsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ServerControllerListSessionsRequest,
            $0.ServerControllerListSessionsResponse>(
        'ListSessions',
        listSessions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ServerControllerListSessionsRequest.fromBuffer(value),
        ($0.ServerControllerListSessionsResponse value) =>
            value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ServerControllerDeleteSessionRequest, $1.Empty>(
            'DeleteSession',
            deleteSession_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ServerControllerDeleteSessionRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ServerControllerTestDirectConnectionResponse>
      testDirectConnection_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ServerControllerTestDirectConnectionRequest>
              $request) async {
    return testDirectConnection($call, await $request);
  }

  $async.Future<$0.ServerControllerTestDirectConnectionResponse>
      testDirectConnection($grpc.ServiceCall call,
          $0.ServerControllerTestDirectConnectionRequest request);

  $async.Future<$0.ServerControllerTestConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerTestConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.ServerControllerTestConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.ServerControllerTestConnectionRequest request);

  $async.Future<$0.ServerControllerAuditResponse> audit_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerAuditRequest> $request) async {
    return audit($call, await $request);
  }

  $async.Future<$0.ServerControllerAuditResponse> audit(
      $grpc.ServiceCall call, $0.ServerControllerAuditRequest request);

  $async.Future<$0.ServerControllerSetupResponse> setup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerSetupRequest> $request) async {
    return setup($call, await $request);
  }

  $async.Future<$0.ServerControllerSetupResponse> setup(
      $grpc.ServiceCall call, $0.ServerControllerSetupRequest request);

  $async.Stream<$0.ServerControllerStreamSetupLogsResponse> streamSetupLogs_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerStreamSetupLogsRequest>
          $request) async* {
    yield* streamSetupLogs($call, await $request);
  }

  $async.Stream<$0.ServerControllerStreamSetupLogsResponse> streamSetupLogs(
      $grpc.ServiceCall call,
      $0.ServerControllerStreamSetupLogsRequest request);

  $async.Future<$0.ServerControllerListSessionsResponse> listSessions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerListSessionsRequest> $request) async {
    return listSessions($call, await $request);
  }

  $async.Future<$0.ServerControllerListSessionsResponse> listSessions(
      $grpc.ServiceCall call, $0.ServerControllerListSessionsRequest request);

  $async.Future<$1.Empty> deleteSession_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ServerControllerDeleteSessionRequest> $request) async {
    return deleteSession($call, await $request);
  }

  $async.Future<$1.Empty> deleteSession(
      $grpc.ServiceCall call, $0.ServerControllerDeleteSessionRequest request);
}

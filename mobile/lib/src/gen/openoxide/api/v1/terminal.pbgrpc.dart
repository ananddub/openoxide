// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/terminal.proto.

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
import 'terminal.pb.dart' as $0;

export 'terminal.pb.dart';

/// Interactive terminal over the panel's ConnectRPC API. Browsers cannot stream request bodies over the Connect protocol, so the session is split in two legs: a server-streaming RPC carries all terminal output for the session lifetime, and a unary RPC feeds input, resize and stop commands addressed by the session id returned in the first `started` event.
@$pb.GrpcServiceName('openoxide.api.v1.TerminalService')
class TerminalServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  TerminalServiceClient(super.channel, {super.options, super.interceptors});

  /// Opens a Docker or SSH PTY session and streams its output. The first event is always `started` carrying the `session_id`; the stream ends after `exit` or `error`.
  $grpc.ResponseStream<$0.TerminalEvent> openTerminal(
    $0.OpenTerminalRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$openTerminal, $async.Stream.fromIterable([request]),
        options: options);
  }

  /// Sends a command (input / resize / stop) to an open terminal session.
  $grpc.ResponseFuture<$1.Empty> executionCommand(
    $0.ExecutionCommandRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$executionCommand, request, options: options);
  }

  // method descriptors

  static final _$openTerminal =
      $grpc.ClientMethod<$0.OpenTerminalRequest, $0.TerminalEvent>(
          '/openoxide.api.v1.TerminalService/OpenTerminal',
          ($0.OpenTerminalRequest value) => value.writeToBuffer(),
          $0.TerminalEvent.fromBuffer);
  static final _$executionCommand =
      $grpc.ClientMethod<$0.ExecutionCommandRequest, $1.Empty>(
          '/openoxide.api.v1.TerminalService/ExecutionCommand',
          ($0.ExecutionCommandRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.TerminalService')
abstract class TerminalServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.TerminalService';

  TerminalServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.OpenTerminalRequest, $0.TerminalEvent>(
        'OpenTerminal',
        openTerminal_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.OpenTerminalRequest.fromBuffer(value),
        ($0.TerminalEvent value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ExecutionCommandRequest, $1.Empty>(
        'ExecutionCommand',
        executionCommand_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ExecutionCommandRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Stream<$0.TerminalEvent> openTerminal_Pre($grpc.ServiceCall $call,
      $async.Future<$0.OpenTerminalRequest> $request) async* {
    yield* openTerminal($call, await $request);
  }

  $async.Stream<$0.TerminalEvent> openTerminal(
      $grpc.ServiceCall call, $0.OpenTerminalRequest request);

  $async.Future<$1.Empty> executionCommand_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExecutionCommandRequest> $request) async {
    return executionCommand($call, await $request);
  }

  $async.Future<$1.Empty> executionCommand(
      $grpc.ServiceCall call, $0.ExecutionCommandRequest request);
}

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/realtime.proto.

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

import 'common.pb.dart' as $0;
import 'realtime.pb.dart' as $1;

export 'realtime.pb.dart';

/// Table-level change notifications; the server never streams row data here, only which tables changed within the caller's organization. Clients refetch through their regular permission-checked queries, so realtime routing inherits query-level access control.
@$pb.GrpcServiceName('openoxide.api.v1.RealtimeService')
class RealtimeServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  RealtimeServiceClient(super.channel, {super.options, super.interceptors});

  /// Opens a per-organization watch stream; the first event always carries every table as a resync snapshot covering the gap between the initial fetch and the stream opening, later events carry only changed tables, and a burst of commits on one table coalesces into a single notification.
  $grpc.ResponseStream<$1.WatchTablesEvent> watchTables(
    $0.Empty request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$watchTables, $async.Stream.fromIterable([request]),
        options: options);
  }

  // method descriptors

  static final _$watchTables =
      $grpc.ClientMethod<$0.Empty, $1.WatchTablesEvent>(
          '/openoxide.api.v1.RealtimeService/WatchTables',
          ($0.Empty value) => value.writeToBuffer(),
          $1.WatchTablesEvent.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.RealtimeService')
abstract class RealtimeServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.RealtimeService';

  RealtimeServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.Empty, $1.WatchTablesEvent>(
        'WatchTables',
        watchTables_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.Empty.fromBuffer(value),
        ($1.WatchTablesEvent value) => value.writeToBuffer()));
  }

  $async.Stream<$1.WatchTablesEvent> watchTables_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Empty> $request) async* {
    yield* watchTables($call, await $request);
  }

  $async.Stream<$1.WatchTablesEvent> watchTables(
      $grpc.ServiceCall call, $0.Empty request);
}

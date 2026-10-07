// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/settings.proto.

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

import 'settings.pb.dart' as $0;

export 'settings.pb.dart';

/// Singleton panel settings row.
@$pb.GrpcServiceName('openoxide.api.v1.SettingsService')
class SettingsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SettingsServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.SettingsGetSettingsResponse> getSettings(
    $0.SettingsGetSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.SettingsUpdateSettingsResponse> updateSettings(
    $0.SettingsUpdateSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSettings, request, options: options);
  }

  // method descriptors

  static final _$getSettings = $grpc.ClientMethod<$0.SettingsGetSettingsRequest,
          $0.SettingsGetSettingsResponse>(
      '/openoxide.api.v1.SettingsService/GetSettings',
      ($0.SettingsGetSettingsRequest value) => value.writeToBuffer(),
      $0.SettingsGetSettingsResponse.fromBuffer);
  static final _$updateSettings = $grpc.ClientMethod<
          $0.SettingsUpdateSettingsRequest, $0.SettingsUpdateSettingsResponse>(
      '/openoxide.api.v1.SettingsService/UpdateSettings',
      ($0.SettingsUpdateSettingsRequest value) => value.writeToBuffer(),
      $0.SettingsUpdateSettingsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.SettingsService')
abstract class SettingsServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.SettingsService';

  SettingsServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SettingsGetSettingsRequest,
            $0.SettingsGetSettingsResponse>(
        'GetSettings',
        getSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SettingsGetSettingsRequest.fromBuffer(value),
        ($0.SettingsGetSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SettingsUpdateSettingsRequest,
            $0.SettingsUpdateSettingsResponse>(
        'UpdateSettings',
        updateSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SettingsUpdateSettingsRequest.fromBuffer(value),
        ($0.SettingsUpdateSettingsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.SettingsGetSettingsResponse> getSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SettingsGetSettingsRequest> $request) async {
    return getSettings($call, await $request);
  }

  $async.Future<$0.SettingsGetSettingsResponse> getSettings(
      $grpc.ServiceCall call, $0.SettingsGetSettingsRequest request);

  $async.Future<$0.SettingsUpdateSettingsResponse> updateSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SettingsUpdateSettingsRequest> $request) async {
    return updateSettings($call, await $request);
  }

  $async.Future<$0.SettingsUpdateSettingsResponse> updateSettings(
      $grpc.ServiceCall call, $0.SettingsUpdateSettingsRequest request);
}

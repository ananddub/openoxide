// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ssh_key.proto.

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
import 'ssh_key.pb.dart' as $0;

export 'ssh_key.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.SshKeyService')
class SshKeyServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SshKeyServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.SshKeyListKeysResponse> listKeys(
    $0.SshKeyListKeysRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listKeys, request, options: options);
  }

  $grpc.ResponseFuture<$0.SshKeyGetKeyResponse> getKey(
    $0.SshKeyGetKeyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getKey, request, options: options);
  }

  $grpc.ResponseFuture<$0.SshKeyCreateKeyResponse> createKey(
    $0.SshKeyCreateKeyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createKey, request, options: options);
  }

  $grpc.ResponseFuture<$0.SshKeyGenerateKeyResponse> generateKey(
    $0.SshKeyGenerateKeyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$generateKey, request, options: options);
  }

  $grpc.ResponseFuture<$0.SshKeyGeneratePairResponse> generatePair(
    $0.SshKeyGeneratePairRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$generatePair, request, options: options);
  }

  $grpc.ResponseFuture<$0.SshKeyUpdateKeyResponse> updateKey(
    $0.SshKeyUpdateKeyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateKey, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> markUsed(
    $0.SshKeyMarkUsedRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$markUsed, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteKey(
    $0.SshKeyDeleteKeyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteKey, request, options: options);
  }

  // method descriptors

  static final _$listKeys =
      $grpc.ClientMethod<$0.SshKeyListKeysRequest, $0.SshKeyListKeysResponse>(
          '/openoxide.api.v1.SshKeyService/ListKeys',
          ($0.SshKeyListKeysRequest value) => value.writeToBuffer(),
          $0.SshKeyListKeysResponse.fromBuffer);
  static final _$getKey =
      $grpc.ClientMethod<$0.SshKeyGetKeyRequest, $0.SshKeyGetKeyResponse>(
          '/openoxide.api.v1.SshKeyService/GetKey',
          ($0.SshKeyGetKeyRequest value) => value.writeToBuffer(),
          $0.SshKeyGetKeyResponse.fromBuffer);
  static final _$createKey =
      $grpc.ClientMethod<$0.SshKeyCreateKeyRequest, $0.SshKeyCreateKeyResponse>(
          '/openoxide.api.v1.SshKeyService/CreateKey',
          ($0.SshKeyCreateKeyRequest value) => value.writeToBuffer(),
          $0.SshKeyCreateKeyResponse.fromBuffer);
  static final _$generateKey = $grpc.ClientMethod<$0.SshKeyGenerateKeyRequest,
          $0.SshKeyGenerateKeyResponse>(
      '/openoxide.api.v1.SshKeyService/GenerateKey',
      ($0.SshKeyGenerateKeyRequest value) => value.writeToBuffer(),
      $0.SshKeyGenerateKeyResponse.fromBuffer);
  static final _$generatePair = $grpc.ClientMethod<$0.SshKeyGeneratePairRequest,
          $0.SshKeyGeneratePairResponse>(
      '/openoxide.api.v1.SshKeyService/GeneratePair',
      ($0.SshKeyGeneratePairRequest value) => value.writeToBuffer(),
      $0.SshKeyGeneratePairResponse.fromBuffer);
  static final _$updateKey =
      $grpc.ClientMethod<$0.SshKeyUpdateKeyRequest, $0.SshKeyUpdateKeyResponse>(
          '/openoxide.api.v1.SshKeyService/UpdateKey',
          ($0.SshKeyUpdateKeyRequest value) => value.writeToBuffer(),
          $0.SshKeyUpdateKeyResponse.fromBuffer);
  static final _$markUsed =
      $grpc.ClientMethod<$0.SshKeyMarkUsedRequest, $1.Empty>(
          '/openoxide.api.v1.SshKeyService/MarkUsed',
          ($0.SshKeyMarkUsedRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteKey =
      $grpc.ClientMethod<$0.SshKeyDeleteKeyRequest, $1.Empty>(
          '/openoxide.api.v1.SshKeyService/DeleteKey',
          ($0.SshKeyDeleteKeyRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.SshKeyService')
abstract class SshKeyServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.SshKeyService';

  SshKeyServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SshKeyListKeysRequest,
            $0.SshKeyListKeysResponse>(
        'ListKeys',
        listKeys_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyListKeysRequest.fromBuffer(value),
        ($0.SshKeyListKeysResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SshKeyGetKeyRequest, $0.SshKeyGetKeyResponse>(
            'GetKey',
            getKey_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SshKeyGetKeyRequest.fromBuffer(value),
            ($0.SshKeyGetKeyResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyCreateKeyRequest,
            $0.SshKeyCreateKeyResponse>(
        'CreateKey',
        createKey_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyCreateKeyRequest.fromBuffer(value),
        ($0.SshKeyCreateKeyResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyGenerateKeyRequest,
            $0.SshKeyGenerateKeyResponse>(
        'GenerateKey',
        generateKey_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyGenerateKeyRequest.fromBuffer(value),
        ($0.SshKeyGenerateKeyResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyGeneratePairRequest,
            $0.SshKeyGeneratePairResponse>(
        'GeneratePair',
        generatePair_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyGeneratePairRequest.fromBuffer(value),
        ($0.SshKeyGeneratePairResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyUpdateKeyRequest,
            $0.SshKeyUpdateKeyResponse>(
        'UpdateKey',
        updateKey_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyUpdateKeyRequest.fromBuffer(value),
        ($0.SshKeyUpdateKeyResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyMarkUsedRequest, $1.Empty>(
        'MarkUsed',
        markUsed_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyMarkUsedRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SshKeyDeleteKeyRequest, $1.Empty>(
        'DeleteKey',
        deleteKey_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SshKeyDeleteKeyRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.SshKeyListKeysResponse> listKeys_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SshKeyListKeysRequest> $request) async {
    return listKeys($call, await $request);
  }

  $async.Future<$0.SshKeyListKeysResponse> listKeys(
      $grpc.ServiceCall call, $0.SshKeyListKeysRequest request);

  $async.Future<$0.SshKeyGetKeyResponse> getKey_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SshKeyGetKeyRequest> $request) async {
    return getKey($call, await $request);
  }

  $async.Future<$0.SshKeyGetKeyResponse> getKey(
      $grpc.ServiceCall call, $0.SshKeyGetKeyRequest request);

  $async.Future<$0.SshKeyCreateKeyResponse> createKey_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SshKeyCreateKeyRequest> $request) async {
    return createKey($call, await $request);
  }

  $async.Future<$0.SshKeyCreateKeyResponse> createKey(
      $grpc.ServiceCall call, $0.SshKeyCreateKeyRequest request);

  $async.Future<$0.SshKeyGenerateKeyResponse> generateKey_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SshKeyGenerateKeyRequest> $request) async {
    return generateKey($call, await $request);
  }

  $async.Future<$0.SshKeyGenerateKeyResponse> generateKey(
      $grpc.ServiceCall call, $0.SshKeyGenerateKeyRequest request);

  $async.Future<$0.SshKeyGeneratePairResponse> generatePair_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SshKeyGeneratePairRequest> $request) async {
    return generatePair($call, await $request);
  }

  $async.Future<$0.SshKeyGeneratePairResponse> generatePair(
      $grpc.ServiceCall call, $0.SshKeyGeneratePairRequest request);

  $async.Future<$0.SshKeyUpdateKeyResponse> updateKey_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SshKeyUpdateKeyRequest> $request) async {
    return updateKey($call, await $request);
  }

  $async.Future<$0.SshKeyUpdateKeyResponse> updateKey(
      $grpc.ServiceCall call, $0.SshKeyUpdateKeyRequest request);

  $async.Future<$1.Empty> markUsed_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SshKeyMarkUsedRequest> $request) async {
    return markUsed($call, await $request);
  }

  $async.Future<$1.Empty> markUsed(
      $grpc.ServiceCall call, $0.SshKeyMarkUsedRequest request);

  $async.Future<$1.Empty> deleteKey_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SshKeyDeleteKeyRequest> $request) async {
    return deleteKey($call, await $request);
  }

  $async.Future<$1.Empty> deleteKey(
      $grpc.ServiceCall call, $0.SshKeyDeleteKeyRequest request);
}

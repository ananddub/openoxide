// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/vault.proto.

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
import 'vault.pb.dart' as $0;

export 'vault.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.VaultService')
class VaultServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  VaultServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListVaultProvidersResponse> listProviders(
    $0.ListVaultProvidersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listProviders, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetVaultProviderResponse> getProvider(
    $0.GetVaultProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateVaultProviderResponse> createProvider(
    $0.CreateVaultProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateVaultProviderResponse> updateProvider(
    $0.UpdateVaultProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateProvider, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteProvider(
    $0.DeleteVaultProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestVaultProviderResponse> testProvider(
    $0.TestVaultProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestVaultConnectionResponse> testConnection(
    $0.TestVaultConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListVaultSecretsResponse> listSecrets(
    $0.ListVaultSecretsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listSecrets, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListVaultAssignmentsResponse> listAssignments(
    $0.ListVaultAssignmentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listAssignments, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateVaultAssignmentsResponse> updateAssignments(
    $0.UpdateVaultAssignmentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateAssignments, request, options: options);
  }

  // method descriptors

  static final _$listProviders = $grpc.ClientMethod<
          $0.ListVaultProvidersRequest, $0.ListVaultProvidersResponse>(
      '/openoxide.api.v1.VaultService/ListProviders',
      ($0.ListVaultProvidersRequest value) => value.writeToBuffer(),
      $0.ListVaultProvidersResponse.fromBuffer);
  static final _$getProvider = $grpc.ClientMethod<$0.GetVaultProviderRequest,
          $0.GetVaultProviderResponse>(
      '/openoxide.api.v1.VaultService/GetProvider',
      ($0.GetVaultProviderRequest value) => value.writeToBuffer(),
      $0.GetVaultProviderResponse.fromBuffer);
  static final _$createProvider = $grpc.ClientMethod<
          $0.CreateVaultProviderRequest, $0.CreateVaultProviderResponse>(
      '/openoxide.api.v1.VaultService/CreateProvider',
      ($0.CreateVaultProviderRequest value) => value.writeToBuffer(),
      $0.CreateVaultProviderResponse.fromBuffer);
  static final _$updateProvider = $grpc.ClientMethod<
          $0.UpdateVaultProviderRequest, $0.UpdateVaultProviderResponse>(
      '/openoxide.api.v1.VaultService/UpdateProvider',
      ($0.UpdateVaultProviderRequest value) => value.writeToBuffer(),
      $0.UpdateVaultProviderResponse.fromBuffer);
  static final _$deleteProvider =
      $grpc.ClientMethod<$0.DeleteVaultProviderRequest, $1.Empty>(
          '/openoxide.api.v1.VaultService/DeleteProvider',
          ($0.DeleteVaultProviderRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testProvider = $grpc.ClientMethod<$0.TestVaultProviderRequest,
          $0.TestVaultProviderResponse>(
      '/openoxide.api.v1.VaultService/TestProvider',
      ($0.TestVaultProviderRequest value) => value.writeToBuffer(),
      $0.TestVaultProviderResponse.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<
          $0.TestVaultConnectionRequest, $0.TestVaultConnectionResponse>(
      '/openoxide.api.v1.VaultService/TestConnection',
      ($0.TestVaultConnectionRequest value) => value.writeToBuffer(),
      $0.TestVaultConnectionResponse.fromBuffer);
  static final _$listSecrets = $grpc.ClientMethod<$0.ListVaultSecretsRequest,
          $0.ListVaultSecretsResponse>(
      '/openoxide.api.v1.VaultService/ListSecrets',
      ($0.ListVaultSecretsRequest value) => value.writeToBuffer(),
      $0.ListVaultSecretsResponse.fromBuffer);
  static final _$listAssignments = $grpc.ClientMethod<
          $0.ListVaultAssignmentsRequest, $0.ListVaultAssignmentsResponse>(
      '/openoxide.api.v1.VaultService/ListAssignments',
      ($0.ListVaultAssignmentsRequest value) => value.writeToBuffer(),
      $0.ListVaultAssignmentsResponse.fromBuffer);
  static final _$updateAssignments = $grpc.ClientMethod<
          $0.UpdateVaultAssignmentsRequest, $0.UpdateVaultAssignmentsResponse>(
      '/openoxide.api.v1.VaultService/UpdateAssignments',
      ($0.UpdateVaultAssignmentsRequest value) => value.writeToBuffer(),
      $0.UpdateVaultAssignmentsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.VaultService')
abstract class VaultServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.VaultService';

  VaultServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListVaultProvidersRequest,
            $0.ListVaultProvidersResponse>(
        'ListProviders',
        listProviders_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListVaultProvidersRequest.fromBuffer(value),
        ($0.ListVaultProvidersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetVaultProviderRequest,
            $0.GetVaultProviderResponse>(
        'GetProvider',
        getProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetVaultProviderRequest.fromBuffer(value),
        ($0.GetVaultProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateVaultProviderRequest,
            $0.CreateVaultProviderResponse>(
        'CreateProvider',
        createProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateVaultProviderRequest.fromBuffer(value),
        ($0.CreateVaultProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateVaultProviderRequest,
            $0.UpdateVaultProviderResponse>(
        'UpdateProvider',
        updateProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateVaultProviderRequest.fromBuffer(value),
        ($0.UpdateVaultProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteVaultProviderRequest, $1.Empty>(
        'DeleteProvider',
        deleteProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteVaultProviderRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestVaultProviderRequest,
            $0.TestVaultProviderResponse>(
        'TestProvider',
        testProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestVaultProviderRequest.fromBuffer(value),
        ($0.TestVaultProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestVaultConnectionRequest,
            $0.TestVaultConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestVaultConnectionRequest.fromBuffer(value),
        ($0.TestVaultConnectionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListVaultSecretsRequest,
            $0.ListVaultSecretsResponse>(
        'ListSecrets',
        listSecrets_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListVaultSecretsRequest.fromBuffer(value),
        ($0.ListVaultSecretsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListVaultAssignmentsRequest,
            $0.ListVaultAssignmentsResponse>(
        'ListAssignments',
        listAssignments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListVaultAssignmentsRequest.fromBuffer(value),
        ($0.ListVaultAssignmentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateVaultAssignmentsRequest,
            $0.UpdateVaultAssignmentsResponse>(
        'UpdateAssignments',
        updateAssignments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateVaultAssignmentsRequest.fromBuffer(value),
        ($0.UpdateVaultAssignmentsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListVaultProvidersResponse> listProviders_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListVaultProvidersRequest> $request) async {
    return listProviders($call, await $request);
  }

  $async.Future<$0.ListVaultProvidersResponse> listProviders(
      $grpc.ServiceCall call, $0.ListVaultProvidersRequest request);

  $async.Future<$0.GetVaultProviderResponse> getProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetVaultProviderRequest> $request) async {
    return getProvider($call, await $request);
  }

  $async.Future<$0.GetVaultProviderResponse> getProvider(
      $grpc.ServiceCall call, $0.GetVaultProviderRequest request);

  $async.Future<$0.CreateVaultProviderResponse> createProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateVaultProviderRequest> $request) async {
    return createProvider($call, await $request);
  }

  $async.Future<$0.CreateVaultProviderResponse> createProvider(
      $grpc.ServiceCall call, $0.CreateVaultProviderRequest request);

  $async.Future<$0.UpdateVaultProviderResponse> updateProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateVaultProviderRequest> $request) async {
    return updateProvider($call, await $request);
  }

  $async.Future<$0.UpdateVaultProviderResponse> updateProvider(
      $grpc.ServiceCall call, $0.UpdateVaultProviderRequest request);

  $async.Future<$1.Empty> deleteProvider_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteVaultProviderRequest> $request) async {
    return deleteProvider($call, await $request);
  }

  $async.Future<$1.Empty> deleteProvider(
      $grpc.ServiceCall call, $0.DeleteVaultProviderRequest request);

  $async.Future<$0.TestVaultProviderResponse> testProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestVaultProviderRequest> $request) async {
    return testProvider($call, await $request);
  }

  $async.Future<$0.TestVaultProviderResponse> testProvider(
      $grpc.ServiceCall call, $0.TestVaultProviderRequest request);

  $async.Future<$0.TestVaultConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestVaultConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.TestVaultConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.TestVaultConnectionRequest request);

  $async.Future<$0.ListVaultSecretsResponse> listSecrets_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListVaultSecretsRequest> $request) async {
    return listSecrets($call, await $request);
  }

  $async.Future<$0.ListVaultSecretsResponse> listSecrets(
      $grpc.ServiceCall call, $0.ListVaultSecretsRequest request);

  $async.Future<$0.ListVaultAssignmentsResponse> listAssignments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListVaultAssignmentsRequest> $request) async {
    return listAssignments($call, await $request);
  }

  $async.Future<$0.ListVaultAssignmentsResponse> listAssignments(
      $grpc.ServiceCall call, $0.ListVaultAssignmentsRequest request);

  $async.Future<$0.UpdateVaultAssignmentsResponse> updateAssignments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateVaultAssignmentsRequest> $request) async {
    return updateAssignments($call, await $request);
  }

  $async.Future<$0.UpdateVaultAssignmentsResponse> updateAssignments(
      $grpc.ServiceCall call, $0.UpdateVaultAssignmentsRequest request);
}

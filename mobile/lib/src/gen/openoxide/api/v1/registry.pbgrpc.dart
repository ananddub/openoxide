// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/registry.proto.

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
import 'registry.pb.dart' as $0;

export 'registry.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.RegistryService')
class RegistryServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  RegistryServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListRegistriesResponse> listRegistries(
    $0.ListRegistriesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRegistries, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetRegistryResponse> getRegistry(
    $0.GetRegistryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getRegistry, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateRegistryResponse> createRegistry(
    $0.CreateRegistryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createRegistry, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateRegistryResponse> updateRegistry(
    $0.UpdateRegistryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRegistry, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteRegistry(
    $0.DeleteRegistryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRegistry, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestRegistryResponse> testRegistry(
    $0.TestRegistryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testRegistry, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListRegistryRepositoriesResponse> listRepositories(
    $0.ListRegistryRepositoriesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRepositories, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListRegistryTagsResponse> listTags(
    $0.ListRegistryTagsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listTags, request, options: options);
  }

  $grpc.ResponseFuture<$0.RotateRegistryCredentialsResponse> rotateCredentials(
    $0.RotateRegistryCredentialsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rotateCredentials, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetRegistryUsageResponse> getUsage(
    $0.GetRegistryUsageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getUsage, request, options: options);
  }

  // method descriptors

  static final _$listRegistries =
      $grpc.ClientMethod<$0.ListRegistriesRequest, $0.ListRegistriesResponse>(
          '/openoxide.api.v1.RegistryService/ListRegistries',
          ($0.ListRegistriesRequest value) => value.writeToBuffer(),
          $0.ListRegistriesResponse.fromBuffer);
  static final _$getRegistry =
      $grpc.ClientMethod<$0.GetRegistryRequest, $0.GetRegistryResponse>(
          '/openoxide.api.v1.RegistryService/GetRegistry',
          ($0.GetRegistryRequest value) => value.writeToBuffer(),
          $0.GetRegistryResponse.fromBuffer);
  static final _$createRegistry =
      $grpc.ClientMethod<$0.CreateRegistryRequest, $0.CreateRegistryResponse>(
          '/openoxide.api.v1.RegistryService/CreateRegistry',
          ($0.CreateRegistryRequest value) => value.writeToBuffer(),
          $0.CreateRegistryResponse.fromBuffer);
  static final _$updateRegistry =
      $grpc.ClientMethod<$0.UpdateRegistryRequest, $0.UpdateRegistryResponse>(
          '/openoxide.api.v1.RegistryService/UpdateRegistry',
          ($0.UpdateRegistryRequest value) => value.writeToBuffer(),
          $0.UpdateRegistryResponse.fromBuffer);
  static final _$deleteRegistry =
      $grpc.ClientMethod<$0.DeleteRegistryRequest, $1.Empty>(
          '/openoxide.api.v1.RegistryService/DeleteRegistry',
          ($0.DeleteRegistryRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testRegistry =
      $grpc.ClientMethod<$0.TestRegistryRequest, $0.TestRegistryResponse>(
          '/openoxide.api.v1.RegistryService/TestRegistry',
          ($0.TestRegistryRequest value) => value.writeToBuffer(),
          $0.TestRegistryResponse.fromBuffer);
  static final _$listRepositories = $grpc.ClientMethod<
          $0.ListRegistryRepositoriesRequest,
          $0.ListRegistryRepositoriesResponse>(
      '/openoxide.api.v1.RegistryService/ListRepositories',
      ($0.ListRegistryRepositoriesRequest value) => value.writeToBuffer(),
      $0.ListRegistryRepositoriesResponse.fromBuffer);
  static final _$listTags = $grpc.ClientMethod<$0.ListRegistryTagsRequest,
          $0.ListRegistryTagsResponse>(
      '/openoxide.api.v1.RegistryService/ListTags',
      ($0.ListRegistryTagsRequest value) => value.writeToBuffer(),
      $0.ListRegistryTagsResponse.fromBuffer);
  static final _$rotateCredentials = $grpc.ClientMethod<
          $0.RotateRegistryCredentialsRequest,
          $0.RotateRegistryCredentialsResponse>(
      '/openoxide.api.v1.RegistryService/RotateCredentials',
      ($0.RotateRegistryCredentialsRequest value) => value.writeToBuffer(),
      $0.RotateRegistryCredentialsResponse.fromBuffer);
  static final _$getUsage = $grpc.ClientMethod<$0.GetRegistryUsageRequest,
          $0.GetRegistryUsageResponse>(
      '/openoxide.api.v1.RegistryService/GetUsage',
      ($0.GetRegistryUsageRequest value) => value.writeToBuffer(),
      $0.GetRegistryUsageResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.RegistryService')
abstract class RegistryServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.RegistryService';

  RegistryServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListRegistriesRequest,
            $0.ListRegistriesResponse>(
        'ListRegistries',
        listRegistries_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListRegistriesRequest.fromBuffer(value),
        ($0.ListRegistriesResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetRegistryRequest, $0.GetRegistryResponse>(
            'GetRegistry',
            getRegistry_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetRegistryRequest.fromBuffer(value),
            ($0.GetRegistryResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateRegistryRequest,
            $0.CreateRegistryResponse>(
        'CreateRegistry',
        createRegistry_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateRegistryRequest.fromBuffer(value),
        ($0.CreateRegistryResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateRegistryRequest,
            $0.UpdateRegistryResponse>(
        'UpdateRegistry',
        updateRegistry_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateRegistryRequest.fromBuffer(value),
        ($0.UpdateRegistryResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteRegistryRequest, $1.Empty>(
        'DeleteRegistry',
        deleteRegistry_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteRegistryRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.TestRegistryRequest, $0.TestRegistryResponse>(
            'TestRegistry',
            testRegistry_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.TestRegistryRequest.fromBuffer(value),
            ($0.TestRegistryResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListRegistryRepositoriesRequest,
            $0.ListRegistryRepositoriesResponse>(
        'ListRepositories',
        listRepositories_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListRegistryRepositoriesRequest.fromBuffer(value),
        ($0.ListRegistryRepositoriesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListRegistryTagsRequest,
            $0.ListRegistryTagsResponse>(
        'ListTags',
        listTags_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListRegistryTagsRequest.fromBuffer(value),
        ($0.ListRegistryTagsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RotateRegistryCredentialsRequest,
            $0.RotateRegistryCredentialsResponse>(
        'RotateCredentials',
        rotateCredentials_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RotateRegistryCredentialsRequest.fromBuffer(value),
        ($0.RotateRegistryCredentialsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetRegistryUsageRequest,
            $0.GetRegistryUsageResponse>(
        'GetUsage',
        getUsage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetRegistryUsageRequest.fromBuffer(value),
        ($0.GetRegistryUsageResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListRegistriesResponse> listRegistries_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListRegistriesRequest> $request) async {
    return listRegistries($call, await $request);
  }

  $async.Future<$0.ListRegistriesResponse> listRegistries(
      $grpc.ServiceCall call, $0.ListRegistriesRequest request);

  $async.Future<$0.GetRegistryResponse> getRegistry_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetRegistryRequest> $request) async {
    return getRegistry($call, await $request);
  }

  $async.Future<$0.GetRegistryResponse> getRegistry(
      $grpc.ServiceCall call, $0.GetRegistryRequest request);

  $async.Future<$0.CreateRegistryResponse> createRegistry_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateRegistryRequest> $request) async {
    return createRegistry($call, await $request);
  }

  $async.Future<$0.CreateRegistryResponse> createRegistry(
      $grpc.ServiceCall call, $0.CreateRegistryRequest request);

  $async.Future<$0.UpdateRegistryResponse> updateRegistry_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateRegistryRequest> $request) async {
    return updateRegistry($call, await $request);
  }

  $async.Future<$0.UpdateRegistryResponse> updateRegistry(
      $grpc.ServiceCall call, $0.UpdateRegistryRequest request);

  $async.Future<$1.Empty> deleteRegistry_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteRegistryRequest> $request) async {
    return deleteRegistry($call, await $request);
  }

  $async.Future<$1.Empty> deleteRegistry(
      $grpc.ServiceCall call, $0.DeleteRegistryRequest request);

  $async.Future<$0.TestRegistryResponse> testRegistry_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestRegistryRequest> $request) async {
    return testRegistry($call, await $request);
  }

  $async.Future<$0.TestRegistryResponse> testRegistry(
      $grpc.ServiceCall call, $0.TestRegistryRequest request);

  $async.Future<$0.ListRegistryRepositoriesResponse> listRepositories_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListRegistryRepositoriesRequest> $request) async {
    return listRepositories($call, await $request);
  }

  $async.Future<$0.ListRegistryRepositoriesResponse> listRepositories(
      $grpc.ServiceCall call, $0.ListRegistryRepositoriesRequest request);

  $async.Future<$0.ListRegistryTagsResponse> listTags_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListRegistryTagsRequest> $request) async {
    return listTags($call, await $request);
  }

  $async.Future<$0.ListRegistryTagsResponse> listTags(
      $grpc.ServiceCall call, $0.ListRegistryTagsRequest request);

  $async.Future<$0.RotateRegistryCredentialsResponse> rotateCredentials_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RotateRegistryCredentialsRequest> $request) async {
    return rotateCredentials($call, await $request);
  }

  $async.Future<$0.RotateRegistryCredentialsResponse> rotateCredentials(
      $grpc.ServiceCall call, $0.RotateRegistryCredentialsRequest request);

  $async.Future<$0.GetRegistryUsageResponse> getUsage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetRegistryUsageRequest> $request) async {
    return getUsage($call, await $request);
  }

  $async.Future<$0.GetRegistryUsageResponse> getUsage(
      $grpc.ServiceCall call, $0.GetRegistryUsageRequest request);
}

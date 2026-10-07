// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/dns.proto.

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
import 'dns.pb.dart' as $0;

export 'dns.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.DnsService')
class DnsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DnsServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListDnsProvidersResponse> listProviders(
    $0.ListDnsProvidersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listProviders, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetDnsProviderResponse> getProvider(
    $0.GetDnsProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateDnsProviderResponse> createProvider(
    $0.CreateDnsProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateDnsProviderResponse> updateProvider(
    $0.UpdateDnsProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateProvider, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteProvider(
    $0.DeleteDnsProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestDnsProviderResponse> testProvider(
    $0.TestDnsProviderRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testProvider, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestDnsConnectionResponse> testConnection(
    $0.TestDnsConnectionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDnsZonesResponse> listZones(
    $0.ListDnsZonesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listZones, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDnsRecordsResponse> listRecords(
    $0.ListDnsRecordsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRecords, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateDnsRecordResponse> createRecord(
    $0.CreateDnsRecordRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createRecord, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateDnsRecordResponse> updateRecord(
    $0.UpdateDnsRecordRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateRecord, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteRecord(
    $0.DeleteDnsRecordRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteRecord, request, options: options);
  }

  // method descriptors

  static final _$listProviders = $grpc.ClientMethod<$0.ListDnsProvidersRequest,
          $0.ListDnsProvidersResponse>(
      '/openoxide.api.v1.DnsService/ListProviders',
      ($0.ListDnsProvidersRequest value) => value.writeToBuffer(),
      $0.ListDnsProvidersResponse.fromBuffer);
  static final _$getProvider =
      $grpc.ClientMethod<$0.GetDnsProviderRequest, $0.GetDnsProviderResponse>(
          '/openoxide.api.v1.DnsService/GetProvider',
          ($0.GetDnsProviderRequest value) => value.writeToBuffer(),
          $0.GetDnsProviderResponse.fromBuffer);
  static final _$createProvider = $grpc.ClientMethod<
          $0.CreateDnsProviderRequest, $0.CreateDnsProviderResponse>(
      '/openoxide.api.v1.DnsService/CreateProvider',
      ($0.CreateDnsProviderRequest value) => value.writeToBuffer(),
      $0.CreateDnsProviderResponse.fromBuffer);
  static final _$updateProvider = $grpc.ClientMethod<
          $0.UpdateDnsProviderRequest, $0.UpdateDnsProviderResponse>(
      '/openoxide.api.v1.DnsService/UpdateProvider',
      ($0.UpdateDnsProviderRequest value) => value.writeToBuffer(),
      $0.UpdateDnsProviderResponse.fromBuffer);
  static final _$deleteProvider =
      $grpc.ClientMethod<$0.DeleteDnsProviderRequest, $1.Empty>(
          '/openoxide.api.v1.DnsService/DeleteProvider',
          ($0.DeleteDnsProviderRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testProvider =
      $grpc.ClientMethod<$0.TestDnsProviderRequest, $0.TestDnsProviderResponse>(
          '/openoxide.api.v1.DnsService/TestProvider',
          ($0.TestDnsProviderRequest value) => value.writeToBuffer(),
          $0.TestDnsProviderResponse.fromBuffer);
  static final _$testConnection = $grpc.ClientMethod<
          $0.TestDnsConnectionRequest, $0.TestDnsConnectionResponse>(
      '/openoxide.api.v1.DnsService/TestConnection',
      ($0.TestDnsConnectionRequest value) => value.writeToBuffer(),
      $0.TestDnsConnectionResponse.fromBuffer);
  static final _$listZones =
      $grpc.ClientMethod<$0.ListDnsZonesRequest, $0.ListDnsZonesResponse>(
          '/openoxide.api.v1.DnsService/ListZones',
          ($0.ListDnsZonesRequest value) => value.writeToBuffer(),
          $0.ListDnsZonesResponse.fromBuffer);
  static final _$listRecords =
      $grpc.ClientMethod<$0.ListDnsRecordsRequest, $0.ListDnsRecordsResponse>(
          '/openoxide.api.v1.DnsService/ListRecords',
          ($0.ListDnsRecordsRequest value) => value.writeToBuffer(),
          $0.ListDnsRecordsResponse.fromBuffer);
  static final _$createRecord =
      $grpc.ClientMethod<$0.CreateDnsRecordRequest, $0.CreateDnsRecordResponse>(
          '/openoxide.api.v1.DnsService/CreateRecord',
          ($0.CreateDnsRecordRequest value) => value.writeToBuffer(),
          $0.CreateDnsRecordResponse.fromBuffer);
  static final _$updateRecord =
      $grpc.ClientMethod<$0.UpdateDnsRecordRequest, $0.UpdateDnsRecordResponse>(
          '/openoxide.api.v1.DnsService/UpdateRecord',
          ($0.UpdateDnsRecordRequest value) => value.writeToBuffer(),
          $0.UpdateDnsRecordResponse.fromBuffer);
  static final _$deleteRecord =
      $grpc.ClientMethod<$0.DeleteDnsRecordRequest, $1.Empty>(
          '/openoxide.api.v1.DnsService/DeleteRecord',
          ($0.DeleteDnsRecordRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DnsService')
abstract class DnsServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DnsService';

  DnsServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListDnsProvidersRequest,
            $0.ListDnsProvidersResponse>(
        'ListProviders',
        listProviders_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListDnsProvidersRequest.fromBuffer(value),
        ($0.ListDnsProvidersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDnsProviderRequest,
            $0.GetDnsProviderResponse>(
        'GetProvider',
        getProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDnsProviderRequest.fromBuffer(value),
        ($0.GetDnsProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDnsProviderRequest,
            $0.CreateDnsProviderResponse>(
        'CreateProvider',
        createProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDnsProviderRequest.fromBuffer(value),
        ($0.CreateDnsProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateDnsProviderRequest,
            $0.UpdateDnsProviderResponse>(
        'UpdateProvider',
        updateProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateDnsProviderRequest.fromBuffer(value),
        ($0.UpdateDnsProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteDnsProviderRequest, $1.Empty>(
        'DeleteProvider',
        deleteProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteDnsProviderRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestDnsProviderRequest,
            $0.TestDnsProviderResponse>(
        'TestProvider',
        testProvider_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestDnsProviderRequest.fromBuffer(value),
        ($0.TestDnsProviderResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestDnsConnectionRequest,
            $0.TestDnsConnectionResponse>(
        'TestConnection',
        testConnection_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestDnsConnectionRequest.fromBuffer(value),
        ($0.TestDnsConnectionResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListDnsZonesRequest, $0.ListDnsZonesResponse>(
            'ListZones',
            listZones_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListDnsZonesRequest.fromBuffer(value),
            ($0.ListDnsZonesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListDnsRecordsRequest,
            $0.ListDnsRecordsResponse>(
        'ListRecords',
        listRecords_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListDnsRecordsRequest.fromBuffer(value),
        ($0.ListDnsRecordsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDnsRecordRequest,
            $0.CreateDnsRecordResponse>(
        'CreateRecord',
        createRecord_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDnsRecordRequest.fromBuffer(value),
        ($0.CreateDnsRecordResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateDnsRecordRequest,
            $0.UpdateDnsRecordResponse>(
        'UpdateRecord',
        updateRecord_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateDnsRecordRequest.fromBuffer(value),
        ($0.UpdateDnsRecordResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteDnsRecordRequest, $1.Empty>(
        'DeleteRecord',
        deleteRecord_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteDnsRecordRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListDnsProvidersResponse> listProviders_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDnsProvidersRequest> $request) async {
    return listProviders($call, await $request);
  }

  $async.Future<$0.ListDnsProvidersResponse> listProviders(
      $grpc.ServiceCall call, $0.ListDnsProvidersRequest request);

  $async.Future<$0.GetDnsProviderResponse> getProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetDnsProviderRequest> $request) async {
    return getProvider($call, await $request);
  }

  $async.Future<$0.GetDnsProviderResponse> getProvider(
      $grpc.ServiceCall call, $0.GetDnsProviderRequest request);

  $async.Future<$0.CreateDnsProviderResponse> createProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateDnsProviderRequest> $request) async {
    return createProvider($call, await $request);
  }

  $async.Future<$0.CreateDnsProviderResponse> createProvider(
      $grpc.ServiceCall call, $0.CreateDnsProviderRequest request);

  $async.Future<$0.UpdateDnsProviderResponse> updateProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateDnsProviderRequest> $request) async {
    return updateProvider($call, await $request);
  }

  $async.Future<$0.UpdateDnsProviderResponse> updateProvider(
      $grpc.ServiceCall call, $0.UpdateDnsProviderRequest request);

  $async.Future<$1.Empty> deleteProvider_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteDnsProviderRequest> $request) async {
    return deleteProvider($call, await $request);
  }

  $async.Future<$1.Empty> deleteProvider(
      $grpc.ServiceCall call, $0.DeleteDnsProviderRequest request);

  $async.Future<$0.TestDnsProviderResponse> testProvider_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestDnsProviderRequest> $request) async {
    return testProvider($call, await $request);
  }

  $async.Future<$0.TestDnsProviderResponse> testProvider(
      $grpc.ServiceCall call, $0.TestDnsProviderRequest request);

  $async.Future<$0.TestDnsConnectionResponse> testConnection_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestDnsConnectionRequest> $request) async {
    return testConnection($call, await $request);
  }

  $async.Future<$0.TestDnsConnectionResponse> testConnection(
      $grpc.ServiceCall call, $0.TestDnsConnectionRequest request);

  $async.Future<$0.ListDnsZonesResponse> listZones_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ListDnsZonesRequest> $request) async {
    return listZones($call, await $request);
  }

  $async.Future<$0.ListDnsZonesResponse> listZones(
      $grpc.ServiceCall call, $0.ListDnsZonesRequest request);

  $async.Future<$0.ListDnsRecordsResponse> listRecords_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDnsRecordsRequest> $request) async {
    return listRecords($call, await $request);
  }

  $async.Future<$0.ListDnsRecordsResponse> listRecords(
      $grpc.ServiceCall call, $0.ListDnsRecordsRequest request);

  $async.Future<$0.CreateDnsRecordResponse> createRecord_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateDnsRecordRequest> $request) async {
    return createRecord($call, await $request);
  }

  $async.Future<$0.CreateDnsRecordResponse> createRecord(
      $grpc.ServiceCall call, $0.CreateDnsRecordRequest request);

  $async.Future<$0.UpdateDnsRecordResponse> updateRecord_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateDnsRecordRequest> $request) async {
    return updateRecord($call, await $request);
  }

  $async.Future<$0.UpdateDnsRecordResponse> updateRecord(
      $grpc.ServiceCall call, $0.UpdateDnsRecordRequest request);

  $async.Future<$1.Empty> deleteRecord_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteDnsRecordRequest> $request) async {
    return deleteRecord($call, await $request);
  }

  $async.Future<$1.Empty> deleteRecord(
      $grpc.ServiceCall call, $0.DeleteDnsRecordRequest request);
}

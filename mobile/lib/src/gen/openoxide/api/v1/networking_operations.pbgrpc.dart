// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/networking_operations.proto.

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

import 'networking_operations.pb.dart' as $0;

export 'networking_operations.pb.dart';

/// Networking utilities: CDN purge plus domain and root-overlay-network diagnostics. Not a domain/certificate/DNS CRUD owner service.
@$pb.GrpcServiceName('openoxide.api.v1.NetworkingOperationsService')
class NetworkingOperationsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  NetworkingOperationsServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.NetworkingPurgeCdnResponse> purgeCdn(
    $0.NetworkingPurgeCdnRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$purgeCdn, request, options: options);
  }

  $grpc.ResponseFuture<$0.NetworkingDiagnoseDomainsResponse> diagnoseDomains(
    $0.NetworkingDiagnoseDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$diagnoseDomains, request, options: options);
  }

  $grpc.ResponseFuture<$0.NetworkingDiagnoseRootNetworkResponse>
      diagnoseRootNetwork(
    $0.NetworkingDiagnoseRootNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$diagnoseRootNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.NetworkingRepairRootNetworkResponse>
      repairRootNetwork(
    $0.NetworkingRepairRootNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$repairRootNetwork, request, options: options);
  }

  // method descriptors

  static final _$purgeCdn = $grpc.ClientMethod<$0.NetworkingPurgeCdnRequest,
          $0.NetworkingPurgeCdnResponse>(
      '/openoxide.api.v1.NetworkingOperationsService/PurgeCdn',
      ($0.NetworkingPurgeCdnRequest value) => value.writeToBuffer(),
      $0.NetworkingPurgeCdnResponse.fromBuffer);
  static final _$diagnoseDomains = $grpc.ClientMethod<
          $0.NetworkingDiagnoseDomainsRequest,
          $0.NetworkingDiagnoseDomainsResponse>(
      '/openoxide.api.v1.NetworkingOperationsService/DiagnoseDomains',
      ($0.NetworkingDiagnoseDomainsRequest value) => value.writeToBuffer(),
      $0.NetworkingDiagnoseDomainsResponse.fromBuffer);
  static final _$diagnoseRootNetwork = $grpc.ClientMethod<
          $0.NetworkingDiagnoseRootNetworkRequest,
          $0.NetworkingDiagnoseRootNetworkResponse>(
      '/openoxide.api.v1.NetworkingOperationsService/DiagnoseRootNetwork',
      ($0.NetworkingDiagnoseRootNetworkRequest value) => value.writeToBuffer(),
      $0.NetworkingDiagnoseRootNetworkResponse.fromBuffer);
  static final _$repairRootNetwork = $grpc.ClientMethod<
          $0.NetworkingRepairRootNetworkRequest,
          $0.NetworkingRepairRootNetworkResponse>(
      '/openoxide.api.v1.NetworkingOperationsService/RepairRootNetwork',
      ($0.NetworkingRepairRootNetworkRequest value) => value.writeToBuffer(),
      $0.NetworkingRepairRootNetworkResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.NetworkingOperationsService')
abstract class NetworkingOperationsServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.NetworkingOperationsService';

  NetworkingOperationsServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.NetworkingPurgeCdnRequest,
            $0.NetworkingPurgeCdnResponse>(
        'PurgeCdn',
        purgeCdn_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.NetworkingPurgeCdnRequest.fromBuffer(value),
        ($0.NetworkingPurgeCdnResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.NetworkingDiagnoseDomainsRequest,
            $0.NetworkingDiagnoseDomainsResponse>(
        'DiagnoseDomains',
        diagnoseDomains_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.NetworkingDiagnoseDomainsRequest.fromBuffer(value),
        ($0.NetworkingDiagnoseDomainsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.NetworkingDiagnoseRootNetworkRequest,
            $0.NetworkingDiagnoseRootNetworkResponse>(
        'DiagnoseRootNetwork',
        diagnoseRootNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.NetworkingDiagnoseRootNetworkRequest.fromBuffer(value),
        ($0.NetworkingDiagnoseRootNetworkResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.NetworkingRepairRootNetworkRequest,
            $0.NetworkingRepairRootNetworkResponse>(
        'RepairRootNetwork',
        repairRootNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.NetworkingRepairRootNetworkRequest.fromBuffer(value),
        ($0.NetworkingRepairRootNetworkResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.NetworkingPurgeCdnResponse> purgeCdn_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.NetworkingPurgeCdnRequest> $request) async {
    return purgeCdn($call, await $request);
  }

  $async.Future<$0.NetworkingPurgeCdnResponse> purgeCdn(
      $grpc.ServiceCall call, $0.NetworkingPurgeCdnRequest request);

  $async.Future<$0.NetworkingDiagnoseDomainsResponse> diagnoseDomains_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.NetworkingDiagnoseDomainsRequest> $request) async {
    return diagnoseDomains($call, await $request);
  }

  $async.Future<$0.NetworkingDiagnoseDomainsResponse> diagnoseDomains(
      $grpc.ServiceCall call, $0.NetworkingDiagnoseDomainsRequest request);

  $async.Future<$0.NetworkingDiagnoseRootNetworkResponse>
      diagnoseRootNetwork_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.NetworkingDiagnoseRootNetworkRequest>
              $request) async {
    return diagnoseRootNetwork($call, await $request);
  }

  $async.Future<$0.NetworkingDiagnoseRootNetworkResponse> diagnoseRootNetwork(
      $grpc.ServiceCall call, $0.NetworkingDiagnoseRootNetworkRequest request);

  $async.Future<$0.NetworkingRepairRootNetworkResponse> repairRootNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.NetworkingRepairRootNetworkRequest> $request) async {
    return repairRootNetwork($call, await $request);
  }

  $async.Future<$0.NetworkingRepairRootNetworkResponse> repairRootNetwork(
      $grpc.ServiceCall call, $0.NetworkingRepairRootNetworkRequest request);
}

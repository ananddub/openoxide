// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/overview.proto.

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

import 'overview.pb.dart' as $0;

export 'overview.pb.dart';

/// Sole overview owner service; absorbs the former OverviewOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.OverviewService')
class OverviewServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  OverviewServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.OverviewListServicesResponse> listServices(
    $0.OverviewListServicesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listServices, request, options: options);
  }

  $grpc.ResponseFuture<$0.OverviewListDomainsResponse> listDomains(
    $0.OverviewListDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDomains, request, options: options);
  }

  $grpc.ResponseFuture<$0.OverviewListBackupsResponse> listBackups(
    $0.OverviewListBackupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listBackups, request, options: options);
  }

  $grpc.ResponseFuture<$0.OverviewListOrganizationServicesResponse>
      listOrganizationServices(
    $0.OverviewListOrganizationServicesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listOrganizationServices, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.OverviewListOrganizationDomainsResponse>
      listOrganizationDomains(
    $0.OverviewListOrganizationDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listOrganizationDomains, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.OverviewListOrganizationBackupsResponse>
      listOrganizationBackups(
    $0.OverviewListOrganizationBackupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listOrganizationBackups, request,
        options: options);
  }

  // method descriptors

  static final _$listServices = $grpc.ClientMethod<
          $0.OverviewListServicesRequest, $0.OverviewListServicesResponse>(
      '/openoxide.api.v1.OverviewService/ListServices',
      ($0.OverviewListServicesRequest value) => value.writeToBuffer(),
      $0.OverviewListServicesResponse.fromBuffer);
  static final _$listDomains = $grpc.ClientMethod<$0.OverviewListDomainsRequest,
          $0.OverviewListDomainsResponse>(
      '/openoxide.api.v1.OverviewService/ListDomains',
      ($0.OverviewListDomainsRequest value) => value.writeToBuffer(),
      $0.OverviewListDomainsResponse.fromBuffer);
  static final _$listBackups = $grpc.ClientMethod<$0.OverviewListBackupsRequest,
          $0.OverviewListBackupsResponse>(
      '/openoxide.api.v1.OverviewService/ListBackups',
      ($0.OverviewListBackupsRequest value) => value.writeToBuffer(),
      $0.OverviewListBackupsResponse.fromBuffer);
  static final _$listOrganizationServices = $grpc.ClientMethod<
          $0.OverviewListOrganizationServicesRequest,
          $0.OverviewListOrganizationServicesResponse>(
      '/openoxide.api.v1.OverviewService/ListOrganizationServices',
      ($0.OverviewListOrganizationServicesRequest value) =>
          value.writeToBuffer(),
      $0.OverviewListOrganizationServicesResponse.fromBuffer);
  static final _$listOrganizationDomains = $grpc.ClientMethod<
          $0.OverviewListOrganizationDomainsRequest,
          $0.OverviewListOrganizationDomainsResponse>(
      '/openoxide.api.v1.OverviewService/ListOrganizationDomains',
      ($0.OverviewListOrganizationDomainsRequest value) =>
          value.writeToBuffer(),
      $0.OverviewListOrganizationDomainsResponse.fromBuffer);
  static final _$listOrganizationBackups = $grpc.ClientMethod<
          $0.OverviewListOrganizationBackupsRequest,
          $0.OverviewListOrganizationBackupsResponse>(
      '/openoxide.api.v1.OverviewService/ListOrganizationBackups',
      ($0.OverviewListOrganizationBackupsRequest value) =>
          value.writeToBuffer(),
      $0.OverviewListOrganizationBackupsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.OverviewService')
abstract class OverviewServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.OverviewService';

  OverviewServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.OverviewListServicesRequest,
            $0.OverviewListServicesResponse>(
        'ListServices',
        listServices_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListServicesRequest.fromBuffer(value),
        ($0.OverviewListServicesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.OverviewListDomainsRequest,
            $0.OverviewListDomainsResponse>(
        'ListDomains',
        listDomains_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListDomainsRequest.fromBuffer(value),
        ($0.OverviewListDomainsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.OverviewListBackupsRequest,
            $0.OverviewListBackupsResponse>(
        'ListBackups',
        listBackups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListBackupsRequest.fromBuffer(value),
        ($0.OverviewListBackupsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.OverviewListOrganizationServicesRequest,
            $0.OverviewListOrganizationServicesResponse>(
        'ListOrganizationServices',
        listOrganizationServices_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListOrganizationServicesRequest.fromBuffer(value),
        ($0.OverviewListOrganizationServicesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.OverviewListOrganizationDomainsRequest,
            $0.OverviewListOrganizationDomainsResponse>(
        'ListOrganizationDomains',
        listOrganizationDomains_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListOrganizationDomainsRequest.fromBuffer(value),
        ($0.OverviewListOrganizationDomainsResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.OverviewListOrganizationBackupsRequest,
            $0.OverviewListOrganizationBackupsResponse>(
        'ListOrganizationBackups',
        listOrganizationBackups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.OverviewListOrganizationBackupsRequest.fromBuffer(value),
        ($0.OverviewListOrganizationBackupsResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.OverviewListServicesResponse> listServices_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.OverviewListServicesRequest> $request) async {
    return listServices($call, await $request);
  }

  $async.Future<$0.OverviewListServicesResponse> listServices(
      $grpc.ServiceCall call, $0.OverviewListServicesRequest request);

  $async.Future<$0.OverviewListDomainsResponse> listDomains_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.OverviewListDomainsRequest> $request) async {
    return listDomains($call, await $request);
  }

  $async.Future<$0.OverviewListDomainsResponse> listDomains(
      $grpc.ServiceCall call, $0.OverviewListDomainsRequest request);

  $async.Future<$0.OverviewListBackupsResponse> listBackups_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.OverviewListBackupsRequest> $request) async {
    return listBackups($call, await $request);
  }

  $async.Future<$0.OverviewListBackupsResponse> listBackups(
      $grpc.ServiceCall call, $0.OverviewListBackupsRequest request);

  $async.Future<$0.OverviewListOrganizationServicesResponse>
      listOrganizationServices_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.OverviewListOrganizationServicesRequest>
              $request) async {
    return listOrganizationServices($call, await $request);
  }

  $async.Future<$0.OverviewListOrganizationServicesResponse>
      listOrganizationServices($grpc.ServiceCall call,
          $0.OverviewListOrganizationServicesRequest request);

  $async.Future<$0.OverviewListOrganizationDomainsResponse>
      listOrganizationDomains_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.OverviewListOrganizationDomainsRequest>
              $request) async {
    return listOrganizationDomains($call, await $request);
  }

  $async.Future<$0.OverviewListOrganizationDomainsResponse>
      listOrganizationDomains($grpc.ServiceCall call,
          $0.OverviewListOrganizationDomainsRequest request);

  $async.Future<$0.OverviewListOrganizationBackupsResponse>
      listOrganizationBackups_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.OverviewListOrganizationBackupsRequest>
              $request) async {
    return listOrganizationBackups($call, await $request);
  }

  $async.Future<$0.OverviewListOrganizationBackupsResponse>
      listOrganizationBackups($grpc.ServiceCall call,
          $0.OverviewListOrganizationBackupsRequest request);
}

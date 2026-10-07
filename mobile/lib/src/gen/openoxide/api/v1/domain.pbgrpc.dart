// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/domain.proto.

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
import 'domain.pb.dart' as $0;

export 'domain.pb.dart';

/// Sole domain owner service; absorbs the former DomainOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.DomainService')
class DomainServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DomainServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListDomainsResponse> listDomains(
    $0.ListDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDomains, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetDomainResponse> getDomain(
    $0.GetDomainRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDomain, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListApplicationDomainsResponse>
      listApplicationDomains(
    $0.ListApplicationDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listApplicationDomains, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ListComposeDomainsResponse> listComposeDomains(
    $0.ListComposeDomainsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listComposeDomains, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateDomainResponse> createDomain(
    $0.CreateDomainRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDomain, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateDomainResponse> updateDomain(
    $0.UpdateDomainRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateDomain, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteDomain(
    $0.DeleteDomainRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteDomain, request, options: options);
  }

  $grpc.ResponseFuture<$0.ValidateDomainDnsResponse> validateDomainDns(
    $0.ValidateDomainDnsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$validateDomainDns, request, options: options);
  }

  // method descriptors

  static final _$listDomains =
      $grpc.ClientMethod<$0.ListDomainsRequest, $0.ListDomainsResponse>(
          '/openoxide.api.v1.DomainService/ListDomains',
          ($0.ListDomainsRequest value) => value.writeToBuffer(),
          $0.ListDomainsResponse.fromBuffer);
  static final _$getDomain =
      $grpc.ClientMethod<$0.GetDomainRequest, $0.GetDomainResponse>(
          '/openoxide.api.v1.DomainService/GetDomain',
          ($0.GetDomainRequest value) => value.writeToBuffer(),
          $0.GetDomainResponse.fromBuffer);
  static final _$listApplicationDomains = $grpc.ClientMethod<
          $0.ListApplicationDomainsRequest, $0.ListApplicationDomainsResponse>(
      '/openoxide.api.v1.DomainService/ListApplicationDomains',
      ($0.ListApplicationDomainsRequest value) => value.writeToBuffer(),
      $0.ListApplicationDomainsResponse.fromBuffer);
  static final _$listComposeDomains = $grpc.ClientMethod<
          $0.ListComposeDomainsRequest, $0.ListComposeDomainsResponse>(
      '/openoxide.api.v1.DomainService/ListComposeDomains',
      ($0.ListComposeDomainsRequest value) => value.writeToBuffer(),
      $0.ListComposeDomainsResponse.fromBuffer);
  static final _$createDomain =
      $grpc.ClientMethod<$0.CreateDomainRequest, $0.CreateDomainResponse>(
          '/openoxide.api.v1.DomainService/CreateDomain',
          ($0.CreateDomainRequest value) => value.writeToBuffer(),
          $0.CreateDomainResponse.fromBuffer);
  static final _$updateDomain =
      $grpc.ClientMethod<$0.UpdateDomainRequest, $0.UpdateDomainResponse>(
          '/openoxide.api.v1.DomainService/UpdateDomain',
          ($0.UpdateDomainRequest value) => value.writeToBuffer(),
          $0.UpdateDomainResponse.fromBuffer);
  static final _$deleteDomain =
      $grpc.ClientMethod<$0.DeleteDomainRequest, $1.Empty>(
          '/openoxide.api.v1.DomainService/DeleteDomain',
          ($0.DeleteDomainRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$validateDomainDns = $grpc.ClientMethod<
          $0.ValidateDomainDnsRequest, $0.ValidateDomainDnsResponse>(
      '/openoxide.api.v1.DomainService/ValidateDomainDns',
      ($0.ValidateDomainDnsRequest value) => value.writeToBuffer(),
      $0.ValidateDomainDnsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.DomainService')
abstract class DomainServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.DomainService';

  DomainServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.ListDomainsRequest, $0.ListDomainsResponse>(
            'ListDomains',
            listDomains_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListDomainsRequest.fromBuffer(value),
            ($0.ListDomainsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDomainRequest, $0.GetDomainResponse>(
        'GetDomain',
        getDomain_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetDomainRequest.fromBuffer(value),
        ($0.GetDomainResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListApplicationDomainsRequest,
            $0.ListApplicationDomainsResponse>(
        'ListApplicationDomains',
        listApplicationDomains_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListApplicationDomainsRequest.fromBuffer(value),
        ($0.ListApplicationDomainsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListComposeDomainsRequest,
            $0.ListComposeDomainsResponse>(
        'ListComposeDomains',
        listComposeDomains_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListComposeDomainsRequest.fromBuffer(value),
        ($0.ListComposeDomainsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreateDomainRequest, $0.CreateDomainResponse>(
            'CreateDomain',
            createDomain_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateDomainRequest.fromBuffer(value),
            ($0.CreateDomainResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateDomainRequest, $0.UpdateDomainResponse>(
            'UpdateDomain',
            updateDomain_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateDomainRequest.fromBuffer(value),
            ($0.UpdateDomainResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteDomainRequest, $1.Empty>(
        'DeleteDomain',
        deleteDomain_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteDomainRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ValidateDomainDnsRequest,
            $0.ValidateDomainDnsResponse>(
        'ValidateDomainDns',
        validateDomainDns_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ValidateDomainDnsRequest.fromBuffer(value),
        ($0.ValidateDomainDnsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListDomainsResponse> listDomains_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ListDomainsRequest> $request) async {
    return listDomains($call, await $request);
  }

  $async.Future<$0.ListDomainsResponse> listDomains(
      $grpc.ServiceCall call, $0.ListDomainsRequest request);

  $async.Future<$0.GetDomainResponse> getDomain_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetDomainRequest> $request) async {
    return getDomain($call, await $request);
  }

  $async.Future<$0.GetDomainResponse> getDomain(
      $grpc.ServiceCall call, $0.GetDomainRequest request);

  $async.Future<$0.ListApplicationDomainsResponse> listApplicationDomains_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListApplicationDomainsRequest> $request) async {
    return listApplicationDomains($call, await $request);
  }

  $async.Future<$0.ListApplicationDomainsResponse> listApplicationDomains(
      $grpc.ServiceCall call, $0.ListApplicationDomainsRequest request);

  $async.Future<$0.ListComposeDomainsResponse> listComposeDomains_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListComposeDomainsRequest> $request) async {
    return listComposeDomains($call, await $request);
  }

  $async.Future<$0.ListComposeDomainsResponse> listComposeDomains(
      $grpc.ServiceCall call, $0.ListComposeDomainsRequest request);

  $async.Future<$0.CreateDomainResponse> createDomain_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateDomainRequest> $request) async {
    return createDomain($call, await $request);
  }

  $async.Future<$0.CreateDomainResponse> createDomain(
      $grpc.ServiceCall call, $0.CreateDomainRequest request);

  $async.Future<$0.UpdateDomainResponse> updateDomain_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateDomainRequest> $request) async {
    return updateDomain($call, await $request);
  }

  $async.Future<$0.UpdateDomainResponse> updateDomain(
      $grpc.ServiceCall call, $0.UpdateDomainRequest request);

  $async.Future<$1.Empty> deleteDomain_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteDomainRequest> $request) async {
    return deleteDomain($call, await $request);
  }

  $async.Future<$1.Empty> deleteDomain(
      $grpc.ServiceCall call, $0.DeleteDomainRequest request);

  $async.Future<$0.ValidateDomainDnsResponse> validateDomainDns_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ValidateDomainDnsRequest> $request) async {
    return validateDomainDns($call, await $request);
  }

  $async.Future<$0.ValidateDomainDnsResponse> validateDomainDns(
      $grpc.ServiceCall call, $0.ValidateDomainDnsRequest request);
}

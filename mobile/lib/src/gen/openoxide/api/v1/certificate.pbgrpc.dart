// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/certificate.proto.

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

import 'certificate.pb.dart' as $0;
import 'common.pb.dart' as $1;

export 'certificate.pb.dart';

/// Sole certificate owner service; replaces CertificateOperationsService.
@$pb.GrpcServiceName('openoxide.api.v1.CertificateService')
class CertificateServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  CertificateServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListCertificatesResponse> listCertificates(
    $0.ListCertificatesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listCertificates, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetCertificateResponse> getCertificate(
    $0.GetCertificateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getCertificate, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateCertificateResponse> createCertificate(
    $0.CreateCertificateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createCertificate, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateCertificateResponse> updateCertificate(
    $0.UpdateCertificateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateCertificate, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteCertificate(
    $0.DeleteCertificateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteCertificate, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListCertificateDependenciesResponse> listDependencies(
    $0.ListCertificateDependenciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDependencies, request, options: options);
  }

  $grpc.ResponseFuture<$0.RenewCertificateResponse> renewCertificate(
    $0.RenewCertificateRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$renewCertificate, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListCertificateRenewalsResponse> listRenewals(
    $0.ListCertificateRenewalsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listRenewals, request, options: options);
  }

  // method descriptors

  static final _$listCertificates = $grpc.ClientMethod<
          $0.ListCertificatesRequest, $0.ListCertificatesResponse>(
      '/openoxide.api.v1.CertificateService/ListCertificates',
      ($0.ListCertificatesRequest value) => value.writeToBuffer(),
      $0.ListCertificatesResponse.fromBuffer);
  static final _$getCertificate =
      $grpc.ClientMethod<$0.GetCertificateRequest, $0.GetCertificateResponse>(
          '/openoxide.api.v1.CertificateService/GetCertificate',
          ($0.GetCertificateRequest value) => value.writeToBuffer(),
          $0.GetCertificateResponse.fromBuffer);
  static final _$createCertificate = $grpc.ClientMethod<
          $0.CreateCertificateRequest, $0.CreateCertificateResponse>(
      '/openoxide.api.v1.CertificateService/CreateCertificate',
      ($0.CreateCertificateRequest value) => value.writeToBuffer(),
      $0.CreateCertificateResponse.fromBuffer);
  static final _$updateCertificate = $grpc.ClientMethod<
          $0.UpdateCertificateRequest, $0.UpdateCertificateResponse>(
      '/openoxide.api.v1.CertificateService/UpdateCertificate',
      ($0.UpdateCertificateRequest value) => value.writeToBuffer(),
      $0.UpdateCertificateResponse.fromBuffer);
  static final _$deleteCertificate =
      $grpc.ClientMethod<$0.DeleteCertificateRequest, $1.Empty>(
          '/openoxide.api.v1.CertificateService/DeleteCertificate',
          ($0.DeleteCertificateRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listDependencies = $grpc.ClientMethod<
          $0.ListCertificateDependenciesRequest,
          $0.ListCertificateDependenciesResponse>(
      '/openoxide.api.v1.CertificateService/ListDependencies',
      ($0.ListCertificateDependenciesRequest value) => value.writeToBuffer(),
      $0.ListCertificateDependenciesResponse.fromBuffer);
  static final _$renewCertificate = $grpc.ClientMethod<
          $0.RenewCertificateRequest, $0.RenewCertificateResponse>(
      '/openoxide.api.v1.CertificateService/RenewCertificate',
      ($0.RenewCertificateRequest value) => value.writeToBuffer(),
      $0.RenewCertificateResponse.fromBuffer);
  static final _$listRenewals = $grpc.ClientMethod<
          $0.ListCertificateRenewalsRequest,
          $0.ListCertificateRenewalsResponse>(
      '/openoxide.api.v1.CertificateService/ListRenewals',
      ($0.ListCertificateRenewalsRequest value) => value.writeToBuffer(),
      $0.ListCertificateRenewalsResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.CertificateService')
abstract class CertificateServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.CertificateService';

  CertificateServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListCertificatesRequest,
            $0.ListCertificatesResponse>(
        'ListCertificates',
        listCertificates_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListCertificatesRequest.fromBuffer(value),
        ($0.ListCertificatesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetCertificateRequest,
            $0.GetCertificateResponse>(
        'GetCertificate',
        getCertificate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetCertificateRequest.fromBuffer(value),
        ($0.GetCertificateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateCertificateRequest,
            $0.CreateCertificateResponse>(
        'CreateCertificate',
        createCertificate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateCertificateRequest.fromBuffer(value),
        ($0.CreateCertificateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateCertificateRequest,
            $0.UpdateCertificateResponse>(
        'UpdateCertificate',
        updateCertificate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateCertificateRequest.fromBuffer(value),
        ($0.UpdateCertificateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteCertificateRequest, $1.Empty>(
        'DeleteCertificate',
        deleteCertificate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteCertificateRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListCertificateDependenciesRequest,
            $0.ListCertificateDependenciesResponse>(
        'ListDependencies',
        listDependencies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListCertificateDependenciesRequest.fromBuffer(value),
        ($0.ListCertificateDependenciesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RenewCertificateRequest,
            $0.RenewCertificateResponse>(
        'RenewCertificate',
        renewCertificate_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RenewCertificateRequest.fromBuffer(value),
        ($0.RenewCertificateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListCertificateRenewalsRequest,
            $0.ListCertificateRenewalsResponse>(
        'ListRenewals',
        listRenewals_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListCertificateRenewalsRequest.fromBuffer(value),
        ($0.ListCertificateRenewalsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListCertificatesResponse> listCertificates_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListCertificatesRequest> $request) async {
    return listCertificates($call, await $request);
  }

  $async.Future<$0.ListCertificatesResponse> listCertificates(
      $grpc.ServiceCall call, $0.ListCertificatesRequest request);

  $async.Future<$0.GetCertificateResponse> getCertificate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetCertificateRequest> $request) async {
    return getCertificate($call, await $request);
  }

  $async.Future<$0.GetCertificateResponse> getCertificate(
      $grpc.ServiceCall call, $0.GetCertificateRequest request);

  $async.Future<$0.CreateCertificateResponse> createCertificate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateCertificateRequest> $request) async {
    return createCertificate($call, await $request);
  }

  $async.Future<$0.CreateCertificateResponse> createCertificate(
      $grpc.ServiceCall call, $0.CreateCertificateRequest request);

  $async.Future<$0.UpdateCertificateResponse> updateCertificate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateCertificateRequest> $request) async {
    return updateCertificate($call, await $request);
  }

  $async.Future<$0.UpdateCertificateResponse> updateCertificate(
      $grpc.ServiceCall call, $0.UpdateCertificateRequest request);

  $async.Future<$1.Empty> deleteCertificate_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteCertificateRequest> $request) async {
    return deleteCertificate($call, await $request);
  }

  $async.Future<$1.Empty> deleteCertificate(
      $grpc.ServiceCall call, $0.DeleteCertificateRequest request);

  $async.Future<$0.ListCertificateDependenciesResponse> listDependencies_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListCertificateDependenciesRequest> $request) async {
    return listDependencies($call, await $request);
  }

  $async.Future<$0.ListCertificateDependenciesResponse> listDependencies(
      $grpc.ServiceCall call, $0.ListCertificateDependenciesRequest request);

  $async.Future<$0.RenewCertificateResponse> renewCertificate_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RenewCertificateRequest> $request) async {
    return renewCertificate($call, await $request);
  }

  $async.Future<$0.RenewCertificateResponse> renewCertificate(
      $grpc.ServiceCall call, $0.RenewCertificateRequest request);

  $async.Future<$0.ListCertificateRenewalsResponse> listRenewals_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListCertificateRenewalsRequest> $request) async {
    return listRenewals($call, await $request);
  }

  $async.Future<$0.ListCertificateRenewalsResponse> listRenewals(
      $grpc.ServiceCall call, $0.ListCertificateRenewalsRequest request);
}

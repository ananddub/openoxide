// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/organization.proto.

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
import 'organization.pb.dart' as $0;

export 'organization.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.OrganizationService')
class OrganizationServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  OrganizationServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Organization> getOrganization(
    $0.GetOrganizationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getOrganization, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListOrganizationsResponse> listOrganizations(
    $0.ListOrganizationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listOrganizations, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> leaveOrganization(
    $0.LeaveOrganizationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$leaveOrganization, request, options: options);
  }

  $grpc.ResponseFuture<$0.Organization> createOrganization(
    $0.CreateOrganizationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createOrganization, request, options: options);
  }

  $grpc.ResponseFuture<$0.Organization> updateOrganization(
    $0.UpdateOrganizationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateOrganization, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteOrganization(
    $0.DeleteOrganizationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteOrganization, request, options: options);
  }

  // method descriptors

  static final _$getOrganization =
      $grpc.ClientMethod<$0.GetOrganizationRequest, $0.Organization>(
          '/openoxide.api.v1.OrganizationService/GetOrganization',
          ($0.GetOrganizationRequest value) => value.writeToBuffer(),
          $0.Organization.fromBuffer);
  static final _$listOrganizations = $grpc.ClientMethod<
          $0.ListOrganizationsRequest, $0.ListOrganizationsResponse>(
      '/openoxide.api.v1.OrganizationService/ListOrganizations',
      ($0.ListOrganizationsRequest value) => value.writeToBuffer(),
      $0.ListOrganizationsResponse.fromBuffer);
  static final _$leaveOrganization =
      $grpc.ClientMethod<$0.LeaveOrganizationRequest, $1.Empty>(
          '/openoxide.api.v1.OrganizationService/LeaveOrganization',
          ($0.LeaveOrganizationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$createOrganization =
      $grpc.ClientMethod<$0.CreateOrganizationRequest, $0.Organization>(
          '/openoxide.api.v1.OrganizationService/CreateOrganization',
          ($0.CreateOrganizationRequest value) => value.writeToBuffer(),
          $0.Organization.fromBuffer);
  static final _$updateOrganization =
      $grpc.ClientMethod<$0.UpdateOrganizationRequest, $0.Organization>(
          '/openoxide.api.v1.OrganizationService/UpdateOrganization',
          ($0.UpdateOrganizationRequest value) => value.writeToBuffer(),
          $0.Organization.fromBuffer);
  static final _$deleteOrganization =
      $grpc.ClientMethod<$0.DeleteOrganizationRequest, $1.Empty>(
          '/openoxide.api.v1.OrganizationService/DeleteOrganization',
          ($0.DeleteOrganizationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.OrganizationService')
abstract class OrganizationServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.OrganizationService';

  OrganizationServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetOrganizationRequest, $0.Organization>(
        'GetOrganization',
        getOrganization_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetOrganizationRequest.fromBuffer(value),
        ($0.Organization value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListOrganizationsRequest,
            $0.ListOrganizationsResponse>(
        'ListOrganizations',
        listOrganizations_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListOrganizationsRequest.fromBuffer(value),
        ($0.ListOrganizationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LeaveOrganizationRequest, $1.Empty>(
        'LeaveOrganization',
        leaveOrganization_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.LeaveOrganizationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreateOrganizationRequest, $0.Organization>(
            'CreateOrganization',
            createOrganization_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateOrganizationRequest.fromBuffer(value),
            ($0.Organization value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.UpdateOrganizationRequest, $0.Organization>(
            'UpdateOrganization',
            updateOrganization_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.UpdateOrganizationRequest.fromBuffer(value),
            ($0.Organization value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteOrganizationRequest, $1.Empty>(
        'DeleteOrganization',
        deleteOrganization_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteOrganizationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.Organization> getOrganization_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetOrganizationRequest> $request) async {
    return getOrganization($call, await $request);
  }

  $async.Future<$0.Organization> getOrganization(
      $grpc.ServiceCall call, $0.GetOrganizationRequest request);

  $async.Future<$0.ListOrganizationsResponse> listOrganizations_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListOrganizationsRequest> $request) async {
    return listOrganizations($call, await $request);
  }

  $async.Future<$0.ListOrganizationsResponse> listOrganizations(
      $grpc.ServiceCall call, $0.ListOrganizationsRequest request);

  $async.Future<$1.Empty> leaveOrganization_Pre($grpc.ServiceCall $call,
      $async.Future<$0.LeaveOrganizationRequest> $request) async {
    return leaveOrganization($call, await $request);
  }

  $async.Future<$1.Empty> leaveOrganization(
      $grpc.ServiceCall call, $0.LeaveOrganizationRequest request);

  $async.Future<$0.Organization> createOrganization_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateOrganizationRequest> $request) async {
    return createOrganization($call, await $request);
  }

  $async.Future<$0.Organization> createOrganization(
      $grpc.ServiceCall call, $0.CreateOrganizationRequest request);

  $async.Future<$0.Organization> updateOrganization_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateOrganizationRequest> $request) async {
    return updateOrganization($call, await $request);
  }

  $async.Future<$0.Organization> updateOrganization(
      $grpc.ServiceCall call, $0.UpdateOrganizationRequest request);

  $async.Future<$1.Empty> deleteOrganization_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteOrganizationRequest> $request) async {
    return deleteOrganization($call, await $request);
  }

  $async.Future<$1.Empty> deleteOrganization(
      $grpc.ServiceCall call, $0.DeleteOrganizationRequest request);
}

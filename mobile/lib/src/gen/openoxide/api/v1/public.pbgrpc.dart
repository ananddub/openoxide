// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/public.proto.

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

import 'public.pb.dart' as $0;

export 'public.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.PublicService')
class PublicServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PublicServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListGitBranchesResponse> listGitBranches(
    $0.ListGitBranchesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listGitBranches, request, options: options);
  }

  $grpc.ResponseFuture<$0.GenerateDomainResponse> generateDomain(
    $0.GenerateDomainRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$generateDomain, request, options: options);
  }

  // method descriptors

  static final _$listGitBranches =
      $grpc.ClientMethod<$0.ListGitBranchesRequest, $0.ListGitBranchesResponse>(
          '/openoxide.api.v1.PublicService/ListGitBranches',
          ($0.ListGitBranchesRequest value) => value.writeToBuffer(),
          $0.ListGitBranchesResponse.fromBuffer);
  static final _$generateDomain =
      $grpc.ClientMethod<$0.GenerateDomainRequest, $0.GenerateDomainResponse>(
          '/openoxide.api.v1.PublicService/GenerateDomain',
          ($0.GenerateDomainRequest value) => value.writeToBuffer(),
          $0.GenerateDomainResponse.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.PublicService')
abstract class PublicServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.PublicService';

  PublicServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListGitBranchesRequest,
            $0.ListGitBranchesResponse>(
        'ListGitBranches',
        listGitBranches_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListGitBranchesRequest.fromBuffer(value),
        ($0.ListGitBranchesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GenerateDomainRequest,
            $0.GenerateDomainResponse>(
        'GenerateDomain',
        generateDomain_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GenerateDomainRequest.fromBuffer(value),
        ($0.GenerateDomainResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListGitBranchesResponse> listGitBranches_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListGitBranchesRequest> $request) async {
    return listGitBranches($call, await $request);
  }

  $async.Future<$0.ListGitBranchesResponse> listGitBranches(
      $grpc.ServiceCall call, $0.ListGitBranchesRequest request);

  $async.Future<$0.GenerateDomainResponse> generateDomain_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GenerateDomainRequest> $request) async {
    return generateDomain($call, await $request);
  }

  $async.Future<$0.GenerateDomainResponse> generateDomain(
      $grpc.ServiceCall call, $0.GenerateDomainRequest request);
}

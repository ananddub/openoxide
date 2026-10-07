// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/swarm.proto.

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
import 'swarm.pb.dart' as $0;

export 'swarm.pb.dart';

/// Sole swarm owner service; absorbs the former SwarmOperationsService; `server_id` selects the target engine, absent means the panel's local Docker daemon.
@$pb.GrpcServiceName('openoxide.api.v1.SwarmService')
class SwarmServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SwarmServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.SwarmGetSettingsResponse> getSettings(
    $0.SwarmGetSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmUpdateSettingsResponse> updateSettings(
    $0.SwarmUpdateSettingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmListNetworksResponse> listNetworks(
    $0.SwarmListNetworksRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNetworks, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmCreateNetworkResponse> createNetwork(
    $0.SwarmCreateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmUpdateNetworkResponse> updateNetwork(
    $0.SwarmUpdateNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteNetwork(
    $0.SwarmDeleteNetworkRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteNetwork, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmListServicesResponse> listServices(
    $0.SwarmListServicesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listServices, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmGetServiceResponse> getService(
    $0.SwarmGetServiceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getService, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmUpdateServiceResponse> updateService(
    $0.SwarmUpdateServiceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateService, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmInfoResponse> info(
    $0.SwarmInfoRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$info, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmGetJoinTokensResponse> getJoinTokens(
    $0.SwarmGetJoinTokensRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getJoinTokens, request, options: options);
  }

  $grpc.ResponseFuture<$0.SwarmListNodesResponse> listNodes(
    $0.SwarmListNodesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNodes, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> promoteNode(
    $0.SwarmPromoteNodeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$promoteNode, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> demoteNode(
    $0.SwarmDemoteNodeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$demoteNode, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> updateNodeAvailability(
    $0.SwarmUpdateNodeAvailabilityRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNodeAvailability, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeNode(
    $0.SwarmRemoveNodeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeNode, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> leave(
    $0.SwarmLeaveRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$leave, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> join(
    $0.SwarmJoinRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$join, request, options: options);
  }

  // method descriptors

  static final _$getSettings = $grpc.ClientMethod<$0.SwarmGetSettingsRequest,
          $0.SwarmGetSettingsResponse>(
      '/openoxide.api.v1.SwarmService/GetSettings',
      ($0.SwarmGetSettingsRequest value) => value.writeToBuffer(),
      $0.SwarmGetSettingsResponse.fromBuffer);
  static final _$updateSettings = $grpc.ClientMethod<
          $0.SwarmUpdateSettingsRequest, $0.SwarmUpdateSettingsResponse>(
      '/openoxide.api.v1.SwarmService/UpdateSettings',
      ($0.SwarmUpdateSettingsRequest value) => value.writeToBuffer(),
      $0.SwarmUpdateSettingsResponse.fromBuffer);
  static final _$listNetworks = $grpc.ClientMethod<$0.SwarmListNetworksRequest,
          $0.SwarmListNetworksResponse>(
      '/openoxide.api.v1.SwarmService/ListNetworks',
      ($0.SwarmListNetworksRequest value) => value.writeToBuffer(),
      $0.SwarmListNetworksResponse.fromBuffer);
  static final _$createNetwork = $grpc.ClientMethod<
          $0.SwarmCreateNetworkRequest, $0.SwarmCreateNetworkResponse>(
      '/openoxide.api.v1.SwarmService/CreateNetwork',
      ($0.SwarmCreateNetworkRequest value) => value.writeToBuffer(),
      $0.SwarmCreateNetworkResponse.fromBuffer);
  static final _$updateNetwork = $grpc.ClientMethod<
          $0.SwarmUpdateNetworkRequest, $0.SwarmUpdateNetworkResponse>(
      '/openoxide.api.v1.SwarmService/UpdateNetwork',
      ($0.SwarmUpdateNetworkRequest value) => value.writeToBuffer(),
      $0.SwarmUpdateNetworkResponse.fromBuffer);
  static final _$deleteNetwork =
      $grpc.ClientMethod<$0.SwarmDeleteNetworkRequest, $1.Empty>(
          '/openoxide.api.v1.SwarmService/DeleteNetwork',
          ($0.SwarmDeleteNetworkRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listServices = $grpc.ClientMethod<$0.SwarmListServicesRequest,
          $0.SwarmListServicesResponse>(
      '/openoxide.api.v1.SwarmService/ListServices',
      ($0.SwarmListServicesRequest value) => value.writeToBuffer(),
      $0.SwarmListServicesResponse.fromBuffer);
  static final _$getService =
      $grpc.ClientMethod<$0.SwarmGetServiceRequest, $0.SwarmGetServiceResponse>(
          '/openoxide.api.v1.SwarmService/GetService',
          ($0.SwarmGetServiceRequest value) => value.writeToBuffer(),
          $0.SwarmGetServiceResponse.fromBuffer);
  static final _$updateService = $grpc.ClientMethod<
          $0.SwarmUpdateServiceRequest, $0.SwarmUpdateServiceResponse>(
      '/openoxide.api.v1.SwarmService/UpdateService',
      ($0.SwarmUpdateServiceRequest value) => value.writeToBuffer(),
      $0.SwarmUpdateServiceResponse.fromBuffer);
  static final _$info =
      $grpc.ClientMethod<$0.SwarmInfoRequest, $0.SwarmInfoResponse>(
          '/openoxide.api.v1.SwarmService/Info',
          ($0.SwarmInfoRequest value) => value.writeToBuffer(),
          $0.SwarmInfoResponse.fromBuffer);
  static final _$getJoinTokens = $grpc.ClientMethod<
          $0.SwarmGetJoinTokensRequest, $0.SwarmGetJoinTokensResponse>(
      '/openoxide.api.v1.SwarmService/GetJoinTokens',
      ($0.SwarmGetJoinTokensRequest value) => value.writeToBuffer(),
      $0.SwarmGetJoinTokensResponse.fromBuffer);
  static final _$listNodes =
      $grpc.ClientMethod<$0.SwarmListNodesRequest, $0.SwarmListNodesResponse>(
          '/openoxide.api.v1.SwarmService/ListNodes',
          ($0.SwarmListNodesRequest value) => value.writeToBuffer(),
          $0.SwarmListNodesResponse.fromBuffer);
  static final _$promoteNode =
      $grpc.ClientMethod<$0.SwarmPromoteNodeRequest, $1.Empty>(
          '/openoxide.api.v1.SwarmService/PromoteNode',
          ($0.SwarmPromoteNodeRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$demoteNode =
      $grpc.ClientMethod<$0.SwarmDemoteNodeRequest, $1.Empty>(
          '/openoxide.api.v1.SwarmService/DemoteNode',
          ($0.SwarmDemoteNodeRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$updateNodeAvailability =
      $grpc.ClientMethod<$0.SwarmUpdateNodeAvailabilityRequest, $1.Empty>(
          '/openoxide.api.v1.SwarmService/UpdateNodeAvailability',
          ($0.SwarmUpdateNodeAvailabilityRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$removeNode =
      $grpc.ClientMethod<$0.SwarmRemoveNodeRequest, $1.Empty>(
          '/openoxide.api.v1.SwarmService/RemoveNode',
          ($0.SwarmRemoveNodeRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$leave = $grpc.ClientMethod<$0.SwarmLeaveRequest, $1.Empty>(
      '/openoxide.api.v1.SwarmService/Leave',
      ($0.SwarmLeaveRequest value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
  static final _$join = $grpc.ClientMethod<$0.SwarmJoinRequest, $1.Empty>(
      '/openoxide.api.v1.SwarmService/Join',
      ($0.SwarmJoinRequest value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.SwarmService')
abstract class SwarmServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.SwarmService';

  SwarmServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SwarmGetSettingsRequest,
            $0.SwarmGetSettingsResponse>(
        'GetSettings',
        getSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmGetSettingsRequest.fromBuffer(value),
        ($0.SwarmGetSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmUpdateSettingsRequest,
            $0.SwarmUpdateSettingsResponse>(
        'UpdateSettings',
        updateSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmUpdateSettingsRequest.fromBuffer(value),
        ($0.SwarmUpdateSettingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmListNetworksRequest,
            $0.SwarmListNetworksResponse>(
        'ListNetworks',
        listNetworks_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmListNetworksRequest.fromBuffer(value),
        ($0.SwarmListNetworksResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmCreateNetworkRequest,
            $0.SwarmCreateNetworkResponse>(
        'CreateNetwork',
        createNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmCreateNetworkRequest.fromBuffer(value),
        ($0.SwarmCreateNetworkResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmUpdateNetworkRequest,
            $0.SwarmUpdateNetworkResponse>(
        'UpdateNetwork',
        updateNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmUpdateNetworkRequest.fromBuffer(value),
        ($0.SwarmUpdateNetworkResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmDeleteNetworkRequest, $1.Empty>(
        'DeleteNetwork',
        deleteNetwork_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmDeleteNetworkRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmListServicesRequest,
            $0.SwarmListServicesResponse>(
        'ListServices',
        listServices_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmListServicesRequest.fromBuffer(value),
        ($0.SwarmListServicesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmGetServiceRequest,
            $0.SwarmGetServiceResponse>(
        'GetService',
        getService_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmGetServiceRequest.fromBuffer(value),
        ($0.SwarmGetServiceResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmUpdateServiceRequest,
            $0.SwarmUpdateServiceResponse>(
        'UpdateService',
        updateService_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmUpdateServiceRequest.fromBuffer(value),
        ($0.SwarmUpdateServiceResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmInfoRequest, $0.SwarmInfoResponse>(
        'Info',
        info_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SwarmInfoRequest.fromBuffer(value),
        ($0.SwarmInfoResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmGetJoinTokensRequest,
            $0.SwarmGetJoinTokensResponse>(
        'GetJoinTokens',
        getJoinTokens_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmGetJoinTokensRequest.fromBuffer(value),
        ($0.SwarmGetJoinTokensResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmListNodesRequest,
            $0.SwarmListNodesResponse>(
        'ListNodes',
        listNodes_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmListNodesRequest.fromBuffer(value),
        ($0.SwarmListNodesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmPromoteNodeRequest, $1.Empty>(
        'PromoteNode',
        promoteNode_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmPromoteNodeRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmDemoteNodeRequest, $1.Empty>(
        'DemoteNode',
        demoteNode_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmDemoteNodeRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SwarmUpdateNodeAvailabilityRequest, $1.Empty>(
            'UpdateNodeAvailability',
            updateNodeAvailability_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SwarmUpdateNodeAvailabilityRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmRemoveNodeRequest, $1.Empty>(
        'RemoveNode',
        removeNode_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SwarmRemoveNodeRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmLeaveRequest, $1.Empty>(
        'Leave',
        leave_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SwarmLeaveRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SwarmJoinRequest, $1.Empty>(
        'Join',
        join_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SwarmJoinRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.SwarmGetSettingsResponse> getSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmGetSettingsRequest> $request) async {
    return getSettings($call, await $request);
  }

  $async.Future<$0.SwarmGetSettingsResponse> getSettings(
      $grpc.ServiceCall call, $0.SwarmGetSettingsRequest request);

  $async.Future<$0.SwarmUpdateSettingsResponse> updateSettings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmUpdateSettingsRequest> $request) async {
    return updateSettings($call, await $request);
  }

  $async.Future<$0.SwarmUpdateSettingsResponse> updateSettings(
      $grpc.ServiceCall call, $0.SwarmUpdateSettingsRequest request);

  $async.Future<$0.SwarmListNetworksResponse> listNetworks_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmListNetworksRequest> $request) async {
    return listNetworks($call, await $request);
  }

  $async.Future<$0.SwarmListNetworksResponse> listNetworks(
      $grpc.ServiceCall call, $0.SwarmListNetworksRequest request);

  $async.Future<$0.SwarmCreateNetworkResponse> createNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmCreateNetworkRequest> $request) async {
    return createNetwork($call, await $request);
  }

  $async.Future<$0.SwarmCreateNetworkResponse> createNetwork(
      $grpc.ServiceCall call, $0.SwarmCreateNetworkRequest request);

  $async.Future<$0.SwarmUpdateNetworkResponse> updateNetwork_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmUpdateNetworkRequest> $request) async {
    return updateNetwork($call, await $request);
  }

  $async.Future<$0.SwarmUpdateNetworkResponse> updateNetwork(
      $grpc.ServiceCall call, $0.SwarmUpdateNetworkRequest request);

  $async.Future<$1.Empty> deleteNetwork_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmDeleteNetworkRequest> $request) async {
    return deleteNetwork($call, await $request);
  }

  $async.Future<$1.Empty> deleteNetwork(
      $grpc.ServiceCall call, $0.SwarmDeleteNetworkRequest request);

  $async.Future<$0.SwarmListServicesResponse> listServices_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmListServicesRequest> $request) async {
    return listServices($call, await $request);
  }

  $async.Future<$0.SwarmListServicesResponse> listServices(
      $grpc.ServiceCall call, $0.SwarmListServicesRequest request);

  $async.Future<$0.SwarmGetServiceResponse> getService_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmGetServiceRequest> $request) async {
    return getService($call, await $request);
  }

  $async.Future<$0.SwarmGetServiceResponse> getService(
      $grpc.ServiceCall call, $0.SwarmGetServiceRequest request);

  $async.Future<$0.SwarmUpdateServiceResponse> updateService_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmUpdateServiceRequest> $request) async {
    return updateService($call, await $request);
  }

  $async.Future<$0.SwarmUpdateServiceResponse> updateService(
      $grpc.ServiceCall call, $0.SwarmUpdateServiceRequest request);

  $async.Future<$0.SwarmInfoResponse> info_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmInfoRequest> $request) async {
    return info($call, await $request);
  }

  $async.Future<$0.SwarmInfoResponse> info(
      $grpc.ServiceCall call, $0.SwarmInfoRequest request);

  $async.Future<$0.SwarmGetJoinTokensResponse> getJoinTokens_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmGetJoinTokensRequest> $request) async {
    return getJoinTokens($call, await $request);
  }

  $async.Future<$0.SwarmGetJoinTokensResponse> getJoinTokens(
      $grpc.ServiceCall call, $0.SwarmGetJoinTokensRequest request);

  $async.Future<$0.SwarmListNodesResponse> listNodes_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SwarmListNodesRequest> $request) async {
    return listNodes($call, await $request);
  }

  $async.Future<$0.SwarmListNodesResponse> listNodes(
      $grpc.ServiceCall call, $0.SwarmListNodesRequest request);

  $async.Future<$1.Empty> promoteNode_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmPromoteNodeRequest> $request) async {
    return promoteNode($call, await $request);
  }

  $async.Future<$1.Empty> promoteNode(
      $grpc.ServiceCall call, $0.SwarmPromoteNodeRequest request);

  $async.Future<$1.Empty> demoteNode_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmDemoteNodeRequest> $request) async {
    return demoteNode($call, await $request);
  }

  $async.Future<$1.Empty> demoteNode(
      $grpc.ServiceCall call, $0.SwarmDemoteNodeRequest request);

  $async.Future<$1.Empty> updateNodeAvailability_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmUpdateNodeAvailabilityRequest> $request) async {
    return updateNodeAvailability($call, await $request);
  }

  $async.Future<$1.Empty> updateNodeAvailability(
      $grpc.ServiceCall call, $0.SwarmUpdateNodeAvailabilityRequest request);

  $async.Future<$1.Empty> removeNode_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmRemoveNodeRequest> $request) async {
    return removeNode($call, await $request);
  }

  $async.Future<$1.Empty> removeNode(
      $grpc.ServiceCall call, $0.SwarmRemoveNodeRequest request);

  $async.Future<$1.Empty> leave_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmLeaveRequest> $request) async {
    return leave($call, await $request);
  }

  $async.Future<$1.Empty> leave(
      $grpc.ServiceCall call, $0.SwarmLeaveRequest request);

  $async.Future<$1.Empty> join_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SwarmJoinRequest> $request) async {
    return join($call, await $request);
  }

  $async.Future<$1.Empty> join(
      $grpc.ServiceCall call, $0.SwarmJoinRequest request);
}

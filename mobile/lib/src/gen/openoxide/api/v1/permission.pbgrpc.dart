// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/permission.proto.

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
import 'permission.pb.dart' as $0;

export 'permission.pb.dart';

/// Group, policy, member and invite management for the active organization; `ListPermissions`/`GetPermission`/`CreatePermission`/`UpdatePermission`/`DeletePermission` are aliases of the corresponding `*Group` rpcs and share the `PermissionGroup` payload.
@$pb.GrpcServiceName('openoxide.api.v1.PermissionService')
class PermissionServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PermissionServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListPermissionsResponse> listPermissions(
    $0.ListPermissionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPermissions, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetPermissionResponse> getPermission(
    $0.GetPermissionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPermission, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreatePermissionResponse> createPermission(
    $0.CreatePermissionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPermission, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> updatePermission(
    $0.UpdatePermissionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePermission, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deletePermission(
    $0.DeletePermissionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePermission, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionListGroupsResponse> listGroups(
    $0.PermissionListGroupsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listGroups, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionGetGroupResponse> getGroup(
    $0.PermissionGetGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getGroup, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionCreateGroupResponse> createGroup(
    $0.PermissionCreateGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createGroup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> updateGroup(
    $0.PermissionUpdateGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateGroup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteGroup(
    $0.PermissionDeleteGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteGroup, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionListPoliciesResponse> listPolicies(
    $0.PermissionListPoliciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPolicies, request, options: options);
  }

  /// Effective actions for the calling user inside the active organization (auth-only: every member may inspect their own grants to gate the UI).
  $grpc.ResponseFuture<$0.GetMyPermissionsResponse> getMyPermissions(
    $0.GetMyPermissionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getMyPermissions, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> assignMemberGroup(
    $0.PermissionAssignMemberGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$assignMemberGroup, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> setGroupActive(
    $0.PermissionSetGroupActiveRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setGroupActive, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionListUserPoliciesResponse> listUserPolicies(
    $0.PermissionListUserPoliciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listUserPolicies, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionListMembersResponse> listMembers(
    $0.PermissionListMembersRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMembers, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> addMember(
    $0.PermissionAddMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addMember, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> updateMemberRole(
    $0.PermissionUpdateMemberRoleRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateMemberRole, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeMember(
    $0.PermissionRemoveMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeMember, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionListInvitesResponse> listInvites(
    $0.PermissionListInvitesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listInvites, request, options: options);
  }

  $grpc.ResponseFuture<$0.PermissionCreateInviteResponse> createInvite(
    $0.PermissionCreateInviteRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createInvite, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> acceptInvite(
    $0.PermissionAcceptInviteRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acceptInvite, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> cancelInvite(
    $0.PermissionCancelInviteRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelInvite, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> replaceUserPolicies(
    $0.PermissionReplaceUserPoliciesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$replaceUserPolicies, request, options: options);
  }

  // method descriptors

  static final _$listPermissions =
      $grpc.ClientMethod<$0.ListPermissionsRequest, $0.ListPermissionsResponse>(
          '/openoxide.api.v1.PermissionService/ListPermissions',
          ($0.ListPermissionsRequest value) => value.writeToBuffer(),
          $0.ListPermissionsResponse.fromBuffer);
  static final _$getPermission =
      $grpc.ClientMethod<$0.GetPermissionRequest, $0.GetPermissionResponse>(
          '/openoxide.api.v1.PermissionService/GetPermission',
          ($0.GetPermissionRequest value) => value.writeToBuffer(),
          $0.GetPermissionResponse.fromBuffer);
  static final _$createPermission = $grpc.ClientMethod<
          $0.CreatePermissionRequest, $0.CreatePermissionResponse>(
      '/openoxide.api.v1.PermissionService/CreatePermission',
      ($0.CreatePermissionRequest value) => value.writeToBuffer(),
      $0.CreatePermissionResponse.fromBuffer);
  static final _$updatePermission =
      $grpc.ClientMethod<$0.UpdatePermissionRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/UpdatePermission',
          ($0.UpdatePermissionRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deletePermission =
      $grpc.ClientMethod<$0.DeletePermissionRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/DeletePermission',
          ($0.DeletePermissionRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listGroups = $grpc.ClientMethod<$0.PermissionListGroupsRequest,
          $0.PermissionListGroupsResponse>(
      '/openoxide.api.v1.PermissionService/ListGroups',
      ($0.PermissionListGroupsRequest value) => value.writeToBuffer(),
      $0.PermissionListGroupsResponse.fromBuffer);
  static final _$getGroup = $grpc.ClientMethod<$0.PermissionGetGroupRequest,
          $0.PermissionGetGroupResponse>(
      '/openoxide.api.v1.PermissionService/GetGroup',
      ($0.PermissionGetGroupRequest value) => value.writeToBuffer(),
      $0.PermissionGetGroupResponse.fromBuffer);
  static final _$createGroup = $grpc.ClientMethod<
          $0.PermissionCreateGroupRequest, $0.PermissionCreateGroupResponse>(
      '/openoxide.api.v1.PermissionService/CreateGroup',
      ($0.PermissionCreateGroupRequest value) => value.writeToBuffer(),
      $0.PermissionCreateGroupResponse.fromBuffer);
  static final _$updateGroup =
      $grpc.ClientMethod<$0.PermissionUpdateGroupRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/UpdateGroup',
          ($0.PermissionUpdateGroupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$deleteGroup =
      $grpc.ClientMethod<$0.PermissionDeleteGroupRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/DeleteGroup',
          ($0.PermissionDeleteGroupRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listPolicies = $grpc.ClientMethod<
          $0.PermissionListPoliciesRequest, $0.PermissionListPoliciesResponse>(
      '/openoxide.api.v1.PermissionService/ListPolicies',
      ($0.PermissionListPoliciesRequest value) => value.writeToBuffer(),
      $0.PermissionListPoliciesResponse.fromBuffer);
  static final _$getMyPermissions = $grpc.ClientMethod<
          $0.GetMyPermissionsRequest, $0.GetMyPermissionsResponse>(
      '/openoxide.api.v1.PermissionService/GetMyPermissions',
      ($0.GetMyPermissionsRequest value) => value.writeToBuffer(),
      $0.GetMyPermissionsResponse.fromBuffer);
  static final _$assignMemberGroup =
      $grpc.ClientMethod<$0.PermissionAssignMemberGroupRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/AssignMemberGroup',
          ($0.PermissionAssignMemberGroupRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$setGroupActive =
      $grpc.ClientMethod<$0.PermissionSetGroupActiveRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/SetGroupActive',
          ($0.PermissionSetGroupActiveRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listUserPolicies = $grpc.ClientMethod<
          $0.PermissionListUserPoliciesRequest,
          $0.PermissionListUserPoliciesResponse>(
      '/openoxide.api.v1.PermissionService/ListUserPolicies',
      ($0.PermissionListUserPoliciesRequest value) => value.writeToBuffer(),
      $0.PermissionListUserPoliciesResponse.fromBuffer);
  static final _$listMembers = $grpc.ClientMethod<
          $0.PermissionListMembersRequest, $0.PermissionListMembersResponse>(
      '/openoxide.api.v1.PermissionService/ListMembers',
      ($0.PermissionListMembersRequest value) => value.writeToBuffer(),
      $0.PermissionListMembersResponse.fromBuffer);
  static final _$addMember =
      $grpc.ClientMethod<$0.PermissionAddMemberRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/AddMember',
          ($0.PermissionAddMemberRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$updateMemberRole =
      $grpc.ClientMethod<$0.PermissionUpdateMemberRoleRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/UpdateMemberRole',
          ($0.PermissionUpdateMemberRoleRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$removeMember =
      $grpc.ClientMethod<$0.PermissionRemoveMemberRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/RemoveMember',
          ($0.PermissionRemoveMemberRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listInvites = $grpc.ClientMethod<
          $0.PermissionListInvitesRequest, $0.PermissionListInvitesResponse>(
      '/openoxide.api.v1.PermissionService/ListInvites',
      ($0.PermissionListInvitesRequest value) => value.writeToBuffer(),
      $0.PermissionListInvitesResponse.fromBuffer);
  static final _$createInvite = $grpc.ClientMethod<
          $0.PermissionCreateInviteRequest, $0.PermissionCreateInviteResponse>(
      '/openoxide.api.v1.PermissionService/CreateInvite',
      ($0.PermissionCreateInviteRequest value) => value.writeToBuffer(),
      $0.PermissionCreateInviteResponse.fromBuffer);
  static final _$acceptInvite =
      $grpc.ClientMethod<$0.PermissionAcceptInviteRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/AcceptInvite',
          ($0.PermissionAcceptInviteRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$cancelInvite =
      $grpc.ClientMethod<$0.PermissionCancelInviteRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/CancelInvite',
          ($0.PermissionCancelInviteRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$replaceUserPolicies =
      $grpc.ClientMethod<$0.PermissionReplaceUserPoliciesRequest, $1.Empty>(
          '/openoxide.api.v1.PermissionService/ReplaceUserPolicies',
          ($0.PermissionReplaceUserPoliciesRequest value) =>
              value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.PermissionService')
abstract class PermissionServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.PermissionService';

  PermissionServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListPermissionsRequest,
            $0.ListPermissionsResponse>(
        'ListPermissions',
        listPermissions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListPermissionsRequest.fromBuffer(value),
        ($0.ListPermissionsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetPermissionRequest, $0.GetPermissionResponse>(
            'GetPermission',
            getPermission_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetPermissionRequest.fromBuffer(value),
            ($0.GetPermissionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreatePermissionRequest,
            $0.CreatePermissionResponse>(
        'CreatePermission',
        createPermission_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreatePermissionRequest.fromBuffer(value),
        ($0.CreatePermissionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdatePermissionRequest, $1.Empty>(
        'UpdatePermission',
        updatePermission_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdatePermissionRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeletePermissionRequest, $1.Empty>(
        'DeletePermission',
        deletePermission_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeletePermissionRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionListGroupsRequest,
            $0.PermissionListGroupsResponse>(
        'ListGroups',
        listGroups_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionListGroupsRequest.fromBuffer(value),
        ($0.PermissionListGroupsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionGetGroupRequest,
            $0.PermissionGetGroupResponse>(
        'GetGroup',
        getGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionGetGroupRequest.fromBuffer(value),
        ($0.PermissionGetGroupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionCreateGroupRequest,
            $0.PermissionCreateGroupResponse>(
        'CreateGroup',
        createGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionCreateGroupRequest.fromBuffer(value),
        ($0.PermissionCreateGroupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionUpdateGroupRequest, $1.Empty>(
        'UpdateGroup',
        updateGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionUpdateGroupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionDeleteGroupRequest, $1.Empty>(
        'DeleteGroup',
        deleteGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionDeleteGroupRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionListPoliciesRequest,
            $0.PermissionListPoliciesResponse>(
        'ListPolicies',
        listPolicies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionListPoliciesRequest.fromBuffer(value),
        ($0.PermissionListPoliciesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetMyPermissionsRequest,
            $0.GetMyPermissionsResponse>(
        'GetMyPermissions',
        getMyPermissions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetMyPermissionsRequest.fromBuffer(value),
        ($0.GetMyPermissionsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.PermissionAssignMemberGroupRequest, $1.Empty>(
            'AssignMemberGroup',
            assignMemberGroup_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.PermissionAssignMemberGroupRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.PermissionSetGroupActiveRequest, $1.Empty>(
            'SetGroupActive',
            setGroupActive_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.PermissionSetGroupActiveRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionListUserPoliciesRequest,
            $0.PermissionListUserPoliciesResponse>(
        'ListUserPolicies',
        listUserPolicies_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionListUserPoliciesRequest.fromBuffer(value),
        ($0.PermissionListUserPoliciesResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionListMembersRequest,
            $0.PermissionListMembersResponse>(
        'ListMembers',
        listMembers_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionListMembersRequest.fromBuffer(value),
        ($0.PermissionListMembersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionAddMemberRequest, $1.Empty>(
        'AddMember',
        addMember_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionAddMemberRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.PermissionUpdateMemberRoleRequest, $1.Empty>(
            'UpdateMemberRole',
            updateMemberRole_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.PermissionUpdateMemberRoleRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionRemoveMemberRequest, $1.Empty>(
        'RemoveMember',
        removeMember_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionRemoveMemberRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionListInvitesRequest,
            $0.PermissionListInvitesResponse>(
        'ListInvites',
        listInvites_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionListInvitesRequest.fromBuffer(value),
        ($0.PermissionListInvitesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionCreateInviteRequest,
            $0.PermissionCreateInviteResponse>(
        'CreateInvite',
        createInvite_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionCreateInviteRequest.fromBuffer(value),
        ($0.PermissionCreateInviteResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionAcceptInviteRequest, $1.Empty>(
        'AcceptInvite',
        acceptInvite_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionAcceptInviteRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PermissionCancelInviteRequest, $1.Empty>(
        'CancelInvite',
        cancelInvite_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PermissionCancelInviteRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.PermissionReplaceUserPoliciesRequest, $1.Empty>(
            'ReplaceUserPolicies',
            replaceUserPolicies_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.PermissionReplaceUserPoliciesRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListPermissionsResponse> listPermissions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListPermissionsRequest> $request) async {
    return listPermissions($call, await $request);
  }

  $async.Future<$0.ListPermissionsResponse> listPermissions(
      $grpc.ServiceCall call, $0.ListPermissionsRequest request);

  $async.Future<$0.GetPermissionResponse> getPermission_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetPermissionRequest> $request) async {
    return getPermission($call, await $request);
  }

  $async.Future<$0.GetPermissionResponse> getPermission(
      $grpc.ServiceCall call, $0.GetPermissionRequest request);

  $async.Future<$0.CreatePermissionResponse> createPermission_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreatePermissionRequest> $request) async {
    return createPermission($call, await $request);
  }

  $async.Future<$0.CreatePermissionResponse> createPermission(
      $grpc.ServiceCall call, $0.CreatePermissionRequest request);

  $async.Future<$1.Empty> updatePermission_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdatePermissionRequest> $request) async {
    return updatePermission($call, await $request);
  }

  $async.Future<$1.Empty> updatePermission(
      $grpc.ServiceCall call, $0.UpdatePermissionRequest request);

  $async.Future<$1.Empty> deletePermission_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeletePermissionRequest> $request) async {
    return deletePermission($call, await $request);
  }

  $async.Future<$1.Empty> deletePermission(
      $grpc.ServiceCall call, $0.DeletePermissionRequest request);

  $async.Future<$0.PermissionListGroupsResponse> listGroups_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionListGroupsRequest> $request) async {
    return listGroups($call, await $request);
  }

  $async.Future<$0.PermissionListGroupsResponse> listGroups(
      $grpc.ServiceCall call, $0.PermissionListGroupsRequest request);

  $async.Future<$0.PermissionGetGroupResponse> getGroup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionGetGroupRequest> $request) async {
    return getGroup($call, await $request);
  }

  $async.Future<$0.PermissionGetGroupResponse> getGroup(
      $grpc.ServiceCall call, $0.PermissionGetGroupRequest request);

  $async.Future<$0.PermissionCreateGroupResponse> createGroup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionCreateGroupRequest> $request) async {
    return createGroup($call, await $request);
  }

  $async.Future<$0.PermissionCreateGroupResponse> createGroup(
      $grpc.ServiceCall call, $0.PermissionCreateGroupRequest request);

  $async.Future<$1.Empty> updateGroup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionUpdateGroupRequest> $request) async {
    return updateGroup($call, await $request);
  }

  $async.Future<$1.Empty> updateGroup(
      $grpc.ServiceCall call, $0.PermissionUpdateGroupRequest request);

  $async.Future<$1.Empty> deleteGroup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionDeleteGroupRequest> $request) async {
    return deleteGroup($call, await $request);
  }

  $async.Future<$1.Empty> deleteGroup(
      $grpc.ServiceCall call, $0.PermissionDeleteGroupRequest request);

  $async.Future<$0.PermissionListPoliciesResponse> listPolicies_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionListPoliciesRequest> $request) async {
    return listPolicies($call, await $request);
  }

  $async.Future<$0.PermissionListPoliciesResponse> listPolicies(
      $grpc.ServiceCall call, $0.PermissionListPoliciesRequest request);

  $async.Future<$0.GetMyPermissionsResponse> getMyPermissions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetMyPermissionsRequest> $request) async {
    return getMyPermissions($call, await $request);
  }

  $async.Future<$0.GetMyPermissionsResponse> getMyPermissions(
      $grpc.ServiceCall call, $0.GetMyPermissionsRequest request);

  $async.Future<$1.Empty> assignMemberGroup_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionAssignMemberGroupRequest> $request) async {
    return assignMemberGroup($call, await $request);
  }

  $async.Future<$1.Empty> assignMemberGroup(
      $grpc.ServiceCall call, $0.PermissionAssignMemberGroupRequest request);

  $async.Future<$1.Empty> setGroupActive_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionSetGroupActiveRequest> $request) async {
    return setGroupActive($call, await $request);
  }

  $async.Future<$1.Empty> setGroupActive(
      $grpc.ServiceCall call, $0.PermissionSetGroupActiveRequest request);

  $async.Future<$0.PermissionListUserPoliciesResponse> listUserPolicies_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionListUserPoliciesRequest> $request) async {
    return listUserPolicies($call, await $request);
  }

  $async.Future<$0.PermissionListUserPoliciesResponse> listUserPolicies(
      $grpc.ServiceCall call, $0.PermissionListUserPoliciesRequest request);

  $async.Future<$0.PermissionListMembersResponse> listMembers_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionListMembersRequest> $request) async {
    return listMembers($call, await $request);
  }

  $async.Future<$0.PermissionListMembersResponse> listMembers(
      $grpc.ServiceCall call, $0.PermissionListMembersRequest request);

  $async.Future<$1.Empty> addMember_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionAddMemberRequest> $request) async {
    return addMember($call, await $request);
  }

  $async.Future<$1.Empty> addMember(
      $grpc.ServiceCall call, $0.PermissionAddMemberRequest request);

  $async.Future<$1.Empty> updateMemberRole_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionUpdateMemberRoleRequest> $request) async {
    return updateMemberRole($call, await $request);
  }

  $async.Future<$1.Empty> updateMemberRole(
      $grpc.ServiceCall call, $0.PermissionUpdateMemberRoleRequest request);

  $async.Future<$1.Empty> removeMember_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionRemoveMemberRequest> $request) async {
    return removeMember($call, await $request);
  }

  $async.Future<$1.Empty> removeMember(
      $grpc.ServiceCall call, $0.PermissionRemoveMemberRequest request);

  $async.Future<$0.PermissionListInvitesResponse> listInvites_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionListInvitesRequest> $request) async {
    return listInvites($call, await $request);
  }

  $async.Future<$0.PermissionListInvitesResponse> listInvites(
      $grpc.ServiceCall call, $0.PermissionListInvitesRequest request);

  $async.Future<$0.PermissionCreateInviteResponse> createInvite_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PermissionCreateInviteRequest> $request) async {
    return createInvite($call, await $request);
  }

  $async.Future<$0.PermissionCreateInviteResponse> createInvite(
      $grpc.ServiceCall call, $0.PermissionCreateInviteRequest request);

  $async.Future<$1.Empty> acceptInvite_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionAcceptInviteRequest> $request) async {
    return acceptInvite($call, await $request);
  }

  $async.Future<$1.Empty> acceptInvite(
      $grpc.ServiceCall call, $0.PermissionAcceptInviteRequest request);

  $async.Future<$1.Empty> cancelInvite_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionCancelInviteRequest> $request) async {
    return cancelInvite($call, await $request);
  }

  $async.Future<$1.Empty> cancelInvite(
      $grpc.ServiceCall call, $0.PermissionCancelInviteRequest request);

  $async.Future<$1.Empty> replaceUserPolicies_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PermissionReplaceUserPoliciesRequest> $request) async {
    return replaceUserPolicies($call, await $request);
  }

  $async.Future<$1.Empty> replaceUserPolicies(
      $grpc.ServiceCall call, $0.PermissionReplaceUserPoliciesRequest request);
}

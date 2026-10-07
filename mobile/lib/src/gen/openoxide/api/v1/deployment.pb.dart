// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/deployment.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Deployment extends $pb.GeneratedMessage {
  factory Deployment({
    $fixnum.Int64? id,
    $core.String? resourceType,
    $fixnum.Int64? resourceId,
    $core.String? status,
    $core.String? error,
    $core.String? logs,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $fixnum.Int64? userId,
    $core.String? userEmail,
  }) {
    final result = Deployment._();
    if (id != null) result.id = id;
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    if (status != null) result.status = status;
    if (error != null) result.error = error;
    if (logs != null) result.logs = logs;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (userId != null) result.userId = userId;
    if (userEmail != null) result.userEmail = userEmail;
    return result;
  }

  Deployment._();

  factory Deployment.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Deployment()..mergeFromBuffer(data, registry);
  factory Deployment.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Deployment()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Deployment',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Deployment.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'resourceType')
    ..aInt64(3, _omitFieldNames ? '' : 'resourceId')
    ..aOS(4, _omitFieldNames ? '' : 'status')
    ..aOS(5, _omitFieldNames ? '' : 'error')
    ..aOS(6, _omitFieldNames ? '' : 'logs')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..aInt64(9, _omitFieldNames ? '' : 'userId')
    ..aOS(10, _omitFieldNames ? '' : 'userEmail')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Deployment clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Deployment copyWith(void Function(Deployment) updates) =>
      super.copyWith((message) => updates(message as Deployment)) as Deployment;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Deployment() / Deployment.new instead')
  static Deployment create() => Deployment._();
  static $pb.GeneratedMessage $_createMessage() => Deployment._();
  @$core.override
  Deployment createEmptyInstance() => Deployment._();
  @$core.pragma('dart2js:noInline')
  static Deployment getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Deployment>(Deployment.$_createMessage);
  static Deployment? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get resourceType => $_getSZ(1);
  @$pb.TagNumber(2)
  set resourceType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasResourceType() => $_has(1);
  @$pb.TagNumber(2)
  void clearResourceType() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get resourceId => $_getI64(2);
  @$pb.TagNumber(3)
  set resourceId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasResourceId() => $_has(2);
  @$pb.TagNumber(3)
  void clearResourceId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get status => $_getSZ(3);
  @$pb.TagNumber(4)
  set status($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get error => $_getSZ(4);
  @$pb.TagNumber(5)
  set error($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasError() => $_has(4);
  @$pb.TagNumber(5)
  void clearError() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get logs => $_getSZ(5);
  @$pb.TagNumber(6)
  set logs($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLogs() => $_has(5);
  @$pb.TagNumber(6)
  void clearLogs() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get updatedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set updatedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasUpdatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearUpdatedAt() => $_clearField(8);

  /// Acting user (0 = system-triggered: webhook/schedule/preview).
  @$pb.TagNumber(9)
  $fixnum.Int64 get userId => $_getI64(8);
  @$pb.TagNumber(9)
  set userId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUserId() => $_has(8);
  @$pb.TagNumber(9)
  void clearUserId() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get userEmail => $_getSZ(9);
  @$pb.TagNumber(10)
  set userEmail($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUserEmail() => $_has(9);
  @$pb.TagNumber(10)
  void clearUserEmail() => $_clearField(10);
}

class ListDeploymentsRequest extends $pb.GeneratedMessage {
  factory ListDeploymentsRequest({
    $core.String? resourceType,
    $fixnum.Int64? resourceId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = ListDeploymentsRequest._();
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  ListDeploymentsRequest._();

  factory ListDeploymentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDeploymentsRequest()..mergeFromBuffer(data, registry);
  factory ListDeploymentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDeploymentsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDeploymentsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDeploymentsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'resourceType')
    ..aInt64(2, _omitFieldNames ? '' : 'resourceId')
    ..aInt64(3, _omitFieldNames ? '' : 'limit')
    ..aInt64(4, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDeploymentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDeploymentsRequest copyWith(
          void Function(ListDeploymentsRequest) updates) =>
      super.copyWith((message) => updates(message as ListDeploymentsRequest))
          as ListDeploymentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDeploymentsRequest() / ListDeploymentsRequest.new instead')
  static ListDeploymentsRequest create() => ListDeploymentsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDeploymentsRequest._();
  @$core.override
  ListDeploymentsRequest createEmptyInstance() => ListDeploymentsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDeploymentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDeploymentsRequest>(
          ListDeploymentsRequest.$_createMessage);
  static ListDeploymentsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get resourceType => $_getSZ(0);
  @$pb.TagNumber(1)
  set resourceType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasResourceType() => $_has(0);
  @$pb.TagNumber(1)
  void clearResourceType() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get resourceId => $_getI64(1);
  @$pb.TagNumber(2)
  set resourceId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasResourceId() => $_has(1);
  @$pb.TagNumber(2)
  void clearResourceId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get limit => $_getI64(2);
  @$pb.TagNumber(3)
  set limit($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLimit() => $_has(2);
  @$pb.TagNumber(3)
  void clearLimit() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get offset => $_getI64(3);
  @$pb.TagNumber(4)
  set offset($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOffset() => $_has(3);
  @$pb.TagNumber(4)
  void clearOffset() => $_clearField(4);
}

class ListDeploymentsResponse extends $pb.GeneratedMessage {
  factory ListDeploymentsResponse({
    $core.Iterable<Deployment>? deployments,
  }) {
    final result = ListDeploymentsResponse._();
    if (deployments != null) result.deployments.addAll(deployments);
    return result;
  }

  ListDeploymentsResponse._();

  factory ListDeploymentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDeploymentsResponse()..mergeFromBuffer(data, registry);
  factory ListDeploymentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDeploymentsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDeploymentsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDeploymentsResponse.$_createMessage)
    ..pPM<Deployment>(1, _omitFieldNames ? '' : 'deployments',
        subBuilder: Deployment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDeploymentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDeploymentsResponse copyWith(
          void Function(ListDeploymentsResponse) updates) =>
      super.copyWith((message) => updates(message as ListDeploymentsResponse))
          as ListDeploymentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDeploymentsResponse() / ListDeploymentsResponse.new instead')
  static ListDeploymentsResponse create() => ListDeploymentsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDeploymentsResponse._();
  @$core.override
  ListDeploymentsResponse createEmptyInstance() => ListDeploymentsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDeploymentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDeploymentsResponse>(
          ListDeploymentsResponse.$_createMessage);
  static ListDeploymentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Deployment> get deployments => $_getList(0);
}

class GetDeploymentRequest extends $pb.GeneratedMessage {
  factory GetDeploymentRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetDeploymentRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetDeploymentRequest._();

  factory GetDeploymentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDeploymentRequest()..mergeFromBuffer(data, registry);
  factory GetDeploymentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDeploymentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDeploymentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDeploymentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDeploymentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDeploymentRequest copyWith(void Function(GetDeploymentRequest) updates) =>
      super.copyWith((message) => updates(message as GetDeploymentRequest))
          as GetDeploymentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetDeploymentRequest() / GetDeploymentRequest.new instead')
  static GetDeploymentRequest create() => GetDeploymentRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetDeploymentRequest._();
  @$core.override
  GetDeploymentRequest createEmptyInstance() => GetDeploymentRequest._();
  @$core.pragma('dart2js:noInline')
  static GetDeploymentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDeploymentRequest>(
          GetDeploymentRequest.$_createMessage);
  static GetDeploymentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class SearchLogsRequest extends $pb.GeneratedMessage {
  factory SearchLogsRequest({
    $core.String? query,
    $core.String? resourceType,
    $fixnum.Int64? resourceId,
  }) {
    final result = SearchLogsRequest._();
    if (query != null) result.query = query;
    if (resourceType != null) result.resourceType = resourceType;
    if (resourceId != null) result.resourceId = resourceId;
    return result;
  }

  SearchLogsRequest._();

  factory SearchLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SearchLogsRequest()..mergeFromBuffer(data, registry);
  factory SearchLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SearchLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SearchLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SearchLogsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'query')
    ..aOS(2, _omitFieldNames ? '' : 'resourceType')
    ..aInt64(3, _omitFieldNames ? '' : 'resourceId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchLogsRequest copyWith(void Function(SearchLogsRequest) updates) =>
      super.copyWith((message) => updates(message as SearchLogsRequest))
          as SearchLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SearchLogsRequest() / SearchLogsRequest.new instead')
  static SearchLogsRequest create() => SearchLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() => SearchLogsRequest._();
  @$core.override
  SearchLogsRequest createEmptyInstance() => SearchLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static SearchLogsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SearchLogsRequest>(
          SearchLogsRequest.$_createMessage);
  static SearchLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get query => $_getSZ(0);
  @$pb.TagNumber(1)
  set query($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasQuery() => $_has(0);
  @$pb.TagNumber(1)
  void clearQuery() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get resourceType => $_getSZ(1);
  @$pb.TagNumber(2)
  set resourceType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasResourceType() => $_has(1);
  @$pb.TagNumber(2)
  void clearResourceType() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get resourceId => $_getI64(2);
  @$pb.TagNumber(3)
  set resourceId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasResourceId() => $_has(2);
  @$pb.TagNumber(3)
  void clearResourceId() => $_clearField(3);
}

class StreamTargetRequest extends $pb.GeneratedMessage {
  factory StreamTargetRequest({
    $fixnum.Int64? id,
  }) {
    final result = StreamTargetRequest._();
    if (id != null) result.id = id;
    return result;
  }

  StreamTargetRequest._();

  factory StreamTargetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StreamTargetRequest()..mergeFromBuffer(data, registry);
  factory StreamTargetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StreamTargetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StreamTargetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: StreamTargetRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StreamTargetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StreamTargetRequest copyWith(void Function(StreamTargetRequest) updates) =>
      super.copyWith((message) => updates(message as StreamTargetRequest))
          as StreamTargetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use StreamTargetRequest() / StreamTargetRequest.new instead')
  static StreamTargetRequest create() => StreamTargetRequest._();
  static $pb.GeneratedMessage $_createMessage() => StreamTargetRequest._();
  @$core.override
  StreamTargetRequest createEmptyInstance() => StreamTargetRequest._();
  @$core.pragma('dart2js:noInline')
  static StreamTargetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StreamTargetRequest>(
          StreamTargetRequest.$_createMessage);
  static StreamTargetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Global live container stats for the monitoring page. No server_id means the panel host's Agent; the stream covers every running container.
class DockerStatsRequest extends $pb.GeneratedMessage {
  factory DockerStatsRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = DockerStatsRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  DockerStatsRequest._();

  factory DockerStatsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DockerStatsRequest()..mergeFromBuffer(data, registry);
  factory DockerStatsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DockerStatsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DockerStatsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DockerStatsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DockerStatsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DockerStatsRequest copyWith(void Function(DockerStatsRequest) updates) =>
      super.copyWith((message) => updates(message as DockerStatsRequest))
          as DockerStatsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DockerStatsRequest() / DockerStatsRequest.new instead')
  static DockerStatsRequest create() => DockerStatsRequest._();
  static $pb.GeneratedMessage $_createMessage() => DockerStatsRequest._();
  @$core.override
  DockerStatsRequest createEmptyInstance() => DockerStatsRequest._();
  @$core.pragma('dart2js:noInline')
  static DockerStatsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DockerStatsRequest>(
          DockerStatsRequest.$_createMessage);
  static DockerStatsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class LogTargetRequest extends $pb.GeneratedMessage {
  factory LogTargetRequest({
    $core.String? target,
    $fixnum.Int64? serverId,
    $fixnum.Int64? tail,
    $core.String? filter,
    $core.bool? timestamps,
    $core.bool? follow,
  }) {
    final result = LogTargetRequest._();
    if (target != null) result.target = target;
    if (serverId != null) result.serverId = serverId;
    if (tail != null) result.tail = tail;
    if (filter != null) result.filter = filter;
    if (timestamps != null) result.timestamps = timestamps;
    if (follow != null) result.follow = follow;
    return result;
  }

  LogTargetRequest._();

  factory LogTargetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogTargetRequest()..mergeFromBuffer(data, registry);
  factory LogTargetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogTargetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogTargetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: LogTargetRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'target')
    ..aInt64(2, _omitFieldNames ? '' : 'serverId')
    ..aInt64(3, _omitFieldNames ? '' : 'tail')
    ..aOS(4, _omitFieldNames ? '' : 'filter')
    ..aOB(5, _omitFieldNames ? '' : 'timestamps')
    ..aOB(6, _omitFieldNames ? '' : 'follow')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogTargetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogTargetRequest copyWith(void Function(LogTargetRequest) updates) =>
      super.copyWith((message) => updates(message as LogTargetRequest))
          as LogTargetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use LogTargetRequest() / LogTargetRequest.new instead')
  static LogTargetRequest create() => LogTargetRequest._();
  static $pb.GeneratedMessage $_createMessage() => LogTargetRequest._();
  @$core.override
  LogTargetRequest createEmptyInstance() => LogTargetRequest._();
  @$core.pragma('dart2js:noInline')
  static LogTargetRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogTargetRequest>(
          LogTargetRequest.$_createMessage);
  static LogTargetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get target => $_getSZ(0);
  @$pb.TagNumber(1)
  set target($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTarget() => $_has(0);
  @$pb.TagNumber(1)
  void clearTarget() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get serverId => $_getI64(1);
  @$pb.TagNumber(2)
  set serverId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get tail => $_getI64(2);
  @$pb.TagNumber(3)
  set tail($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTail() => $_has(2);
  @$pb.TagNumber(3)
  void clearTail() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get filter => $_getSZ(3);
  @$pb.TagNumber(4)
  set filter($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFilter() => $_has(3);
  @$pb.TagNumber(4)
  void clearFilter() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get timestamps => $_getBF(4);
  @$pb.TagNumber(5)
  set timestamps($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTimestamps() => $_has(4);
  @$pb.TagNumber(5)
  void clearTimestamps() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get follow => $_getBF(5);
  @$pb.TagNumber(6)
  set follow($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasFollow() => $_has(5);
  @$pb.TagNumber(6)
  void clearFollow() => $_clearField(6);
}

class ComposeLogsRequest extends $pb.GeneratedMessage {
  factory ComposeLogsRequest({
    $fixnum.Int64? composeId,
    $core.String? service,
    $fixnum.Int64? serverId,
    $core.bool? timestamps,
    $core.bool? follow,
    $fixnum.Int64? tail,
  }) {
    final result = ComposeLogsRequest._();
    if (composeId != null) result.composeId = composeId;
    if (service != null) result.service = service;
    if (serverId != null) result.serverId = serverId;
    if (timestamps != null) result.timestamps = timestamps;
    if (follow != null) result.follow = follow;
    if (tail != null) result.tail = tail;
    return result;
  }

  ComposeLogsRequest._();

  factory ComposeLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeLogsRequest()..mergeFromBuffer(data, registry);
  factory ComposeLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aOS(2, _omitFieldNames ? '' : 'service')
    ..aInt64(3, _omitFieldNames ? '' : 'serverId')
    ..aOB(4, _omitFieldNames ? '' : 'timestamps')
    ..aOB(5, _omitFieldNames ? '' : 'follow')
    ..aInt64(6, _omitFieldNames ? '' : 'tail')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeLogsRequest copyWith(void Function(ComposeLogsRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeLogsRequest))
          as ComposeLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposeLogsRequest() / ComposeLogsRequest.new instead')
  static ComposeLogsRequest create() => ComposeLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ComposeLogsRequest._();
  @$core.override
  ComposeLogsRequest createEmptyInstance() => ComposeLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeLogsRequest>(
          ComposeLogsRequest.$_createMessage);
  static ComposeLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get service => $_getSZ(1);
  @$pb.TagNumber(2)
  set service($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasService() => $_has(1);
  @$pb.TagNumber(2)
  void clearService() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get serverId => $_getI64(2);
  @$pb.TagNumber(3)
  set serverId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasServerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearServerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get timestamps => $_getBF(3);
  @$pb.TagNumber(4)
  set timestamps($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTimestamps() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimestamps() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get follow => $_getBF(4);
  @$pb.TagNumber(5)
  set follow($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFollow() => $_has(4);
  @$pb.TagNumber(5)
  void clearFollow() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get tail => $_getI64(5);
  @$pb.TagNumber(6)
  set tail($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTail() => $_has(5);
  @$pb.TagNumber(6)
  void clearTail() => $_clearField(6);
}

class ResourceTargetRequest extends $pb.GeneratedMessage {
  factory ResourceTargetRequest({
    $fixnum.Int64? id,
  }) {
    final result = ResourceTargetRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ResourceTargetRequest._();

  factory ResourceTargetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResourceTargetRequest()..mergeFromBuffer(data, registry);
  factory ResourceTargetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResourceTargetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ResourceTargetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ResourceTargetRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceTargetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceTargetRequest copyWith(
          void Function(ResourceTargetRequest) updates) =>
      super.copyWith((message) => updates(message as ResourceTargetRequest))
          as ResourceTargetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ResourceTargetRequest() / ResourceTargetRequest.new instead')
  static ResourceTargetRequest create() => ResourceTargetRequest._();
  static $pb.GeneratedMessage $_createMessage() => ResourceTargetRequest._();
  @$core.override
  ResourceTargetRequest createEmptyInstance() => ResourceTargetRequest._();
  @$core.pragma('dart2js:noInline')
  static ResourceTargetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ResourceTargetRequest>(
          ResourceTargetRequest.$_createMessage);
  static ResourceTargetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class CancelDeploymentRequest extends $pb.GeneratedMessage {
  factory CancelDeploymentRequest({
    $fixnum.Int64? id,
  }) {
    final result = CancelDeploymentRequest._();
    if (id != null) result.id = id;
    return result;
  }

  CancelDeploymentRequest._();

  factory CancelDeploymentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CancelDeploymentRequest()..mergeFromBuffer(data, registry);
  factory CancelDeploymentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CancelDeploymentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CancelDeploymentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CancelDeploymentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelDeploymentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelDeploymentRequest copyWith(
          void Function(CancelDeploymentRequest) updates) =>
      super.copyWith((message) => updates(message as CancelDeploymentRequest))
          as CancelDeploymentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CancelDeploymentRequest() / CancelDeploymentRequest.new instead')
  static CancelDeploymentRequest create() => CancelDeploymentRequest._();
  static $pb.GeneratedMessage $_createMessage() => CancelDeploymentRequest._();
  @$core.override
  CancelDeploymentRequest createEmptyInstance() => CancelDeploymentRequest._();
  @$core.pragma('dart2js:noInline')
  static CancelDeploymentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CancelDeploymentRequest>(
          CancelDeploymentRequest.$_createMessage);
  static CancelDeploymentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DeploymentEvent extends $pb.GeneratedMessage {
  factory DeploymentEvent({
    $core.String? type,
    $fixnum.Int64? timestamp,
    $core.String? payload,
    $core.String? error,
  }) {
    final result = DeploymentEvent._();
    if (type != null) result.type = type;
    if (timestamp != null) result.timestamp = timestamp;
    if (payload != null) result.payload = payload;
    if (error != null) result.error = error;
    return result;
  }

  DeploymentEvent._();

  factory DeploymentEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeploymentEvent()..mergeFromBuffer(data, registry);
  factory DeploymentEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeploymentEvent()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeploymentEvent',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeploymentEvent.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'type')
    ..aInt64(2, _omitFieldNames ? '' : 'timestamp')
    ..aOS(3, _omitFieldNames ? '' : 'payload')
    ..aOS(4, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeploymentEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeploymentEvent copyWith(void Function(DeploymentEvent) updates) =>
      super.copyWith((message) => updates(message as DeploymentEvent))
          as DeploymentEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DeploymentEvent() / DeploymentEvent.new instead')
  static DeploymentEvent create() => DeploymentEvent._();
  static $pb.GeneratedMessage $_createMessage() => DeploymentEvent._();
  @$core.override
  DeploymentEvent createEmptyInstance() => DeploymentEvent._();
  @$core.pragma('dart2js:noInline')
  static DeploymentEvent getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeploymentEvent>(
          DeploymentEvent.$_createMessage);
  static DeploymentEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get type => $_getSZ(0);
  @$pb.TagNumber(1)
  set type($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasType() => $_has(0);
  @$pb.TagNumber(1)
  void clearType() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get timestamp => $_getI64(1);
  @$pb.TagNumber(2)
  set timestamp($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get payload => $_getSZ(2);
  @$pb.TagNumber(3)
  set payload($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPayload() => $_has(2);
  @$pb.TagNumber(3)
  void clearPayload() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get error => $_getSZ(3);
  @$pb.TagNumber(4)
  set error($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasError() => $_has(3);
  @$pb.TagNumber(4)
  void clearError() => $_clearField(4);
}

class LogExport extends $pb.GeneratedMessage {
  factory LogExport({
    $core.String? content,
    $core.String? fileName,
  }) {
    final result = LogExport._();
    if (content != null) result.content = content;
    if (fileName != null) result.fileName = fileName;
    return result;
  }

  LogExport._();

  factory LogExport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogExport()..mergeFromBuffer(data, registry);
  factory LogExport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogExport()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogExport',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: LogExport.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..aOS(2, _omitFieldNames ? '' : 'fileName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogExport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogExport copyWith(void Function(LogExport) updates) =>
      super.copyWith((message) => updates(message as LogExport)) as LogExport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use LogExport() / LogExport.new instead')
  static LogExport create() => LogExport._();
  static $pb.GeneratedMessage $_createMessage() => LogExport._();
  @$core.override
  LogExport createEmptyInstance() => LogExport._();
  @$core.pragma('dart2js:noInline')
  static LogExport getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LogExport>(LogExport.$_createMessage);
  static LogExport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => $_clearField(2);
}

class ContainerList extends $pb.GeneratedMessage {
  factory ContainerList({
    $core.Iterable<Container>? containers,
  }) {
    final result = ContainerList._();
    if (containers != null) result.containers.addAll(containers);
    return result;
  }

  ContainerList._();

  factory ContainerList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ContainerList()..mergeFromBuffer(data, registry);
  factory ContainerList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ContainerList()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ContainerList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ContainerList.$_createMessage)
    ..pPM<Container>(1, _omitFieldNames ? '' : 'containers',
        subBuilder: Container.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ContainerList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ContainerList copyWith(void Function(ContainerList) updates) =>
      super.copyWith((message) => updates(message as ContainerList))
          as ContainerList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ContainerList() / ContainerList.new instead')
  static ContainerList create() => ContainerList._();
  static $pb.GeneratedMessage $_createMessage() => ContainerList._();
  @$core.override
  ContainerList createEmptyInstance() => ContainerList._();
  @$core.pragma('dart2js:noInline')
  static ContainerList getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ContainerList>(
          ContainerList.$_createMessage);
  static ContainerList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Container> get containers => $_getList(0);
}

class Container extends $pb.GeneratedMessage {
  factory Container({
    $core.String? id,
    $core.String? name,
    $core.String? image,
    $core.String? state,
    $core.String? status,
  }) {
    final result = Container._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (image != null) result.image = image;
    if (state != null) result.state = state;
    if (status != null) result.status = status;
    return result;
  }

  Container._();

  factory Container.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Container()..mergeFromBuffer(data, registry);
  factory Container.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Container()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Container',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Container.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'image')
    ..aOS(4, _omitFieldNames ? '' : 'state')
    ..aOS(5, _omitFieldNames ? '' : 'status')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Container clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Container copyWith(void Function(Container) updates) =>
      super.copyWith((message) => updates(message as Container)) as Container;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Container() / Container.new instead')
  static Container create() => Container._();
  static $pb.GeneratedMessage $_createMessage() => Container._();
  @$core.override
  Container createEmptyInstance() => Container._();
  @$core.pragma('dart2js:noInline')
  static Container getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Container>(Container.$_createMessage);
  static Container? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get image => $_getSZ(2);
  @$pb.TagNumber(3)
  set image($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasImage() => $_has(2);
  @$pb.TagNumber(3)
  void clearImage() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get state => $_getSZ(3);
  @$pb.TagNumber(4)
  set state($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasState() => $_has(3);
  @$pb.TagNumber(4)
  void clearState() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get status => $_getSZ(4);
  @$pb.TagNumber(5)
  set status($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(5)
  void clearStatus() => $_clearField(5);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

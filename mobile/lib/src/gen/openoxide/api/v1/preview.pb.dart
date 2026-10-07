// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/preview.proto.

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

class PreviewDeployment extends $pb.GeneratedMessage {
  factory PreviewDeployment({
    $fixnum.Int64? id,
    $fixnum.Int64? applicationId,
    $core.String? status,
    $core.String? branch,
    $core.String? url,
    $core.String? error,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = PreviewDeployment._();
    if (id != null) result.id = id;
    if (applicationId != null) result.applicationId = applicationId;
    if (status != null) result.status = status;
    if (branch != null) result.branch = branch;
    if (url != null) result.url = url;
    if (error != null) result.error = error;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  PreviewDeployment._();

  factory PreviewDeployment.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PreviewDeployment()..mergeFromBuffer(data, registry);
  factory PreviewDeployment.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PreviewDeployment()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PreviewDeployment',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PreviewDeployment.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'applicationId')
    ..aOS(3, _omitFieldNames ? '' : 'status')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'url')
    ..aOS(6, _omitFieldNames ? '' : 'error')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewDeployment clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewDeployment copyWith(void Function(PreviewDeployment) updates) =>
      super.copyWith((message) => updates(message as PreviewDeployment))
          as PreviewDeployment;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PreviewDeployment() / PreviewDeployment.new instead')
  static PreviewDeployment create() => PreviewDeployment._();
  static $pb.GeneratedMessage $_createMessage() => PreviewDeployment._();
  @$core.override
  PreviewDeployment createEmptyInstance() => PreviewDeployment._();
  @$core.pragma('dart2js:noInline')
  static PreviewDeployment getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PreviewDeployment>(
          PreviewDeployment.$_createMessage);
  static PreviewDeployment? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get applicationId => $_getI64(1);
  @$pb.TagNumber(2)
  set applicationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApplicationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearApplicationId() => $_clearField(2);

  /// Provider/deployment lifecycle string (e.g. "ERROR", "CLOSED"); open set.
  @$pb.TagNumber(3)
  $core.String get status => $_getSZ(2);
  @$pb.TagNumber(3)
  set status($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get branch => $_getSZ(3);
  @$pb.TagNumber(4)
  set branch($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBranch() => $_has(3);
  @$pb.TagNumber(4)
  void clearBranch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get url => $_getSZ(4);
  @$pb.TagNumber(5)
  set url($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearUrl() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get error => $_getSZ(5);
  @$pb.TagNumber(6)
  set error($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasError() => $_has(5);
  @$pb.TagNumber(6)
  void clearError() => $_clearField(6);

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
}

class ListPreviewsRequest extends $pb.GeneratedMessage {
  factory ListPreviewsRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ListPreviewsRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ListPreviewsRequest._();

  factory ListPreviewsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPreviewsRequest()..mergeFromBuffer(data, registry);
  factory ListPreviewsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPreviewsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPreviewsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPreviewsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPreviewsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPreviewsRequest copyWith(void Function(ListPreviewsRequest) updates) =>
      super.copyWith((message) => updates(message as ListPreviewsRequest))
          as ListPreviewsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListPreviewsRequest() / ListPreviewsRequest.new instead')
  static ListPreviewsRequest create() => ListPreviewsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListPreviewsRequest._();
  @$core.override
  ListPreviewsRequest createEmptyInstance() => ListPreviewsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListPreviewsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPreviewsRequest>(
          ListPreviewsRequest.$_createMessage);
  static ListPreviewsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ListPreviewsResponse extends $pb.GeneratedMessage {
  factory ListPreviewsResponse({
    $core.Iterable<PreviewDeployment>? previews,
  }) {
    final result = ListPreviewsResponse._();
    if (previews != null) result.previews.addAll(previews);
    return result;
  }

  ListPreviewsResponse._();

  factory ListPreviewsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPreviewsResponse()..mergeFromBuffer(data, registry);
  factory ListPreviewsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPreviewsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPreviewsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPreviewsResponse.$_createMessage)
    ..pPM<PreviewDeployment>(1, _omitFieldNames ? '' : 'previews',
        subBuilder: PreviewDeployment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPreviewsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPreviewsResponse copyWith(void Function(ListPreviewsResponse) updates) =>
      super.copyWith((message) => updates(message as ListPreviewsResponse))
          as ListPreviewsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListPreviewsResponse() / ListPreviewsResponse.new instead')
  static ListPreviewsResponse create() => ListPreviewsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListPreviewsResponse._();
  @$core.override
  ListPreviewsResponse createEmptyInstance() => ListPreviewsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListPreviewsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPreviewsResponse>(
          ListPreviewsResponse.$_createMessage);
  static ListPreviewsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PreviewDeployment> get previews => $_getList(0);
}

class GetPreviewRequest extends $pb.GeneratedMessage {
  factory GetPreviewRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetPreviewRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetPreviewRequest._();

  factory GetPreviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPreviewRequest()..mergeFromBuffer(data, registry);
  factory GetPreviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPreviewRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPreviewRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetPreviewRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPreviewRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPreviewRequest copyWith(void Function(GetPreviewRequest) updates) =>
      super.copyWith((message) => updates(message as GetPreviewRequest))
          as GetPreviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetPreviewRequest() / GetPreviewRequest.new instead')
  static GetPreviewRequest create() => GetPreviewRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetPreviewRequest._();
  @$core.override
  GetPreviewRequest createEmptyInstance() => GetPreviewRequest._();
  @$core.pragma('dart2js:noInline')
  static GetPreviewRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetPreviewRequest>(
          GetPreviewRequest.$_createMessage);
  static GetPreviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetPreviewResponse extends $pb.GeneratedMessage {
  factory GetPreviewResponse({
    PreviewDeployment? preview,
  }) {
    final result = GetPreviewResponse._();
    if (preview != null) result.preview = preview;
    return result;
  }

  GetPreviewResponse._();

  factory GetPreviewResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPreviewResponse()..mergeFromBuffer(data, registry);
  factory GetPreviewResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetPreviewResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPreviewResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetPreviewResponse.$_createMessage)
    ..aOM<PreviewDeployment>(1, _omitFieldNames ? '' : 'preview',
        subBuilder: PreviewDeployment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPreviewResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPreviewResponse copyWith(void Function(GetPreviewResponse) updates) =>
      super.copyWith((message) => updates(message as GetPreviewResponse))
          as GetPreviewResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetPreviewResponse() / GetPreviewResponse.new instead')
  static GetPreviewResponse create() => GetPreviewResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetPreviewResponse._();
  @$core.override
  GetPreviewResponse createEmptyInstance() => GetPreviewResponse._();
  @$core.pragma('dart2js:noInline')
  static GetPreviewResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetPreviewResponse>(
          GetPreviewResponse.$_createMessage);
  static GetPreviewResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PreviewDeployment get preview => $_getN(0);
  @$pb.TagNumber(1)
  set preview(PreviewDeployment value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPreview() => $_has(0);
  @$pb.TagNumber(1)
  void clearPreview() => $_clearField(1);
  @$pb.TagNumber(1)
  PreviewDeployment ensurePreview() => $_ensure(0);
}

class CreatePreviewRequest extends $pb.GeneratedMessage {
  factory CreatePreviewRequest({
    $fixnum.Int64? applicationId,
    $core.String? branch,
    $core.String? commit,
  }) {
    final result = CreatePreviewRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (branch != null) result.branch = branch;
    if (commit != null) result.commit = commit;
    return result;
  }

  CreatePreviewRequest._();

  factory CreatePreviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePreviewRequest()..mergeFromBuffer(data, registry);
  factory CreatePreviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePreviewRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePreviewRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatePreviewRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aOS(2, _omitFieldNames ? '' : 'branch')
    ..aOS(3, _omitFieldNames ? '' : 'commit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePreviewRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePreviewRequest copyWith(void Function(CreatePreviewRequest) updates) =>
      super.copyWith((message) => updates(message as CreatePreviewRequest))
          as CreatePreviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatePreviewRequest() / CreatePreviewRequest.new instead')
  static CreatePreviewRequest create() => CreatePreviewRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreatePreviewRequest._();
  @$core.override
  CreatePreviewRequest createEmptyInstance() => CreatePreviewRequest._();
  @$core.pragma('dart2js:noInline')
  static CreatePreviewRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePreviewRequest>(
          CreatePreviewRequest.$_createMessage);
  static CreatePreviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get branch => $_getSZ(1);
  @$pb.TagNumber(2)
  set branch($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBranch() => $_has(1);
  @$pb.TagNumber(2)
  void clearBranch() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get commit => $_getSZ(2);
  @$pb.TagNumber(3)
  set commit($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCommit() => $_has(2);
  @$pb.TagNumber(3)
  void clearCommit() => $_clearField(3);
}

class CreatePreviewResponse extends $pb.GeneratedMessage {
  factory CreatePreviewResponse({
    PreviewDeployment? preview,
  }) {
    final result = CreatePreviewResponse._();
    if (preview != null) result.preview = preview;
    return result;
  }

  CreatePreviewResponse._();

  factory CreatePreviewResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePreviewResponse()..mergeFromBuffer(data, registry);
  factory CreatePreviewResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePreviewResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePreviewResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatePreviewResponse.$_createMessage)
    ..aOM<PreviewDeployment>(1, _omitFieldNames ? '' : 'preview',
        subBuilder: PreviewDeployment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePreviewResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePreviewResponse copyWith(
          void Function(CreatePreviewResponse) updates) =>
      super.copyWith((message) => updates(message as CreatePreviewResponse))
          as CreatePreviewResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatePreviewResponse() / CreatePreviewResponse.new instead')
  static CreatePreviewResponse create() => CreatePreviewResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreatePreviewResponse._();
  @$core.override
  CreatePreviewResponse createEmptyInstance() => CreatePreviewResponse._();
  @$core.pragma('dart2js:noInline')
  static CreatePreviewResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePreviewResponse>(
          CreatePreviewResponse.$_createMessage);
  static CreatePreviewResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PreviewDeployment get preview => $_getN(0);
  @$pb.TagNumber(1)
  set preview(PreviewDeployment value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPreview() => $_has(0);
  @$pb.TagNumber(1)
  void clearPreview() => $_clearField(1);
  @$pb.TagNumber(1)
  PreviewDeployment ensurePreview() => $_ensure(0);
}

class RedeployPreviewRequest extends $pb.GeneratedMessage {
  factory RedeployPreviewRequest({
    $fixnum.Int64? id,
  }) {
    final result = RedeployPreviewRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RedeployPreviewRequest._();

  factory RedeployPreviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RedeployPreviewRequest()..mergeFromBuffer(data, registry);
  factory RedeployPreviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RedeployPreviewRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RedeployPreviewRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RedeployPreviewRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RedeployPreviewRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RedeployPreviewRequest copyWith(
          void Function(RedeployPreviewRequest) updates) =>
      super.copyWith((message) => updates(message as RedeployPreviewRequest))
          as RedeployPreviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RedeployPreviewRequest() / RedeployPreviewRequest.new instead')
  static RedeployPreviewRequest create() => RedeployPreviewRequest._();
  static $pb.GeneratedMessage $_createMessage() => RedeployPreviewRequest._();
  @$core.override
  RedeployPreviewRequest createEmptyInstance() => RedeployPreviewRequest._();
  @$core.pragma('dart2js:noInline')
  static RedeployPreviewRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RedeployPreviewRequest>(
          RedeployPreviewRequest.$_createMessage);
  static RedeployPreviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class RedeployPreviewResponse extends $pb.GeneratedMessage {
  factory RedeployPreviewResponse({
    PreviewDeployment? preview,
  }) {
    final result = RedeployPreviewResponse._();
    if (preview != null) result.preview = preview;
    return result;
  }

  RedeployPreviewResponse._();

  factory RedeployPreviewResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RedeployPreviewResponse()..mergeFromBuffer(data, registry);
  factory RedeployPreviewResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RedeployPreviewResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RedeployPreviewResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RedeployPreviewResponse.$_createMessage)
    ..aOM<PreviewDeployment>(1, _omitFieldNames ? '' : 'preview',
        subBuilder: PreviewDeployment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RedeployPreviewResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RedeployPreviewResponse copyWith(
          void Function(RedeployPreviewResponse) updates) =>
      super.copyWith((message) => updates(message as RedeployPreviewResponse))
          as RedeployPreviewResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RedeployPreviewResponse() / RedeployPreviewResponse.new instead')
  static RedeployPreviewResponse create() => RedeployPreviewResponse._();
  static $pb.GeneratedMessage $_createMessage() => RedeployPreviewResponse._();
  @$core.override
  RedeployPreviewResponse createEmptyInstance() => RedeployPreviewResponse._();
  @$core.pragma('dart2js:noInline')
  static RedeployPreviewResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RedeployPreviewResponse>(
          RedeployPreviewResponse.$_createMessage);
  static RedeployPreviewResponse? _defaultInstance;

  @$pb.TagNumber(1)
  PreviewDeployment get preview => $_getN(0);
  @$pb.TagNumber(1)
  set preview(PreviewDeployment value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPreview() => $_has(0);
  @$pb.TagNumber(1)
  void clearPreview() => $_clearField(1);
  @$pb.TagNumber(1)
  PreviewDeployment ensurePreview() => $_ensure(0);
}

class DeletePreviewRequest extends $pb.GeneratedMessage {
  factory DeletePreviewRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeletePreviewRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeletePreviewRequest._();

  factory DeletePreviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeletePreviewRequest()..mergeFromBuffer(data, registry);
  factory DeletePreviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeletePreviewRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeletePreviewRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeletePreviewRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePreviewRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePreviewRequest copyWith(void Function(DeletePreviewRequest) updates) =>
      super.copyWith((message) => updates(message as DeletePreviewRequest))
          as DeletePreviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeletePreviewRequest() / DeletePreviewRequest.new instead')
  static DeletePreviewRequest create() => DeletePreviewRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeletePreviewRequest._();
  @$core.override
  DeletePreviewRequest createEmptyInstance() => DeletePreviewRequest._();
  @$core.pragma('dart2js:noInline')
  static DeletePreviewRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeletePreviewRequest>(
          DeletePreviewRequest.$_createMessage);
  static DeletePreviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

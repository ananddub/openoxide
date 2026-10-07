// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/tag.proto.

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

class Tag extends $pb.GeneratedMessage {
  factory Tag({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? color,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
  }) {
    final result = Tag._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (color != null) result.color = color;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  Tag._();

  factory Tag.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Tag()..mergeFromBuffer(data, registry);
  factory Tag.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Tag()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Tag',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Tag.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'color')
    ..aInt64(4, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(5, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Tag clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Tag copyWith(void Function(Tag) updates) =>
      super.copyWith((message) => updates(message as Tag)) as Tag;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Tag() / Tag.new instead')
  static Tag create() => Tag._();
  static $pb.GeneratedMessage $_createMessage() => Tag._();
  @$core.override
  Tag createEmptyInstance() => Tag._();
  @$core.pragma('dart2js:noInline')
  static Tag getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Tag>(Tag.$_createMessage);
  static Tag? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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

  /// Free-form hex color, e.g. "#6b7280".
  @$pb.TagNumber(3)
  $core.String get color => $_getSZ(2);
  @$pb.TagNumber(3)
  set color($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasColor() => $_has(2);
  @$pb.TagNumber(3)
  void clearColor() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get organizationId => $_getI64(3);
  @$pb.TagNumber(4)
  set organizationId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOrganizationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearOrganizationId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get createdAt => $_getI64(4);
  @$pb.TagNumber(5)
  set createdAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);
}

class ListTagsRequest extends $pb.GeneratedMessage {
  factory ListTagsRequest() => ListTagsRequest._();

  ListTagsRequest._();

  factory ListTagsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListTagsRequest()..mergeFromBuffer(data, registry);
  factory ListTagsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListTagsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListTagsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListTagsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListTagsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListTagsRequest copyWith(void Function(ListTagsRequest) updates) =>
      super.copyWith((message) => updates(message as ListTagsRequest))
          as ListTagsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ListTagsRequest() / ListTagsRequest.new instead')
  static ListTagsRequest create() => ListTagsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListTagsRequest._();
  @$core.override
  ListTagsRequest createEmptyInstance() => ListTagsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListTagsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListTagsRequest>(
          ListTagsRequest.$_createMessage);
  static ListTagsRequest? _defaultInstance;
}

class ListTagsResponse extends $pb.GeneratedMessage {
  factory ListTagsResponse({
    $core.Iterable<Tag>? tags,
  }) {
    final result = ListTagsResponse._();
    if (tags != null) result.tags.addAll(tags);
    return result;
  }

  ListTagsResponse._();

  factory ListTagsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListTagsResponse()..mergeFromBuffer(data, registry);
  factory ListTagsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListTagsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListTagsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListTagsResponse.$_createMessage)
    ..pPM<Tag>(1, _omitFieldNames ? '' : 'tags',
        subBuilder: Tag.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListTagsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListTagsResponse copyWith(void Function(ListTagsResponse) updates) =>
      super.copyWith((message) => updates(message as ListTagsResponse))
          as ListTagsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ListTagsResponse() / ListTagsResponse.new instead')
  static ListTagsResponse create() => ListTagsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListTagsResponse._();
  @$core.override
  ListTagsResponse createEmptyInstance() => ListTagsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListTagsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListTagsResponse>(
          ListTagsResponse.$_createMessage);
  static ListTagsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Tag> get tags => $_getList(0);
}

class GetTagRequest extends $pb.GeneratedMessage {
  factory GetTagRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetTagRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetTagRequest._();

  factory GetTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTagRequest()..mergeFromBuffer(data, registry);
  factory GetTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTagRequest copyWith(void Function(GetTagRequest) updates) =>
      super.copyWith((message) => updates(message as GetTagRequest))
          as GetTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetTagRequest() / GetTagRequest.new instead')
  static GetTagRequest create() => GetTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetTagRequest._();
  @$core.override
  GetTagRequest createEmptyInstance() => GetTagRequest._();
  @$core.pragma('dart2js:noInline')
  static GetTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTagRequest>(
          GetTagRequest.$_createMessage);
  static GetTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetTagResponse extends $pb.GeneratedMessage {
  factory GetTagResponse({
    Tag? tag,
  }) {
    final result = GetTagResponse._();
    if (tag != null) result.tag = tag;
    return result;
  }

  GetTagResponse._();

  factory GetTagResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTagResponse()..mergeFromBuffer(data, registry);
  factory GetTagResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTagResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetTagResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetTagResponse.$_createMessage)
    ..aOM<Tag>(1, _omitFieldNames ? '' : 'tag', subBuilder: Tag.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTagResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTagResponse copyWith(void Function(GetTagResponse) updates) =>
      super.copyWith((message) => updates(message as GetTagResponse))
          as GetTagResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetTagResponse() / GetTagResponse.new instead')
  static GetTagResponse create() => GetTagResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetTagResponse._();
  @$core.override
  GetTagResponse createEmptyInstance() => GetTagResponse._();
  @$core.pragma('dart2js:noInline')
  static GetTagResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTagResponse>(
          GetTagResponse.$_createMessage);
  static GetTagResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Tag get tag => $_getN(0);
  @$pb.TagNumber(1)
  set tag(Tag value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasTag() => $_has(0);
  @$pb.TagNumber(1)
  void clearTag() => $_clearField(1);
  @$pb.TagNumber(1)
  Tag ensureTag() => $_ensure(0);
}

class CreateTagRequest extends $pb.GeneratedMessage {
  factory CreateTagRequest({
    $core.String? name,
    $core.String? color,
    $fixnum.Int64? organizationId,
  }) {
    final result = CreateTagRequest._();
    if (name != null) result.name = name;
    if (color != null) result.color = color;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  CreateTagRequest._();

  factory CreateTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateTagRequest()..mergeFromBuffer(data, registry);
  factory CreateTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateTagRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'color')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateTagRequest copyWith(void Function(CreateTagRequest) updates) =>
      super.copyWith((message) => updates(message as CreateTagRequest))
          as CreateTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CreateTagRequest() / CreateTagRequest.new instead')
  static CreateTagRequest create() => CreateTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateTagRequest._();
  @$core.override
  CreateTagRequest createEmptyInstance() => CreateTagRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateTagRequest>(
          CreateTagRequest.$_createMessage);
  static CreateTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get color => $_getSZ(1);
  @$pb.TagNumber(2)
  set color($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasColor() => $_has(1);
  @$pb.TagNumber(2)
  void clearColor() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);
}

class CreateTagResponse extends $pb.GeneratedMessage {
  factory CreateTagResponse({
    Tag? tag,
  }) {
    final result = CreateTagResponse._();
    if (tag != null) result.tag = tag;
    return result;
  }

  CreateTagResponse._();

  factory CreateTagResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateTagResponse()..mergeFromBuffer(data, registry);
  factory CreateTagResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateTagResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateTagResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateTagResponse.$_createMessage)
    ..aOM<Tag>(1, _omitFieldNames ? '' : 'tag', subBuilder: Tag.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateTagResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateTagResponse copyWith(void Function(CreateTagResponse) updates) =>
      super.copyWith((message) => updates(message as CreateTagResponse))
          as CreateTagResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CreateTagResponse() / CreateTagResponse.new instead')
  static CreateTagResponse create() => CreateTagResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateTagResponse._();
  @$core.override
  CreateTagResponse createEmptyInstance() => CreateTagResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateTagResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateTagResponse>(
          CreateTagResponse.$_createMessage);
  static CreateTagResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Tag get tag => $_getN(0);
  @$pb.TagNumber(1)
  set tag(Tag value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasTag() => $_has(0);
  @$pb.TagNumber(1)
  void clearTag() => $_clearField(1);
  @$pb.TagNumber(1)
  Tag ensureTag() => $_ensure(0);
}

class UpdateTagRequest extends $pb.GeneratedMessage {
  factory UpdateTagRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? color,
  }) {
    final result = UpdateTagRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (color != null) result.color = color;
    return result;
  }

  UpdateTagRequest._();

  factory UpdateTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateTagRequest()..mergeFromBuffer(data, registry);
  factory UpdateTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'color')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateTagRequest copyWith(void Function(UpdateTagRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateTagRequest))
          as UpdateTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use UpdateTagRequest() / UpdateTagRequest.new instead')
  static UpdateTagRequest create() => UpdateTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateTagRequest._();
  @$core.override
  UpdateTagRequest createEmptyInstance() => UpdateTagRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateTagRequest>(
          UpdateTagRequest.$_createMessage);
  static UpdateTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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
  $core.String get color => $_getSZ(2);
  @$pb.TagNumber(3)
  set color($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasColor() => $_has(2);
  @$pb.TagNumber(3)
  void clearColor() => $_clearField(3);
}

class UpdateTagResponse extends $pb.GeneratedMessage {
  factory UpdateTagResponse({
    Tag? tag,
  }) {
    final result = UpdateTagResponse._();
    if (tag != null) result.tag = tag;
    return result;
  }

  UpdateTagResponse._();

  factory UpdateTagResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateTagResponse()..mergeFromBuffer(data, registry);
  factory UpdateTagResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateTagResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateTagResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateTagResponse.$_createMessage)
    ..aOM<Tag>(1, _omitFieldNames ? '' : 'tag', subBuilder: Tag.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateTagResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateTagResponse copyWith(void Function(UpdateTagResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateTagResponse))
          as UpdateTagResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use UpdateTagResponse() / UpdateTagResponse.new instead')
  static UpdateTagResponse create() => UpdateTagResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateTagResponse._();
  @$core.override
  UpdateTagResponse createEmptyInstance() => UpdateTagResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateTagResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateTagResponse>(
          UpdateTagResponse.$_createMessage);
  static UpdateTagResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Tag get tag => $_getN(0);
  @$pb.TagNumber(1)
  set tag(Tag value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasTag() => $_has(0);
  @$pb.TagNumber(1)
  void clearTag() => $_clearField(1);
  @$pb.TagNumber(1)
  Tag ensureTag() => $_ensure(0);
}

class DeleteTagRequest extends $pb.GeneratedMessage {
  factory DeleteTagRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteTagRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteTagRequest._();

  factory DeleteTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteTagRequest()..mergeFromBuffer(data, registry);
  factory DeleteTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteTagRequest copyWith(void Function(DeleteTagRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteTagRequest))
          as DeleteTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DeleteTagRequest() / DeleteTagRequest.new instead')
  static DeleteTagRequest create() => DeleteTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteTagRequest._();
  @$core.override
  DeleteTagRequest createEmptyInstance() => DeleteTagRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteTagRequest>(
          DeleteTagRequest.$_createMessage);
  static DeleteTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class AddTagRequest extends $pb.GeneratedMessage {
  factory AddTagRequest({
    $fixnum.Int64? id,
  }) {
    final result = AddTagRequest._();
    if (id != null) result.id = id;
    return result;
  }

  AddTagRequest._();

  factory AddTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AddTagRequest()..mergeFromBuffer(data, registry);
  factory AddTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AddTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AddTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddTagRequest copyWith(void Function(AddTagRequest) updates) =>
      super.copyWith((message) => updates(message as AddTagRequest))
          as AddTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AddTagRequest() / AddTagRequest.new instead')
  static AddTagRequest create() => AddTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => AddTagRequest._();
  @$core.override
  AddTagRequest createEmptyInstance() => AddTagRequest._();
  @$core.pragma('dart2js:noInline')
  static AddTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddTagRequest>(
          AddTagRequest.$_createMessage);
  static AddTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class AddTagResponse extends $pb.GeneratedMessage {
  factory AddTagResponse({
    $core.bool? success,
  }) {
    final result = AddTagResponse._();
    if (success != null) result.success = success;
    return result;
  }

  AddTagResponse._();

  factory AddTagResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AddTagResponse()..mergeFromBuffer(data, registry);
  factory AddTagResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AddTagResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddTagResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AddTagResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddTagResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddTagResponse copyWith(void Function(AddTagResponse) updates) =>
      super.copyWith((message) => updates(message as AddTagResponse))
          as AddTagResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AddTagResponse() / AddTagResponse.new instead')
  static AddTagResponse create() => AddTagResponse._();
  static $pb.GeneratedMessage $_createMessage() => AddTagResponse._();
  @$core.override
  AddTagResponse createEmptyInstance() => AddTagResponse._();
  @$core.pragma('dart2js:noInline')
  static AddTagResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddTagResponse>(
          AddTagResponse.$_createMessage);
  static AddTagResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

class RemoveTagRequest extends $pb.GeneratedMessage {
  factory RemoveTagRequest({
    $fixnum.Int64? id,
  }) {
    final result = RemoveTagRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RemoveTagRequest._();

  factory RemoveTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoveTagRequest()..mergeFromBuffer(data, registry);
  factory RemoveTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RemoveTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoveTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RemoveTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveTagRequest copyWith(void Function(RemoveTagRequest) updates) =>
      super.copyWith((message) => updates(message as RemoveTagRequest))
          as RemoveTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use RemoveTagRequest() / RemoveTagRequest.new instead')
  static RemoveTagRequest create() => RemoveTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => RemoveTagRequest._();
  @$core.override
  RemoveTagRequest createEmptyInstance() => RemoveTagRequest._();
  @$core.pragma('dart2js:noInline')
  static RemoveTagRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveTagRequest>(
          RemoveTagRequest.$_createMessage);
  static RemoveTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListProjectTagsRequest extends $pb.GeneratedMessage {
  factory ListProjectTagsRequest({
    $fixnum.Int64? projectId,
  }) {
    final result = ListProjectTagsRequest._();
    if (projectId != null) result.projectId = projectId;
    return result;
  }

  ListProjectTagsRequest._();

  factory ListProjectTagsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProjectTagsRequest()..mergeFromBuffer(data, registry);
  factory ListProjectTagsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProjectTagsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListProjectTagsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListProjectTagsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'projectId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProjectTagsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProjectTagsRequest copyWith(
          void Function(ListProjectTagsRequest) updates) =>
      super.copyWith((message) => updates(message as ListProjectTagsRequest))
          as ListProjectTagsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListProjectTagsRequest() / ListProjectTagsRequest.new instead')
  static ListProjectTagsRequest create() => ListProjectTagsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListProjectTagsRequest._();
  @$core.override
  ListProjectTagsRequest createEmptyInstance() => ListProjectTagsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListProjectTagsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListProjectTagsRequest>(
          ListProjectTagsRequest.$_createMessage);
  static ListProjectTagsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get projectId => $_getI64(0);
  @$pb.TagNumber(1)
  set projectId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectId() => $_clearField(1);
}

class ListProjectTagsResponse extends $pb.GeneratedMessage {
  factory ListProjectTagsResponse({
    $core.Iterable<Tag>? tags,
  }) {
    final result = ListProjectTagsResponse._();
    if (tags != null) result.tags.addAll(tags);
    return result;
  }

  ListProjectTagsResponse._();

  factory ListProjectTagsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProjectTagsResponse()..mergeFromBuffer(data, registry);
  factory ListProjectTagsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProjectTagsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListProjectTagsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListProjectTagsResponse.$_createMessage)
    ..pPM<Tag>(1, _omitFieldNames ? '' : 'tags',
        subBuilder: Tag.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProjectTagsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProjectTagsResponse copyWith(
          void Function(ListProjectTagsResponse) updates) =>
      super.copyWith((message) => updates(message as ListProjectTagsResponse))
          as ListProjectTagsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListProjectTagsResponse() / ListProjectTagsResponse.new instead')
  static ListProjectTagsResponse create() => ListProjectTagsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListProjectTagsResponse._();
  @$core.override
  ListProjectTagsResponse createEmptyInstance() => ListProjectTagsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListProjectTagsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListProjectTagsResponse>(
          ListProjectTagsResponse.$_createMessage);
  static ListProjectTagsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Tag> get tags => $_getList(0);
}

class AttachProjectTagRequest extends $pb.GeneratedMessage {
  factory AttachProjectTagRequest({
    $fixnum.Int64? projectId,
    $fixnum.Int64? tagId,
  }) {
    final result = AttachProjectTagRequest._();
    if (projectId != null) result.projectId = projectId;
    if (tagId != null) result.tagId = tagId;
    return result;
  }

  AttachProjectTagRequest._();

  factory AttachProjectTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AttachProjectTagRequest()..mergeFromBuffer(data, registry);
  factory AttachProjectTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AttachProjectTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AttachProjectTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AttachProjectTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'projectId')
    ..aInt64(2, _omitFieldNames ? '' : 'tagId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AttachProjectTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AttachProjectTagRequest copyWith(
          void Function(AttachProjectTagRequest) updates) =>
      super.copyWith((message) => updates(message as AttachProjectTagRequest))
          as AttachProjectTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AttachProjectTagRequest() / AttachProjectTagRequest.new instead')
  static AttachProjectTagRequest create() => AttachProjectTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => AttachProjectTagRequest._();
  @$core.override
  AttachProjectTagRequest createEmptyInstance() => AttachProjectTagRequest._();
  @$core.pragma('dart2js:noInline')
  static AttachProjectTagRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AttachProjectTagRequest>(
          AttachProjectTagRequest.$_createMessage);
  static AttachProjectTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get projectId => $_getI64(0);
  @$pb.TagNumber(1)
  set projectId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get tagId => $_getI64(1);
  @$pb.TagNumber(2)
  set tagId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTagId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTagId() => $_clearField(2);
}

class AttachProjectTagResponse extends $pb.GeneratedMessage {
  factory AttachProjectTagResponse({
    $core.bool? success,
  }) {
    final result = AttachProjectTagResponse._();
    if (success != null) result.success = success;
    return result;
  }

  AttachProjectTagResponse._();

  factory AttachProjectTagResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AttachProjectTagResponse()..mergeFromBuffer(data, registry);
  factory AttachProjectTagResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AttachProjectTagResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AttachProjectTagResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AttachProjectTagResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AttachProjectTagResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AttachProjectTagResponse copyWith(
          void Function(AttachProjectTagResponse) updates) =>
      super.copyWith((message) => updates(message as AttachProjectTagResponse))
          as AttachProjectTagResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AttachProjectTagResponse() / AttachProjectTagResponse.new instead')
  static AttachProjectTagResponse create() => AttachProjectTagResponse._();
  static $pb.GeneratedMessage $_createMessage() => AttachProjectTagResponse._();
  @$core.override
  AttachProjectTagResponse createEmptyInstance() =>
      AttachProjectTagResponse._();
  @$core.pragma('dart2js:noInline')
  static AttachProjectTagResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AttachProjectTagResponse>(
          AttachProjectTagResponse.$_createMessage);
  static AttachProjectTagResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

class DetachProjectTagRequest extends $pb.GeneratedMessage {
  factory DetachProjectTagRequest({
    $fixnum.Int64? projectId,
    $fixnum.Int64? tagId,
  }) {
    final result = DetachProjectTagRequest._();
    if (projectId != null) result.projectId = projectId;
    if (tagId != null) result.tagId = tagId;
    return result;
  }

  DetachProjectTagRequest._();

  factory DetachProjectTagRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DetachProjectTagRequest()..mergeFromBuffer(data, registry);
  factory DetachProjectTagRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DetachProjectTagRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DetachProjectTagRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DetachProjectTagRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'projectId')
    ..aInt64(2, _omitFieldNames ? '' : 'tagId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DetachProjectTagRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DetachProjectTagRequest copyWith(
          void Function(DetachProjectTagRequest) updates) =>
      super.copyWith((message) => updates(message as DetachProjectTagRequest))
          as DetachProjectTagRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DetachProjectTagRequest() / DetachProjectTagRequest.new instead')
  static DetachProjectTagRequest create() => DetachProjectTagRequest._();
  static $pb.GeneratedMessage $_createMessage() => DetachProjectTagRequest._();
  @$core.override
  DetachProjectTagRequest createEmptyInstance() => DetachProjectTagRequest._();
  @$core.pragma('dart2js:noInline')
  static DetachProjectTagRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DetachProjectTagRequest>(
          DetachProjectTagRequest.$_createMessage);
  static DetachProjectTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get projectId => $_getI64(0);
  @$pb.TagNumber(1)
  set projectId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get tagId => $_getI64(1);
  @$pb.TagNumber(2)
  set tagId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTagId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTagId() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

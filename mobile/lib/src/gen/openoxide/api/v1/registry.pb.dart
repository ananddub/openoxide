// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/registry.proto.

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

class Registry extends $pb.GeneratedMessage {
  factory Registry({
    $fixnum.Int64? id,
    $core.String? registryName,
    $core.String? imagePrefix,
    $core.String? username,
    $core.String? password,
    $core.String? registryUrl,
    $core.String? registryType,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Registry._();
    if (id != null) result.id = id;
    if (registryName != null) result.registryName = registryName;
    if (imagePrefix != null) result.imagePrefix = imagePrefix;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    if (registryUrl != null) result.registryUrl = registryUrl;
    if (registryType != null) result.registryType = registryType;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Registry._();

  factory Registry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Registry()..mergeFromBuffer(data, registry);
  factory Registry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Registry()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Registry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Registry.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'registryName')
    ..aOS(3, _omitFieldNames ? '' : 'imagePrefix')
    ..aOS(4, _omitFieldNames ? '' : 'username')
    ..aOS(5, _omitFieldNames ? '' : 'password')
    ..aOS(6, _omitFieldNames ? '' : 'registryUrl')
    ..aOS(7, _omitFieldNames ? '' : 'registryType')
    ..aInt64(8, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(9, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Registry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Registry copyWith(void Function(Registry) updates) =>
      super.copyWith((message) => updates(message as Registry)) as Registry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Registry() / Registry.new instead')
  static Registry create() => Registry._();
  static $pb.GeneratedMessage $_createMessage() => Registry._();
  @$core.override
  Registry createEmptyInstance() => Registry._();
  @$core.pragma('dart2js:noInline')
  static Registry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Registry>(Registry.$_createMessage);
  static Registry? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get registryName => $_getSZ(1);
  @$pb.TagNumber(2)
  set registryName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRegistryName() => $_has(1);
  @$pb.TagNumber(2)
  void clearRegistryName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get imagePrefix => $_getSZ(2);
  @$pb.TagNumber(3)
  set imagePrefix($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasImagePrefix() => $_has(2);
  @$pb.TagNumber(3)
  void clearImagePrefix() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get username => $_getSZ(3);
  @$pb.TagNumber(4)
  set username($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUsername() => $_has(3);
  @$pb.TagNumber(4)
  void clearUsername() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get password => $_getSZ(4);
  @$pb.TagNumber(5)
  set password($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPassword() => $_has(4);
  @$pb.TagNumber(5)
  void clearPassword() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get registryUrl => $_getSZ(5);
  @$pb.TagNumber(6)
  set registryUrl($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRegistryUrl() => $_has(5);
  @$pb.TagNumber(6)
  void clearRegistryUrl() => $_clearField(6);

  /// Free-form provider label (dockerhub, ghcr, custom, ...).
  @$pb.TagNumber(7)
  $core.String get registryType => $_getSZ(6);
  @$pb.TagNumber(7)
  set registryType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRegistryType() => $_has(6);
  @$pb.TagNumber(7)
  void clearRegistryType() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get createdAt => $_getI64(7);
  @$pb.TagNumber(8)
  set createdAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCreatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedAt() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get updatedAt => $_getI64(8);
  @$pb.TagNumber(9)
  set updatedAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUpdatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearUpdatedAt() => $_clearField(9);
}

class RegistryUsageReference extends $pb.GeneratedMessage {
  factory RegistryUsageReference({
    $core.String? resourceId,
    $core.String? resourceName,
    $core.String? resourceType,
    $core.String? usageKind,
  }) {
    final result = RegistryUsageReference._();
    if (resourceId != null) result.resourceId = resourceId;
    if (resourceName != null) result.resourceName = resourceName;
    if (resourceType != null) result.resourceType = resourceType;
    if (usageKind != null) result.usageKind = usageKind;
    return result;
  }

  RegistryUsageReference._();

  factory RegistryUsageReference.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RegistryUsageReference()..mergeFromBuffer(data, registry);
  factory RegistryUsageReference.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RegistryUsageReference()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RegistryUsageReference',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RegistryUsageReference.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'resourceId')
    ..aOS(2, _omitFieldNames ? '' : 'resourceName')
    ..aOS(3, _omitFieldNames ? '' : 'resourceType')
    ..aOS(4, _omitFieldNames ? '' : 'usageKind')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegistryUsageReference clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegistryUsageReference copyWith(
          void Function(RegistryUsageReference) updates) =>
      super.copyWith((message) => updates(message as RegistryUsageReference))
          as RegistryUsageReference;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RegistryUsageReference() / RegistryUsageReference.new instead')
  static RegistryUsageReference create() => RegistryUsageReference._();
  static $pb.GeneratedMessage $_createMessage() => RegistryUsageReference._();
  @$core.override
  RegistryUsageReference createEmptyInstance() => RegistryUsageReference._();
  @$core.pragma('dart2js:noInline')
  static RegistryUsageReference getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegistryUsageReference>(
          RegistryUsageReference.$_createMessage);
  static RegistryUsageReference? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get resourceId => $_getSZ(0);
  @$pb.TagNumber(1)
  set resourceId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasResourceId() => $_has(0);
  @$pb.TagNumber(1)
  void clearResourceId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get resourceName => $_getSZ(1);
  @$pb.TagNumber(2)
  set resourceName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasResourceName() => $_has(1);
  @$pb.TagNumber(2)
  void clearResourceName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get resourceType => $_getSZ(2);
  @$pb.TagNumber(3)
  set resourceType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasResourceType() => $_has(2);
  @$pb.TagNumber(3)
  void clearResourceType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get usageKind => $_getSZ(3);
  @$pb.TagNumber(4)
  set usageKind($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUsageKind() => $_has(3);
  @$pb.TagNumber(4)
  void clearUsageKind() => $_clearField(4);
}

class ListRegistriesRequest extends $pb.GeneratedMessage {
  factory ListRegistriesRequest() => ListRegistriesRequest._();

  ListRegistriesRequest._();

  factory ListRegistriesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistriesRequest()..mergeFromBuffer(data, registry);
  factory ListRegistriesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistriesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistriesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistriesRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistriesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistriesRequest copyWith(
          void Function(ListRegistriesRequest) updates) =>
      super.copyWith((message) => updates(message as ListRegistriesRequest))
          as ListRegistriesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistriesRequest() / ListRegistriesRequest.new instead')
  static ListRegistriesRequest create() => ListRegistriesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListRegistriesRequest._();
  @$core.override
  ListRegistriesRequest createEmptyInstance() => ListRegistriesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListRegistriesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistriesRequest>(
          ListRegistriesRequest.$_createMessage);
  static ListRegistriesRequest? _defaultInstance;
}

class ListRegistriesResponse extends $pb.GeneratedMessage {
  factory ListRegistriesResponse({
    $core.Iterable<Registry>? registries,
  }) {
    final result = ListRegistriesResponse._();
    if (registries != null) result.registries.addAll(registries);
    return result;
  }

  ListRegistriesResponse._();

  factory ListRegistriesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistriesResponse()..mergeFromBuffer(data, registry);
  factory ListRegistriesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistriesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistriesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistriesResponse.$_createMessage)
    ..pPM<Registry>(1, _omitFieldNames ? '' : 'registries',
        subBuilder: Registry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistriesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistriesResponse copyWith(
          void Function(ListRegistriesResponse) updates) =>
      super.copyWith((message) => updates(message as ListRegistriesResponse))
          as ListRegistriesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistriesResponse() / ListRegistriesResponse.new instead')
  static ListRegistriesResponse create() => ListRegistriesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListRegistriesResponse._();
  @$core.override
  ListRegistriesResponse createEmptyInstance() => ListRegistriesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListRegistriesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistriesResponse>(
          ListRegistriesResponse.$_createMessage);
  static ListRegistriesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Registry> get registries => $_getList(0);
}

class GetRegistryRequest extends $pb.GeneratedMessage {
  factory GetRegistryRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetRegistryRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetRegistryRequest._();

  factory GetRegistryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryRequest()..mergeFromBuffer(data, registry);
  factory GetRegistryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetRegistryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetRegistryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryRequest copyWith(void Function(GetRegistryRequest) updates) =>
      super.copyWith((message) => updates(message as GetRegistryRequest))
          as GetRegistryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetRegistryRequest() / GetRegistryRequest.new instead')
  static GetRegistryRequest create() => GetRegistryRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetRegistryRequest._();
  @$core.override
  GetRegistryRequest createEmptyInstance() => GetRegistryRequest._();
  @$core.pragma('dart2js:noInline')
  static GetRegistryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetRegistryRequest>(
          GetRegistryRequest.$_createMessage);
  static GetRegistryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetRegistryResponse extends $pb.GeneratedMessage {
  factory GetRegistryResponse({
    Registry? registry,
  }) {
    final result = GetRegistryResponse._();
    if (registry != null) result.registry = registry;
    return result;
  }

  GetRegistryResponse._();

  factory GetRegistryResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryResponse()..mergeFromBuffer(data, registry);
  factory GetRegistryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetRegistryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetRegistryResponse.$_createMessage)
    ..aOM<Registry>(1, _omitFieldNames ? '' : 'registry',
        subBuilder: Registry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryResponse copyWith(void Function(GetRegistryResponse) updates) =>
      super.copyWith((message) => updates(message as GetRegistryResponse))
          as GetRegistryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use GetRegistryResponse() / GetRegistryResponse.new instead')
  static GetRegistryResponse create() => GetRegistryResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetRegistryResponse._();
  @$core.override
  GetRegistryResponse createEmptyInstance() => GetRegistryResponse._();
  @$core.pragma('dart2js:noInline')
  static GetRegistryResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetRegistryResponse>(
          GetRegistryResponse.$_createMessage);
  static GetRegistryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Registry get registry => $_getN(0);
  @$pb.TagNumber(1)
  set registry(Registry value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistry() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistry() => $_clearField(1);
  @$pb.TagNumber(1)
  Registry ensureRegistry() => $_ensure(0);
}

class CreateRegistryRequest extends $pb.GeneratedMessage {
  factory CreateRegistryRequest({
    $core.String? registryName,
    $core.String? imagePrefix,
    $core.String? username,
    $core.String? password,
    $core.String? registryUrl,
    $core.String? registryType,
  }) {
    final result = CreateRegistryRequest._();
    if (registryName != null) result.registryName = registryName;
    if (imagePrefix != null) result.imagePrefix = imagePrefix;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    if (registryUrl != null) result.registryUrl = registryUrl;
    if (registryType != null) result.registryType = registryType;
    return result;
  }

  CreateRegistryRequest._();

  factory CreateRegistryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateRegistryRequest()..mergeFromBuffer(data, registry);
  factory CreateRegistryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateRegistryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateRegistryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateRegistryRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'registryName')
    ..aOS(2, _omitFieldNames ? '' : 'imagePrefix')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'password')
    ..aOS(5, _omitFieldNames ? '' : 'registryUrl')
    ..aOS(6, _omitFieldNames ? '' : 'registryType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateRegistryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateRegistryRequest copyWith(
          void Function(CreateRegistryRequest) updates) =>
      super.copyWith((message) => updates(message as CreateRegistryRequest))
          as CreateRegistryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateRegistryRequest() / CreateRegistryRequest.new instead')
  static CreateRegistryRequest create() => CreateRegistryRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateRegistryRequest._();
  @$core.override
  CreateRegistryRequest createEmptyInstance() => CreateRegistryRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateRegistryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateRegistryRequest>(
          CreateRegistryRequest.$_createMessage);
  static CreateRegistryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get registryName => $_getSZ(0);
  @$pb.TagNumber(1)
  set registryName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistryName() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistryName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get imagePrefix => $_getSZ(1);
  @$pb.TagNumber(2)
  set imagePrefix($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasImagePrefix() => $_has(1);
  @$pb.TagNumber(2)
  void clearImagePrefix() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get password => $_getSZ(3);
  @$pb.TagNumber(4)
  set password($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPassword() => $_has(3);
  @$pb.TagNumber(4)
  void clearPassword() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get registryUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set registryUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRegistryUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearRegistryUrl() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get registryType => $_getSZ(5);
  @$pb.TagNumber(6)
  set registryType($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRegistryType() => $_has(5);
  @$pb.TagNumber(6)
  void clearRegistryType() => $_clearField(6);
}

class CreateRegistryResponse extends $pb.GeneratedMessage {
  factory CreateRegistryResponse({
    Registry? registry,
  }) {
    final result = CreateRegistryResponse._();
    if (registry != null) result.registry = registry;
    return result;
  }

  CreateRegistryResponse._();

  factory CreateRegistryResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateRegistryResponse()..mergeFromBuffer(data, registry);
  factory CreateRegistryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateRegistryResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateRegistryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateRegistryResponse.$_createMessage)
    ..aOM<Registry>(1, _omitFieldNames ? '' : 'registry',
        subBuilder: Registry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateRegistryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateRegistryResponse copyWith(
          void Function(CreateRegistryResponse) updates) =>
      super.copyWith((message) => updates(message as CreateRegistryResponse))
          as CreateRegistryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateRegistryResponse() / CreateRegistryResponse.new instead')
  static CreateRegistryResponse create() => CreateRegistryResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateRegistryResponse._();
  @$core.override
  CreateRegistryResponse createEmptyInstance() => CreateRegistryResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateRegistryResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateRegistryResponse>(
          CreateRegistryResponse.$_createMessage);
  static CreateRegistryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Registry get registry => $_getN(0);
  @$pb.TagNumber(1)
  set registry(Registry value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistry() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistry() => $_clearField(1);
  @$pb.TagNumber(1)
  Registry ensureRegistry() => $_ensure(0);
}

/// An absent field keeps its stored value; `image_prefix` present-but-empty clears it.
class UpdateRegistryRequest extends $pb.GeneratedMessage {
  factory UpdateRegistryRequest({
    $fixnum.Int64? id,
    $core.String? registryName,
    $core.String? imagePrefix,
    $core.String? username,
    $core.String? password,
    $core.String? registryUrl,
    $core.String? registryType,
  }) {
    final result = UpdateRegistryRequest._();
    if (id != null) result.id = id;
    if (registryName != null) result.registryName = registryName;
    if (imagePrefix != null) result.imagePrefix = imagePrefix;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    if (registryUrl != null) result.registryUrl = registryUrl;
    if (registryType != null) result.registryType = registryType;
    return result;
  }

  UpdateRegistryRequest._();

  factory UpdateRegistryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateRegistryRequest()..mergeFromBuffer(data, registry);
  factory UpdateRegistryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateRegistryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateRegistryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateRegistryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'registryName')
    ..aOS(3, _omitFieldNames ? '' : 'imagePrefix')
    ..aOS(4, _omitFieldNames ? '' : 'username')
    ..aOS(5, _omitFieldNames ? '' : 'password')
    ..aOS(6, _omitFieldNames ? '' : 'registryUrl')
    ..aOS(7, _omitFieldNames ? '' : 'registryType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateRegistryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateRegistryRequest copyWith(
          void Function(UpdateRegistryRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateRegistryRequest))
          as UpdateRegistryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateRegistryRequest() / UpdateRegistryRequest.new instead')
  static UpdateRegistryRequest create() => UpdateRegistryRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateRegistryRequest._();
  @$core.override
  UpdateRegistryRequest createEmptyInstance() => UpdateRegistryRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateRegistryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateRegistryRequest>(
          UpdateRegistryRequest.$_createMessage);
  static UpdateRegistryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get registryName => $_getSZ(1);
  @$pb.TagNumber(2)
  set registryName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRegistryName() => $_has(1);
  @$pb.TagNumber(2)
  void clearRegistryName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get imagePrefix => $_getSZ(2);
  @$pb.TagNumber(3)
  set imagePrefix($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasImagePrefix() => $_has(2);
  @$pb.TagNumber(3)
  void clearImagePrefix() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get username => $_getSZ(3);
  @$pb.TagNumber(4)
  set username($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUsername() => $_has(3);
  @$pb.TagNumber(4)
  void clearUsername() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get password => $_getSZ(4);
  @$pb.TagNumber(5)
  set password($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPassword() => $_has(4);
  @$pb.TagNumber(5)
  void clearPassword() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get registryUrl => $_getSZ(5);
  @$pb.TagNumber(6)
  set registryUrl($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRegistryUrl() => $_has(5);
  @$pb.TagNumber(6)
  void clearRegistryUrl() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get registryType => $_getSZ(6);
  @$pb.TagNumber(7)
  set registryType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRegistryType() => $_has(6);
  @$pb.TagNumber(7)
  void clearRegistryType() => $_clearField(7);
}

class UpdateRegistryResponse extends $pb.GeneratedMessage {
  factory UpdateRegistryResponse({
    Registry? registry,
  }) {
    final result = UpdateRegistryResponse._();
    if (registry != null) result.registry = registry;
    return result;
  }

  UpdateRegistryResponse._();

  factory UpdateRegistryResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateRegistryResponse()..mergeFromBuffer(data, registry);
  factory UpdateRegistryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateRegistryResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateRegistryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateRegistryResponse.$_createMessage)
    ..aOM<Registry>(1, _omitFieldNames ? '' : 'registry',
        subBuilder: Registry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateRegistryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateRegistryResponse copyWith(
          void Function(UpdateRegistryResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateRegistryResponse))
          as UpdateRegistryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateRegistryResponse() / UpdateRegistryResponse.new instead')
  static UpdateRegistryResponse create() => UpdateRegistryResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateRegistryResponse._();
  @$core.override
  UpdateRegistryResponse createEmptyInstance() => UpdateRegistryResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateRegistryResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateRegistryResponse>(
          UpdateRegistryResponse.$_createMessage);
  static UpdateRegistryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Registry get registry => $_getN(0);
  @$pb.TagNumber(1)
  set registry(Registry value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistry() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistry() => $_clearField(1);
  @$pb.TagNumber(1)
  Registry ensureRegistry() => $_ensure(0);
}

class DeleteRegistryRequest extends $pb.GeneratedMessage {
  factory DeleteRegistryRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteRegistryRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteRegistryRequest._();

  factory DeleteRegistryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteRegistryRequest()..mergeFromBuffer(data, registry);
  factory DeleteRegistryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteRegistryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteRegistryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteRegistryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteRegistryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteRegistryRequest copyWith(
          void Function(DeleteRegistryRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteRegistryRequest))
          as DeleteRegistryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteRegistryRequest() / DeleteRegistryRequest.new instead')
  static DeleteRegistryRequest create() => DeleteRegistryRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteRegistryRequest._();
  @$core.override
  DeleteRegistryRequest createEmptyInstance() => DeleteRegistryRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteRegistryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteRegistryRequest>(
          DeleteRegistryRequest.$_createMessage);
  static DeleteRegistryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// With `id` set, the saved registry's stored credentials are tested; otherwise the raw url/username/password are used (create dialog).
class TestRegistryRequest extends $pb.GeneratedMessage {
  factory TestRegistryRequest({
    $fixnum.Int64? id,
    $core.String? registryUrl,
    $core.String? username,
    $core.String? password,
  }) {
    final result = TestRegistryRequest._();
    if (id != null) result.id = id;
    if (registryUrl != null) result.registryUrl = registryUrl;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    return result;
  }

  TestRegistryRequest._();

  factory TestRegistryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestRegistryRequest()..mergeFromBuffer(data, registry);
  factory TestRegistryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestRegistryRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestRegistryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestRegistryRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'registryUrl')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestRegistryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestRegistryRequest copyWith(void Function(TestRegistryRequest) updates) =>
      super.copyWith((message) => updates(message as TestRegistryRequest))
          as TestRegistryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use TestRegistryRequest() / TestRegistryRequest.new instead')
  static TestRegistryRequest create() => TestRegistryRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestRegistryRequest._();
  @$core.override
  TestRegistryRequest createEmptyInstance() => TestRegistryRequest._();
  @$core.pragma('dart2js:noInline')
  static TestRegistryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestRegistryRequest>(
          TestRegistryRequest.$_createMessage);
  static TestRegistryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get registryUrl => $_getSZ(1);
  @$pb.TagNumber(2)
  set registryUrl($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRegistryUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearRegistryUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get password => $_getSZ(3);
  @$pb.TagNumber(4)
  set password($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPassword() => $_has(3);
  @$pb.TagNumber(4)
  void clearPassword() => $_clearField(4);
}

class TestRegistryResponse extends $pb.GeneratedMessage {
  factory TestRegistryResponse({
    $core.bool? success,
  }) {
    final result = TestRegistryResponse._();
    if (success != null) result.success = success;
    return result;
  }

  TestRegistryResponse._();

  factory TestRegistryResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestRegistryResponse()..mergeFromBuffer(data, registry);
  factory TestRegistryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestRegistryResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestRegistryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestRegistryResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestRegistryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestRegistryResponse copyWith(void Function(TestRegistryResponse) updates) =>
      super.copyWith((message) => updates(message as TestRegistryResponse))
          as TestRegistryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestRegistryResponse() / TestRegistryResponse.new instead')
  static TestRegistryResponse create() => TestRegistryResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestRegistryResponse._();
  @$core.override
  TestRegistryResponse createEmptyInstance() => TestRegistryResponse._();
  @$core.pragma('dart2js:noInline')
  static TestRegistryResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestRegistryResponse>(
          TestRegistryResponse.$_createMessage);
  static TestRegistryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

class ListRegistryRepositoriesRequest extends $pb.GeneratedMessage {
  factory ListRegistryRepositoriesRequest({
    $fixnum.Int64? registryId,
  }) {
    final result = ListRegistryRepositoriesRequest._();
    if (registryId != null) result.registryId = registryId;
    return result;
  }

  ListRegistryRepositoriesRequest._();

  factory ListRegistryRepositoriesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryRepositoriesRequest()..mergeFromBuffer(data, registry);
  factory ListRegistryRepositoriesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryRepositoriesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistryRepositoriesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistryRepositoriesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'registryId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryRepositoriesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryRepositoriesRequest copyWith(
          void Function(ListRegistryRepositoriesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListRegistryRepositoriesRequest))
          as ListRegistryRepositoriesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistryRepositoriesRequest() / ListRegistryRepositoriesRequest.new instead')
  static ListRegistryRepositoriesRequest create() =>
      ListRegistryRepositoriesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListRegistryRepositoriesRequest._();
  @$core.override
  ListRegistryRepositoriesRequest createEmptyInstance() =>
      ListRegistryRepositoriesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListRegistryRepositoriesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistryRepositoriesRequest>(
          ListRegistryRepositoriesRequest.$_createMessage);
  static ListRegistryRepositoriesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get registryId => $_getI64(0);
  @$pb.TagNumber(1)
  set registryId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistryId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistryId() => $_clearField(1);
}

class ListRegistryRepositoriesResponse extends $pb.GeneratedMessage {
  factory ListRegistryRepositoriesResponse({
    $core.Iterable<$core.String>? repositories,
  }) {
    final result = ListRegistryRepositoriesResponse._();
    if (repositories != null) result.repositories.addAll(repositories);
    return result;
  }

  ListRegistryRepositoriesResponse._();

  factory ListRegistryRepositoriesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryRepositoriesResponse()..mergeFromBuffer(data, registry);
  factory ListRegistryRepositoriesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryRepositoriesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistryRepositoriesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistryRepositoriesResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'repositories')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryRepositoriesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryRepositoriesResponse copyWith(
          void Function(ListRegistryRepositoriesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListRegistryRepositoriesResponse))
          as ListRegistryRepositoriesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistryRepositoriesResponse() / ListRegistryRepositoriesResponse.new instead')
  static ListRegistryRepositoriesResponse create() =>
      ListRegistryRepositoriesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListRegistryRepositoriesResponse._();
  @$core.override
  ListRegistryRepositoriesResponse createEmptyInstance() =>
      ListRegistryRepositoriesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListRegistryRepositoriesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistryRepositoriesResponse>(
          ListRegistryRepositoriesResponse.$_createMessage);
  static ListRegistryRepositoriesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get repositories => $_getList(0);
}

class ListRegistryTagsRequest extends $pb.GeneratedMessage {
  factory ListRegistryTagsRequest({
    $fixnum.Int64? registryId,
    $core.String? repository,
  }) {
    final result = ListRegistryTagsRequest._();
    if (registryId != null) result.registryId = registryId;
    if (repository != null) result.repository = repository;
    return result;
  }

  ListRegistryTagsRequest._();

  factory ListRegistryTagsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryTagsRequest()..mergeFromBuffer(data, registry);
  factory ListRegistryTagsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryTagsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistryTagsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistryTagsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'registryId')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryTagsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryTagsRequest copyWith(
          void Function(ListRegistryTagsRequest) updates) =>
      super.copyWith((message) => updates(message as ListRegistryTagsRequest))
          as ListRegistryTagsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistryTagsRequest() / ListRegistryTagsRequest.new instead')
  static ListRegistryTagsRequest create() => ListRegistryTagsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListRegistryTagsRequest._();
  @$core.override
  ListRegistryTagsRequest createEmptyInstance() => ListRegistryTagsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListRegistryTagsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistryTagsRequest>(
          ListRegistryTagsRequest.$_createMessage);
  static ListRegistryTagsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get registryId => $_getI64(0);
  @$pb.TagNumber(1)
  set registryId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistryId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistryId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);
}

class ListRegistryTagsResponse extends $pb.GeneratedMessage {
  factory ListRegistryTagsResponse({
    $core.String? repository,
    $core.Iterable<$core.String>? tags,
  }) {
    final result = ListRegistryTagsResponse._();
    if (repository != null) result.repository = repository;
    if (tags != null) result.tags.addAll(tags);
    return result;
  }

  ListRegistryTagsResponse._();

  factory ListRegistryTagsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryTagsResponse()..mergeFromBuffer(data, registry);
  factory ListRegistryTagsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRegistryTagsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRegistryTagsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRegistryTagsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'repository')
    ..pPS(2, _omitFieldNames ? '' : 'tags')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryTagsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRegistryTagsResponse copyWith(
          void Function(ListRegistryTagsResponse) updates) =>
      super.copyWith((message) => updates(message as ListRegistryTagsResponse))
          as ListRegistryTagsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRegistryTagsResponse() / ListRegistryTagsResponse.new instead')
  static ListRegistryTagsResponse create() => ListRegistryTagsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListRegistryTagsResponse._();
  @$core.override
  ListRegistryTagsResponse createEmptyInstance() =>
      ListRegistryTagsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListRegistryTagsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRegistryTagsResponse>(
          ListRegistryTagsResponse.$_createMessage);
  static ListRegistryTagsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get repository => $_getSZ(0);
  @$pb.TagNumber(1)
  set repository($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRepository() => $_has(0);
  @$pb.TagNumber(1)
  void clearRepository() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get tags => $_getList(1);
}

class RotateRegistryCredentialsRequest extends $pb.GeneratedMessage {
  factory RotateRegistryCredentialsRequest({
    $fixnum.Int64? registryId,
    $core.String? username,
    $core.String? password,
  }) {
    final result = RotateRegistryCredentialsRequest._();
    if (registryId != null) result.registryId = registryId;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    return result;
  }

  RotateRegistryCredentialsRequest._();

  factory RotateRegistryCredentialsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RotateRegistryCredentialsRequest()..mergeFromBuffer(data, registry);
  factory RotateRegistryCredentialsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RotateRegistryCredentialsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RotateRegistryCredentialsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RotateRegistryCredentialsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'registryId')
    ..aOS(2, _omitFieldNames ? '' : 'username')
    ..aOS(3, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RotateRegistryCredentialsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RotateRegistryCredentialsRequest copyWith(
          void Function(RotateRegistryCredentialsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RotateRegistryCredentialsRequest))
          as RotateRegistryCredentialsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RotateRegistryCredentialsRequest() / RotateRegistryCredentialsRequest.new instead')
  static RotateRegistryCredentialsRequest create() =>
      RotateRegistryCredentialsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RotateRegistryCredentialsRequest._();
  @$core.override
  RotateRegistryCredentialsRequest createEmptyInstance() =>
      RotateRegistryCredentialsRequest._();
  @$core.pragma('dart2js:noInline')
  static RotateRegistryCredentialsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RotateRegistryCredentialsRequest>(
          RotateRegistryCredentialsRequest.$_createMessage);
  static RotateRegistryCredentialsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get registryId => $_getI64(0);
  @$pb.TagNumber(1)
  set registryId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistryId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistryId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get username => $_getSZ(1);
  @$pb.TagNumber(2)
  set username($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUsername() => $_has(1);
  @$pb.TagNumber(2)
  void clearUsername() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get password => $_getSZ(2);
  @$pb.TagNumber(3)
  set password($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPassword() => $_has(2);
  @$pb.TagNumber(3)
  void clearPassword() => $_clearField(3);
}

class RotateRegistryCredentialsResponse extends $pb.GeneratedMessage {
  factory RotateRegistryCredentialsResponse({
    Registry? registry,
  }) {
    final result = RotateRegistryCredentialsResponse._();
    if (registry != null) result.registry = registry;
    return result;
  }

  RotateRegistryCredentialsResponse._();

  factory RotateRegistryCredentialsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RotateRegistryCredentialsResponse()..mergeFromBuffer(data, registry);
  factory RotateRegistryCredentialsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RotateRegistryCredentialsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RotateRegistryCredentialsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RotateRegistryCredentialsResponse.$_createMessage)
    ..aOM<Registry>(1, _omitFieldNames ? '' : 'registry',
        subBuilder: Registry.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RotateRegistryCredentialsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RotateRegistryCredentialsResponse copyWith(
          void Function(RotateRegistryCredentialsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RotateRegistryCredentialsResponse))
          as RotateRegistryCredentialsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RotateRegistryCredentialsResponse() / RotateRegistryCredentialsResponse.new instead')
  static RotateRegistryCredentialsResponse create() =>
      RotateRegistryCredentialsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      RotateRegistryCredentialsResponse._();
  @$core.override
  RotateRegistryCredentialsResponse createEmptyInstance() =>
      RotateRegistryCredentialsResponse._();
  @$core.pragma('dart2js:noInline')
  static RotateRegistryCredentialsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RotateRegistryCredentialsResponse>(
          RotateRegistryCredentialsResponse.$_createMessage);
  static RotateRegistryCredentialsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Registry get registry => $_getN(0);
  @$pb.TagNumber(1)
  set registry(Registry value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistry() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistry() => $_clearField(1);
  @$pb.TagNumber(1)
  Registry ensureRegistry() => $_ensure(0);
}

class GetRegistryUsageRequest extends $pb.GeneratedMessage {
  factory GetRegistryUsageRequest({
    $fixnum.Int64? registryId,
  }) {
    final result = GetRegistryUsageRequest._();
    if (registryId != null) result.registryId = registryId;
    return result;
  }

  GetRegistryUsageRequest._();

  factory GetRegistryUsageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryUsageRequest()..mergeFromBuffer(data, registry);
  factory GetRegistryUsageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryUsageRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetRegistryUsageRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetRegistryUsageRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'registryId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryUsageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryUsageRequest copyWith(
          void Function(GetRegistryUsageRequest) updates) =>
      super.copyWith((message) => updates(message as GetRegistryUsageRequest))
          as GetRegistryUsageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetRegistryUsageRequest() / GetRegistryUsageRequest.new instead')
  static GetRegistryUsageRequest create() => GetRegistryUsageRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetRegistryUsageRequest._();
  @$core.override
  GetRegistryUsageRequest createEmptyInstance() => GetRegistryUsageRequest._();
  @$core.pragma('dart2js:noInline')
  static GetRegistryUsageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetRegistryUsageRequest>(
          GetRegistryUsageRequest.$_createMessage);
  static GetRegistryUsageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get registryId => $_getI64(0);
  @$pb.TagNumber(1)
  set registryId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRegistryId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRegistryId() => $_clearField(1);
}

class GetRegistryUsageResponse extends $pb.GeneratedMessage {
  factory GetRegistryUsageResponse({
    $core.Iterable<RegistryUsageReference>? usage,
  }) {
    final result = GetRegistryUsageResponse._();
    if (usage != null) result.usage.addAll(usage);
    return result;
  }

  GetRegistryUsageResponse._();

  factory GetRegistryUsageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryUsageResponse()..mergeFromBuffer(data, registry);
  factory GetRegistryUsageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetRegistryUsageResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetRegistryUsageResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetRegistryUsageResponse.$_createMessage)
    ..pPM<RegistryUsageReference>(1, _omitFieldNames ? '' : 'usage',
        subBuilder: RegistryUsageReference.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryUsageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetRegistryUsageResponse copyWith(
          void Function(GetRegistryUsageResponse) updates) =>
      super.copyWith((message) => updates(message as GetRegistryUsageResponse))
          as GetRegistryUsageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetRegistryUsageResponse() / GetRegistryUsageResponse.new instead')
  static GetRegistryUsageResponse create() => GetRegistryUsageResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetRegistryUsageResponse._();
  @$core.override
  GetRegistryUsageResponse createEmptyInstance() =>
      GetRegistryUsageResponse._();
  @$core.pragma('dart2js:noInline')
  static GetRegistryUsageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetRegistryUsageResponse>(
          GetRegistryUsageResponse.$_createMessage);
  static GetRegistryUsageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RegistryUsageReference> get usage => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

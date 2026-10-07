// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/vault.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'vault.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'vault.pbenum.dart';

class VaultProvider extends $pb.GeneratedMessage {
  factory VaultProvider({
    $fixnum.Int64? id,
    $core.String? name,
    VaultProviderType? providerType,
    $core.String? apiUrl,
    $core.String? authToken,
    $core.String? namespace,
    $core.String? configJson,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = VaultProvider._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (authToken != null) result.authToken = authToken;
    if (namespace != null) result.namespace = namespace;
    if (configJson != null) result.configJson = configJson;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  VaultProvider._();

  factory VaultProvider.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultProvider()..mergeFromBuffer(data, registry);
  factory VaultProvider.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultProvider()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'VaultProvider',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: VaultProvider.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<VaultProviderType>(3, _omitFieldNames ? '' : 'providerType',
        enumValues: VaultProviderType.values)
    ..aOS(4, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(5, _omitFieldNames ? '' : 'authToken')
    ..aOS(6, _omitFieldNames ? '' : 'namespace')
    ..aOS(7, _omitFieldNames ? '' : 'configJson')
    ..aInt64(8, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..pPM<VaultProviderAssignment>(11, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultProvider clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultProvider copyWith(void Function(VaultProvider) updates) =>
      super.copyWith((message) => updates(message as VaultProvider))
          as VaultProvider;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use VaultProvider() / VaultProvider.new instead')
  static VaultProvider create() => VaultProvider._();
  static $pb.GeneratedMessage $_createMessage() => VaultProvider._();
  @$core.override
  VaultProvider createEmptyInstance() => VaultProvider._();
  @$core.pragma('dart2js:noInline')
  static VaultProvider getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VaultProvider>(
          VaultProvider.$_createMessage);
  static VaultProvider? _defaultInstance;

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
  VaultProviderType get providerType => $_getN(2);
  @$pb.TagNumber(3)
  set providerType(VaultProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProviderType() => $_has(2);
  @$pb.TagNumber(3)
  void clearProviderType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get apiUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set apiUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasApiUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearApiUrl() => $_clearField(4);

  /// Masked server-side once a token is stored; never the real secret.
  @$pb.TagNumber(5)
  $core.String get authToken => $_getSZ(4);
  @$pb.TagNumber(5)
  set authToken($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAuthToken() => $_has(4);
  @$pb.TagNumber(5)
  void clearAuthToken() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get namespace => $_getSZ(5);
  @$pb.TagNumber(6)
  set namespace($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasNamespace() => $_has(5);
  @$pb.TagNumber(6)
  void clearNamespace() => $_clearField(6);

  /// Provider-specific settings document (also carries `_assignments`).
  @$pb.TagNumber(7)
  $core.String get configJson => $_getSZ(6);
  @$pb.TagNumber(7)
  set configJson($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasConfigJson() => $_has(6);
  @$pb.TagNumber(7)
  void clearConfigJson() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get organizationId => $_getI64(7);
  @$pb.TagNumber(8)
  set organizationId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOrganizationId() => $_has(7);
  @$pb.TagNumber(8)
  void clearOrganizationId() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get createdAt => $_getI64(8);
  @$pb.TagNumber(9)
  set createdAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCreatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearCreatedAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get updatedAt => $_getI64(9);
  @$pb.TagNumber(10)
  set updatedAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUpdatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearUpdatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(10);
}

class VaultProviderAssignment extends $pb.GeneratedMessage {
  factory VaultProviderAssignment({
    $fixnum.Int64? projectId,
    $core.Iterable<$fixnum.Int64>? environmentIds,
  }) {
    final result = VaultProviderAssignment._();
    if (projectId != null) result.projectId = projectId;
    if (environmentIds != null) result.environmentIds.addAll(environmentIds);
    return result;
  }

  VaultProviderAssignment._();

  factory VaultProviderAssignment.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultProviderAssignment()..mergeFromBuffer(data, registry);
  factory VaultProviderAssignment.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultProviderAssignment()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'VaultProviderAssignment',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: VaultProviderAssignment.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'projectId')
    ..p<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'environmentIds', $pb.PbFieldType.K6)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultProviderAssignment clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultProviderAssignment copyWith(
          void Function(VaultProviderAssignment) updates) =>
      super.copyWith((message) => updates(message as VaultProviderAssignment))
          as VaultProviderAssignment;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use VaultProviderAssignment() / VaultProviderAssignment.new instead')
  static VaultProviderAssignment create() => VaultProviderAssignment._();
  static $pb.GeneratedMessage $_createMessage() => VaultProviderAssignment._();
  @$core.override
  VaultProviderAssignment createEmptyInstance() => VaultProviderAssignment._();
  @$core.pragma('dart2js:noInline')
  static VaultProviderAssignment getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<VaultProviderAssignment>(
          VaultProviderAssignment.$_createMessage);
  static VaultProviderAssignment? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get projectId => $_getI64(0);
  @$pb.TagNumber(1)
  set projectId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$fixnum.Int64> get environmentIds => $_getList(1);
}

class ListVaultProvidersRequest extends $pb.GeneratedMessage {
  factory ListVaultProvidersRequest() => ListVaultProvidersRequest._();

  ListVaultProvidersRequest._();

  factory ListVaultProvidersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultProvidersRequest()..mergeFromBuffer(data, registry);
  factory ListVaultProvidersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultProvidersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultProvidersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultProvidersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultProvidersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultProvidersRequest copyWith(
          void Function(ListVaultProvidersRequest) updates) =>
      super.copyWith((message) => updates(message as ListVaultProvidersRequest))
          as ListVaultProvidersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultProvidersRequest() / ListVaultProvidersRequest.new instead')
  static ListVaultProvidersRequest create() => ListVaultProvidersRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListVaultProvidersRequest._();
  @$core.override
  ListVaultProvidersRequest createEmptyInstance() =>
      ListVaultProvidersRequest._();
  @$core.pragma('dart2js:noInline')
  static ListVaultProvidersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultProvidersRequest>(
          ListVaultProvidersRequest.$_createMessage);
  static ListVaultProvidersRequest? _defaultInstance;
}

class ListVaultProvidersResponse extends $pb.GeneratedMessage {
  factory ListVaultProvidersResponse({
    $core.Iterable<VaultProvider>? providers,
  }) {
    final result = ListVaultProvidersResponse._();
    if (providers != null) result.providers.addAll(providers);
    return result;
  }

  ListVaultProvidersResponse._();

  factory ListVaultProvidersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultProvidersResponse()..mergeFromBuffer(data, registry);
  factory ListVaultProvidersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultProvidersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultProvidersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultProvidersResponse.$_createMessage)
    ..pPM<VaultProvider>(1, _omitFieldNames ? '' : 'providers',
        subBuilder: VaultProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultProvidersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultProvidersResponse copyWith(
          void Function(ListVaultProvidersResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListVaultProvidersResponse))
          as ListVaultProvidersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultProvidersResponse() / ListVaultProvidersResponse.new instead')
  static ListVaultProvidersResponse create() => ListVaultProvidersResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListVaultProvidersResponse._();
  @$core.override
  ListVaultProvidersResponse createEmptyInstance() =>
      ListVaultProvidersResponse._();
  @$core.pragma('dart2js:noInline')
  static ListVaultProvidersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultProvidersResponse>(
          ListVaultProvidersResponse.$_createMessage);
  static ListVaultProvidersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<VaultProvider> get providers => $_getList(0);
}

class GetVaultProviderRequest extends $pb.GeneratedMessage {
  factory GetVaultProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetVaultProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetVaultProviderRequest._();

  factory GetVaultProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetVaultProviderRequest()..mergeFromBuffer(data, registry);
  factory GetVaultProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetVaultProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetVaultProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetVaultProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetVaultProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetVaultProviderRequest copyWith(
          void Function(GetVaultProviderRequest) updates) =>
      super.copyWith((message) => updates(message as GetVaultProviderRequest))
          as GetVaultProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetVaultProviderRequest() / GetVaultProviderRequest.new instead')
  static GetVaultProviderRequest create() => GetVaultProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetVaultProviderRequest._();
  @$core.override
  GetVaultProviderRequest createEmptyInstance() => GetVaultProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static GetVaultProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetVaultProviderRequest>(
          GetVaultProviderRequest.$_createMessage);
  static GetVaultProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetVaultProviderResponse extends $pb.GeneratedMessage {
  factory GetVaultProviderResponse({
    VaultProvider? provider,
  }) {
    final result = GetVaultProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  GetVaultProviderResponse._();

  factory GetVaultProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetVaultProviderResponse()..mergeFromBuffer(data, registry);
  factory GetVaultProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetVaultProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetVaultProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetVaultProviderResponse.$_createMessage)
    ..aOM<VaultProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: VaultProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetVaultProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetVaultProviderResponse copyWith(
          void Function(GetVaultProviderResponse) updates) =>
      super.copyWith((message) => updates(message as GetVaultProviderResponse))
          as GetVaultProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetVaultProviderResponse() / GetVaultProviderResponse.new instead')
  static GetVaultProviderResponse create() => GetVaultProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetVaultProviderResponse._();
  @$core.override
  GetVaultProviderResponse createEmptyInstance() =>
      GetVaultProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static GetVaultProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetVaultProviderResponse>(
          GetVaultProviderResponse.$_createMessage);
  static GetVaultProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  VaultProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(VaultProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  VaultProvider ensureProvider() => $_ensure(0);
}

class CreateVaultProviderRequest extends $pb.GeneratedMessage {
  factory CreateVaultProviderRequest({
    $core.String? name,
    VaultProviderType? providerType,
    $core.String? apiUrl,
    $core.String? authToken,
    $core.String? namespace,
    $core.String? configJson,
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = CreateVaultProviderRequest._();
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (authToken != null) result.authToken = authToken;
    if (namespace != null) result.namespace = namespace;
    if (configJson != null) result.configJson = configJson;
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  CreateVaultProviderRequest._();

  factory CreateVaultProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateVaultProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateVaultProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateVaultProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateVaultProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateVaultProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aE<VaultProviderType>(2, _omitFieldNames ? '' : 'providerType',
        enumValues: VaultProviderType.values)
    ..aOS(3, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(4, _omitFieldNames ? '' : 'authToken')
    ..aOS(5, _omitFieldNames ? '' : 'namespace')
    ..aOS(6, _omitFieldNames ? '' : 'configJson')
    ..pPM<VaultProviderAssignment>(7, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateVaultProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateVaultProviderRequest copyWith(
          void Function(CreateVaultProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateVaultProviderRequest))
          as CreateVaultProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateVaultProviderRequest() / CreateVaultProviderRequest.new instead')
  static CreateVaultProviderRequest create() => CreateVaultProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateVaultProviderRequest._();
  @$core.override
  CreateVaultProviderRequest createEmptyInstance() =>
      CreateVaultProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateVaultProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateVaultProviderRequest>(
          CreateVaultProviderRequest.$_createMessage);
  static CreateVaultProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  VaultProviderType get providerType => $_getN(1);
  @$pb.TagNumber(2)
  set providerType(VaultProviderType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasProviderType() => $_has(1);
  @$pb.TagNumber(2)
  void clearProviderType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get apiUrl => $_getSZ(2);
  @$pb.TagNumber(3)
  set apiUrl($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasApiUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearApiUrl() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get authToken => $_getSZ(3);
  @$pb.TagNumber(4)
  set authToken($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAuthToken() => $_has(3);
  @$pb.TagNumber(4)
  void clearAuthToken() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get namespace => $_getSZ(4);
  @$pb.TagNumber(5)
  set namespace($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasNamespace() => $_has(4);
  @$pb.TagNumber(5)
  void clearNamespace() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get configJson => $_getSZ(5);
  @$pb.TagNumber(6)
  set configJson($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasConfigJson() => $_has(5);
  @$pb.TagNumber(6)
  void clearConfigJson() => $_clearField(6);

  @$pb.TagNumber(7)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(6);
}

class CreateVaultProviderResponse extends $pb.GeneratedMessage {
  factory CreateVaultProviderResponse({
    VaultProvider? provider,
  }) {
    final result = CreateVaultProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  CreateVaultProviderResponse._();

  factory CreateVaultProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateVaultProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateVaultProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateVaultProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateVaultProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateVaultProviderResponse.$_createMessage)
    ..aOM<VaultProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: VaultProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateVaultProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateVaultProviderResponse copyWith(
          void Function(CreateVaultProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateVaultProviderResponse))
          as CreateVaultProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateVaultProviderResponse() / CreateVaultProviderResponse.new instead')
  static CreateVaultProviderResponse create() =>
      CreateVaultProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateVaultProviderResponse._();
  @$core.override
  CreateVaultProviderResponse createEmptyInstance() =>
      CreateVaultProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateVaultProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateVaultProviderResponse>(
          CreateVaultProviderResponse.$_createMessage);
  static CreateVaultProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  VaultProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(VaultProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  VaultProvider ensureProvider() => $_ensure(0);
}

/// provider_type is immutable after creation; an absent auth_token keeps the stored token, a present-but-empty one clears it; `assignments` must be a full replacement list, absent means "leave unchanged".
class UpdateVaultProviderRequest extends $pb.GeneratedMessage {
  factory UpdateVaultProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? apiUrl,
    $core.String? authToken,
    $core.String? namespace,
    $core.String? configJson,
    VaultAssignmentsUpdate? assignments,
  }) {
    final result = UpdateVaultProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (authToken != null) result.authToken = authToken;
    if (namespace != null) result.namespace = namespace;
    if (configJson != null) result.configJson = configJson;
    if (assignments != null) result.assignments = assignments;
    return result;
  }

  UpdateVaultProviderRequest._();

  factory UpdateVaultProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateVaultProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateVaultProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateVaultProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(4, _omitFieldNames ? '' : 'authToken')
    ..aOS(5, _omitFieldNames ? '' : 'namespace')
    ..aOS(6, _omitFieldNames ? '' : 'configJson')
    ..aOM<VaultAssignmentsUpdate>(7, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultAssignmentsUpdate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultProviderRequest copyWith(
          void Function(UpdateVaultProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateVaultProviderRequest))
          as UpdateVaultProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateVaultProviderRequest() / UpdateVaultProviderRequest.new instead')
  static UpdateVaultProviderRequest create() => UpdateVaultProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateVaultProviderRequest._();
  @$core.override
  UpdateVaultProviderRequest createEmptyInstance() =>
      UpdateVaultProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateVaultProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateVaultProviderRequest>(
          UpdateVaultProviderRequest.$_createMessage);
  static UpdateVaultProviderRequest? _defaultInstance;

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
  $core.String get apiUrl => $_getSZ(2);
  @$pb.TagNumber(3)
  set apiUrl($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasApiUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearApiUrl() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get authToken => $_getSZ(3);
  @$pb.TagNumber(4)
  set authToken($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAuthToken() => $_has(3);
  @$pb.TagNumber(4)
  void clearAuthToken() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get namespace => $_getSZ(4);
  @$pb.TagNumber(5)
  set namespace($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasNamespace() => $_has(4);
  @$pb.TagNumber(5)
  void clearNamespace() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get configJson => $_getSZ(5);
  @$pb.TagNumber(6)
  set configJson($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasConfigJson() => $_has(5);
  @$pb.TagNumber(6)
  void clearConfigJson() => $_clearField(6);

  @$pb.TagNumber(7)
  VaultAssignmentsUpdate get assignments => $_getN(6);
  @$pb.TagNumber(7)
  set assignments(VaultAssignmentsUpdate value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasAssignments() => $_has(6);
  @$pb.TagNumber(7)
  void clearAssignments() => $_clearField(7);
  @$pb.TagNumber(7)
  VaultAssignmentsUpdate ensureAssignments() => $_ensure(6);
}

class VaultAssignmentsUpdate extends $pb.GeneratedMessage {
  factory VaultAssignmentsUpdate({
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = VaultAssignmentsUpdate._();
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  VaultAssignmentsUpdate._();

  factory VaultAssignmentsUpdate.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultAssignmentsUpdate()..mergeFromBuffer(data, registry);
  factory VaultAssignmentsUpdate.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      VaultAssignmentsUpdate()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'VaultAssignmentsUpdate',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: VaultAssignmentsUpdate.$_createMessage)
    ..pPM<VaultProviderAssignment>(1, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultAssignmentsUpdate clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  VaultAssignmentsUpdate copyWith(
          void Function(VaultAssignmentsUpdate) updates) =>
      super.copyWith((message) => updates(message as VaultAssignmentsUpdate))
          as VaultAssignmentsUpdate;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use VaultAssignmentsUpdate() / VaultAssignmentsUpdate.new instead')
  static VaultAssignmentsUpdate create() => VaultAssignmentsUpdate._();
  static $pb.GeneratedMessage $_createMessage() => VaultAssignmentsUpdate._();
  @$core.override
  VaultAssignmentsUpdate createEmptyInstance() => VaultAssignmentsUpdate._();
  @$core.pragma('dart2js:noInline')
  static VaultAssignmentsUpdate getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<VaultAssignmentsUpdate>(
          VaultAssignmentsUpdate.$_createMessage);
  static VaultAssignmentsUpdate? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(0);
}

class UpdateVaultProviderResponse extends $pb.GeneratedMessage {
  factory UpdateVaultProviderResponse({
    VaultProvider? provider,
  }) {
    final result = UpdateVaultProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateVaultProviderResponse._();

  factory UpdateVaultProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateVaultProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateVaultProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateVaultProviderResponse.$_createMessage)
    ..aOM<VaultProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: VaultProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultProviderResponse copyWith(
          void Function(UpdateVaultProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateVaultProviderResponse))
          as UpdateVaultProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateVaultProviderResponse() / UpdateVaultProviderResponse.new instead')
  static UpdateVaultProviderResponse create() =>
      UpdateVaultProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateVaultProviderResponse._();
  @$core.override
  UpdateVaultProviderResponse createEmptyInstance() =>
      UpdateVaultProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateVaultProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateVaultProviderResponse>(
          UpdateVaultProviderResponse.$_createMessage);
  static UpdateVaultProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  VaultProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(VaultProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  VaultProvider ensureProvider() => $_ensure(0);
}

class DeleteVaultProviderRequest extends $pb.GeneratedMessage {
  factory DeleteVaultProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteVaultProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteVaultProviderRequest._();

  factory DeleteVaultProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteVaultProviderRequest()..mergeFromBuffer(data, registry);
  factory DeleteVaultProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteVaultProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteVaultProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteVaultProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteVaultProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteVaultProviderRequest copyWith(
          void Function(DeleteVaultProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeleteVaultProviderRequest))
          as DeleteVaultProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteVaultProviderRequest() / DeleteVaultProviderRequest.new instead')
  static DeleteVaultProviderRequest create() => DeleteVaultProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      DeleteVaultProviderRequest._();
  @$core.override
  DeleteVaultProviderRequest createEmptyInstance() =>
      DeleteVaultProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteVaultProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteVaultProviderRequest>(
          DeleteVaultProviderRequest.$_createMessage);
  static DeleteVaultProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestVaultProviderRequest extends $pb.GeneratedMessage {
  factory TestVaultProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = TestVaultProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  TestVaultProviderRequest._();

  factory TestVaultProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultProviderRequest()..mergeFromBuffer(data, registry);
  factory TestVaultProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestVaultProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestVaultProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultProviderRequest copyWith(
          void Function(TestVaultProviderRequest) updates) =>
      super.copyWith((message) => updates(message as TestVaultProviderRequest))
          as TestVaultProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestVaultProviderRequest() / TestVaultProviderRequest.new instead')
  static TestVaultProviderRequest create() => TestVaultProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestVaultProviderRequest._();
  @$core.override
  TestVaultProviderRequest createEmptyInstance() =>
      TestVaultProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static TestVaultProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestVaultProviderRequest>(
          TestVaultProviderRequest.$_createMessage);
  static TestVaultProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestVaultProviderResponse extends $pb.GeneratedMessage {
  factory TestVaultProviderResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestVaultProviderResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestVaultProviderResponse._();

  factory TestVaultProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultProviderResponse()..mergeFromBuffer(data, registry);
  factory TestVaultProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestVaultProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestVaultProviderResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultProviderResponse copyWith(
          void Function(TestVaultProviderResponse) updates) =>
      super.copyWith((message) => updates(message as TestVaultProviderResponse))
          as TestVaultProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestVaultProviderResponse() / TestVaultProviderResponse.new instead')
  static TestVaultProviderResponse create() => TestVaultProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TestVaultProviderResponse._();
  @$core.override
  TestVaultProviderResponse createEmptyInstance() =>
      TestVaultProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static TestVaultProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestVaultProviderResponse>(
          TestVaultProviderResponse.$_createMessage);
  static TestVaultProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

/// Unsaved-credentials test used by the create dialog.
class TestVaultConnectionRequest extends $pb.GeneratedMessage {
  factory TestVaultConnectionRequest({
    VaultProviderType? providerType,
    $core.String? apiUrl,
    $core.String? authToken,
    $core.String? namespace,
    $core.String? configJson,
  }) {
    final result = TestVaultConnectionRequest._();
    if (providerType != null) result.providerType = providerType;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (authToken != null) result.authToken = authToken;
    if (namespace != null) result.namespace = namespace;
    if (configJson != null) result.configJson = configJson;
    return result;
  }

  TestVaultConnectionRequest._();

  factory TestVaultConnectionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultConnectionRequest()..mergeFromBuffer(data, registry);
  factory TestVaultConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestVaultConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestVaultConnectionRequest.$_createMessage)
    ..aE<VaultProviderType>(1, _omitFieldNames ? '' : 'providerType',
        enumValues: VaultProviderType.values)
    ..aOS(2, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(3, _omitFieldNames ? '' : 'authToken')
    ..aOS(4, _omitFieldNames ? '' : 'namespace')
    ..aOS(5, _omitFieldNames ? '' : 'configJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultConnectionRequest copyWith(
          void Function(TestVaultConnectionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TestVaultConnectionRequest))
          as TestVaultConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestVaultConnectionRequest() / TestVaultConnectionRequest.new instead')
  static TestVaultConnectionRequest create() => TestVaultConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TestVaultConnectionRequest._();
  @$core.override
  TestVaultConnectionRequest createEmptyInstance() =>
      TestVaultConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static TestVaultConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestVaultConnectionRequest>(
          TestVaultConnectionRequest.$_createMessage);
  static TestVaultConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  VaultProviderType get providerType => $_getN(0);
  @$pb.TagNumber(1)
  set providerType(VaultProviderType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderType() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get apiUrl => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiUrl($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get authToken => $_getSZ(2);
  @$pb.TagNumber(3)
  set authToken($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAuthToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearAuthToken() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get namespace => $_getSZ(3);
  @$pb.TagNumber(4)
  set namespace($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasNamespace() => $_has(3);
  @$pb.TagNumber(4)
  void clearNamespace() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get configJson => $_getSZ(4);
  @$pb.TagNumber(5)
  set configJson($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasConfigJson() => $_has(4);
  @$pb.TagNumber(5)
  void clearConfigJson() => $_clearField(5);
}

class TestVaultConnectionResponse extends $pb.GeneratedMessage {
  factory TestVaultConnectionResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestVaultConnectionResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestVaultConnectionResponse._();

  factory TestVaultConnectionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultConnectionResponse()..mergeFromBuffer(data, registry);
  factory TestVaultConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestVaultConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestVaultConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestVaultConnectionResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestVaultConnectionResponse copyWith(
          void Function(TestVaultConnectionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TestVaultConnectionResponse))
          as TestVaultConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestVaultConnectionResponse() / TestVaultConnectionResponse.new instead')
  static TestVaultConnectionResponse create() =>
      TestVaultConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TestVaultConnectionResponse._();
  @$core.override
  TestVaultConnectionResponse createEmptyInstance() =>
      TestVaultConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static TestVaultConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestVaultConnectionResponse>(
          TestVaultConnectionResponse.$_createMessage);
  static TestVaultConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

class ListVaultSecretsRequest extends $pb.GeneratedMessage {
  factory ListVaultSecretsRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = ListVaultSecretsRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  ListVaultSecretsRequest._();

  factory ListVaultSecretsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultSecretsRequest()..mergeFromBuffer(data, registry);
  factory ListVaultSecretsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultSecretsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultSecretsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultSecretsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultSecretsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultSecretsRequest copyWith(
          void Function(ListVaultSecretsRequest) updates) =>
      super.copyWith((message) => updates(message as ListVaultSecretsRequest))
          as ListVaultSecretsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultSecretsRequest() / ListVaultSecretsRequest.new instead')
  static ListVaultSecretsRequest create() => ListVaultSecretsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListVaultSecretsRequest._();
  @$core.override
  ListVaultSecretsRequest createEmptyInstance() => ListVaultSecretsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListVaultSecretsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultSecretsRequest>(
          ListVaultSecretsRequest.$_createMessage);
  static ListVaultSecretsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class ListVaultSecretsResponse extends $pb.GeneratedMessage {
  factory ListVaultSecretsResponse({
    $core.Iterable<$core.String>? secrets,
  }) {
    final result = ListVaultSecretsResponse._();
    if (secrets != null) result.secrets.addAll(secrets);
    return result;
  }

  ListVaultSecretsResponse._();

  factory ListVaultSecretsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultSecretsResponse()..mergeFromBuffer(data, registry);
  factory ListVaultSecretsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultSecretsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultSecretsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultSecretsResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'secrets')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultSecretsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultSecretsResponse copyWith(
          void Function(ListVaultSecretsResponse) updates) =>
      super.copyWith((message) => updates(message as ListVaultSecretsResponse))
          as ListVaultSecretsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultSecretsResponse() / ListVaultSecretsResponse.new instead')
  static ListVaultSecretsResponse create() => ListVaultSecretsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListVaultSecretsResponse._();
  @$core.override
  ListVaultSecretsResponse createEmptyInstance() =>
      ListVaultSecretsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListVaultSecretsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultSecretsResponse>(
          ListVaultSecretsResponse.$_createMessage);
  static ListVaultSecretsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get secrets => $_getList(0);
}

class ListVaultAssignmentsRequest extends $pb.GeneratedMessage {
  factory ListVaultAssignmentsRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = ListVaultAssignmentsRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  ListVaultAssignmentsRequest._();

  factory ListVaultAssignmentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultAssignmentsRequest()..mergeFromBuffer(data, registry);
  factory ListVaultAssignmentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultAssignmentsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultAssignmentsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultAssignmentsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultAssignmentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultAssignmentsRequest copyWith(
          void Function(ListVaultAssignmentsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListVaultAssignmentsRequest))
          as ListVaultAssignmentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultAssignmentsRequest() / ListVaultAssignmentsRequest.new instead')
  static ListVaultAssignmentsRequest create() =>
      ListVaultAssignmentsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListVaultAssignmentsRequest._();
  @$core.override
  ListVaultAssignmentsRequest createEmptyInstance() =>
      ListVaultAssignmentsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListVaultAssignmentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultAssignmentsRequest>(
          ListVaultAssignmentsRequest.$_createMessage);
  static ListVaultAssignmentsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class ListVaultAssignmentsResponse extends $pb.GeneratedMessage {
  factory ListVaultAssignmentsResponse({
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = ListVaultAssignmentsResponse._();
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  ListVaultAssignmentsResponse._();

  factory ListVaultAssignmentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultAssignmentsResponse()..mergeFromBuffer(data, registry);
  factory ListVaultAssignmentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListVaultAssignmentsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListVaultAssignmentsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListVaultAssignmentsResponse.$_createMessage)
    ..pPM<VaultProviderAssignment>(1, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultAssignmentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListVaultAssignmentsResponse copyWith(
          void Function(ListVaultAssignmentsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListVaultAssignmentsResponse))
          as ListVaultAssignmentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListVaultAssignmentsResponse() / ListVaultAssignmentsResponse.new instead')
  static ListVaultAssignmentsResponse create() =>
      ListVaultAssignmentsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListVaultAssignmentsResponse._();
  @$core.override
  ListVaultAssignmentsResponse createEmptyInstance() =>
      ListVaultAssignmentsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListVaultAssignmentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListVaultAssignmentsResponse>(
          ListVaultAssignmentsResponse.$_createMessage);
  static ListVaultAssignmentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(0);
}

/// Full replacement of the provider's assignment list.
class UpdateVaultAssignmentsRequest extends $pb.GeneratedMessage {
  factory UpdateVaultAssignmentsRequest({
    $fixnum.Int64? providerId,
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = UpdateVaultAssignmentsRequest._();
    if (providerId != null) result.providerId = providerId;
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  UpdateVaultAssignmentsRequest._();

  factory UpdateVaultAssignmentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultAssignmentsRequest()..mergeFromBuffer(data, registry);
  factory UpdateVaultAssignmentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultAssignmentsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateVaultAssignmentsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateVaultAssignmentsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..pPM<VaultProviderAssignment>(2, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultAssignmentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultAssignmentsRequest copyWith(
          void Function(UpdateVaultAssignmentsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateVaultAssignmentsRequest))
          as UpdateVaultAssignmentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateVaultAssignmentsRequest() / UpdateVaultAssignmentsRequest.new instead')
  static UpdateVaultAssignmentsRequest create() =>
      UpdateVaultAssignmentsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateVaultAssignmentsRequest._();
  @$core.override
  UpdateVaultAssignmentsRequest createEmptyInstance() =>
      UpdateVaultAssignmentsRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateVaultAssignmentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateVaultAssignmentsRequest>(
          UpdateVaultAssignmentsRequest.$_createMessage);
  static UpdateVaultAssignmentsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(1);
}

class UpdateVaultAssignmentsResponse extends $pb.GeneratedMessage {
  factory UpdateVaultAssignmentsResponse({
    $core.Iterable<VaultProviderAssignment>? assignments,
  }) {
    final result = UpdateVaultAssignmentsResponse._();
    if (assignments != null) result.assignments.addAll(assignments);
    return result;
  }

  UpdateVaultAssignmentsResponse._();

  factory UpdateVaultAssignmentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultAssignmentsResponse()..mergeFromBuffer(data, registry);
  factory UpdateVaultAssignmentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateVaultAssignmentsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateVaultAssignmentsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateVaultAssignmentsResponse.$_createMessage)
    ..pPM<VaultProviderAssignment>(1, _omitFieldNames ? '' : 'assignments',
        subBuilder: VaultProviderAssignment.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultAssignmentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateVaultAssignmentsResponse copyWith(
          void Function(UpdateVaultAssignmentsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateVaultAssignmentsResponse))
          as UpdateVaultAssignmentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateVaultAssignmentsResponse() / UpdateVaultAssignmentsResponse.new instead')
  static UpdateVaultAssignmentsResponse create() =>
      UpdateVaultAssignmentsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateVaultAssignmentsResponse._();
  @$core.override
  UpdateVaultAssignmentsResponse createEmptyInstance() =>
      UpdateVaultAssignmentsResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateVaultAssignmentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateVaultAssignmentsResponse>(
          UpdateVaultAssignmentsResponse.$_createMessage);
  static UpdateVaultAssignmentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<VaultProviderAssignment> get assignments => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

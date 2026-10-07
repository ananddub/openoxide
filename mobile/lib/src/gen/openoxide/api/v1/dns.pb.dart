// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/dns.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'dns.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'dns.pbenum.dart';

class DnsProvider extends $pb.GeneratedMessage {
  factory DnsProvider({
    $fixnum.Int64? id,
    $core.String? name,
    DnsProviderType? providerType,
    $core.String? credentialsJson,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = DnsProvider._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (credentialsJson != null) result.credentialsJson = credentialsJson;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  DnsProvider._();

  factory DnsProvider.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsProvider()..mergeFromBuffer(data, registry);
  factory DnsProvider.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsProvider()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DnsProvider',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DnsProvider.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<DnsProviderType>(3, _omitFieldNames ? '' : 'providerType',
        enumValues: DnsProviderType.values)
    ..aOS(4, _omitFieldNames ? '' : 'credentialsJson')
    ..aInt64(5, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(6, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(7, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsProvider clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsProvider copyWith(void Function(DnsProvider) updates) =>
      super.copyWith((message) => updates(message as DnsProvider))
          as DnsProvider;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DnsProvider() / DnsProvider.new instead')
  static DnsProvider create() => DnsProvider._();
  static $pb.GeneratedMessage $_createMessage() => DnsProvider._();
  @$core.override
  DnsProvider createEmptyInstance() => DnsProvider._();
  @$core.pragma('dart2js:noInline')
  static DnsProvider getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DnsProvider>(
          DnsProvider.$_createMessage);
  static DnsProvider? _defaultInstance;

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
  DnsProviderType get providerType => $_getN(2);
  @$pb.TagNumber(3)
  set providerType(DnsProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProviderType() => $_has(2);
  @$pb.TagNumber(3)
  void clearProviderType() => $_clearField(3);

  /// Provider credentials document; shape is provider-specific.
  @$pb.TagNumber(4)
  $core.String get credentialsJson => $_getSZ(3);
  @$pb.TagNumber(4)
  set credentialsJson($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCredentialsJson() => $_has(3);
  @$pb.TagNumber(4)
  void clearCredentialsJson() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get organizationId => $_getI64(4);
  @$pb.TagNumber(5)
  set organizationId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOrganizationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearOrganizationId() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get createdAt => $_getI64(5);
  @$pb.TagNumber(6)
  set createdAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCreatedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get updatedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set updatedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasUpdatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearUpdatedAt() => $_clearField(7);
}

class DnsZone extends $pb.GeneratedMessage {
  factory DnsZone({
    $core.String? id,
    $core.String? name,
    $core.String? status,
  }) {
    final result = DnsZone._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (status != null) result.status = status;
    return result;
  }

  DnsZone._();

  factory DnsZone.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsZone()..mergeFromBuffer(data, registry);
  factory DnsZone.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsZone()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DnsZone',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DnsZone.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'status')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsZone clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsZone copyWith(void Function(DnsZone) updates) =>
      super.copyWith((message) => updates(message as DnsZone)) as DnsZone;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DnsZone() / DnsZone.new instead')
  static DnsZone create() => DnsZone._();
  static $pb.GeneratedMessage $_createMessage() => DnsZone._();
  @$core.override
  DnsZone createEmptyInstance() => DnsZone._();
  @$core.pragma('dart2js:noInline')
  static DnsZone getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DnsZone>(DnsZone.$_createMessage);
  static DnsZone? _defaultInstance;

  /// Zone identifier as issued by the provider (not a panel row id).
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
  $core.String get status => $_getSZ(2);
  @$pb.TagNumber(3)
  set status($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);
}

class DnsRecord extends $pb.GeneratedMessage {
  factory DnsRecord({
    $core.String? id,
    $core.String? zoneId,
    $core.String? recordType,
    $core.String? name,
    $core.String? content,
    $core.int? ttl,
    $core.bool? proxied,
  }) {
    final result = DnsRecord._();
    if (id != null) result.id = id;
    if (zoneId != null) result.zoneId = zoneId;
    if (recordType != null) result.recordType = recordType;
    if (name != null) result.name = name;
    if (content != null) result.content = content;
    if (ttl != null) result.ttl = ttl;
    if (proxied != null) result.proxied = proxied;
    return result;
  }

  DnsRecord._();

  factory DnsRecord.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsRecord()..mergeFromBuffer(data, registry);
  factory DnsRecord.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DnsRecord()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DnsRecord',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DnsRecord.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'zoneId')
    ..aOS(3, _omitFieldNames ? '' : 'recordType')
    ..aOS(4, _omitFieldNames ? '' : 'name')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..aI(6, _omitFieldNames ? '' : 'ttl', fieldType: $pb.PbFieldType.OU3)
    ..aOB(7, _omitFieldNames ? '' : 'proxied')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsRecord clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DnsRecord copyWith(void Function(DnsRecord) updates) =>
      super.copyWith((message) => updates(message as DnsRecord)) as DnsRecord;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DnsRecord() / DnsRecord.new instead')
  static DnsRecord create() => DnsRecord._();
  static $pb.GeneratedMessage $_createMessage() => DnsRecord._();
  @$core.override
  DnsRecord createEmptyInstance() => DnsRecord._();
  @$core.pragma('dart2js:noInline')
  static DnsRecord getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DnsRecord>(DnsRecord.$_createMessage);
  static DnsRecord? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get zoneId => $_getSZ(1);
  @$pb.TagNumber(2)
  set zoneId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasZoneId() => $_has(1);
  @$pb.TagNumber(2)
  void clearZoneId() => $_clearField(2);

  /// Provider-native record type ("A", "AAAA", "CNAME", "TXT", "MX", ...).
  @$pb.TagNumber(3)
  $core.String get recordType => $_getSZ(2);
  @$pb.TagNumber(3)
  set recordType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRecordType() => $_has(2);
  @$pb.TagNumber(3)
  void clearRecordType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get name => $_getSZ(3);
  @$pb.TagNumber(4)
  set name($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasName() => $_has(3);
  @$pb.TagNumber(4)
  void clearName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.int get ttl => $_getIZ(5);
  @$pb.TagNumber(6)
  set ttl($core.int value) => $_setUnsignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTtl() => $_has(5);
  @$pb.TagNumber(6)
  void clearTtl() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get proxied => $_getBF(6);
  @$pb.TagNumber(7)
  set proxied($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasProxied() => $_has(6);
  @$pb.TagNumber(7)
  void clearProxied() => $_clearField(7);
}

class ListDnsProvidersRequest extends $pb.GeneratedMessage {
  factory ListDnsProvidersRequest() => ListDnsProvidersRequest._();

  ListDnsProvidersRequest._();

  factory ListDnsProvidersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsProvidersRequest()..mergeFromBuffer(data, registry);
  factory ListDnsProvidersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsProvidersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsProvidersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsProvidersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsProvidersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsProvidersRequest copyWith(
          void Function(ListDnsProvidersRequest) updates) =>
      super.copyWith((message) => updates(message as ListDnsProvidersRequest))
          as ListDnsProvidersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDnsProvidersRequest() / ListDnsProvidersRequest.new instead')
  static ListDnsProvidersRequest create() => ListDnsProvidersRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsProvidersRequest._();
  @$core.override
  ListDnsProvidersRequest createEmptyInstance() => ListDnsProvidersRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDnsProvidersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsProvidersRequest>(
          ListDnsProvidersRequest.$_createMessage);
  static ListDnsProvidersRequest? _defaultInstance;
}

class ListDnsProvidersResponse extends $pb.GeneratedMessage {
  factory ListDnsProvidersResponse({
    $core.Iterable<DnsProvider>? providers,
  }) {
    final result = ListDnsProvidersResponse._();
    if (providers != null) result.providers.addAll(providers);
    return result;
  }

  ListDnsProvidersResponse._();

  factory ListDnsProvidersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsProvidersResponse()..mergeFromBuffer(data, registry);
  factory ListDnsProvidersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsProvidersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsProvidersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsProvidersResponse.$_createMessage)
    ..pPM<DnsProvider>(1, _omitFieldNames ? '' : 'providers',
        subBuilder: DnsProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsProvidersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsProvidersResponse copyWith(
          void Function(ListDnsProvidersResponse) updates) =>
      super.copyWith((message) => updates(message as ListDnsProvidersResponse))
          as ListDnsProvidersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDnsProvidersResponse() / ListDnsProvidersResponse.new instead')
  static ListDnsProvidersResponse create() => ListDnsProvidersResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsProvidersResponse._();
  @$core.override
  ListDnsProvidersResponse createEmptyInstance() =>
      ListDnsProvidersResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDnsProvidersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsProvidersResponse>(
          ListDnsProvidersResponse.$_createMessage);
  static ListDnsProvidersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DnsProvider> get providers => $_getList(0);
}

class GetDnsProviderRequest extends $pb.GeneratedMessage {
  factory GetDnsProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetDnsProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetDnsProviderRequest._();

  factory GetDnsProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDnsProviderRequest()..mergeFromBuffer(data, registry);
  factory GetDnsProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDnsProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDnsProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDnsProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDnsProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDnsProviderRequest copyWith(
          void Function(GetDnsProviderRequest) updates) =>
      super.copyWith((message) => updates(message as GetDnsProviderRequest))
          as GetDnsProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetDnsProviderRequest() / GetDnsProviderRequest.new instead')
  static GetDnsProviderRequest create() => GetDnsProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetDnsProviderRequest._();
  @$core.override
  GetDnsProviderRequest createEmptyInstance() => GetDnsProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static GetDnsProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDnsProviderRequest>(
          GetDnsProviderRequest.$_createMessage);
  static GetDnsProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetDnsProviderResponse extends $pb.GeneratedMessage {
  factory GetDnsProviderResponse({
    DnsProvider? provider,
  }) {
    final result = GetDnsProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  GetDnsProviderResponse._();

  factory GetDnsProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDnsProviderResponse()..mergeFromBuffer(data, registry);
  factory GetDnsProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDnsProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDnsProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDnsProviderResponse.$_createMessage)
    ..aOM<DnsProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: DnsProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDnsProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDnsProviderResponse copyWith(
          void Function(GetDnsProviderResponse) updates) =>
      super.copyWith((message) => updates(message as GetDnsProviderResponse))
          as GetDnsProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetDnsProviderResponse() / GetDnsProviderResponse.new instead')
  static GetDnsProviderResponse create() => GetDnsProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetDnsProviderResponse._();
  @$core.override
  GetDnsProviderResponse createEmptyInstance() => GetDnsProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static GetDnsProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDnsProviderResponse>(
          GetDnsProviderResponse.$_createMessage);
  static GetDnsProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DnsProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(DnsProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  DnsProvider ensureProvider() => $_ensure(0);
}

class CreateDnsProviderRequest extends $pb.GeneratedMessage {
  factory CreateDnsProviderRequest({
    $core.String? name,
    DnsProviderType? providerType,
    $core.String? credentialsJson,
  }) {
    final result = CreateDnsProviderRequest._();
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (credentialsJson != null) result.credentialsJson = credentialsJson;
    return result;
  }

  CreateDnsProviderRequest._();

  factory CreateDnsProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateDnsProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDnsProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDnsProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aE<DnsProviderType>(2, _omitFieldNames ? '' : 'providerType',
        enumValues: DnsProviderType.values)
    ..aOS(3, _omitFieldNames ? '' : 'credentialsJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsProviderRequest copyWith(
          void Function(CreateDnsProviderRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDnsProviderRequest))
          as CreateDnsProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDnsProviderRequest() / CreateDnsProviderRequest.new instead')
  static CreateDnsProviderRequest create() => CreateDnsProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateDnsProviderRequest._();
  @$core.override
  CreateDnsProviderRequest createEmptyInstance() =>
      CreateDnsProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateDnsProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDnsProviderRequest>(
          CreateDnsProviderRequest.$_createMessage);
  static CreateDnsProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  DnsProviderType get providerType => $_getN(1);
  @$pb.TagNumber(2)
  set providerType(DnsProviderType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasProviderType() => $_has(1);
  @$pb.TagNumber(2)
  void clearProviderType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get credentialsJson => $_getSZ(2);
  @$pb.TagNumber(3)
  set credentialsJson($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCredentialsJson() => $_has(2);
  @$pb.TagNumber(3)
  void clearCredentialsJson() => $_clearField(3);
}

class CreateDnsProviderResponse extends $pb.GeneratedMessage {
  factory CreateDnsProviderResponse({
    DnsProvider? provider,
  }) {
    final result = CreateDnsProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  CreateDnsProviderResponse._();

  factory CreateDnsProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateDnsProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDnsProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDnsProviderResponse.$_createMessage)
    ..aOM<DnsProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: DnsProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsProviderResponse copyWith(
          void Function(CreateDnsProviderResponse) updates) =>
      super.copyWith((message) => updates(message as CreateDnsProviderResponse))
          as CreateDnsProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDnsProviderResponse() / CreateDnsProviderResponse.new instead')
  static CreateDnsProviderResponse create() => CreateDnsProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateDnsProviderResponse._();
  @$core.override
  CreateDnsProviderResponse createEmptyInstance() =>
      CreateDnsProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateDnsProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDnsProviderResponse>(
          CreateDnsProviderResponse.$_createMessage);
  static CreateDnsProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DnsProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(DnsProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  DnsProvider ensureProvider() => $_ensure(0);
}

class UpdateDnsProviderRequest extends $pb.GeneratedMessage {
  factory UpdateDnsProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    DnsProviderType? providerType,
    $core.String? credentialsJson,
  }) {
    final result = UpdateDnsProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (credentialsJson != null) result.credentialsJson = credentialsJson;
    return result;
  }

  UpdateDnsProviderRequest._();

  factory UpdateDnsProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateDnsProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDnsProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDnsProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<DnsProviderType>(3, _omitFieldNames ? '' : 'providerType',
        enumValues: DnsProviderType.values)
    ..aOS(4, _omitFieldNames ? '' : 'credentialsJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsProviderRequest copyWith(
          void Function(UpdateDnsProviderRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateDnsProviderRequest))
          as UpdateDnsProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDnsProviderRequest() / UpdateDnsProviderRequest.new instead')
  static UpdateDnsProviderRequest create() => UpdateDnsProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDnsProviderRequest._();
  @$core.override
  UpdateDnsProviderRequest createEmptyInstance() =>
      UpdateDnsProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateDnsProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDnsProviderRequest>(
          UpdateDnsProviderRequest.$_createMessage);
  static UpdateDnsProviderRequest? _defaultInstance;

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
  DnsProviderType get providerType => $_getN(2);
  @$pb.TagNumber(3)
  set providerType(DnsProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProviderType() => $_has(2);
  @$pb.TagNumber(3)
  void clearProviderType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get credentialsJson => $_getSZ(3);
  @$pb.TagNumber(4)
  set credentialsJson($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCredentialsJson() => $_has(3);
  @$pb.TagNumber(4)
  void clearCredentialsJson() => $_clearField(4);
}

class UpdateDnsProviderResponse extends $pb.GeneratedMessage {
  factory UpdateDnsProviderResponse({
    DnsProvider? provider,
  }) {
    final result = UpdateDnsProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateDnsProviderResponse._();

  factory UpdateDnsProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateDnsProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDnsProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDnsProviderResponse.$_createMessage)
    ..aOM<DnsProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: DnsProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsProviderResponse copyWith(
          void Function(UpdateDnsProviderResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateDnsProviderResponse))
          as UpdateDnsProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDnsProviderResponse() / UpdateDnsProviderResponse.new instead')
  static UpdateDnsProviderResponse create() => UpdateDnsProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateDnsProviderResponse._();
  @$core.override
  UpdateDnsProviderResponse createEmptyInstance() =>
      UpdateDnsProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateDnsProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDnsProviderResponse>(
          UpdateDnsProviderResponse.$_createMessage);
  static UpdateDnsProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DnsProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(DnsProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  DnsProvider ensureProvider() => $_ensure(0);
}

class DeleteDnsProviderRequest extends $pb.GeneratedMessage {
  factory DeleteDnsProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteDnsProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteDnsProviderRequest._();

  factory DeleteDnsProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDnsProviderRequest()..mergeFromBuffer(data, registry);
  factory DeleteDnsProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDnsProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteDnsProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteDnsProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDnsProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDnsProviderRequest copyWith(
          void Function(DeleteDnsProviderRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteDnsProviderRequest))
          as DeleteDnsProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteDnsProviderRequest() / DeleteDnsProviderRequest.new instead')
  static DeleteDnsProviderRequest create() => DeleteDnsProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteDnsProviderRequest._();
  @$core.override
  DeleteDnsProviderRequest createEmptyInstance() =>
      DeleteDnsProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteDnsProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteDnsProviderRequest>(
          DeleteDnsProviderRequest.$_createMessage);
  static DeleteDnsProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestDnsProviderRequest extends $pb.GeneratedMessage {
  factory TestDnsProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = TestDnsProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  TestDnsProviderRequest._();

  factory TestDnsProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsProviderRequest()..mergeFromBuffer(data, registry);
  factory TestDnsProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDnsProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDnsProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsProviderRequest copyWith(
          void Function(TestDnsProviderRequest) updates) =>
      super.copyWith((message) => updates(message as TestDnsProviderRequest))
          as TestDnsProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDnsProviderRequest() / TestDnsProviderRequest.new instead')
  static TestDnsProviderRequest create() => TestDnsProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestDnsProviderRequest._();
  @$core.override
  TestDnsProviderRequest createEmptyInstance() => TestDnsProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static TestDnsProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDnsProviderRequest>(
          TestDnsProviderRequest.$_createMessage);
  static TestDnsProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestDnsProviderResponse extends $pb.GeneratedMessage {
  factory TestDnsProviderResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestDnsProviderResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestDnsProviderResponse._();

  factory TestDnsProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsProviderResponse()..mergeFromBuffer(data, registry);
  factory TestDnsProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDnsProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDnsProviderResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsProviderResponse copyWith(
          void Function(TestDnsProviderResponse) updates) =>
      super.copyWith((message) => updates(message as TestDnsProviderResponse))
          as TestDnsProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDnsProviderResponse() / TestDnsProviderResponse.new instead')
  static TestDnsProviderResponse create() => TestDnsProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestDnsProviderResponse._();
  @$core.override
  TestDnsProviderResponse createEmptyInstance() => TestDnsProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static TestDnsProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDnsProviderResponse>(
          TestDnsProviderResponse.$_createMessage);
  static TestDnsProviderResponse? _defaultInstance;

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

/// Unsaved-credentials test used by the create dialog before a provider row exists.
class TestDnsConnectionRequest extends $pb.GeneratedMessage {
  factory TestDnsConnectionRequest({
    DnsProviderType? providerType,
    $core.String? credentialsJson,
  }) {
    final result = TestDnsConnectionRequest._();
    if (providerType != null) result.providerType = providerType;
    if (credentialsJson != null) result.credentialsJson = credentialsJson;
    return result;
  }

  TestDnsConnectionRequest._();

  factory TestDnsConnectionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsConnectionRequest()..mergeFromBuffer(data, registry);
  factory TestDnsConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDnsConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDnsConnectionRequest.$_createMessage)
    ..aE<DnsProviderType>(1, _omitFieldNames ? '' : 'providerType',
        enumValues: DnsProviderType.values)
    ..aOS(2, _omitFieldNames ? '' : 'credentialsJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsConnectionRequest copyWith(
          void Function(TestDnsConnectionRequest) updates) =>
      super.copyWith((message) => updates(message as TestDnsConnectionRequest))
          as TestDnsConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDnsConnectionRequest() / TestDnsConnectionRequest.new instead')
  static TestDnsConnectionRequest create() => TestDnsConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestDnsConnectionRequest._();
  @$core.override
  TestDnsConnectionRequest createEmptyInstance() =>
      TestDnsConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static TestDnsConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDnsConnectionRequest>(
          TestDnsConnectionRequest.$_createMessage);
  static TestDnsConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  DnsProviderType get providerType => $_getN(0);
  @$pb.TagNumber(1)
  set providerType(DnsProviderType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderType() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get credentialsJson => $_getSZ(1);
  @$pb.TagNumber(2)
  set credentialsJson($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCredentialsJson() => $_has(1);
  @$pb.TagNumber(2)
  void clearCredentialsJson() => $_clearField(2);
}

class TestDnsConnectionResponse extends $pb.GeneratedMessage {
  factory TestDnsConnectionResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestDnsConnectionResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestDnsConnectionResponse._();

  factory TestDnsConnectionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsConnectionResponse()..mergeFromBuffer(data, registry);
  factory TestDnsConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDnsConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDnsConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDnsConnectionResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDnsConnectionResponse copyWith(
          void Function(TestDnsConnectionResponse) updates) =>
      super.copyWith((message) => updates(message as TestDnsConnectionResponse))
          as TestDnsConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDnsConnectionResponse() / TestDnsConnectionResponse.new instead')
  static TestDnsConnectionResponse create() => TestDnsConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TestDnsConnectionResponse._();
  @$core.override
  TestDnsConnectionResponse createEmptyInstance() =>
      TestDnsConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static TestDnsConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDnsConnectionResponse>(
          TestDnsConnectionResponse.$_createMessage);
  static TestDnsConnectionResponse? _defaultInstance;

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

class ListDnsZonesRequest extends $pb.GeneratedMessage {
  factory ListDnsZonesRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = ListDnsZonesRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  ListDnsZonesRequest._();

  factory ListDnsZonesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsZonesRequest()..mergeFromBuffer(data, registry);
  factory ListDnsZonesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsZonesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsZonesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsZonesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsZonesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsZonesRequest copyWith(void Function(ListDnsZonesRequest) updates) =>
      super.copyWith((message) => updates(message as ListDnsZonesRequest))
          as ListDnsZonesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListDnsZonesRequest() / ListDnsZonesRequest.new instead')
  static ListDnsZonesRequest create() => ListDnsZonesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsZonesRequest._();
  @$core.override
  ListDnsZonesRequest createEmptyInstance() => ListDnsZonesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDnsZonesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsZonesRequest>(
          ListDnsZonesRequest.$_createMessage);
  static ListDnsZonesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class ListDnsZonesResponse extends $pb.GeneratedMessage {
  factory ListDnsZonesResponse({
    $core.Iterable<DnsZone>? zones,
  }) {
    final result = ListDnsZonesResponse._();
    if (zones != null) result.zones.addAll(zones);
    return result;
  }

  ListDnsZonesResponse._();

  factory ListDnsZonesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsZonesResponse()..mergeFromBuffer(data, registry);
  factory ListDnsZonesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsZonesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsZonesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsZonesResponse.$_createMessage)
    ..pPM<DnsZone>(1, _omitFieldNames ? '' : 'zones',
        subBuilder: DnsZone.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsZonesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsZonesResponse copyWith(void Function(ListDnsZonesResponse) updates) =>
      super.copyWith((message) => updates(message as ListDnsZonesResponse))
          as ListDnsZonesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDnsZonesResponse() / ListDnsZonesResponse.new instead')
  static ListDnsZonesResponse create() => ListDnsZonesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsZonesResponse._();
  @$core.override
  ListDnsZonesResponse createEmptyInstance() => ListDnsZonesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDnsZonesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsZonesResponse>(
          ListDnsZonesResponse.$_createMessage);
  static ListDnsZonesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DnsZone> get zones => $_getList(0);
}

class ListDnsRecordsRequest extends $pb.GeneratedMessage {
  factory ListDnsRecordsRequest({
    $fixnum.Int64? providerId,
    $core.String? zoneId,
  }) {
    final result = ListDnsRecordsRequest._();
    if (providerId != null) result.providerId = providerId;
    if (zoneId != null) result.zoneId = zoneId;
    return result;
  }

  ListDnsRecordsRequest._();

  factory ListDnsRecordsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsRecordsRequest()..mergeFromBuffer(data, registry);
  factory ListDnsRecordsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsRecordsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsRecordsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsRecordsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'zoneId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsRecordsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsRecordsRequest copyWith(
          void Function(ListDnsRecordsRequest) updates) =>
      super.copyWith((message) => updates(message as ListDnsRecordsRequest))
          as ListDnsRecordsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDnsRecordsRequest() / ListDnsRecordsRequest.new instead')
  static ListDnsRecordsRequest create() => ListDnsRecordsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsRecordsRequest._();
  @$core.override
  ListDnsRecordsRequest createEmptyInstance() => ListDnsRecordsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDnsRecordsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsRecordsRequest>(
          ListDnsRecordsRequest.$_createMessage);
  static ListDnsRecordsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get zoneId => $_getSZ(1);
  @$pb.TagNumber(2)
  set zoneId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasZoneId() => $_has(1);
  @$pb.TagNumber(2)
  void clearZoneId() => $_clearField(2);
}

class ListDnsRecordsResponse extends $pb.GeneratedMessage {
  factory ListDnsRecordsResponse({
    $core.Iterable<DnsRecord>? records,
  }) {
    final result = ListDnsRecordsResponse._();
    if (records != null) result.records.addAll(records);
    return result;
  }

  ListDnsRecordsResponse._();

  factory ListDnsRecordsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsRecordsResponse()..mergeFromBuffer(data, registry);
  factory ListDnsRecordsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDnsRecordsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDnsRecordsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDnsRecordsResponse.$_createMessage)
    ..pPM<DnsRecord>(1, _omitFieldNames ? '' : 'records',
        subBuilder: DnsRecord.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsRecordsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDnsRecordsResponse copyWith(
          void Function(ListDnsRecordsResponse) updates) =>
      super.copyWith((message) => updates(message as ListDnsRecordsResponse))
          as ListDnsRecordsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDnsRecordsResponse() / ListDnsRecordsResponse.new instead')
  static ListDnsRecordsResponse create() => ListDnsRecordsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDnsRecordsResponse._();
  @$core.override
  ListDnsRecordsResponse createEmptyInstance() => ListDnsRecordsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDnsRecordsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDnsRecordsResponse>(
          ListDnsRecordsResponse.$_createMessage);
  static ListDnsRecordsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DnsRecord> get records => $_getList(0);
}

class CreateDnsRecordRequest extends $pb.GeneratedMessage {
  factory CreateDnsRecordRequest({
    $fixnum.Int64? providerId,
    $core.String? zoneId,
    $core.String? recordType,
    $core.String? name,
    $core.String? content,
    $core.int? ttl,
    $core.bool? proxied,
  }) {
    final result = CreateDnsRecordRequest._();
    if (providerId != null) result.providerId = providerId;
    if (zoneId != null) result.zoneId = zoneId;
    if (recordType != null) result.recordType = recordType;
    if (name != null) result.name = name;
    if (content != null) result.content = content;
    if (ttl != null) result.ttl = ttl;
    if (proxied != null) result.proxied = proxied;
    return result;
  }

  CreateDnsRecordRequest._();

  factory CreateDnsRecordRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsRecordRequest()..mergeFromBuffer(data, registry);
  factory CreateDnsRecordRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsRecordRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDnsRecordRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDnsRecordRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'zoneId')
    ..aOS(3, _omitFieldNames ? '' : 'recordType')
    ..aOS(4, _omitFieldNames ? '' : 'name')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..aI(6, _omitFieldNames ? '' : 'ttl', fieldType: $pb.PbFieldType.OU3)
    ..aOB(7, _omitFieldNames ? '' : 'proxied')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsRecordRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsRecordRequest copyWith(
          void Function(CreateDnsRecordRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDnsRecordRequest))
          as CreateDnsRecordRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDnsRecordRequest() / CreateDnsRecordRequest.new instead')
  static CreateDnsRecordRequest create() => CreateDnsRecordRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateDnsRecordRequest._();
  @$core.override
  CreateDnsRecordRequest createEmptyInstance() => CreateDnsRecordRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateDnsRecordRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDnsRecordRequest>(
          CreateDnsRecordRequest.$_createMessage);
  static CreateDnsRecordRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get zoneId => $_getSZ(1);
  @$pb.TagNumber(2)
  set zoneId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasZoneId() => $_has(1);
  @$pb.TagNumber(2)
  void clearZoneId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get recordType => $_getSZ(2);
  @$pb.TagNumber(3)
  set recordType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRecordType() => $_has(2);
  @$pb.TagNumber(3)
  void clearRecordType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get name => $_getSZ(3);
  @$pb.TagNumber(4)
  set name($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasName() => $_has(3);
  @$pb.TagNumber(4)
  void clearName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.int get ttl => $_getIZ(5);
  @$pb.TagNumber(6)
  set ttl($core.int value) => $_setUnsignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTtl() => $_has(5);
  @$pb.TagNumber(6)
  void clearTtl() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get proxied => $_getBF(6);
  @$pb.TagNumber(7)
  set proxied($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasProxied() => $_has(6);
  @$pb.TagNumber(7)
  void clearProxied() => $_clearField(7);
}

class CreateDnsRecordResponse extends $pb.GeneratedMessage {
  factory CreateDnsRecordResponse({
    DnsRecord? record,
  }) {
    final result = CreateDnsRecordResponse._();
    if (record != null) result.record = record;
    return result;
  }

  CreateDnsRecordResponse._();

  factory CreateDnsRecordResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsRecordResponse()..mergeFromBuffer(data, registry);
  factory CreateDnsRecordResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDnsRecordResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDnsRecordResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDnsRecordResponse.$_createMessage)
    ..aOM<DnsRecord>(1, _omitFieldNames ? '' : 'record',
        subBuilder: DnsRecord.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsRecordResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDnsRecordResponse copyWith(
          void Function(CreateDnsRecordResponse) updates) =>
      super.copyWith((message) => updates(message as CreateDnsRecordResponse))
          as CreateDnsRecordResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDnsRecordResponse() / CreateDnsRecordResponse.new instead')
  static CreateDnsRecordResponse create() => CreateDnsRecordResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateDnsRecordResponse._();
  @$core.override
  CreateDnsRecordResponse createEmptyInstance() => CreateDnsRecordResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateDnsRecordResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDnsRecordResponse>(
          CreateDnsRecordResponse.$_createMessage);
  static CreateDnsRecordResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DnsRecord get record => $_getN(0);
  @$pb.TagNumber(1)
  set record(DnsRecord value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => $_clearField(1);
  @$pb.TagNumber(1)
  DnsRecord ensureRecord() => $_ensure(0);
}

class DeleteDnsRecordRequest extends $pb.GeneratedMessage {
  factory DeleteDnsRecordRequest({
    $fixnum.Int64? providerId,
    $core.String? zoneId,
    $core.String? name,
    $core.String? id,
    $core.String? recordType,
    $core.String? content,
  }) {
    final result = DeleteDnsRecordRequest._();
    if (providerId != null) result.providerId = providerId;
    if (zoneId != null) result.zoneId = zoneId;
    if (name != null) result.name = name;
    if (id != null) result.id = id;
    if (recordType != null) result.recordType = recordType;
    if (content != null) result.content = content;
    return result;
  }

  DeleteDnsRecordRequest._();

  factory DeleteDnsRecordRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDnsRecordRequest()..mergeFromBuffer(data, registry);
  factory DeleteDnsRecordRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDnsRecordRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteDnsRecordRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteDnsRecordRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'zoneId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOS(4, _omitFieldNames ? '' : 'id')
    ..aOS(5, _omitFieldNames ? '' : 'recordType')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDnsRecordRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDnsRecordRequest copyWith(
          void Function(DeleteDnsRecordRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteDnsRecordRequest))
          as DeleteDnsRecordRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteDnsRecordRequest() / DeleteDnsRecordRequest.new instead')
  static DeleteDnsRecordRequest create() => DeleteDnsRecordRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteDnsRecordRequest._();
  @$core.override
  DeleteDnsRecordRequest createEmptyInstance() => DeleteDnsRecordRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteDnsRecordRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteDnsRecordRequest>(
          DeleteDnsRecordRequest.$_createMessage);
  static DeleteDnsRecordRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get zoneId => $_getSZ(1);
  @$pb.TagNumber(2)
  set zoneId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasZoneId() => $_has(1);
  @$pb.TagNumber(2)
  void clearZoneId() => $_clearField(2);

  /// Record name relative to the zone ("www", "@" for apex). Providers whose API
  /// has no record id identify a row by name + type + value, so it always goes back.
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  /// The id ListRecords returned. Preferred when the provider has one.
  @$pb.TagNumber(4)
  $core.String get id => $_getSZ(3);
  @$pb.TagNumber(4)
  set id($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasId() => $_has(3);
  @$pb.TagNumber(4)
  void clearId() => $_clearField(4);

  /// Narrow the match to the clicked row: a name can hold several types, and a
  /// type can hold several values.
  @$pb.TagNumber(5)
  $core.String get recordType => $_getSZ(4);
  @$pb.TagNumber(5)
  set recordType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRecordType() => $_has(4);
  @$pb.TagNumber(5)
  void clearRecordType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => $_clearField(6);
}

/// Edit one row in place. The old_* fields identify the row as ListRecords
/// returned it; without them a rename would leave the original record behind.
class UpdateDnsRecordRequest extends $pb.GeneratedMessage {
  factory UpdateDnsRecordRequest({
    $fixnum.Int64? providerId,
    $core.String? zoneId,
    $core.String? id,
    $core.String? oldName,
    $core.String? oldRecordType,
    $core.String? oldContent,
    $core.String? recordType,
    $core.String? name,
    $core.String? content,
    $core.int? ttl,
    $core.bool? proxied,
  }) {
    final result = UpdateDnsRecordRequest._();
    if (providerId != null) result.providerId = providerId;
    if (zoneId != null) result.zoneId = zoneId;
    if (id != null) result.id = id;
    if (oldName != null) result.oldName = oldName;
    if (oldRecordType != null) result.oldRecordType = oldRecordType;
    if (oldContent != null) result.oldContent = oldContent;
    if (recordType != null) result.recordType = recordType;
    if (name != null) result.name = name;
    if (content != null) result.content = content;
    if (ttl != null) result.ttl = ttl;
    if (proxied != null) result.proxied = proxied;
    return result;
  }

  UpdateDnsRecordRequest._();

  factory UpdateDnsRecordRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsRecordRequest()..mergeFromBuffer(data, registry);
  factory UpdateDnsRecordRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsRecordRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDnsRecordRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDnsRecordRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'zoneId')
    ..aOS(3, _omitFieldNames ? '' : 'id')
    ..aOS(4, _omitFieldNames ? '' : 'oldName')
    ..aOS(5, _omitFieldNames ? '' : 'oldRecordType')
    ..aOS(6, _omitFieldNames ? '' : 'oldContent')
    ..aOS(7, _omitFieldNames ? '' : 'recordType')
    ..aOS(8, _omitFieldNames ? '' : 'name')
    ..aOS(9, _omitFieldNames ? '' : 'content')
    ..aI(10, _omitFieldNames ? '' : 'ttl', fieldType: $pb.PbFieldType.OU3)
    ..aOB(11, _omitFieldNames ? '' : 'proxied')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsRecordRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsRecordRequest copyWith(
          void Function(UpdateDnsRecordRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateDnsRecordRequest))
          as UpdateDnsRecordRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDnsRecordRequest() / UpdateDnsRecordRequest.new instead')
  static UpdateDnsRecordRequest create() => UpdateDnsRecordRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDnsRecordRequest._();
  @$core.override
  UpdateDnsRecordRequest createEmptyInstance() => UpdateDnsRecordRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateDnsRecordRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDnsRecordRequest>(
          UpdateDnsRecordRequest.$_createMessage);
  static UpdateDnsRecordRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get zoneId => $_getSZ(1);
  @$pb.TagNumber(2)
  set zoneId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasZoneId() => $_has(1);
  @$pb.TagNumber(2)
  void clearZoneId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get id => $_getSZ(2);
  @$pb.TagNumber(3)
  set id($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasId() => $_has(2);
  @$pb.TagNumber(3)
  void clearId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get oldName => $_getSZ(3);
  @$pb.TagNumber(4)
  set oldName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOldName() => $_has(3);
  @$pb.TagNumber(4)
  void clearOldName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get oldRecordType => $_getSZ(4);
  @$pb.TagNumber(5)
  set oldRecordType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOldRecordType() => $_has(4);
  @$pb.TagNumber(5)
  void clearOldRecordType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get oldContent => $_getSZ(5);
  @$pb.TagNumber(6)
  set oldContent($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOldContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearOldContent() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get recordType => $_getSZ(6);
  @$pb.TagNumber(7)
  set recordType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRecordType() => $_has(6);
  @$pb.TagNumber(7)
  void clearRecordType() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get name => $_getSZ(7);
  @$pb.TagNumber(8)
  set name($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasName() => $_has(7);
  @$pb.TagNumber(8)
  void clearName() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get content => $_getSZ(8);
  @$pb.TagNumber(9)
  set content($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasContent() => $_has(8);
  @$pb.TagNumber(9)
  void clearContent() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.int get ttl => $_getIZ(9);
  @$pb.TagNumber(10)
  set ttl($core.int value) => $_setUnsignedInt32(9, value);
  @$pb.TagNumber(10)
  $core.bool hasTtl() => $_has(9);
  @$pb.TagNumber(10)
  void clearTtl() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get proxied => $_getBF(10);
  @$pb.TagNumber(11)
  set proxied($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasProxied() => $_has(10);
  @$pb.TagNumber(11)
  void clearProxied() => $_clearField(11);
}

class UpdateDnsRecordResponse extends $pb.GeneratedMessage {
  factory UpdateDnsRecordResponse({
    DnsRecord? record,
  }) {
    final result = UpdateDnsRecordResponse._();
    if (record != null) result.record = record;
    return result;
  }

  UpdateDnsRecordResponse._();

  factory UpdateDnsRecordResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsRecordResponse()..mergeFromBuffer(data, registry);
  factory UpdateDnsRecordResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDnsRecordResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDnsRecordResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDnsRecordResponse.$_createMessage)
    ..aOM<DnsRecord>(1, _omitFieldNames ? '' : 'record',
        subBuilder: DnsRecord.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsRecordResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDnsRecordResponse copyWith(
          void Function(UpdateDnsRecordResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateDnsRecordResponse))
          as UpdateDnsRecordResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDnsRecordResponse() / UpdateDnsRecordResponse.new instead')
  static UpdateDnsRecordResponse create() => UpdateDnsRecordResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDnsRecordResponse._();
  @$core.override
  UpdateDnsRecordResponse createEmptyInstance() => UpdateDnsRecordResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateDnsRecordResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDnsRecordResponse>(
          UpdateDnsRecordResponse.$_createMessage);
  static UpdateDnsRecordResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DnsRecord get record => $_getN(0);
  @$pb.TagNumber(1)
  set record(DnsRecord value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => $_clearField(1);
  @$pb.TagNumber(1)
  DnsRecord ensureRecord() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

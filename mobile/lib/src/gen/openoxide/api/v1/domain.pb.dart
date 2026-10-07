// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/domain.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'domain.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'domain.pbenum.dart';

class Domain extends $pb.GeneratedMessage {
  factory Domain({
    $fixnum.Int64? id,
    $core.String? host,
    $core.bool? https,
    $fixnum.Int64? port,
    $core.String? path,
    $core.String? internalPath,
    $core.String? customEntrypoint,
    $core.String? serviceName,
    $core.String? customCertResolver,
    $core.bool? stripPath,
    $core.String? middlewares,
    DomainType? domainType,
    DomainCertificateType? certificateType,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Domain._();
    if (id != null) result.id = id;
    if (host != null) result.host = host;
    if (https != null) result.https = https;
    if (port != null) result.port = port;
    if (path != null) result.path = path;
    if (internalPath != null) result.internalPath = internalPath;
    if (customEntrypoint != null) result.customEntrypoint = customEntrypoint;
    if (serviceName != null) result.serviceName = serviceName;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (stripPath != null) result.stripPath = stripPath;
    if (middlewares != null) result.middlewares = middlewares;
    if (domainType != null) result.domainType = domainType;
    if (certificateType != null) result.certificateType = certificateType;
    if (serviceId != null) result.serviceId = serviceId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Domain._();

  factory Domain.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Domain()..mergeFromBuffer(data, registry);
  factory Domain.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Domain()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Domain',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Domain.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'host')
    ..aOB(3, _omitFieldNames ? '' : 'https')
    ..aInt64(4, _omitFieldNames ? '' : 'port')
    ..aOS(5, _omitFieldNames ? '' : 'path')
    ..aOS(6, _omitFieldNames ? '' : 'internalPath')
    ..aOS(7, _omitFieldNames ? '' : 'customEntrypoint')
    ..aOS(8, _omitFieldNames ? '' : 'serviceName')
    ..aOS(9, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(10, _omitFieldNames ? '' : 'stripPath')
    ..aOS(11, _omitFieldNames ? '' : 'middlewares')
    ..aE<DomainType>(12, _omitFieldNames ? '' : 'domainType',
        enumValues: DomainType.values)
    ..aE<DomainCertificateType>(13, _omitFieldNames ? '' : 'certificateType',
        enumValues: DomainCertificateType.values)
    ..aInt64(14, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(15, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(16, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Domain clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Domain copyWith(void Function(Domain) updates) =>
      super.copyWith((message) => updates(message as Domain)) as Domain;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Domain() / Domain.new instead')
  static Domain create() => Domain._();
  static $pb.GeneratedMessage $_createMessage() => Domain._();
  @$core.override
  Domain createEmptyInstance() => Domain._();
  @$core.pragma('dart2js:noInline')
  static Domain getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Domain>(Domain.$_createMessage);
  static Domain? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get host => $_getSZ(1);
  @$pb.TagNumber(2)
  set host($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHost() => $_has(1);
  @$pb.TagNumber(2)
  void clearHost() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get https => $_getBF(2);
  @$pb.TagNumber(3)
  set https($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHttps() => $_has(2);
  @$pb.TagNumber(3)
  void clearHttps() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get port => $_getI64(3);
  @$pb.TagNumber(4)
  set port($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPort() => $_has(3);
  @$pb.TagNumber(4)
  void clearPort() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get path => $_getSZ(4);
  @$pb.TagNumber(5)
  set path($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get internalPath => $_getSZ(5);
  @$pb.TagNumber(6)
  set internalPath($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasInternalPath() => $_has(5);
  @$pb.TagNumber(6)
  void clearInternalPath() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get customEntrypoint => $_getSZ(6);
  @$pb.TagNumber(7)
  set customEntrypoint($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCustomEntrypoint() => $_has(6);
  @$pb.TagNumber(7)
  void clearCustomEntrypoint() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get serviceName => $_getSZ(7);
  @$pb.TagNumber(8)
  set serviceName($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasServiceName() => $_has(7);
  @$pb.TagNumber(8)
  void clearServiceName() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get customCertResolver => $_getSZ(8);
  @$pb.TagNumber(9)
  set customCertResolver($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCustomCertResolver() => $_has(8);
  @$pb.TagNumber(9)
  void clearCustomCertResolver() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get stripPath => $_getBF(9);
  @$pb.TagNumber(10)
  set stripPath($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasStripPath() => $_has(9);
  @$pb.TagNumber(10)
  void clearStripPath() => $_clearField(10);

  /// Traefik middleware names as a JSON array document, e.g. `["auth"]`.
  @$pb.TagNumber(11)
  $core.String get middlewares => $_getSZ(10);
  @$pb.TagNumber(11)
  set middlewares($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasMiddlewares() => $_has(10);
  @$pb.TagNumber(11)
  void clearMiddlewares() => $_clearField(11);

  @$pb.TagNumber(12)
  DomainType get domainType => $_getN(11);
  @$pb.TagNumber(12)
  set domainType(DomainType value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasDomainType() => $_has(11);
  @$pb.TagNumber(12)
  void clearDomainType() => $_clearField(12);

  @$pb.TagNumber(13)
  DomainCertificateType get certificateType => $_getN(12);
  @$pb.TagNumber(13)
  set certificateType(DomainCertificateType value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasCertificateType() => $_has(12);
  @$pb.TagNumber(13)
  void clearCertificateType() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get serviceId => $_getI64(13);
  @$pb.TagNumber(14)
  set serviceId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasServiceId() => $_has(13);
  @$pb.TagNumber(14)
  void clearServiceId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get createdAt => $_getI64(14);
  @$pb.TagNumber(15)
  set createdAt($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasCreatedAt() => $_has(14);
  @$pb.TagNumber(15)
  void clearCreatedAt() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get updatedAt => $_getI64(15);
  @$pb.TagNumber(16)
  set updatedAt($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasUpdatedAt() => $_has(15);
  @$pb.TagNumber(16)
  void clearUpdatedAt() => $_clearField(16);
}

class ListDomainsRequest extends $pb.GeneratedMessage {
  factory ListDomainsRequest() => ListDomainsRequest._();

  ListDomainsRequest._();

  factory ListDomainsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDomainsRequest()..mergeFromBuffer(data, registry);
  factory ListDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDomainsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDomainsRequest copyWith(void Function(ListDomainsRequest) updates) =>
      super.copyWith((message) => updates(message as ListDomainsRequest))
          as ListDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ListDomainsRequest() / ListDomainsRequest.new instead')
  static ListDomainsRequest create() => ListDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDomainsRequest._();
  @$core.override
  ListDomainsRequest createEmptyInstance() => ListDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDomainsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDomainsRequest>(
          ListDomainsRequest.$_createMessage);
  static ListDomainsRequest? _defaultInstance;
}

class ListDomainsResponse extends $pb.GeneratedMessage {
  factory ListDomainsResponse({
    $core.Iterable<Domain>? domains,
  }) {
    final result = ListDomainsResponse._();
    if (domains != null) result.domains.addAll(domains);
    return result;
  }

  ListDomainsResponse._();

  factory ListDomainsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDomainsResponse()..mergeFromBuffer(data, registry);
  factory ListDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDomainsResponse.$_createMessage)
    ..pPM<Domain>(1, _omitFieldNames ? '' : 'domains',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDomainsResponse copyWith(void Function(ListDomainsResponse) updates) =>
      super.copyWith((message) => updates(message as ListDomainsResponse))
          as ListDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListDomainsResponse() / ListDomainsResponse.new instead')
  static ListDomainsResponse create() => ListDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDomainsResponse._();
  @$core.override
  ListDomainsResponse createEmptyInstance() => ListDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDomainsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDomainsResponse>(
          ListDomainsResponse.$_createMessage);
  static ListDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Domain> get domains => $_getList(0);
}

class GetDomainRequest extends $pb.GeneratedMessage {
  factory GetDomainRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetDomainRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetDomainRequest._();

  factory GetDomainRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDomainRequest()..mergeFromBuffer(data, registry);
  factory GetDomainRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDomainRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDomainRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDomainRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDomainRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDomainRequest copyWith(void Function(GetDomainRequest) updates) =>
      super.copyWith((message) => updates(message as GetDomainRequest))
          as GetDomainRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetDomainRequest() / GetDomainRequest.new instead')
  static GetDomainRequest create() => GetDomainRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetDomainRequest._();
  @$core.override
  GetDomainRequest createEmptyInstance() => GetDomainRequest._();
  @$core.pragma('dart2js:noInline')
  static GetDomainRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDomainRequest>(
          GetDomainRequest.$_createMessage);
  static GetDomainRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetDomainResponse extends $pb.GeneratedMessage {
  factory GetDomainResponse({
    Domain? domain,
  }) {
    final result = GetDomainResponse._();
    if (domain != null) result.domain = domain;
    return result;
  }

  GetDomainResponse._();

  factory GetDomainResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDomainResponse()..mergeFromBuffer(data, registry);
  factory GetDomainResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDomainResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDomainResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDomainResponse.$_createMessage)
    ..aOM<Domain>(1, _omitFieldNames ? '' : 'domain',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDomainResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDomainResponse copyWith(void Function(GetDomainResponse) updates) =>
      super.copyWith((message) => updates(message as GetDomainResponse))
          as GetDomainResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetDomainResponse() / GetDomainResponse.new instead')
  static GetDomainResponse create() => GetDomainResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetDomainResponse._();
  @$core.override
  GetDomainResponse createEmptyInstance() => GetDomainResponse._();
  @$core.pragma('dart2js:noInline')
  static GetDomainResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDomainResponse>(
          GetDomainResponse.$_createMessage);
  static GetDomainResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Domain get domain => $_getN(0);
  @$pb.TagNumber(1)
  set domain(Domain value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDomain() => $_has(0);
  @$pb.TagNumber(1)
  void clearDomain() => $_clearField(1);
  @$pb.TagNumber(1)
  Domain ensureDomain() => $_ensure(0);
}

class ListApplicationDomainsRequest extends $pb.GeneratedMessage {
  factory ListApplicationDomainsRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ListApplicationDomainsRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ListApplicationDomainsRequest._();

  factory ListApplicationDomainsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationDomainsRequest()..mergeFromBuffer(data, registry);
  factory ListApplicationDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListApplicationDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListApplicationDomainsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationDomainsRequest copyWith(
          void Function(ListApplicationDomainsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListApplicationDomainsRequest))
          as ListApplicationDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListApplicationDomainsRequest() / ListApplicationDomainsRequest.new instead')
  static ListApplicationDomainsRequest create() =>
      ListApplicationDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListApplicationDomainsRequest._();
  @$core.override
  ListApplicationDomainsRequest createEmptyInstance() =>
      ListApplicationDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListApplicationDomainsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListApplicationDomainsRequest>(
          ListApplicationDomainsRequest.$_createMessage);
  static ListApplicationDomainsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ListApplicationDomainsResponse extends $pb.GeneratedMessage {
  factory ListApplicationDomainsResponse({
    $core.Iterable<Domain>? domains,
  }) {
    final result = ListApplicationDomainsResponse._();
    if (domains != null) result.domains.addAll(domains);
    return result;
  }

  ListApplicationDomainsResponse._();

  factory ListApplicationDomainsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationDomainsResponse()..mergeFromBuffer(data, registry);
  factory ListApplicationDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListApplicationDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListApplicationDomainsResponse.$_createMessage)
    ..pPM<Domain>(1, _omitFieldNames ? '' : 'domains',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationDomainsResponse copyWith(
          void Function(ListApplicationDomainsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListApplicationDomainsResponse))
          as ListApplicationDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListApplicationDomainsResponse() / ListApplicationDomainsResponse.new instead')
  static ListApplicationDomainsResponse create() =>
      ListApplicationDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListApplicationDomainsResponse._();
  @$core.override
  ListApplicationDomainsResponse createEmptyInstance() =>
      ListApplicationDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListApplicationDomainsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListApplicationDomainsResponse>(
          ListApplicationDomainsResponse.$_createMessage);
  static ListApplicationDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Domain> get domains => $_getList(0);
}

class ListComposeDomainsRequest extends $pb.GeneratedMessage {
  factory ListComposeDomainsRequest({
    $fixnum.Int64? composeId,
  }) {
    final result = ListComposeDomainsRequest._();
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  ListComposeDomainsRequest._();

  factory ListComposeDomainsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposeDomainsRequest()..mergeFromBuffer(data, registry);
  factory ListComposeDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposeDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListComposeDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListComposeDomainsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposeDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposeDomainsRequest copyWith(
          void Function(ListComposeDomainsRequest) updates) =>
      super.copyWith((message) => updates(message as ListComposeDomainsRequest))
          as ListComposeDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListComposeDomainsRequest() / ListComposeDomainsRequest.new instead')
  static ListComposeDomainsRequest create() => ListComposeDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListComposeDomainsRequest._();
  @$core.override
  ListComposeDomainsRequest createEmptyInstance() =>
      ListComposeDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListComposeDomainsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListComposeDomainsRequest>(
          ListComposeDomainsRequest.$_createMessage);
  static ListComposeDomainsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);
}

class ListComposeDomainsResponse extends $pb.GeneratedMessage {
  factory ListComposeDomainsResponse({
    $core.Iterable<Domain>? domains,
  }) {
    final result = ListComposeDomainsResponse._();
    if (domains != null) result.domains.addAll(domains);
    return result;
  }

  ListComposeDomainsResponse._();

  factory ListComposeDomainsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposeDomainsResponse()..mergeFromBuffer(data, registry);
  factory ListComposeDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposeDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListComposeDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListComposeDomainsResponse.$_createMessage)
    ..pPM<Domain>(1, _omitFieldNames ? '' : 'domains',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposeDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposeDomainsResponse copyWith(
          void Function(ListComposeDomainsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListComposeDomainsResponse))
          as ListComposeDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListComposeDomainsResponse() / ListComposeDomainsResponse.new instead')
  static ListComposeDomainsResponse create() => ListComposeDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListComposeDomainsResponse._();
  @$core.override
  ListComposeDomainsResponse createEmptyInstance() =>
      ListComposeDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListComposeDomainsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListComposeDomainsResponse>(
          ListComposeDomainsResponse.$_createMessage);
  static ListComposeDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Domain> get domains => $_getList(0);
}

/// Absent fields fall back to server defaults: path "/", middlewares "[]", domain_type derived from application_id/compose_id, certificate_type NONE.
class CreateDomainRequest extends $pb.GeneratedMessage {
  factory CreateDomainRequest({
    $core.String? host,
    $core.bool? https,
    $fixnum.Int64? port,
    $core.String? path,
    $core.String? internalPath,
    $core.String? customEntrypoint,
    $core.String? serviceName,
    $core.String? customCertResolver,
    $core.bool? stripPath,
    $core.String? middlewares,
    DomainType? domainType,
    DomainCertificateType? certificateType,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
  }) {
    final result = CreateDomainRequest._();
    if (host != null) result.host = host;
    if (https != null) result.https = https;
    if (port != null) result.port = port;
    if (path != null) result.path = path;
    if (internalPath != null) result.internalPath = internalPath;
    if (customEntrypoint != null) result.customEntrypoint = customEntrypoint;
    if (serviceName != null) result.serviceName = serviceName;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (stripPath != null) result.stripPath = stripPath;
    if (middlewares != null) result.middlewares = middlewares;
    if (domainType != null) result.domainType = domainType;
    if (certificateType != null) result.certificateType = certificateType;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  CreateDomainRequest._();

  factory CreateDomainRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDomainRequest()..mergeFromBuffer(data, registry);
  factory CreateDomainRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDomainRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDomainRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDomainRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'host')
    ..aOB(2, _omitFieldNames ? '' : 'https')
    ..aInt64(3, _omitFieldNames ? '' : 'port')
    ..aOS(4, _omitFieldNames ? '' : 'path')
    ..aOS(5, _omitFieldNames ? '' : 'internalPath')
    ..aOS(6, _omitFieldNames ? '' : 'customEntrypoint')
    ..aOS(7, _omitFieldNames ? '' : 'serviceName')
    ..aOS(8, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(9, _omitFieldNames ? '' : 'stripPath')
    ..aOS(10, _omitFieldNames ? '' : 'middlewares')
    ..aE<DomainType>(11, _omitFieldNames ? '' : 'domainType',
        enumValues: DomainType.values)
    ..aE<DomainCertificateType>(12, _omitFieldNames ? '' : 'certificateType',
        enumValues: DomainCertificateType.values)
    ..aInt64(13, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(14, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDomainRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDomainRequest copyWith(void Function(CreateDomainRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDomainRequest))
          as CreateDomainRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use CreateDomainRequest() / CreateDomainRequest.new instead')
  static CreateDomainRequest create() => CreateDomainRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateDomainRequest._();
  @$core.override
  CreateDomainRequest createEmptyInstance() => CreateDomainRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateDomainRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDomainRequest>(
          CreateDomainRequest.$_createMessage);
  static CreateDomainRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get host => $_getSZ(0);
  @$pb.TagNumber(1)
  set host($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearHost() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get https => $_getBF(1);
  @$pb.TagNumber(2)
  set https($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHttps() => $_has(1);
  @$pb.TagNumber(2)
  void clearHttps() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get port => $_getI64(2);
  @$pb.TagNumber(3)
  set port($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPort() => $_has(2);
  @$pb.TagNumber(3)
  void clearPort() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get path => $_getSZ(3);
  @$pb.TagNumber(4)
  set path($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearPath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get internalPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set internalPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasInternalPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearInternalPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get customEntrypoint => $_getSZ(5);
  @$pb.TagNumber(6)
  set customEntrypoint($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCustomEntrypoint() => $_has(5);
  @$pb.TagNumber(6)
  void clearCustomEntrypoint() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serviceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set serviceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceName() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get customCertResolver => $_getSZ(7);
  @$pb.TagNumber(8)
  set customCertResolver($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCustomCertResolver() => $_has(7);
  @$pb.TagNumber(8)
  void clearCustomCertResolver() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get stripPath => $_getBF(8);
  @$pb.TagNumber(9)
  set stripPath($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasStripPath() => $_has(8);
  @$pb.TagNumber(9)
  void clearStripPath() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get middlewares => $_getSZ(9);
  @$pb.TagNumber(10)
  set middlewares($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasMiddlewares() => $_has(9);
  @$pb.TagNumber(10)
  void clearMiddlewares() => $_clearField(10);

  @$pb.TagNumber(11)
  DomainType get domainType => $_getN(10);
  @$pb.TagNumber(11)
  set domainType(DomainType value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasDomainType() => $_has(10);
  @$pb.TagNumber(11)
  void clearDomainType() => $_clearField(11);

  @$pb.TagNumber(12)
  DomainCertificateType get certificateType => $_getN(11);
  @$pb.TagNumber(12)
  set certificateType(DomainCertificateType value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasCertificateType() => $_has(11);
  @$pb.TagNumber(12)
  void clearCertificateType() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get applicationId => $_getI64(12);
  @$pb.TagNumber(13)
  set applicationId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasApplicationId() => $_has(12);
  @$pb.TagNumber(13)
  void clearApplicationId() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get composeId => $_getI64(13);
  @$pb.TagNumber(14)
  set composeId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasComposeId() => $_has(13);
  @$pb.TagNumber(14)
  void clearComposeId() => $_clearField(14);
}

class CreateDomainResponse extends $pb.GeneratedMessage {
  factory CreateDomainResponse({
    Domain? domain,
  }) {
    final result = CreateDomainResponse._();
    if (domain != null) result.domain = domain;
    return result;
  }

  CreateDomainResponse._();

  factory CreateDomainResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDomainResponse()..mergeFromBuffer(data, registry);
  factory CreateDomainResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDomainResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDomainResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDomainResponse.$_createMessage)
    ..aOM<Domain>(1, _omitFieldNames ? '' : 'domain',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDomainResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDomainResponse copyWith(void Function(CreateDomainResponse) updates) =>
      super.copyWith((message) => updates(message as CreateDomainResponse))
          as CreateDomainResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDomainResponse() / CreateDomainResponse.new instead')
  static CreateDomainResponse create() => CreateDomainResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateDomainResponse._();
  @$core.override
  CreateDomainResponse createEmptyInstance() => CreateDomainResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateDomainResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDomainResponse>(
          CreateDomainResponse.$_createMessage);
  static CreateDomainResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Domain get domain => $_getN(0);
  @$pb.TagNumber(1)
  set domain(Domain value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDomain() => $_has(0);
  @$pb.TagNumber(1)
  void clearDomain() => $_clearField(1);
  @$pb.TagNumber(1)
  Domain ensureDomain() => $_ensure(0);
}

/// domain_type is not patchable; it follows the owning service kind.
class UpdateDomainRequest extends $pb.GeneratedMessage {
  factory UpdateDomainRequest({
    $fixnum.Int64? id,
    $core.String? host,
    $core.bool? https,
    $fixnum.Int64? port,
    $core.String? path,
    $core.String? internalPath,
    $core.String? customEntrypoint,
    $core.String? serviceName,
    $core.String? customCertResolver,
    $core.bool? stripPath,
    $core.String? middlewares,
    DomainCertificateType? certificateType,
  }) {
    final result = UpdateDomainRequest._();
    if (id != null) result.id = id;
    if (host != null) result.host = host;
    if (https != null) result.https = https;
    if (port != null) result.port = port;
    if (path != null) result.path = path;
    if (internalPath != null) result.internalPath = internalPath;
    if (customEntrypoint != null) result.customEntrypoint = customEntrypoint;
    if (serviceName != null) result.serviceName = serviceName;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (stripPath != null) result.stripPath = stripPath;
    if (middlewares != null) result.middlewares = middlewares;
    if (certificateType != null) result.certificateType = certificateType;
    return result;
  }

  UpdateDomainRequest._();

  factory UpdateDomainRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDomainRequest()..mergeFromBuffer(data, registry);
  factory UpdateDomainRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDomainRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDomainRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDomainRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'host')
    ..aOB(3, _omitFieldNames ? '' : 'https')
    ..aInt64(4, _omitFieldNames ? '' : 'port')
    ..aOS(5, _omitFieldNames ? '' : 'path')
    ..aOS(6, _omitFieldNames ? '' : 'internalPath')
    ..aOS(7, _omitFieldNames ? '' : 'customEntrypoint')
    ..aOS(8, _omitFieldNames ? '' : 'serviceName')
    ..aOS(9, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(10, _omitFieldNames ? '' : 'stripPath')
    ..aOS(11, _omitFieldNames ? '' : 'middlewares')
    ..aE<DomainCertificateType>(12, _omitFieldNames ? '' : 'certificateType',
        enumValues: DomainCertificateType.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDomainRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDomainRequest copyWith(void Function(UpdateDomainRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateDomainRequest))
          as UpdateDomainRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use UpdateDomainRequest() / UpdateDomainRequest.new instead')
  static UpdateDomainRequest create() => UpdateDomainRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDomainRequest._();
  @$core.override
  UpdateDomainRequest createEmptyInstance() => UpdateDomainRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateDomainRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDomainRequest>(
          UpdateDomainRequest.$_createMessage);
  static UpdateDomainRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get host => $_getSZ(1);
  @$pb.TagNumber(2)
  set host($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHost() => $_has(1);
  @$pb.TagNumber(2)
  void clearHost() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get https => $_getBF(2);
  @$pb.TagNumber(3)
  set https($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHttps() => $_has(2);
  @$pb.TagNumber(3)
  void clearHttps() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get port => $_getI64(3);
  @$pb.TagNumber(4)
  set port($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPort() => $_has(3);
  @$pb.TagNumber(4)
  void clearPort() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get path => $_getSZ(4);
  @$pb.TagNumber(5)
  set path($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get internalPath => $_getSZ(5);
  @$pb.TagNumber(6)
  set internalPath($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasInternalPath() => $_has(5);
  @$pb.TagNumber(6)
  void clearInternalPath() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get customEntrypoint => $_getSZ(6);
  @$pb.TagNumber(7)
  set customEntrypoint($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCustomEntrypoint() => $_has(6);
  @$pb.TagNumber(7)
  void clearCustomEntrypoint() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get serviceName => $_getSZ(7);
  @$pb.TagNumber(8)
  set serviceName($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasServiceName() => $_has(7);
  @$pb.TagNumber(8)
  void clearServiceName() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get customCertResolver => $_getSZ(8);
  @$pb.TagNumber(9)
  set customCertResolver($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCustomCertResolver() => $_has(8);
  @$pb.TagNumber(9)
  void clearCustomCertResolver() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get stripPath => $_getBF(9);
  @$pb.TagNumber(10)
  set stripPath($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasStripPath() => $_has(9);
  @$pb.TagNumber(10)
  void clearStripPath() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get middlewares => $_getSZ(10);
  @$pb.TagNumber(11)
  set middlewares($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasMiddlewares() => $_has(10);
  @$pb.TagNumber(11)
  void clearMiddlewares() => $_clearField(11);

  @$pb.TagNumber(12)
  DomainCertificateType get certificateType => $_getN(11);
  @$pb.TagNumber(12)
  set certificateType(DomainCertificateType value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasCertificateType() => $_has(11);
  @$pb.TagNumber(12)
  void clearCertificateType() => $_clearField(12);
}

class UpdateDomainResponse extends $pb.GeneratedMessage {
  factory UpdateDomainResponse({
    Domain? domain,
  }) {
    final result = UpdateDomainResponse._();
    if (domain != null) result.domain = domain;
    return result;
  }

  UpdateDomainResponse._();

  factory UpdateDomainResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDomainResponse()..mergeFromBuffer(data, registry);
  factory UpdateDomainResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDomainResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDomainResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDomainResponse.$_createMessage)
    ..aOM<Domain>(1, _omitFieldNames ? '' : 'domain',
        subBuilder: Domain.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDomainResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDomainResponse copyWith(void Function(UpdateDomainResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateDomainResponse))
          as UpdateDomainResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDomainResponse() / UpdateDomainResponse.new instead')
  static UpdateDomainResponse create() => UpdateDomainResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDomainResponse._();
  @$core.override
  UpdateDomainResponse createEmptyInstance() => UpdateDomainResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateDomainResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDomainResponse>(
          UpdateDomainResponse.$_createMessage);
  static UpdateDomainResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Domain get domain => $_getN(0);
  @$pb.TagNumber(1)
  set domain(Domain value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDomain() => $_has(0);
  @$pb.TagNumber(1)
  void clearDomain() => $_clearField(1);
  @$pb.TagNumber(1)
  Domain ensureDomain() => $_ensure(0);
}

class DeleteDomainRequest extends $pb.GeneratedMessage {
  factory DeleteDomainRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteDomainRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteDomainRequest._();

  factory DeleteDomainRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDomainRequest()..mergeFromBuffer(data, registry);
  factory DeleteDomainRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDomainRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteDomainRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteDomainRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDomainRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDomainRequest copyWith(void Function(DeleteDomainRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteDomainRequest))
          as DeleteDomainRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use DeleteDomainRequest() / DeleteDomainRequest.new instead')
  static DeleteDomainRequest create() => DeleteDomainRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteDomainRequest._();
  @$core.override
  DeleteDomainRequest createEmptyInstance() => DeleteDomainRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteDomainRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteDomainRequest>(
          DeleteDomainRequest.$_createMessage);
  static DeleteDomainRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Resolves the host's A/AAAA records (CNAME chains are followed by the resolver) and compares them with the target server's IP. server_id absent means the panel's own machine.
class ValidateDomainDnsRequest extends $pb.GeneratedMessage {
  factory ValidateDomainDnsRequest({
    $core.String? host,
    $fixnum.Int64? serverId,
  }) {
    final result = ValidateDomainDnsRequest._();
    if (host != null) result.host = host;
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  ValidateDomainDnsRequest._();

  factory ValidateDomainDnsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidateDomainDnsRequest()..mergeFromBuffer(data, registry);
  factory ValidateDomainDnsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidateDomainDnsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ValidateDomainDnsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ValidateDomainDnsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'host')
    ..aInt64(2, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidateDomainDnsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidateDomainDnsRequest copyWith(
          void Function(ValidateDomainDnsRequest) updates) =>
      super.copyWith((message) => updates(message as ValidateDomainDnsRequest))
          as ValidateDomainDnsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ValidateDomainDnsRequest() / ValidateDomainDnsRequest.new instead')
  static ValidateDomainDnsRequest create() => ValidateDomainDnsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ValidateDomainDnsRequest._();
  @$core.override
  ValidateDomainDnsRequest createEmptyInstance() =>
      ValidateDomainDnsRequest._();
  @$core.pragma('dart2js:noInline')
  static ValidateDomainDnsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ValidateDomainDnsRequest>(
          ValidateDomainDnsRequest.$_createMessage);
  static ValidateDomainDnsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get host => $_getSZ(0);
  @$pb.TagNumber(1)
  set host($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearHost() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get serverId => $_getI64(1);
  @$pb.TagNumber(2)
  set serverId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerId() => $_clearField(2);
}

class ValidateDomainDnsResponse extends $pb.GeneratedMessage {
  factory ValidateDomainDnsResponse({
    $core.bool? isValid,
    $core.Iterable<$core.String>? resolvedIps,
    $core.Iterable<$core.String>? expectedIps,
    $core.String? error,
  }) {
    final result = ValidateDomainDnsResponse._();
    if (isValid != null) result.isValid = isValid;
    if (resolvedIps != null) result.resolvedIps.addAll(resolvedIps);
    if (expectedIps != null) result.expectedIps.addAll(expectedIps);
    if (error != null) result.error = error;
    return result;
  }

  ValidateDomainDnsResponse._();

  factory ValidateDomainDnsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidateDomainDnsResponse()..mergeFromBuffer(data, registry);
  factory ValidateDomainDnsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ValidateDomainDnsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ValidateDomainDnsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ValidateDomainDnsResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'isValid')
    ..pPS(2, _omitFieldNames ? '' : 'resolvedIps')
    ..pPS(3, _omitFieldNames ? '' : 'expectedIps')
    ..aOS(4, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidateDomainDnsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ValidateDomainDnsResponse copyWith(
          void Function(ValidateDomainDnsResponse) updates) =>
      super.copyWith((message) => updates(message as ValidateDomainDnsResponse))
          as ValidateDomainDnsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ValidateDomainDnsResponse() / ValidateDomainDnsResponse.new instead')
  static ValidateDomainDnsResponse create() => ValidateDomainDnsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ValidateDomainDnsResponse._();
  @$core.override
  ValidateDomainDnsResponse createEmptyInstance() =>
      ValidateDomainDnsResponse._();
  @$core.pragma('dart2js:noInline')
  static ValidateDomainDnsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ValidateDomainDnsResponse>(
          ValidateDomainDnsResponse.$_createMessage);
  static ValidateDomainDnsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get isValid => $_getBF(0);
  @$pb.TagNumber(1)
  set isValid($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIsValid() => $_has(0);
  @$pb.TagNumber(1)
  void clearIsValid() => $_clearField(1);

  /// Public IPs the host currently resolves to.
  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get resolvedIps => $_getList(1);

  /// IPs the host is expected to point at; empty when the server has none configured.
  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get expectedIps => $_getList(2);

  /// Human-readable failure reason; absent when is_valid.
  @$pb.TagNumber(4)
  $core.String get error => $_getSZ(3);
  @$pb.TagNumber(4)
  set error($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasError() => $_has(3);
  @$pb.TagNumber(4)
  void clearError() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

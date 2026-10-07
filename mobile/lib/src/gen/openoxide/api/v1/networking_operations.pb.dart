// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/networking_operations.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'networking_operations.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'networking_operations.pbenum.dart';

class NetworkingDomainDiagnostic extends $pb.GeneratedMessage {
  factory NetworkingDomainDiagnostic({
    $core.String? host,
    $core.Iterable<$core.String>? resolvedAddresses,
    $core.bool? dnsOk,
    $core.bool? tcpOk,
    $core.bool? httpOk,
    $core.int? httpStatus,
    $core.String? error,
  }) {
    final result = NetworkingDomainDiagnostic._();
    if (host != null) result.host = host;
    if (resolvedAddresses != null)
      result.resolvedAddresses.addAll(resolvedAddresses);
    if (dnsOk != null) result.dnsOk = dnsOk;
    if (tcpOk != null) result.tcpOk = tcpOk;
    if (httpOk != null) result.httpOk = httpOk;
    if (httpStatus != null) result.httpStatus = httpStatus;
    if (error != null) result.error = error;
    return result;
  }

  NetworkingDomainDiagnostic._();

  factory NetworkingDomainDiagnostic.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDomainDiagnostic()..mergeFromBuffer(data, registry);
  factory NetworkingDomainDiagnostic.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDomainDiagnostic()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingDomainDiagnostic',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingDomainDiagnostic.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'host')
    ..pPS(2, _omitFieldNames ? '' : 'resolvedAddresses')
    ..aOB(3, _omitFieldNames ? '' : 'dnsOk')
    ..aOB(4, _omitFieldNames ? '' : 'tcpOk')
    ..aOB(5, _omitFieldNames ? '' : 'httpOk')
    ..aI(6, _omitFieldNames ? '' : 'httpStatus', fieldType: $pb.PbFieldType.OU3)
    ..aOS(7, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDomainDiagnostic clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDomainDiagnostic copyWith(
          void Function(NetworkingDomainDiagnostic) updates) =>
      super.copyWith(
              (message) => updates(message as NetworkingDomainDiagnostic))
          as NetworkingDomainDiagnostic;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingDomainDiagnostic() / NetworkingDomainDiagnostic.new instead')
  static NetworkingDomainDiagnostic create() => NetworkingDomainDiagnostic._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingDomainDiagnostic._();
  @$core.override
  NetworkingDomainDiagnostic createEmptyInstance() =>
      NetworkingDomainDiagnostic._();
  @$core.pragma('dart2js:noInline')
  static NetworkingDomainDiagnostic getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingDomainDiagnostic>(
          NetworkingDomainDiagnostic.$_createMessage);
  static NetworkingDomainDiagnostic? _defaultInstance;

  /// Lower-cased host that was probed.
  @$pb.TagNumber(1)
  $core.String get host => $_getSZ(0);
  @$pb.TagNumber(1)
  set host($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearHost() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get resolvedAddresses => $_getList(1);

  @$pb.TagNumber(3)
  $core.bool get dnsOk => $_getBF(2);
  @$pb.TagNumber(3)
  set dnsOk($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDnsOk() => $_has(2);
  @$pb.TagNumber(3)
  void clearDnsOk() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get tcpOk => $_getBF(3);
  @$pb.TagNumber(4)
  set tcpOk($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTcpOk() => $_has(3);
  @$pb.TagNumber(4)
  void clearTcpOk() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get httpOk => $_getBF(4);
  @$pb.TagNumber(5)
  set httpOk($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHttpOk() => $_has(4);
  @$pb.TagNumber(5)
  void clearHttpOk() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.int get httpStatus => $_getIZ(5);
  @$pb.TagNumber(6)
  set httpStatus($core.int value) => $_setUnsignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHttpStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearHttpStatus() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get error => $_getSZ(6);
  @$pb.TagNumber(7)
  set error($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasError() => $_has(6);
  @$pb.TagNumber(7)
  void clearError() => $_clearField(7);
}

class NetworkingRootNetworkStatus extends $pb.GeneratedMessage {
  factory NetworkingRootNetworkStatus({
    $core.String? name,
    $core.bool? exists,
    $core.bool? healthy,
    $core.bool? repaired,
    $core.String? driver,
    $core.String? scope,
    $core.bool? attachable,
    $fixnum.Int64? connectedResources,
    $core.String? issue,
  }) {
    final result = NetworkingRootNetworkStatus._();
    if (name != null) result.name = name;
    if (exists != null) result.exists = exists;
    if (healthy != null) result.healthy = healthy;
    if (repaired != null) result.repaired = repaired;
    if (driver != null) result.driver = driver;
    if (scope != null) result.scope = scope;
    if (attachable != null) result.attachable = attachable;
    if (connectedResources != null)
      result.connectedResources = connectedResources;
    if (issue != null) result.issue = issue;
    return result;
  }

  NetworkingRootNetworkStatus._();

  factory NetworkingRootNetworkStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRootNetworkStatus()..mergeFromBuffer(data, registry);
  factory NetworkingRootNetworkStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRootNetworkStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingRootNetworkStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingRootNetworkStatus.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOB(2, _omitFieldNames ? '' : 'exists')
    ..aOB(3, _omitFieldNames ? '' : 'healthy')
    ..aOB(4, _omitFieldNames ? '' : 'repaired')
    ..aOS(5, _omitFieldNames ? '' : 'driver')
    ..aOS(6, _omitFieldNames ? '' : 'scope')
    ..aOB(7, _omitFieldNames ? '' : 'attachable')
    ..aInt64(8, _omitFieldNames ? '' : 'connectedResources')
    ..aOS(9, _omitFieldNames ? '' : 'issue')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRootNetworkStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRootNetworkStatus copyWith(
          void Function(NetworkingRootNetworkStatus) updates) =>
      super.copyWith(
              (message) => updates(message as NetworkingRootNetworkStatus))
          as NetworkingRootNetworkStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingRootNetworkStatus() / NetworkingRootNetworkStatus.new instead')
  static NetworkingRootNetworkStatus create() =>
      NetworkingRootNetworkStatus._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingRootNetworkStatus._();
  @$core.override
  NetworkingRootNetworkStatus createEmptyInstance() =>
      NetworkingRootNetworkStatus._();
  @$core.pragma('dart2js:noInline')
  static NetworkingRootNetworkStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingRootNetworkStatus>(
          NetworkingRootNetworkStatus.$_createMessage);
  static NetworkingRootNetworkStatus? _defaultInstance;

  /// Overlay network name (the `openoxide` root network).
  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get exists => $_getBF(1);
  @$pb.TagNumber(2)
  set exists($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExists() => $_has(1);
  @$pb.TagNumber(2)
  void clearExists() => $_clearField(2);

  /// Encrypted attachable overlay/swarm network with the expected name.
  @$pb.TagNumber(3)
  $core.bool get healthy => $_getBF(2);
  @$pb.TagNumber(3)
  set healthy($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHealthy() => $_has(2);
  @$pb.TagNumber(3)
  void clearHealthy() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get repaired => $_getBF(3);
  @$pb.TagNumber(4)
  set repaired($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRepaired() => $_has(3);
  @$pb.TagNumber(4)
  void clearRepaired() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get driver => $_getSZ(4);
  @$pb.TagNumber(5)
  set driver($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDriver() => $_has(4);
  @$pb.TagNumber(5)
  void clearDriver() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get scope => $_getSZ(5);
  @$pb.TagNumber(6)
  set scope($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasScope() => $_has(5);
  @$pb.TagNumber(6)
  void clearScope() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get attachable => $_getBF(6);
  @$pb.TagNumber(7)
  set attachable($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAttachable() => $_has(6);
  @$pb.TagNumber(7)
  void clearAttachable() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get connectedResources => $_getI64(7);
  @$pb.TagNumber(8)
  set connectedResources($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasConnectedResources() => $_has(7);
  @$pb.TagNumber(8)
  void clearConnectedResources() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get issue => $_getSZ(8);
  @$pb.TagNumber(9)
  set issue($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasIssue() => $_has(8);
  @$pb.TagNumber(9)
  void clearIssue() => $_clearField(9);
}

class NetworkingPurgeCdnRequest extends $pb.GeneratedMessage {
  factory NetworkingPurgeCdnRequest({
    NetworkingCdnProvider? provider,
    $core.String? apiToken,
    $core.String? zoneId,
    $core.String? serviceId,
    $fixnum.Int64? pullZoneId,
    $core.bool? purgeAll,
    $core.Iterable<$core.String>? urls,
  }) {
    final result = NetworkingPurgeCdnRequest._();
    if (provider != null) result.provider = provider;
    if (apiToken != null) result.apiToken = apiToken;
    if (zoneId != null) result.zoneId = zoneId;
    if (serviceId != null) result.serviceId = serviceId;
    if (pullZoneId != null) result.pullZoneId = pullZoneId;
    if (purgeAll != null) result.purgeAll = purgeAll;
    if (urls != null) result.urls.addAll(urls);
    return result;
  }

  NetworkingPurgeCdnRequest._();

  factory NetworkingPurgeCdnRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingPurgeCdnRequest()..mergeFromBuffer(data, registry);
  factory NetworkingPurgeCdnRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingPurgeCdnRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingPurgeCdnRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingPurgeCdnRequest.$_createMessage)
    ..aE<NetworkingCdnProvider>(1, _omitFieldNames ? '' : 'provider',
        enumValues: NetworkingCdnProvider.values)
    ..aOS(2, _omitFieldNames ? '' : 'apiToken')
    ..aOS(3, _omitFieldNames ? '' : 'zoneId')
    ..aOS(4, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(5, _omitFieldNames ? '' : 'pullZoneId')
    ..aOB(6, _omitFieldNames ? '' : 'purgeAll')
    ..pPS(7, _omitFieldNames ? '' : 'urls')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingPurgeCdnRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingPurgeCdnRequest copyWith(
          void Function(NetworkingPurgeCdnRequest) updates) =>
      super.copyWith((message) => updates(message as NetworkingPurgeCdnRequest))
          as NetworkingPurgeCdnRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingPurgeCdnRequest() / NetworkingPurgeCdnRequest.new instead')
  static NetworkingPurgeCdnRequest create() => NetworkingPurgeCdnRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingPurgeCdnRequest._();
  @$core.override
  NetworkingPurgeCdnRequest createEmptyInstance() =>
      NetworkingPurgeCdnRequest._();
  @$core.pragma('dart2js:noInline')
  static NetworkingPurgeCdnRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingPurgeCdnRequest>(
          NetworkingPurgeCdnRequest.$_createMessage);
  static NetworkingPurgeCdnRequest? _defaultInstance;

  @$pb.TagNumber(1)
  NetworkingCdnProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(NetworkingCdnProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);

  /// Provider API token; never echoed back in the response.
  @$pb.TagNumber(2)
  $core.String get apiToken => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiToken($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiToken() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiToken() => $_clearField(2);

  /// Cloudflare only.
  @$pb.TagNumber(3)
  $core.String get zoneId => $_getSZ(2);
  @$pb.TagNumber(3)
  set zoneId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasZoneId() => $_has(2);
  @$pb.TagNumber(3)
  void clearZoneId() => $_clearField(3);

  /// Fastly only.
  @$pb.TagNumber(4)
  $core.String get serviceId => $_getSZ(3);
  @$pb.TagNumber(4)
  set serviceId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasServiceId() => $_has(3);
  @$pb.TagNumber(4)
  void clearServiceId() => $_clearField(4);

  /// Bunny only.
  @$pb.TagNumber(5)
  $fixnum.Int64 get pullZoneId => $_getI64(4);
  @$pb.TagNumber(5)
  set pullZoneId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPullZoneId() => $_has(4);
  @$pb.TagNumber(5)
  void clearPullZoneId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get purgeAll => $_getBF(5);
  @$pb.TagNumber(6)
  set purgeAll($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPurgeAll() => $_has(5);
  @$pb.TagNumber(6)
  void clearPurgeAll() => $_clearField(6);

  @$pb.TagNumber(7)
  $pb.PbList<$core.String> get urls => $_getList(6);
}

class NetworkingPurgeCdnResponse extends $pb.GeneratedMessage {
  factory NetworkingPurgeCdnResponse({
    $core.String? provider,
    $core.bool? success,
    $core.int? status,
    $core.String? requestId,
    $core.String? message,
  }) {
    final result = NetworkingPurgeCdnResponse._();
    if (provider != null) result.provider = provider;
    if (success != null) result.success = success;
    if (status != null) result.status = status;
    if (requestId != null) result.requestId = requestId;
    if (message != null) result.message = message;
    return result;
  }

  NetworkingPurgeCdnResponse._();

  factory NetworkingPurgeCdnResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingPurgeCdnResponse()..mergeFromBuffer(data, registry);
  factory NetworkingPurgeCdnResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingPurgeCdnResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingPurgeCdnResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingPurgeCdnResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'provider')
    ..aOB(2, _omitFieldNames ? '' : 'success')
    ..aI(3, _omitFieldNames ? '' : 'status', fieldType: $pb.PbFieldType.OU3)
    ..aOS(4, _omitFieldNames ? '' : 'requestId')
    ..aOS(5, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingPurgeCdnResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingPurgeCdnResponse copyWith(
          void Function(NetworkingPurgeCdnResponse) updates) =>
      super.copyWith(
              (message) => updates(message as NetworkingPurgeCdnResponse))
          as NetworkingPurgeCdnResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingPurgeCdnResponse() / NetworkingPurgeCdnResponse.new instead')
  static NetworkingPurgeCdnResponse create() => NetworkingPurgeCdnResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingPurgeCdnResponse._();
  @$core.override
  NetworkingPurgeCdnResponse createEmptyInstance() =>
      NetworkingPurgeCdnResponse._();
  @$core.pragma('dart2js:noInline')
  static NetworkingPurgeCdnResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingPurgeCdnResponse>(
          NetworkingPurgeCdnResponse.$_createMessage);
  static NetworkingPurgeCdnResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get provider => $_getSZ(0);
  @$pb.TagNumber(1)
  set provider($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => $_clearField(2);

  /// Upstream provider HTTP status code.
  @$pb.TagNumber(3)
  $core.int get status => $_getIZ(2);
  @$pb.TagNumber(3)
  set status($core.int value) => $_setUnsignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get requestId => $_getSZ(3);
  @$pb.TagNumber(4)
  set requestId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRequestId() => $_has(3);
  @$pb.TagNumber(4)
  void clearRequestId() => $_clearField(4);

  /// Truncated provider response body.
  @$pb.TagNumber(5)
  $core.String get message => $_getSZ(4);
  @$pb.TagNumber(5)
  set message($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearMessage() => $_clearField(5);
}

class NetworkingDiagnoseDomainsRequest extends $pb.GeneratedMessage {
  factory NetworkingDiagnoseDomainsRequest({
    $core.String? host,
    $core.int? port,
    $core.bool? https,
    $core.String? path,
  }) {
    final result = NetworkingDiagnoseDomainsRequest._();
    if (host != null) result.host = host;
    if (port != null) result.port = port;
    if (https != null) result.https = https;
    if (path != null) result.path = path;
    return result;
  }

  NetworkingDiagnoseDomainsRequest._();

  factory NetworkingDiagnoseDomainsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseDomainsRequest()..mergeFromBuffer(data, registry);
  factory NetworkingDiagnoseDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingDiagnoseDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingDiagnoseDomainsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'host')
    ..aI(2, _omitFieldNames ? '' : 'port', fieldType: $pb.PbFieldType.OU3)
    ..aOB(3, _omitFieldNames ? '' : 'https')
    ..aOS(4, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseDomainsRequest copyWith(
          void Function(NetworkingDiagnoseDomainsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as NetworkingDiagnoseDomainsRequest))
          as NetworkingDiagnoseDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingDiagnoseDomainsRequest() / NetworkingDiagnoseDomainsRequest.new instead')
  static NetworkingDiagnoseDomainsRequest create() =>
      NetworkingDiagnoseDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingDiagnoseDomainsRequest._();
  @$core.override
  NetworkingDiagnoseDomainsRequest createEmptyInstance() =>
      NetworkingDiagnoseDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static NetworkingDiagnoseDomainsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingDiagnoseDomainsRequest>(
          NetworkingDiagnoseDomainsRequest.$_createMessage);
  static NetworkingDiagnoseDomainsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get host => $_getSZ(0);
  @$pb.TagNumber(1)
  set host($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearHost() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get port => $_getIZ(1);
  @$pb.TagNumber(2)
  set port($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get https => $_getBF(2);
  @$pb.TagNumber(3)
  set https($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHttps() => $_has(2);
  @$pb.TagNumber(3)
  void clearHttps() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get path => $_getSZ(3);
  @$pb.TagNumber(4)
  set path($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearPath() => $_clearField(4);
}

class NetworkingDiagnoseDomainsResponse extends $pb.GeneratedMessage {
  factory NetworkingDiagnoseDomainsResponse({
    NetworkingDomainDiagnostic? diagnostic,
  }) {
    final result = NetworkingDiagnoseDomainsResponse._();
    if (diagnostic != null) result.diagnostic = diagnostic;
    return result;
  }

  NetworkingDiagnoseDomainsResponse._();

  factory NetworkingDiagnoseDomainsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseDomainsResponse()..mergeFromBuffer(data, registry);
  factory NetworkingDiagnoseDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingDiagnoseDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingDiagnoseDomainsResponse.$_createMessage)
    ..aOM<NetworkingDomainDiagnostic>(1, _omitFieldNames ? '' : 'diagnostic',
        subBuilder: NetworkingDomainDiagnostic.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseDomainsResponse copyWith(
          void Function(NetworkingDiagnoseDomainsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as NetworkingDiagnoseDomainsResponse))
          as NetworkingDiagnoseDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingDiagnoseDomainsResponse() / NetworkingDiagnoseDomainsResponse.new instead')
  static NetworkingDiagnoseDomainsResponse create() =>
      NetworkingDiagnoseDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingDiagnoseDomainsResponse._();
  @$core.override
  NetworkingDiagnoseDomainsResponse createEmptyInstance() =>
      NetworkingDiagnoseDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static NetworkingDiagnoseDomainsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingDiagnoseDomainsResponse>(
          NetworkingDiagnoseDomainsResponse.$_createMessage);
  static NetworkingDiagnoseDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  NetworkingDomainDiagnostic get diagnostic => $_getN(0);
  @$pb.TagNumber(1)
  set diagnostic(NetworkingDomainDiagnostic value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDiagnostic() => $_has(0);
  @$pb.TagNumber(1)
  void clearDiagnostic() => $_clearField(1);
  @$pb.TagNumber(1)
  NetworkingDomainDiagnostic ensureDiagnostic() => $_ensure(0);
}

class NetworkingDiagnoseRootNetworkRequest extends $pb.GeneratedMessage {
  factory NetworkingDiagnoseRootNetworkRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = NetworkingDiagnoseRootNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  NetworkingDiagnoseRootNetworkRequest._();

  factory NetworkingDiagnoseRootNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseRootNetworkRequest()..mergeFromBuffer(data, registry);
  factory NetworkingDiagnoseRootNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseRootNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingDiagnoseRootNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingDiagnoseRootNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseRootNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseRootNetworkRequest copyWith(
          void Function(NetworkingDiagnoseRootNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as NetworkingDiagnoseRootNetworkRequest))
          as NetworkingDiagnoseRootNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingDiagnoseRootNetworkRequest() / NetworkingDiagnoseRootNetworkRequest.new instead')
  static NetworkingDiagnoseRootNetworkRequest create() =>
      NetworkingDiagnoseRootNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingDiagnoseRootNetworkRequest._();
  @$core.override
  NetworkingDiagnoseRootNetworkRequest createEmptyInstance() =>
      NetworkingDiagnoseRootNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static NetworkingDiagnoseRootNetworkRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              NetworkingDiagnoseRootNetworkRequest>(
          NetworkingDiagnoseRootNetworkRequest.$_createMessage);
  static NetworkingDiagnoseRootNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class NetworkingDiagnoseRootNetworkResponse extends $pb.GeneratedMessage {
  factory NetworkingDiagnoseRootNetworkResponse({
    NetworkingRootNetworkStatus? status,
  }) {
    final result = NetworkingDiagnoseRootNetworkResponse._();
    if (status != null) result.status = status;
    return result;
  }

  NetworkingDiagnoseRootNetworkResponse._();

  factory NetworkingDiagnoseRootNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseRootNetworkResponse()..mergeFromBuffer(data, registry);
  factory NetworkingDiagnoseRootNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingDiagnoseRootNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingDiagnoseRootNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          NetworkingDiagnoseRootNetworkResponse.$_createMessage)
    ..aOM<NetworkingRootNetworkStatus>(1, _omitFieldNames ? '' : 'status',
        subBuilder: NetworkingRootNetworkStatus.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseRootNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingDiagnoseRootNetworkResponse copyWith(
          void Function(NetworkingDiagnoseRootNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as NetworkingDiagnoseRootNetworkResponse))
          as NetworkingDiagnoseRootNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingDiagnoseRootNetworkResponse() / NetworkingDiagnoseRootNetworkResponse.new instead')
  static NetworkingDiagnoseRootNetworkResponse create() =>
      NetworkingDiagnoseRootNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingDiagnoseRootNetworkResponse._();
  @$core.override
  NetworkingDiagnoseRootNetworkResponse createEmptyInstance() =>
      NetworkingDiagnoseRootNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static NetworkingDiagnoseRootNetworkResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              NetworkingDiagnoseRootNetworkResponse>(
          NetworkingDiagnoseRootNetworkResponse.$_createMessage);
  static NetworkingDiagnoseRootNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  NetworkingRootNetworkStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(NetworkingRootNetworkStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
  @$pb.TagNumber(1)
  NetworkingRootNetworkStatus ensureStatus() => $_ensure(0);
}

class NetworkingRepairRootNetworkRequest extends $pb.GeneratedMessage {
  factory NetworkingRepairRootNetworkRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = NetworkingRepairRootNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  NetworkingRepairRootNetworkRequest._();

  factory NetworkingRepairRootNetworkRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRepairRootNetworkRequest()..mergeFromBuffer(data, registry);
  factory NetworkingRepairRootNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRepairRootNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingRepairRootNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingRepairRootNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRepairRootNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRepairRootNetworkRequest copyWith(
          void Function(NetworkingRepairRootNetworkRequest) updates) =>
      super.copyWith((message) =>
              updates(message as NetworkingRepairRootNetworkRequest))
          as NetworkingRepairRootNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingRepairRootNetworkRequest() / NetworkingRepairRootNetworkRequest.new instead')
  static NetworkingRepairRootNetworkRequest create() =>
      NetworkingRepairRootNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingRepairRootNetworkRequest._();
  @$core.override
  NetworkingRepairRootNetworkRequest createEmptyInstance() =>
      NetworkingRepairRootNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static NetworkingRepairRootNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NetworkingRepairRootNetworkRequest>(
          NetworkingRepairRootNetworkRequest.$_createMessage);
  static NetworkingRepairRootNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class NetworkingRepairRootNetworkResponse extends $pb.GeneratedMessage {
  factory NetworkingRepairRootNetworkResponse({
    NetworkingRootNetworkStatus? status,
  }) {
    final result = NetworkingRepairRootNetworkResponse._();
    if (status != null) result.status = status;
    return result;
  }

  NetworkingRepairRootNetworkResponse._();

  factory NetworkingRepairRootNetworkResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRepairRootNetworkResponse()..mergeFromBuffer(data, registry);
  factory NetworkingRepairRootNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NetworkingRepairRootNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NetworkingRepairRootNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: NetworkingRepairRootNetworkResponse.$_createMessage)
    ..aOM<NetworkingRootNetworkStatus>(1, _omitFieldNames ? '' : 'status',
        subBuilder: NetworkingRootNetworkStatus.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRepairRootNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NetworkingRepairRootNetworkResponse copyWith(
          void Function(NetworkingRepairRootNetworkResponse) updates) =>
      super.copyWith((message) =>
              updates(message as NetworkingRepairRootNetworkResponse))
          as NetworkingRepairRootNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use NetworkingRepairRootNetworkResponse() / NetworkingRepairRootNetworkResponse.new instead')
  static NetworkingRepairRootNetworkResponse create() =>
      NetworkingRepairRootNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      NetworkingRepairRootNetworkResponse._();
  @$core.override
  NetworkingRepairRootNetworkResponse createEmptyInstance() =>
      NetworkingRepairRootNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static NetworkingRepairRootNetworkResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              NetworkingRepairRootNetworkResponse>(
          NetworkingRepairRootNetworkResponse.$_createMessage);
  static NetworkingRepairRootNetworkResponse? _defaultInstance;

  @$pb.TagNumber(1)
  NetworkingRootNetworkStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(NetworkingRootNetworkStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
  @$pb.TagNumber(1)
  NetworkingRootNetworkStatus ensureStatus() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

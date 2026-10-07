// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/certificate.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'certificate.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'certificate.pbenum.dart';

class Certificate extends $pb.GeneratedMessage {
  factory Certificate({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? certificateData,
    $core.bool? hasPrivateKey,
    $core.String? certificatePath,
    $core.bool? autoRenew,
    $core.String? serverId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Certificate._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (certificateData != null) result.certificateData = certificateData;
    if (hasPrivateKey != null) result.hasPrivateKey = hasPrivateKey;
    if (certificatePath != null) result.certificatePath = certificatePath;
    if (autoRenew != null) result.autoRenew = autoRenew;
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Certificate._();

  factory Certificate.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Certificate()..mergeFromBuffer(data, registry);
  factory Certificate.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Certificate()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Certificate',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Certificate.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'certificateData')
    ..aOB(4, _omitFieldNames ? '' : 'hasPrivateKey')
    ..aOS(5, _omitFieldNames ? '' : 'certificatePath')
    ..aOB(6, _omitFieldNames ? '' : 'autoRenew')
    ..aOS(7, _omitFieldNames ? '' : 'serverId')
    ..aInt64(8, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Certificate clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Certificate copyWith(void Function(Certificate) updates) =>
      super.copyWith((message) => updates(message as Certificate))
          as Certificate;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Certificate() / Certificate.new instead')
  static Certificate create() => Certificate._();
  static $pb.GeneratedMessage $_createMessage() => Certificate._();
  @$core.override
  Certificate createEmptyInstance() => Certificate._();
  @$core.pragma('dart2js:noInline')
  static Certificate getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Certificate>(
          Certificate.$_createMessage);
  static Certificate? _defaultInstance;

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

  /// PEM certificate chain.
  @$pb.TagNumber(3)
  $core.String get certificateData => $_getSZ(2);
  @$pb.TagNumber(3)
  set certificateData($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCertificateData() => $_has(2);
  @$pb.TagNumber(3)
  void clearCertificateData() => $_clearField(3);

  /// Private key material is never echoed back; presence only.
  @$pb.TagNumber(4)
  $core.bool get hasPrivateKey => $_getBF(3);
  @$pb.TagNumber(4)
  set hasPrivateKey($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasHasPrivateKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearHasPrivateKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get certificatePath => $_getSZ(4);
  @$pb.TagNumber(5)
  set certificatePath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCertificatePath() => $_has(4);
  @$pb.TagNumber(5)
  void clearCertificatePath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get autoRenew => $_getBF(5);
  @$pb.TagNumber(6)
  set autoRenew($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAutoRenew() => $_has(5);
  @$pb.TagNumber(6)
  void clearAutoRenew() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serverId => $_getSZ(6);
  @$pb.TagNumber(7)
  set serverId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServerId() => $_has(6);
  @$pb.TagNumber(7)
  void clearServerId() => $_clearField(7);

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
}

class CertificateRenewal extends $pb.GeneratedMessage {
  factory CertificateRenewal({
    $fixnum.Int64? id,
    $fixnum.Int64? certificateId,
    $fixnum.Int64? organizationId,
    CertificateRenewalStatus? status,
    $fixnum.Int64? previousExpiresAt,
    $fixnum.Int64? newExpiresAt,
    $core.String? error,
    $fixnum.Int64? startedAt,
    $fixnum.Int64? finishedAt,
  }) {
    final result = CertificateRenewal._();
    if (id != null) result.id = id;
    if (certificateId != null) result.certificateId = certificateId;
    if (organizationId != null) result.organizationId = organizationId;
    if (status != null) result.status = status;
    if (previousExpiresAt != null) result.previousExpiresAt = previousExpiresAt;
    if (newExpiresAt != null) result.newExpiresAt = newExpiresAt;
    if (error != null) result.error = error;
    if (startedAt != null) result.startedAt = startedAt;
    if (finishedAt != null) result.finishedAt = finishedAt;
    return result;
  }

  CertificateRenewal._();

  factory CertificateRenewal.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CertificateRenewal()..mergeFromBuffer(data, registry);
  factory CertificateRenewal.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CertificateRenewal()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CertificateRenewal',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CertificateRenewal.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'certificateId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aE<CertificateRenewalStatus>(4, _omitFieldNames ? '' : 'status',
        enumValues: CertificateRenewalStatus.values)
    ..aInt64(5, _omitFieldNames ? '' : 'previousExpiresAt')
    ..aInt64(6, _omitFieldNames ? '' : 'newExpiresAt')
    ..aOS(7, _omitFieldNames ? '' : 'error')
    ..aInt64(8, _omitFieldNames ? '' : 'startedAt')
    ..aInt64(9, _omitFieldNames ? '' : 'finishedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CertificateRenewal clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CertificateRenewal copyWith(void Function(CertificateRenewal) updates) =>
      super.copyWith((message) => updates(message as CertificateRenewal))
          as CertificateRenewal;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CertificateRenewal() / CertificateRenewal.new instead')
  static CertificateRenewal create() => CertificateRenewal._();
  static $pb.GeneratedMessage $_createMessage() => CertificateRenewal._();
  @$core.override
  CertificateRenewal createEmptyInstance() => CertificateRenewal._();
  @$core.pragma('dart2js:noInline')
  static CertificateRenewal getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CertificateRenewal>(
          CertificateRenewal.$_createMessage);
  static CertificateRenewal? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get certificateId => $_getI64(1);
  @$pb.TagNumber(2)
  set certificateId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCertificateId() => $_has(1);
  @$pb.TagNumber(2)
  void clearCertificateId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  @$pb.TagNumber(4)
  CertificateRenewalStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(CertificateRenewalStatus value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get previousExpiresAt => $_getI64(4);
  @$pb.TagNumber(5)
  set previousExpiresAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPreviousExpiresAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearPreviousExpiresAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get newExpiresAt => $_getI64(5);
  @$pb.TagNumber(6)
  set newExpiresAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasNewExpiresAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearNewExpiresAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get error => $_getSZ(6);
  @$pb.TagNumber(7)
  set error($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasError() => $_has(6);
  @$pb.TagNumber(7)
  void clearError() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get startedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set startedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasStartedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearStartedAt() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get finishedAt => $_getI64(8);
  @$pb.TagNumber(9)
  set finishedAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasFinishedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearFinishedAt() => $_clearField(9);
}

class CertificateDependencies extends $pb.GeneratedMessage {
  factory CertificateDependencies({
    $fixnum.Int64? renewals,
    $fixnum.Int64? runningRenewals,
  }) {
    final result = CertificateDependencies._();
    if (renewals != null) result.renewals = renewals;
    if (runningRenewals != null) result.runningRenewals = runningRenewals;
    return result;
  }

  CertificateDependencies._();

  factory CertificateDependencies.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CertificateDependencies()..mergeFromBuffer(data, registry);
  factory CertificateDependencies.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CertificateDependencies()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CertificateDependencies',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CertificateDependencies.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'renewals')
    ..aInt64(2, _omitFieldNames ? '' : 'runningRenewals')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CertificateDependencies clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CertificateDependencies copyWith(
          void Function(CertificateDependencies) updates) =>
      super.copyWith((message) => updates(message as CertificateDependencies))
          as CertificateDependencies;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CertificateDependencies() / CertificateDependencies.new instead')
  static CertificateDependencies create() => CertificateDependencies._();
  static $pb.GeneratedMessage $_createMessage() => CertificateDependencies._();
  @$core.override
  CertificateDependencies createEmptyInstance() => CertificateDependencies._();
  @$core.pragma('dart2js:noInline')
  static CertificateDependencies getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CertificateDependencies>(
          CertificateDependencies.$_createMessage);
  static CertificateDependencies? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get renewals => $_getI64(0);
  @$pb.TagNumber(1)
  set renewals($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRenewals() => $_has(0);
  @$pb.TagNumber(1)
  void clearRenewals() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get runningRenewals => $_getI64(1);
  @$pb.TagNumber(2)
  set runningRenewals($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRunningRenewals() => $_has(1);
  @$pb.TagNumber(2)
  void clearRunningRenewals() => $_clearField(2);
}

class ListCertificatesRequest extends $pb.GeneratedMessage {
  factory ListCertificatesRequest() => ListCertificatesRequest._();

  ListCertificatesRequest._();

  factory ListCertificatesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificatesRequest()..mergeFromBuffer(data, registry);
  factory ListCertificatesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificatesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificatesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificatesRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificatesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificatesRequest copyWith(
          void Function(ListCertificatesRequest) updates) =>
      super.copyWith((message) => updates(message as ListCertificatesRequest))
          as ListCertificatesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificatesRequest() / ListCertificatesRequest.new instead')
  static ListCertificatesRequest create() => ListCertificatesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListCertificatesRequest._();
  @$core.override
  ListCertificatesRequest createEmptyInstance() => ListCertificatesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListCertificatesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListCertificatesRequest>(
          ListCertificatesRequest.$_createMessage);
  static ListCertificatesRequest? _defaultInstance;
}

class ListCertificatesResponse extends $pb.GeneratedMessage {
  factory ListCertificatesResponse({
    $core.Iterable<Certificate>? certificates,
  }) {
    final result = ListCertificatesResponse._();
    if (certificates != null) result.certificates.addAll(certificates);
    return result;
  }

  ListCertificatesResponse._();

  factory ListCertificatesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificatesResponse()..mergeFromBuffer(data, registry);
  factory ListCertificatesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificatesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificatesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificatesResponse.$_createMessage)
    ..pPM<Certificate>(1, _omitFieldNames ? '' : 'certificates',
        subBuilder: Certificate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificatesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificatesResponse copyWith(
          void Function(ListCertificatesResponse) updates) =>
      super.copyWith((message) => updates(message as ListCertificatesResponse))
          as ListCertificatesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificatesResponse() / ListCertificatesResponse.new instead')
  static ListCertificatesResponse create() => ListCertificatesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListCertificatesResponse._();
  @$core.override
  ListCertificatesResponse createEmptyInstance() =>
      ListCertificatesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListCertificatesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListCertificatesResponse>(
          ListCertificatesResponse.$_createMessage);
  static ListCertificatesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Certificate> get certificates => $_getList(0);
}

class GetCertificateRequest extends $pb.GeneratedMessage {
  factory GetCertificateRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetCertificateRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetCertificateRequest._();

  factory GetCertificateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetCertificateRequest()..mergeFromBuffer(data, registry);
  factory GetCertificateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetCertificateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetCertificateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetCertificateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetCertificateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetCertificateRequest copyWith(
          void Function(GetCertificateRequest) updates) =>
      super.copyWith((message) => updates(message as GetCertificateRequest))
          as GetCertificateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetCertificateRequest() / GetCertificateRequest.new instead')
  static GetCertificateRequest create() => GetCertificateRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetCertificateRequest._();
  @$core.override
  GetCertificateRequest createEmptyInstance() => GetCertificateRequest._();
  @$core.pragma('dart2js:noInline')
  static GetCertificateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetCertificateRequest>(
          GetCertificateRequest.$_createMessage);
  static GetCertificateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetCertificateResponse extends $pb.GeneratedMessage {
  factory GetCertificateResponse({
    Certificate? certificate,
  }) {
    final result = GetCertificateResponse._();
    if (certificate != null) result.certificate = certificate;
    return result;
  }

  GetCertificateResponse._();

  factory GetCertificateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetCertificateResponse()..mergeFromBuffer(data, registry);
  factory GetCertificateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetCertificateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetCertificateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetCertificateResponse.$_createMessage)
    ..aOM<Certificate>(1, _omitFieldNames ? '' : 'certificate',
        subBuilder: Certificate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetCertificateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetCertificateResponse copyWith(
          void Function(GetCertificateResponse) updates) =>
      super.copyWith((message) => updates(message as GetCertificateResponse))
          as GetCertificateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetCertificateResponse() / GetCertificateResponse.new instead')
  static GetCertificateResponse create() => GetCertificateResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetCertificateResponse._();
  @$core.override
  GetCertificateResponse createEmptyInstance() => GetCertificateResponse._();
  @$core.pragma('dart2js:noInline')
  static GetCertificateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetCertificateResponse>(
          GetCertificateResponse.$_createMessage);
  static GetCertificateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Certificate get certificate => $_getN(0);
  @$pb.TagNumber(1)
  set certificate(Certificate value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificate() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificate() => $_clearField(1);
  @$pb.TagNumber(1)
  Certificate ensureCertificate() => $_ensure(0);
}

class CreateCertificateRequest extends $pb.GeneratedMessage {
  factory CreateCertificateRequest({
    $core.String? name,
    $core.String? certificateData,
    $core.String? privateKey,
    $core.String? certificatePath,
    $core.bool? autoRenew,
    $core.String? serverId,
  }) {
    final result = CreateCertificateRequest._();
    if (name != null) result.name = name;
    if (certificateData != null) result.certificateData = certificateData;
    if (privateKey != null) result.privateKey = privateKey;
    if (certificatePath != null) result.certificatePath = certificatePath;
    if (autoRenew != null) result.autoRenew = autoRenew;
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  CreateCertificateRequest._();

  factory CreateCertificateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateCertificateRequest()..mergeFromBuffer(data, registry);
  factory CreateCertificateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateCertificateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateCertificateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateCertificateRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'certificateData')
    ..aOS(3, _omitFieldNames ? '' : 'privateKey')
    ..aOS(4, _omitFieldNames ? '' : 'certificatePath')
    ..aOB(5, _omitFieldNames ? '' : 'autoRenew')
    ..aOS(6, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateCertificateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateCertificateRequest copyWith(
          void Function(CreateCertificateRequest) updates) =>
      super.copyWith((message) => updates(message as CreateCertificateRequest))
          as CreateCertificateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateCertificateRequest() / CreateCertificateRequest.new instead')
  static CreateCertificateRequest create() => CreateCertificateRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateCertificateRequest._();
  @$core.override
  CreateCertificateRequest createEmptyInstance() =>
      CreateCertificateRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateCertificateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateCertificateRequest>(
          CreateCertificateRequest.$_createMessage);
  static CreateCertificateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get certificateData => $_getSZ(1);
  @$pb.TagNumber(2)
  set certificateData($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCertificateData() => $_has(1);
  @$pb.TagNumber(2)
  void clearCertificateData() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get privateKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set privateKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPrivateKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearPrivateKey() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get certificatePath => $_getSZ(3);
  @$pb.TagNumber(4)
  set certificatePath($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCertificatePath() => $_has(3);
  @$pb.TagNumber(4)
  void clearCertificatePath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get autoRenew => $_getBF(4);
  @$pb.TagNumber(5)
  set autoRenew($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAutoRenew() => $_has(4);
  @$pb.TagNumber(5)
  void clearAutoRenew() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serverId => $_getSZ(5);
  @$pb.TagNumber(6)
  set serverId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServerId() => $_has(5);
  @$pb.TagNumber(6)
  void clearServerId() => $_clearField(6);
}

class CreateCertificateResponse extends $pb.GeneratedMessage {
  factory CreateCertificateResponse({
    Certificate? certificate,
  }) {
    final result = CreateCertificateResponse._();
    if (certificate != null) result.certificate = certificate;
    return result;
  }

  CreateCertificateResponse._();

  factory CreateCertificateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateCertificateResponse()..mergeFromBuffer(data, registry);
  factory CreateCertificateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateCertificateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateCertificateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateCertificateResponse.$_createMessage)
    ..aOM<Certificate>(1, _omitFieldNames ? '' : 'certificate',
        subBuilder: Certificate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateCertificateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateCertificateResponse copyWith(
          void Function(CreateCertificateResponse) updates) =>
      super.copyWith((message) => updates(message as CreateCertificateResponse))
          as CreateCertificateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateCertificateResponse() / CreateCertificateResponse.new instead')
  static CreateCertificateResponse create() => CreateCertificateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateCertificateResponse._();
  @$core.override
  CreateCertificateResponse createEmptyInstance() =>
      CreateCertificateResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateCertificateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateCertificateResponse>(
          CreateCertificateResponse.$_createMessage);
  static CreateCertificateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Certificate get certificate => $_getN(0);
  @$pb.TagNumber(1)
  set certificate(Certificate value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificate() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificate() => $_clearField(1);
  @$pb.TagNumber(1)
  Certificate ensureCertificate() => $_ensure(0);
}

/// An absent field keeps its stored value; `server_id` present-but-empty detaches the server.
class UpdateCertificateRequest extends $pb.GeneratedMessage {
  factory UpdateCertificateRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? certificateData,
    $core.String? privateKey,
    $core.String? certificatePath,
    $core.bool? autoRenew,
    $core.String? serverId,
  }) {
    final result = UpdateCertificateRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (certificateData != null) result.certificateData = certificateData;
    if (privateKey != null) result.privateKey = privateKey;
    if (certificatePath != null) result.certificatePath = certificatePath;
    if (autoRenew != null) result.autoRenew = autoRenew;
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  UpdateCertificateRequest._();

  factory UpdateCertificateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateCertificateRequest()..mergeFromBuffer(data, registry);
  factory UpdateCertificateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateCertificateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateCertificateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateCertificateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'certificateData')
    ..aOS(4, _omitFieldNames ? '' : 'privateKey')
    ..aOS(5, _omitFieldNames ? '' : 'certificatePath')
    ..aOB(6, _omitFieldNames ? '' : 'autoRenew')
    ..aOS(7, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateCertificateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateCertificateRequest copyWith(
          void Function(UpdateCertificateRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateCertificateRequest))
          as UpdateCertificateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateCertificateRequest() / UpdateCertificateRequest.new instead')
  static UpdateCertificateRequest create() => UpdateCertificateRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateCertificateRequest._();
  @$core.override
  UpdateCertificateRequest createEmptyInstance() =>
      UpdateCertificateRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateCertificateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateCertificateRequest>(
          UpdateCertificateRequest.$_createMessage);
  static UpdateCertificateRequest? _defaultInstance;

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
  $core.String get certificateData => $_getSZ(2);
  @$pb.TagNumber(3)
  set certificateData($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCertificateData() => $_has(2);
  @$pb.TagNumber(3)
  void clearCertificateData() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get privateKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set privateKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivateKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivateKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get certificatePath => $_getSZ(4);
  @$pb.TagNumber(5)
  set certificatePath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCertificatePath() => $_has(4);
  @$pb.TagNumber(5)
  void clearCertificatePath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get autoRenew => $_getBF(5);
  @$pb.TagNumber(6)
  set autoRenew($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAutoRenew() => $_has(5);
  @$pb.TagNumber(6)
  void clearAutoRenew() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serverId => $_getSZ(6);
  @$pb.TagNumber(7)
  set serverId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServerId() => $_has(6);
  @$pb.TagNumber(7)
  void clearServerId() => $_clearField(7);
}

class UpdateCertificateResponse extends $pb.GeneratedMessage {
  factory UpdateCertificateResponse({
    Certificate? certificate,
  }) {
    final result = UpdateCertificateResponse._();
    if (certificate != null) result.certificate = certificate;
    return result;
  }

  UpdateCertificateResponse._();

  factory UpdateCertificateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateCertificateResponse()..mergeFromBuffer(data, registry);
  factory UpdateCertificateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateCertificateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateCertificateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateCertificateResponse.$_createMessage)
    ..aOM<Certificate>(1, _omitFieldNames ? '' : 'certificate',
        subBuilder: Certificate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateCertificateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateCertificateResponse copyWith(
          void Function(UpdateCertificateResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateCertificateResponse))
          as UpdateCertificateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateCertificateResponse() / UpdateCertificateResponse.new instead')
  static UpdateCertificateResponse create() => UpdateCertificateResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateCertificateResponse._();
  @$core.override
  UpdateCertificateResponse createEmptyInstance() =>
      UpdateCertificateResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateCertificateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateCertificateResponse>(
          UpdateCertificateResponse.$_createMessage);
  static UpdateCertificateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Certificate get certificate => $_getN(0);
  @$pb.TagNumber(1)
  set certificate(Certificate value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificate() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificate() => $_clearField(1);
  @$pb.TagNumber(1)
  Certificate ensureCertificate() => $_ensure(0);
}

class DeleteCertificateRequest extends $pb.GeneratedMessage {
  factory DeleteCertificateRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteCertificateRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteCertificateRequest._();

  factory DeleteCertificateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteCertificateRequest()..mergeFromBuffer(data, registry);
  factory DeleteCertificateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteCertificateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteCertificateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteCertificateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteCertificateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteCertificateRequest copyWith(
          void Function(DeleteCertificateRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteCertificateRequest))
          as DeleteCertificateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteCertificateRequest() / DeleteCertificateRequest.new instead')
  static DeleteCertificateRequest create() => DeleteCertificateRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteCertificateRequest._();
  @$core.override
  DeleteCertificateRequest createEmptyInstance() =>
      DeleteCertificateRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteCertificateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteCertificateRequest>(
          DeleteCertificateRequest.$_createMessage);
  static DeleteCertificateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListCertificateDependenciesRequest extends $pb.GeneratedMessage {
  factory ListCertificateDependenciesRequest({
    $fixnum.Int64? certificateId,
  }) {
    final result = ListCertificateDependenciesRequest._();
    if (certificateId != null) result.certificateId = certificateId;
    return result;
  }

  ListCertificateDependenciesRequest._();

  factory ListCertificateDependenciesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateDependenciesRequest()..mergeFromBuffer(data, registry);
  factory ListCertificateDependenciesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateDependenciesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificateDependenciesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificateDependenciesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'certificateId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateDependenciesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateDependenciesRequest copyWith(
          void Function(ListCertificateDependenciesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ListCertificateDependenciesRequest))
          as ListCertificateDependenciesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificateDependenciesRequest() / ListCertificateDependenciesRequest.new instead')
  static ListCertificateDependenciesRequest create() =>
      ListCertificateDependenciesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListCertificateDependenciesRequest._();
  @$core.override
  ListCertificateDependenciesRequest createEmptyInstance() =>
      ListCertificateDependenciesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListCertificateDependenciesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListCertificateDependenciesRequest>(
          ListCertificateDependenciesRequest.$_createMessage);
  static ListCertificateDependenciesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get certificateId => $_getI64(0);
  @$pb.TagNumber(1)
  set certificateId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificateId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificateId() => $_clearField(1);
}

class ListCertificateDependenciesResponse extends $pb.GeneratedMessage {
  factory ListCertificateDependenciesResponse({
    CertificateDependencies? dependencies,
  }) {
    final result = ListCertificateDependenciesResponse._();
    if (dependencies != null) result.dependencies = dependencies;
    return result;
  }

  ListCertificateDependenciesResponse._();

  factory ListCertificateDependenciesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateDependenciesResponse()..mergeFromBuffer(data, registry);
  factory ListCertificateDependenciesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateDependenciesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificateDependenciesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificateDependenciesResponse.$_createMessage)
    ..aOM<CertificateDependencies>(1, _omitFieldNames ? '' : 'dependencies',
        subBuilder: CertificateDependencies.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateDependenciesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateDependenciesResponse copyWith(
          void Function(ListCertificateDependenciesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ListCertificateDependenciesResponse))
          as ListCertificateDependenciesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificateDependenciesResponse() / ListCertificateDependenciesResponse.new instead')
  static ListCertificateDependenciesResponse create() =>
      ListCertificateDependenciesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListCertificateDependenciesResponse._();
  @$core.override
  ListCertificateDependenciesResponse createEmptyInstance() =>
      ListCertificateDependenciesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListCertificateDependenciesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ListCertificateDependenciesResponse>(
          ListCertificateDependenciesResponse.$_createMessage);
  static ListCertificateDependenciesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  CertificateDependencies get dependencies => $_getN(0);
  @$pb.TagNumber(1)
  set dependencies(CertificateDependencies value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDependencies() => $_has(0);
  @$pb.TagNumber(1)
  void clearDependencies() => $_clearField(1);
  @$pb.TagNumber(1)
  CertificateDependencies ensureDependencies() => $_ensure(0);
}

class RenewCertificateRequest extends $pb.GeneratedMessage {
  factory RenewCertificateRequest({
    $fixnum.Int64? id,
    $core.String? certificateData,
    $core.String? privateKey,
  }) {
    final result = RenewCertificateRequest._();
    if (id != null) result.id = id;
    if (certificateData != null) result.certificateData = certificateData;
    if (privateKey != null) result.privateKey = privateKey;
    return result;
  }

  RenewCertificateRequest._();

  factory RenewCertificateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RenewCertificateRequest()..mergeFromBuffer(data, registry);
  factory RenewCertificateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RenewCertificateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RenewCertificateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RenewCertificateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'certificateData')
    ..aOS(3, _omitFieldNames ? '' : 'privateKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RenewCertificateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RenewCertificateRequest copyWith(
          void Function(RenewCertificateRequest) updates) =>
      super.copyWith((message) => updates(message as RenewCertificateRequest))
          as RenewCertificateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RenewCertificateRequest() / RenewCertificateRequest.new instead')
  static RenewCertificateRequest create() => RenewCertificateRequest._();
  static $pb.GeneratedMessage $_createMessage() => RenewCertificateRequest._();
  @$core.override
  RenewCertificateRequest createEmptyInstance() => RenewCertificateRequest._();
  @$core.pragma('dart2js:noInline')
  static RenewCertificateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RenewCertificateRequest>(
          RenewCertificateRequest.$_createMessage);
  static RenewCertificateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get certificateData => $_getSZ(1);
  @$pb.TagNumber(2)
  set certificateData($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCertificateData() => $_has(1);
  @$pb.TagNumber(2)
  void clearCertificateData() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get privateKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set privateKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPrivateKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearPrivateKey() => $_clearField(3);
}

class RenewCertificateResponse extends $pb.GeneratedMessage {
  factory RenewCertificateResponse({
    Certificate? certificate,
  }) {
    final result = RenewCertificateResponse._();
    if (certificate != null) result.certificate = certificate;
    return result;
  }

  RenewCertificateResponse._();

  factory RenewCertificateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RenewCertificateResponse()..mergeFromBuffer(data, registry);
  factory RenewCertificateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RenewCertificateResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RenewCertificateResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RenewCertificateResponse.$_createMessage)
    ..aOM<Certificate>(1, _omitFieldNames ? '' : 'certificate',
        subBuilder: Certificate.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RenewCertificateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RenewCertificateResponse copyWith(
          void Function(RenewCertificateResponse) updates) =>
      super.copyWith((message) => updates(message as RenewCertificateResponse))
          as RenewCertificateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RenewCertificateResponse() / RenewCertificateResponse.new instead')
  static RenewCertificateResponse create() => RenewCertificateResponse._();
  static $pb.GeneratedMessage $_createMessage() => RenewCertificateResponse._();
  @$core.override
  RenewCertificateResponse createEmptyInstance() =>
      RenewCertificateResponse._();
  @$core.pragma('dart2js:noInline')
  static RenewCertificateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RenewCertificateResponse>(
          RenewCertificateResponse.$_createMessage);
  static RenewCertificateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Certificate get certificate => $_getN(0);
  @$pb.TagNumber(1)
  set certificate(Certificate value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificate() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificate() => $_clearField(1);
  @$pb.TagNumber(1)
  Certificate ensureCertificate() => $_ensure(0);
}

class ListCertificateRenewalsRequest extends $pb.GeneratedMessage {
  factory ListCertificateRenewalsRequest({
    $fixnum.Int64? certificateId,
  }) {
    final result = ListCertificateRenewalsRequest._();
    if (certificateId != null) result.certificateId = certificateId;
    return result;
  }

  ListCertificateRenewalsRequest._();

  factory ListCertificateRenewalsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateRenewalsRequest()..mergeFromBuffer(data, registry);
  factory ListCertificateRenewalsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateRenewalsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificateRenewalsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificateRenewalsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'certificateId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateRenewalsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateRenewalsRequest copyWith(
          void Function(ListCertificateRenewalsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListCertificateRenewalsRequest))
          as ListCertificateRenewalsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificateRenewalsRequest() / ListCertificateRenewalsRequest.new instead')
  static ListCertificateRenewalsRequest create() =>
      ListCertificateRenewalsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListCertificateRenewalsRequest._();
  @$core.override
  ListCertificateRenewalsRequest createEmptyInstance() =>
      ListCertificateRenewalsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListCertificateRenewalsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListCertificateRenewalsRequest>(
          ListCertificateRenewalsRequest.$_createMessage);
  static ListCertificateRenewalsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get certificateId => $_getI64(0);
  @$pb.TagNumber(1)
  set certificateId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCertificateId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCertificateId() => $_clearField(1);
}

class ListCertificateRenewalsResponse extends $pb.GeneratedMessage {
  factory ListCertificateRenewalsResponse({
    $core.Iterable<CertificateRenewal>? renewals,
  }) {
    final result = ListCertificateRenewalsResponse._();
    if (renewals != null) result.renewals.addAll(renewals);
    return result;
  }

  ListCertificateRenewalsResponse._();

  factory ListCertificateRenewalsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateRenewalsResponse()..mergeFromBuffer(data, registry);
  factory ListCertificateRenewalsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListCertificateRenewalsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListCertificateRenewalsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListCertificateRenewalsResponse.$_createMessage)
    ..pPM<CertificateRenewal>(1, _omitFieldNames ? '' : 'renewals',
        subBuilder: CertificateRenewal.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateRenewalsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListCertificateRenewalsResponse copyWith(
          void Function(ListCertificateRenewalsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListCertificateRenewalsResponse))
          as ListCertificateRenewalsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListCertificateRenewalsResponse() / ListCertificateRenewalsResponse.new instead')
  static ListCertificateRenewalsResponse create() =>
      ListCertificateRenewalsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListCertificateRenewalsResponse._();
  @$core.override
  ListCertificateRenewalsResponse createEmptyInstance() =>
      ListCertificateRenewalsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListCertificateRenewalsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListCertificateRenewalsResponse>(
          ListCertificateRenewalsResponse.$_createMessage);
  static ListCertificateRenewalsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<CertificateRenewal> get renewals => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

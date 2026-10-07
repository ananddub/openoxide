// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/overview.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'overview.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'overview.pbenum.dart';

class OverviewServiceItem extends $pb.GeneratedMessage {
  factory OverviewServiceItem({
    $fixnum.Int64? id,
    $core.String? name,
    OverviewServiceType? serviceType,
    $core.String? status,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? projectId,
    $core.String? projectName,
    $fixnum.Int64? environmentId,
    $core.String? environmentName,
    $core.String? dbKind,
    $fixnum.Int64? serverId,
    $core.String? serverName,
  }) {
    final result = OverviewServiceItem._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (serviceType != null) result.serviceType = serviceType;
    if (status != null) result.status = status;
    if (createdAt != null) result.createdAt = createdAt;
    if (projectId != null) result.projectId = projectId;
    if (projectName != null) result.projectName = projectName;
    if (environmentId != null) result.environmentId = environmentId;
    if (environmentName != null) result.environmentName = environmentName;
    if (dbKind != null) result.dbKind = dbKind;
    if (serverId != null) result.serverId = serverId;
    if (serverName != null) result.serverName = serverName;
    return result;
  }

  OverviewServiceItem._();

  factory OverviewServiceItem.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewServiceItem()..mergeFromBuffer(data, registry);
  factory OverviewServiceItem.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewServiceItem()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewServiceItem',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewServiceItem.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<OverviewServiceType>(3, _omitFieldNames ? '' : 'serviceType',
        enumValues: OverviewServiceType.values)
    ..aOS(4, _omitFieldNames ? '' : 'status')
    ..aInt64(5, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(6, _omitFieldNames ? '' : 'projectId')
    ..aOS(7, _omitFieldNames ? '' : 'projectName')
    ..aInt64(8, _omitFieldNames ? '' : 'environmentId')
    ..aOS(9, _omitFieldNames ? '' : 'environmentName')
    ..aOS(10, _omitFieldNames ? '' : 'dbKind')
    ..aInt64(11, _omitFieldNames ? '' : 'serverId')
    ..aOS(12, _omitFieldNames ? '' : 'serverName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewServiceItem clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewServiceItem copyWith(void Function(OverviewServiceItem) updates) =>
      super.copyWith((message) => updates(message as OverviewServiceItem))
          as OverviewServiceItem;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use OverviewServiceItem() / OverviewServiceItem.new instead')
  static OverviewServiceItem create() => OverviewServiceItem._();
  static $pb.GeneratedMessage $_createMessage() => OverviewServiceItem._();
  @$core.override
  OverviewServiceItem createEmptyInstance() => OverviewServiceItem._();
  @$core.pragma('dart2js:noInline')
  static OverviewServiceItem getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewServiceItem>(
          OverviewServiceItem.$_createMessage);
  static OverviewServiceItem? _defaultInstance;

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
  OverviewServiceType get serviceType => $_getN(2);
  @$pb.TagNumber(3)
  set serviceType(OverviewServiceType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasServiceType() => $_has(2);
  @$pb.TagNumber(3)
  void clearServiceType() => $_clearField(3);

  /// Free-form runtime status string from the services table.
  @$pb.TagNumber(4)
  $core.String get status => $_getSZ(3);
  @$pb.TagNumber(4)
  set status($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get createdAt => $_getI64(4);
  @$pb.TagNumber(5)
  set createdAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get projectId => $_getI64(5);
  @$pb.TagNumber(6)
  set projectId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasProjectId() => $_has(5);
  @$pb.TagNumber(6)
  void clearProjectId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get projectName => $_getSZ(6);
  @$pb.TagNumber(7)
  set projectName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasProjectName() => $_has(6);
  @$pb.TagNumber(7)
  void clearProjectName() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get environmentId => $_getI64(7);
  @$pb.TagNumber(8)
  set environmentId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEnvironmentId() => $_has(7);
  @$pb.TagNumber(8)
  void clearEnvironmentId() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get environmentName => $_getSZ(8);
  @$pb.TagNumber(9)
  set environmentName($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEnvironmentName() => $_has(8);
  @$pb.TagNumber(9)
  void clearEnvironmentName() => $_clearField(9);

  /// Lowercased database kind; only set for DATABASE services.
  @$pb.TagNumber(10)
  $core.String get dbKind => $_getSZ(9);
  @$pb.TagNumber(10)
  set dbKind($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasDbKind() => $_has(9);
  @$pb.TagNumber(10)
  void clearDbKind() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get serverId => $_getI64(10);
  @$pb.TagNumber(11)
  set serverId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasServerId() => $_has(10);
  @$pb.TagNumber(11)
  void clearServerId() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get serverName => $_getSZ(11);
  @$pb.TagNumber(12)
  set serverName($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasServerName() => $_has(11);
  @$pb.TagNumber(12)
  void clearServerName() => $_clearField(12);
}

/// Mirrors the domains table columns so detail-page edit dialogs can hydrate from the list; keep aligned with the Domain message in domain.proto.
class OverviewDomainItem extends $pb.GeneratedMessage {
  factory OverviewDomainItem({
    $fixnum.Int64? id,
    $core.String? host,
    $core.String? path,
    $core.int? port,
    $core.bool? https,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $core.String? serviceName,
    $core.String? projectName,
    $core.String? internalPath,
    $core.String? customEntrypoint,
    $core.String? customCertResolver,
    $core.bool? stripPath,
    $core.String? middlewares,
    $core.String? domainType,
    $core.String? certificateType,
    $core.String? domainServiceName,
  }) {
    final result = OverviewDomainItem._();
    if (id != null) result.id = id;
    if (host != null) result.host = host;
    if (path != null) result.path = path;
    if (port != null) result.port = port;
    if (https != null) result.https = https;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (serviceName != null) result.serviceName = serviceName;
    if (projectName != null) result.projectName = projectName;
    if (internalPath != null) result.internalPath = internalPath;
    if (customEntrypoint != null) result.customEntrypoint = customEntrypoint;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (stripPath != null) result.stripPath = stripPath;
    if (middlewares != null) result.middlewares = middlewares;
    if (domainType != null) result.domainType = domainType;
    if (certificateType != null) result.certificateType = certificateType;
    if (domainServiceName != null) result.domainServiceName = domainServiceName;
    return result;
  }

  OverviewDomainItem._();

  factory OverviewDomainItem.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewDomainItem()..mergeFromBuffer(data, registry);
  factory OverviewDomainItem.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewDomainItem()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewDomainItem',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewDomainItem.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'host')
    ..aOS(3, _omitFieldNames ? '' : 'path')
    ..aI(4, _omitFieldNames ? '' : 'port')
    ..aOB(5, _omitFieldNames ? '' : 'https')
    ..aInt64(6, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(7, _omitFieldNames ? '' : 'composeId')
    ..aOS(8, _omitFieldNames ? '' : 'serviceName')
    ..aOS(9, _omitFieldNames ? '' : 'projectName')
    ..aOS(10, _omitFieldNames ? '' : 'internalPath')
    ..aOS(11, _omitFieldNames ? '' : 'customEntrypoint')
    ..aOS(12, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(13, _omitFieldNames ? '' : 'stripPath')
    ..aOS(14, _omitFieldNames ? '' : 'middlewares')
    ..aOS(15, _omitFieldNames ? '' : 'domainType')
    ..aOS(16, _omitFieldNames ? '' : 'certificateType')
    ..aOS(17, _omitFieldNames ? '' : 'domainServiceName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewDomainItem clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewDomainItem copyWith(void Function(OverviewDomainItem) updates) =>
      super.copyWith((message) => updates(message as OverviewDomainItem))
          as OverviewDomainItem;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use OverviewDomainItem() / OverviewDomainItem.new instead')
  static OverviewDomainItem create() => OverviewDomainItem._();
  static $pb.GeneratedMessage $_createMessage() => OverviewDomainItem._();
  @$core.override
  OverviewDomainItem createEmptyInstance() => OverviewDomainItem._();
  @$core.pragma('dart2js:noInline')
  static OverviewDomainItem getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewDomainItem>(
          OverviewDomainItem.$_createMessage);
  static OverviewDomainItem? _defaultInstance;

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
  $core.String get path => $_getSZ(2);
  @$pb.TagNumber(3)
  set path($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearPath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get port => $_getIZ(3);
  @$pb.TagNumber(4)
  set port($core.int value) => $_setSignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPort() => $_has(3);
  @$pb.TagNumber(4)
  void clearPort() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get https => $_getBF(4);
  @$pb.TagNumber(5)
  set https($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHttps() => $_has(4);
  @$pb.TagNumber(5)
  void clearHttps() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get applicationId => $_getI64(5);
  @$pb.TagNumber(6)
  set applicationId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApplicationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearApplicationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get composeId => $_getI64(6);
  @$pb.TagNumber(7)
  set composeId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasComposeId() => $_has(6);
  @$pb.TagNumber(7)
  void clearComposeId() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get serviceName => $_getSZ(7);
  @$pb.TagNumber(8)
  set serviceName($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasServiceName() => $_has(7);
  @$pb.TagNumber(8)
  void clearServiceName() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get projectName => $_getSZ(8);
  @$pb.TagNumber(9)
  set projectName($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasProjectName() => $_has(8);
  @$pb.TagNumber(9)
  void clearProjectName() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get internalPath => $_getSZ(9);
  @$pb.TagNumber(10)
  set internalPath($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasInternalPath() => $_has(9);
  @$pb.TagNumber(10)
  void clearInternalPath() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get customEntrypoint => $_getSZ(10);
  @$pb.TagNumber(11)
  set customEntrypoint($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCustomEntrypoint() => $_has(10);
  @$pb.TagNumber(11)
  void clearCustomEntrypoint() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get customCertResolver => $_getSZ(11);
  @$pb.TagNumber(12)
  set customCertResolver($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasCustomCertResolver() => $_has(11);
  @$pb.TagNumber(12)
  void clearCustomCertResolver() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get stripPath => $_getBF(12);
  @$pb.TagNumber(13)
  set stripPath($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasStripPath() => $_has(12);
  @$pb.TagNumber(13)
  void clearStripPath() => $_clearField(13);

  /// JSON array of middleware names stored verbatim on the row.
  @$pb.TagNumber(14)
  $core.String get middlewares => $_getSZ(13);
  @$pb.TagNumber(14)
  set middlewares($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasMiddlewares() => $_has(13);
  @$pb.TagNumber(14)
  void clearMiddlewares() => $_clearField(14);

  /// Raw domains-table strings (APPLICATION/COMPOSE, ACME/CUSTOM/...); passed through without panel-side normalisation.
  @$pb.TagNumber(15)
  $core.String get domainType => $_getSZ(14);
  @$pb.TagNumber(15)
  set domainType($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasDomainType() => $_has(14);
  @$pb.TagNumber(15)
  void clearDomainType() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get certificateType => $_getSZ(15);
  @$pb.TagNumber(16)
  set certificateType($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasCertificateType() => $_has(15);
  @$pb.TagNumber(16)
  void clearCertificateType() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.String get domainServiceName => $_getSZ(16);
  @$pb.TagNumber(17)
  set domainServiceName($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasDomainServiceName() => $_has(16);
  @$pb.TagNumber(17)
  void clearDomainServiceName() => $_clearField(17);
}

/// The linkage mirrors are flattened into the payload top level and only the column matching the backup's service kind is set.
class OverviewBackupItem extends $pb.GeneratedMessage {
  factory OverviewBackupItem({
    $fixnum.Int64? id,
    $core.String? name,
    OverviewBackupType? backupType,
    OverviewBackupStatus? status,
    $core.String? destination,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? postgresId,
    $fixnum.Int64? mysqlId,
    $fixnum.Int64? mariadbId,
    $fixnum.Int64? mongoId,
    $fixnum.Int64? redisId,
    $fixnum.Int64? libsqlId,
  }) {
    final result = OverviewBackupItem._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (backupType != null) result.backupType = backupType;
    if (status != null) result.status = status;
    if (destination != null) result.destination = destination;
    if (createdAt != null) result.createdAt = createdAt;
    if (serviceId != null) result.serviceId = serviceId;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (postgresId != null) result.postgresId = postgresId;
    if (mysqlId != null) result.mysqlId = mysqlId;
    if (mariadbId != null) result.mariadbId = mariadbId;
    if (mongoId != null) result.mongoId = mongoId;
    if (redisId != null) result.redisId = redisId;
    if (libsqlId != null) result.libsqlId = libsqlId;
    return result;
  }

  OverviewBackupItem._();

  factory OverviewBackupItem.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewBackupItem()..mergeFromBuffer(data, registry);
  factory OverviewBackupItem.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewBackupItem()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewBackupItem',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewBackupItem.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<OverviewBackupType>(3, _omitFieldNames ? '' : 'backupType',
        enumValues: OverviewBackupType.values)
    ..aE<OverviewBackupStatus>(4, _omitFieldNames ? '' : 'status',
        enumValues: OverviewBackupStatus.values)
    ..aOS(5, _omitFieldNames ? '' : 'destination')
    ..aInt64(6, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(7, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(8, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(9, _omitFieldNames ? '' : 'composeId')
    ..aInt64(10, _omitFieldNames ? '' : 'postgresId')
    ..aInt64(11, _omitFieldNames ? '' : 'mysqlId')
    ..aInt64(12, _omitFieldNames ? '' : 'mariadbId')
    ..aInt64(13, _omitFieldNames ? '' : 'mongoId')
    ..aInt64(14, _omitFieldNames ? '' : 'redisId')
    ..aInt64(15, _omitFieldNames ? '' : 'libsqlId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewBackupItem clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewBackupItem copyWith(void Function(OverviewBackupItem) updates) =>
      super.copyWith((message) => updates(message as OverviewBackupItem))
          as OverviewBackupItem;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use OverviewBackupItem() / OverviewBackupItem.new instead')
  static OverviewBackupItem create() => OverviewBackupItem._();
  static $pb.GeneratedMessage $_createMessage() => OverviewBackupItem._();
  @$core.override
  OverviewBackupItem createEmptyInstance() => OverviewBackupItem._();
  @$core.pragma('dart2js:noInline')
  static OverviewBackupItem getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewBackupItem>(
          OverviewBackupItem.$_createMessage);
  static OverviewBackupItem? _defaultInstance;

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
  OverviewBackupType get backupType => $_getN(2);
  @$pb.TagNumber(3)
  set backupType(OverviewBackupType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasBackupType() => $_has(2);
  @$pb.TagNumber(3)
  void clearBackupType() => $_clearField(3);

  @$pb.TagNumber(4)
  OverviewBackupStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(OverviewBackupStatus value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  /// Display label ("Destination #<id>"), not a stable identifier.
  @$pb.TagNumber(5)
  $core.String get destination => $_getSZ(4);
  @$pb.TagNumber(5)
  set destination($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDestination() => $_has(4);
  @$pb.TagNumber(5)
  void clearDestination() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get createdAt => $_getI64(5);
  @$pb.TagNumber(6)
  set createdAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCreatedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get serviceId => $_getI64(6);
  @$pb.TagNumber(7)
  set serviceId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceId() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceId() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get applicationId => $_getI64(7);
  @$pb.TagNumber(8)
  set applicationId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasApplicationId() => $_has(7);
  @$pb.TagNumber(8)
  void clearApplicationId() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get composeId => $_getI64(8);
  @$pb.TagNumber(9)
  set composeId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasComposeId() => $_has(8);
  @$pb.TagNumber(9)
  void clearComposeId() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get postgresId => $_getI64(9);
  @$pb.TagNumber(10)
  set postgresId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasPostgresId() => $_has(9);
  @$pb.TagNumber(10)
  void clearPostgresId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get mysqlId => $_getI64(10);
  @$pb.TagNumber(11)
  set mysqlId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasMysqlId() => $_has(10);
  @$pb.TagNumber(11)
  void clearMysqlId() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get mariadbId => $_getI64(11);
  @$pb.TagNumber(12)
  set mariadbId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasMariadbId() => $_has(11);
  @$pb.TagNumber(12)
  void clearMariadbId() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get mongoId => $_getI64(12);
  @$pb.TagNumber(13)
  set mongoId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasMongoId() => $_has(12);
  @$pb.TagNumber(13)
  void clearMongoId() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get redisId => $_getI64(13);
  @$pb.TagNumber(14)
  set redisId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasRedisId() => $_has(13);
  @$pb.TagNumber(14)
  void clearRedisId() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get libsqlId => $_getI64(14);
  @$pb.TagNumber(15)
  set libsqlId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasLibsqlId() => $_has(14);
  @$pb.TagNumber(15)
  void clearLibsqlId() => $_clearField(15);
}

/// Legacy aliases (no server-side implementation yet); same shapes as the organization-scoped rpcs below.
class OverviewListServicesRequest extends $pb.GeneratedMessage {
  factory OverviewListServicesRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListServicesRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (environmentId != null) result.environmentId = environmentId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListServicesRequest._();

  factory OverviewListServicesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListServicesRequest()..mergeFromBuffer(data, registry);
  factory OverviewListServicesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListServicesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListServicesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListServicesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(3, _omitFieldNames ? '' : 'limit')
    ..aInt64(4, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListServicesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListServicesRequest copyWith(
          void Function(OverviewListServicesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListServicesRequest))
          as OverviewListServicesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListServicesRequest() / OverviewListServicesRequest.new instead')
  static OverviewListServicesRequest create() =>
      OverviewListServicesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListServicesRequest._();
  @$core.override
  OverviewListServicesRequest createEmptyInstance() =>
      OverviewListServicesRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListServicesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListServicesRequest>(
          OverviewListServicesRequest.$_createMessage);
  static OverviewListServicesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);

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

class OverviewListServicesResponse extends $pb.GeneratedMessage {
  factory OverviewListServicesResponse({
    $core.Iterable<OverviewServiceItem>? services,
  }) {
    final result = OverviewListServicesResponse._();
    if (services != null) result.services.addAll(services);
    return result;
  }

  OverviewListServicesResponse._();

  factory OverviewListServicesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListServicesResponse()..mergeFromBuffer(data, registry);
  factory OverviewListServicesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListServicesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListServicesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListServicesResponse.$_createMessage)
    ..pPM<OverviewServiceItem>(1, _omitFieldNames ? '' : 'services',
        subBuilder: OverviewServiceItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListServicesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListServicesResponse copyWith(
          void Function(OverviewListServicesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListServicesResponse))
          as OverviewListServicesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListServicesResponse() / OverviewListServicesResponse.new instead')
  static OverviewListServicesResponse create() =>
      OverviewListServicesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListServicesResponse._();
  @$core.override
  OverviewListServicesResponse createEmptyInstance() =>
      OverviewListServicesResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListServicesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListServicesResponse>(
          OverviewListServicesResponse.$_createMessage);
  static OverviewListServicesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewServiceItem> get services => $_getList(0);
}

class OverviewListDomainsRequest extends $pb.GeneratedMessage {
  factory OverviewListDomainsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListDomainsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (environmentId != null) result.environmentId = environmentId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListDomainsRequest._();

  factory OverviewListDomainsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListDomainsRequest()..mergeFromBuffer(data, registry);
  factory OverviewListDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListDomainsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(3, _omitFieldNames ? '' : 'limit')
    ..aInt64(4, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListDomainsRequest copyWith(
          void Function(OverviewListDomainsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListDomainsRequest))
          as OverviewListDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListDomainsRequest() / OverviewListDomainsRequest.new instead')
  static OverviewListDomainsRequest create() => OverviewListDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListDomainsRequest._();
  @$core.override
  OverviewListDomainsRequest createEmptyInstance() =>
      OverviewListDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListDomainsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListDomainsRequest>(
          OverviewListDomainsRequest.$_createMessage);
  static OverviewListDomainsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);

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

class OverviewListDomainsResponse extends $pb.GeneratedMessage {
  factory OverviewListDomainsResponse({
    $core.Iterable<OverviewDomainItem>? domains,
  }) {
    final result = OverviewListDomainsResponse._();
    if (domains != null) result.domains.addAll(domains);
    return result;
  }

  OverviewListDomainsResponse._();

  factory OverviewListDomainsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListDomainsResponse()..mergeFromBuffer(data, registry);
  factory OverviewListDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListDomainsResponse.$_createMessage)
    ..pPM<OverviewDomainItem>(1, _omitFieldNames ? '' : 'domains',
        subBuilder: OverviewDomainItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListDomainsResponse copyWith(
          void Function(OverviewListDomainsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListDomainsResponse))
          as OverviewListDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListDomainsResponse() / OverviewListDomainsResponse.new instead')
  static OverviewListDomainsResponse create() =>
      OverviewListDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListDomainsResponse._();
  @$core.override
  OverviewListDomainsResponse createEmptyInstance() =>
      OverviewListDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListDomainsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListDomainsResponse>(
          OverviewListDomainsResponse.$_createMessage);
  static OverviewListDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewDomainItem> get domains => $_getList(0);
}

class OverviewListBackupsRequest extends $pb.GeneratedMessage {
  factory OverviewListBackupsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListBackupsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (environmentId != null) result.environmentId = environmentId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListBackupsRequest._();

  factory OverviewListBackupsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListBackupsRequest()..mergeFromBuffer(data, registry);
  factory OverviewListBackupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListBackupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListBackupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListBackupsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(3, _omitFieldNames ? '' : 'limit')
    ..aInt64(4, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListBackupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListBackupsRequest copyWith(
          void Function(OverviewListBackupsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListBackupsRequest))
          as OverviewListBackupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListBackupsRequest() / OverviewListBackupsRequest.new instead')
  static OverviewListBackupsRequest create() => OverviewListBackupsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListBackupsRequest._();
  @$core.override
  OverviewListBackupsRequest createEmptyInstance() =>
      OverviewListBackupsRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListBackupsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListBackupsRequest>(
          OverviewListBackupsRequest.$_createMessage);
  static OverviewListBackupsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);

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

class OverviewListBackupsResponse extends $pb.GeneratedMessage {
  factory OverviewListBackupsResponse({
    $core.Iterable<OverviewBackupItem>? backups,
  }) {
    final result = OverviewListBackupsResponse._();
    if (backups != null) result.backups.addAll(backups);
    return result;
  }

  OverviewListBackupsResponse._();

  factory OverviewListBackupsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListBackupsResponse()..mergeFromBuffer(data, registry);
  factory OverviewListBackupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListBackupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListBackupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OverviewListBackupsResponse.$_createMessage)
    ..pPM<OverviewBackupItem>(1, _omitFieldNames ? '' : 'backups',
        subBuilder: OverviewBackupItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListBackupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListBackupsResponse copyWith(
          void Function(OverviewListBackupsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as OverviewListBackupsResponse))
          as OverviewListBackupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListBackupsResponse() / OverviewListBackupsResponse.new instead')
  static OverviewListBackupsResponse create() =>
      OverviewListBackupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListBackupsResponse._();
  @$core.override
  OverviewListBackupsResponse createEmptyInstance() =>
      OverviewListBackupsResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListBackupsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OverviewListBackupsResponse>(
          OverviewListBackupsResponse.$_createMessage);
  static OverviewListBackupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewBackupItem> get backups => $_getList(0);
}

class OverviewListOrganizationServicesRequest extends $pb.GeneratedMessage {
  factory OverviewListOrganizationServicesRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListOrganizationServicesRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListOrganizationServicesRequest._();

  factory OverviewListOrganizationServicesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationServicesRequest()
        ..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationServicesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationServicesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationServicesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationServicesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'limit')
    ..aInt64(3, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationServicesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationServicesRequest copyWith(
          void Function(OverviewListOrganizationServicesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationServicesRequest))
          as OverviewListOrganizationServicesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationServicesRequest() / OverviewListOrganizationServicesRequest.new instead')
  static OverviewListOrganizationServicesRequest create() =>
      OverviewListOrganizationServicesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationServicesRequest._();
  @$core.override
  OverviewListOrganizationServicesRequest createEmptyInstance() =>
      OverviewListOrganizationServicesRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationServicesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationServicesRequest>(
          OverviewListOrganizationServicesRequest.$_createMessage);
  static OverviewListOrganizationServicesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(2)
  set limit($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get offset => $_getI64(2);
  @$pb.TagNumber(3)
  set offset($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOffset() => $_has(2);
  @$pb.TagNumber(3)
  void clearOffset() => $_clearField(3);
}

class OverviewListOrganizationServicesResponse extends $pb.GeneratedMessage {
  factory OverviewListOrganizationServicesResponse({
    $core.Iterable<OverviewServiceItem>? services,
  }) {
    final result = OverviewListOrganizationServicesResponse._();
    if (services != null) result.services.addAll(services);
    return result;
  }

  OverviewListOrganizationServicesResponse._();

  factory OverviewListOrganizationServicesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationServicesResponse()
        ..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationServicesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationServicesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationServicesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationServicesResponse.$_createMessage)
    ..pPM<OverviewServiceItem>(1, _omitFieldNames ? '' : 'services',
        subBuilder: OverviewServiceItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationServicesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationServicesResponse copyWith(
          void Function(OverviewListOrganizationServicesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationServicesResponse))
          as OverviewListOrganizationServicesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationServicesResponse() / OverviewListOrganizationServicesResponse.new instead')
  static OverviewListOrganizationServicesResponse create() =>
      OverviewListOrganizationServicesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationServicesResponse._();
  @$core.override
  OverviewListOrganizationServicesResponse createEmptyInstance() =>
      OverviewListOrganizationServicesResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationServicesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationServicesResponse>(
          OverviewListOrganizationServicesResponse.$_createMessage);
  static OverviewListOrganizationServicesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewServiceItem> get services => $_getList(0);
}

class OverviewListOrganizationDomainsRequest extends $pb.GeneratedMessage {
  factory OverviewListOrganizationDomainsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListOrganizationDomainsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListOrganizationDomainsRequest._();

  factory OverviewListOrganizationDomainsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationDomainsRequest()..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationDomainsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationDomainsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationDomainsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationDomainsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'limit')
    ..aInt64(3, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationDomainsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationDomainsRequest copyWith(
          void Function(OverviewListOrganizationDomainsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationDomainsRequest))
          as OverviewListOrganizationDomainsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationDomainsRequest() / OverviewListOrganizationDomainsRequest.new instead')
  static OverviewListOrganizationDomainsRequest create() =>
      OverviewListOrganizationDomainsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationDomainsRequest._();
  @$core.override
  OverviewListOrganizationDomainsRequest createEmptyInstance() =>
      OverviewListOrganizationDomainsRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationDomainsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationDomainsRequest>(
          OverviewListOrganizationDomainsRequest.$_createMessage);
  static OverviewListOrganizationDomainsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(2)
  set limit($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get offset => $_getI64(2);
  @$pb.TagNumber(3)
  set offset($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOffset() => $_has(2);
  @$pb.TagNumber(3)
  void clearOffset() => $_clearField(3);
}

class OverviewListOrganizationDomainsResponse extends $pb.GeneratedMessage {
  factory OverviewListOrganizationDomainsResponse({
    $core.Iterable<OverviewDomainItem>? domains,
  }) {
    final result = OverviewListOrganizationDomainsResponse._();
    if (domains != null) result.domains.addAll(domains);
    return result;
  }

  OverviewListOrganizationDomainsResponse._();

  factory OverviewListOrganizationDomainsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationDomainsResponse()
        ..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationDomainsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationDomainsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationDomainsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationDomainsResponse.$_createMessage)
    ..pPM<OverviewDomainItem>(1, _omitFieldNames ? '' : 'domains',
        subBuilder: OverviewDomainItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationDomainsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationDomainsResponse copyWith(
          void Function(OverviewListOrganizationDomainsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationDomainsResponse))
          as OverviewListOrganizationDomainsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationDomainsResponse() / OverviewListOrganizationDomainsResponse.new instead')
  static OverviewListOrganizationDomainsResponse create() =>
      OverviewListOrganizationDomainsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationDomainsResponse._();
  @$core.override
  OverviewListOrganizationDomainsResponse createEmptyInstance() =>
      OverviewListOrganizationDomainsResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationDomainsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationDomainsResponse>(
          OverviewListOrganizationDomainsResponse.$_createMessage);
  static OverviewListOrganizationDomainsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewDomainItem> get domains => $_getList(0);
}

class OverviewListOrganizationBackupsRequest extends $pb.GeneratedMessage {
  factory OverviewListOrganizationBackupsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = OverviewListOrganizationBackupsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  OverviewListOrganizationBackupsRequest._();

  factory OverviewListOrganizationBackupsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationBackupsRequest()..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationBackupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationBackupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationBackupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationBackupsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'limit')
    ..aInt64(3, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationBackupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationBackupsRequest copyWith(
          void Function(OverviewListOrganizationBackupsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationBackupsRequest))
          as OverviewListOrganizationBackupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationBackupsRequest() / OverviewListOrganizationBackupsRequest.new instead')
  static OverviewListOrganizationBackupsRequest create() =>
      OverviewListOrganizationBackupsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationBackupsRequest._();
  @$core.override
  OverviewListOrganizationBackupsRequest createEmptyInstance() =>
      OverviewListOrganizationBackupsRequest._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationBackupsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationBackupsRequest>(
          OverviewListOrganizationBackupsRequest.$_createMessage);
  static OverviewListOrganizationBackupsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(2)
  set limit($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get offset => $_getI64(2);
  @$pb.TagNumber(3)
  set offset($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOffset() => $_has(2);
  @$pb.TagNumber(3)
  void clearOffset() => $_clearField(3);
}

class OverviewListOrganizationBackupsResponse extends $pb.GeneratedMessage {
  factory OverviewListOrganizationBackupsResponse({
    $core.Iterable<OverviewBackupItem>? backups,
  }) {
    final result = OverviewListOrganizationBackupsResponse._();
    if (backups != null) result.backups.addAll(backups);
    return result;
  }

  OverviewListOrganizationBackupsResponse._();

  factory OverviewListOrganizationBackupsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationBackupsResponse()
        ..mergeFromBuffer(data, registry);
  factory OverviewListOrganizationBackupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OverviewListOrganizationBackupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OverviewListOrganizationBackupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          OverviewListOrganizationBackupsResponse.$_createMessage)
    ..pPM<OverviewBackupItem>(1, _omitFieldNames ? '' : 'backups',
        subBuilder: OverviewBackupItem.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationBackupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OverviewListOrganizationBackupsResponse copyWith(
          void Function(OverviewListOrganizationBackupsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as OverviewListOrganizationBackupsResponse))
          as OverviewListOrganizationBackupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use OverviewListOrganizationBackupsResponse() / OverviewListOrganizationBackupsResponse.new instead')
  static OverviewListOrganizationBackupsResponse create() =>
      OverviewListOrganizationBackupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      OverviewListOrganizationBackupsResponse._();
  @$core.override
  OverviewListOrganizationBackupsResponse createEmptyInstance() =>
      OverviewListOrganizationBackupsResponse._();
  @$core.pragma('dart2js:noInline')
  static OverviewListOrganizationBackupsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              OverviewListOrganizationBackupsResponse>(
          OverviewListOrganizationBackupsResponse.$_createMessage);
  static OverviewListOrganizationBackupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OverviewBackupItem> get backups => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

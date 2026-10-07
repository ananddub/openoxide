// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/backup.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'backup.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'backup.pbenum.dart';

/// kind "database_backup": a scheduled database backup rule.
class BackupDatabase extends $pb.GeneratedMessage {
  factory BackupDatabase({
    $fixnum.Int64? id,
    $core.String? appName,
    $core.String? schedule,
    $core.bool? enabled,
    $core.String? databaseName,
    $core.String? prefix,
    $core.String? serviceName,
    $fixnum.Int64? keepLatestCount,
    $core.String? backupType,
    $core.String? databaseType,
    $core.String? metadata,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? destinationId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = BackupDatabase._();
    if (id != null) result.id = id;
    if (appName != null) result.appName = appName;
    if (schedule != null) result.schedule = schedule;
    if (enabled != null) result.enabled = enabled;
    if (databaseName != null) result.databaseName = databaseName;
    if (prefix != null) result.prefix = prefix;
    if (serviceName != null) result.serviceName = serviceName;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (backupType != null) result.backupType = backupType;
    if (databaseType != null) result.databaseType = databaseType;
    if (metadata != null) result.metadata = metadata;
    if (serviceId != null) result.serviceId = serviceId;
    if (destinationId != null) result.destinationId = destinationId;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  BackupDatabase._();

  factory BackupDatabase.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDatabase()..mergeFromBuffer(data, registry);
  factory BackupDatabase.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDatabase()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDatabase',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDatabase.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'appName')
    ..aOS(3, _omitFieldNames ? '' : 'schedule')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOS(5, _omitFieldNames ? '' : 'databaseName')
    ..aOS(6, _omitFieldNames ? '' : 'prefix')
    ..aOS(7, _omitFieldNames ? '' : 'serviceName')
    ..aInt64(8, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOS(9, _omitFieldNames ? '' : 'backupType')
    ..aOS(10, _omitFieldNames ? '' : 'databaseType')
    ..aOS(11, _omitFieldNames ? '' : 'metadata')
    ..aInt64(12, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(13, _omitFieldNames ? '' : 'destinationId')
    ..aInt64(14, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(15, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(16, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDatabase clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDatabase copyWith(void Function(BackupDatabase) updates) =>
      super.copyWith((message) => updates(message as BackupDatabase))
          as BackupDatabase;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupDatabase() / BackupDatabase.new instead')
  static BackupDatabase create() => BackupDatabase._();
  static $pb.GeneratedMessage $_createMessage() => BackupDatabase._();
  @$core.override
  BackupDatabase createEmptyInstance() => BackupDatabase._();
  @$core.pragma('dart2js:noInline')
  static BackupDatabase getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BackupDatabase>(
          BackupDatabase.$_createMessage);
  static BackupDatabase? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get appName => $_getSZ(1);
  @$pb.TagNumber(2)
  set appName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAppName() => $_has(1);
  @$pb.TagNumber(2)
  void clearAppName() => $_clearField(2);

  /// Cron expression driving the schedule.
  @$pb.TagNumber(3)
  $core.String get schedule => $_getSZ(2);
  @$pb.TagNumber(3)
  set schedule($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSchedule() => $_has(2);
  @$pb.TagNumber(3)
  void clearSchedule() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get enabled => $_getBF(3);
  @$pb.TagNumber(4)
  set enabled($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnabled() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnabled() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get databaseName => $_getSZ(4);
  @$pb.TagNumber(5)
  set databaseName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDatabaseName() => $_has(4);
  @$pb.TagNumber(5)
  void clearDatabaseName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get prefix => $_getSZ(5);
  @$pb.TagNumber(6)
  set prefix($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPrefix() => $_has(5);
  @$pb.TagNumber(6)
  void clearPrefix() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serviceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set serviceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceName() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get keepLatestCount => $_getI64(7);
  @$pb.TagNumber(8)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasKeepLatestCount() => $_has(7);
  @$pb.TagNumber(8)
  void clearKeepLatestCount() => $_clearField(8);

  /// Free-form backup engine selector (e.g. "pg_dump").
  @$pb.TagNumber(9)
  $core.String get backupType => $_getSZ(8);
  @$pb.TagNumber(9)
  set backupType($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasBackupType() => $_has(8);
  @$pb.TagNumber(9)
  void clearBackupType() => $_clearField(9);

  /// Free-form database flavor stored as a string; canonical inputs are POSTGRES/POSTGRESQL/MYSQL/MARIADB/MONGO/MONGODB/REDIS/LIBSQL.
  @$pb.TagNumber(10)
  $core.String get databaseType => $_getSZ(9);
  @$pb.TagNumber(10)
  set databaseType($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasDatabaseType() => $_has(9);
  @$pb.TagNumber(10)
  void clearDatabaseType() => $_clearField(10);

  /// Optional JSON document with extra backup configuration.
  @$pb.TagNumber(11)
  $core.String get metadata => $_getSZ(10);
  @$pb.TagNumber(11)
  set metadata($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasMetadata() => $_has(10);
  @$pb.TagNumber(11)
  void clearMetadata() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get serviceId => $_getI64(11);
  @$pb.TagNumber(12)
  set serviceId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasServiceId() => $_has(11);
  @$pb.TagNumber(12)
  void clearServiceId() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get destinationId => $_getI64(12);
  @$pb.TagNumber(13)
  set destinationId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasDestinationId() => $_has(12);
  @$pb.TagNumber(13)
  void clearDestinationId() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get organizationId => $_getI64(13);
  @$pb.TagNumber(14)
  set organizationId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasOrganizationId() => $_has(13);
  @$pb.TagNumber(14)
  void clearOrganizationId() => $_clearField(14);

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

/// kind "volume_backup": a scheduled volume backup rule.
class BackupVolume extends $pb.GeneratedMessage {
  factory BackupVolume({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? volumeName,
    $core.String? prefix,
    $core.String? serviceType,
    $core.String? appName,
    $core.String? serviceName,
    $core.bool? turnOff,
    $core.String? cronExpression,
    $fixnum.Int64? keepLatestCount,
    $core.bool? enabled,
    $fixnum.Int64? destinationId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? postgresId,
    $fixnum.Int64? mysqlId,
    $fixnum.Int64? mariadbId,
    $fixnum.Int64? mongoId,
    $fixnum.Int64? redisId,
    $fixnum.Int64? libsqlId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = BackupVolume._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (volumeName != null) result.volumeName = volumeName;
    if (prefix != null) result.prefix = prefix;
    if (serviceType != null) result.serviceType = serviceType;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (turnOff != null) result.turnOff = turnOff;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (enabled != null) result.enabled = enabled;
    if (destinationId != null) result.destinationId = destinationId;
    if (organizationId != null) result.organizationId = organizationId;
    if (serviceId != null) result.serviceId = serviceId;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (postgresId != null) result.postgresId = postgresId;
    if (mysqlId != null) result.mysqlId = mysqlId;
    if (mariadbId != null) result.mariadbId = mariadbId;
    if (mongoId != null) result.mongoId = mongoId;
    if (redisId != null) result.redisId = redisId;
    if (libsqlId != null) result.libsqlId = libsqlId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  BackupVolume._();

  factory BackupVolume.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVolume()..mergeFromBuffer(data, registry);
  factory BackupVolume.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVolume()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupVolume',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupVolume.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'volumeName')
    ..aOS(4, _omitFieldNames ? '' : 'prefix')
    ..aOS(5, _omitFieldNames ? '' : 'serviceType')
    ..aOS(6, _omitFieldNames ? '' : 'appName')
    ..aOS(7, _omitFieldNames ? '' : 'serviceName')
    ..aOB(8, _omitFieldNames ? '' : 'turnOff')
    ..aOS(9, _omitFieldNames ? '' : 'cronExpression')
    ..aInt64(10, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOB(11, _omitFieldNames ? '' : 'enabled')
    ..aInt64(12, _omitFieldNames ? '' : 'destinationId')
    ..aInt64(13, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(14, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(15, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(16, _omitFieldNames ? '' : 'composeId')
    ..aInt64(17, _omitFieldNames ? '' : 'postgresId')
    ..aInt64(18, _omitFieldNames ? '' : 'mysqlId')
    ..aInt64(19, _omitFieldNames ? '' : 'mariadbId')
    ..aInt64(20, _omitFieldNames ? '' : 'mongoId')
    ..aInt64(21, _omitFieldNames ? '' : 'redisId')
    ..aInt64(22, _omitFieldNames ? '' : 'libsqlId')
    ..aInt64(23, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(24, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVolume clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVolume copyWith(void Function(BackupVolume) updates) =>
      super.copyWith((message) => updates(message as BackupVolume))
          as BackupVolume;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupVolume() / BackupVolume.new instead')
  static BackupVolume create() => BackupVolume._();
  static $pb.GeneratedMessage $_createMessage() => BackupVolume._();
  @$core.override
  BackupVolume createEmptyInstance() => BackupVolume._();
  @$core.pragma('dart2js:noInline')
  static BackupVolume getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BackupVolume>(
          BackupVolume.$_createMessage);
  static BackupVolume? _defaultInstance;

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
  $core.String get volumeName => $_getSZ(2);
  @$pb.TagNumber(3)
  set volumeName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVolumeName() => $_has(2);
  @$pb.TagNumber(3)
  void clearVolumeName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get prefix => $_getSZ(3);
  @$pb.TagNumber(4)
  set prefix($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrefix() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrefix() => $_clearField(4);

  /// Free-form service flavor stored as a string (APPLICATION, COMPOSE, POSTGRES, MYSQL, MARIADB, MONGO, REDIS, LIBSQL, "standalone", ...).
  @$pb.TagNumber(5)
  $core.String get serviceType => $_getSZ(4);
  @$pb.TagNumber(5)
  set serviceType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServiceType() => $_has(4);
  @$pb.TagNumber(5)
  void clearServiceType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get appName => $_getSZ(5);
  @$pb.TagNumber(6)
  set appName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAppName() => $_has(5);
  @$pb.TagNumber(6)
  void clearAppName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serviceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set serviceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceName() => $_clearField(7);

  /// Stops the owning service for the duration of the backup.
  @$pb.TagNumber(8)
  $core.bool get turnOff => $_getBF(7);
  @$pb.TagNumber(8)
  set turnOff($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasTurnOff() => $_has(7);
  @$pb.TagNumber(8)
  void clearTurnOff() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get cronExpression => $_getSZ(8);
  @$pb.TagNumber(9)
  set cronExpression($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCronExpression() => $_has(8);
  @$pb.TagNumber(9)
  void clearCronExpression() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get keepLatestCount => $_getI64(9);
  @$pb.TagNumber(10)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasKeepLatestCount() => $_has(9);
  @$pb.TagNumber(10)
  void clearKeepLatestCount() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.bool get enabled => $_getBF(10);
  @$pb.TagNumber(11)
  set enabled($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasEnabled() => $_has(10);
  @$pb.TagNumber(11)
  void clearEnabled() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get destinationId => $_getI64(11);
  @$pb.TagNumber(12)
  set destinationId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasDestinationId() => $_has(11);
  @$pb.TagNumber(12)
  void clearDestinationId() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get organizationId => $_getI64(12);
  @$pb.TagNumber(13)
  set organizationId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasOrganizationId() => $_has(12);
  @$pb.TagNumber(13)
  void clearOrganizationId() => $_clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get serviceId => $_getI64(13);
  @$pb.TagNumber(14)
  set serviceId($fixnum.Int64 value) => $_setInt64(13, value);
  @$pb.TagNumber(14)
  $core.bool hasServiceId() => $_has(13);
  @$pb.TagNumber(14)
  void clearServiceId() => $_clearField(14);

  /// Typed linkage mirrors derived from (service_type, service_id).
  @$pb.TagNumber(15)
  $fixnum.Int64 get applicationId => $_getI64(14);
  @$pb.TagNumber(15)
  set applicationId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasApplicationId() => $_has(14);
  @$pb.TagNumber(15)
  void clearApplicationId() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get composeId => $_getI64(15);
  @$pb.TagNumber(16)
  set composeId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasComposeId() => $_has(15);
  @$pb.TagNumber(16)
  void clearComposeId() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get postgresId => $_getI64(16);
  @$pb.TagNumber(17)
  set postgresId($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasPostgresId() => $_has(16);
  @$pb.TagNumber(17)
  void clearPostgresId() => $_clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get mysqlId => $_getI64(17);
  @$pb.TagNumber(18)
  set mysqlId($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasMysqlId() => $_has(17);
  @$pb.TagNumber(18)
  void clearMysqlId() => $_clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get mariadbId => $_getI64(18);
  @$pb.TagNumber(19)
  set mariadbId($fixnum.Int64 value) => $_setInt64(18, value);
  @$pb.TagNumber(19)
  $core.bool hasMariadbId() => $_has(18);
  @$pb.TagNumber(19)
  void clearMariadbId() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get mongoId => $_getI64(19);
  @$pb.TagNumber(20)
  set mongoId($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasMongoId() => $_has(19);
  @$pb.TagNumber(20)
  void clearMongoId() => $_clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get redisId => $_getI64(20);
  @$pb.TagNumber(21)
  set redisId($fixnum.Int64 value) => $_setInt64(20, value);
  @$pb.TagNumber(21)
  $core.bool hasRedisId() => $_has(20);
  @$pb.TagNumber(21)
  void clearRedisId() => $_clearField(21);

  @$pb.TagNumber(22)
  $fixnum.Int64 get libsqlId => $_getI64(21);
  @$pb.TagNumber(22)
  set libsqlId($fixnum.Int64 value) => $_setInt64(21, value);
  @$pb.TagNumber(22)
  $core.bool hasLibsqlId() => $_has(21);
  @$pb.TagNumber(22)
  void clearLibsqlId() => $_clearField(22);

  @$pb.TagNumber(23)
  $fixnum.Int64 get createdAt => $_getI64(22);
  @$pb.TagNumber(23)
  set createdAt($fixnum.Int64 value) => $_setInt64(22, value);
  @$pb.TagNumber(23)
  $core.bool hasCreatedAt() => $_has(22);
  @$pb.TagNumber(23)
  void clearCreatedAt() => $_clearField(23);

  @$pb.TagNumber(24)
  $fixnum.Int64 get updatedAt => $_getI64(23);
  @$pb.TagNumber(24)
  set updatedAt($fixnum.Int64 value) => $_setInt64(23, value);
  @$pb.TagNumber(24)
  $core.bool hasUpdatedAt() => $_has(23);
  @$pb.TagNumber(24)
  void clearUpdatedAt() => $_clearField(24);
}

/// kind "backup_execution": one recorded backup/restore run.
class BackupExecution extends $pb.GeneratedMessage {
  factory BackupExecution({
    $fixnum.Int64? id,
    BackupKind? backupKind,
    BackupOperation? operation,
    $fixnum.Int64? backupId,
    BackupExecutionStatus? status,
    $core.String? objectKey,
    $core.String? checksumSha256,
    $fixnum.Int64? sizeBytes,
    $fixnum.Int64? attempt,
    $core.String? error,
    $fixnum.Int64? startedAt,
    $fixnum.Int64? finishedAt,
  }) {
    final result = BackupExecution._();
    if (id != null) result.id = id;
    if (backupKind != null) result.backupKind = backupKind;
    if (operation != null) result.operation = operation;
    if (backupId != null) result.backupId = backupId;
    if (status != null) result.status = status;
    if (objectKey != null) result.objectKey = objectKey;
    if (checksumSha256 != null) result.checksumSha256 = checksumSha256;
    if (sizeBytes != null) result.sizeBytes = sizeBytes;
    if (attempt != null) result.attempt = attempt;
    if (error != null) result.error = error;
    if (startedAt != null) result.startedAt = startedAt;
    if (finishedAt != null) result.finishedAt = finishedAt;
    return result;
  }

  BackupExecution._();

  factory BackupExecution.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupExecution()..mergeFromBuffer(data, registry);
  factory BackupExecution.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupExecution()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupExecution',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupExecution.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<BackupKind>(2, _omitFieldNames ? '' : 'backupKind',
        enumValues: BackupKind.values)
    ..aE<BackupOperation>(3, _omitFieldNames ? '' : 'operation',
        enumValues: BackupOperation.values)
    ..aInt64(4, _omitFieldNames ? '' : 'backupId')
    ..aE<BackupExecutionStatus>(5, _omitFieldNames ? '' : 'status',
        enumValues: BackupExecutionStatus.values)
    ..aOS(6, _omitFieldNames ? '' : 'objectKey')
    ..aOS(7, _omitFieldNames ? '' : 'checksumSha256')
    ..aInt64(8, _omitFieldNames ? '' : 'sizeBytes')
    ..aInt64(9, _omitFieldNames ? '' : 'attempt')
    ..aOS(10, _omitFieldNames ? '' : 'error')
    ..aInt64(11, _omitFieldNames ? '' : 'startedAt')
    ..aInt64(12, _omitFieldNames ? '' : 'finishedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupExecution clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupExecution copyWith(void Function(BackupExecution) updates) =>
      super.copyWith((message) => updates(message as BackupExecution))
          as BackupExecution;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupExecution() / BackupExecution.new instead')
  static BackupExecution create() => BackupExecution._();
  static $pb.GeneratedMessage $_createMessage() => BackupExecution._();
  @$core.override
  BackupExecution createEmptyInstance() => BackupExecution._();
  @$core.pragma('dart2js:noInline')
  static BackupExecution getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BackupExecution>(
          BackupExecution.$_createMessage);
  static BackupExecution? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  BackupKind get backupKind => $_getN(1);
  @$pb.TagNumber(2)
  set backupKind(BackupKind value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasBackupKind() => $_has(1);
  @$pb.TagNumber(2)
  void clearBackupKind() => $_clearField(2);

  @$pb.TagNumber(3)
  BackupOperation get operation => $_getN(2);
  @$pb.TagNumber(3)
  set operation(BackupOperation value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasOperation() => $_has(2);
  @$pb.TagNumber(3)
  void clearOperation() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get backupId => $_getI64(3);
  @$pb.TagNumber(4)
  set backupId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBackupId() => $_has(3);
  @$pb.TagNumber(4)
  void clearBackupId() => $_clearField(4);

  @$pb.TagNumber(5)
  BackupExecutionStatus get status => $_getN(4);
  @$pb.TagNumber(5)
  set status(BackupExecutionStatus value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(5)
  void clearStatus() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get objectKey => $_getSZ(5);
  @$pb.TagNumber(6)
  set objectKey($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasObjectKey() => $_has(5);
  @$pb.TagNumber(6)
  void clearObjectKey() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get checksumSha256 => $_getSZ(6);
  @$pb.TagNumber(7)
  set checksumSha256($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasChecksumSha256() => $_has(6);
  @$pb.TagNumber(7)
  void clearChecksumSha256() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get sizeBytes => $_getI64(7);
  @$pb.TagNumber(8)
  set sizeBytes($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasSizeBytes() => $_has(7);
  @$pb.TagNumber(8)
  void clearSizeBytes() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get attempt => $_getI64(8);
  @$pb.TagNumber(9)
  set attempt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasAttempt() => $_has(8);
  @$pb.TagNumber(9)
  void clearAttempt() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get error => $_getSZ(9);
  @$pb.TagNumber(10)
  set error($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasError() => $_has(9);
  @$pb.TagNumber(10)
  void clearError() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get startedAt => $_getI64(10);
  @$pb.TagNumber(11)
  set startedAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasStartedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearStartedAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get finishedAt => $_getI64(11);
  @$pb.TagNumber(12)
  set finishedAt($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasFinishedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearFinishedAt() => $_clearField(12);
}

/// kind "backup_file": one remote object listed through rclone.
class BackupFile extends $pb.GeneratedMessage {
  factory BackupFile({
    $core.String? name,
    $core.String? path,
    $fixnum.Int64? sizeBytes,
    $core.String? modifiedAt,
  }) {
    final result = BackupFile._();
    if (name != null) result.name = name;
    if (path != null) result.path = path;
    if (sizeBytes != null) result.sizeBytes = sizeBytes;
    if (modifiedAt != null) result.modifiedAt = modifiedAt;
    return result;
  }

  BackupFile._();

  factory BackupFile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupFile()..mergeFromBuffer(data, registry);
  factory BackupFile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupFile()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupFile',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupFile.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..aInt64(3, _omitFieldNames ? '' : 'sizeBytes')
    ..aOS(4, _omitFieldNames ? '' : 'modifiedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupFile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupFile copyWith(void Function(BackupFile) updates) =>
      super.copyWith((message) => updates(message as BackupFile)) as BackupFile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupFile() / BackupFile.new instead')
  static BackupFile create() => BackupFile._();
  static $pb.GeneratedMessage $_createMessage() => BackupFile._();
  @$core.override
  BackupFile createEmptyInstance() => BackupFile._();
  @$core.pragma('dart2js:noInline')
  static BackupFile getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupFile>(BackupFile.$_createMessage);
  static BackupFile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sizeBytes => $_getI64(2);
  @$pb.TagNumber(3)
  set sizeBytes($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSizeBytes() => $_has(2);
  @$pb.TagNumber(3)
  void clearSizeBytes() => $_clearField(3);

  /// Provider-formatted modification timestamp string.
  @$pb.TagNumber(4)
  $core.String get modifiedAt => $_getSZ(3);
  @$pb.TagNumber(4)
  set modifiedAt($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasModifiedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearModifiedAt() => $_clearField(4);
}

/// Local archive produced by a panel or compose-config backup.
class BackupArchive extends $pb.GeneratedMessage {
  factory BackupArchive({
    $core.String? path,
    $core.String? checksumSha256,
    $fixnum.Int64? sizeBytes,
    $fixnum.Int64? createdAt,
  }) {
    final result = BackupArchive._();
    if (path != null) result.path = path;
    if (checksumSha256 != null) result.checksumSha256 = checksumSha256;
    if (sizeBytes != null) result.sizeBytes = sizeBytes;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  BackupArchive._();

  factory BackupArchive.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupArchive()..mergeFromBuffer(data, registry);
  factory BackupArchive.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupArchive()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupArchive',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupArchive.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'checksumSha256')
    ..aInt64(3, _omitFieldNames ? '' : 'sizeBytes')
    ..aInt64(4, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupArchive clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupArchive copyWith(void Function(BackupArchive) updates) =>
      super.copyWith((message) => updates(message as BackupArchive))
          as BackupArchive;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupArchive() / BackupArchive.new instead')
  static BackupArchive create() => BackupArchive._();
  static $pb.GeneratedMessage $_createMessage() => BackupArchive._();
  @$core.override
  BackupArchive createEmptyInstance() => BackupArchive._();
  @$core.pragma('dart2js:noInline')
  static BackupArchive getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BackupArchive>(
          BackupArchive.$_createMessage);
  static BackupArchive? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get checksumSha256 => $_getSZ(1);
  @$pb.TagNumber(2)
  set checksumSha256($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChecksumSha256() => $_has(1);
  @$pb.TagNumber(2)
  void clearChecksumSha256() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sizeBytes => $_getI64(2);
  @$pb.TagNumber(3)
  set sizeBytes($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSizeBytes() => $_has(2);
  @$pb.TagNumber(3)
  void clearSizeBytes() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get createdAt => $_getI64(3);
  @$pb.TagNumber(4)
  set createdAt($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCreatedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreatedAt() => $_clearField(4);
}

/// Staged restore (or rollback) awaiting a panel restart.
class BackupStagedRestore extends $pb.GeneratedMessage {
  factory BackupStagedRestore({
    $core.String? restoreId,
    $core.String? checksumSha256,
    $core.bool? restartRequired,
    $core.String? pendingMarker,
  }) {
    final result = BackupStagedRestore._();
    if (restoreId != null) result.restoreId = restoreId;
    if (checksumSha256 != null) result.checksumSha256 = checksumSha256;
    if (restartRequired != null) result.restartRequired = restartRequired;
    if (pendingMarker != null) result.pendingMarker = pendingMarker;
    return result;
  }

  BackupStagedRestore._();

  factory BackupStagedRestore.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStagedRestore()..mergeFromBuffer(data, registry);
  factory BackupStagedRestore.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStagedRestore()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupStagedRestore',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupStagedRestore.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'restoreId')
    ..aOS(2, _omitFieldNames ? '' : 'checksumSha256')
    ..aOB(3, _omitFieldNames ? '' : 'restartRequired')
    ..aOS(4, _omitFieldNames ? '' : 'pendingMarker')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStagedRestore clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStagedRestore copyWith(void Function(BackupStagedRestore) updates) =>
      super.copyWith((message) => updates(message as BackupStagedRestore))
          as BackupStagedRestore;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use BackupStagedRestore() / BackupStagedRestore.new instead')
  static BackupStagedRestore create() => BackupStagedRestore._();
  static $pb.GeneratedMessage $_createMessage() => BackupStagedRestore._();
  @$core.override
  BackupStagedRestore createEmptyInstance() => BackupStagedRestore._();
  @$core.pragma('dart2js:noInline')
  static BackupStagedRestore getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupStagedRestore>(
          BackupStagedRestore.$_createMessage);
  static BackupStagedRestore? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get restoreId => $_getSZ(0);
  @$pb.TagNumber(1)
  set restoreId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRestoreId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRestoreId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get checksumSha256 => $_getSZ(1);
  @$pb.TagNumber(2)
  set checksumSha256($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChecksumSha256() => $_has(1);
  @$pb.TagNumber(2)
  void clearChecksumSha256() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get restartRequired => $_getBF(2);
  @$pb.TagNumber(3)
  set restartRequired($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRestartRequired() => $_has(2);
  @$pb.TagNumber(3)
  void clearRestartRequired() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get pendingMarker => $_getSZ(3);
  @$pb.TagNumber(4)
  set pendingMarker($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPendingMarker() => $_has(3);
  @$pb.TagNumber(4)
  void clearPendingMarker() => $_clearField(4);
}

/// kind "panel_restore_status": progress of a staged panel restore.
class BackupRestoreStatus extends $pb.GeneratedMessage {
  factory BackupRestoreStatus({
    $core.String? restoreId,
    $core.String? status,
    $core.String? message,
    $fixnum.Int64? updatedAt,
  }) {
    final result = BackupRestoreStatus._();
    if (restoreId != null) result.restoreId = restoreId;
    if (status != null) result.status = status;
    if (message != null) result.message = message;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  BackupRestoreStatus._();

  factory BackupRestoreStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreStatus()..mergeFromBuffer(data, registry);
  factory BackupRestoreStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRestoreStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRestoreStatus.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'restoreId')
    ..aOS(2, _omitFieldNames ? '' : 'status')
    ..aOS(3, _omitFieldNames ? '' : 'message')
    ..aInt64(4, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreStatus copyWith(void Function(BackupRestoreStatus) updates) =>
      super.copyWith((message) => updates(message as BackupRestoreStatus))
          as BackupRestoreStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use BackupRestoreStatus() / BackupRestoreStatus.new instead')
  static BackupRestoreStatus create() => BackupRestoreStatus._();
  static $pb.GeneratedMessage $_createMessage() => BackupRestoreStatus._();
  @$core.override
  BackupRestoreStatus createEmptyInstance() => BackupRestoreStatus._();
  @$core.pragma('dart2js:noInline')
  static BackupRestoreStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRestoreStatus>(
          BackupRestoreStatus.$_createMessage);
  static BackupRestoreStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get restoreId => $_getSZ(0);
  @$pb.TagNumber(1)
  set restoreId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRestoreId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRestoreId() => $_clearField(1);

  /// "PENDING_RESTART" while staged; terminal values come from the restore history file, so the string stays free-form.
  @$pb.TagNumber(2)
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get message => $_getSZ(2);
  @$pb.TagNumber(3)
  set message($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessage() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get updatedAt => $_getI64(3);
  @$pb.TagNumber(4)
  set updatedAt($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUpdatedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearUpdatedAt() => $_clearField(4);
}

/// kind "retention_preview": which files a keep-latest rule would keep/delete.
class BackupRetentionPreview extends $pb.GeneratedMessage {
  factory BackupRetentionPreview({
    $core.Iterable<BackupFile>? keep,
    $core.Iterable<BackupFile>? delete,
  }) {
    final result = BackupRetentionPreview._();
    if (keep != null) result.keep.addAll(keep);
    if (delete != null) result.delete.addAll(delete);
    return result;
  }

  BackupRetentionPreview._();

  factory BackupRetentionPreview.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRetentionPreview()..mergeFromBuffer(data, registry);
  factory BackupRetentionPreview.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRetentionPreview()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRetentionPreview',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRetentionPreview.$_createMessage)
    ..pPM<BackupFile>(1, _omitFieldNames ? '' : 'keep',
        subBuilder: BackupFile.$_createMessage)
    ..pPM<BackupFile>(2, _omitFieldNames ? '' : 'delete',
        subBuilder: BackupFile.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRetentionPreview clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRetentionPreview copyWith(
          void Function(BackupRetentionPreview) updates) =>
      super.copyWith((message) => updates(message as BackupRetentionPreview))
          as BackupRetentionPreview;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRetentionPreview() / BackupRetentionPreview.new instead')
  static BackupRetentionPreview create() => BackupRetentionPreview._();
  static $pb.GeneratedMessage $_createMessage() => BackupRetentionPreview._();
  @$core.override
  BackupRetentionPreview createEmptyInstance() => BackupRetentionPreview._();
  @$core.pragma('dart2js:noInline')
  static BackupRetentionPreview getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRetentionPreview>(
          BackupRetentionPreview.$_createMessage);
  static BackupRetentionPreview? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupFile> get keep => $_getList(0);

  @$pb.TagNumber(2)
  $pb.PbList<BackupFile> get delete => $_getList(1);
}

/// kind "backup_integrity": checksum verification outcome for one object.
class BackupIntegrity extends $pb.GeneratedMessage {
  factory BackupIntegrity({
    $core.bool? valid,
    $core.String? expectedSha256,
    $core.String? actualSha256,
    $fixnum.Int64? sizeBytes,
  }) {
    final result = BackupIntegrity._();
    if (valid != null) result.valid = valid;
    if (expectedSha256 != null) result.expectedSha256 = expectedSha256;
    if (actualSha256 != null) result.actualSha256 = actualSha256;
    if (sizeBytes != null) result.sizeBytes = sizeBytes;
    return result;
  }

  BackupIntegrity._();

  factory BackupIntegrity.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupIntegrity()..mergeFromBuffer(data, registry);
  factory BackupIntegrity.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupIntegrity()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupIntegrity',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupIntegrity.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'valid')
    ..aOS(2, _omitFieldNames ? '' : 'expectedSha256')
    ..aOS(3, _omitFieldNames ? '' : 'actualSha256')
    ..aInt64(4, _omitFieldNames ? '' : 'sizeBytes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupIntegrity clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupIntegrity copyWith(void Function(BackupIntegrity) updates) =>
      super.copyWith((message) => updates(message as BackupIntegrity))
          as BackupIntegrity;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BackupIntegrity() / BackupIntegrity.new instead')
  static BackupIntegrity create() => BackupIntegrity._();
  static $pb.GeneratedMessage $_createMessage() => BackupIntegrity._();
  @$core.override
  BackupIntegrity createEmptyInstance() => BackupIntegrity._();
  @$core.pragma('dart2js:noInline')
  static BackupIntegrity getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BackupIntegrity>(
          BackupIntegrity.$_createMessage);
  static BackupIntegrity? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get valid => $_getBF(0);
  @$pb.TagNumber(1)
  set valid($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasValid() => $_has(0);
  @$pb.TagNumber(1)
  void clearValid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get expectedSha256 => $_getSZ(1);
  @$pb.TagNumber(2)
  set expectedSha256($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExpectedSha256() => $_has(1);
  @$pb.TagNumber(2)
  void clearExpectedSha256() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get actualSha256 => $_getSZ(2);
  @$pb.TagNumber(3)
  set actualSha256($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasActualSha256() => $_has(2);
  @$pb.TagNumber(3)
  void clearActualSha256() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get sizeBytes => $_getI64(3);
  @$pb.TagNumber(4)
  set sizeBytes($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSizeBytes() => $_has(3);
  @$pb.TagNumber(4)
  void clearSizeBytes() => $_clearField(4);
}

class BackupListBackupsRequest extends $pb.GeneratedMessage {
  factory BackupListBackupsRequest() => BackupListBackupsRequest._();

  BackupListBackupsRequest._();

  factory BackupListBackupsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListBackupsRequest()..mergeFromBuffer(data, registry);
  factory BackupListBackupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListBackupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListBackupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListBackupsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListBackupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListBackupsRequest copyWith(
          void Function(BackupListBackupsRequest) updates) =>
      super.copyWith((message) => updates(message as BackupListBackupsRequest))
          as BackupListBackupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListBackupsRequest() / BackupListBackupsRequest.new instead')
  static BackupListBackupsRequest create() => BackupListBackupsRequest._();
  static $pb.GeneratedMessage $_createMessage() => BackupListBackupsRequest._();
  @$core.override
  BackupListBackupsRequest createEmptyInstance() =>
      BackupListBackupsRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupListBackupsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListBackupsRequest>(
          BackupListBackupsRequest.$_createMessage);
  static BackupListBackupsRequest? _defaultInstance;
}

class BackupListBackupsResponse extends $pb.GeneratedMessage {
  factory BackupListBackupsResponse({
    $core.Iterable<BackupDatabase>? backups,
  }) {
    final result = BackupListBackupsResponse._();
    if (backups != null) result.backups.addAll(backups);
    return result;
  }

  BackupListBackupsResponse._();

  factory BackupListBackupsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListBackupsResponse()..mergeFromBuffer(data, registry);
  factory BackupListBackupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListBackupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListBackupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListBackupsResponse.$_createMessage)
    ..pPM<BackupDatabase>(1, _omitFieldNames ? '' : 'backups',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListBackupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListBackupsResponse copyWith(
          void Function(BackupListBackupsResponse) updates) =>
      super.copyWith((message) => updates(message as BackupListBackupsResponse))
          as BackupListBackupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListBackupsResponse() / BackupListBackupsResponse.new instead')
  static BackupListBackupsResponse create() => BackupListBackupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListBackupsResponse._();
  @$core.override
  BackupListBackupsResponse createEmptyInstance() =>
      BackupListBackupsResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupListBackupsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListBackupsResponse>(
          BackupListBackupsResponse.$_createMessage);
  static BackupListBackupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupDatabase> get backups => $_getList(0);
}

class BackupGetBackupRequest extends $pb.GeneratedMessage {
  factory BackupGetBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupGetBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupGetBackupRequest._();

  factory BackupGetBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupGetBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetBackupRequest copyWith(
          void Function(BackupGetBackupRequest) updates) =>
      super.copyWith((message) => updates(message as BackupGetBackupRequest))
          as BackupGetBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetBackupRequest() / BackupGetBackupRequest.new instead')
  static BackupGetBackupRequest create() => BackupGetBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() => BackupGetBackupRequest._();
  @$core.override
  BackupGetBackupRequest createEmptyInstance() => BackupGetBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupGetBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetBackupRequest>(
          BackupGetBackupRequest.$_createMessage);
  static BackupGetBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupGetBackupResponse extends $pb.GeneratedMessage {
  factory BackupGetBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupGetBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupGetBackupResponse._();

  factory BackupGetBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupGetBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetBackupResponse copyWith(
          void Function(BackupGetBackupResponse) updates) =>
      super.copyWith((message) => updates(message as BackupGetBackupResponse))
          as BackupGetBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetBackupResponse() / BackupGetBackupResponse.new instead')
  static BackupGetBackupResponse create() => BackupGetBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() => BackupGetBackupResponse._();
  @$core.override
  BackupGetBackupResponse createEmptyInstance() => BackupGetBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupGetBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetBackupResponse>(
          BackupGetBackupResponse.$_createMessage);
  static BackupGetBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

class BackupCreateBackupRequest extends $pb.GeneratedMessage {
  factory BackupCreateBackupRequest({
    $core.String? appName,
    $core.String? schedule,
    $core.String? databaseName,
    $core.String? prefix,
    $core.String? serviceName,
    $fixnum.Int64? keepLatestCount,
    $core.String? backupType,
    $core.String? databaseType,
    $core.String? metadata,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? destinationId,
    $fixnum.Int64? organizationId,
  }) {
    final result = BackupCreateBackupRequest._();
    if (appName != null) result.appName = appName;
    if (schedule != null) result.schedule = schedule;
    if (databaseName != null) result.databaseName = databaseName;
    if (prefix != null) result.prefix = prefix;
    if (serviceName != null) result.serviceName = serviceName;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (backupType != null) result.backupType = backupType;
    if (databaseType != null) result.databaseType = databaseType;
    if (metadata != null) result.metadata = metadata;
    if (serviceId != null) result.serviceId = serviceId;
    if (destinationId != null) result.destinationId = destinationId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  BackupCreateBackupRequest._();

  factory BackupCreateBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupCreateBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateBackupRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'appName')
    ..aOS(2, _omitFieldNames ? '' : 'schedule')
    ..aOS(3, _omitFieldNames ? '' : 'databaseName')
    ..aOS(4, _omitFieldNames ? '' : 'prefix')
    ..aOS(5, _omitFieldNames ? '' : 'serviceName')
    ..aInt64(6, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOS(7, _omitFieldNames ? '' : 'backupType')
    ..aOS(8, _omitFieldNames ? '' : 'databaseType')
    ..aOS(9, _omitFieldNames ? '' : 'metadata')
    ..aInt64(10, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(11, _omitFieldNames ? '' : 'destinationId')
    ..aInt64(12, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateBackupRequest copyWith(
          void Function(BackupCreateBackupRequest) updates) =>
      super.copyWith((message) => updates(message as BackupCreateBackupRequest))
          as BackupCreateBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateBackupRequest() / BackupCreateBackupRequest.new instead')
  static BackupCreateBackupRequest create() => BackupCreateBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateBackupRequest._();
  @$core.override
  BackupCreateBackupRequest createEmptyInstance() =>
      BackupCreateBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateBackupRequest>(
          BackupCreateBackupRequest.$_createMessage);
  static BackupCreateBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get appName => $_getSZ(0);
  @$pb.TagNumber(1)
  set appName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAppName() => $_has(0);
  @$pb.TagNumber(1)
  void clearAppName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get schedule => $_getSZ(1);
  @$pb.TagNumber(2)
  set schedule($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSchedule() => $_has(1);
  @$pb.TagNumber(2)
  void clearSchedule() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get databaseName => $_getSZ(2);
  @$pb.TagNumber(3)
  set databaseName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDatabaseName() => $_has(2);
  @$pb.TagNumber(3)
  void clearDatabaseName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get prefix => $_getSZ(3);
  @$pb.TagNumber(4)
  set prefix($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrefix() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrefix() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get serviceName => $_getSZ(4);
  @$pb.TagNumber(5)
  set serviceName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServiceName() => $_has(4);
  @$pb.TagNumber(5)
  void clearServiceName() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get keepLatestCount => $_getI64(5);
  @$pb.TagNumber(6)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasKeepLatestCount() => $_has(5);
  @$pb.TagNumber(6)
  void clearKeepLatestCount() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get backupType => $_getSZ(6);
  @$pb.TagNumber(7)
  set backupType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBackupType() => $_has(6);
  @$pb.TagNumber(7)
  void clearBackupType() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get databaseType => $_getSZ(7);
  @$pb.TagNumber(8)
  set databaseType($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabaseType() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabaseType() => $_clearField(8);

  /// Optional JSON document with extra backup configuration.
  @$pb.TagNumber(9)
  $core.String get metadata => $_getSZ(8);
  @$pb.TagNumber(9)
  set metadata($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasMetadata() => $_has(8);
  @$pb.TagNumber(9)
  void clearMetadata() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get serviceId => $_getI64(9);
  @$pb.TagNumber(10)
  set serviceId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasServiceId() => $_has(9);
  @$pb.TagNumber(10)
  void clearServiceId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get destinationId => $_getI64(10);
  @$pb.TagNumber(11)
  set destinationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDestinationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearDestinationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get organizationId => $_getI64(11);
  @$pb.TagNumber(12)
  set organizationId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOrganizationId() => $_has(11);
  @$pb.TagNumber(12)
  void clearOrganizationId() => $_clearField(12);
}

class BackupCreateBackupResponse extends $pb.GeneratedMessage {
  factory BackupCreateBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupCreateBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupCreateBackupResponse._();

  factory BackupCreateBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupCreateBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateBackupResponse copyWith(
          void Function(BackupCreateBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupCreateBackupResponse))
          as BackupCreateBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateBackupResponse() / BackupCreateBackupResponse.new instead')
  static BackupCreateBackupResponse create() => BackupCreateBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateBackupResponse._();
  @$core.override
  BackupCreateBackupResponse createEmptyInstance() =>
      BackupCreateBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateBackupResponse>(
          BackupCreateBackupResponse.$_createMessage);
  static BackupCreateBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

/// An absent field keeps its stored value; `enabled` replaces the i64 flag.
class BackupUpdateBackupRequest extends $pb.GeneratedMessage {
  factory BackupUpdateBackupRequest({
    $fixnum.Int64? id,
    $core.String? appName,
    $core.String? schedule,
    $core.String? databaseName,
    $core.String? prefix,
    $core.String? serviceName,
    $fixnum.Int64? keepLatestCount,
    $core.String? backupType,
    $core.String? databaseType,
    $core.String? metadata,
    $fixnum.Int64? destinationId,
    $core.bool? enabled,
  }) {
    final result = BackupUpdateBackupRequest._();
    if (id != null) result.id = id;
    if (appName != null) result.appName = appName;
    if (schedule != null) result.schedule = schedule;
    if (databaseName != null) result.databaseName = databaseName;
    if (prefix != null) result.prefix = prefix;
    if (serviceName != null) result.serviceName = serviceName;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (backupType != null) result.backupType = backupType;
    if (databaseType != null) result.databaseType = databaseType;
    if (metadata != null) result.metadata = metadata;
    if (destinationId != null) result.destinationId = destinationId;
    if (enabled != null) result.enabled = enabled;
    return result;
  }

  BackupUpdateBackupRequest._();

  factory BackupUpdateBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupUpdateBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'appName')
    ..aOS(3, _omitFieldNames ? '' : 'schedule')
    ..aOS(4, _omitFieldNames ? '' : 'databaseName')
    ..aOS(5, _omitFieldNames ? '' : 'prefix')
    ..aOS(6, _omitFieldNames ? '' : 'serviceName')
    ..aInt64(7, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOS(8, _omitFieldNames ? '' : 'backupType')
    ..aOS(9, _omitFieldNames ? '' : 'databaseType')
    ..aOS(10, _omitFieldNames ? '' : 'metadata')
    ..aInt64(11, _omitFieldNames ? '' : 'destinationId')
    ..aOB(12, _omitFieldNames ? '' : 'enabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateBackupRequest copyWith(
          void Function(BackupUpdateBackupRequest) updates) =>
      super.copyWith((message) => updates(message as BackupUpdateBackupRequest))
          as BackupUpdateBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateBackupRequest() / BackupUpdateBackupRequest.new instead')
  static BackupUpdateBackupRequest create() => BackupUpdateBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateBackupRequest._();
  @$core.override
  BackupUpdateBackupRequest createEmptyInstance() =>
      BackupUpdateBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateBackupRequest>(
          BackupUpdateBackupRequest.$_createMessage);
  static BackupUpdateBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get appName => $_getSZ(1);
  @$pb.TagNumber(2)
  set appName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAppName() => $_has(1);
  @$pb.TagNumber(2)
  void clearAppName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get schedule => $_getSZ(2);
  @$pb.TagNumber(3)
  set schedule($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSchedule() => $_has(2);
  @$pb.TagNumber(3)
  void clearSchedule() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get databaseName => $_getSZ(3);
  @$pb.TagNumber(4)
  set databaseName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDatabaseName() => $_has(3);
  @$pb.TagNumber(4)
  void clearDatabaseName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get prefix => $_getSZ(4);
  @$pb.TagNumber(5)
  set prefix($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPrefix() => $_has(4);
  @$pb.TagNumber(5)
  void clearPrefix() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serviceName => $_getSZ(5);
  @$pb.TagNumber(6)
  set serviceName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServiceName() => $_has(5);
  @$pb.TagNumber(6)
  void clearServiceName() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get keepLatestCount => $_getI64(6);
  @$pb.TagNumber(7)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasKeepLatestCount() => $_has(6);
  @$pb.TagNumber(7)
  void clearKeepLatestCount() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get backupType => $_getSZ(7);
  @$pb.TagNumber(8)
  set backupType($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBackupType() => $_has(7);
  @$pb.TagNumber(8)
  void clearBackupType() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get databaseType => $_getSZ(8);
  @$pb.TagNumber(9)
  set databaseType($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasDatabaseType() => $_has(8);
  @$pb.TagNumber(9)
  void clearDatabaseType() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get metadata => $_getSZ(9);
  @$pb.TagNumber(10)
  set metadata($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasMetadata() => $_has(9);
  @$pb.TagNumber(10)
  void clearMetadata() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get destinationId => $_getI64(10);
  @$pb.TagNumber(11)
  set destinationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDestinationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearDestinationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get enabled => $_getBF(11);
  @$pb.TagNumber(12)
  set enabled($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnabled() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnabled() => $_clearField(12);
}

class BackupUpdateBackupResponse extends $pb.GeneratedMessage {
  factory BackupUpdateBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupUpdateBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupUpdateBackupResponse._();

  factory BackupUpdateBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupUpdateBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateBackupResponse copyWith(
          void Function(BackupUpdateBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupUpdateBackupResponse))
          as BackupUpdateBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateBackupResponse() / BackupUpdateBackupResponse.new instead')
  static BackupUpdateBackupResponse create() => BackupUpdateBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateBackupResponse._();
  @$core.override
  BackupUpdateBackupResponse createEmptyInstance() =>
      BackupUpdateBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateBackupResponse>(
          BackupUpdateBackupResponse.$_createMessage);
  static BackupUpdateBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

class BackupDeleteBackupRequest extends $pb.GeneratedMessage {
  factory BackupDeleteBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupDeleteBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupDeleteBackupRequest._();

  factory BackupDeleteBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupDeleteBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDeleteBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDeleteBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteBackupRequest copyWith(
          void Function(BackupDeleteBackupRequest) updates) =>
      super.copyWith((message) => updates(message as BackupDeleteBackupRequest))
          as BackupDeleteBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupDeleteBackupRequest() / BackupDeleteBackupRequest.new instead')
  static BackupDeleteBackupRequest create() => BackupDeleteBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupDeleteBackupRequest._();
  @$core.override
  BackupDeleteBackupRequest createEmptyInstance() =>
      BackupDeleteBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupDeleteBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupDeleteBackupRequest>(
          BackupDeleteBackupRequest.$_createMessage);
  static BackupDeleteBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupRunBackupRequest extends $pb.GeneratedMessage {
  factory BackupRunBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupRunBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupRunBackupRequest._();

  factory BackupRunBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRunBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunBackupRequest copyWith(
          void Function(BackupRunBackupRequest) updates) =>
      super.copyWith((message) => updates(message as BackupRunBackupRequest))
          as BackupRunBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunBackupRequest() / BackupRunBackupRequest.new instead')
  static BackupRunBackupRequest create() => BackupRunBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() => BackupRunBackupRequest._();
  @$core.override
  BackupRunBackupRequest createEmptyInstance() => BackupRunBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRunBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunBackupRequest>(
          BackupRunBackupRequest.$_createMessage);
  static BackupRunBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupRestoreBackupRequest extends $pb.GeneratedMessage {
  factory BackupRestoreBackupRequest({
    $fixnum.Int64? id,
    $core.String? backupFile,
  }) {
    final result = BackupRestoreBackupRequest._();
    if (id != null) result.id = id;
    if (backupFile != null) result.backupFile = backupFile;
    return result;
  }

  BackupRestoreBackupRequest._();

  factory BackupRestoreBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRestoreBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRestoreBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRestoreBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'backupFile')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreBackupRequest copyWith(
          void Function(BackupRestoreBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRestoreBackupRequest))
          as BackupRestoreBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRestoreBackupRequest() / BackupRestoreBackupRequest.new instead')
  static BackupRestoreBackupRequest create() => BackupRestoreBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRestoreBackupRequest._();
  @$core.override
  BackupRestoreBackupRequest createEmptyInstance() =>
      BackupRestoreBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRestoreBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRestoreBackupRequest>(
          BackupRestoreBackupRequest.$_createMessage);
  static BackupRestoreBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get backupFile => $_getSZ(1);
  @$pb.TagNumber(2)
  set backupFile($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBackupFile() => $_has(1);
  @$pb.TagNumber(2)
  void clearBackupFile() => $_clearField(2);
}

class BackupValidateDestinationRequest extends $pb.GeneratedMessage {
  factory BackupValidateDestinationRequest({
    $fixnum.Int64? destinationId,
  }) {
    final result = BackupValidateDestinationRequest._();
    if (destinationId != null) result.destinationId = destinationId;
    return result;
  }

  BackupValidateDestinationRequest._();

  factory BackupValidateDestinationRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupValidateDestinationRequest()..mergeFromBuffer(data, registry);
  factory BackupValidateDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupValidateDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupValidateDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupValidateDestinationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'destinationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupValidateDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupValidateDestinationRequest copyWith(
          void Function(BackupValidateDestinationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupValidateDestinationRequest))
          as BackupValidateDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupValidateDestinationRequest() / BackupValidateDestinationRequest.new instead')
  static BackupValidateDestinationRequest create() =>
      BackupValidateDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupValidateDestinationRequest._();
  @$core.override
  BackupValidateDestinationRequest createEmptyInstance() =>
      BackupValidateDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupValidateDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupValidateDestinationRequest>(
          BackupValidateDestinationRequest.$_createMessage);
  static BackupValidateDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get destinationId => $_getI64(0);
  @$pb.TagNumber(1)
  set destinationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDestinationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestinationId() => $_clearField(1);
}

class BackupValidateDestinationResponse extends $pb.GeneratedMessage {
  factory BackupValidateDestinationResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = BackupValidateDestinationResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  BackupValidateDestinationResponse._();

  factory BackupValidateDestinationResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupValidateDestinationResponse()..mergeFromBuffer(data, registry);
  factory BackupValidateDestinationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupValidateDestinationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupValidateDestinationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupValidateDestinationResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupValidateDestinationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupValidateDestinationResponse copyWith(
          void Function(BackupValidateDestinationResponse) updates) =>
      super.copyWith((message) =>
              updates(message as BackupValidateDestinationResponse))
          as BackupValidateDestinationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupValidateDestinationResponse() / BackupValidateDestinationResponse.new instead')
  static BackupValidateDestinationResponse create() =>
      BackupValidateDestinationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupValidateDestinationResponse._();
  @$core.override
  BackupValidateDestinationResponse createEmptyInstance() =>
      BackupValidateDestinationResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupValidateDestinationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupValidateDestinationResponse>(
          BackupValidateDestinationResponse.$_createMessage);
  static BackupValidateDestinationResponse? _defaultInstance;

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

class BackupRunComposeBackupRequest extends $pb.GeneratedMessage {
  factory BackupRunComposeBackupRequest({
    $fixnum.Int64? composeId,
    $core.bool? includeSecrets,
  }) {
    final result = BackupRunComposeBackupRequest._();
    if (composeId != null) result.composeId = composeId;
    if (includeSecrets != null) result.includeSecrets = includeSecrets;
    return result;
  }

  BackupRunComposeBackupRequest._();

  factory BackupRunComposeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunComposeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRunComposeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunComposeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunComposeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunComposeBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aOB(2, _omitFieldNames ? '' : 'includeSecrets')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunComposeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunComposeBackupRequest copyWith(
          void Function(BackupRunComposeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunComposeBackupRequest))
          as BackupRunComposeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunComposeBackupRequest() / BackupRunComposeBackupRequest.new instead')
  static BackupRunComposeBackupRequest create() =>
      BackupRunComposeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunComposeBackupRequest._();
  @$core.override
  BackupRunComposeBackupRequest createEmptyInstance() =>
      BackupRunComposeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRunComposeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunComposeBackupRequest>(
          BackupRunComposeBackupRequest.$_createMessage);
  static BackupRunComposeBackupRequest? _defaultInstance;

  /// Compose id the config bundle is exported for (former envelope `id`).
  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get includeSecrets => $_getBF(1);
  @$pb.TagNumber(2)
  set includeSecrets($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasIncludeSecrets() => $_has(1);
  @$pb.TagNumber(2)
  void clearIncludeSecrets() => $_clearField(2);
}

class BackupRunComposeBackupResponse extends $pb.GeneratedMessage {
  factory BackupRunComposeBackupResponse({
    BackupArchive? archive,
  }) {
    final result = BackupRunComposeBackupResponse._();
    if (archive != null) result.archive = archive;
    return result;
  }

  BackupRunComposeBackupResponse._();

  factory BackupRunComposeBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunComposeBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupRunComposeBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunComposeBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunComposeBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunComposeBackupResponse.$_createMessage)
    ..aOM<BackupArchive>(1, _omitFieldNames ? '' : 'archive',
        subBuilder: BackupArchive.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunComposeBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunComposeBackupResponse copyWith(
          void Function(BackupRunComposeBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunComposeBackupResponse))
          as BackupRunComposeBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunComposeBackupResponse() / BackupRunComposeBackupResponse.new instead')
  static BackupRunComposeBackupResponse create() =>
      BackupRunComposeBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunComposeBackupResponse._();
  @$core.override
  BackupRunComposeBackupResponse createEmptyInstance() =>
      BackupRunComposeBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupRunComposeBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunComposeBackupResponse>(
          BackupRunComposeBackupResponse.$_createMessage);
  static BackupRunComposeBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupArchive get archive => $_getN(0);
  @$pb.TagNumber(1)
  set archive(BackupArchive value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupArchive ensureArchive() => $_ensure(0);
}

class BackupRunPanelBackupRequest extends $pb.GeneratedMessage {
  factory BackupRunPanelBackupRequest() => BackupRunPanelBackupRequest._();

  BackupRunPanelBackupRequest._();

  factory BackupRunPanelBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunPanelBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRunPanelBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunPanelBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunPanelBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunPanelBackupRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunPanelBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunPanelBackupRequest copyWith(
          void Function(BackupRunPanelBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunPanelBackupRequest))
          as BackupRunPanelBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunPanelBackupRequest() / BackupRunPanelBackupRequest.new instead')
  static BackupRunPanelBackupRequest create() =>
      BackupRunPanelBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunPanelBackupRequest._();
  @$core.override
  BackupRunPanelBackupRequest createEmptyInstance() =>
      BackupRunPanelBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRunPanelBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunPanelBackupRequest>(
          BackupRunPanelBackupRequest.$_createMessage);
  static BackupRunPanelBackupRequest? _defaultInstance;
}

class BackupRunPanelBackupResponse extends $pb.GeneratedMessage {
  factory BackupRunPanelBackupResponse({
    BackupArchive? archive,
  }) {
    final result = BackupRunPanelBackupResponse._();
    if (archive != null) result.archive = archive;
    return result;
  }

  BackupRunPanelBackupResponse._();

  factory BackupRunPanelBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunPanelBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupRunPanelBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunPanelBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunPanelBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunPanelBackupResponse.$_createMessage)
    ..aOM<BackupArchive>(1, _omitFieldNames ? '' : 'archive',
        subBuilder: BackupArchive.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunPanelBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunPanelBackupResponse copyWith(
          void Function(BackupRunPanelBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunPanelBackupResponse))
          as BackupRunPanelBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunPanelBackupResponse() / BackupRunPanelBackupResponse.new instead')
  static BackupRunPanelBackupResponse create() =>
      BackupRunPanelBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunPanelBackupResponse._();
  @$core.override
  BackupRunPanelBackupResponse createEmptyInstance() =>
      BackupRunPanelBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupRunPanelBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunPanelBackupResponse>(
          BackupRunPanelBackupResponse.$_createMessage);
  static BackupRunPanelBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupArchive get archive => $_getN(0);
  @$pb.TagNumber(1)
  set archive(BackupArchive value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupArchive ensureArchive() => $_ensure(0);
}

class BackupStageRestoreRequest extends $pb.GeneratedMessage {
  factory BackupStageRestoreRequest({
    $core.String? archive,
    $core.String? checksumSha256,
  }) {
    final result = BackupStageRestoreRequest._();
    if (archive != null) result.archive = archive;
    if (checksumSha256 != null) result.checksumSha256 = checksumSha256;
    return result;
  }

  BackupStageRestoreRequest._();

  factory BackupStageRestoreRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStageRestoreRequest()..mergeFromBuffer(data, registry);
  factory BackupStageRestoreRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStageRestoreRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupStageRestoreRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupStageRestoreRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'archive')
    ..aOS(2, _omitFieldNames ? '' : 'checksumSha256')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStageRestoreRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStageRestoreRequest copyWith(
          void Function(BackupStageRestoreRequest) updates) =>
      super.copyWith((message) => updates(message as BackupStageRestoreRequest))
          as BackupStageRestoreRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupStageRestoreRequest() / BackupStageRestoreRequest.new instead')
  static BackupStageRestoreRequest create() => BackupStageRestoreRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupStageRestoreRequest._();
  @$core.override
  BackupStageRestoreRequest createEmptyInstance() =>
      BackupStageRestoreRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupStageRestoreRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupStageRestoreRequest>(
          BackupStageRestoreRequest.$_createMessage);
  static BackupStageRestoreRequest? _defaultInstance;

  /// Absolute path to a .tar.gz panel archive inside the backup directory.
  @$pb.TagNumber(1)
  $core.String get archive => $_getSZ(0);
  @$pb.TagNumber(1)
  set archive($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get checksumSha256 => $_getSZ(1);
  @$pb.TagNumber(2)
  set checksumSha256($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChecksumSha256() => $_has(1);
  @$pb.TagNumber(2)
  void clearChecksumSha256() => $_clearField(2);
}

class BackupStageRestoreResponse extends $pb.GeneratedMessage {
  factory BackupStageRestoreResponse({
    BackupStagedRestore? staged,
  }) {
    final result = BackupStageRestoreResponse._();
    if (staged != null) result.staged = staged;
    return result;
  }

  BackupStageRestoreResponse._();

  factory BackupStageRestoreResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStageRestoreResponse()..mergeFromBuffer(data, registry);
  factory BackupStageRestoreResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupStageRestoreResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupStageRestoreResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupStageRestoreResponse.$_createMessage)
    ..aOM<BackupStagedRestore>(1, _omitFieldNames ? '' : 'staged',
        subBuilder: BackupStagedRestore.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStageRestoreResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupStageRestoreResponse copyWith(
          void Function(BackupStageRestoreResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupStageRestoreResponse))
          as BackupStageRestoreResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupStageRestoreResponse() / BackupStageRestoreResponse.new instead')
  static BackupStageRestoreResponse create() => BackupStageRestoreResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupStageRestoreResponse._();
  @$core.override
  BackupStageRestoreResponse createEmptyInstance() =>
      BackupStageRestoreResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupStageRestoreResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupStageRestoreResponse>(
          BackupStageRestoreResponse.$_createMessage);
  static BackupStageRestoreResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupStagedRestore get staged => $_getN(0);
  @$pb.TagNumber(1)
  set staged(BackupStagedRestore value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStaged() => $_has(0);
  @$pb.TagNumber(1)
  void clearStaged() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupStagedRestore ensureStaged() => $_ensure(0);
}

class BackupGetRestoreStatusRequest extends $pb.GeneratedMessage {
  factory BackupGetRestoreStatusRequest({
    $core.String? restoreId,
  }) {
    final result = BackupGetRestoreStatusRequest._();
    if (restoreId != null) result.restoreId = restoreId;
    return result;
  }

  BackupGetRestoreStatusRequest._();

  factory BackupGetRestoreStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetRestoreStatusRequest()..mergeFromBuffer(data, registry);
  factory BackupGetRestoreStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetRestoreStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetRestoreStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetRestoreStatusRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'restoreId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetRestoreStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetRestoreStatusRequest copyWith(
          void Function(BackupGetRestoreStatusRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetRestoreStatusRequest))
          as BackupGetRestoreStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetRestoreStatusRequest() / BackupGetRestoreStatusRequest.new instead')
  static BackupGetRestoreStatusRequest create() =>
      BackupGetRestoreStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetRestoreStatusRequest._();
  @$core.override
  BackupGetRestoreStatusRequest createEmptyInstance() =>
      BackupGetRestoreStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupGetRestoreStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetRestoreStatusRequest>(
          BackupGetRestoreStatusRequest.$_createMessage);
  static BackupGetRestoreStatusRequest? _defaultInstance;

  /// Restore handle returned by StageRestore/RollbackRestore.
  @$pb.TagNumber(1)
  $core.String get restoreId => $_getSZ(0);
  @$pb.TagNumber(1)
  set restoreId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRestoreId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRestoreId() => $_clearField(1);
}

class BackupGetRestoreStatusResponse extends $pb.GeneratedMessage {
  factory BackupGetRestoreStatusResponse({
    BackupRestoreStatus? status,
  }) {
    final result = BackupGetRestoreStatusResponse._();
    if (status != null) result.status = status;
    return result;
  }

  BackupGetRestoreStatusResponse._();

  factory BackupGetRestoreStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetRestoreStatusResponse()..mergeFromBuffer(data, registry);
  factory BackupGetRestoreStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetRestoreStatusResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetRestoreStatusResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetRestoreStatusResponse.$_createMessage)
    ..aOM<BackupRestoreStatus>(1, _omitFieldNames ? '' : 'status',
        subBuilder: BackupRestoreStatus.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetRestoreStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetRestoreStatusResponse copyWith(
          void Function(BackupGetRestoreStatusResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetRestoreStatusResponse))
          as BackupGetRestoreStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetRestoreStatusResponse() / BackupGetRestoreStatusResponse.new instead')
  static BackupGetRestoreStatusResponse create() =>
      BackupGetRestoreStatusResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetRestoreStatusResponse._();
  @$core.override
  BackupGetRestoreStatusResponse createEmptyInstance() =>
      BackupGetRestoreStatusResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupGetRestoreStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetRestoreStatusResponse>(
          BackupGetRestoreStatusResponse.$_createMessage);
  static BackupGetRestoreStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupRestoreStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(BackupRestoreStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupRestoreStatus ensureStatus() => $_ensure(0);
}

class BackupRollbackRestoreRequest extends $pb.GeneratedMessage {
  factory BackupRollbackRestoreRequest({
    $core.String? recoveryDatabase,
  }) {
    final result = BackupRollbackRestoreRequest._();
    if (recoveryDatabase != null) result.recoveryDatabase = recoveryDatabase;
    return result;
  }

  BackupRollbackRestoreRequest._();

  factory BackupRollbackRestoreRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRollbackRestoreRequest()..mergeFromBuffer(data, registry);
  factory BackupRollbackRestoreRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRollbackRestoreRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRollbackRestoreRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRollbackRestoreRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'recoveryDatabase')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRollbackRestoreRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRollbackRestoreRequest copyWith(
          void Function(BackupRollbackRestoreRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRollbackRestoreRequest))
          as BackupRollbackRestoreRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRollbackRestoreRequest() / BackupRollbackRestoreRequest.new instead')
  static BackupRollbackRestoreRequest create() =>
      BackupRollbackRestoreRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRollbackRestoreRequest._();
  @$core.override
  BackupRollbackRestoreRequest createEmptyInstance() =>
      BackupRollbackRestoreRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRollbackRestoreRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRollbackRestoreRequest>(
          BackupRollbackRestoreRequest.$_createMessage);
  static BackupRollbackRestoreRequest? _defaultInstance;

  /// Absolute path to a `.pre-restore-` database snapshot.
  @$pb.TagNumber(1)
  $core.String get recoveryDatabase => $_getSZ(0);
  @$pb.TagNumber(1)
  set recoveryDatabase($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRecoveryDatabase() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecoveryDatabase() => $_clearField(1);
}

class BackupRollbackRestoreResponse extends $pb.GeneratedMessage {
  factory BackupRollbackRestoreResponse({
    BackupStagedRestore? staged,
  }) {
    final result = BackupRollbackRestoreResponse._();
    if (staged != null) result.staged = staged;
    return result;
  }

  BackupRollbackRestoreResponse._();

  factory BackupRollbackRestoreResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRollbackRestoreResponse()..mergeFromBuffer(data, registry);
  factory BackupRollbackRestoreResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRollbackRestoreResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRollbackRestoreResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRollbackRestoreResponse.$_createMessage)
    ..aOM<BackupStagedRestore>(1, _omitFieldNames ? '' : 'staged',
        subBuilder: BackupStagedRestore.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRollbackRestoreResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRollbackRestoreResponse copyWith(
          void Function(BackupRollbackRestoreResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRollbackRestoreResponse))
          as BackupRollbackRestoreResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRollbackRestoreResponse() / BackupRollbackRestoreResponse.new instead')
  static BackupRollbackRestoreResponse create() =>
      BackupRollbackRestoreResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRollbackRestoreResponse._();
  @$core.override
  BackupRollbackRestoreResponse createEmptyInstance() =>
      BackupRollbackRestoreResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupRollbackRestoreResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRollbackRestoreResponse>(
          BackupRollbackRestoreResponse.$_createMessage);
  static BackupRollbackRestoreResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupStagedRestore get staged => $_getN(0);
  @$pb.TagNumber(1)
  set staged(BackupStagedRestore value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStaged() => $_has(0);
  @$pb.TagNumber(1)
  void clearStaged() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupStagedRestore ensureStaged() => $_ensure(0);
}

class BackupListFilesRequest extends $pb.GeneratedMessage {
  factory BackupListFilesRequest({
    $fixnum.Int64? destinationId,
    $core.String? prefix,
  }) {
    final result = BackupListFilesRequest._();
    if (destinationId != null) result.destinationId = destinationId;
    if (prefix != null) result.prefix = prefix;
    return result;
  }

  BackupListFilesRequest._();

  factory BackupListFilesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListFilesRequest()..mergeFromBuffer(data, registry);
  factory BackupListFilesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListFilesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListFilesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListFilesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'destinationId')
    ..aOS(2, _omitFieldNames ? '' : 'prefix')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListFilesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListFilesRequest copyWith(
          void Function(BackupListFilesRequest) updates) =>
      super.copyWith((message) => updates(message as BackupListFilesRequest))
          as BackupListFilesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListFilesRequest() / BackupListFilesRequest.new instead')
  static BackupListFilesRequest create() => BackupListFilesRequest._();
  static $pb.GeneratedMessage $_createMessage() => BackupListFilesRequest._();
  @$core.override
  BackupListFilesRequest createEmptyInstance() => BackupListFilesRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupListFilesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListFilesRequest>(
          BackupListFilesRequest.$_createMessage);
  static BackupListFilesRequest? _defaultInstance;

  /// S3 destination the objects are listed through (former envelope `organization_id` slot).
  @$pb.TagNumber(1)
  $fixnum.Int64 get destinationId => $_getI64(0);
  @$pb.TagNumber(1)
  set destinationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDestinationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestinationId() => $_clearField(1);

  /// Object key prefix filter; empty lists the whole destination.
  @$pb.TagNumber(2)
  $core.String get prefix => $_getSZ(1);
  @$pb.TagNumber(2)
  set prefix($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPrefix() => $_has(1);
  @$pb.TagNumber(2)
  void clearPrefix() => $_clearField(2);
}

class BackupListFilesResponse extends $pb.GeneratedMessage {
  factory BackupListFilesResponse({
    $core.Iterable<BackupFile>? files,
  }) {
    final result = BackupListFilesResponse._();
    if (files != null) result.files.addAll(files);
    return result;
  }

  BackupListFilesResponse._();

  factory BackupListFilesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListFilesResponse()..mergeFromBuffer(data, registry);
  factory BackupListFilesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListFilesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListFilesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListFilesResponse.$_createMessage)
    ..pPM<BackupFile>(1, _omitFieldNames ? '' : 'files',
        subBuilder: BackupFile.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListFilesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListFilesResponse copyWith(
          void Function(BackupListFilesResponse) updates) =>
      super.copyWith((message) => updates(message as BackupListFilesResponse))
          as BackupListFilesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListFilesResponse() / BackupListFilesResponse.new instead')
  static BackupListFilesResponse create() => BackupListFilesResponse._();
  static $pb.GeneratedMessage $_createMessage() => BackupListFilesResponse._();
  @$core.override
  BackupListFilesResponse createEmptyInstance() => BackupListFilesResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupListFilesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListFilesResponse>(
          BackupListFilesResponse.$_createMessage);
  static BackupListFilesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupFile> get files => $_getList(0);
}

class BackupDownloadFileRequest extends $pb.GeneratedMessage {
  factory BackupDownloadFileRequest({
    $fixnum.Int64? destinationId,
    $core.String? objectKey,
  }) {
    final result = BackupDownloadFileRequest._();
    if (destinationId != null) result.destinationId = destinationId;
    if (objectKey != null) result.objectKey = objectKey;
    return result;
  }

  BackupDownloadFileRequest._();

  factory BackupDownloadFileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDownloadFileRequest()..mergeFromBuffer(data, registry);
  factory BackupDownloadFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDownloadFileRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDownloadFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDownloadFileRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'destinationId')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDownloadFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDownloadFileRequest copyWith(
          void Function(BackupDownloadFileRequest) updates) =>
      super.copyWith((message) => updates(message as BackupDownloadFileRequest))
          as BackupDownloadFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupDownloadFileRequest() / BackupDownloadFileRequest.new instead')
  static BackupDownloadFileRequest create() => BackupDownloadFileRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupDownloadFileRequest._();
  @$core.override
  BackupDownloadFileRequest createEmptyInstance() =>
      BackupDownloadFileRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupDownloadFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupDownloadFileRequest>(
          BackupDownloadFileRequest.$_createMessage);
  static BackupDownloadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get destinationId => $_getI64(0);
  @$pb.TagNumber(1)
  set destinationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDestinationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestinationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjectKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectKey() => $_clearField(2);
}

/// The file is streamed back to the node; the reply only reports its metadata.
class BackupDownloadFileResponse extends $pb.GeneratedMessage {
  factory BackupDownloadFileResponse({
    $fixnum.Int64? sizeBytes,
    $core.String? objectKey,
  }) {
    final result = BackupDownloadFileResponse._();
    if (sizeBytes != null) result.sizeBytes = sizeBytes;
    if (objectKey != null) result.objectKey = objectKey;
    return result;
  }

  BackupDownloadFileResponse._();

  factory BackupDownloadFileResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDownloadFileResponse()..mergeFromBuffer(data, registry);
  factory BackupDownloadFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDownloadFileResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDownloadFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDownloadFileResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'sizeBytes')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDownloadFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDownloadFileResponse copyWith(
          void Function(BackupDownloadFileResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupDownloadFileResponse))
          as BackupDownloadFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupDownloadFileResponse() / BackupDownloadFileResponse.new instead')
  static BackupDownloadFileResponse create() => BackupDownloadFileResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupDownloadFileResponse._();
  @$core.override
  BackupDownloadFileResponse createEmptyInstance() =>
      BackupDownloadFileResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupDownloadFileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupDownloadFileResponse>(
          BackupDownloadFileResponse.$_createMessage);
  static BackupDownloadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get sizeBytes => $_getI64(0);
  @$pb.TagNumber(1)
  set sizeBytes($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSizeBytes() => $_has(0);
  @$pb.TagNumber(1)
  void clearSizeBytes() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjectKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectKey() => $_clearField(2);
}

class BackupPreviewRetentionRequest extends $pb.GeneratedMessage {
  factory BackupPreviewRetentionRequest({
    $fixnum.Int64? destinationId,
    $core.String? prefix,
    $fixnum.Int64? keepLatest,
  }) {
    final result = BackupPreviewRetentionRequest._();
    if (destinationId != null) result.destinationId = destinationId;
    if (prefix != null) result.prefix = prefix;
    if (keepLatest != null) result.keepLatest = keepLatest;
    return result;
  }

  BackupPreviewRetentionRequest._();

  factory BackupPreviewRetentionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupPreviewRetentionRequest()..mergeFromBuffer(data, registry);
  factory BackupPreviewRetentionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupPreviewRetentionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupPreviewRetentionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupPreviewRetentionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'destinationId')
    ..aOS(2, _omitFieldNames ? '' : 'prefix')
    ..aInt64(3, _omitFieldNames ? '' : 'keepLatest')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupPreviewRetentionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupPreviewRetentionRequest copyWith(
          void Function(BackupPreviewRetentionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupPreviewRetentionRequest))
          as BackupPreviewRetentionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupPreviewRetentionRequest() / BackupPreviewRetentionRequest.new instead')
  static BackupPreviewRetentionRequest create() =>
      BackupPreviewRetentionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupPreviewRetentionRequest._();
  @$core.override
  BackupPreviewRetentionRequest createEmptyInstance() =>
      BackupPreviewRetentionRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupPreviewRetentionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupPreviewRetentionRequest>(
          BackupPreviewRetentionRequest.$_createMessage);
  static BackupPreviewRetentionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get destinationId => $_getI64(0);
  @$pb.TagNumber(1)
  set destinationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDestinationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestinationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get prefix => $_getSZ(1);
  @$pb.TagNumber(2)
  set prefix($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPrefix() => $_has(1);
  @$pb.TagNumber(2)
  void clearPrefix() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get keepLatest => $_getI64(2);
  @$pb.TagNumber(3)
  set keepLatest($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasKeepLatest() => $_has(2);
  @$pb.TagNumber(3)
  void clearKeepLatest() => $_clearField(3);
}

class BackupPreviewRetentionResponse extends $pb.GeneratedMessage {
  factory BackupPreviewRetentionResponse({
    BackupRetentionPreview? preview,
  }) {
    final result = BackupPreviewRetentionResponse._();
    if (preview != null) result.preview = preview;
    return result;
  }

  BackupPreviewRetentionResponse._();

  factory BackupPreviewRetentionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupPreviewRetentionResponse()..mergeFromBuffer(data, registry);
  factory BackupPreviewRetentionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupPreviewRetentionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupPreviewRetentionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupPreviewRetentionResponse.$_createMessage)
    ..aOM<BackupRetentionPreview>(1, _omitFieldNames ? '' : 'preview',
        subBuilder: BackupRetentionPreview.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupPreviewRetentionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupPreviewRetentionResponse copyWith(
          void Function(BackupPreviewRetentionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupPreviewRetentionResponse))
          as BackupPreviewRetentionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupPreviewRetentionResponse() / BackupPreviewRetentionResponse.new instead')
  static BackupPreviewRetentionResponse create() =>
      BackupPreviewRetentionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupPreviewRetentionResponse._();
  @$core.override
  BackupPreviewRetentionResponse createEmptyInstance() =>
      BackupPreviewRetentionResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupPreviewRetentionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupPreviewRetentionResponse>(
          BackupPreviewRetentionResponse.$_createMessage);
  static BackupPreviewRetentionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupRetentionPreview get preview => $_getN(0);
  @$pb.TagNumber(1)
  set preview(BackupRetentionPreview value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPreview() => $_has(0);
  @$pb.TagNumber(1)
  void clearPreview() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupRetentionPreview ensurePreview() => $_ensure(0);
}

class BackupVerifyFilesRequest extends $pb.GeneratedMessage {
  factory BackupVerifyFilesRequest({
    $fixnum.Int64? destinationId,
    $core.String? objectKey,
    $core.String? checksumSha256,
  }) {
    final result = BackupVerifyFilesRequest._();
    if (destinationId != null) result.destinationId = destinationId;
    if (objectKey != null) result.objectKey = objectKey;
    if (checksumSha256 != null) result.checksumSha256 = checksumSha256;
    return result;
  }

  BackupVerifyFilesRequest._();

  factory BackupVerifyFilesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVerifyFilesRequest()..mergeFromBuffer(data, registry);
  factory BackupVerifyFilesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVerifyFilesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupVerifyFilesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupVerifyFilesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'destinationId')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..aOS(3, _omitFieldNames ? '' : 'checksumSha256')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVerifyFilesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVerifyFilesRequest copyWith(
          void Function(BackupVerifyFilesRequest) updates) =>
      super.copyWith((message) => updates(message as BackupVerifyFilesRequest))
          as BackupVerifyFilesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupVerifyFilesRequest() / BackupVerifyFilesRequest.new instead')
  static BackupVerifyFilesRequest create() => BackupVerifyFilesRequest._();
  static $pb.GeneratedMessage $_createMessage() => BackupVerifyFilesRequest._();
  @$core.override
  BackupVerifyFilesRequest createEmptyInstance() =>
      BackupVerifyFilesRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupVerifyFilesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupVerifyFilesRequest>(
          BackupVerifyFilesRequest.$_createMessage);
  static BackupVerifyFilesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get destinationId => $_getI64(0);
  @$pb.TagNumber(1)
  set destinationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDestinationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestinationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjectKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectKey() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get checksumSha256 => $_getSZ(2);
  @$pb.TagNumber(3)
  set checksumSha256($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasChecksumSha256() => $_has(2);
  @$pb.TagNumber(3)
  void clearChecksumSha256() => $_clearField(3);
}

class BackupVerifyFilesResponse extends $pb.GeneratedMessage {
  factory BackupVerifyFilesResponse({
    BackupIntegrity? integrity,
  }) {
    final result = BackupVerifyFilesResponse._();
    if (integrity != null) result.integrity = integrity;
    return result;
  }

  BackupVerifyFilesResponse._();

  factory BackupVerifyFilesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVerifyFilesResponse()..mergeFromBuffer(data, registry);
  factory BackupVerifyFilesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupVerifyFilesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupVerifyFilesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupVerifyFilesResponse.$_createMessage)
    ..aOM<BackupIntegrity>(1, _omitFieldNames ? '' : 'integrity',
        subBuilder: BackupIntegrity.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVerifyFilesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupVerifyFilesResponse copyWith(
          void Function(BackupVerifyFilesResponse) updates) =>
      super.copyWith((message) => updates(message as BackupVerifyFilesResponse))
          as BackupVerifyFilesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupVerifyFilesResponse() / BackupVerifyFilesResponse.new instead')
  static BackupVerifyFilesResponse create() => BackupVerifyFilesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupVerifyFilesResponse._();
  @$core.override
  BackupVerifyFilesResponse createEmptyInstance() =>
      BackupVerifyFilesResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupVerifyFilesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupVerifyFilesResponse>(
          BackupVerifyFilesResponse.$_createMessage);
  static BackupVerifyFilesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupIntegrity get integrity => $_getN(0);
  @$pb.TagNumber(1)
  set integrity(BackupIntegrity value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasIntegrity() => $_has(0);
  @$pb.TagNumber(1)
  void clearIntegrity() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupIntegrity ensureIntegrity() => $_ensure(0);
}

class BackupListExecutionsRequest extends $pb.GeneratedMessage {
  factory BackupListExecutionsRequest({
    $fixnum.Int64? limit,
  }) {
    final result = BackupListExecutionsRequest._();
    if (limit != null) result.limit = limit;
    return result;
  }

  BackupListExecutionsRequest._();

  factory BackupListExecutionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListExecutionsRequest()..mergeFromBuffer(data, registry);
  factory BackupListExecutionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListExecutionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListExecutionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListExecutionsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListExecutionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListExecutionsRequest copyWith(
          void Function(BackupListExecutionsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupListExecutionsRequest))
          as BackupListExecutionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListExecutionsRequest() / BackupListExecutionsRequest.new instead')
  static BackupListExecutionsRequest create() =>
      BackupListExecutionsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListExecutionsRequest._();
  @$core.override
  BackupListExecutionsRequest createEmptyInstance() =>
      BackupListExecutionsRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupListExecutionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListExecutionsRequest>(
          BackupListExecutionsRequest.$_createMessage);
  static BackupListExecutionsRequest? _defaultInstance;

  /// Cap on returned rows; the server clamps it to 1..=500 (default 100).
  @$pb.TagNumber(1)
  $fixnum.Int64 get limit => $_getI64(0);
  @$pb.TagNumber(1)
  set limit($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasLimit() => $_has(0);
  @$pb.TagNumber(1)
  void clearLimit() => $_clearField(1);
}

class BackupListExecutionsResponse extends $pb.GeneratedMessage {
  factory BackupListExecutionsResponse({
    $core.Iterable<BackupExecution>? executions,
  }) {
    final result = BackupListExecutionsResponse._();
    if (executions != null) result.executions.addAll(executions);
    return result;
  }

  BackupListExecutionsResponse._();

  factory BackupListExecutionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListExecutionsResponse()..mergeFromBuffer(data, registry);
  factory BackupListExecutionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListExecutionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListExecutionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListExecutionsResponse.$_createMessage)
    ..pPM<BackupExecution>(1, _omitFieldNames ? '' : 'executions',
        subBuilder: BackupExecution.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListExecutionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListExecutionsResponse copyWith(
          void Function(BackupListExecutionsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupListExecutionsResponse))
          as BackupListExecutionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListExecutionsResponse() / BackupListExecutionsResponse.new instead')
  static BackupListExecutionsResponse create() =>
      BackupListExecutionsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListExecutionsResponse._();
  @$core.override
  BackupListExecutionsResponse createEmptyInstance() =>
      BackupListExecutionsResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupListExecutionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListExecutionsResponse>(
          BackupListExecutionsResponse.$_createMessage);
  static BackupListExecutionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupExecution> get executions => $_getList(0);
}

class BackupRetryExecutionRequest extends $pb.GeneratedMessage {
  factory BackupRetryExecutionRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupRetryExecutionRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupRetryExecutionRequest._();

  factory BackupRetryExecutionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRetryExecutionRequest()..mergeFromBuffer(data, registry);
  factory BackupRetryExecutionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRetryExecutionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRetryExecutionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRetryExecutionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRetryExecutionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRetryExecutionRequest copyWith(
          void Function(BackupRetryExecutionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRetryExecutionRequest))
          as BackupRetryExecutionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRetryExecutionRequest() / BackupRetryExecutionRequest.new instead')
  static BackupRetryExecutionRequest create() =>
      BackupRetryExecutionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRetryExecutionRequest._();
  @$core.override
  BackupRetryExecutionRequest createEmptyInstance() =>
      BackupRetryExecutionRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRetryExecutionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRetryExecutionRequest>(
          BackupRetryExecutionRequest.$_createMessage);
  static BackupRetryExecutionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupListDatabaseBackupsRequest extends $pb.GeneratedMessage {
  factory BackupListDatabaseBackupsRequest() =>
      BackupListDatabaseBackupsRequest._();

  BackupListDatabaseBackupsRequest._();

  factory BackupListDatabaseBackupsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListDatabaseBackupsRequest()..mergeFromBuffer(data, registry);
  factory BackupListDatabaseBackupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListDatabaseBackupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListDatabaseBackupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListDatabaseBackupsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListDatabaseBackupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListDatabaseBackupsRequest copyWith(
          void Function(BackupListDatabaseBackupsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupListDatabaseBackupsRequest))
          as BackupListDatabaseBackupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListDatabaseBackupsRequest() / BackupListDatabaseBackupsRequest.new instead')
  static BackupListDatabaseBackupsRequest create() =>
      BackupListDatabaseBackupsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListDatabaseBackupsRequest._();
  @$core.override
  BackupListDatabaseBackupsRequest createEmptyInstance() =>
      BackupListDatabaseBackupsRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupListDatabaseBackupsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListDatabaseBackupsRequest>(
          BackupListDatabaseBackupsRequest.$_createMessage);
  static BackupListDatabaseBackupsRequest? _defaultInstance;
}

class BackupListDatabaseBackupsResponse extends $pb.GeneratedMessage {
  factory BackupListDatabaseBackupsResponse({
    $core.Iterable<BackupDatabase>? backups,
  }) {
    final result = BackupListDatabaseBackupsResponse._();
    if (backups != null) result.backups.addAll(backups);
    return result;
  }

  BackupListDatabaseBackupsResponse._();

  factory BackupListDatabaseBackupsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListDatabaseBackupsResponse()..mergeFromBuffer(data, registry);
  factory BackupListDatabaseBackupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListDatabaseBackupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListDatabaseBackupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListDatabaseBackupsResponse.$_createMessage)
    ..pPM<BackupDatabase>(1, _omitFieldNames ? '' : 'backups',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListDatabaseBackupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListDatabaseBackupsResponse copyWith(
          void Function(BackupListDatabaseBackupsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as BackupListDatabaseBackupsResponse))
          as BackupListDatabaseBackupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListDatabaseBackupsResponse() / BackupListDatabaseBackupsResponse.new instead')
  static BackupListDatabaseBackupsResponse create() =>
      BackupListDatabaseBackupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListDatabaseBackupsResponse._();
  @$core.override
  BackupListDatabaseBackupsResponse createEmptyInstance() =>
      BackupListDatabaseBackupsResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupListDatabaseBackupsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListDatabaseBackupsResponse>(
          BackupListDatabaseBackupsResponse.$_createMessage);
  static BackupListDatabaseBackupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupDatabase> get backups => $_getList(0);
}

class BackupGetDatabaseBackupRequest extends $pb.GeneratedMessage {
  factory BackupGetDatabaseBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupGetDatabaseBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupGetDatabaseBackupRequest._();

  factory BackupGetDatabaseBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetDatabaseBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupGetDatabaseBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetDatabaseBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetDatabaseBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetDatabaseBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetDatabaseBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetDatabaseBackupRequest copyWith(
          void Function(BackupGetDatabaseBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetDatabaseBackupRequest))
          as BackupGetDatabaseBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetDatabaseBackupRequest() / BackupGetDatabaseBackupRequest.new instead')
  static BackupGetDatabaseBackupRequest create() =>
      BackupGetDatabaseBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetDatabaseBackupRequest._();
  @$core.override
  BackupGetDatabaseBackupRequest createEmptyInstance() =>
      BackupGetDatabaseBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupGetDatabaseBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetDatabaseBackupRequest>(
          BackupGetDatabaseBackupRequest.$_createMessage);
  static BackupGetDatabaseBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupGetDatabaseBackupResponse extends $pb.GeneratedMessage {
  factory BackupGetDatabaseBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupGetDatabaseBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupGetDatabaseBackupResponse._();

  factory BackupGetDatabaseBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetDatabaseBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupGetDatabaseBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetDatabaseBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetDatabaseBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetDatabaseBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetDatabaseBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetDatabaseBackupResponse copyWith(
          void Function(BackupGetDatabaseBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetDatabaseBackupResponse))
          as BackupGetDatabaseBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetDatabaseBackupResponse() / BackupGetDatabaseBackupResponse.new instead')
  static BackupGetDatabaseBackupResponse create() =>
      BackupGetDatabaseBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetDatabaseBackupResponse._();
  @$core.override
  BackupGetDatabaseBackupResponse createEmptyInstance() =>
      BackupGetDatabaseBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupGetDatabaseBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetDatabaseBackupResponse>(
          BackupGetDatabaseBackupResponse.$_createMessage);
  static BackupGetDatabaseBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

class BackupCreateDatabaseBackupRequest extends $pb.GeneratedMessage {
  factory BackupCreateDatabaseBackupRequest({
    $core.String? appName,
    $core.String? schedule,
    $core.String? databaseName,
    $core.String? prefix,
    $core.String? serviceName,
    $fixnum.Int64? keepLatestCount,
    $core.String? backupType,
    $core.String? databaseType,
    $core.String? metadata,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? destinationId,
    $fixnum.Int64? organizationId,
  }) {
    final result = BackupCreateDatabaseBackupRequest._();
    if (appName != null) result.appName = appName;
    if (schedule != null) result.schedule = schedule;
    if (databaseName != null) result.databaseName = databaseName;
    if (prefix != null) result.prefix = prefix;
    if (serviceName != null) result.serviceName = serviceName;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (backupType != null) result.backupType = backupType;
    if (databaseType != null) result.databaseType = databaseType;
    if (metadata != null) result.metadata = metadata;
    if (serviceId != null) result.serviceId = serviceId;
    if (destinationId != null) result.destinationId = destinationId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  BackupCreateDatabaseBackupRequest._();

  factory BackupCreateDatabaseBackupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateDatabaseBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupCreateDatabaseBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateDatabaseBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateDatabaseBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateDatabaseBackupRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'appName')
    ..aOS(2, _omitFieldNames ? '' : 'schedule')
    ..aOS(3, _omitFieldNames ? '' : 'databaseName')
    ..aOS(4, _omitFieldNames ? '' : 'prefix')
    ..aOS(5, _omitFieldNames ? '' : 'serviceName')
    ..aInt64(6, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOS(7, _omitFieldNames ? '' : 'backupType')
    ..aOS(8, _omitFieldNames ? '' : 'databaseType')
    ..aOS(9, _omitFieldNames ? '' : 'metadata')
    ..aInt64(10, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(11, _omitFieldNames ? '' : 'destinationId')
    ..aInt64(12, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateDatabaseBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateDatabaseBackupRequest copyWith(
          void Function(BackupCreateDatabaseBackupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as BackupCreateDatabaseBackupRequest))
          as BackupCreateDatabaseBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateDatabaseBackupRequest() / BackupCreateDatabaseBackupRequest.new instead')
  static BackupCreateDatabaseBackupRequest create() =>
      BackupCreateDatabaseBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateDatabaseBackupRequest._();
  @$core.override
  BackupCreateDatabaseBackupRequest createEmptyInstance() =>
      BackupCreateDatabaseBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateDatabaseBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateDatabaseBackupRequest>(
          BackupCreateDatabaseBackupRequest.$_createMessage);
  static BackupCreateDatabaseBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get appName => $_getSZ(0);
  @$pb.TagNumber(1)
  set appName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAppName() => $_has(0);
  @$pb.TagNumber(1)
  void clearAppName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get schedule => $_getSZ(1);
  @$pb.TagNumber(2)
  set schedule($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSchedule() => $_has(1);
  @$pb.TagNumber(2)
  void clearSchedule() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get databaseName => $_getSZ(2);
  @$pb.TagNumber(3)
  set databaseName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDatabaseName() => $_has(2);
  @$pb.TagNumber(3)
  void clearDatabaseName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get prefix => $_getSZ(3);
  @$pb.TagNumber(4)
  set prefix($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrefix() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrefix() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get serviceName => $_getSZ(4);
  @$pb.TagNumber(5)
  set serviceName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServiceName() => $_has(4);
  @$pb.TagNumber(5)
  void clearServiceName() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get keepLatestCount => $_getI64(5);
  @$pb.TagNumber(6)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasKeepLatestCount() => $_has(5);
  @$pb.TagNumber(6)
  void clearKeepLatestCount() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get backupType => $_getSZ(6);
  @$pb.TagNumber(7)
  set backupType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBackupType() => $_has(6);
  @$pb.TagNumber(7)
  void clearBackupType() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get databaseType => $_getSZ(7);
  @$pb.TagNumber(8)
  set databaseType($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasDatabaseType() => $_has(7);
  @$pb.TagNumber(8)
  void clearDatabaseType() => $_clearField(8);

  /// Optional JSON document with extra backup configuration.
  @$pb.TagNumber(9)
  $core.String get metadata => $_getSZ(8);
  @$pb.TagNumber(9)
  set metadata($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasMetadata() => $_has(8);
  @$pb.TagNumber(9)
  void clearMetadata() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get serviceId => $_getI64(9);
  @$pb.TagNumber(10)
  set serviceId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasServiceId() => $_has(9);
  @$pb.TagNumber(10)
  void clearServiceId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get destinationId => $_getI64(10);
  @$pb.TagNumber(11)
  set destinationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDestinationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearDestinationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get organizationId => $_getI64(11);
  @$pb.TagNumber(12)
  set organizationId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasOrganizationId() => $_has(11);
  @$pb.TagNumber(12)
  void clearOrganizationId() => $_clearField(12);
}

class BackupCreateDatabaseBackupResponse extends $pb.GeneratedMessage {
  factory BackupCreateDatabaseBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupCreateDatabaseBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupCreateDatabaseBackupResponse._();

  factory BackupCreateDatabaseBackupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateDatabaseBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupCreateDatabaseBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateDatabaseBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateDatabaseBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateDatabaseBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateDatabaseBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateDatabaseBackupResponse copyWith(
          void Function(BackupCreateDatabaseBackupResponse) updates) =>
      super.copyWith((message) =>
              updates(message as BackupCreateDatabaseBackupResponse))
          as BackupCreateDatabaseBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateDatabaseBackupResponse() / BackupCreateDatabaseBackupResponse.new instead')
  static BackupCreateDatabaseBackupResponse create() =>
      BackupCreateDatabaseBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateDatabaseBackupResponse._();
  @$core.override
  BackupCreateDatabaseBackupResponse createEmptyInstance() =>
      BackupCreateDatabaseBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateDatabaseBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateDatabaseBackupResponse>(
          BackupCreateDatabaseBackupResponse.$_createMessage);
  static BackupCreateDatabaseBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

/// An absent field keeps its stored value.
class BackupUpdateDatabaseBackupRequest extends $pb.GeneratedMessage {
  factory BackupUpdateDatabaseBackupRequest({
    $fixnum.Int64? id,
    $core.String? appName,
    $core.String? schedule,
    $core.String? databaseName,
    $core.String? prefix,
    $core.String? serviceName,
    $fixnum.Int64? keepLatestCount,
    $core.String? backupType,
    $core.String? databaseType,
    $core.String? metadata,
    $fixnum.Int64? destinationId,
    $core.bool? enabled,
  }) {
    final result = BackupUpdateDatabaseBackupRequest._();
    if (id != null) result.id = id;
    if (appName != null) result.appName = appName;
    if (schedule != null) result.schedule = schedule;
    if (databaseName != null) result.databaseName = databaseName;
    if (prefix != null) result.prefix = prefix;
    if (serviceName != null) result.serviceName = serviceName;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (backupType != null) result.backupType = backupType;
    if (databaseType != null) result.databaseType = databaseType;
    if (metadata != null) result.metadata = metadata;
    if (destinationId != null) result.destinationId = destinationId;
    if (enabled != null) result.enabled = enabled;
    return result;
  }

  BackupUpdateDatabaseBackupRequest._();

  factory BackupUpdateDatabaseBackupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateDatabaseBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupUpdateDatabaseBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateDatabaseBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateDatabaseBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateDatabaseBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'appName')
    ..aOS(3, _omitFieldNames ? '' : 'schedule')
    ..aOS(4, _omitFieldNames ? '' : 'databaseName')
    ..aOS(5, _omitFieldNames ? '' : 'prefix')
    ..aOS(6, _omitFieldNames ? '' : 'serviceName')
    ..aInt64(7, _omitFieldNames ? '' : 'keepLatestCount')
    ..aOS(8, _omitFieldNames ? '' : 'backupType')
    ..aOS(9, _omitFieldNames ? '' : 'databaseType')
    ..aOS(10, _omitFieldNames ? '' : 'metadata')
    ..aInt64(11, _omitFieldNames ? '' : 'destinationId')
    ..aOB(12, _omitFieldNames ? '' : 'enabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateDatabaseBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateDatabaseBackupRequest copyWith(
          void Function(BackupUpdateDatabaseBackupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as BackupUpdateDatabaseBackupRequest))
          as BackupUpdateDatabaseBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateDatabaseBackupRequest() / BackupUpdateDatabaseBackupRequest.new instead')
  static BackupUpdateDatabaseBackupRequest create() =>
      BackupUpdateDatabaseBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateDatabaseBackupRequest._();
  @$core.override
  BackupUpdateDatabaseBackupRequest createEmptyInstance() =>
      BackupUpdateDatabaseBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateDatabaseBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateDatabaseBackupRequest>(
          BackupUpdateDatabaseBackupRequest.$_createMessage);
  static BackupUpdateDatabaseBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get appName => $_getSZ(1);
  @$pb.TagNumber(2)
  set appName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAppName() => $_has(1);
  @$pb.TagNumber(2)
  void clearAppName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get schedule => $_getSZ(2);
  @$pb.TagNumber(3)
  set schedule($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSchedule() => $_has(2);
  @$pb.TagNumber(3)
  void clearSchedule() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get databaseName => $_getSZ(3);
  @$pb.TagNumber(4)
  set databaseName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDatabaseName() => $_has(3);
  @$pb.TagNumber(4)
  void clearDatabaseName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get prefix => $_getSZ(4);
  @$pb.TagNumber(5)
  set prefix($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPrefix() => $_has(4);
  @$pb.TagNumber(5)
  void clearPrefix() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serviceName => $_getSZ(5);
  @$pb.TagNumber(6)
  set serviceName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServiceName() => $_has(5);
  @$pb.TagNumber(6)
  void clearServiceName() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get keepLatestCount => $_getI64(6);
  @$pb.TagNumber(7)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasKeepLatestCount() => $_has(6);
  @$pb.TagNumber(7)
  void clearKeepLatestCount() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get backupType => $_getSZ(7);
  @$pb.TagNumber(8)
  set backupType($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBackupType() => $_has(7);
  @$pb.TagNumber(8)
  void clearBackupType() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get databaseType => $_getSZ(8);
  @$pb.TagNumber(9)
  set databaseType($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasDatabaseType() => $_has(8);
  @$pb.TagNumber(9)
  void clearDatabaseType() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get metadata => $_getSZ(9);
  @$pb.TagNumber(10)
  set metadata($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasMetadata() => $_has(9);
  @$pb.TagNumber(10)
  void clearMetadata() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get destinationId => $_getI64(10);
  @$pb.TagNumber(11)
  set destinationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDestinationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearDestinationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get enabled => $_getBF(11);
  @$pb.TagNumber(12)
  set enabled($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnabled() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnabled() => $_clearField(12);
}

class BackupUpdateDatabaseBackupResponse extends $pb.GeneratedMessage {
  factory BackupUpdateDatabaseBackupResponse({
    BackupDatabase? backup,
  }) {
    final result = BackupUpdateDatabaseBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupUpdateDatabaseBackupResponse._();

  factory BackupUpdateDatabaseBackupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateDatabaseBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupUpdateDatabaseBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateDatabaseBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateDatabaseBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateDatabaseBackupResponse.$_createMessage)
    ..aOM<BackupDatabase>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupDatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateDatabaseBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateDatabaseBackupResponse copyWith(
          void Function(BackupUpdateDatabaseBackupResponse) updates) =>
      super.copyWith((message) =>
              updates(message as BackupUpdateDatabaseBackupResponse))
          as BackupUpdateDatabaseBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateDatabaseBackupResponse() / BackupUpdateDatabaseBackupResponse.new instead')
  static BackupUpdateDatabaseBackupResponse create() =>
      BackupUpdateDatabaseBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateDatabaseBackupResponse._();
  @$core.override
  BackupUpdateDatabaseBackupResponse createEmptyInstance() =>
      BackupUpdateDatabaseBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateDatabaseBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateDatabaseBackupResponse>(
          BackupUpdateDatabaseBackupResponse.$_createMessage);
  static BackupUpdateDatabaseBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupDatabase get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupDatabase value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupDatabase ensureBackup() => $_ensure(0);
}

class BackupDeleteDatabaseBackupRequest extends $pb.GeneratedMessage {
  factory BackupDeleteDatabaseBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupDeleteDatabaseBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupDeleteDatabaseBackupRequest._();

  factory BackupDeleteDatabaseBackupRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteDatabaseBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupDeleteDatabaseBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteDatabaseBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDeleteDatabaseBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDeleteDatabaseBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteDatabaseBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteDatabaseBackupRequest copyWith(
          void Function(BackupDeleteDatabaseBackupRequest) updates) =>
      super.copyWith((message) =>
              updates(message as BackupDeleteDatabaseBackupRequest))
          as BackupDeleteDatabaseBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupDeleteDatabaseBackupRequest() / BackupDeleteDatabaseBackupRequest.new instead')
  static BackupDeleteDatabaseBackupRequest create() =>
      BackupDeleteDatabaseBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupDeleteDatabaseBackupRequest._();
  @$core.override
  BackupDeleteDatabaseBackupRequest createEmptyInstance() =>
      BackupDeleteDatabaseBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupDeleteDatabaseBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupDeleteDatabaseBackupRequest>(
          BackupDeleteDatabaseBackupRequest.$_createMessage);
  static BackupDeleteDatabaseBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupRunDatabaseBackupRequest extends $pb.GeneratedMessage {
  factory BackupRunDatabaseBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupRunDatabaseBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupRunDatabaseBackupRequest._();

  factory BackupRunDatabaseBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunDatabaseBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRunDatabaseBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunDatabaseBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunDatabaseBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunDatabaseBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunDatabaseBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunDatabaseBackupRequest copyWith(
          void Function(BackupRunDatabaseBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunDatabaseBackupRequest))
          as BackupRunDatabaseBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunDatabaseBackupRequest() / BackupRunDatabaseBackupRequest.new instead')
  static BackupRunDatabaseBackupRequest create() =>
      BackupRunDatabaseBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunDatabaseBackupRequest._();
  @$core.override
  BackupRunDatabaseBackupRequest createEmptyInstance() =>
      BackupRunDatabaseBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRunDatabaseBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunDatabaseBackupRequest>(
          BackupRunDatabaseBackupRequest.$_createMessage);
  static BackupRunDatabaseBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupListVolumeBackupsRequest extends $pb.GeneratedMessage {
  factory BackupListVolumeBackupsRequest() =>
      BackupListVolumeBackupsRequest._();

  BackupListVolumeBackupsRequest._();

  factory BackupListVolumeBackupsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListVolumeBackupsRequest()..mergeFromBuffer(data, registry);
  factory BackupListVolumeBackupsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListVolumeBackupsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListVolumeBackupsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListVolumeBackupsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListVolumeBackupsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListVolumeBackupsRequest copyWith(
          void Function(BackupListVolumeBackupsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupListVolumeBackupsRequest))
          as BackupListVolumeBackupsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListVolumeBackupsRequest() / BackupListVolumeBackupsRequest.new instead')
  static BackupListVolumeBackupsRequest create() =>
      BackupListVolumeBackupsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListVolumeBackupsRequest._();
  @$core.override
  BackupListVolumeBackupsRequest createEmptyInstance() =>
      BackupListVolumeBackupsRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupListVolumeBackupsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListVolumeBackupsRequest>(
          BackupListVolumeBackupsRequest.$_createMessage);
  static BackupListVolumeBackupsRequest? _defaultInstance;
}

class BackupListVolumeBackupsResponse extends $pb.GeneratedMessage {
  factory BackupListVolumeBackupsResponse({
    $core.Iterable<BackupVolume>? backups,
  }) {
    final result = BackupListVolumeBackupsResponse._();
    if (backups != null) result.backups.addAll(backups);
    return result;
  }

  BackupListVolumeBackupsResponse._();

  factory BackupListVolumeBackupsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListVolumeBackupsResponse()..mergeFromBuffer(data, registry);
  factory BackupListVolumeBackupsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupListVolumeBackupsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupListVolumeBackupsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupListVolumeBackupsResponse.$_createMessage)
    ..pPM<BackupVolume>(1, _omitFieldNames ? '' : 'backups',
        subBuilder: BackupVolume.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListVolumeBackupsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupListVolumeBackupsResponse copyWith(
          void Function(BackupListVolumeBackupsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupListVolumeBackupsResponse))
          as BackupListVolumeBackupsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupListVolumeBackupsResponse() / BackupListVolumeBackupsResponse.new instead')
  static BackupListVolumeBackupsResponse create() =>
      BackupListVolumeBackupsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupListVolumeBackupsResponse._();
  @$core.override
  BackupListVolumeBackupsResponse createEmptyInstance() =>
      BackupListVolumeBackupsResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupListVolumeBackupsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupListVolumeBackupsResponse>(
          BackupListVolumeBackupsResponse.$_createMessage);
  static BackupListVolumeBackupsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BackupVolume> get backups => $_getList(0);
}

class BackupGetVolumeBackupRequest extends $pb.GeneratedMessage {
  factory BackupGetVolumeBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupGetVolumeBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupGetVolumeBackupRequest._();

  factory BackupGetVolumeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetVolumeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupGetVolumeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetVolumeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetVolumeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetVolumeBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetVolumeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetVolumeBackupRequest copyWith(
          void Function(BackupGetVolumeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetVolumeBackupRequest))
          as BackupGetVolumeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetVolumeBackupRequest() / BackupGetVolumeBackupRequest.new instead')
  static BackupGetVolumeBackupRequest create() =>
      BackupGetVolumeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetVolumeBackupRequest._();
  @$core.override
  BackupGetVolumeBackupRequest createEmptyInstance() =>
      BackupGetVolumeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupGetVolumeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetVolumeBackupRequest>(
          BackupGetVolumeBackupRequest.$_createMessage);
  static BackupGetVolumeBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupGetVolumeBackupResponse extends $pb.GeneratedMessage {
  factory BackupGetVolumeBackupResponse({
    BackupVolume? backup,
  }) {
    final result = BackupGetVolumeBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupGetVolumeBackupResponse._();

  factory BackupGetVolumeBackupResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetVolumeBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupGetVolumeBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupGetVolumeBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupGetVolumeBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupGetVolumeBackupResponse.$_createMessage)
    ..aOM<BackupVolume>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupVolume.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetVolumeBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupGetVolumeBackupResponse copyWith(
          void Function(BackupGetVolumeBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupGetVolumeBackupResponse))
          as BackupGetVolumeBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupGetVolumeBackupResponse() / BackupGetVolumeBackupResponse.new instead')
  static BackupGetVolumeBackupResponse create() =>
      BackupGetVolumeBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupGetVolumeBackupResponse._();
  @$core.override
  BackupGetVolumeBackupResponse createEmptyInstance() =>
      BackupGetVolumeBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupGetVolumeBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupGetVolumeBackupResponse>(
          BackupGetVolumeBackupResponse.$_createMessage);
  static BackupGetVolumeBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupVolume get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupVolume value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupVolume ensureBackup() => $_ensure(0);
}

/// At least one of `service_id` and the typed linkage ids must resolve to a service; the server folds them into a single services row.
class BackupCreateVolumeBackupRequest extends $pb.GeneratedMessage {
  factory BackupCreateVolumeBackupRequest({
    $core.String? name,
    $core.String? volumeName,
    $core.String? prefix,
    $core.String? serviceType,
    $core.String? appName,
    $core.String? serviceName,
    $core.bool? turnOff,
    $core.String? cronExpression,
    $fixnum.Int64? keepLatestCount,
    $fixnum.Int64? destinationId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? serviceId,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $fixnum.Int64? databaseId,
    $fixnum.Int64? postgresId,
    $fixnum.Int64? mysqlId,
    $fixnum.Int64? mariadbId,
    $fixnum.Int64? mongoId,
    $fixnum.Int64? redisId,
    $fixnum.Int64? libsqlId,
  }) {
    final result = BackupCreateVolumeBackupRequest._();
    if (name != null) result.name = name;
    if (volumeName != null) result.volumeName = volumeName;
    if (prefix != null) result.prefix = prefix;
    if (serviceType != null) result.serviceType = serviceType;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (turnOff != null) result.turnOff = turnOff;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (destinationId != null) result.destinationId = destinationId;
    if (organizationId != null) result.organizationId = organizationId;
    if (serviceId != null) result.serviceId = serviceId;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (databaseId != null) result.databaseId = databaseId;
    if (postgresId != null) result.postgresId = postgresId;
    if (mysqlId != null) result.mysqlId = mysqlId;
    if (mariadbId != null) result.mariadbId = mariadbId;
    if (mongoId != null) result.mongoId = mongoId;
    if (redisId != null) result.redisId = redisId;
    if (libsqlId != null) result.libsqlId = libsqlId;
    return result;
  }

  BackupCreateVolumeBackupRequest._();

  factory BackupCreateVolumeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateVolumeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupCreateVolumeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateVolumeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateVolumeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateVolumeBackupRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'volumeName')
    ..aOS(3, _omitFieldNames ? '' : 'prefix')
    ..aOS(4, _omitFieldNames ? '' : 'serviceType')
    ..aOS(5, _omitFieldNames ? '' : 'appName')
    ..aOS(6, _omitFieldNames ? '' : 'serviceName')
    ..aOB(7, _omitFieldNames ? '' : 'turnOff')
    ..aOS(8, _omitFieldNames ? '' : 'cronExpression')
    ..aInt64(9, _omitFieldNames ? '' : 'keepLatestCount')
    ..aInt64(10, _omitFieldNames ? '' : 'destinationId')
    ..aInt64(11, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(12, _omitFieldNames ? '' : 'serviceId')
    ..aInt64(13, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(14, _omitFieldNames ? '' : 'composeId')
    ..aInt64(15, _omitFieldNames ? '' : 'databaseId')
    ..aInt64(16, _omitFieldNames ? '' : 'postgresId')
    ..aInt64(17, _omitFieldNames ? '' : 'mysqlId')
    ..aInt64(18, _omitFieldNames ? '' : 'mariadbId')
    ..aInt64(19, _omitFieldNames ? '' : 'mongoId')
    ..aInt64(20, _omitFieldNames ? '' : 'redisId')
    ..aInt64(21, _omitFieldNames ? '' : 'libsqlId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateVolumeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateVolumeBackupRequest copyWith(
          void Function(BackupCreateVolumeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupCreateVolumeBackupRequest))
          as BackupCreateVolumeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateVolumeBackupRequest() / BackupCreateVolumeBackupRequest.new instead')
  static BackupCreateVolumeBackupRequest create() =>
      BackupCreateVolumeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateVolumeBackupRequest._();
  @$core.override
  BackupCreateVolumeBackupRequest createEmptyInstance() =>
      BackupCreateVolumeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateVolumeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateVolumeBackupRequest>(
          BackupCreateVolumeBackupRequest.$_createMessage);
  static BackupCreateVolumeBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get volumeName => $_getSZ(1);
  @$pb.TagNumber(2)
  set volumeName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVolumeName() => $_has(1);
  @$pb.TagNumber(2)
  void clearVolumeName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get prefix => $_getSZ(2);
  @$pb.TagNumber(3)
  set prefix($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPrefix() => $_has(2);
  @$pb.TagNumber(3)
  void clearPrefix() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get serviceType => $_getSZ(3);
  @$pb.TagNumber(4)
  set serviceType($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasServiceType() => $_has(3);
  @$pb.TagNumber(4)
  void clearServiceType() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get appName => $_getSZ(4);
  @$pb.TagNumber(5)
  set appName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAppName() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get serviceName => $_getSZ(5);
  @$pb.TagNumber(6)
  set serviceName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServiceName() => $_has(5);
  @$pb.TagNumber(6)
  void clearServiceName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get turnOff => $_getBF(6);
  @$pb.TagNumber(7)
  set turnOff($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasTurnOff() => $_has(6);
  @$pb.TagNumber(7)
  void clearTurnOff() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get cronExpression => $_getSZ(7);
  @$pb.TagNumber(8)
  set cronExpression($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCronExpression() => $_has(7);
  @$pb.TagNumber(8)
  void clearCronExpression() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get keepLatestCount => $_getI64(8);
  @$pb.TagNumber(9)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasKeepLatestCount() => $_has(8);
  @$pb.TagNumber(9)
  void clearKeepLatestCount() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get destinationId => $_getI64(9);
  @$pb.TagNumber(10)
  set destinationId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasDestinationId() => $_has(9);
  @$pb.TagNumber(10)
  void clearDestinationId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get organizationId => $_getI64(10);
  @$pb.TagNumber(11)
  set organizationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasOrganizationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearOrganizationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get serviceId => $_getI64(11);
  @$pb.TagNumber(12)
  set serviceId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasServiceId() => $_has(11);
  @$pb.TagNumber(12)
  void clearServiceId() => $_clearField(12);

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

  @$pb.TagNumber(15)
  $fixnum.Int64 get databaseId => $_getI64(14);
  @$pb.TagNumber(15)
  set databaseId($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasDatabaseId() => $_has(14);
  @$pb.TagNumber(15)
  void clearDatabaseId() => $_clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get postgresId => $_getI64(15);
  @$pb.TagNumber(16)
  set postgresId($fixnum.Int64 value) => $_setInt64(15, value);
  @$pb.TagNumber(16)
  $core.bool hasPostgresId() => $_has(15);
  @$pb.TagNumber(16)
  void clearPostgresId() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get mysqlId => $_getI64(16);
  @$pb.TagNumber(17)
  set mysqlId($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasMysqlId() => $_has(16);
  @$pb.TagNumber(17)
  void clearMysqlId() => $_clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get mariadbId => $_getI64(17);
  @$pb.TagNumber(18)
  set mariadbId($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasMariadbId() => $_has(17);
  @$pb.TagNumber(18)
  void clearMariadbId() => $_clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get mongoId => $_getI64(18);
  @$pb.TagNumber(19)
  set mongoId($fixnum.Int64 value) => $_setInt64(18, value);
  @$pb.TagNumber(19)
  $core.bool hasMongoId() => $_has(18);
  @$pb.TagNumber(19)
  void clearMongoId() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get redisId => $_getI64(19);
  @$pb.TagNumber(20)
  set redisId($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasRedisId() => $_has(19);
  @$pb.TagNumber(20)
  void clearRedisId() => $_clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get libsqlId => $_getI64(20);
  @$pb.TagNumber(21)
  set libsqlId($fixnum.Int64 value) => $_setInt64(20, value);
  @$pb.TagNumber(21)
  $core.bool hasLibsqlId() => $_has(20);
  @$pb.TagNumber(21)
  void clearLibsqlId() => $_clearField(21);
}

class BackupCreateVolumeBackupResponse extends $pb.GeneratedMessage {
  factory BackupCreateVolumeBackupResponse({
    BackupVolume? backup,
  }) {
    final result = BackupCreateVolumeBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupCreateVolumeBackupResponse._();

  factory BackupCreateVolumeBackupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateVolumeBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupCreateVolumeBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupCreateVolumeBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupCreateVolumeBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupCreateVolumeBackupResponse.$_createMessage)
    ..aOM<BackupVolume>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupVolume.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateVolumeBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupCreateVolumeBackupResponse copyWith(
          void Function(BackupCreateVolumeBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupCreateVolumeBackupResponse))
          as BackupCreateVolumeBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupCreateVolumeBackupResponse() / BackupCreateVolumeBackupResponse.new instead')
  static BackupCreateVolumeBackupResponse create() =>
      BackupCreateVolumeBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupCreateVolumeBackupResponse._();
  @$core.override
  BackupCreateVolumeBackupResponse createEmptyInstance() =>
      BackupCreateVolumeBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupCreateVolumeBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupCreateVolumeBackupResponse>(
          BackupCreateVolumeBackupResponse.$_createMessage);
  static BackupCreateVolumeBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupVolume get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupVolume value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupVolume ensureBackup() => $_ensure(0);
}

/// An absent field keeps its stored value.
class BackupUpdateVolumeBackupRequest extends $pb.GeneratedMessage {
  factory BackupUpdateVolumeBackupRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? volumeName,
    $core.String? prefix,
    $core.String? serviceType,
    $core.String? appName,
    $core.String? serviceName,
    $core.bool? turnOff,
    $core.String? cronExpression,
    $fixnum.Int64? keepLatestCount,
    $fixnum.Int64? destinationId,
    $core.bool? enabled,
  }) {
    final result = BackupUpdateVolumeBackupRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (volumeName != null) result.volumeName = volumeName;
    if (prefix != null) result.prefix = prefix;
    if (serviceType != null) result.serviceType = serviceType;
    if (appName != null) result.appName = appName;
    if (serviceName != null) result.serviceName = serviceName;
    if (turnOff != null) result.turnOff = turnOff;
    if (cronExpression != null) result.cronExpression = cronExpression;
    if (keepLatestCount != null) result.keepLatestCount = keepLatestCount;
    if (destinationId != null) result.destinationId = destinationId;
    if (enabled != null) result.enabled = enabled;
    return result;
  }

  BackupUpdateVolumeBackupRequest._();

  factory BackupUpdateVolumeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateVolumeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupUpdateVolumeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateVolumeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateVolumeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateVolumeBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'volumeName')
    ..aOS(4, _omitFieldNames ? '' : 'prefix')
    ..aOS(5, _omitFieldNames ? '' : 'serviceType')
    ..aOS(6, _omitFieldNames ? '' : 'appName')
    ..aOS(7, _omitFieldNames ? '' : 'serviceName')
    ..aOB(8, _omitFieldNames ? '' : 'turnOff')
    ..aOS(9, _omitFieldNames ? '' : 'cronExpression')
    ..aInt64(10, _omitFieldNames ? '' : 'keepLatestCount')
    ..aInt64(11, _omitFieldNames ? '' : 'destinationId')
    ..aOB(12, _omitFieldNames ? '' : 'enabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateVolumeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateVolumeBackupRequest copyWith(
          void Function(BackupUpdateVolumeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupUpdateVolumeBackupRequest))
          as BackupUpdateVolumeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateVolumeBackupRequest() / BackupUpdateVolumeBackupRequest.new instead')
  static BackupUpdateVolumeBackupRequest create() =>
      BackupUpdateVolumeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateVolumeBackupRequest._();
  @$core.override
  BackupUpdateVolumeBackupRequest createEmptyInstance() =>
      BackupUpdateVolumeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateVolumeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateVolumeBackupRequest>(
          BackupUpdateVolumeBackupRequest.$_createMessage);
  static BackupUpdateVolumeBackupRequest? _defaultInstance;

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
  $core.String get volumeName => $_getSZ(2);
  @$pb.TagNumber(3)
  set volumeName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVolumeName() => $_has(2);
  @$pb.TagNumber(3)
  void clearVolumeName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get prefix => $_getSZ(3);
  @$pb.TagNumber(4)
  set prefix($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrefix() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrefix() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get serviceType => $_getSZ(4);
  @$pb.TagNumber(5)
  set serviceType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServiceType() => $_has(4);
  @$pb.TagNumber(5)
  void clearServiceType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get appName => $_getSZ(5);
  @$pb.TagNumber(6)
  set appName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAppName() => $_has(5);
  @$pb.TagNumber(6)
  void clearAppName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serviceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set serviceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceName() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get turnOff => $_getBF(7);
  @$pb.TagNumber(8)
  set turnOff($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasTurnOff() => $_has(7);
  @$pb.TagNumber(8)
  void clearTurnOff() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get cronExpression => $_getSZ(8);
  @$pb.TagNumber(9)
  set cronExpression($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCronExpression() => $_has(8);
  @$pb.TagNumber(9)
  void clearCronExpression() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get keepLatestCount => $_getI64(9);
  @$pb.TagNumber(10)
  set keepLatestCount($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasKeepLatestCount() => $_has(9);
  @$pb.TagNumber(10)
  void clearKeepLatestCount() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get destinationId => $_getI64(10);
  @$pb.TagNumber(11)
  set destinationId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDestinationId() => $_has(10);
  @$pb.TagNumber(11)
  void clearDestinationId() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get enabled => $_getBF(11);
  @$pb.TagNumber(12)
  set enabled($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEnabled() => $_has(11);
  @$pb.TagNumber(12)
  void clearEnabled() => $_clearField(12);
}

class BackupUpdateVolumeBackupResponse extends $pb.GeneratedMessage {
  factory BackupUpdateVolumeBackupResponse({
    BackupVolume? backup,
  }) {
    final result = BackupUpdateVolumeBackupResponse._();
    if (backup != null) result.backup = backup;
    return result;
  }

  BackupUpdateVolumeBackupResponse._();

  factory BackupUpdateVolumeBackupResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateVolumeBackupResponse()..mergeFromBuffer(data, registry);
  factory BackupUpdateVolumeBackupResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupUpdateVolumeBackupResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupUpdateVolumeBackupResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupUpdateVolumeBackupResponse.$_createMessage)
    ..aOM<BackupVolume>(1, _omitFieldNames ? '' : 'backup',
        subBuilder: BackupVolume.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateVolumeBackupResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupUpdateVolumeBackupResponse copyWith(
          void Function(BackupUpdateVolumeBackupResponse) updates) =>
      super.copyWith(
              (message) => updates(message as BackupUpdateVolumeBackupResponse))
          as BackupUpdateVolumeBackupResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupUpdateVolumeBackupResponse() / BackupUpdateVolumeBackupResponse.new instead')
  static BackupUpdateVolumeBackupResponse create() =>
      BackupUpdateVolumeBackupResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupUpdateVolumeBackupResponse._();
  @$core.override
  BackupUpdateVolumeBackupResponse createEmptyInstance() =>
      BackupUpdateVolumeBackupResponse._();
  @$core.pragma('dart2js:noInline')
  static BackupUpdateVolumeBackupResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupUpdateVolumeBackupResponse>(
          BackupUpdateVolumeBackupResponse.$_createMessage);
  static BackupUpdateVolumeBackupResponse? _defaultInstance;

  @$pb.TagNumber(1)
  BackupVolume get backup => $_getN(0);
  @$pb.TagNumber(1)
  set backup(BackupVolume value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasBackup() => $_has(0);
  @$pb.TagNumber(1)
  void clearBackup() => $_clearField(1);
  @$pb.TagNumber(1)
  BackupVolume ensureBackup() => $_ensure(0);
}

class BackupDeleteVolumeBackupRequest extends $pb.GeneratedMessage {
  factory BackupDeleteVolumeBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupDeleteVolumeBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupDeleteVolumeBackupRequest._();

  factory BackupDeleteVolumeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteVolumeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupDeleteVolumeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupDeleteVolumeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupDeleteVolumeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupDeleteVolumeBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteVolumeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupDeleteVolumeBackupRequest copyWith(
          void Function(BackupDeleteVolumeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupDeleteVolumeBackupRequest))
          as BackupDeleteVolumeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupDeleteVolumeBackupRequest() / BackupDeleteVolumeBackupRequest.new instead')
  static BackupDeleteVolumeBackupRequest create() =>
      BackupDeleteVolumeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupDeleteVolumeBackupRequest._();
  @$core.override
  BackupDeleteVolumeBackupRequest createEmptyInstance() =>
      BackupDeleteVolumeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupDeleteVolumeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupDeleteVolumeBackupRequest>(
          BackupDeleteVolumeBackupRequest.$_createMessage);
  static BackupDeleteVolumeBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupRunVolumeBackupRequest extends $pb.GeneratedMessage {
  factory BackupRunVolumeBackupRequest({
    $fixnum.Int64? id,
  }) {
    final result = BackupRunVolumeBackupRequest._();
    if (id != null) result.id = id;
    return result;
  }

  BackupRunVolumeBackupRequest._();

  factory BackupRunVolumeBackupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunVolumeBackupRequest()..mergeFromBuffer(data, registry);
  factory BackupRunVolumeBackupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRunVolumeBackupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRunVolumeBackupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRunVolumeBackupRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunVolumeBackupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRunVolumeBackupRequest copyWith(
          void Function(BackupRunVolumeBackupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRunVolumeBackupRequest))
          as BackupRunVolumeBackupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRunVolumeBackupRequest() / BackupRunVolumeBackupRequest.new instead')
  static BackupRunVolumeBackupRequest create() =>
      BackupRunVolumeBackupRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRunVolumeBackupRequest._();
  @$core.override
  BackupRunVolumeBackupRequest createEmptyInstance() =>
      BackupRunVolumeBackupRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRunVolumeBackupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRunVolumeBackupRequest>(
          BackupRunVolumeBackupRequest.$_createMessage);
  static BackupRunVolumeBackupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class BackupRestoreDatabaseRequest extends $pb.GeneratedMessage {
  factory BackupRestoreDatabaseRequest({
    $fixnum.Int64? id,
    $core.String? backupFile,
  }) {
    final result = BackupRestoreDatabaseRequest._();
    if (id != null) result.id = id;
    if (backupFile != null) result.backupFile = backupFile;
    return result;
  }

  BackupRestoreDatabaseRequest._();

  factory BackupRestoreDatabaseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreDatabaseRequest()..mergeFromBuffer(data, registry);
  factory BackupRestoreDatabaseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreDatabaseRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRestoreDatabaseRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRestoreDatabaseRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'backupFile')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreDatabaseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreDatabaseRequest copyWith(
          void Function(BackupRestoreDatabaseRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRestoreDatabaseRequest))
          as BackupRestoreDatabaseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRestoreDatabaseRequest() / BackupRestoreDatabaseRequest.new instead')
  static BackupRestoreDatabaseRequest create() =>
      BackupRestoreDatabaseRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRestoreDatabaseRequest._();
  @$core.override
  BackupRestoreDatabaseRequest createEmptyInstance() =>
      BackupRestoreDatabaseRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRestoreDatabaseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRestoreDatabaseRequest>(
          BackupRestoreDatabaseRequest.$_createMessage);
  static BackupRestoreDatabaseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get backupFile => $_getSZ(1);
  @$pb.TagNumber(2)
  set backupFile($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBackupFile() => $_has(1);
  @$pb.TagNumber(2)
  void clearBackupFile() => $_clearField(2);
}

class BackupRestoreVolumeRequest extends $pb.GeneratedMessage {
  factory BackupRestoreVolumeRequest({
    $fixnum.Int64? id,
    $core.String? backupFile,
  }) {
    final result = BackupRestoreVolumeRequest._();
    if (id != null) result.id = id;
    if (backupFile != null) result.backupFile = backupFile;
    return result;
  }

  BackupRestoreVolumeRequest._();

  factory BackupRestoreVolumeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreVolumeRequest()..mergeFromBuffer(data, registry);
  factory BackupRestoreVolumeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BackupRestoreVolumeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BackupRestoreVolumeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: BackupRestoreVolumeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'backupFile')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreVolumeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BackupRestoreVolumeRequest copyWith(
          void Function(BackupRestoreVolumeRequest) updates) =>
      super.copyWith(
              (message) => updates(message as BackupRestoreVolumeRequest))
          as BackupRestoreVolumeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use BackupRestoreVolumeRequest() / BackupRestoreVolumeRequest.new instead')
  static BackupRestoreVolumeRequest create() => BackupRestoreVolumeRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      BackupRestoreVolumeRequest._();
  @$core.override
  BackupRestoreVolumeRequest createEmptyInstance() =>
      BackupRestoreVolumeRequest._();
  @$core.pragma('dart2js:noInline')
  static BackupRestoreVolumeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupRestoreVolumeRequest>(
          BackupRestoreVolumeRequest.$_createMessage);
  static BackupRestoreVolumeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get backupFile => $_getSZ(1);
  @$pb.TagNumber(2)
  set backupFile($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBackupFile() => $_has(1);
  @$pb.TagNumber(2)
  void clearBackupFile() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

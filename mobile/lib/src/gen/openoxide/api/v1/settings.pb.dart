// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/settings.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'settings.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'settings.pbenum.dart';

class Settings extends $pb.GeneratedMessage {
  factory Settings({
    $fixnum.Int64? id,
    $core.String? serverIp,
    SettingsCertificateType? certificateType,
    $core.String? customCertResolver,
    $core.bool? https,
    $core.String? host,
    $core.String? letsEncryptEmail,
    $core.bool? enableDockerCleanup,
    $core.String? logCleanupCron,
    $core.String? metricsConfig,
    $core.String? panelBackupCron,
    $fixnum.Int64? logRetentionDays,
    $core.bool? panelBackupEnabled,
    $core.bool? logCleanupEnabled,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Settings._();
    if (id != null) result.id = id;
    if (serverIp != null) result.serverIp = serverIp;
    if (certificateType != null) result.certificateType = certificateType;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (https != null) result.https = https;
    if (host != null) result.host = host;
    if (letsEncryptEmail != null) result.letsEncryptEmail = letsEncryptEmail;
    if (enableDockerCleanup != null)
      result.enableDockerCleanup = enableDockerCleanup;
    if (logCleanupCron != null) result.logCleanupCron = logCleanupCron;
    if (metricsConfig != null) result.metricsConfig = metricsConfig;
    if (panelBackupCron != null) result.panelBackupCron = panelBackupCron;
    if (logRetentionDays != null) result.logRetentionDays = logRetentionDays;
    if (panelBackupEnabled != null)
      result.panelBackupEnabled = panelBackupEnabled;
    if (logCleanupEnabled != null) result.logCleanupEnabled = logCleanupEnabled;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Settings._();

  factory Settings.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Settings()..mergeFromBuffer(data, registry);
  factory Settings.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Settings()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Settings',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Settings.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'serverIp')
    ..aE<SettingsCertificateType>(3, _omitFieldNames ? '' : 'certificateType',
        enumValues: SettingsCertificateType.values)
    ..aOS(4, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(5, _omitFieldNames ? '' : 'https')
    ..aOS(6, _omitFieldNames ? '' : 'host')
    ..aOS(7, _omitFieldNames ? '' : 'letsEncryptEmail')
    ..aOB(8, _omitFieldNames ? '' : 'enableDockerCleanup')
    ..aOS(9, _omitFieldNames ? '' : 'logCleanupCron')
    ..aOS(10, _omitFieldNames ? '' : 'metricsConfig')
    ..aOS(11, _omitFieldNames ? '' : 'panelBackupCron')
    ..aInt64(12, _omitFieldNames ? '' : 'logRetentionDays')
    ..aOB(13, _omitFieldNames ? '' : 'panelBackupEnabled')
    ..aOB(14, _omitFieldNames ? '' : 'logCleanupEnabled')
    ..aInt64(15, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Settings clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Settings copyWith(void Function(Settings) updates) =>
      super.copyWith((message) => updates(message as Settings)) as Settings;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Settings() / Settings.new instead')
  static Settings create() => Settings._();
  static $pb.GeneratedMessage $_createMessage() => Settings._();
  @$core.override
  Settings createEmptyInstance() => Settings._();
  @$core.pragma('dart2js:noInline')
  static Settings getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Settings>(Settings.$_createMessage);
  static Settings? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get serverIp => $_getSZ(1);
  @$pb.TagNumber(2)
  set serverIp($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerIp() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerIp() => $_clearField(2);

  @$pb.TagNumber(3)
  SettingsCertificateType get certificateType => $_getN(2);
  @$pb.TagNumber(3)
  set certificateType(SettingsCertificateType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasCertificateType() => $_has(2);
  @$pb.TagNumber(3)
  void clearCertificateType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get customCertResolver => $_getSZ(3);
  @$pb.TagNumber(4)
  set customCertResolver($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCustomCertResolver() => $_has(3);
  @$pb.TagNumber(4)
  void clearCustomCertResolver() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get https => $_getBF(4);
  @$pb.TagNumber(5)
  set https($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHttps() => $_has(4);
  @$pb.TagNumber(5)
  void clearHttps() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get host => $_getSZ(5);
  @$pb.TagNumber(6)
  set host($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHost() => $_has(5);
  @$pb.TagNumber(6)
  void clearHost() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get letsEncryptEmail => $_getSZ(6);
  @$pb.TagNumber(7)
  set letsEncryptEmail($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasLetsEncryptEmail() => $_has(6);
  @$pb.TagNumber(7)
  void clearLetsEncryptEmail() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get enableDockerCleanup => $_getBF(7);
  @$pb.TagNumber(8)
  set enableDockerCleanup($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEnableDockerCleanup() => $_has(7);
  @$pb.TagNumber(8)
  void clearEnableDockerCleanup() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get logCleanupCron => $_getSZ(8);
  @$pb.TagNumber(9)
  set logCleanupCron($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasLogCleanupCron() => $_has(8);
  @$pb.TagNumber(9)
  void clearLogCleanupCron() => $_clearField(9);

  /// Metrics scrape configuration as an opaque JSON document ("{}" default).
  @$pb.TagNumber(10)
  $core.String get metricsConfig => $_getSZ(9);
  @$pb.TagNumber(10)
  set metricsConfig($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasMetricsConfig() => $_has(9);
  @$pb.TagNumber(10)
  void clearMetricsConfig() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get panelBackupCron => $_getSZ(10);
  @$pb.TagNumber(11)
  set panelBackupCron($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasPanelBackupCron() => $_has(10);
  @$pb.TagNumber(11)
  void clearPanelBackupCron() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get logRetentionDays => $_getI64(11);
  @$pb.TagNumber(12)
  set logRetentionDays($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasLogRetentionDays() => $_has(11);
  @$pb.TagNumber(12)
  void clearLogRetentionDays() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get panelBackupEnabled => $_getBF(12);
  @$pb.TagNumber(13)
  set panelBackupEnabled($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasPanelBackupEnabled() => $_has(12);
  @$pb.TagNumber(13)
  void clearPanelBackupEnabled() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.bool get logCleanupEnabled => $_getBF(13);
  @$pb.TagNumber(14)
  set logCleanupEnabled($core.bool value) => $_setBool(13, value);
  @$pb.TagNumber(14)
  $core.bool hasLogCleanupEnabled() => $_has(13);
  @$pb.TagNumber(14)
  void clearLogCleanupEnabled() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get updatedAt => $_getI64(14);
  @$pb.TagNumber(15)
  set updatedAt($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasUpdatedAt() => $_has(14);
  @$pb.TagNumber(15)
  void clearUpdatedAt() => $_clearField(15);
}

class SettingsGetSettingsRequest extends $pb.GeneratedMessage {
  factory SettingsGetSettingsRequest() => SettingsGetSettingsRequest._();

  SettingsGetSettingsRequest._();

  factory SettingsGetSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsGetSettingsRequest()..mergeFromBuffer(data, registry);
  factory SettingsGetSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsGetSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsGetSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SettingsGetSettingsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsGetSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsGetSettingsRequest copyWith(
          void Function(SettingsGetSettingsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SettingsGetSettingsRequest))
          as SettingsGetSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SettingsGetSettingsRequest() / SettingsGetSettingsRequest.new instead')
  static SettingsGetSettingsRequest create() => SettingsGetSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SettingsGetSettingsRequest._();
  @$core.override
  SettingsGetSettingsRequest createEmptyInstance() =>
      SettingsGetSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static SettingsGetSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsGetSettingsRequest>(
          SettingsGetSettingsRequest.$_createMessage);
  static SettingsGetSettingsRequest? _defaultInstance;
}

class SettingsGetSettingsResponse extends $pb.GeneratedMessage {
  factory SettingsGetSettingsResponse({
    Settings? settings,
  }) {
    final result = SettingsGetSettingsResponse._();
    if (settings != null) result.settings = settings;
    return result;
  }

  SettingsGetSettingsResponse._();

  factory SettingsGetSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsGetSettingsResponse()..mergeFromBuffer(data, registry);
  factory SettingsGetSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsGetSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsGetSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SettingsGetSettingsResponse.$_createMessage)
    ..aOM<Settings>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: Settings.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsGetSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsGetSettingsResponse copyWith(
          void Function(SettingsGetSettingsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SettingsGetSettingsResponse))
          as SettingsGetSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SettingsGetSettingsResponse() / SettingsGetSettingsResponse.new instead')
  static SettingsGetSettingsResponse create() =>
      SettingsGetSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SettingsGetSettingsResponse._();
  @$core.override
  SettingsGetSettingsResponse createEmptyInstance() =>
      SettingsGetSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static SettingsGetSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsGetSettingsResponse>(
          SettingsGetSettingsResponse.$_createMessage);
  static SettingsGetSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Settings get settings => $_getN(0);
  @$pb.TagNumber(1)
  set settings(Settings value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSettings() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettings() => $_clearField(1);
  @$pb.TagNumber(1)
  Settings ensureSettings() => $_ensure(0);
}

/// Patch semantics: an absent field keeps its stored value.
class SettingsUpdateSettingsRequest extends $pb.GeneratedMessage {
  factory SettingsUpdateSettingsRequest({
    $core.String? serverIp,
    SettingsCertificateType? certificateType,
    $core.String? customCertResolver,
    $core.bool? https,
    $core.String? host,
    $core.String? letsEncryptEmail,
    $core.bool? enableDockerCleanup,
    $core.String? logCleanupCron,
    $core.String? metricsConfig,
    $core.String? panelBackupCron,
    $fixnum.Int64? logRetentionDays,
    $core.bool? panelBackupEnabled,
    $core.bool? logCleanupEnabled,
  }) {
    final result = SettingsUpdateSettingsRequest._();
    if (serverIp != null) result.serverIp = serverIp;
    if (certificateType != null) result.certificateType = certificateType;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (https != null) result.https = https;
    if (host != null) result.host = host;
    if (letsEncryptEmail != null) result.letsEncryptEmail = letsEncryptEmail;
    if (enableDockerCleanup != null)
      result.enableDockerCleanup = enableDockerCleanup;
    if (logCleanupCron != null) result.logCleanupCron = logCleanupCron;
    if (metricsConfig != null) result.metricsConfig = metricsConfig;
    if (panelBackupCron != null) result.panelBackupCron = panelBackupCron;
    if (logRetentionDays != null) result.logRetentionDays = logRetentionDays;
    if (panelBackupEnabled != null)
      result.panelBackupEnabled = panelBackupEnabled;
    if (logCleanupEnabled != null) result.logCleanupEnabled = logCleanupEnabled;
    return result;
  }

  SettingsUpdateSettingsRequest._();

  factory SettingsUpdateSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsUpdateSettingsRequest()..mergeFromBuffer(data, registry);
  factory SettingsUpdateSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsUpdateSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsUpdateSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SettingsUpdateSettingsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'serverIp')
    ..aE<SettingsCertificateType>(2, _omitFieldNames ? '' : 'certificateType',
        enumValues: SettingsCertificateType.values)
    ..aOS(3, _omitFieldNames ? '' : 'customCertResolver')
    ..aOB(4, _omitFieldNames ? '' : 'https')
    ..aOS(5, _omitFieldNames ? '' : 'host')
    ..aOS(6, _omitFieldNames ? '' : 'letsEncryptEmail')
    ..aOB(7, _omitFieldNames ? '' : 'enableDockerCleanup')
    ..aOS(8, _omitFieldNames ? '' : 'logCleanupCron')
    ..aOS(9, _omitFieldNames ? '' : 'metricsConfig')
    ..aOS(10, _omitFieldNames ? '' : 'panelBackupCron')
    ..aInt64(11, _omitFieldNames ? '' : 'logRetentionDays')
    ..aOB(12, _omitFieldNames ? '' : 'panelBackupEnabled')
    ..aOB(13, _omitFieldNames ? '' : 'logCleanupEnabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsUpdateSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsUpdateSettingsRequest copyWith(
          void Function(SettingsUpdateSettingsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SettingsUpdateSettingsRequest))
          as SettingsUpdateSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SettingsUpdateSettingsRequest() / SettingsUpdateSettingsRequest.new instead')
  static SettingsUpdateSettingsRequest create() =>
      SettingsUpdateSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SettingsUpdateSettingsRequest._();
  @$core.override
  SettingsUpdateSettingsRequest createEmptyInstance() =>
      SettingsUpdateSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static SettingsUpdateSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsUpdateSettingsRequest>(
          SettingsUpdateSettingsRequest.$_createMessage);
  static SettingsUpdateSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get serverIp => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverIp($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerIp() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerIp() => $_clearField(1);

  @$pb.TagNumber(2)
  SettingsCertificateType get certificateType => $_getN(1);
  @$pb.TagNumber(2)
  set certificateType(SettingsCertificateType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasCertificateType() => $_has(1);
  @$pb.TagNumber(2)
  void clearCertificateType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get customCertResolver => $_getSZ(2);
  @$pb.TagNumber(3)
  set customCertResolver($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCustomCertResolver() => $_has(2);
  @$pb.TagNumber(3)
  void clearCustomCertResolver() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get https => $_getBF(3);
  @$pb.TagNumber(4)
  set https($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasHttps() => $_has(3);
  @$pb.TagNumber(4)
  void clearHttps() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get host => $_getSZ(4);
  @$pb.TagNumber(5)
  set host($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHost() => $_has(4);
  @$pb.TagNumber(5)
  void clearHost() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get letsEncryptEmail => $_getSZ(5);
  @$pb.TagNumber(6)
  set letsEncryptEmail($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLetsEncryptEmail() => $_has(5);
  @$pb.TagNumber(6)
  void clearLetsEncryptEmail() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get enableDockerCleanup => $_getBF(6);
  @$pb.TagNumber(7)
  set enableDockerCleanup($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasEnableDockerCleanup() => $_has(6);
  @$pb.TagNumber(7)
  void clearEnableDockerCleanup() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get logCleanupCron => $_getSZ(7);
  @$pb.TagNumber(8)
  set logCleanupCron($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasLogCleanupCron() => $_has(7);
  @$pb.TagNumber(8)
  void clearLogCleanupCron() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get metricsConfig => $_getSZ(8);
  @$pb.TagNumber(9)
  set metricsConfig($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasMetricsConfig() => $_has(8);
  @$pb.TagNumber(9)
  void clearMetricsConfig() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get panelBackupCron => $_getSZ(9);
  @$pb.TagNumber(10)
  set panelBackupCron($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasPanelBackupCron() => $_has(9);
  @$pb.TagNumber(10)
  void clearPanelBackupCron() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get logRetentionDays => $_getI64(10);
  @$pb.TagNumber(11)
  set logRetentionDays($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLogRetentionDays() => $_has(10);
  @$pb.TagNumber(11)
  void clearLogRetentionDays() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.bool get panelBackupEnabled => $_getBF(11);
  @$pb.TagNumber(12)
  set panelBackupEnabled($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(12)
  $core.bool hasPanelBackupEnabled() => $_has(11);
  @$pb.TagNumber(12)
  void clearPanelBackupEnabled() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get logCleanupEnabled => $_getBF(12);
  @$pb.TagNumber(13)
  set logCleanupEnabled($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasLogCleanupEnabled() => $_has(12);
  @$pb.TagNumber(13)
  void clearLogCleanupEnabled() => $_clearField(13);
}

class SettingsUpdateSettingsResponse extends $pb.GeneratedMessage {
  factory SettingsUpdateSettingsResponse({
    Settings? settings,
  }) {
    final result = SettingsUpdateSettingsResponse._();
    if (settings != null) result.settings = settings;
    return result;
  }

  SettingsUpdateSettingsResponse._();

  factory SettingsUpdateSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsUpdateSettingsResponse()..mergeFromBuffer(data, registry);
  factory SettingsUpdateSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SettingsUpdateSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsUpdateSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SettingsUpdateSettingsResponse.$_createMessage)
    ..aOM<Settings>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: Settings.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsUpdateSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsUpdateSettingsResponse copyWith(
          void Function(SettingsUpdateSettingsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SettingsUpdateSettingsResponse))
          as SettingsUpdateSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SettingsUpdateSettingsResponse() / SettingsUpdateSettingsResponse.new instead')
  static SettingsUpdateSettingsResponse create() =>
      SettingsUpdateSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SettingsUpdateSettingsResponse._();
  @$core.override
  SettingsUpdateSettingsResponse createEmptyInstance() =>
      SettingsUpdateSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static SettingsUpdateSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsUpdateSettingsResponse>(
          SettingsUpdateSettingsResponse.$_createMessage);
  static SettingsUpdateSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Settings get settings => $_getN(0);
  @$pb.TagNumber(1)
  set settings(Settings value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSettings() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettings() => $_clearField(1);
  @$pb.TagNumber(1)
  Settings ensureSettings() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

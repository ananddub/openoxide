// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/application.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'application.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'application.pbenum.dart';

class Application extends $pb.GeneratedMessage {
  factory Application({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? appName,
    ApplicationSourceType? sourceType,
    ApplicationBuildType? buildType,
    $core.String? status,
    $core.String? lastError,
    $core.String? triggerType,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? buildServerId,
    $fixnum.Int64? registryId,
    $core.Iterable<$core.String>? networkIds,
    $core.bool? detachRustployNetwork,
    $core.String? envVar,
    $core.String? icon,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? dockerImage,
    $core.String? registryUrl,
    $core.String? customGitUrl,
    $core.String? customGitBranch,
    $core.String? previewWildcard,
    $fixnum.Int64? previewPort,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.String? dockerfile,
    $core.String? dockerContextPath,
    $core.String? dockerBuildStage,
    $core.String? publishDirectory,
    $core.bool? isStaticSpa,
    $core.String? railpackVersion,
    $core.String? herokuVersion,
    $core.String? buildPath,
    $core.String? dropBuildPath,
    $core.bool? cleanCache,
    $core.bool? autoDeploy,
    $core.String? memoryReservation,
    $core.String? memoryLimit,
    $core.String? cpuReservation,
    $core.String? cpuLimit,
    $fixnum.Int64? replicas,
    $core.String? command,
    $core.String? args,
    $core.String? watchPaths,
    $core.bool? enableSubmodules,
  }) {
    final result = Application._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (appName != null) result.appName = appName;
    if (sourceType != null) result.sourceType = sourceType;
    if (buildType != null) result.buildType = buildType;
    if (status != null) result.status = status;
    if (lastError != null) result.lastError = lastError;
    if (triggerType != null) result.triggerType = triggerType;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (buildServerId != null) result.buildServerId = buildServerId;
    if (registryId != null) result.registryId = registryId;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (detachRustployNetwork != null)
      result.detachRustployNetwork = detachRustployNetwork;
    if (envVar != null) result.envVar = envVar;
    if (icon != null) result.icon = icon;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (dockerImage != null) result.dockerImage = dockerImage;
    if (registryUrl != null) result.registryUrl = registryUrl;
    if (customGitUrl != null) result.customGitUrl = customGitUrl;
    if (customGitBranch != null) result.customGitBranch = customGitBranch;
    if (previewWildcard != null) result.previewWildcard = previewWildcard;
    if (previewPort != null) result.previewPort = previewPort;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (dockerfile != null) result.dockerfile = dockerfile;
    if (dockerContextPath != null) result.dockerContextPath = dockerContextPath;
    if (dockerBuildStage != null) result.dockerBuildStage = dockerBuildStage;
    if (publishDirectory != null) result.publishDirectory = publishDirectory;
    if (isStaticSpa != null) result.isStaticSpa = isStaticSpa;
    if (railpackVersion != null) result.railpackVersion = railpackVersion;
    if (herokuVersion != null) result.herokuVersion = herokuVersion;
    if (buildPath != null) result.buildPath = buildPath;
    if (dropBuildPath != null) result.dropBuildPath = dropBuildPath;
    if (cleanCache != null) result.cleanCache = cleanCache;
    if (autoDeploy != null) result.autoDeploy = autoDeploy;
    if (memoryReservation != null) result.memoryReservation = memoryReservation;
    if (memoryLimit != null) result.memoryLimit = memoryLimit;
    if (cpuReservation != null) result.cpuReservation = cpuReservation;
    if (cpuLimit != null) result.cpuLimit = cpuLimit;
    if (replicas != null) result.replicas = replicas;
    if (command != null) result.command = command;
    if (args != null) result.args = args;
    if (watchPaths != null) result.watchPaths = watchPaths;
    if (enableSubmodules != null) result.enableSubmodules = enableSubmodules;
    return result;
  }

  Application._();

  factory Application.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Application()..mergeFromBuffer(data, registry);
  factory Application.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Application()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Application',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Application.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'appName')
    ..aE<ApplicationSourceType>(5, _omitFieldNames ? '' : 'sourceType',
        enumValues: ApplicationSourceType.values)
    ..aE<ApplicationBuildType>(6, _omitFieldNames ? '' : 'buildType',
        enumValues: ApplicationBuildType.values)
    ..aOS(7, _omitFieldNames ? '' : 'status')
    ..aOS(8, _omitFieldNames ? '' : 'lastError')
    ..aOS(9, _omitFieldNames ? '' : 'triggerType')
    ..aInt64(10, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(11, _omitFieldNames ? '' : 'serverId')
    ..aInt64(12, _omitFieldNames ? '' : 'buildServerId')
    ..aInt64(13, _omitFieldNames ? '' : 'registryId')
    ..pPS(14, _omitFieldNames ? '' : 'networkIds')
    ..aOB(15, _omitFieldNames ? '' : 'detachRustployNetwork')
    ..aOS(16, _omitFieldNames ? '' : 'envVar')
    ..aOS(17, _omitFieldNames ? '' : 'icon')
    ..aOS(18, _omitFieldNames ? '' : 'repository')
    ..aOS(19, _omitFieldNames ? '' : 'owner')
    ..aOS(20, _omitFieldNames ? '' : 'branch')
    ..aOS(21, _omitFieldNames ? '' : 'dockerImage')
    ..aOS(22, _omitFieldNames ? '' : 'registryUrl')
    ..aOS(23, _omitFieldNames ? '' : 'customGitUrl')
    ..aOS(24, _omitFieldNames ? '' : 'customGitBranch')
    ..aOS(25, _omitFieldNames ? '' : 'previewWildcard')
    ..aInt64(26, _omitFieldNames ? '' : 'previewPort')
    ..aInt64(27, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(28, _omitFieldNames ? '' : 'updatedAt')
    ..aOS(29, _omitFieldNames ? '' : 'dockerfile')
    ..aOS(30, _omitFieldNames ? '' : 'dockerContextPath')
    ..aOS(31, _omitFieldNames ? '' : 'dockerBuildStage')
    ..aOS(32, _omitFieldNames ? '' : 'publishDirectory')
    ..aOB(33, _omitFieldNames ? '' : 'isStaticSpa')
    ..aOS(34, _omitFieldNames ? '' : 'railpackVersion')
    ..aOS(35, _omitFieldNames ? '' : 'herokuVersion')
    ..aOS(36, _omitFieldNames ? '' : 'buildPath')
    ..aOS(37, _omitFieldNames ? '' : 'dropBuildPath')
    ..aOB(38, _omitFieldNames ? '' : 'cleanCache')
    ..aOB(39, _omitFieldNames ? '' : 'autoDeploy')
    ..aOS(40, _omitFieldNames ? '' : 'memoryReservation')
    ..aOS(41, _omitFieldNames ? '' : 'memoryLimit')
    ..aOS(42, _omitFieldNames ? '' : 'cpuReservation')
    ..aOS(43, _omitFieldNames ? '' : 'cpuLimit')
    ..aInt64(44, _omitFieldNames ? '' : 'replicas')
    ..aOS(45, _omitFieldNames ? '' : 'command')
    ..aOS(46, _omitFieldNames ? '' : 'args')
    ..aOS(47, _omitFieldNames ? '' : 'watchPaths')
    ..aOB(48, _omitFieldNames ? '' : 'enableSubmodules')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Application clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Application copyWith(void Function(Application) updates) =>
      super.copyWith((message) => updates(message as Application))
          as Application;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Application() / Application.new instead')
  static Application create() => Application._();
  static $pb.GeneratedMessage $_createMessage() => Application._();
  @$core.override
  Application createEmptyInstance() => Application._();
  @$core.pragma('dart2js:noInline')
  static Application getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Application>(
          Application.$_createMessage);
  static Application? _defaultInstance;

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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get appName => $_getSZ(3);
  @$pb.TagNumber(4)
  set appName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAppName() => $_has(3);
  @$pb.TagNumber(4)
  void clearAppName() => $_clearField(4);

  @$pb.TagNumber(5)
  ApplicationSourceType get sourceType => $_getN(4);
  @$pb.TagNumber(5)
  set sourceType(ApplicationSourceType value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSourceType() => $_has(4);
  @$pb.TagNumber(5)
  void clearSourceType() => $_clearField(5);

  @$pb.TagNumber(6)
  ApplicationBuildType get buildType => $_getN(5);
  @$pb.TagNumber(6)
  set buildType(ApplicationBuildType value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasBuildType() => $_has(5);
  @$pb.TagNumber(6)
  void clearBuildType() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get status => $_getSZ(6);
  @$pb.TagNumber(7)
  set status($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasStatus() => $_has(6);
  @$pb.TagNumber(7)
  void clearStatus() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get lastError => $_getSZ(7);
  @$pb.TagNumber(8)
  set lastError($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasLastError() => $_has(7);
  @$pb.TagNumber(8)
  void clearLastError() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get triggerType => $_getSZ(8);
  @$pb.TagNumber(9)
  set triggerType($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasTriggerType() => $_has(8);
  @$pb.TagNumber(9)
  void clearTriggerType() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get environmentId => $_getI64(9);
  @$pb.TagNumber(10)
  set environmentId($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasEnvironmentId() => $_has(9);
  @$pb.TagNumber(10)
  void clearEnvironmentId() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get serverId => $_getI64(10);
  @$pb.TagNumber(11)
  set serverId($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasServerId() => $_has(10);
  @$pb.TagNumber(11)
  void clearServerId() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get buildServerId => $_getI64(11);
  @$pb.TagNumber(12)
  set buildServerId($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasBuildServerId() => $_has(11);
  @$pb.TagNumber(12)
  void clearBuildServerId() => $_clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get registryId => $_getI64(12);
  @$pb.TagNumber(13)
  set registryId($fixnum.Int64 value) => $_setInt64(12, value);
  @$pb.TagNumber(13)
  $core.bool hasRegistryId() => $_has(12);
  @$pb.TagNumber(13)
  void clearRegistryId() => $_clearField(13);

  @$pb.TagNumber(14)
  $pb.PbList<$core.String> get networkIds => $_getList(13);

  @$pb.TagNumber(15)
  $core.bool get detachRustployNetwork => $_getBF(14);
  @$pb.TagNumber(15)
  set detachRustployNetwork($core.bool value) => $_setBool(14, value);
  @$pb.TagNumber(15)
  $core.bool hasDetachRustployNetwork() => $_has(14);
  @$pb.TagNumber(15)
  void clearDetachRustployNetwork() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get envVar => $_getSZ(15);
  @$pb.TagNumber(16)
  set envVar($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasEnvVar() => $_has(15);
  @$pb.TagNumber(16)
  void clearEnvVar() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.String get icon => $_getSZ(16);
  @$pb.TagNumber(17)
  set icon($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasIcon() => $_has(16);
  @$pb.TagNumber(17)
  void clearIcon() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.String get repository => $_getSZ(17);
  @$pb.TagNumber(18)
  set repository($core.String value) => $_setString(17, value);
  @$pb.TagNumber(18)
  $core.bool hasRepository() => $_has(17);
  @$pb.TagNumber(18)
  void clearRepository() => $_clearField(18);

  @$pb.TagNumber(19)
  $core.String get owner => $_getSZ(18);
  @$pb.TagNumber(19)
  set owner($core.String value) => $_setString(18, value);
  @$pb.TagNumber(19)
  $core.bool hasOwner() => $_has(18);
  @$pb.TagNumber(19)
  void clearOwner() => $_clearField(19);

  @$pb.TagNumber(20)
  $core.String get branch => $_getSZ(19);
  @$pb.TagNumber(20)
  set branch($core.String value) => $_setString(19, value);
  @$pb.TagNumber(20)
  $core.bool hasBranch() => $_has(19);
  @$pb.TagNumber(20)
  void clearBranch() => $_clearField(20);

  @$pb.TagNumber(21)
  $core.String get dockerImage => $_getSZ(20);
  @$pb.TagNumber(21)
  set dockerImage($core.String value) => $_setString(20, value);
  @$pb.TagNumber(21)
  $core.bool hasDockerImage() => $_has(20);
  @$pb.TagNumber(21)
  void clearDockerImage() => $_clearField(21);

  @$pb.TagNumber(22)
  $core.String get registryUrl => $_getSZ(21);
  @$pb.TagNumber(22)
  set registryUrl($core.String value) => $_setString(21, value);
  @$pb.TagNumber(22)
  $core.bool hasRegistryUrl() => $_has(21);
  @$pb.TagNumber(22)
  void clearRegistryUrl() => $_clearField(22);

  @$pb.TagNumber(23)
  $core.String get customGitUrl => $_getSZ(22);
  @$pb.TagNumber(23)
  set customGitUrl($core.String value) => $_setString(22, value);
  @$pb.TagNumber(23)
  $core.bool hasCustomGitUrl() => $_has(22);
  @$pb.TagNumber(23)
  void clearCustomGitUrl() => $_clearField(23);

  @$pb.TagNumber(24)
  $core.String get customGitBranch => $_getSZ(23);
  @$pb.TagNumber(24)
  set customGitBranch($core.String value) => $_setString(23, value);
  @$pb.TagNumber(24)
  $core.bool hasCustomGitBranch() => $_has(23);
  @$pb.TagNumber(24)
  void clearCustomGitBranch() => $_clearField(24);

  @$pb.TagNumber(25)
  $core.String get previewWildcard => $_getSZ(24);
  @$pb.TagNumber(25)
  set previewWildcard($core.String value) => $_setString(24, value);
  @$pb.TagNumber(25)
  $core.bool hasPreviewWildcard() => $_has(24);
  @$pb.TagNumber(25)
  void clearPreviewWildcard() => $_clearField(25);

  @$pb.TagNumber(26)
  $fixnum.Int64 get previewPort => $_getI64(25);
  @$pb.TagNumber(26)
  set previewPort($fixnum.Int64 value) => $_setInt64(25, value);
  @$pb.TagNumber(26)
  $core.bool hasPreviewPort() => $_has(25);
  @$pb.TagNumber(26)
  void clearPreviewPort() => $_clearField(26);

  @$pb.TagNumber(27)
  $fixnum.Int64 get createdAt => $_getI64(26);
  @$pb.TagNumber(27)
  set createdAt($fixnum.Int64 value) => $_setInt64(26, value);
  @$pb.TagNumber(27)
  $core.bool hasCreatedAt() => $_has(26);
  @$pb.TagNumber(27)
  void clearCreatedAt() => $_clearField(27);

  @$pb.TagNumber(28)
  $fixnum.Int64 get updatedAt => $_getI64(27);
  @$pb.TagNumber(28)
  set updatedAt($fixnum.Int64 value) => $_setInt64(27, value);
  @$pb.TagNumber(28)
  $core.bool hasUpdatedAt() => $_has(27);
  @$pb.TagNumber(28)
  void clearUpdatedAt() => $_clearField(28);

  /// Build/resource round-trip fields: the settings cards read these back after save; without them every card re-rendered blank defaults.
  @$pb.TagNumber(29)
  $core.String get dockerfile => $_getSZ(28);
  @$pb.TagNumber(29)
  set dockerfile($core.String value) => $_setString(28, value);
  @$pb.TagNumber(29)
  $core.bool hasDockerfile() => $_has(28);
  @$pb.TagNumber(29)
  void clearDockerfile() => $_clearField(29);

  @$pb.TagNumber(30)
  $core.String get dockerContextPath => $_getSZ(29);
  @$pb.TagNumber(30)
  set dockerContextPath($core.String value) => $_setString(29, value);
  @$pb.TagNumber(30)
  $core.bool hasDockerContextPath() => $_has(29);
  @$pb.TagNumber(30)
  void clearDockerContextPath() => $_clearField(30);

  @$pb.TagNumber(31)
  $core.String get dockerBuildStage => $_getSZ(30);
  @$pb.TagNumber(31)
  set dockerBuildStage($core.String value) => $_setString(30, value);
  @$pb.TagNumber(31)
  $core.bool hasDockerBuildStage() => $_has(30);
  @$pb.TagNumber(31)
  void clearDockerBuildStage() => $_clearField(31);

  @$pb.TagNumber(32)
  $core.String get publishDirectory => $_getSZ(31);
  @$pb.TagNumber(32)
  set publishDirectory($core.String value) => $_setString(31, value);
  @$pb.TagNumber(32)
  $core.bool hasPublishDirectory() => $_has(31);
  @$pb.TagNumber(32)
  void clearPublishDirectory() => $_clearField(32);

  @$pb.TagNumber(33)
  $core.bool get isStaticSpa => $_getBF(32);
  @$pb.TagNumber(33)
  set isStaticSpa($core.bool value) => $_setBool(32, value);
  @$pb.TagNumber(33)
  $core.bool hasIsStaticSpa() => $_has(32);
  @$pb.TagNumber(33)
  void clearIsStaticSpa() => $_clearField(33);

  @$pb.TagNumber(34)
  $core.String get railpackVersion => $_getSZ(33);
  @$pb.TagNumber(34)
  set railpackVersion($core.String value) => $_setString(33, value);
  @$pb.TagNumber(34)
  $core.bool hasRailpackVersion() => $_has(33);
  @$pb.TagNumber(34)
  void clearRailpackVersion() => $_clearField(34);

  @$pb.TagNumber(35)
  $core.String get herokuVersion => $_getSZ(34);
  @$pb.TagNumber(35)
  set herokuVersion($core.String value) => $_setString(34, value);
  @$pb.TagNumber(35)
  $core.bool hasHerokuVersion() => $_has(34);
  @$pb.TagNumber(35)
  void clearHerokuVersion() => $_clearField(35);

  @$pb.TagNumber(36)
  $core.String get buildPath => $_getSZ(35);
  @$pb.TagNumber(36)
  set buildPath($core.String value) => $_setString(35, value);
  @$pb.TagNumber(36)
  $core.bool hasBuildPath() => $_has(35);
  @$pb.TagNumber(36)
  void clearBuildPath() => $_clearField(36);

  @$pb.TagNumber(37)
  $core.String get dropBuildPath => $_getSZ(36);
  @$pb.TagNumber(37)
  set dropBuildPath($core.String value) => $_setString(36, value);
  @$pb.TagNumber(37)
  $core.bool hasDropBuildPath() => $_has(36);
  @$pb.TagNumber(37)
  void clearDropBuildPath() => $_clearField(37);

  @$pb.TagNumber(38)
  $core.bool get cleanCache => $_getBF(37);
  @$pb.TagNumber(38)
  set cleanCache($core.bool value) => $_setBool(37, value);
  @$pb.TagNumber(38)
  $core.bool hasCleanCache() => $_has(37);
  @$pb.TagNumber(38)
  void clearCleanCache() => $_clearField(38);

  @$pb.TagNumber(39)
  $core.bool get autoDeploy => $_getBF(38);
  @$pb.TagNumber(39)
  set autoDeploy($core.bool value) => $_setBool(38, value);
  @$pb.TagNumber(39)
  $core.bool hasAutoDeploy() => $_has(38);
  @$pb.TagNumber(39)
  void clearAutoDeploy() => $_clearField(39);

  @$pb.TagNumber(40)
  $core.String get memoryReservation => $_getSZ(39);
  @$pb.TagNumber(40)
  set memoryReservation($core.String value) => $_setString(39, value);
  @$pb.TagNumber(40)
  $core.bool hasMemoryReservation() => $_has(39);
  @$pb.TagNumber(40)
  void clearMemoryReservation() => $_clearField(40);

  @$pb.TagNumber(41)
  $core.String get memoryLimit => $_getSZ(40);
  @$pb.TagNumber(41)
  set memoryLimit($core.String value) => $_setString(40, value);
  @$pb.TagNumber(41)
  $core.bool hasMemoryLimit() => $_has(40);
  @$pb.TagNumber(41)
  void clearMemoryLimit() => $_clearField(41);

  @$pb.TagNumber(42)
  $core.String get cpuReservation => $_getSZ(41);
  @$pb.TagNumber(42)
  set cpuReservation($core.String value) => $_setString(41, value);
  @$pb.TagNumber(42)
  $core.bool hasCpuReservation() => $_has(41);
  @$pb.TagNumber(42)
  void clearCpuReservation() => $_clearField(42);

  @$pb.TagNumber(43)
  $core.String get cpuLimit => $_getSZ(42);
  @$pb.TagNumber(43)
  set cpuLimit($core.String value) => $_setString(42, value);
  @$pb.TagNumber(43)
  $core.bool hasCpuLimit() => $_has(42);
  @$pb.TagNumber(43)
  void clearCpuLimit() => $_clearField(43);

  @$pb.TagNumber(44)
  $fixnum.Int64 get replicas => $_getI64(43);
  @$pb.TagNumber(44)
  set replicas($fixnum.Int64 value) => $_setInt64(43, value);
  @$pb.TagNumber(44)
  $core.bool hasReplicas() => $_has(43);
  @$pb.TagNumber(44)
  void clearReplicas() => $_clearField(44);

  @$pb.TagNumber(45)
  $core.String get command => $_getSZ(44);
  @$pb.TagNumber(45)
  set command($core.String value) => $_setString(44, value);
  @$pb.TagNumber(45)
  $core.bool hasCommand() => $_has(44);
  @$pb.TagNumber(45)
  void clearCommand() => $_clearField(45);

  @$pb.TagNumber(46)
  $core.String get args => $_getSZ(45);
  @$pb.TagNumber(46)
  set args($core.String value) => $_setString(45, value);
  @$pb.TagNumber(46)
  $core.bool hasArgs() => $_has(45);
  @$pb.TagNumber(46)
  void clearArgs() => $_clearField(46);

  @$pb.TagNumber(47)
  $core.String get watchPaths => $_getSZ(46);
  @$pb.TagNumber(47)
  set watchPaths($core.String value) => $_setString(46, value);
  @$pb.TagNumber(47)
  $core.bool hasWatchPaths() => $_has(46);
  @$pb.TagNumber(47)
  void clearWatchPaths() => $_clearField(47);

  @$pb.TagNumber(48)
  $core.bool get enableSubmodules => $_getBF(47);
  @$pb.TagNumber(48)
  set enableSubmodules($core.bool value) => $_setBool(47, value);
  @$pb.TagNumber(48)
  $core.bool hasEnableSubmodules() => $_has(47);
  @$pb.TagNumber(48)
  void clearEnableSubmodules() => $_clearField(48);
}

class GetApplicationRequest extends $pb.GeneratedMessage {
  factory GetApplicationRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetApplicationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetApplicationRequest._();

  factory GetApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetApplicationRequest()..mergeFromBuffer(data, registry);
  factory GetApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetApplicationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetApplicationRequest copyWith(
          void Function(GetApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as GetApplicationRequest))
          as GetApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetApplicationRequest() / GetApplicationRequest.new instead')
  static GetApplicationRequest create() => GetApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetApplicationRequest._();
  @$core.override
  GetApplicationRequest createEmptyInstance() => GetApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static GetApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetApplicationRequest>(
          GetApplicationRequest.$_createMessage);
  static GetApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListApplicationsRequest extends $pb.GeneratedMessage {
  factory ListApplicationsRequest({
    $fixnum.Int64? environmentId,
  }) {
    final result = ListApplicationsRequest._();
    if (environmentId != null) result.environmentId = environmentId;
    return result;
  }

  ListApplicationsRequest._();

  factory ListApplicationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationsRequest()..mergeFromBuffer(data, registry);
  factory ListApplicationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListApplicationsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListApplicationsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'environmentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationsRequest copyWith(
          void Function(ListApplicationsRequest) updates) =>
      super.copyWith((message) => updates(message as ListApplicationsRequest))
          as ListApplicationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListApplicationsRequest() / ListApplicationsRequest.new instead')
  static ListApplicationsRequest create() => ListApplicationsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListApplicationsRequest._();
  @$core.override
  ListApplicationsRequest createEmptyInstance() => ListApplicationsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListApplicationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListApplicationsRequest>(
          ListApplicationsRequest.$_createMessage);
  static ListApplicationsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get environmentId => $_getI64(0);
  @$pb.TagNumber(1)
  set environmentId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEnvironmentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnvironmentId() => $_clearField(1);
}

class ListApplicationsResponse extends $pb.GeneratedMessage {
  factory ListApplicationsResponse({
    $core.Iterable<Application>? applications,
  }) {
    final result = ListApplicationsResponse._();
    if (applications != null) result.applications.addAll(applications);
    return result;
  }

  ListApplicationsResponse._();

  factory ListApplicationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationsResponse()..mergeFromBuffer(data, registry);
  factory ListApplicationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListApplicationsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListApplicationsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListApplicationsResponse.$_createMessage)
    ..pPM<Application>(1, _omitFieldNames ? '' : 'applications',
        subBuilder: Application.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListApplicationsResponse copyWith(
          void Function(ListApplicationsResponse) updates) =>
      super.copyWith((message) => updates(message as ListApplicationsResponse))
          as ListApplicationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListApplicationsResponse() / ListApplicationsResponse.new instead')
  static ListApplicationsResponse create() => ListApplicationsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListApplicationsResponse._();
  @$core.override
  ListApplicationsResponse createEmptyInstance() =>
      ListApplicationsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListApplicationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListApplicationsResponse>(
          ListApplicationsResponse.$_createMessage);
  static ListApplicationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Application> get applications => $_getList(0);
}

class CreateApplicationRequest extends $pb.GeneratedMessage {
  factory CreateApplicationRequest({
    $core.String? name,
    $core.String? description,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    ApplicationSourceType? sourceType,
    ApplicationBuildType? buildType,
  }) {
    final result = CreateApplicationRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (sourceType != null) result.sourceType = sourceType;
    if (buildType != null) result.buildType = buildType;
    return result;
  }

  CreateApplicationRequest._();

  factory CreateApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateApplicationRequest()..mergeFromBuffer(data, registry);
  factory CreateApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateApplicationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aInt64(3, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(4, _omitFieldNames ? '' : 'serverId')
    ..aE<ApplicationSourceType>(5, _omitFieldNames ? '' : 'sourceType',
        enumValues: ApplicationSourceType.values)
    ..aE<ApplicationBuildType>(6, _omitFieldNames ? '' : 'buildType',
        enumValues: ApplicationBuildType.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateApplicationRequest copyWith(
          void Function(CreateApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as CreateApplicationRequest))
          as CreateApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateApplicationRequest() / CreateApplicationRequest.new instead')
  static CreateApplicationRequest create() => CreateApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateApplicationRequest._();
  @$core.override
  CreateApplicationRequest createEmptyInstance() =>
      CreateApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateApplicationRequest>(
          CreateApplicationRequest.$_createMessage);
  static CreateApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get environmentId => $_getI64(2);
  @$pb.TagNumber(3)
  set environmentId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasEnvironmentId() => $_has(2);
  @$pb.TagNumber(3)
  void clearEnvironmentId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get serverId => $_getI64(3);
  @$pb.TagNumber(4)
  set serverId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasServerId() => $_has(3);
  @$pb.TagNumber(4)
  void clearServerId() => $_clearField(4);

  @$pb.TagNumber(5)
  ApplicationSourceType get sourceType => $_getN(4);
  @$pb.TagNumber(5)
  set sourceType(ApplicationSourceType value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSourceType() => $_has(4);
  @$pb.TagNumber(5)
  void clearSourceType() => $_clearField(5);

  @$pb.TagNumber(6)
  ApplicationBuildType get buildType => $_getN(5);
  @$pb.TagNumber(6)
  set buildType(ApplicationBuildType value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasBuildType() => $_has(5);
  @$pb.TagNumber(6)
  void clearBuildType() => $_clearField(6);
}

class UpdateApplicationRequest extends $pb.GeneratedMessage {
  factory UpdateApplicationRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    ApplicationBuildType? buildType,
    $core.String? triggerType,
    $core.String? envVar,
    $fixnum.Int64? serverId,
    $fixnum.Int64? buildServerId,
    $fixnum.Int64? registryId,
    $core.Iterable<$core.String>? networkIds,
    $core.bool? detachRustployNetwork,
  }) {
    final result = UpdateApplicationRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (buildType != null) result.buildType = buildType;
    if (triggerType != null) result.triggerType = triggerType;
    if (envVar != null) result.envVar = envVar;
    if (serverId != null) result.serverId = serverId;
    if (buildServerId != null) result.buildServerId = buildServerId;
    if (registryId != null) result.registryId = registryId;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (detachRustployNetwork != null)
      result.detachRustployNetwork = detachRustployNetwork;
    return result;
  }

  UpdateApplicationRequest._();

  factory UpdateApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateApplicationRequest()..mergeFromBuffer(data, registry);
  factory UpdateApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateApplicationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aE<ApplicationBuildType>(4, _omitFieldNames ? '' : 'buildType',
        enumValues: ApplicationBuildType.values)
    ..aOS(5, _omitFieldNames ? '' : 'triggerType')
    ..aOS(6, _omitFieldNames ? '' : 'envVar')
    ..aInt64(7, _omitFieldNames ? '' : 'serverId')
    ..aInt64(8, _omitFieldNames ? '' : 'buildServerId')
    ..aInt64(9, _omitFieldNames ? '' : 'registryId')
    ..pPS(10, _omitFieldNames ? '' : 'networkIds')
    ..aOB(11, _omitFieldNames ? '' : 'detachRustployNetwork')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateApplicationRequest copyWith(
          void Function(UpdateApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateApplicationRequest))
          as UpdateApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateApplicationRequest() / UpdateApplicationRequest.new instead')
  static UpdateApplicationRequest create() => UpdateApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateApplicationRequest._();
  @$core.override
  UpdateApplicationRequest createEmptyInstance() =>
      UpdateApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateApplicationRequest>(
          UpdateApplicationRequest.$_createMessage);
  static UpdateApplicationRequest? _defaultInstance;

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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  ApplicationBuildType get buildType => $_getN(3);
  @$pb.TagNumber(4)
  set buildType(ApplicationBuildType value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasBuildType() => $_has(3);
  @$pb.TagNumber(4)
  void clearBuildType() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get triggerType => $_getSZ(4);
  @$pb.TagNumber(5)
  set triggerType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTriggerType() => $_has(4);
  @$pb.TagNumber(5)
  void clearTriggerType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get envVar => $_getSZ(5);
  @$pb.TagNumber(6)
  set envVar($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasEnvVar() => $_has(5);
  @$pb.TagNumber(6)
  void clearEnvVar() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get serverId => $_getI64(6);
  @$pb.TagNumber(7)
  set serverId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServerId() => $_has(6);
  @$pb.TagNumber(7)
  void clearServerId() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get buildServerId => $_getI64(7);
  @$pb.TagNumber(8)
  set buildServerId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBuildServerId() => $_has(7);
  @$pb.TagNumber(8)
  void clearBuildServerId() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get registryId => $_getI64(8);
  @$pb.TagNumber(9)
  set registryId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasRegistryId() => $_has(8);
  @$pb.TagNumber(9)
  void clearRegistryId() => $_clearField(9);

  @$pb.TagNumber(10)
  $pb.PbList<$core.String> get networkIds => $_getList(9);

  @$pb.TagNumber(11)
  $core.bool get detachRustployNetwork => $_getBF(10);
  @$pb.TagNumber(11)
  set detachRustployNetwork($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDetachRustployNetwork() => $_has(10);
  @$pb.TagNumber(11)
  void clearDetachRustployNetwork() => $_clearField(11);
}

class DeleteApplicationRequest extends $pb.GeneratedMessage {
  factory DeleteApplicationRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteApplicationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteApplicationRequest._();

  factory DeleteApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteApplicationRequest()..mergeFromBuffer(data, registry);
  factory DeleteApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteApplicationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteApplicationRequest copyWith(
          void Function(DeleteApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteApplicationRequest))
          as DeleteApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteApplicationRequest() / DeleteApplicationRequest.new instead')
  static DeleteApplicationRequest create() => DeleteApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteApplicationRequest._();
  @$core.override
  DeleteApplicationRequest createEmptyInstance() =>
      DeleteApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteApplicationRequest>(
          DeleteApplicationRequest.$_createMessage);
  static DeleteApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ImportApplicationRequest extends $pb.GeneratedMessage {
  factory ImportApplicationRequest({
    $core.String? archive,
  }) {
    final result = ImportApplicationRequest._();
    if (archive != null) result.archive = archive;
    return result;
  }

  ImportApplicationRequest._();

  factory ImportApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportApplicationRequest()..mergeFromBuffer(data, registry);
  factory ImportApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ImportApplicationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportApplicationRequest copyWith(
          void Function(ImportApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as ImportApplicationRequest))
          as ImportApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportApplicationRequest() / ImportApplicationRequest.new instead')
  static ImportApplicationRequest create() => ImportApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => ImportApplicationRequest._();
  @$core.override
  ImportApplicationRequest createEmptyInstance() =>
      ImportApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static ImportApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportApplicationRequest>(
          ImportApplicationRequest.$_createMessage);
  static ImportApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get archive => $_getSZ(0);
  @$pb.TagNumber(1)
  set archive($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);
}

class ExportApplicationRequest extends $pb.GeneratedMessage {
  factory ExportApplicationRequest({
    $fixnum.Int64? id,
    $core.bool? includeSecrets,
  }) {
    final result = ExportApplicationRequest._();
    if (id != null) result.id = id;
    if (includeSecrets != null) result.includeSecrets = includeSecrets;
    return result;
  }

  ExportApplicationRequest._();

  factory ExportApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportApplicationRequest()..mergeFromBuffer(data, registry);
  factory ExportApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExportApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ExportApplicationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'includeSecrets')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportApplicationRequest copyWith(
          void Function(ExportApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as ExportApplicationRequest))
          as ExportApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ExportApplicationRequest() / ExportApplicationRequest.new instead')
  static ExportApplicationRequest create() => ExportApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => ExportApplicationRequest._();
  @$core.override
  ExportApplicationRequest createEmptyInstance() =>
      ExportApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static ExportApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExportApplicationRequest>(
          ExportApplicationRequest.$_createMessage);
  static ExportApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get includeSecrets => $_getBF(1);
  @$pb.TagNumber(2)
  set includeSecrets($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasIncludeSecrets() => $_has(1);
  @$pb.TagNumber(2)
  void clearIncludeSecrets() => $_clearField(2);
}

class ApplicationExport extends $pb.GeneratedMessage {
  factory ApplicationExport({
    $core.String? format,
    $fixnum.Int64? schemaVersion,
    $core.String? archive,
  }) {
    final result = ApplicationExport._();
    if (format != null) result.format = format;
    if (schemaVersion != null) result.schemaVersion = schemaVersion;
    if (archive != null) result.archive = archive;
    return result;
  }

  ApplicationExport._();

  factory ApplicationExport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationExport()..mergeFromBuffer(data, registry);
  factory ApplicationExport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationExport()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationExport',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationExport.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'format')
    ..aInt64(2, _omitFieldNames ? '' : 'schemaVersion')
    ..aOS(3, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationExport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationExport copyWith(void Function(ApplicationExport) updates) =>
      super.copyWith((message) => updates(message as ApplicationExport))
          as ApplicationExport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationExport() / ApplicationExport.new instead')
  static ApplicationExport create() => ApplicationExport._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationExport._();
  @$core.override
  ApplicationExport createEmptyInstance() => ApplicationExport._();
  @$core.pragma('dart2js:noInline')
  static ApplicationExport getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationExport>(
          ApplicationExport.$_createMessage);
  static ApplicationExport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get format => $_getSZ(0);
  @$pb.TagNumber(1)
  set format($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFormat() => $_has(0);
  @$pb.TagNumber(1)
  void clearFormat() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get schemaVersion => $_getI64(1);
  @$pb.TagNumber(2)
  set schemaVersion($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSchemaVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearSchemaVersion() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get archive => $_getSZ(2);
  @$pb.TagNumber(3)
  set archive($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasArchive() => $_has(2);
  @$pb.TagNumber(3)
  void clearArchive() => $_clearField(3);
}

class SetGithubSourceRequest extends $pb.GeneratedMessage {
  factory SetGithubSourceRequest({
    $fixnum.Int64? id,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? buildPath,
    $fixnum.Int64? providerId,
    $core.bool? autoDeploy,
  }) {
    final result = SetGithubSourceRequest._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (providerId != null) result.providerId = providerId;
    if (autoDeploy != null) result.autoDeploy = autoDeploy;
    return result;
  }

  SetGithubSourceRequest._();

  factory SetGithubSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGithubSourceRequest()..mergeFromBuffer(data, registry);
  factory SetGithubSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGithubSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetGithubSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetGithubSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'buildPath')
    ..aInt64(6, _omitFieldNames ? '' : 'providerId')
    ..aOB(7, _omitFieldNames ? '' : 'autoDeploy')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGithubSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGithubSourceRequest copyWith(
          void Function(SetGithubSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetGithubSourceRequest))
          as SetGithubSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetGithubSourceRequest() / SetGithubSourceRequest.new instead')
  static SetGithubSourceRequest create() => SetGithubSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetGithubSourceRequest._();
  @$core.override
  SetGithubSourceRequest createEmptyInstance() => SetGithubSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetGithubSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetGithubSourceRequest>(
          SetGithubSourceRequest.$_createMessage);
  static SetGithubSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get owner => $_getSZ(2);
  @$pb.TagNumber(3)
  set owner($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwner() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwner() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get branch => $_getSZ(3);
  @$pb.TagNumber(4)
  set branch($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBranch() => $_has(3);
  @$pb.TagNumber(4)
  void clearBranch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get buildPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set buildPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBuildPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearBuildPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get providerId => $_getI64(5);
  @$pb.TagNumber(6)
  set providerId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasProviderId() => $_has(5);
  @$pb.TagNumber(6)
  void clearProviderId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get autoDeploy => $_getBF(6);
  @$pb.TagNumber(7)
  set autoDeploy($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAutoDeploy() => $_has(6);
  @$pb.TagNumber(7)
  void clearAutoDeploy() => $_clearField(7);
}

class SetGitlabSourceRequest extends $pb.GeneratedMessage {
  factory SetGitlabSourceRequest({
    $fixnum.Int64? id,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? buildPath,
    $core.String? pathNamespace,
    $fixnum.Int64? providerId,
    $fixnum.Int64? projectId,
  }) {
    final result = SetGitlabSourceRequest._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (pathNamespace != null) result.pathNamespace = pathNamespace;
    if (providerId != null) result.providerId = providerId;
    if (projectId != null) result.projectId = projectId;
    return result;
  }

  SetGitlabSourceRequest._();

  factory SetGitlabSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGitlabSourceRequest()..mergeFromBuffer(data, registry);
  factory SetGitlabSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGitlabSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetGitlabSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetGitlabSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'buildPath')
    ..aOS(6, _omitFieldNames ? '' : 'pathNamespace')
    ..aInt64(7, _omitFieldNames ? '' : 'providerId')
    ..aInt64(8, _omitFieldNames ? '' : 'projectId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGitlabSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGitlabSourceRequest copyWith(
          void Function(SetGitlabSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetGitlabSourceRequest))
          as SetGitlabSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetGitlabSourceRequest() / SetGitlabSourceRequest.new instead')
  static SetGitlabSourceRequest create() => SetGitlabSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetGitlabSourceRequest._();
  @$core.override
  SetGitlabSourceRequest createEmptyInstance() => SetGitlabSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetGitlabSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetGitlabSourceRequest>(
          SetGitlabSourceRequest.$_createMessage);
  static SetGitlabSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get owner => $_getSZ(2);
  @$pb.TagNumber(3)
  set owner($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwner() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwner() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get branch => $_getSZ(3);
  @$pb.TagNumber(4)
  set branch($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBranch() => $_has(3);
  @$pb.TagNumber(4)
  void clearBranch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get buildPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set buildPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBuildPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearBuildPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get pathNamespace => $_getSZ(5);
  @$pb.TagNumber(6)
  set pathNamespace($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPathNamespace() => $_has(5);
  @$pb.TagNumber(6)
  void clearPathNamespace() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get providerId => $_getI64(6);
  @$pb.TagNumber(7)
  set providerId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasProviderId() => $_has(6);
  @$pb.TagNumber(7)
  void clearProviderId() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get projectId => $_getI64(7);
  @$pb.TagNumber(8)
  set projectId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasProjectId() => $_has(7);
  @$pb.TagNumber(8)
  void clearProjectId() => $_clearField(8);
}

class SetBitbucketSourceRequest extends $pb.GeneratedMessage {
  factory SetBitbucketSourceRequest({
    $fixnum.Int64? id,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? buildPath,
    $core.String? repositorySlug,
    $fixnum.Int64? providerId,
  }) {
    final result = SetBitbucketSourceRequest._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (repositorySlug != null) result.repositorySlug = repositorySlug;
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  SetBitbucketSourceRequest._();

  factory SetBitbucketSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetBitbucketSourceRequest()..mergeFromBuffer(data, registry);
  factory SetBitbucketSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetBitbucketSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetBitbucketSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetBitbucketSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'buildPath')
    ..aOS(6, _omitFieldNames ? '' : 'repositorySlug')
    ..aInt64(7, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetBitbucketSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetBitbucketSourceRequest copyWith(
          void Function(SetBitbucketSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetBitbucketSourceRequest))
          as SetBitbucketSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetBitbucketSourceRequest() / SetBitbucketSourceRequest.new instead')
  static SetBitbucketSourceRequest create() => SetBitbucketSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SetBitbucketSourceRequest._();
  @$core.override
  SetBitbucketSourceRequest createEmptyInstance() =>
      SetBitbucketSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetBitbucketSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetBitbucketSourceRequest>(
          SetBitbucketSourceRequest.$_createMessage);
  static SetBitbucketSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get owner => $_getSZ(2);
  @$pb.TagNumber(3)
  set owner($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwner() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwner() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get branch => $_getSZ(3);
  @$pb.TagNumber(4)
  set branch($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBranch() => $_has(3);
  @$pb.TagNumber(4)
  void clearBranch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get buildPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set buildPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBuildPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearBuildPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get repositorySlug => $_getSZ(5);
  @$pb.TagNumber(6)
  set repositorySlug($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRepositorySlug() => $_has(5);
  @$pb.TagNumber(6)
  void clearRepositorySlug() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get providerId => $_getI64(6);
  @$pb.TagNumber(7)
  set providerId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasProviderId() => $_has(6);
  @$pb.TagNumber(7)
  void clearProviderId() => $_clearField(7);
}

class SetGiteaSourceRequest extends $pb.GeneratedMessage {
  factory SetGiteaSourceRequest({
    $fixnum.Int64? id,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? buildPath,
    $fixnum.Int64? providerId,
  }) {
    final result = SetGiteaSourceRequest._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  SetGiteaSourceRequest._();

  factory SetGiteaSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGiteaSourceRequest()..mergeFromBuffer(data, registry);
  factory SetGiteaSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetGiteaSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetGiteaSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetGiteaSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'buildPath')
    ..aInt64(6, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGiteaSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetGiteaSourceRequest copyWith(
          void Function(SetGiteaSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetGiteaSourceRequest))
          as SetGiteaSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetGiteaSourceRequest() / SetGiteaSourceRequest.new instead')
  static SetGiteaSourceRequest create() => SetGiteaSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetGiteaSourceRequest._();
  @$core.override
  SetGiteaSourceRequest createEmptyInstance() => SetGiteaSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetGiteaSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetGiteaSourceRequest>(
          SetGiteaSourceRequest.$_createMessage);
  static SetGiteaSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get owner => $_getSZ(2);
  @$pb.TagNumber(3)
  set owner($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwner() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwner() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get branch => $_getSZ(3);
  @$pb.TagNumber(4)
  set branch($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBranch() => $_has(3);
  @$pb.TagNumber(4)
  void clearBranch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get buildPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set buildPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBuildPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearBuildPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get providerId => $_getI64(5);
  @$pb.TagNumber(6)
  set providerId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasProviderId() => $_has(5);
  @$pb.TagNumber(6)
  void clearProviderId() => $_clearField(6);
}

class SetCustomGitSourceRequest extends $pb.GeneratedMessage {
  factory SetCustomGitSourceRequest({
    $fixnum.Int64? id,
    $core.String? url,
    $core.String? branch,
    $core.String? buildPath,
    $fixnum.Int64? sshKeyId,
  }) {
    final result = SetCustomGitSourceRequest._();
    if (id != null) result.id = id;
    if (url != null) result.url = url;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    return result;
  }

  SetCustomGitSourceRequest._();

  factory SetCustomGitSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetCustomGitSourceRequest()..mergeFromBuffer(data, registry);
  factory SetCustomGitSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetCustomGitSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetCustomGitSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetCustomGitSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'url')
    ..aOS(3, _omitFieldNames ? '' : 'branch')
    ..aOS(4, _omitFieldNames ? '' : 'buildPath')
    ..aInt64(5, _omitFieldNames ? '' : 'sshKeyId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetCustomGitSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetCustomGitSourceRequest copyWith(
          void Function(SetCustomGitSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetCustomGitSourceRequest))
          as SetCustomGitSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetCustomGitSourceRequest() / SetCustomGitSourceRequest.new instead')
  static SetCustomGitSourceRequest create() => SetCustomGitSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SetCustomGitSourceRequest._();
  @$core.override
  SetCustomGitSourceRequest createEmptyInstance() =>
      SetCustomGitSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetCustomGitSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetCustomGitSourceRequest>(
          SetCustomGitSourceRequest.$_createMessage);
  static SetCustomGitSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get url => $_getSZ(1);
  @$pb.TagNumber(2)
  set url($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get branch => $_getSZ(2);
  @$pb.TagNumber(3)
  set branch($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBranch() => $_has(2);
  @$pb.TagNumber(3)
  void clearBranch() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get buildPath => $_getSZ(3);
  @$pb.TagNumber(4)
  set buildPath($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBuildPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearBuildPath() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get sshKeyId => $_getI64(4);
  @$pb.TagNumber(5)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSshKeyId() => $_has(4);
  @$pb.TagNumber(5)
  void clearSshKeyId() => $_clearField(5);
}

class SetDockerSourceRequest extends $pb.GeneratedMessage {
  factory SetDockerSourceRequest({
    $fixnum.Int64? id,
    $core.String? image,
    $core.String? username,
    $core.String? password,
    $core.String? registryUrl,
  }) {
    final result = SetDockerSourceRequest._();
    if (id != null) result.id = id;
    if (image != null) result.image = image;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    if (registryUrl != null) result.registryUrl = registryUrl;
    return result;
  }

  SetDockerSourceRequest._();

  factory SetDockerSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDockerSourceRequest()..mergeFromBuffer(data, registry);
  factory SetDockerSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDockerSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetDockerSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetDockerSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'image')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'password')
    ..aOS(5, _omitFieldNames ? '' : 'registryUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDockerSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDockerSourceRequest copyWith(
          void Function(SetDockerSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetDockerSourceRequest))
          as SetDockerSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetDockerSourceRequest() / SetDockerSourceRequest.new instead')
  static SetDockerSourceRequest create() => SetDockerSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetDockerSourceRequest._();
  @$core.override
  SetDockerSourceRequest createEmptyInstance() => SetDockerSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetDockerSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetDockerSourceRequest>(
          SetDockerSourceRequest.$_createMessage);
  static SetDockerSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get image => $_getSZ(1);
  @$pb.TagNumber(2)
  set image($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasImage() => $_has(1);
  @$pb.TagNumber(2)
  void clearImage() => $_clearField(2);

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
}

class SetDropSourceRequest extends $pb.GeneratedMessage {
  factory SetDropSourceRequest({
    $fixnum.Int64? id,
    $core.String? buildPath,
  }) {
    final result = SetDropSourceRequest._();
    if (id != null) result.id = id;
    if (buildPath != null) result.buildPath = buildPath;
    return result;
  }

  SetDropSourceRequest._();

  factory SetDropSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDropSourceRequest()..mergeFromBuffer(data, registry);
  factory SetDropSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetDropSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetDropSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetDropSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'buildPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDropSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetDropSourceRequest copyWith(void Function(SetDropSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetDropSourceRequest))
          as SetDropSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetDropSourceRequest() / SetDropSourceRequest.new instead')
  static SetDropSourceRequest create() => SetDropSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetDropSourceRequest._();
  @$core.override
  SetDropSourceRequest createEmptyInstance() => SetDropSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetDropSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetDropSourceRequest>(
          SetDropSourceRequest.$_createMessage);
  static SetDropSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get buildPath => $_getSZ(1);
  @$pb.TagNumber(2)
  set buildPath($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBuildPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearBuildPath() => $_clearField(2);
}

class DropSourceHeader extends $pb.GeneratedMessage {
  factory DropSourceHeader({
    $fixnum.Int64? applicationId,
    $core.String? buildPath,
  }) {
    final result = DropSourceHeader._();
    if (applicationId != null) result.applicationId = applicationId;
    if (buildPath != null) result.buildPath = buildPath;
    return result;
  }

  DropSourceHeader._();

  factory DropSourceHeader.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DropSourceHeader()..mergeFromBuffer(data, registry);
  factory DropSourceHeader.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DropSourceHeader()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DropSourceHeader',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DropSourceHeader.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aOS(2, _omitFieldNames ? '' : 'buildPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DropSourceHeader clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DropSourceHeader copyWith(void Function(DropSourceHeader) updates) =>
      super.copyWith((message) => updates(message as DropSourceHeader))
          as DropSourceHeader;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DropSourceHeader() / DropSourceHeader.new instead')
  static DropSourceHeader create() => DropSourceHeader._();
  static $pb.GeneratedMessage $_createMessage() => DropSourceHeader._();
  @$core.override
  DropSourceHeader createEmptyInstance() => DropSourceHeader._();
  @$core.pragma('dart2js:noInline')
  static DropSourceHeader getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DropSourceHeader>(
          DropSourceHeader.$_createMessage);
  static DropSourceHeader? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get buildPath => $_getSZ(1);
  @$pb.TagNumber(2)
  set buildPath($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBuildPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearBuildPath() => $_clearField(2);
}

enum UploadSourceRequest_Payload { header, chunk, notSet }

class UploadSourceRequest extends $pb.GeneratedMessage {
  factory UploadSourceRequest({
    DropSourceHeader? header,
    $core.List<$core.int>? chunk,
  }) {
    final result = UploadSourceRequest._();
    if (header != null) result.header = header;
    if (chunk != null) result.chunk = chunk;
    return result;
  }

  UploadSourceRequest._();

  factory UploadSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UploadSourceRequest()..mergeFromBuffer(data, registry);
  factory UploadSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UploadSourceRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, UploadSourceRequest_Payload>
      _UploadSourceRequest_PayloadByTag = {
    1: UploadSourceRequest_Payload.header,
    2: UploadSourceRequest_Payload.chunk,
    0: UploadSourceRequest_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UploadSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UploadSourceRequest.$_createMessage)
    ..oo(0, [1, 2])
    ..aOM<DropSourceHeader>(1, _omitFieldNames ? '' : 'header',
        subBuilder: DropSourceHeader.$_createMessage)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'chunk', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UploadSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UploadSourceRequest copyWith(void Function(UploadSourceRequest) updates) =>
      super.copyWith((message) => updates(message as UploadSourceRequest))
          as UploadSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use UploadSourceRequest() / UploadSourceRequest.new instead')
  static UploadSourceRequest create() => UploadSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => UploadSourceRequest._();
  @$core.override
  UploadSourceRequest createEmptyInstance() => UploadSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static UploadSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UploadSourceRequest>(
          UploadSourceRequest.$_createMessage);
  static UploadSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  UploadSourceRequest_Payload whichPayload() =>
      _UploadSourceRequest_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  DropSourceHeader get header => $_getN(0);
  @$pb.TagNumber(1)
  set header(DropSourceHeader value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHeader() => $_has(0);
  @$pb.TagNumber(1)
  void clearHeader() => $_clearField(1);
  @$pb.TagNumber(1)
  DropSourceHeader ensureHeader() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get chunk => $_getN(1);
  @$pb.TagNumber(2)
  set chunk($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChunk() => $_has(1);
  @$pb.TagNumber(2)
  void clearChunk() => $_clearField(2);
}

class UpdateBuildConfigRequest extends $pb.GeneratedMessage {
  factory UpdateBuildConfigRequest({
    $fixnum.Int64? id,
    $core.String? buildArgs,
    $core.String? buildSecrets,
    $core.String? dockerfile,
    $core.String? contextPath,
    $core.String? buildStage,
    $core.String? publishDirectory,
    $core.bool? staticSpa,
    $core.bool? createEnvFile,
    $core.String? command,
    $core.String? args,
    $core.String? buildPath,
    $core.bool? cleanCache,
    $core.bool? enableSubmodules,
    $core.String? watchPaths,
    $core.String? railpackVersion,
    $core.String? herokuVersion,
    $core.bool? autoDeploy,
  }) {
    final result = UpdateBuildConfigRequest._();
    if (id != null) result.id = id;
    if (buildArgs != null) result.buildArgs = buildArgs;
    if (buildSecrets != null) result.buildSecrets = buildSecrets;
    if (dockerfile != null) result.dockerfile = dockerfile;
    if (contextPath != null) result.contextPath = contextPath;
    if (buildStage != null) result.buildStage = buildStage;
    if (publishDirectory != null) result.publishDirectory = publishDirectory;
    if (staticSpa != null) result.staticSpa = staticSpa;
    if (createEnvFile != null) result.createEnvFile = createEnvFile;
    if (command != null) result.command = command;
    if (args != null) result.args = args;
    if (buildPath != null) result.buildPath = buildPath;
    if (cleanCache != null) result.cleanCache = cleanCache;
    if (enableSubmodules != null) result.enableSubmodules = enableSubmodules;
    if (watchPaths != null) result.watchPaths = watchPaths;
    if (railpackVersion != null) result.railpackVersion = railpackVersion;
    if (herokuVersion != null) result.herokuVersion = herokuVersion;
    if (autoDeploy != null) result.autoDeploy = autoDeploy;
    return result;
  }

  UpdateBuildConfigRequest._();

  factory UpdateBuildConfigRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBuildConfigRequest()..mergeFromBuffer(data, registry);
  factory UpdateBuildConfigRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBuildConfigRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateBuildConfigRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateBuildConfigRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'buildArgs')
    ..aOS(3, _omitFieldNames ? '' : 'buildSecrets')
    ..aOS(4, _omitFieldNames ? '' : 'dockerfile')
    ..aOS(5, _omitFieldNames ? '' : 'contextPath')
    ..aOS(6, _omitFieldNames ? '' : 'buildStage')
    ..aOS(7, _omitFieldNames ? '' : 'publishDirectory')
    ..aOB(8, _omitFieldNames ? '' : 'staticSpa')
    ..aOB(9, _omitFieldNames ? '' : 'createEnvFile')
    ..aOS(10, _omitFieldNames ? '' : 'command')
    ..aOS(11, _omitFieldNames ? '' : 'args')
    ..aOS(12, _omitFieldNames ? '' : 'buildPath')
    ..aOB(13, _omitFieldNames ? '' : 'cleanCache')
    ..aOB(14, _omitFieldNames ? '' : 'enableSubmodules')
    ..aOS(15, _omitFieldNames ? '' : 'watchPaths')
    ..aOS(16, _omitFieldNames ? '' : 'railpackVersion')
    ..aOS(17, _omitFieldNames ? '' : 'herokuVersion')
    ..aOB(18, _omitFieldNames ? '' : 'autoDeploy')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBuildConfigRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBuildConfigRequest copyWith(
          void Function(UpdateBuildConfigRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateBuildConfigRequest))
          as UpdateBuildConfigRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateBuildConfigRequest() / UpdateBuildConfigRequest.new instead')
  static UpdateBuildConfigRequest create() => UpdateBuildConfigRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateBuildConfigRequest._();
  @$core.override
  UpdateBuildConfigRequest createEmptyInstance() =>
      UpdateBuildConfigRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateBuildConfigRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateBuildConfigRequest>(
          UpdateBuildConfigRequest.$_createMessage);
  static UpdateBuildConfigRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get buildArgs => $_getSZ(1);
  @$pb.TagNumber(2)
  set buildArgs($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBuildArgs() => $_has(1);
  @$pb.TagNumber(2)
  void clearBuildArgs() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get buildSecrets => $_getSZ(2);
  @$pb.TagNumber(3)
  set buildSecrets($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBuildSecrets() => $_has(2);
  @$pb.TagNumber(3)
  void clearBuildSecrets() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get dockerfile => $_getSZ(3);
  @$pb.TagNumber(4)
  set dockerfile($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDockerfile() => $_has(3);
  @$pb.TagNumber(4)
  void clearDockerfile() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get contextPath => $_getSZ(4);
  @$pb.TagNumber(5)
  set contextPath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContextPath() => $_has(4);
  @$pb.TagNumber(5)
  void clearContextPath() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get buildStage => $_getSZ(5);
  @$pb.TagNumber(6)
  set buildStage($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasBuildStage() => $_has(5);
  @$pb.TagNumber(6)
  void clearBuildStage() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get publishDirectory => $_getSZ(6);
  @$pb.TagNumber(7)
  set publishDirectory($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasPublishDirectory() => $_has(6);
  @$pb.TagNumber(7)
  void clearPublishDirectory() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get staticSpa => $_getBF(7);
  @$pb.TagNumber(8)
  set staticSpa($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasStaticSpa() => $_has(7);
  @$pb.TagNumber(8)
  void clearStaticSpa() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get createEnvFile => $_getBF(8);
  @$pb.TagNumber(9)
  set createEnvFile($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCreateEnvFile() => $_has(8);
  @$pb.TagNumber(9)
  void clearCreateEnvFile() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get command => $_getSZ(9);
  @$pb.TagNumber(10)
  set command($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCommand() => $_has(9);
  @$pb.TagNumber(10)
  void clearCommand() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get args => $_getSZ(10);
  @$pb.TagNumber(11)
  set args($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasArgs() => $_has(10);
  @$pb.TagNumber(11)
  void clearArgs() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get buildPath => $_getSZ(11);
  @$pb.TagNumber(12)
  set buildPath($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasBuildPath() => $_has(11);
  @$pb.TagNumber(12)
  void clearBuildPath() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get cleanCache => $_getBF(12);
  @$pb.TagNumber(13)
  set cleanCache($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasCleanCache() => $_has(12);
  @$pb.TagNumber(13)
  void clearCleanCache() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.bool get enableSubmodules => $_getBF(13);
  @$pb.TagNumber(14)
  set enableSubmodules($core.bool value) => $_setBool(13, value);
  @$pb.TagNumber(14)
  $core.bool hasEnableSubmodules() => $_has(13);
  @$pb.TagNumber(14)
  void clearEnableSubmodules() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get watchPaths => $_getSZ(14);
  @$pb.TagNumber(15)
  set watchPaths($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasWatchPaths() => $_has(14);
  @$pb.TagNumber(15)
  void clearWatchPaths() => $_clearField(15);

  /// Railpack/Heroku version pins and the autodeploy switch (service_git) — the build-settings and deploy-settings cards persist through here.
  @$pb.TagNumber(16)
  $core.String get railpackVersion => $_getSZ(15);
  @$pb.TagNumber(16)
  set railpackVersion($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasRailpackVersion() => $_has(15);
  @$pb.TagNumber(16)
  void clearRailpackVersion() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.String get herokuVersion => $_getSZ(16);
  @$pb.TagNumber(17)
  set herokuVersion($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasHerokuVersion() => $_has(16);
  @$pb.TagNumber(17)
  void clearHerokuVersion() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.bool get autoDeploy => $_getBF(17);
  @$pb.TagNumber(18)
  set autoDeploy($core.bool value) => $_setBool(17, value);
  @$pb.TagNumber(18)
  $core.bool hasAutoDeploy() => $_has(17);
  @$pb.TagNumber(18)
  void clearAutoDeploy() => $_clearField(18);
}

class UpdateResourceConfigRequest extends $pb.GeneratedMessage {
  factory UpdateResourceConfigRequest({
    $fixnum.Int64? id,
    $core.String? memoryReservation,
    $core.String? memoryLimit,
    $core.String? cpuReservation,
    $core.String? cpuLimit,
    $fixnum.Int64? replicas,
  }) {
    final result = UpdateResourceConfigRequest._();
    if (id != null) result.id = id;
    if (memoryReservation != null) result.memoryReservation = memoryReservation;
    if (memoryLimit != null) result.memoryLimit = memoryLimit;
    if (cpuReservation != null) result.cpuReservation = cpuReservation;
    if (cpuLimit != null) result.cpuLimit = cpuLimit;
    if (replicas != null) result.replicas = replicas;
    return result;
  }

  UpdateResourceConfigRequest._();

  factory UpdateResourceConfigRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateResourceConfigRequest()..mergeFromBuffer(data, registry);
  factory UpdateResourceConfigRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateResourceConfigRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateResourceConfigRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateResourceConfigRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'memoryReservation')
    ..aOS(3, _omitFieldNames ? '' : 'memoryLimit')
    ..aOS(4, _omitFieldNames ? '' : 'cpuReservation')
    ..aOS(5, _omitFieldNames ? '' : 'cpuLimit')
    ..aInt64(6, _omitFieldNames ? '' : 'replicas')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateResourceConfigRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateResourceConfigRequest copyWith(
          void Function(UpdateResourceConfigRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateResourceConfigRequest))
          as UpdateResourceConfigRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateResourceConfigRequest() / UpdateResourceConfigRequest.new instead')
  static UpdateResourceConfigRequest create() =>
      UpdateResourceConfigRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateResourceConfigRequest._();
  @$core.override
  UpdateResourceConfigRequest createEmptyInstance() =>
      UpdateResourceConfigRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateResourceConfigRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateResourceConfigRequest>(
          UpdateResourceConfigRequest.$_createMessage);
  static UpdateResourceConfigRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get memoryReservation => $_getSZ(1);
  @$pb.TagNumber(2)
  set memoryReservation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMemoryReservation() => $_has(1);
  @$pb.TagNumber(2)
  void clearMemoryReservation() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get memoryLimit => $_getSZ(2);
  @$pb.TagNumber(3)
  set memoryLimit($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMemoryLimit() => $_has(2);
  @$pb.TagNumber(3)
  void clearMemoryLimit() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get cpuReservation => $_getSZ(3);
  @$pb.TagNumber(4)
  set cpuReservation($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCpuReservation() => $_has(3);
  @$pb.TagNumber(4)
  void clearCpuReservation() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get cpuLimit => $_getSZ(4);
  @$pb.TagNumber(5)
  set cpuLimit($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCpuLimit() => $_has(4);
  @$pb.TagNumber(5)
  void clearCpuLimit() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get replicas => $_getI64(5);
  @$pb.TagNumber(6)
  set replicas($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasReplicas() => $_has(5);
  @$pb.TagNumber(6)
  void clearReplicas() => $_clearField(6);
}

class UpdatePreviewConfigRequest extends $pb.GeneratedMessage {
  factory UpdatePreviewConfigRequest({
    $fixnum.Int64? id,
    $core.String? previewEnv,
    $core.String? buildArgs,
    $core.String? buildSecrets,
    $core.String? labels,
    $core.String? wildcard,
    $fixnum.Int64? port,
    $core.bool? https,
    $core.String? path,
    $core.String? certificateType,
    $core.String? customCertResolver,
    $fixnum.Int64? limit,
    $core.bool? active,
    $core.bool? requireCollaboratorPermissions,
  }) {
    final result = UpdatePreviewConfigRequest._();
    if (id != null) result.id = id;
    if (previewEnv != null) result.previewEnv = previewEnv;
    if (buildArgs != null) result.buildArgs = buildArgs;
    if (buildSecrets != null) result.buildSecrets = buildSecrets;
    if (labels != null) result.labels = labels;
    if (wildcard != null) result.wildcard = wildcard;
    if (port != null) result.port = port;
    if (https != null) result.https = https;
    if (path != null) result.path = path;
    if (certificateType != null) result.certificateType = certificateType;
    if (customCertResolver != null)
      result.customCertResolver = customCertResolver;
    if (limit != null) result.limit = limit;
    if (active != null) result.active = active;
    if (requireCollaboratorPermissions != null)
      result.requireCollaboratorPermissions = requireCollaboratorPermissions;
    return result;
  }

  UpdatePreviewConfigRequest._();

  factory UpdatePreviewConfigRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdatePreviewConfigRequest()..mergeFromBuffer(data, registry);
  factory UpdatePreviewConfigRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdatePreviewConfigRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdatePreviewConfigRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdatePreviewConfigRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'previewEnv')
    ..aOS(3, _omitFieldNames ? '' : 'buildArgs')
    ..aOS(4, _omitFieldNames ? '' : 'buildSecrets')
    ..aOS(5, _omitFieldNames ? '' : 'labels')
    ..aOS(6, _omitFieldNames ? '' : 'wildcard')
    ..aInt64(7, _omitFieldNames ? '' : 'port')
    ..aOB(8, _omitFieldNames ? '' : 'https')
    ..aOS(9, _omitFieldNames ? '' : 'path')
    ..aOS(10, _omitFieldNames ? '' : 'certificateType')
    ..aOS(11, _omitFieldNames ? '' : 'customCertResolver')
    ..aInt64(12, _omitFieldNames ? '' : 'limit')
    ..aOB(13, _omitFieldNames ? '' : 'active')
    ..aOB(14, _omitFieldNames ? '' : 'requireCollaboratorPermissions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdatePreviewConfigRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdatePreviewConfigRequest copyWith(
          void Function(UpdatePreviewConfigRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdatePreviewConfigRequest))
          as UpdatePreviewConfigRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdatePreviewConfigRequest() / UpdatePreviewConfigRequest.new instead')
  static UpdatePreviewConfigRequest create() => UpdatePreviewConfigRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdatePreviewConfigRequest._();
  @$core.override
  UpdatePreviewConfigRequest createEmptyInstance() =>
      UpdatePreviewConfigRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdatePreviewConfigRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdatePreviewConfigRequest>(
          UpdatePreviewConfigRequest.$_createMessage);
  static UpdatePreviewConfigRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get previewEnv => $_getSZ(1);
  @$pb.TagNumber(2)
  set previewEnv($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPreviewEnv() => $_has(1);
  @$pb.TagNumber(2)
  void clearPreviewEnv() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get buildArgs => $_getSZ(2);
  @$pb.TagNumber(3)
  set buildArgs($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBuildArgs() => $_has(2);
  @$pb.TagNumber(3)
  void clearBuildArgs() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get buildSecrets => $_getSZ(3);
  @$pb.TagNumber(4)
  set buildSecrets($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBuildSecrets() => $_has(3);
  @$pb.TagNumber(4)
  void clearBuildSecrets() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get labels => $_getSZ(4);
  @$pb.TagNumber(5)
  set labels($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLabels() => $_has(4);
  @$pb.TagNumber(5)
  void clearLabels() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get wildcard => $_getSZ(5);
  @$pb.TagNumber(6)
  set wildcard($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasWildcard() => $_has(5);
  @$pb.TagNumber(6)
  void clearWildcard() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get port => $_getI64(6);
  @$pb.TagNumber(7)
  set port($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasPort() => $_has(6);
  @$pb.TagNumber(7)
  void clearPort() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.bool get https => $_getBF(7);
  @$pb.TagNumber(8)
  set https($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasHttps() => $_has(7);
  @$pb.TagNumber(8)
  void clearHttps() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get path => $_getSZ(8);
  @$pb.TagNumber(9)
  set path($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPath() => $_has(8);
  @$pb.TagNumber(9)
  void clearPath() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get certificateType => $_getSZ(9);
  @$pb.TagNumber(10)
  set certificateType($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCertificateType() => $_has(9);
  @$pb.TagNumber(10)
  void clearCertificateType() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get customCertResolver => $_getSZ(10);
  @$pb.TagNumber(11)
  set customCertResolver($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCustomCertResolver() => $_has(10);
  @$pb.TagNumber(11)
  void clearCustomCertResolver() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get limit => $_getI64(11);
  @$pb.TagNumber(12)
  set limit($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasLimit() => $_has(11);
  @$pb.TagNumber(12)
  void clearLimit() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get active => $_getBF(12);
  @$pb.TagNumber(13)
  set active($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasActive() => $_has(12);
  @$pb.TagNumber(13)
  void clearActive() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.bool get requireCollaboratorPermissions => $_getBF(13);
  @$pb.TagNumber(14)
  set requireCollaboratorPermissions($core.bool value) => $_setBool(13, value);
  @$pb.TagNumber(14)
  $core.bool hasRequireCollaboratorPermissions() => $_has(13);
  @$pb.TagNumber(14)
  void clearRequireCollaboratorPermissions() => $_clearField(14);
}

class ApplicationOperationRequest extends $pb.GeneratedMessage {
  factory ApplicationOperationRequest({
    $fixnum.Int64? id,
  }) {
    final result = ApplicationOperationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ApplicationOperationRequest._();

  factory ApplicationOperationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationOperationRequest()..mergeFromBuffer(data, registry);
  factory ApplicationOperationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationOperationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationOperationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationOperationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationOperationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationOperationRequest copyWith(
          void Function(ApplicationOperationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationOperationRequest))
          as ApplicationOperationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationOperationRequest() / ApplicationOperationRequest.new instead')
  static ApplicationOperationRequest create() =>
      ApplicationOperationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationOperationRequest._();
  @$core.override
  ApplicationOperationRequest createEmptyInstance() =>
      ApplicationOperationRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationOperationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationOperationRequest>(
          ApplicationOperationRequest.$_createMessage);
  static ApplicationOperationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ApplicationOperation extends $pb.GeneratedMessage {
  factory ApplicationOperation({
    Application? application,
    $core.String? operation,
  }) {
    final result = ApplicationOperation._();
    if (application != null) result.application = application;
    if (operation != null) result.operation = operation;
    return result;
  }

  ApplicationOperation._();

  factory ApplicationOperation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationOperation()..mergeFromBuffer(data, registry);
  factory ApplicationOperation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationOperation()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationOperation',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationOperation.$_createMessage)
    ..aOM<Application>(1, _omitFieldNames ? '' : 'application',
        subBuilder: Application.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'operation')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationOperation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationOperation copyWith(void Function(ApplicationOperation) updates) =>
      super.copyWith((message) => updates(message as ApplicationOperation))
          as ApplicationOperation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationOperation() / ApplicationOperation.new instead')
  static ApplicationOperation create() => ApplicationOperation._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationOperation._();
  @$core.override
  ApplicationOperation createEmptyInstance() => ApplicationOperation._();
  @$core.pragma('dart2js:noInline')
  static ApplicationOperation getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationOperation>(
          ApplicationOperation.$_createMessage);
  static ApplicationOperation? _defaultInstance;

  @$pb.TagNumber(1)
  Application get application => $_getN(0);
  @$pb.TagNumber(1)
  set application(Application value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasApplication() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplication() => $_clearField(1);
  @$pb.TagNumber(1)
  Application ensureApplication() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get operation => $_getSZ(1);
  @$pb.TagNumber(2)
  set operation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOperation() => $_has(1);
  @$pb.TagNumber(2)
  void clearOperation() => $_clearField(2);
}

class ApplicationWebhookToken extends $pb.GeneratedMessage {
  factory ApplicationWebhookToken({
    $core.String? token,
  }) {
    final result = ApplicationWebhookToken._();
    if (token != null) result.token = token;
    return result;
  }

  ApplicationWebhookToken._();

  factory ApplicationWebhookToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationWebhookToken()..mergeFromBuffer(data, registry);
  factory ApplicationWebhookToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationWebhookToken()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationWebhookToken',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationWebhookToken.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationWebhookToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationWebhookToken copyWith(
          void Function(ApplicationWebhookToken) updates) =>
      super.copyWith((message) => updates(message as ApplicationWebhookToken))
          as ApplicationWebhookToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationWebhookToken() / ApplicationWebhookToken.new instead')
  static ApplicationWebhookToken create() => ApplicationWebhookToken._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationWebhookToken._();
  @$core.override
  ApplicationWebhookToken createEmptyInstance() => ApplicationWebhookToken._();
  @$core.pragma('dart2js:noInline')
  static ApplicationWebhookToken getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationWebhookToken>(
          ApplicationWebhookToken.$_createMessage);
  static ApplicationWebhookToken? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);
}

class MoveApplicationRequest extends $pb.GeneratedMessage {
  factory MoveApplicationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? targetEnvironmentId,
  }) {
    final result = MoveApplicationRequest._();
    if (id != null) result.id = id;
    if (targetEnvironmentId != null)
      result.targetEnvironmentId = targetEnvironmentId;
    return result;
  }

  MoveApplicationRequest._();

  factory MoveApplicationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MoveApplicationRequest()..mergeFromBuffer(data, registry);
  factory MoveApplicationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MoveApplicationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MoveApplicationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MoveApplicationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'targetEnvironmentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MoveApplicationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MoveApplicationRequest copyWith(
          void Function(MoveApplicationRequest) updates) =>
      super.copyWith((message) => updates(message as MoveApplicationRequest))
          as MoveApplicationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MoveApplicationRequest() / MoveApplicationRequest.new instead')
  static MoveApplicationRequest create() => MoveApplicationRequest._();
  static $pb.GeneratedMessage $_createMessage() => MoveApplicationRequest._();
  @$core.override
  MoveApplicationRequest createEmptyInstance() => MoveApplicationRequest._();
  @$core.pragma('dart2js:noInline')
  static MoveApplicationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MoveApplicationRequest>(
          MoveApplicationRequest.$_createMessage);
  static MoveApplicationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get targetEnvironmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set targetEnvironmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetEnvironmentId() => $_clearField(2);
}

class ListRollbacksRequest extends $pb.GeneratedMessage {
  factory ListRollbacksRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ListRollbacksRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ListRollbacksRequest._();

  factory ListRollbacksRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRollbacksRequest()..mergeFromBuffer(data, registry);
  factory ListRollbacksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRollbacksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRollbacksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRollbacksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRollbacksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRollbacksRequest copyWith(void Function(ListRollbacksRequest) updates) =>
      super.copyWith((message) => updates(message as ListRollbacksRequest))
          as ListRollbacksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRollbacksRequest() / ListRollbacksRequest.new instead')
  static ListRollbacksRequest create() => ListRollbacksRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListRollbacksRequest._();
  @$core.override
  ListRollbacksRequest createEmptyInstance() => ListRollbacksRequest._();
  @$core.pragma('dart2js:noInline')
  static ListRollbacksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRollbacksRequest>(
          ListRollbacksRequest.$_createMessage);
  static ListRollbacksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ListRollbacksResponse extends $pb.GeneratedMessage {
  factory ListRollbacksResponse({
    $core.Iterable<Rollback>? rollbacks,
  }) {
    final result = ListRollbacksResponse._();
    if (rollbacks != null) result.rollbacks.addAll(rollbacks);
    return result;
  }

  ListRollbacksResponse._();

  factory ListRollbacksResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRollbacksResponse()..mergeFromBuffer(data, registry);
  factory ListRollbacksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListRollbacksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListRollbacksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListRollbacksResponse.$_createMessage)
    ..pPM<Rollback>(1, _omitFieldNames ? '' : 'rollbacks',
        subBuilder: Rollback.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRollbacksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListRollbacksResponse copyWith(
          void Function(ListRollbacksResponse) updates) =>
      super.copyWith((message) => updates(message as ListRollbacksResponse))
          as ListRollbacksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListRollbacksResponse() / ListRollbacksResponse.new instead')
  static ListRollbacksResponse create() => ListRollbacksResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListRollbacksResponse._();
  @$core.override
  ListRollbacksResponse createEmptyInstance() => ListRollbacksResponse._();
  @$core.pragma('dart2js:noInline')
  static ListRollbacksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListRollbacksResponse>(
          ListRollbacksResponse.$_createMessage);
  static ListRollbacksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Rollback> get rollbacks => $_getList(0);
}

class Rollback extends $pb.GeneratedMessage {
  factory Rollback({
    $fixnum.Int64? id,
    $fixnum.Int64? applicationId,
    $core.String? status,
    $core.String? commit,
    $fixnum.Int64? createdAt,
  }) {
    final result = Rollback._();
    if (id != null) result.id = id;
    if (applicationId != null) result.applicationId = applicationId;
    if (status != null) result.status = status;
    if (commit != null) result.commit = commit;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  Rollback._();

  factory Rollback.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Rollback()..mergeFromBuffer(data, registry);
  factory Rollback.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Rollback()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Rollback',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Rollback.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'applicationId')
    ..aOS(3, _omitFieldNames ? '' : 'status')
    ..aOS(4, _omitFieldNames ? '' : 'commit')
    ..aInt64(5, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Rollback clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Rollback copyWith(void Function(Rollback) updates) =>
      super.copyWith((message) => updates(message as Rollback)) as Rollback;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Rollback() / Rollback.new instead')
  static Rollback create() => Rollback._();
  static $pb.GeneratedMessage $_createMessage() => Rollback._();
  @$core.override
  Rollback createEmptyInstance() => Rollback._();
  @$core.pragma('dart2js:noInline')
  static Rollback getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Rollback>(Rollback.$_createMessage);
  static Rollback? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get applicationId => $_getI64(1);
  @$pb.TagNumber(2)
  set applicationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApplicationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearApplicationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get status => $_getSZ(2);
  @$pb.TagNumber(3)
  set status($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get commit => $_getSZ(3);
  @$pb.TagNumber(4)
  set commit($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCommit() => $_has(3);
  @$pb.TagNumber(4)
  void clearCommit() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get createdAt => $_getI64(4);
  @$pb.TagNumber(5)
  set createdAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);
}

class TriggerRollbackRequest extends $pb.GeneratedMessage {
  factory TriggerRollbackRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? rollbackId,
  }) {
    final result = TriggerRollbackRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (rollbackId != null) result.rollbackId = rollbackId;
    return result;
  }

  TriggerRollbackRequest._();

  factory TriggerRollbackRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TriggerRollbackRequest()..mergeFromBuffer(data, registry);
  factory TriggerRollbackRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TriggerRollbackRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TriggerRollbackRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TriggerRollbackRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'rollbackId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TriggerRollbackRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TriggerRollbackRequest copyWith(
          void Function(TriggerRollbackRequest) updates) =>
      super.copyWith((message) => updates(message as TriggerRollbackRequest))
          as TriggerRollbackRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TriggerRollbackRequest() / TriggerRollbackRequest.new instead')
  static TriggerRollbackRequest create() => TriggerRollbackRequest._();
  static $pb.GeneratedMessage $_createMessage() => TriggerRollbackRequest._();
  @$core.override
  TriggerRollbackRequest createEmptyInstance() => TriggerRollbackRequest._();
  @$core.pragma('dart2js:noInline')
  static TriggerRollbackRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TriggerRollbackRequest>(
          TriggerRollbackRequest.$_createMessage);
  static TriggerRollbackRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get rollbackId => $_getI64(1);
  @$pb.TagNumber(2)
  set rollbackId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRollbackId() => $_has(1);
  @$pb.TagNumber(2)
  void clearRollbackId() => $_clearField(2);
}

class RollbackTrigger extends $pb.GeneratedMessage {
  factory RollbackTrigger({
    $fixnum.Int64? rollbackId,
    $core.String? status,
  }) {
    final result = RollbackTrigger._();
    if (rollbackId != null) result.rollbackId = rollbackId;
    if (status != null) result.status = status;
    return result;
  }

  RollbackTrigger._();

  factory RollbackTrigger.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RollbackTrigger()..mergeFromBuffer(data, registry);
  factory RollbackTrigger.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RollbackTrigger()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RollbackTrigger',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RollbackTrigger.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'rollbackId')
    ..aOS(2, _omitFieldNames ? '' : 'status')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RollbackTrigger clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RollbackTrigger copyWith(void Function(RollbackTrigger) updates) =>
      super.copyWith((message) => updates(message as RollbackTrigger))
          as RollbackTrigger;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use RollbackTrigger() / RollbackTrigger.new instead')
  static RollbackTrigger create() => RollbackTrigger._();
  static $pb.GeneratedMessage $_createMessage() => RollbackTrigger._();
  @$core.override
  RollbackTrigger createEmptyInstance() => RollbackTrigger._();
  @$core.pragma('dart2js:noInline')
  static RollbackTrigger getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RollbackTrigger>(
          RollbackTrigger.$_createMessage);
  static RollbackTrigger? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get rollbackId => $_getI64(0);
  @$pb.TagNumber(1)
  set rollbackId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRollbackId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRollbackId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);
}

class DeleteRollbackRequest extends $pb.GeneratedMessage {
  factory DeleteRollbackRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? rollbackId,
  }) {
    final result = DeleteRollbackRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (rollbackId != null) result.rollbackId = rollbackId;
    return result;
  }

  DeleteRollbackRequest._();

  factory DeleteRollbackRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteRollbackRequest()..mergeFromBuffer(data, registry);
  factory DeleteRollbackRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteRollbackRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteRollbackRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteRollbackRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'rollbackId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteRollbackRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteRollbackRequest copyWith(
          void Function(DeleteRollbackRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteRollbackRequest))
          as DeleteRollbackRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteRollbackRequest() / DeleteRollbackRequest.new instead')
  static DeleteRollbackRequest create() => DeleteRollbackRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteRollbackRequest._();
  @$core.override
  DeleteRollbackRequest createEmptyInstance() => DeleteRollbackRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteRollbackRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteRollbackRequest>(
          DeleteRollbackRequest.$_createMessage);
  static DeleteRollbackRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get rollbackId => $_getI64(1);
  @$pb.TagNumber(2)
  set rollbackId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRollbackId() => $_has(1);
  @$pb.TagNumber(2)
  void clearRollbackId() => $_clearField(2);
}

class ApplicationDependencies extends $pb.GeneratedMessage {
  factory ApplicationDependencies({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationDependencies._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationDependencies._();

  factory ApplicationDependencies.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDependencies()..mergeFromBuffer(data, registry);
  factory ApplicationDependencies.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDependencies()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDependencies',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDependencies.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDependencies clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDependencies copyWith(
          void Function(ApplicationDependencies) updates) =>
      super.copyWith((message) => updates(message as ApplicationDependencies))
          as ApplicationDependencies;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDependencies() / ApplicationDependencies.new instead')
  static ApplicationDependencies create() => ApplicationDependencies._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationDependencies._();
  @$core.override
  ApplicationDependencies createEmptyInstance() => ApplicationDependencies._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDependencies getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDependencies>(
          ApplicationDependencies.$_createMessage);
  static ApplicationDependencies? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ResourceListRequest extends $pb.GeneratedMessage {
  factory ResourceListRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ResourceListRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ResourceListRequest._();

  factory ResourceListRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResourceListRequest()..mergeFromBuffer(data, registry);
  factory ResourceListRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResourceListRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ResourceListRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ResourceListRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceListRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceListRequest copyWith(void Function(ResourceListRequest) updates) =>
      super.copyWith((message) => updates(message as ResourceListRequest))
          as ResourceListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ResourceListRequest() / ResourceListRequest.new instead')
  static ResourceListRequest create() => ResourceListRequest._();
  static $pb.GeneratedMessage $_createMessage() => ResourceListRequest._();
  @$core.override
  ResourceListRequest createEmptyInstance() => ResourceListRequest._();
  @$core.pragma('dart2js:noInline')
  static ResourceListRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ResourceListRequest>(
          ResourceListRequest.$_createMessage);
  static ResourceListRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationPorts extends $pb.GeneratedMessage {
  factory ApplicationPorts({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationPorts._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationPorts._();

  factory ApplicationPorts.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPorts()..mergeFromBuffer(data, registry);
  factory ApplicationPorts.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPorts()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationPorts',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationPorts.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPorts clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPorts copyWith(void Function(ApplicationPorts) updates) =>
      super.copyWith((message) => updates(message as ApplicationPorts))
          as ApplicationPorts;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationPorts() / ApplicationPorts.new instead')
  static ApplicationPorts create() => ApplicationPorts._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationPorts._();
  @$core.override
  ApplicationPorts createEmptyInstance() => ApplicationPorts._();
  @$core.pragma('dart2js:noInline')
  static ApplicationPorts getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationPorts>(
          ApplicationPorts.$_createMessage);
  static ApplicationPorts? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationMounts extends $pb.GeneratedMessage {
  factory ApplicationMounts({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationMounts._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationMounts._();

  factory ApplicationMounts.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMounts()..mergeFromBuffer(data, registry);
  factory ApplicationMounts.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMounts()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationMounts',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationMounts.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMounts clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMounts copyWith(void Function(ApplicationMounts) updates) =>
      super.copyWith((message) => updates(message as ApplicationMounts))
          as ApplicationMounts;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationMounts() / ApplicationMounts.new instead')
  static ApplicationMounts create() => ApplicationMounts._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationMounts._();
  @$core.override
  ApplicationMounts createEmptyInstance() => ApplicationMounts._();
  @$core.pragma('dart2js:noInline')
  static ApplicationMounts getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationMounts>(
          ApplicationMounts.$_createMessage);
  static ApplicationMounts? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationNetworks extends $pb.GeneratedMessage {
  factory ApplicationNetworks({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationNetworks._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationNetworks._();

  factory ApplicationNetworks.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationNetworks()..mergeFromBuffer(data, registry);
  factory ApplicationNetworks.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationNetworks()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationNetworks',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationNetworks.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationNetworks clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationNetworks copyWith(void Function(ApplicationNetworks) updates) =>
      super.copyWith((message) => updates(message as ApplicationNetworks))
          as ApplicationNetworks;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ApplicationNetworks() / ApplicationNetworks.new instead')
  static ApplicationNetworks create() => ApplicationNetworks._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationNetworks._();
  @$core.override
  ApplicationNetworks createEmptyInstance() => ApplicationNetworks._();
  @$core.pragma('dart2js:noInline')
  static ApplicationNetworks getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationNetworks>(
          ApplicationNetworks.$_createMessage);
  static ApplicationNetworks? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationPatches extends $pb.GeneratedMessage {
  factory ApplicationPatches({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationPatches._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationPatches._();

  factory ApplicationPatches.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPatches()..mergeFromBuffer(data, registry);
  factory ApplicationPatches.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPatches()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationPatches',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationPatches.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPatches clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPatches copyWith(void Function(ApplicationPatches) updates) =>
      super.copyWith((message) => updates(message as ApplicationPatches))
          as ApplicationPatches;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationPatches() / ApplicationPatches.new instead')
  static ApplicationPatches create() => ApplicationPatches._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationPatches._();
  @$core.override
  ApplicationPatches createEmptyInstance() => ApplicationPatches._();
  @$core.pragma('dart2js:noInline')
  static ApplicationPatches getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationPatches>(
          ApplicationPatches.$_createMessage);
  static ApplicationPatches? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationMiddlewares extends $pb.GeneratedMessage {
  factory ApplicationMiddlewares({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationMiddlewares._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationMiddlewares._();

  factory ApplicationMiddlewares.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddlewares()..mergeFromBuffer(data, registry);
  factory ApplicationMiddlewares.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddlewares()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationMiddlewares',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationMiddlewares.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddlewares clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddlewares copyWith(
          void Function(ApplicationMiddlewares) updates) =>
      super.copyWith((message) => updates(message as ApplicationMiddlewares))
          as ApplicationMiddlewares;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationMiddlewares() / ApplicationMiddlewares.new instead')
  static ApplicationMiddlewares create() => ApplicationMiddlewares._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationMiddlewares._();
  @$core.override
  ApplicationMiddlewares createEmptyInstance() => ApplicationMiddlewares._();
  @$core.pragma('dart2js:noInline')
  static ApplicationMiddlewares getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationMiddlewares>(
          ApplicationMiddlewares.$_createMessage);
  static ApplicationMiddlewares? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationRedirects extends $pb.GeneratedMessage {
  factory ApplicationRedirects({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationRedirects._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationRedirects._();

  factory ApplicationRedirects.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationRedirects()..mergeFromBuffer(data, registry);
  factory ApplicationRedirects.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationRedirects()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationRedirects',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationRedirects.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationRedirects clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationRedirects copyWith(void Function(ApplicationRedirects) updates) =>
      super.copyWith((message) => updates(message as ApplicationRedirects))
          as ApplicationRedirects;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationRedirects() / ApplicationRedirects.new instead')
  static ApplicationRedirects create() => ApplicationRedirects._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationRedirects._();
  @$core.override
  ApplicationRedirects createEmptyInstance() => ApplicationRedirects._();
  @$core.pragma('dart2js:noInline')
  static ApplicationRedirects getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationRedirects>(
          ApplicationRedirects.$_createMessage);
  static ApplicationRedirects? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationSecurity extends $pb.GeneratedMessage {
  factory ApplicationSecurity({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ApplicationSecurity._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ApplicationSecurity._();

  factory ApplicationSecurity.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationSecurity()..mergeFromBuffer(data, registry);
  factory ApplicationSecurity.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationSecurity()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationSecurity',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationSecurity.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationSecurity clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationSecurity copyWith(void Function(ApplicationSecurity) updates) =>
      super.copyWith((message) => updates(message as ApplicationSecurity))
          as ApplicationSecurity;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ApplicationSecurity() / ApplicationSecurity.new instead')
  static ApplicationSecurity create() => ApplicationSecurity._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationSecurity._();
  @$core.override
  ApplicationSecurity createEmptyInstance() => ApplicationSecurity._();
  @$core.pragma('dart2js:noInline')
  static ApplicationSecurity getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationSecurity>(
          ApplicationSecurity.$_createMessage);
  static ApplicationSecurity? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class ApplicationPort extends $pb.GeneratedMessage {
  factory ApplicationPort({
    $fixnum.Int64? id,
    $fixnum.Int64? publishedPort,
    $fixnum.Int64? targetPort,
    ApplicationPortProtocol? protocol,
    ApplicationPortPublishMode? publishMode,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? createdAt,
  }) {
    final result = ApplicationPort._();
    if (id != null) result.id = id;
    if (publishedPort != null) result.publishedPort = publishedPort;
    if (targetPort != null) result.targetPort = targetPort;
    if (protocol != null) result.protocol = protocol;
    if (publishMode != null) result.publishMode = publishMode;
    if (applicationId != null) result.applicationId = applicationId;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  ApplicationPort._();

  factory ApplicationPort.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPort()..mergeFromBuffer(data, registry);
  factory ApplicationPort.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPort()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationPort',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationPort.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'publishedPort')
    ..aInt64(3, _omitFieldNames ? '' : 'targetPort')
    ..aE<ApplicationPortProtocol>(4, _omitFieldNames ? '' : 'protocol',
        enumValues: ApplicationPortProtocol.values)
    ..aE<ApplicationPortPublishMode>(5, _omitFieldNames ? '' : 'publishMode',
        enumValues: ApplicationPortPublishMode.values)
    ..aInt64(6, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPort clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPort copyWith(void Function(ApplicationPort) updates) =>
      super.copyWith((message) => updates(message as ApplicationPort))
          as ApplicationPort;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationPort() / ApplicationPort.new instead')
  static ApplicationPort create() => ApplicationPort._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationPort._();
  @$core.override
  ApplicationPort createEmptyInstance() => ApplicationPort._();
  @$core.pragma('dart2js:noInline')
  static ApplicationPort getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationPort>(
          ApplicationPort.$_createMessage);
  static ApplicationPort? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get publishedPort => $_getI64(1);
  @$pb.TagNumber(2)
  set publishedPort($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPublishedPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPublishedPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get targetPort => $_getI64(2);
  @$pb.TagNumber(3)
  set targetPort($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTargetPort() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetPort() => $_clearField(3);

  @$pb.TagNumber(4)
  ApplicationPortProtocol get protocol => $_getN(3);
  @$pb.TagNumber(4)
  set protocol(ApplicationPortProtocol value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasProtocol() => $_has(3);
  @$pb.TagNumber(4)
  void clearProtocol() => $_clearField(4);

  @$pb.TagNumber(5)
  ApplicationPortPublishMode get publishMode => $_getN(4);
  @$pb.TagNumber(5)
  set publishMode(ApplicationPortPublishMode value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasPublishMode() => $_has(4);
  @$pb.TagNumber(5)
  void clearPublishMode() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get applicationId => $_getI64(5);
  @$pb.TagNumber(6)
  set applicationId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApplicationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearApplicationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);
}

class ApplicationMount extends $pb.GeneratedMessage {
  factory ApplicationMount({
    $fixnum.Int64? id,
    ApplicationMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ApplicationMount._();
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    if (applicationId != null) result.applicationId = applicationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ApplicationMount._();

  factory ApplicationMount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMount()..mergeFromBuffer(data, registry);
  factory ApplicationMount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMount()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationMount',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationMount.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<ApplicationMountType>(2, _omitFieldNames ? '' : 'mountType',
        enumValues: ApplicationMountType.values)
    ..aOS(3, _omitFieldNames ? '' : 'hostPath')
    ..aOS(4, _omitFieldNames ? '' : 'volumeName')
    ..aOS(5, _omitFieldNames ? '' : 'filePath')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..aOS(7, _omitFieldNames ? '' : 'mountPath')
    ..aInt64(8, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMount copyWith(void Function(ApplicationMount) updates) =>
      super.copyWith((message) => updates(message as ApplicationMount))
          as ApplicationMount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationMount() / ApplicationMount.new instead')
  static ApplicationMount create() => ApplicationMount._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationMount._();
  @$core.override
  ApplicationMount createEmptyInstance() => ApplicationMount._();
  @$core.pragma('dart2js:noInline')
  static ApplicationMount getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationMount>(
          ApplicationMount.$_createMessage);
  static ApplicationMount? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ApplicationMountType get mountType => $_getN(1);
  @$pb.TagNumber(2)
  set mountType(ApplicationMountType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasMountType() => $_has(1);
  @$pb.TagNumber(2)
  void clearMountType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get hostPath => $_getSZ(2);
  @$pb.TagNumber(3)
  set hostPath($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHostPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearHostPath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get volumeName => $_getSZ(3);
  @$pb.TagNumber(4)
  set volumeName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasVolumeName() => $_has(3);
  @$pb.TagNumber(4)
  void clearVolumeName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get filePath => $_getSZ(4);
  @$pb.TagNumber(5)
  set filePath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFilePath() => $_has(4);
  @$pb.TagNumber(5)
  void clearFilePath() => $_clearField(5);

  /// Inline file content for FILE mounts.
  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get mountPath => $_getSZ(6);
  @$pb.TagNumber(7)
  set mountPath($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasMountPath() => $_has(6);
  @$pb.TagNumber(7)
  void clearMountPath() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get applicationId => $_getI64(7);
  @$pb.TagNumber(8)
  set applicationId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasApplicationId() => $_has(7);
  @$pb.TagNumber(8)
  void clearApplicationId() => $_clearField(8);

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

class ApplicationPatch extends $pb.GeneratedMessage {
  factory ApplicationPatch({
    $fixnum.Int64? id,
    ApplicationPatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ApplicationPatch._();
    if (id != null) result.id = id;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    if (applicationId != null) result.applicationId = applicationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ApplicationPatch._();

  factory ApplicationPatch.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPatch()..mergeFromBuffer(data, registry);
  factory ApplicationPatch.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationPatch()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationPatch',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationPatch.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<ApplicationPatchType>(2, _omitFieldNames ? '' : 'patchType',
        enumValues: ApplicationPatchType.values)
    ..aOS(3, _omitFieldNames ? '' : 'filePath')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..aInt64(6, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPatch clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationPatch copyWith(void Function(ApplicationPatch) updates) =>
      super.copyWith((message) => updates(message as ApplicationPatch))
          as ApplicationPatch;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ApplicationPatch() / ApplicationPatch.new instead')
  static ApplicationPatch create() => ApplicationPatch._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationPatch._();
  @$core.override
  ApplicationPatch createEmptyInstance() => ApplicationPatch._();
  @$core.pragma('dart2js:noInline')
  static ApplicationPatch getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ApplicationPatch>(
          ApplicationPatch.$_createMessage);
  static ApplicationPatch? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ApplicationPatchType get patchType => $_getN(1);
  @$pb.TagNumber(2)
  set patchType(ApplicationPatchType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPatchType() => $_has(1);
  @$pb.TagNumber(2)
  void clearPatchType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get filePath => $_getSZ(2);
  @$pb.TagNumber(3)
  set filePath($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFilePath() => $_has(2);
  @$pb.TagNumber(3)
  void clearFilePath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get enabled => $_getBF(3);
  @$pb.TagNumber(4)
  set enabled($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnabled() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnabled() => $_clearField(4);

  /// Patch file body (raw text); cleared for DELETE patches.
  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get applicationId => $_getI64(5);
  @$pb.TagNumber(6)
  set applicationId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApplicationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearApplicationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get updatedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set updatedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasUpdatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearUpdatedAt() => $_clearField(8);
}

class ApplicationMiddlewareHeader extends $pb.GeneratedMessage {
  factory ApplicationMiddlewareHeader({
    $core.String? name,
    $core.String? value,
  }) {
    final result = ApplicationMiddlewareHeader._();
    if (name != null) result.name = name;
    if (value != null) result.value = value;
    return result;
  }

  ApplicationMiddlewareHeader._();

  factory ApplicationMiddlewareHeader.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddlewareHeader()..mergeFromBuffer(data, registry);
  factory ApplicationMiddlewareHeader.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddlewareHeader()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationMiddlewareHeader',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationMiddlewareHeader.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'value')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddlewareHeader clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddlewareHeader copyWith(
          void Function(ApplicationMiddlewareHeader) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationMiddlewareHeader))
          as ApplicationMiddlewareHeader;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationMiddlewareHeader() / ApplicationMiddlewareHeader.new instead')
  static ApplicationMiddlewareHeader create() =>
      ApplicationMiddlewareHeader._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationMiddlewareHeader._();
  @$core.override
  ApplicationMiddlewareHeader createEmptyInstance() =>
      ApplicationMiddlewareHeader._();
  @$core.pragma('dart2js:noInline')
  static ApplicationMiddlewareHeader getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationMiddlewareHeader>(
          ApplicationMiddlewareHeader.$_createMessage);
  static ApplicationMiddlewareHeader? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get value => $_getSZ(1);
  @$pb.TagNumber(2)
  set value($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearValue() => $_clearField(2);
}

class ApplicationMiddleware extends $pb.GeneratedMessage {
  factory ApplicationMiddleware({
    $fixnum.Int64? id,
    $fixnum.Int64? applicationId,
    $core.String? name,
    ApplicationMiddlewareType? middlewareType,
    $core.bool? enabled,
    $core.Iterable<ApplicationMiddlewareHeader>? headers,
    $fixnum.Int64? average,
    $fixnum.Int64? burst,
    $core.Iterable<$core.String>? sourceRanges,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ApplicationMiddleware._();
    if (id != null) result.id = id;
    if (applicationId != null) result.applicationId = applicationId;
    if (name != null) result.name = name;
    if (middlewareType != null) result.middlewareType = middlewareType;
    if (enabled != null) result.enabled = enabled;
    if (headers != null) result.headers.addAll(headers);
    if (average != null) result.average = average;
    if (burst != null) result.burst = burst;
    if (sourceRanges != null) result.sourceRanges.addAll(sourceRanges);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ApplicationMiddleware._();

  factory ApplicationMiddleware.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddleware()..mergeFromBuffer(data, registry);
  factory ApplicationMiddleware.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationMiddleware()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationMiddleware',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationMiddleware.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'applicationId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aE<ApplicationMiddlewareType>(4, _omitFieldNames ? '' : 'middlewareType',
        enumValues: ApplicationMiddlewareType.values)
    ..aOB(5, _omitFieldNames ? '' : 'enabled')
    ..pPM<ApplicationMiddlewareHeader>(6, _omitFieldNames ? '' : 'headers',
        subBuilder: ApplicationMiddlewareHeader.$_createMessage)
    ..aInt64(7, _omitFieldNames ? '' : 'average')
    ..aInt64(8, _omitFieldNames ? '' : 'burst')
    ..pPS(9, _omitFieldNames ? '' : 'sourceRanges')
    ..aInt64(10, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(11, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddleware clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationMiddleware copyWith(
          void Function(ApplicationMiddleware) updates) =>
      super.copyWith((message) => updates(message as ApplicationMiddleware))
          as ApplicationMiddleware;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationMiddleware() / ApplicationMiddleware.new instead')
  static ApplicationMiddleware create() => ApplicationMiddleware._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationMiddleware._();
  @$core.override
  ApplicationMiddleware createEmptyInstance() => ApplicationMiddleware._();
  @$core.pragma('dart2js:noInline')
  static ApplicationMiddleware getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationMiddleware>(
          ApplicationMiddleware.$_createMessage);
  static ApplicationMiddleware? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get applicationId => $_getI64(1);
  @$pb.TagNumber(2)
  set applicationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApplicationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearApplicationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  ApplicationMiddlewareType get middlewareType => $_getN(3);
  @$pb.TagNumber(4)
  set middlewareType(ApplicationMiddlewareType value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMiddlewareType() => $_has(3);
  @$pb.TagNumber(4)
  void clearMiddlewareType() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get enabled => $_getBF(4);
  @$pb.TagNumber(5)
  set enabled($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEnabled() => $_has(4);
  @$pb.TagNumber(5)
  void clearEnabled() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<ApplicationMiddlewareHeader> get headers => $_getList(5);

  @$pb.TagNumber(7)
  $fixnum.Int64 get average => $_getI64(6);
  @$pb.TagNumber(7)
  set average($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAverage() => $_has(6);
  @$pb.TagNumber(7)
  void clearAverage() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get burst => $_getI64(7);
  @$pb.TagNumber(8)
  set burst($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBurst() => $_has(7);
  @$pb.TagNumber(8)
  void clearBurst() => $_clearField(8);

  @$pb.TagNumber(9)
  $pb.PbList<$core.String> get sourceRanges => $_getList(8);

  @$pb.TagNumber(10)
  $fixnum.Int64 get createdAt => $_getI64(9);
  @$pb.TagNumber(10)
  set createdAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCreatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearCreatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get updatedAt => $_getI64(10);
  @$pb.TagNumber(11)
  set updatedAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUpdatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearUpdatedAt() => $_clearField(11);
}

class ApplicationRedirect extends $pb.GeneratedMessage {
  factory ApplicationRedirect({
    $fixnum.Int64? id,
    $core.String? regex,
    $core.String? replacement,
    $core.bool? permanent,
    $fixnum.Int64? uniqueConfigKey,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ApplicationRedirect._();
    if (id != null) result.id = id;
    if (regex != null) result.regex = regex;
    if (replacement != null) result.replacement = replacement;
    if (permanent != null) result.permanent = permanent;
    if (uniqueConfigKey != null) result.uniqueConfigKey = uniqueConfigKey;
    if (applicationId != null) result.applicationId = applicationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ApplicationRedirect._();

  factory ApplicationRedirect.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationRedirect()..mergeFromBuffer(data, registry);
  factory ApplicationRedirect.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationRedirect()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationRedirect',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationRedirect.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'regex')
    ..aOS(3, _omitFieldNames ? '' : 'replacement')
    ..aOB(4, _omitFieldNames ? '' : 'permanent')
    ..aInt64(5, _omitFieldNames ? '' : 'uniqueConfigKey')
    ..aInt64(6, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationRedirect clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationRedirect copyWith(void Function(ApplicationRedirect) updates) =>
      super.copyWith((message) => updates(message as ApplicationRedirect))
          as ApplicationRedirect;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ApplicationRedirect() / ApplicationRedirect.new instead')
  static ApplicationRedirect create() => ApplicationRedirect._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationRedirect._();
  @$core.override
  ApplicationRedirect createEmptyInstance() => ApplicationRedirect._();
  @$core.pragma('dart2js:noInline')
  static ApplicationRedirect getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationRedirect>(
          ApplicationRedirect.$_createMessage);
  static ApplicationRedirect? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get regex => $_getSZ(1);
  @$pb.TagNumber(2)
  set regex($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRegex() => $_has(1);
  @$pb.TagNumber(2)
  void clearRegex() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get replacement => $_getSZ(2);
  @$pb.TagNumber(3)
  set replacement($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasReplacement() => $_has(2);
  @$pb.TagNumber(3)
  void clearReplacement() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get permanent => $_getBF(3);
  @$pb.TagNumber(4)
  set permanent($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPermanent() => $_has(3);
  @$pb.TagNumber(4)
  void clearPermanent() => $_clearField(4);

  /// Opaque Traefik routing-config key.
  @$pb.TagNumber(5)
  $fixnum.Int64 get uniqueConfigKey => $_getI64(4);
  @$pb.TagNumber(5)
  set uniqueConfigKey($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUniqueConfigKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearUniqueConfigKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get applicationId => $_getI64(5);
  @$pb.TagNumber(6)
  set applicationId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApplicationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearApplicationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get updatedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set updatedAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasUpdatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearUpdatedAt() => $_clearField(8);
}

class ApplicationSecurityRule extends $pb.GeneratedMessage {
  factory ApplicationSecurityRule({
    $fixnum.Int64? id,
    $core.String? username,
    $core.bool? hasPassword,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? createdAt,
  }) {
    final result = ApplicationSecurityRule._();
    if (id != null) result.id = id;
    if (username != null) result.username = username;
    if (hasPassword != null) result.hasPassword = hasPassword;
    if (applicationId != null) result.applicationId = applicationId;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  ApplicationSecurityRule._();

  factory ApplicationSecurityRule.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationSecurityRule()..mergeFromBuffer(data, registry);
  factory ApplicationSecurityRule.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationSecurityRule()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationSecurityRule',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationSecurityRule.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'username')
    ..aOB(3, _omitFieldNames ? '' : 'hasPassword')
    ..aInt64(4, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(5, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationSecurityRule clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationSecurityRule copyWith(
          void Function(ApplicationSecurityRule) updates) =>
      super.copyWith((message) => updates(message as ApplicationSecurityRule))
          as ApplicationSecurityRule;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationSecurityRule() / ApplicationSecurityRule.new instead')
  static ApplicationSecurityRule create() => ApplicationSecurityRule._();
  static $pb.GeneratedMessage $_createMessage() => ApplicationSecurityRule._();
  @$core.override
  ApplicationSecurityRule createEmptyInstance() => ApplicationSecurityRule._();
  @$core.pragma('dart2js:noInline')
  static ApplicationSecurityRule getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationSecurityRule>(
          ApplicationSecurityRule.$_createMessage);
  static ApplicationSecurityRule? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get username => $_getSZ(1);
  @$pb.TagNumber(2)
  set username($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUsername() => $_has(1);
  @$pb.TagNumber(2)
  void clearUsername() => $_clearField(2);

  /// Password material is never echoed back; presence only.
  @$pb.TagNumber(3)
  $core.bool get hasPassword => $_getBF(2);
  @$pb.TagNumber(3)
  set hasPassword($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHasPassword() => $_has(2);
  @$pb.TagNumber(3)
  void clearHasPassword() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get applicationId => $_getI64(3);
  @$pb.TagNumber(4)
  set applicationId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasApplicationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearApplicationId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get createdAt => $_getI64(4);
  @$pb.TagNumber(5)
  set createdAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);
}

class ApplicationListPortsRequest extends $pb.GeneratedMessage {
  factory ApplicationListPortsRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationListPortsRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationListPortsRequest._();

  factory ApplicationListPortsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPortsRequest()..mergeFromBuffer(data, registry);
  factory ApplicationListPortsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPortsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListPortsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListPortsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPortsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPortsRequest copyWith(
          void Function(ApplicationListPortsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListPortsRequest))
          as ApplicationListPortsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListPortsRequest() / ApplicationListPortsRequest.new instead')
  static ApplicationListPortsRequest create() =>
      ApplicationListPortsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListPortsRequest._();
  @$core.override
  ApplicationListPortsRequest createEmptyInstance() =>
      ApplicationListPortsRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListPortsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListPortsRequest>(
          ApplicationListPortsRequest.$_createMessage);
  static ApplicationListPortsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationListPortsResponse extends $pb.GeneratedMessage {
  factory ApplicationListPortsResponse({
    $core.Iterable<ApplicationPort>? ports,
  }) {
    final result = ApplicationListPortsResponse._();
    if (ports != null) result.ports.addAll(ports);
    return result;
  }

  ApplicationListPortsResponse._();

  factory ApplicationListPortsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPortsResponse()..mergeFromBuffer(data, registry);
  factory ApplicationListPortsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPortsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListPortsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListPortsResponse.$_createMessage)
    ..pPM<ApplicationPort>(1, _omitFieldNames ? '' : 'ports',
        subBuilder: ApplicationPort.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPortsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPortsResponse copyWith(
          void Function(ApplicationListPortsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListPortsResponse))
          as ApplicationListPortsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListPortsResponse() / ApplicationListPortsResponse.new instead')
  static ApplicationListPortsResponse create() =>
      ApplicationListPortsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListPortsResponse._();
  @$core.override
  ApplicationListPortsResponse createEmptyInstance() =>
      ApplicationListPortsResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListPortsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListPortsResponse>(
          ApplicationListPortsResponse.$_createMessage);
  static ApplicationListPortsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationPort> get ports => $_getList(0);
}

class ApplicationCreatePortRequest extends $pb.GeneratedMessage {
  factory ApplicationCreatePortRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? publishedPort,
    $fixnum.Int64? targetPort,
    ApplicationPortProtocol? protocol,
    ApplicationPortPublishMode? publishMode,
  }) {
    final result = ApplicationCreatePortRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (publishedPort != null) result.publishedPort = publishedPort;
    if (targetPort != null) result.targetPort = targetPort;
    if (protocol != null) result.protocol = protocol;
    if (publishMode != null) result.publishMode = publishMode;
    return result;
  }

  ApplicationCreatePortRequest._();

  factory ApplicationCreatePortRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePortRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreatePortRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePortRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreatePortRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreatePortRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'publishedPort')
    ..aInt64(3, _omitFieldNames ? '' : 'targetPort')
    ..aE<ApplicationPortProtocol>(4, _omitFieldNames ? '' : 'protocol',
        enumValues: ApplicationPortProtocol.values)
    ..aE<ApplicationPortPublishMode>(5, _omitFieldNames ? '' : 'publishMode',
        enumValues: ApplicationPortPublishMode.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePortRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePortRequest copyWith(
          void Function(ApplicationCreatePortRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreatePortRequest))
          as ApplicationCreatePortRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreatePortRequest() / ApplicationCreatePortRequest.new instead')
  static ApplicationCreatePortRequest create() =>
      ApplicationCreatePortRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreatePortRequest._();
  @$core.override
  ApplicationCreatePortRequest createEmptyInstance() =>
      ApplicationCreatePortRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreatePortRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreatePortRequest>(
          ApplicationCreatePortRequest.$_createMessage);
  static ApplicationCreatePortRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get publishedPort => $_getI64(1);
  @$pb.TagNumber(2)
  set publishedPort($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPublishedPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPublishedPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get targetPort => $_getI64(2);
  @$pb.TagNumber(3)
  set targetPort($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTargetPort() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetPort() => $_clearField(3);

  @$pb.TagNumber(4)
  ApplicationPortProtocol get protocol => $_getN(3);
  @$pb.TagNumber(4)
  set protocol(ApplicationPortProtocol value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasProtocol() => $_has(3);
  @$pb.TagNumber(4)
  void clearProtocol() => $_clearField(4);

  @$pb.TagNumber(5)
  ApplicationPortPublishMode get publishMode => $_getN(4);
  @$pb.TagNumber(5)
  set publishMode(ApplicationPortPublishMode value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasPublishMode() => $_has(4);
  @$pb.TagNumber(5)
  void clearPublishMode() => $_clearField(5);
}

class ApplicationCreatePortResponse extends $pb.GeneratedMessage {
  factory ApplicationCreatePortResponse({
    ApplicationPort? port,
  }) {
    final result = ApplicationCreatePortResponse._();
    if (port != null) result.port = port;
    return result;
  }

  ApplicationCreatePortResponse._();

  factory ApplicationCreatePortResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePortResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreatePortResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePortResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreatePortResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreatePortResponse.$_createMessage)
    ..aOM<ApplicationPort>(1, _omitFieldNames ? '' : 'port',
        subBuilder: ApplicationPort.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePortResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePortResponse copyWith(
          void Function(ApplicationCreatePortResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreatePortResponse))
          as ApplicationCreatePortResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreatePortResponse() / ApplicationCreatePortResponse.new instead')
  static ApplicationCreatePortResponse create() =>
      ApplicationCreatePortResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreatePortResponse._();
  @$core.override
  ApplicationCreatePortResponse createEmptyInstance() =>
      ApplicationCreatePortResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreatePortResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreatePortResponse>(
          ApplicationCreatePortResponse.$_createMessage);
  static ApplicationCreatePortResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationPort get port => $_getN(0);
  @$pb.TagNumber(1)
  set port(ApplicationPort value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPort() => $_has(0);
  @$pb.TagNumber(1)
  void clearPort() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationPort ensurePort() => $_ensure(0);
}

class ApplicationUpdatePortRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdatePortRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    $fixnum.Int64? publishedPort,
    $fixnum.Int64? targetPort,
    ApplicationPortProtocol? protocol,
    ApplicationPortPublishMode? publishMode,
  }) {
    final result = ApplicationUpdatePortRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (publishedPort != null) result.publishedPort = publishedPort;
    if (targetPort != null) result.targetPort = targetPort;
    if (protocol != null) result.protocol = protocol;
    if (publishMode != null) result.publishMode = publishMode;
    return result;
  }

  ApplicationUpdatePortRequest._();

  factory ApplicationUpdatePortRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePortRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdatePortRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePortRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdatePortRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdatePortRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aInt64(3, _omitFieldNames ? '' : 'publishedPort')
    ..aInt64(4, _omitFieldNames ? '' : 'targetPort')
    ..aE<ApplicationPortProtocol>(5, _omitFieldNames ? '' : 'protocol',
        enumValues: ApplicationPortProtocol.values)
    ..aE<ApplicationPortPublishMode>(6, _omitFieldNames ? '' : 'publishMode',
        enumValues: ApplicationPortPublishMode.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePortRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePortRequest copyWith(
          void Function(ApplicationUpdatePortRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdatePortRequest))
          as ApplicationUpdatePortRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdatePortRequest() / ApplicationUpdatePortRequest.new instead')
  static ApplicationUpdatePortRequest create() =>
      ApplicationUpdatePortRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdatePortRequest._();
  @$core.override
  ApplicationUpdatePortRequest createEmptyInstance() =>
      ApplicationUpdatePortRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdatePortRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdatePortRequest>(
          ApplicationUpdatePortRequest.$_createMessage);
  static ApplicationUpdatePortRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get publishedPort => $_getI64(2);
  @$pb.TagNumber(3)
  set publishedPort($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPublishedPort() => $_has(2);
  @$pb.TagNumber(3)
  void clearPublishedPort() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get targetPort => $_getI64(3);
  @$pb.TagNumber(4)
  set targetPort($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTargetPort() => $_has(3);
  @$pb.TagNumber(4)
  void clearTargetPort() => $_clearField(4);

  @$pb.TagNumber(5)
  ApplicationPortProtocol get protocol => $_getN(4);
  @$pb.TagNumber(5)
  set protocol(ApplicationPortProtocol value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasProtocol() => $_has(4);
  @$pb.TagNumber(5)
  void clearProtocol() => $_clearField(5);

  @$pb.TagNumber(6)
  ApplicationPortPublishMode get publishMode => $_getN(5);
  @$pb.TagNumber(6)
  set publishMode(ApplicationPortPublishMode value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasPublishMode() => $_has(5);
  @$pb.TagNumber(6)
  void clearPublishMode() => $_clearField(6);
}

class ApplicationUpdatePortResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdatePortResponse({
    ApplicationPort? port,
  }) {
    final result = ApplicationUpdatePortResponse._();
    if (port != null) result.port = port;
    return result;
  }

  ApplicationUpdatePortResponse._();

  factory ApplicationUpdatePortResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePortResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdatePortResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePortResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdatePortResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdatePortResponse.$_createMessage)
    ..aOM<ApplicationPort>(1, _omitFieldNames ? '' : 'port',
        subBuilder: ApplicationPort.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePortResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePortResponse copyWith(
          void Function(ApplicationUpdatePortResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdatePortResponse))
          as ApplicationUpdatePortResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdatePortResponse() / ApplicationUpdatePortResponse.new instead')
  static ApplicationUpdatePortResponse create() =>
      ApplicationUpdatePortResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdatePortResponse._();
  @$core.override
  ApplicationUpdatePortResponse createEmptyInstance() =>
      ApplicationUpdatePortResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdatePortResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdatePortResponse>(
          ApplicationUpdatePortResponse.$_createMessage);
  static ApplicationUpdatePortResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationPort get port => $_getN(0);
  @$pb.TagNumber(1)
  set port(ApplicationPort value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPort() => $_has(0);
  @$pb.TagNumber(1)
  void clearPort() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationPort ensurePort() => $_ensure(0);
}

class ApplicationDeletePortRequest extends $pb.GeneratedMessage {
  factory ApplicationDeletePortRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeletePortRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeletePortRequest._();

  factory ApplicationDeletePortRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeletePortRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeletePortRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeletePortRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeletePortRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeletePortRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeletePortRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeletePortRequest copyWith(
          void Function(ApplicationDeletePortRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationDeletePortRequest))
          as ApplicationDeletePortRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeletePortRequest() / ApplicationDeletePortRequest.new instead')
  static ApplicationDeletePortRequest create() =>
      ApplicationDeletePortRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeletePortRequest._();
  @$core.override
  ApplicationDeletePortRequest createEmptyInstance() =>
      ApplicationDeletePortRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeletePortRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeletePortRequest>(
          ApplicationDeletePortRequest.$_createMessage);
  static ApplicationDeletePortRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationListMountsRequest extends $pb.GeneratedMessage {
  factory ApplicationListMountsRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationListMountsRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationListMountsRequest._();

  factory ApplicationListMountsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMountsRequest()..mergeFromBuffer(data, registry);
  factory ApplicationListMountsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMountsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListMountsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListMountsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMountsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMountsRequest copyWith(
          void Function(ApplicationListMountsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListMountsRequest))
          as ApplicationListMountsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListMountsRequest() / ApplicationListMountsRequest.new instead')
  static ApplicationListMountsRequest create() =>
      ApplicationListMountsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListMountsRequest._();
  @$core.override
  ApplicationListMountsRequest createEmptyInstance() =>
      ApplicationListMountsRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListMountsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListMountsRequest>(
          ApplicationListMountsRequest.$_createMessage);
  static ApplicationListMountsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationListMountsResponse extends $pb.GeneratedMessage {
  factory ApplicationListMountsResponse({
    $core.Iterable<ApplicationMount>? mounts,
  }) {
    final result = ApplicationListMountsResponse._();
    if (mounts != null) result.mounts.addAll(mounts);
    return result;
  }

  ApplicationListMountsResponse._();

  factory ApplicationListMountsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMountsResponse()..mergeFromBuffer(data, registry);
  factory ApplicationListMountsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMountsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListMountsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListMountsResponse.$_createMessage)
    ..pPM<ApplicationMount>(1, _omitFieldNames ? '' : 'mounts',
        subBuilder: ApplicationMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMountsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMountsResponse copyWith(
          void Function(ApplicationListMountsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListMountsResponse))
          as ApplicationListMountsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListMountsResponse() / ApplicationListMountsResponse.new instead')
  static ApplicationListMountsResponse create() =>
      ApplicationListMountsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListMountsResponse._();
  @$core.override
  ApplicationListMountsResponse createEmptyInstance() =>
      ApplicationListMountsResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListMountsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListMountsResponse>(
          ApplicationListMountsResponse.$_createMessage);
  static ApplicationListMountsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationMount> get mounts => $_getList(0);
}

class ApplicationCreateMountRequest extends $pb.GeneratedMessage {
  factory ApplicationCreateMountRequest({
    $fixnum.Int64? applicationId,
    ApplicationMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = ApplicationCreateMountRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  ApplicationCreateMountRequest._();

  factory ApplicationCreateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMountRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aE<ApplicationMountType>(2, _omitFieldNames ? '' : 'mountType',
        enumValues: ApplicationMountType.values)
    ..aOS(3, _omitFieldNames ? '' : 'hostPath')
    ..aOS(4, _omitFieldNames ? '' : 'volumeName')
    ..aOS(5, _omitFieldNames ? '' : 'filePath')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..aOS(7, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMountRequest copyWith(
          void Function(ApplicationCreateMountRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreateMountRequest))
          as ApplicationCreateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateMountRequest() / ApplicationCreateMountRequest.new instead')
  static ApplicationCreateMountRequest create() =>
      ApplicationCreateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateMountRequest._();
  @$core.override
  ApplicationCreateMountRequest createEmptyInstance() =>
      ApplicationCreateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateMountRequest>(
          ApplicationCreateMountRequest.$_createMessage);
  static ApplicationCreateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  ApplicationMountType get mountType => $_getN(1);
  @$pb.TagNumber(2)
  set mountType(ApplicationMountType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasMountType() => $_has(1);
  @$pb.TagNumber(2)
  void clearMountType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get hostPath => $_getSZ(2);
  @$pb.TagNumber(3)
  set hostPath($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHostPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearHostPath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get volumeName => $_getSZ(3);
  @$pb.TagNumber(4)
  set volumeName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasVolumeName() => $_has(3);
  @$pb.TagNumber(4)
  void clearVolumeName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get filePath => $_getSZ(4);
  @$pb.TagNumber(5)
  set filePath($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFilePath() => $_has(4);
  @$pb.TagNumber(5)
  void clearFilePath() => $_clearField(5);

  /// Inline file content for FILE mounts.
  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get mountPath => $_getSZ(6);
  @$pb.TagNumber(7)
  set mountPath($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasMountPath() => $_has(6);
  @$pb.TagNumber(7)
  void clearMountPath() => $_clearField(7);
}

class ApplicationCreateMountResponse extends $pb.GeneratedMessage {
  factory ApplicationCreateMountResponse({
    ApplicationMount? mount,
  }) {
    final result = ApplicationCreateMountResponse._();
    if (mount != null) result.mount = mount;
    return result;
  }

  ApplicationCreateMountResponse._();

  factory ApplicationCreateMountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMountResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreateMountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMountResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateMountResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateMountResponse.$_createMessage)
    ..aOM<ApplicationMount>(1, _omitFieldNames ? '' : 'mount',
        subBuilder: ApplicationMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMountResponse copyWith(
          void Function(ApplicationCreateMountResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreateMountResponse))
          as ApplicationCreateMountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateMountResponse() / ApplicationCreateMountResponse.new instead')
  static ApplicationCreateMountResponse create() =>
      ApplicationCreateMountResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateMountResponse._();
  @$core.override
  ApplicationCreateMountResponse createEmptyInstance() =>
      ApplicationCreateMountResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateMountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateMountResponse>(
          ApplicationCreateMountResponse.$_createMessage);
  static ApplicationCreateMountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationMount get mount => $_getN(0);
  @$pb.TagNumber(1)
  set mount(ApplicationMount value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMount() => $_has(0);
  @$pb.TagNumber(1)
  void clearMount() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationMount ensureMount() => $_ensure(0);
}

class ApplicationUpdateMountRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdateMountRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    ApplicationMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = ApplicationUpdateMountRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  ApplicationUpdateMountRequest._();

  factory ApplicationUpdateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMountRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aE<ApplicationMountType>(3, _omitFieldNames ? '' : 'mountType',
        enumValues: ApplicationMountType.values)
    ..aOS(4, _omitFieldNames ? '' : 'hostPath')
    ..aOS(5, _omitFieldNames ? '' : 'volumeName')
    ..aOS(6, _omitFieldNames ? '' : 'filePath')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..aOS(8, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMountRequest copyWith(
          void Function(ApplicationUpdateMountRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdateMountRequest))
          as ApplicationUpdateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateMountRequest() / ApplicationUpdateMountRequest.new instead')
  static ApplicationUpdateMountRequest create() =>
      ApplicationUpdateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateMountRequest._();
  @$core.override
  ApplicationUpdateMountRequest createEmptyInstance() =>
      ApplicationUpdateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateMountRequest>(
          ApplicationUpdateMountRequest.$_createMessage);
  static ApplicationUpdateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  ApplicationMountType get mountType => $_getN(2);
  @$pb.TagNumber(3)
  set mountType(ApplicationMountType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasMountType() => $_has(2);
  @$pb.TagNumber(3)
  void clearMountType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get hostPath => $_getSZ(3);
  @$pb.TagNumber(4)
  set hostPath($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasHostPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearHostPath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get volumeName => $_getSZ(4);
  @$pb.TagNumber(5)
  set volumeName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasVolumeName() => $_has(4);
  @$pb.TagNumber(5)
  void clearVolumeName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get filePath => $_getSZ(5);
  @$pb.TagNumber(6)
  set filePath($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasFilePath() => $_has(5);
  @$pb.TagNumber(6)
  void clearFilePath() => $_clearField(6);

  /// Inline file content for FILE mounts.
  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get mountPath => $_getSZ(7);
  @$pb.TagNumber(8)
  set mountPath($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasMountPath() => $_has(7);
  @$pb.TagNumber(8)
  void clearMountPath() => $_clearField(8);
}

class ApplicationUpdateMountResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdateMountResponse({
    ApplicationMount? mount,
  }) {
    final result = ApplicationUpdateMountResponse._();
    if (mount != null) result.mount = mount;
    return result;
  }

  ApplicationUpdateMountResponse._();

  factory ApplicationUpdateMountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMountResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateMountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMountResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateMountResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateMountResponse.$_createMessage)
    ..aOM<ApplicationMount>(1, _omitFieldNames ? '' : 'mount',
        subBuilder: ApplicationMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMountResponse copyWith(
          void Function(ApplicationUpdateMountResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdateMountResponse))
          as ApplicationUpdateMountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateMountResponse() / ApplicationUpdateMountResponse.new instead')
  static ApplicationUpdateMountResponse create() =>
      ApplicationUpdateMountResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateMountResponse._();
  @$core.override
  ApplicationUpdateMountResponse createEmptyInstance() =>
      ApplicationUpdateMountResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateMountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateMountResponse>(
          ApplicationUpdateMountResponse.$_createMessage);
  static ApplicationUpdateMountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationMount get mount => $_getN(0);
  @$pb.TagNumber(1)
  set mount(ApplicationMount value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMount() => $_has(0);
  @$pb.TagNumber(1)
  void clearMount() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationMount ensureMount() => $_ensure(0);
}

class ApplicationDeleteMountRequest extends $pb.GeneratedMessage {
  factory ApplicationDeleteMountRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeleteMountRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeleteMountRequest._();

  factory ApplicationDeleteMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteMountRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeleteMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeleteMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeleteMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteMountRequest copyWith(
          void Function(ApplicationDeleteMountRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationDeleteMountRequest))
          as ApplicationDeleteMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeleteMountRequest() / ApplicationDeleteMountRequest.new instead')
  static ApplicationDeleteMountRequest create() =>
      ApplicationDeleteMountRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeleteMountRequest._();
  @$core.override
  ApplicationDeleteMountRequest createEmptyInstance() =>
      ApplicationDeleteMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeleteMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeleteMountRequest>(
          ApplicationDeleteMountRequest.$_createMessage);
  static ApplicationDeleteMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationListPatchesRequest extends $pb.GeneratedMessage {
  factory ApplicationListPatchesRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationListPatchesRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationListPatchesRequest._();

  factory ApplicationListPatchesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPatchesRequest()..mergeFromBuffer(data, registry);
  factory ApplicationListPatchesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPatchesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListPatchesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListPatchesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPatchesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPatchesRequest copyWith(
          void Function(ApplicationListPatchesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListPatchesRequest))
          as ApplicationListPatchesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListPatchesRequest() / ApplicationListPatchesRequest.new instead')
  static ApplicationListPatchesRequest create() =>
      ApplicationListPatchesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListPatchesRequest._();
  @$core.override
  ApplicationListPatchesRequest createEmptyInstance() =>
      ApplicationListPatchesRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListPatchesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListPatchesRequest>(
          ApplicationListPatchesRequest.$_createMessage);
  static ApplicationListPatchesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationListPatchesResponse extends $pb.GeneratedMessage {
  factory ApplicationListPatchesResponse({
    $core.Iterable<ApplicationPatch>? patches,
  }) {
    final result = ApplicationListPatchesResponse._();
    if (patches != null) result.patches.addAll(patches);
    return result;
  }

  ApplicationListPatchesResponse._();

  factory ApplicationListPatchesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPatchesResponse()..mergeFromBuffer(data, registry);
  factory ApplicationListPatchesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListPatchesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListPatchesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListPatchesResponse.$_createMessage)
    ..pPM<ApplicationPatch>(1, _omitFieldNames ? '' : 'patches',
        subBuilder: ApplicationPatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPatchesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListPatchesResponse copyWith(
          void Function(ApplicationListPatchesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListPatchesResponse))
          as ApplicationListPatchesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListPatchesResponse() / ApplicationListPatchesResponse.new instead')
  static ApplicationListPatchesResponse create() =>
      ApplicationListPatchesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListPatchesResponse._();
  @$core.override
  ApplicationListPatchesResponse createEmptyInstance() =>
      ApplicationListPatchesResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListPatchesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListPatchesResponse>(
          ApplicationListPatchesResponse.$_createMessage);
  static ApplicationListPatchesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationPatch> get patches => $_getList(0);
}

class ApplicationCreatePatchRequest extends $pb.GeneratedMessage {
  factory ApplicationCreatePatchRequest({
    $fixnum.Int64? applicationId,
    ApplicationPatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
  }) {
    final result = ApplicationCreatePatchRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    return result;
  }

  ApplicationCreatePatchRequest._();

  factory ApplicationCreatePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePatchRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreatePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreatePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreatePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aE<ApplicationPatchType>(2, _omitFieldNames ? '' : 'patchType',
        enumValues: ApplicationPatchType.values)
    ..aOS(3, _omitFieldNames ? '' : 'filePath')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePatchRequest copyWith(
          void Function(ApplicationCreatePatchRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreatePatchRequest))
          as ApplicationCreatePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreatePatchRequest() / ApplicationCreatePatchRequest.new instead')
  static ApplicationCreatePatchRequest create() =>
      ApplicationCreatePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreatePatchRequest._();
  @$core.override
  ApplicationCreatePatchRequest createEmptyInstance() =>
      ApplicationCreatePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreatePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreatePatchRequest>(
          ApplicationCreatePatchRequest.$_createMessage);
  static ApplicationCreatePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  ApplicationPatchType get patchType => $_getN(1);
  @$pb.TagNumber(2)
  set patchType(ApplicationPatchType value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPatchType() => $_has(1);
  @$pb.TagNumber(2)
  void clearPatchType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get filePath => $_getSZ(2);
  @$pb.TagNumber(3)
  set filePath($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFilePath() => $_has(2);
  @$pb.TagNumber(3)
  void clearFilePath() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get enabled => $_getBF(3);
  @$pb.TagNumber(4)
  set enabled($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnabled() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnabled() => $_clearField(4);

  /// Patch file body (raw text); cleared for DELETE patches.
  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => $_clearField(5);
}

class ApplicationCreatePatchResponse extends $pb.GeneratedMessage {
  factory ApplicationCreatePatchResponse({
    ApplicationPatch? patch,
  }) {
    final result = ApplicationCreatePatchResponse._();
    if (patch != null) result.patch = patch;
    return result;
  }

  ApplicationCreatePatchResponse._();

  factory ApplicationCreatePatchResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePatchResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreatePatchResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreatePatchResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreatePatchResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreatePatchResponse.$_createMessage)
    ..aOM<ApplicationPatch>(1, _omitFieldNames ? '' : 'patch',
        subBuilder: ApplicationPatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePatchResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreatePatchResponse copyWith(
          void Function(ApplicationCreatePatchResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreatePatchResponse))
          as ApplicationCreatePatchResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreatePatchResponse() / ApplicationCreatePatchResponse.new instead')
  static ApplicationCreatePatchResponse create() =>
      ApplicationCreatePatchResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreatePatchResponse._();
  @$core.override
  ApplicationCreatePatchResponse createEmptyInstance() =>
      ApplicationCreatePatchResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreatePatchResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreatePatchResponse>(
          ApplicationCreatePatchResponse.$_createMessage);
  static ApplicationCreatePatchResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationPatch get patch => $_getN(0);
  @$pb.TagNumber(1)
  set patch(ApplicationPatch value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPatch() => $_has(0);
  @$pb.TagNumber(1)
  void clearPatch() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationPatch ensurePatch() => $_ensure(0);
}

class ApplicationUpdatePatchRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdatePatchRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    ApplicationPatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
  }) {
    final result = ApplicationUpdatePatchRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    return result;
  }

  ApplicationUpdatePatchRequest._();

  factory ApplicationUpdatePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePatchRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdatePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdatePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdatePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aE<ApplicationPatchType>(3, _omitFieldNames ? '' : 'patchType',
        enumValues: ApplicationPatchType.values)
    ..aOS(4, _omitFieldNames ? '' : 'filePath')
    ..aOB(5, _omitFieldNames ? '' : 'enabled')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePatchRequest copyWith(
          void Function(ApplicationUpdatePatchRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdatePatchRequest))
          as ApplicationUpdatePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdatePatchRequest() / ApplicationUpdatePatchRequest.new instead')
  static ApplicationUpdatePatchRequest create() =>
      ApplicationUpdatePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdatePatchRequest._();
  @$core.override
  ApplicationUpdatePatchRequest createEmptyInstance() =>
      ApplicationUpdatePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdatePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdatePatchRequest>(
          ApplicationUpdatePatchRequest.$_createMessage);
  static ApplicationUpdatePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  ApplicationPatchType get patchType => $_getN(2);
  @$pb.TagNumber(3)
  set patchType(ApplicationPatchType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasPatchType() => $_has(2);
  @$pb.TagNumber(3)
  void clearPatchType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get filePath => $_getSZ(3);
  @$pb.TagNumber(4)
  set filePath($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFilePath() => $_has(3);
  @$pb.TagNumber(4)
  void clearFilePath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get enabled => $_getBF(4);
  @$pb.TagNumber(5)
  set enabled($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEnabled() => $_has(4);
  @$pb.TagNumber(5)
  void clearEnabled() => $_clearField(5);

  /// Patch file body (raw text); cleared for DELETE patches.
  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => $_clearField(6);
}

class ApplicationUpdatePatchResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdatePatchResponse({
    ApplicationPatch? patch,
  }) {
    final result = ApplicationUpdatePatchResponse._();
    if (patch != null) result.patch = patch;
    return result;
  }

  ApplicationUpdatePatchResponse._();

  factory ApplicationUpdatePatchResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePatchResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdatePatchResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdatePatchResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdatePatchResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdatePatchResponse.$_createMessage)
    ..aOM<ApplicationPatch>(1, _omitFieldNames ? '' : 'patch',
        subBuilder: ApplicationPatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePatchResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdatePatchResponse copyWith(
          void Function(ApplicationUpdatePatchResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdatePatchResponse))
          as ApplicationUpdatePatchResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdatePatchResponse() / ApplicationUpdatePatchResponse.new instead')
  static ApplicationUpdatePatchResponse create() =>
      ApplicationUpdatePatchResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdatePatchResponse._();
  @$core.override
  ApplicationUpdatePatchResponse createEmptyInstance() =>
      ApplicationUpdatePatchResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdatePatchResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdatePatchResponse>(
          ApplicationUpdatePatchResponse.$_createMessage);
  static ApplicationUpdatePatchResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationPatch get patch => $_getN(0);
  @$pb.TagNumber(1)
  set patch(ApplicationPatch value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPatch() => $_has(0);
  @$pb.TagNumber(1)
  void clearPatch() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationPatch ensurePatch() => $_ensure(0);
}

class ApplicationDeletePatchRequest extends $pb.GeneratedMessage {
  factory ApplicationDeletePatchRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeletePatchRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeletePatchRequest._();

  factory ApplicationDeletePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeletePatchRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeletePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeletePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeletePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeletePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeletePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeletePatchRequest copyWith(
          void Function(ApplicationDeletePatchRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationDeletePatchRequest))
          as ApplicationDeletePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeletePatchRequest() / ApplicationDeletePatchRequest.new instead')
  static ApplicationDeletePatchRequest create() =>
      ApplicationDeletePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeletePatchRequest._();
  @$core.override
  ApplicationDeletePatchRequest createEmptyInstance() =>
      ApplicationDeletePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeletePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeletePatchRequest>(
          ApplicationDeletePatchRequest.$_createMessage);
  static ApplicationDeletePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationListMiddlewaresRequest extends $pb.GeneratedMessage {
  factory ApplicationListMiddlewaresRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationListMiddlewaresRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationListMiddlewaresRequest._();

  factory ApplicationListMiddlewaresRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMiddlewaresRequest()..mergeFromBuffer(data, registry);
  factory ApplicationListMiddlewaresRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMiddlewaresRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListMiddlewaresRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListMiddlewaresRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMiddlewaresRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMiddlewaresRequest copyWith(
          void Function(ApplicationListMiddlewaresRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationListMiddlewaresRequest))
          as ApplicationListMiddlewaresRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListMiddlewaresRequest() / ApplicationListMiddlewaresRequest.new instead')
  static ApplicationListMiddlewaresRequest create() =>
      ApplicationListMiddlewaresRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListMiddlewaresRequest._();
  @$core.override
  ApplicationListMiddlewaresRequest createEmptyInstance() =>
      ApplicationListMiddlewaresRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListMiddlewaresRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListMiddlewaresRequest>(
          ApplicationListMiddlewaresRequest.$_createMessage);
  static ApplicationListMiddlewaresRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationListMiddlewaresResponse extends $pb.GeneratedMessage {
  factory ApplicationListMiddlewaresResponse({
    $core.Iterable<ApplicationMiddleware>? middlewares,
  }) {
    final result = ApplicationListMiddlewaresResponse._();
    if (middlewares != null) result.middlewares.addAll(middlewares);
    return result;
  }

  ApplicationListMiddlewaresResponse._();

  factory ApplicationListMiddlewaresResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMiddlewaresResponse()..mergeFromBuffer(data, registry);
  factory ApplicationListMiddlewaresResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListMiddlewaresResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListMiddlewaresResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListMiddlewaresResponse.$_createMessage)
    ..pPM<ApplicationMiddleware>(1, _omitFieldNames ? '' : 'middlewares',
        subBuilder: ApplicationMiddleware.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMiddlewaresResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListMiddlewaresResponse copyWith(
          void Function(ApplicationListMiddlewaresResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationListMiddlewaresResponse))
          as ApplicationListMiddlewaresResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListMiddlewaresResponse() / ApplicationListMiddlewaresResponse.new instead')
  static ApplicationListMiddlewaresResponse create() =>
      ApplicationListMiddlewaresResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListMiddlewaresResponse._();
  @$core.override
  ApplicationListMiddlewaresResponse createEmptyInstance() =>
      ApplicationListMiddlewaresResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListMiddlewaresResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListMiddlewaresResponse>(
          ApplicationListMiddlewaresResponse.$_createMessage);
  static ApplicationListMiddlewaresResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationMiddleware> get middlewares => $_getList(0);
}

class ApplicationCreateMiddlewareRequest extends $pb.GeneratedMessage {
  factory ApplicationCreateMiddlewareRequest({
    $fixnum.Int64? applicationId,
    $core.String? name,
    ApplicationMiddlewareType? middlewareType,
    $core.bool? enabled,
    $core.Iterable<ApplicationMiddlewareHeader>? headers,
    $fixnum.Int64? average,
    $fixnum.Int64? burst,
    $core.Iterable<$core.String>? sourceRanges,
  }) {
    final result = ApplicationCreateMiddlewareRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (name != null) result.name = name;
    if (middlewareType != null) result.middlewareType = middlewareType;
    if (enabled != null) result.enabled = enabled;
    if (headers != null) result.headers.addAll(headers);
    if (average != null) result.average = average;
    if (burst != null) result.burst = burst;
    if (sourceRanges != null) result.sourceRanges.addAll(sourceRanges);
    return result;
  }

  ApplicationCreateMiddlewareRequest._();

  factory ApplicationCreateMiddlewareRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMiddlewareRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreateMiddlewareRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMiddlewareRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateMiddlewareRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateMiddlewareRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<ApplicationMiddlewareType>(3, _omitFieldNames ? '' : 'middlewareType',
        enumValues: ApplicationMiddlewareType.values)
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..pPM<ApplicationMiddlewareHeader>(5, _omitFieldNames ? '' : 'headers',
        subBuilder: ApplicationMiddlewareHeader.$_createMessage)
    ..aInt64(6, _omitFieldNames ? '' : 'average')
    ..aInt64(7, _omitFieldNames ? '' : 'burst')
    ..pPS(8, _omitFieldNames ? '' : 'sourceRanges')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMiddlewareRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMiddlewareRequest copyWith(
          void Function(ApplicationCreateMiddlewareRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationCreateMiddlewareRequest))
          as ApplicationCreateMiddlewareRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateMiddlewareRequest() / ApplicationCreateMiddlewareRequest.new instead')
  static ApplicationCreateMiddlewareRequest create() =>
      ApplicationCreateMiddlewareRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateMiddlewareRequest._();
  @$core.override
  ApplicationCreateMiddlewareRequest createEmptyInstance() =>
      ApplicationCreateMiddlewareRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateMiddlewareRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateMiddlewareRequest>(
          ApplicationCreateMiddlewareRequest.$_createMessage);
  static ApplicationCreateMiddlewareRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  ApplicationMiddlewareType get middlewareType => $_getN(2);
  @$pb.TagNumber(3)
  set middlewareType(ApplicationMiddlewareType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasMiddlewareType() => $_has(2);
  @$pb.TagNumber(3)
  void clearMiddlewareType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get enabled => $_getBF(3);
  @$pb.TagNumber(4)
  set enabled($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnabled() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnabled() => $_clearField(4);

  @$pb.TagNumber(5)
  $pb.PbList<ApplicationMiddlewareHeader> get headers => $_getList(4);

  @$pb.TagNumber(6)
  $fixnum.Int64 get average => $_getI64(5);
  @$pb.TagNumber(6)
  set average($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAverage() => $_has(5);
  @$pb.TagNumber(6)
  void clearAverage() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get burst => $_getI64(6);
  @$pb.TagNumber(7)
  set burst($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBurst() => $_has(6);
  @$pb.TagNumber(7)
  void clearBurst() => $_clearField(7);

  @$pb.TagNumber(8)
  $pb.PbList<$core.String> get sourceRanges => $_getList(7);
}

class ApplicationCreateMiddlewareResponse extends $pb.GeneratedMessage {
  factory ApplicationCreateMiddlewareResponse({
    ApplicationMiddleware? middleware,
  }) {
    final result = ApplicationCreateMiddlewareResponse._();
    if (middleware != null) result.middleware = middleware;
    return result;
  }

  ApplicationCreateMiddlewareResponse._();

  factory ApplicationCreateMiddlewareResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMiddlewareResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreateMiddlewareResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateMiddlewareResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateMiddlewareResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateMiddlewareResponse.$_createMessage)
    ..aOM<ApplicationMiddleware>(1, _omitFieldNames ? '' : 'middleware',
        subBuilder: ApplicationMiddleware.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMiddlewareResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateMiddlewareResponse copyWith(
          void Function(ApplicationCreateMiddlewareResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationCreateMiddlewareResponse))
          as ApplicationCreateMiddlewareResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateMiddlewareResponse() / ApplicationCreateMiddlewareResponse.new instead')
  static ApplicationCreateMiddlewareResponse create() =>
      ApplicationCreateMiddlewareResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateMiddlewareResponse._();
  @$core.override
  ApplicationCreateMiddlewareResponse createEmptyInstance() =>
      ApplicationCreateMiddlewareResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateMiddlewareResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ApplicationCreateMiddlewareResponse>(
          ApplicationCreateMiddlewareResponse.$_createMessage);
  static ApplicationCreateMiddlewareResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationMiddleware get middleware => $_getN(0);
  @$pb.TagNumber(1)
  set middleware(ApplicationMiddleware value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMiddleware() => $_has(0);
  @$pb.TagNumber(1)
  void clearMiddleware() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationMiddleware ensureMiddleware() => $_ensure(0);
}

class ApplicationUpdateMiddlewareRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdateMiddlewareRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    $core.String? name,
    ApplicationMiddlewareType? middlewareType,
    $core.bool? enabled,
    $core.Iterable<ApplicationMiddlewareHeader>? headers,
    $fixnum.Int64? average,
    $fixnum.Int64? burst,
    $core.Iterable<$core.String>? sourceRanges,
  }) {
    final result = ApplicationUpdateMiddlewareRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (middlewareType != null) result.middlewareType = middlewareType;
    if (enabled != null) result.enabled = enabled;
    if (headers != null) result.headers.addAll(headers);
    if (average != null) result.average = average;
    if (burst != null) result.burst = burst;
    if (sourceRanges != null) result.sourceRanges.addAll(sourceRanges);
    return result;
  }

  ApplicationUpdateMiddlewareRequest._();

  factory ApplicationUpdateMiddlewareRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMiddlewareRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateMiddlewareRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMiddlewareRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateMiddlewareRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateMiddlewareRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aE<ApplicationMiddlewareType>(4, _omitFieldNames ? '' : 'middlewareType',
        enumValues: ApplicationMiddlewareType.values)
    ..aOB(5, _omitFieldNames ? '' : 'enabled')
    ..pPM<ApplicationMiddlewareHeader>(6, _omitFieldNames ? '' : 'headers',
        subBuilder: ApplicationMiddlewareHeader.$_createMessage)
    ..aInt64(7, _omitFieldNames ? '' : 'average')
    ..aInt64(8, _omitFieldNames ? '' : 'burst')
    ..pPS(9, _omitFieldNames ? '' : 'sourceRanges')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMiddlewareRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMiddlewareRequest copyWith(
          void Function(ApplicationUpdateMiddlewareRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationUpdateMiddlewareRequest))
          as ApplicationUpdateMiddlewareRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateMiddlewareRequest() / ApplicationUpdateMiddlewareRequest.new instead')
  static ApplicationUpdateMiddlewareRequest create() =>
      ApplicationUpdateMiddlewareRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateMiddlewareRequest._();
  @$core.override
  ApplicationUpdateMiddlewareRequest createEmptyInstance() =>
      ApplicationUpdateMiddlewareRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateMiddlewareRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateMiddlewareRequest>(
          ApplicationUpdateMiddlewareRequest.$_createMessage);
  static ApplicationUpdateMiddlewareRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  ApplicationMiddlewareType get middlewareType => $_getN(3);
  @$pb.TagNumber(4)
  set middlewareType(ApplicationMiddlewareType value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMiddlewareType() => $_has(3);
  @$pb.TagNumber(4)
  void clearMiddlewareType() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get enabled => $_getBF(4);
  @$pb.TagNumber(5)
  set enabled($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEnabled() => $_has(4);
  @$pb.TagNumber(5)
  void clearEnabled() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<ApplicationMiddlewareHeader> get headers => $_getList(5);

  @$pb.TagNumber(7)
  $fixnum.Int64 get average => $_getI64(6);
  @$pb.TagNumber(7)
  set average($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAverage() => $_has(6);
  @$pb.TagNumber(7)
  void clearAverage() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get burst => $_getI64(7);
  @$pb.TagNumber(8)
  set burst($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBurst() => $_has(7);
  @$pb.TagNumber(8)
  void clearBurst() => $_clearField(8);

  @$pb.TagNumber(9)
  $pb.PbList<$core.String> get sourceRanges => $_getList(8);
}

class ApplicationUpdateMiddlewareResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdateMiddlewareResponse({
    ApplicationMiddleware? middleware,
  }) {
    final result = ApplicationUpdateMiddlewareResponse._();
    if (middleware != null) result.middleware = middleware;
    return result;
  }

  ApplicationUpdateMiddlewareResponse._();

  factory ApplicationUpdateMiddlewareResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMiddlewareResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateMiddlewareResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateMiddlewareResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateMiddlewareResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateMiddlewareResponse.$_createMessage)
    ..aOM<ApplicationMiddleware>(1, _omitFieldNames ? '' : 'middleware',
        subBuilder: ApplicationMiddleware.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMiddlewareResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateMiddlewareResponse copyWith(
          void Function(ApplicationUpdateMiddlewareResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationUpdateMiddlewareResponse))
          as ApplicationUpdateMiddlewareResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateMiddlewareResponse() / ApplicationUpdateMiddlewareResponse.new instead')
  static ApplicationUpdateMiddlewareResponse create() =>
      ApplicationUpdateMiddlewareResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateMiddlewareResponse._();
  @$core.override
  ApplicationUpdateMiddlewareResponse createEmptyInstance() =>
      ApplicationUpdateMiddlewareResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateMiddlewareResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              ApplicationUpdateMiddlewareResponse>(
          ApplicationUpdateMiddlewareResponse.$_createMessage);
  static ApplicationUpdateMiddlewareResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationMiddleware get middleware => $_getN(0);
  @$pb.TagNumber(1)
  set middleware(ApplicationMiddleware value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMiddleware() => $_has(0);
  @$pb.TagNumber(1)
  void clearMiddleware() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationMiddleware ensureMiddleware() => $_ensure(0);
}

class ApplicationDeleteMiddlewareRequest extends $pb.GeneratedMessage {
  factory ApplicationDeleteMiddlewareRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeleteMiddlewareRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeleteMiddlewareRequest._();

  factory ApplicationDeleteMiddlewareRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteMiddlewareRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeleteMiddlewareRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteMiddlewareRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeleteMiddlewareRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeleteMiddlewareRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteMiddlewareRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteMiddlewareRequest copyWith(
          void Function(ApplicationDeleteMiddlewareRequest) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationDeleteMiddlewareRequest))
          as ApplicationDeleteMiddlewareRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeleteMiddlewareRequest() / ApplicationDeleteMiddlewareRequest.new instead')
  static ApplicationDeleteMiddlewareRequest create() =>
      ApplicationDeleteMiddlewareRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeleteMiddlewareRequest._();
  @$core.override
  ApplicationDeleteMiddlewareRequest createEmptyInstance() =>
      ApplicationDeleteMiddlewareRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeleteMiddlewareRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeleteMiddlewareRequest>(
          ApplicationDeleteMiddlewareRequest.$_createMessage);
  static ApplicationDeleteMiddlewareRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationListRedirectsRequest extends $pb.GeneratedMessage {
  factory ApplicationListRedirectsRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationListRedirectsRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationListRedirectsRequest._();

  factory ApplicationListRedirectsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListRedirectsRequest()..mergeFromBuffer(data, registry);
  factory ApplicationListRedirectsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListRedirectsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListRedirectsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListRedirectsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListRedirectsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListRedirectsRequest copyWith(
          void Function(ApplicationListRedirectsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListRedirectsRequest))
          as ApplicationListRedirectsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListRedirectsRequest() / ApplicationListRedirectsRequest.new instead')
  static ApplicationListRedirectsRequest create() =>
      ApplicationListRedirectsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListRedirectsRequest._();
  @$core.override
  ApplicationListRedirectsRequest createEmptyInstance() =>
      ApplicationListRedirectsRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListRedirectsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListRedirectsRequest>(
          ApplicationListRedirectsRequest.$_createMessage);
  static ApplicationListRedirectsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

class ApplicationListRedirectsResponse extends $pb.GeneratedMessage {
  factory ApplicationListRedirectsResponse({
    $core.Iterable<ApplicationRedirect>? redirects,
  }) {
    final result = ApplicationListRedirectsResponse._();
    if (redirects != null) result.redirects.addAll(redirects);
    return result;
  }

  ApplicationListRedirectsResponse._();

  factory ApplicationListRedirectsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListRedirectsResponse()..mergeFromBuffer(data, registry);
  factory ApplicationListRedirectsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationListRedirectsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationListRedirectsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationListRedirectsResponse.$_createMessage)
    ..pPM<ApplicationRedirect>(1, _omitFieldNames ? '' : 'redirects',
        subBuilder: ApplicationRedirect.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListRedirectsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationListRedirectsResponse copyWith(
          void Function(ApplicationListRedirectsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationListRedirectsResponse))
          as ApplicationListRedirectsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationListRedirectsResponse() / ApplicationListRedirectsResponse.new instead')
  static ApplicationListRedirectsResponse create() =>
      ApplicationListRedirectsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationListRedirectsResponse._();
  @$core.override
  ApplicationListRedirectsResponse createEmptyInstance() =>
      ApplicationListRedirectsResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationListRedirectsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationListRedirectsResponse>(
          ApplicationListRedirectsResponse.$_createMessage);
  static ApplicationListRedirectsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationRedirect> get redirects => $_getList(0);
}

class ApplicationCreateRedirectRequest extends $pb.GeneratedMessage {
  factory ApplicationCreateRedirectRequest({
    $fixnum.Int64? applicationId,
    $core.String? regex,
    $core.String? replacement,
    $core.bool? permanent,
    $fixnum.Int64? uniqueConfigKey,
  }) {
    final result = ApplicationCreateRedirectRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (regex != null) result.regex = regex;
    if (replacement != null) result.replacement = replacement;
    if (permanent != null) result.permanent = permanent;
    if (uniqueConfigKey != null) result.uniqueConfigKey = uniqueConfigKey;
    return result;
  }

  ApplicationCreateRedirectRequest._();

  factory ApplicationCreateRedirectRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateRedirectRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreateRedirectRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateRedirectRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateRedirectRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateRedirectRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aOS(2, _omitFieldNames ? '' : 'regex')
    ..aOS(3, _omitFieldNames ? '' : 'replacement')
    ..aOB(4, _omitFieldNames ? '' : 'permanent')
    ..aInt64(5, _omitFieldNames ? '' : 'uniqueConfigKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateRedirectRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateRedirectRequest copyWith(
          void Function(ApplicationCreateRedirectRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreateRedirectRequest))
          as ApplicationCreateRedirectRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateRedirectRequest() / ApplicationCreateRedirectRequest.new instead')
  static ApplicationCreateRedirectRequest create() =>
      ApplicationCreateRedirectRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateRedirectRequest._();
  @$core.override
  ApplicationCreateRedirectRequest createEmptyInstance() =>
      ApplicationCreateRedirectRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateRedirectRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateRedirectRequest>(
          ApplicationCreateRedirectRequest.$_createMessage);
  static ApplicationCreateRedirectRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get regex => $_getSZ(1);
  @$pb.TagNumber(2)
  set regex($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRegex() => $_has(1);
  @$pb.TagNumber(2)
  void clearRegex() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get replacement => $_getSZ(2);
  @$pb.TagNumber(3)
  set replacement($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasReplacement() => $_has(2);
  @$pb.TagNumber(3)
  void clearReplacement() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get permanent => $_getBF(3);
  @$pb.TagNumber(4)
  set permanent($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPermanent() => $_has(3);
  @$pb.TagNumber(4)
  void clearPermanent() => $_clearField(4);

  /// Opaque Traefik routing-config key.
  @$pb.TagNumber(5)
  $fixnum.Int64 get uniqueConfigKey => $_getI64(4);
  @$pb.TagNumber(5)
  set uniqueConfigKey($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUniqueConfigKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearUniqueConfigKey() => $_clearField(5);
}

class ApplicationCreateRedirectResponse extends $pb.GeneratedMessage {
  factory ApplicationCreateRedirectResponse({
    ApplicationRedirect? redirect,
  }) {
    final result = ApplicationCreateRedirectResponse._();
    if (redirect != null) result.redirect = redirect;
    return result;
  }

  ApplicationCreateRedirectResponse._();

  factory ApplicationCreateRedirectResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateRedirectResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreateRedirectResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateRedirectResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateRedirectResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateRedirectResponse.$_createMessage)
    ..aOM<ApplicationRedirect>(1, _omitFieldNames ? '' : 'redirect',
        subBuilder: ApplicationRedirect.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateRedirectResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateRedirectResponse copyWith(
          void Function(ApplicationCreateRedirectResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationCreateRedirectResponse))
          as ApplicationCreateRedirectResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateRedirectResponse() / ApplicationCreateRedirectResponse.new instead')
  static ApplicationCreateRedirectResponse create() =>
      ApplicationCreateRedirectResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateRedirectResponse._();
  @$core.override
  ApplicationCreateRedirectResponse createEmptyInstance() =>
      ApplicationCreateRedirectResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateRedirectResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateRedirectResponse>(
          ApplicationCreateRedirectResponse.$_createMessage);
  static ApplicationCreateRedirectResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationRedirect get redirect => $_getN(0);
  @$pb.TagNumber(1)
  set redirect(ApplicationRedirect value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRedirect() => $_has(0);
  @$pb.TagNumber(1)
  void clearRedirect() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationRedirect ensureRedirect() => $_ensure(0);
}

class ApplicationUpdateRedirectRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdateRedirectRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    $core.String? regex,
    $core.String? replacement,
    $core.bool? permanent,
    $fixnum.Int64? uniqueConfigKey,
  }) {
    final result = ApplicationUpdateRedirectRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (regex != null) result.regex = regex;
    if (replacement != null) result.replacement = replacement;
    if (permanent != null) result.permanent = permanent;
    if (uniqueConfigKey != null) result.uniqueConfigKey = uniqueConfigKey;
    return result;
  }

  ApplicationUpdateRedirectRequest._();

  factory ApplicationUpdateRedirectRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateRedirectRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateRedirectRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateRedirectRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateRedirectRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateRedirectRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'regex')
    ..aOS(4, _omitFieldNames ? '' : 'replacement')
    ..aOB(5, _omitFieldNames ? '' : 'permanent')
    ..aInt64(6, _omitFieldNames ? '' : 'uniqueConfigKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateRedirectRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateRedirectRequest copyWith(
          void Function(ApplicationUpdateRedirectRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdateRedirectRequest))
          as ApplicationUpdateRedirectRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateRedirectRequest() / ApplicationUpdateRedirectRequest.new instead')
  static ApplicationUpdateRedirectRequest create() =>
      ApplicationUpdateRedirectRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateRedirectRequest._();
  @$core.override
  ApplicationUpdateRedirectRequest createEmptyInstance() =>
      ApplicationUpdateRedirectRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateRedirectRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateRedirectRequest>(
          ApplicationUpdateRedirectRequest.$_createMessage);
  static ApplicationUpdateRedirectRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get regex => $_getSZ(2);
  @$pb.TagNumber(3)
  set regex($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRegex() => $_has(2);
  @$pb.TagNumber(3)
  void clearRegex() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get replacement => $_getSZ(3);
  @$pb.TagNumber(4)
  set replacement($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasReplacement() => $_has(3);
  @$pb.TagNumber(4)
  void clearReplacement() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get permanent => $_getBF(4);
  @$pb.TagNumber(5)
  set permanent($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPermanent() => $_has(4);
  @$pb.TagNumber(5)
  void clearPermanent() => $_clearField(5);

  /// Opaque Traefik routing-config key.
  @$pb.TagNumber(6)
  $fixnum.Int64 get uniqueConfigKey => $_getI64(5);
  @$pb.TagNumber(6)
  set uniqueConfigKey($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasUniqueConfigKey() => $_has(5);
  @$pb.TagNumber(6)
  void clearUniqueConfigKey() => $_clearField(6);
}

class ApplicationUpdateRedirectResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdateRedirectResponse({
    ApplicationRedirect? redirect,
  }) {
    final result = ApplicationUpdateRedirectResponse._();
    if (redirect != null) result.redirect = redirect;
    return result;
  }

  ApplicationUpdateRedirectResponse._();

  factory ApplicationUpdateRedirectResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateRedirectResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateRedirectResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateRedirectResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateRedirectResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateRedirectResponse.$_createMessage)
    ..aOM<ApplicationRedirect>(1, _omitFieldNames ? '' : 'redirect',
        subBuilder: ApplicationRedirect.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateRedirectResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateRedirectResponse copyWith(
          void Function(ApplicationUpdateRedirectResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationUpdateRedirectResponse))
          as ApplicationUpdateRedirectResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateRedirectResponse() / ApplicationUpdateRedirectResponse.new instead')
  static ApplicationUpdateRedirectResponse create() =>
      ApplicationUpdateRedirectResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateRedirectResponse._();
  @$core.override
  ApplicationUpdateRedirectResponse createEmptyInstance() =>
      ApplicationUpdateRedirectResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateRedirectResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateRedirectResponse>(
          ApplicationUpdateRedirectResponse.$_createMessage);
  static ApplicationUpdateRedirectResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationRedirect get redirect => $_getN(0);
  @$pb.TagNumber(1)
  set redirect(ApplicationRedirect value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRedirect() => $_has(0);
  @$pb.TagNumber(1)
  void clearRedirect() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationRedirect ensureRedirect() => $_ensure(0);
}

class ApplicationDeleteRedirectRequest extends $pb.GeneratedMessage {
  factory ApplicationDeleteRedirectRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeleteRedirectRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeleteRedirectRequest._();

  factory ApplicationDeleteRedirectRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteRedirectRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeleteRedirectRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteRedirectRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeleteRedirectRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeleteRedirectRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteRedirectRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteRedirectRequest copyWith(
          void Function(ApplicationDeleteRedirectRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationDeleteRedirectRequest))
          as ApplicationDeleteRedirectRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeleteRedirectRequest() / ApplicationDeleteRedirectRequest.new instead')
  static ApplicationDeleteRedirectRequest create() =>
      ApplicationDeleteRedirectRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeleteRedirectRequest._();
  @$core.override
  ApplicationDeleteRedirectRequest createEmptyInstance() =>
      ApplicationDeleteRedirectRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeleteRedirectRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeleteRedirectRequest>(
          ApplicationDeleteRedirectRequest.$_createMessage);
  static ApplicationDeleteRedirectRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationGetSecurityRequest extends $pb.GeneratedMessage {
  factory ApplicationGetSecurityRequest({
    $fixnum.Int64? applicationId,
  }) {
    final result = ApplicationGetSecurityRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    return result;
  }

  ApplicationGetSecurityRequest._();

  factory ApplicationGetSecurityRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationGetSecurityRequest()..mergeFromBuffer(data, registry);
  factory ApplicationGetSecurityRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationGetSecurityRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationGetSecurityRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationGetSecurityRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationGetSecurityRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationGetSecurityRequest copyWith(
          void Function(ApplicationGetSecurityRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationGetSecurityRequest))
          as ApplicationGetSecurityRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationGetSecurityRequest() / ApplicationGetSecurityRequest.new instead')
  static ApplicationGetSecurityRequest create() =>
      ApplicationGetSecurityRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationGetSecurityRequest._();
  @$core.override
  ApplicationGetSecurityRequest createEmptyInstance() =>
      ApplicationGetSecurityRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationGetSecurityRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationGetSecurityRequest>(
          ApplicationGetSecurityRequest.$_createMessage);
  static ApplicationGetSecurityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);
}

/// `get_security` returns the application's whole rule set, not a single row.
class ApplicationGetSecurityResponse extends $pb.GeneratedMessage {
  factory ApplicationGetSecurityResponse({
    $core.Iterable<ApplicationSecurityRule>? security,
  }) {
    final result = ApplicationGetSecurityResponse._();
    if (security != null) result.security.addAll(security);
    return result;
  }

  ApplicationGetSecurityResponse._();

  factory ApplicationGetSecurityResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationGetSecurityResponse()..mergeFromBuffer(data, registry);
  factory ApplicationGetSecurityResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationGetSecurityResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationGetSecurityResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationGetSecurityResponse.$_createMessage)
    ..pPM<ApplicationSecurityRule>(1, _omitFieldNames ? '' : 'security',
        subBuilder: ApplicationSecurityRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationGetSecurityResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationGetSecurityResponse copyWith(
          void Function(ApplicationGetSecurityResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationGetSecurityResponse))
          as ApplicationGetSecurityResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationGetSecurityResponse() / ApplicationGetSecurityResponse.new instead')
  static ApplicationGetSecurityResponse create() =>
      ApplicationGetSecurityResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationGetSecurityResponse._();
  @$core.override
  ApplicationGetSecurityResponse createEmptyInstance() =>
      ApplicationGetSecurityResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationGetSecurityResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationGetSecurityResponse>(
          ApplicationGetSecurityResponse.$_createMessage);
  static ApplicationGetSecurityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ApplicationSecurityRule> get security => $_getList(0);
}

class ApplicationCreateSecurityRequest extends $pb.GeneratedMessage {
  factory ApplicationCreateSecurityRequest({
    $fixnum.Int64? applicationId,
    $core.String? username,
    $core.String? password,
  }) {
    final result = ApplicationCreateSecurityRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    return result;
  }

  ApplicationCreateSecurityRequest._();

  factory ApplicationCreateSecurityRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateSecurityRequest()..mergeFromBuffer(data, registry);
  factory ApplicationCreateSecurityRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateSecurityRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateSecurityRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateSecurityRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aOS(2, _omitFieldNames ? '' : 'username')
    ..aOS(3, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateSecurityRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateSecurityRequest copyWith(
          void Function(ApplicationCreateSecurityRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationCreateSecurityRequest))
          as ApplicationCreateSecurityRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateSecurityRequest() / ApplicationCreateSecurityRequest.new instead')
  static ApplicationCreateSecurityRequest create() =>
      ApplicationCreateSecurityRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateSecurityRequest._();
  @$core.override
  ApplicationCreateSecurityRequest createEmptyInstance() =>
      ApplicationCreateSecurityRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateSecurityRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateSecurityRequest>(
          ApplicationCreateSecurityRequest.$_createMessage);
  static ApplicationCreateSecurityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

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

class ApplicationCreateSecurityResponse extends $pb.GeneratedMessage {
  factory ApplicationCreateSecurityResponse({
    ApplicationSecurityRule? security,
  }) {
    final result = ApplicationCreateSecurityResponse._();
    if (security != null) result.security = security;
    return result;
  }

  ApplicationCreateSecurityResponse._();

  factory ApplicationCreateSecurityResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateSecurityResponse()..mergeFromBuffer(data, registry);
  factory ApplicationCreateSecurityResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationCreateSecurityResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationCreateSecurityResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationCreateSecurityResponse.$_createMessage)
    ..aOM<ApplicationSecurityRule>(1, _omitFieldNames ? '' : 'security',
        subBuilder: ApplicationSecurityRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateSecurityResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationCreateSecurityResponse copyWith(
          void Function(ApplicationCreateSecurityResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationCreateSecurityResponse))
          as ApplicationCreateSecurityResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationCreateSecurityResponse() / ApplicationCreateSecurityResponse.new instead')
  static ApplicationCreateSecurityResponse create() =>
      ApplicationCreateSecurityResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationCreateSecurityResponse._();
  @$core.override
  ApplicationCreateSecurityResponse createEmptyInstance() =>
      ApplicationCreateSecurityResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationCreateSecurityResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationCreateSecurityResponse>(
          ApplicationCreateSecurityResponse.$_createMessage);
  static ApplicationCreateSecurityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationSecurityRule get security => $_getN(0);
  @$pb.TagNumber(1)
  set security(ApplicationSecurityRule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSecurity() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecurity() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationSecurityRule ensureSecurity() => $_ensure(0);
}

class ApplicationUpdateSecurityRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdateSecurityRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
    $core.String? username,
    $core.String? password,
  }) {
    final result = ApplicationUpdateSecurityRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    if (username != null) result.username = username;
    if (password != null) result.password = password;
    return result;
  }

  ApplicationUpdateSecurityRequest._();

  factory ApplicationUpdateSecurityRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateSecurityRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateSecurityRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateSecurityRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateSecurityRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateSecurityRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateSecurityRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateSecurityRequest copyWith(
          void Function(ApplicationUpdateSecurityRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdateSecurityRequest))
          as ApplicationUpdateSecurityRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateSecurityRequest() / ApplicationUpdateSecurityRequest.new instead')
  static ApplicationUpdateSecurityRequest create() =>
      ApplicationUpdateSecurityRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateSecurityRequest._();
  @$core.override
  ApplicationUpdateSecurityRequest createEmptyInstance() =>
      ApplicationUpdateSecurityRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateSecurityRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateSecurityRequest>(
          ApplicationUpdateSecurityRequest.$_createMessage);
  static ApplicationUpdateSecurityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

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

class ApplicationUpdateSecurityResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdateSecurityResponse({
    ApplicationSecurityRule? security,
  }) {
    final result = ApplicationUpdateSecurityResponse._();
    if (security != null) result.security = security;
    return result;
  }

  ApplicationUpdateSecurityResponse._();

  factory ApplicationUpdateSecurityResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateSecurityResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateSecurityResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateSecurityResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateSecurityResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateSecurityResponse.$_createMessage)
    ..aOM<ApplicationSecurityRule>(1, _omitFieldNames ? '' : 'security',
        subBuilder: ApplicationSecurityRule.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateSecurityResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateSecurityResponse copyWith(
          void Function(ApplicationUpdateSecurityResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationUpdateSecurityResponse))
          as ApplicationUpdateSecurityResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateSecurityResponse() / ApplicationUpdateSecurityResponse.new instead')
  static ApplicationUpdateSecurityResponse create() =>
      ApplicationUpdateSecurityResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateSecurityResponse._();
  @$core.override
  ApplicationUpdateSecurityResponse createEmptyInstance() =>
      ApplicationUpdateSecurityResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateSecurityResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateSecurityResponse>(
          ApplicationUpdateSecurityResponse.$_createMessage);
  static ApplicationUpdateSecurityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ApplicationSecurityRule get security => $_getN(0);
  @$pb.TagNumber(1)
  set security(ApplicationSecurityRule value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSecurity() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecurity() => $_clearField(1);
  @$pb.TagNumber(1)
  ApplicationSecurityRule ensureSecurity() => $_ensure(0);
}

class ApplicationDeleteSecurityRequest extends $pb.GeneratedMessage {
  factory ApplicationDeleteSecurityRequest({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? id,
  }) {
    final result = ApplicationDeleteSecurityRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (id != null) result.id = id;
    return result;
  }

  ApplicationDeleteSecurityRequest._();

  factory ApplicationDeleteSecurityRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteSecurityRequest()..mergeFromBuffer(data, registry);
  factory ApplicationDeleteSecurityRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationDeleteSecurityRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationDeleteSecurityRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationDeleteSecurityRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteSecurityRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationDeleteSecurityRequest copyWith(
          void Function(ApplicationDeleteSecurityRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationDeleteSecurityRequest))
          as ApplicationDeleteSecurityRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationDeleteSecurityRequest() / ApplicationDeleteSecurityRequest.new instead')
  static ApplicationDeleteSecurityRequest create() =>
      ApplicationDeleteSecurityRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationDeleteSecurityRequest._();
  @$core.override
  ApplicationDeleteSecurityRequest createEmptyInstance() =>
      ApplicationDeleteSecurityRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationDeleteSecurityRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationDeleteSecurityRequest>(
          ApplicationDeleteSecurityRequest.$_createMessage);
  static ApplicationDeleteSecurityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);
}

class ApplicationUpdateNetworksRequest extends $pb.GeneratedMessage {
  factory ApplicationUpdateNetworksRequest({
    $fixnum.Int64? applicationId,
    $core.Iterable<$fixnum.Int64>? networkIds,
    $core.bool? detachRustployNetwork,
  }) {
    final result = ApplicationUpdateNetworksRequest._();
    if (applicationId != null) result.applicationId = applicationId;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (detachRustployNetwork != null)
      result.detachRustployNetwork = detachRustployNetwork;
    return result;
  }

  ApplicationUpdateNetworksRequest._();

  factory ApplicationUpdateNetworksRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateNetworksRequest()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateNetworksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..p<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'networkIds', $pb.PbFieldType.K6)
    ..aOB(3, _omitFieldNames ? '' : 'detachRustployNetwork')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateNetworksRequest copyWith(
          void Function(ApplicationUpdateNetworksRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ApplicationUpdateNetworksRequest))
          as ApplicationUpdateNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateNetworksRequest() / ApplicationUpdateNetworksRequest.new instead')
  static ApplicationUpdateNetworksRequest create() =>
      ApplicationUpdateNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateNetworksRequest._();
  @$core.override
  ApplicationUpdateNetworksRequest createEmptyInstance() =>
      ApplicationUpdateNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateNetworksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateNetworksRequest>(
          ApplicationUpdateNetworksRequest.$_createMessage);
  static ApplicationUpdateNetworksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$fixnum.Int64> get networkIds => $_getList(1);

  @$pb.TagNumber(3)
  $core.bool get detachRustployNetwork => $_getBF(2);
  @$pb.TagNumber(3)
  set detachRustployNetwork($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDetachRustployNetwork() => $_has(2);
  @$pb.TagNumber(3)
  void clearDetachRustployNetwork() => $_clearField(3);
}

class ApplicationUpdateNetworksResponse extends $pb.GeneratedMessage {
  factory ApplicationUpdateNetworksResponse({
    $core.bool? success,
    $core.String? operation,
  }) {
    final result = ApplicationUpdateNetworksResponse._();
    if (success != null) result.success = success;
    if (operation != null) result.operation = operation;
    return result;
  }

  ApplicationUpdateNetworksResponse._();

  factory ApplicationUpdateNetworksResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateNetworksResponse()..mergeFromBuffer(data, registry);
  factory ApplicationUpdateNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ApplicationUpdateNetworksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplicationUpdateNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ApplicationUpdateNetworksResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'operation')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplicationUpdateNetworksResponse copyWith(
          void Function(ApplicationUpdateNetworksResponse) updates) =>
      super.copyWith((message) =>
              updates(message as ApplicationUpdateNetworksResponse))
          as ApplicationUpdateNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ApplicationUpdateNetworksResponse() / ApplicationUpdateNetworksResponse.new instead')
  static ApplicationUpdateNetworksResponse create() =>
      ApplicationUpdateNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ApplicationUpdateNetworksResponse._();
  @$core.override
  ApplicationUpdateNetworksResponse createEmptyInstance() =>
      ApplicationUpdateNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static ApplicationUpdateNetworksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplicationUpdateNetworksResponse>(
          ApplicationUpdateNetworksResponse.$_createMessage);
  static ApplicationUpdateNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get operation => $_getSZ(1);
  @$pb.TagNumber(2)
  set operation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOperation() => $_has(1);
  @$pb.TagNumber(2)
  void clearOperation() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

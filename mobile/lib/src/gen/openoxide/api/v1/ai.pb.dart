// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ai.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'ai.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'ai.pbenum.dart';

/// Stored provider configuration; the API key is never echoed back (`has_api_key` reports presence only).
class AiSetting extends $pb.GeneratedMessage {
  factory AiSetting({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? apiUrl,
    $core.String? model,
    $core.bool? isEnabled,
    AiProvider? provider,
    $core.bool? hasApiKey,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = AiSetting._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (model != null) result.model = model;
    if (isEnabled != null) result.isEnabled = isEnabled;
    if (provider != null) result.provider = provider;
    if (hasApiKey != null) result.hasApiKey = hasApiKey;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  AiSetting._();

  factory AiSetting.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiSetting()..mergeFromBuffer(data, registry);
  factory AiSetting.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiSetting()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiSetting',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiSetting.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(4, _omitFieldNames ? '' : 'model')
    ..aOB(5, _omitFieldNames ? '' : 'isEnabled')
    ..aE<AiProvider>(6, _omitFieldNames ? '' : 'provider',
        enumValues: AiProvider.values)
    ..aOB(7, _omitFieldNames ? '' : 'hasApiKey')
    ..aInt64(8, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiSetting clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiSetting copyWith(void Function(AiSetting) updates) =>
      super.copyWith((message) => updates(message as AiSetting)) as AiSetting;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiSetting() / AiSetting.new instead')
  static AiSetting create() => AiSetting._();
  static $pb.GeneratedMessage $_createMessage() => AiSetting._();
  @$core.override
  AiSetting createEmptyInstance() => AiSetting._();
  @$core.pragma('dart2js:noInline')
  static AiSetting getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiSetting>(AiSetting.$_createMessage);
  static AiSetting? _defaultInstance;

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
  $core.String get model => $_getSZ(3);
  @$pb.TagNumber(4)
  set model($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasModel() => $_has(3);
  @$pb.TagNumber(4)
  void clearModel() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isEnabled => $_getBF(4);
  @$pb.TagNumber(5)
  set isEnabled($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIsEnabled() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsEnabled() => $_clearField(5);

  @$pb.TagNumber(6)
  AiProvider get provider => $_getN(5);
  @$pb.TagNumber(6)
  set provider(AiProvider value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasProvider() => $_has(5);
  @$pb.TagNumber(6)
  void clearProvider() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get hasApiKey => $_getBF(6);
  @$pb.TagNumber(7)
  set hasApiKey($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasHasApiKey() => $_has(6);
  @$pb.TagNumber(7)
  void clearHasApiKey() => $_clearField(7);

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

class AiEnvironmentVariable extends $pb.GeneratedMessage {
  factory AiEnvironmentVariable({
    $core.String? name,
    $core.String? value,
  }) {
    final result = AiEnvironmentVariable._();
    if (name != null) result.name = name;
    if (value != null) result.value = value;
    return result;
  }

  AiEnvironmentVariable._();

  factory AiEnvironmentVariable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiEnvironmentVariable()..mergeFromBuffer(data, registry);
  factory AiEnvironmentVariable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiEnvironmentVariable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiEnvironmentVariable',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiEnvironmentVariable.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'value')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiEnvironmentVariable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiEnvironmentVariable copyWith(
          void Function(AiEnvironmentVariable) updates) =>
      super.copyWith((message) => updates(message as AiEnvironmentVariable))
          as AiEnvironmentVariable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiEnvironmentVariable() / AiEnvironmentVariable.new instead')
  static AiEnvironmentVariable create() => AiEnvironmentVariable._();
  static $pb.GeneratedMessage $_createMessage() => AiEnvironmentVariable._();
  @$core.override
  AiEnvironmentVariable createEmptyInstance() => AiEnvironmentVariable._();
  @$core.pragma('dart2js:noInline')
  static AiEnvironmentVariable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiEnvironmentVariable>(
          AiEnvironmentVariable.$_createMessage);
  static AiEnvironmentVariable? _defaultInstance;

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

class AiDomain extends $pb.GeneratedMessage {
  factory AiDomain({
    $core.String? host,
    $fixnum.Int64? port,
    $core.String? serviceName,
  }) {
    final result = AiDomain._();
    if (host != null) result.host = host;
    if (port != null) result.port = port;
    if (serviceName != null) result.serviceName = serviceName;
    return result;
  }

  AiDomain._();

  factory AiDomain.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDomain()..mergeFromBuffer(data, registry);
  factory AiDomain.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDomain()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDomain',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDomain.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'host')
    ..aInt64(2, _omitFieldNames ? '' : 'port')
    ..aOS(3, _omitFieldNames ? '' : 'serviceName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDomain clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDomain copyWith(void Function(AiDomain) updates) =>
      super.copyWith((message) => updates(message as AiDomain)) as AiDomain;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiDomain() / AiDomain.new instead')
  static AiDomain create() => AiDomain._();
  static $pb.GeneratedMessage $_createMessage() => AiDomain._();
  @$core.override
  AiDomain createEmptyInstance() => AiDomain._();
  @$core.pragma('dart2js:noInline')
  static AiDomain getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDomain>(AiDomain.$_createMessage);
  static AiDomain? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get host => $_getSZ(0);
  @$pb.TagNumber(1)
  set host($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearHost() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get port => $_getI64(1);
  @$pb.TagNumber(2)
  set port($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPort() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get serviceName => $_getSZ(2);
  @$pb.TagNumber(3)
  set serviceName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasServiceName() => $_has(2);
  @$pb.TagNumber(3)
  void clearServiceName() => $_clearField(3);
}

class AiGeneratedFile extends $pb.GeneratedMessage {
  factory AiGeneratedFile({
    $core.String? filePath,
    $core.String? content,
  }) {
    final result = AiGeneratedFile._();
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    return result;
  }

  AiGeneratedFile._();

  factory AiGeneratedFile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGeneratedFile()..mergeFromBuffer(data, registry);
  factory AiGeneratedFile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGeneratedFile()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGeneratedFile',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGeneratedFile.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'filePath')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGeneratedFile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGeneratedFile copyWith(void Function(AiGeneratedFile) updates) =>
      super.copyWith((message) => updates(message as AiGeneratedFile))
          as AiGeneratedFile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiGeneratedFile() / AiGeneratedFile.new instead')
  static AiGeneratedFile create() => AiGeneratedFile._();
  static $pb.GeneratedMessage $_createMessage() => AiGeneratedFile._();
  @$core.override
  AiGeneratedFile createEmptyInstance() => AiGeneratedFile._();
  @$core.pragma('dart2js:noInline')
  static AiGeneratedFile getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AiGeneratedFile>(
          AiGeneratedFile.$_createMessage);
  static AiGeneratedFile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get filePath => $_getSZ(0);
  @$pb.TagNumber(1)
  set filePath($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFilePath() => $_has(0);
  @$pb.TagNumber(1)
  void clearFilePath() => $_clearField(1);

  /// File body as text; config files are plain-text templates.
  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);
}

/// One deployable compose suggestion produced by the model.
class AiComposeSuggestion extends $pb.GeneratedMessage {
  factory AiComposeSuggestion({
    $core.String? id,
    $core.String? name,
    $core.String? shortDescription,
    $core.String? description,
    $core.String? dockerCompose,
    $core.Iterable<AiEnvironmentVariable>? envVariables,
    $core.Iterable<AiDomain>? domains,
    $core.Iterable<AiGeneratedFile>? configFiles,
  }) {
    final result = AiComposeSuggestion._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (shortDescription != null) result.shortDescription = shortDescription;
    if (description != null) result.description = description;
    if (dockerCompose != null) result.dockerCompose = dockerCompose;
    if (envVariables != null) result.envVariables.addAll(envVariables);
    if (domains != null) result.domains.addAll(domains);
    if (configFiles != null) result.configFiles.addAll(configFiles);
    return result;
  }

  AiComposeSuggestion._();

  factory AiComposeSuggestion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiComposeSuggestion()..mergeFromBuffer(data, registry);
  factory AiComposeSuggestion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiComposeSuggestion()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiComposeSuggestion',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiComposeSuggestion.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'shortDescription')
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aOS(5, _omitFieldNames ? '' : 'dockerCompose')
    ..pPM<AiEnvironmentVariable>(6, _omitFieldNames ? '' : 'envVariables',
        subBuilder: AiEnvironmentVariable.$_createMessage)
    ..pPM<AiDomain>(7, _omitFieldNames ? '' : 'domains',
        subBuilder: AiDomain.$_createMessage)
    ..pPM<AiGeneratedFile>(8, _omitFieldNames ? '' : 'configFiles',
        subBuilder: AiGeneratedFile.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiComposeSuggestion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiComposeSuggestion copyWith(void Function(AiComposeSuggestion) updates) =>
      super.copyWith((message) => updates(message as AiComposeSuggestion))
          as AiComposeSuggestion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use AiComposeSuggestion() / AiComposeSuggestion.new instead')
  static AiComposeSuggestion create() => AiComposeSuggestion._();
  static $pb.GeneratedMessage $_createMessage() => AiComposeSuggestion._();
  @$core.override
  AiComposeSuggestion createEmptyInstance() => AiComposeSuggestion._();
  @$core.pragma('dart2js:noInline')
  static AiComposeSuggestion getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiComposeSuggestion>(
          AiComposeSuggestion.$_createMessage);
  static AiComposeSuggestion? _defaultInstance;

  /// Model-issued suggestion identifier; unique within one output.
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
  $core.String get shortDescription => $_getSZ(2);
  @$pb.TagNumber(3)
  set shortDescription($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShortDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearShortDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => $_clearField(4);

  /// Docker Compose document (YAML).
  @$pb.TagNumber(5)
  $core.String get dockerCompose => $_getSZ(4);
  @$pb.TagNumber(5)
  set dockerCompose($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDockerCompose() => $_has(4);
  @$pb.TagNumber(5)
  void clearDockerCompose() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<AiEnvironmentVariable> get envVariables => $_getList(5);

  @$pb.TagNumber(7)
  $pb.PbList<AiDomain> get domains => $_getList(6);

  @$pb.TagNumber(8)
  $pb.PbList<AiGeneratedFile> get configFiles => $_getList(7);
}

/// Validated model output: one to three compose suggestions.
class AiGenerationOutput extends $pb.GeneratedMessage {
  factory AiGenerationOutput({
    $core.Iterable<AiComposeSuggestion>? suggestions,
  }) {
    final result = AiGenerationOutput._();
    if (suggestions != null) result.suggestions.addAll(suggestions);
    return result;
  }

  AiGenerationOutput._();

  factory AiGenerationOutput.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGenerationOutput()..mergeFromBuffer(data, registry);
  factory AiGenerationOutput.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGenerationOutput()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGenerationOutput',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGenerationOutput.$_createMessage)
    ..pPM<AiComposeSuggestion>(1, _omitFieldNames ? '' : 'suggestions',
        subBuilder: AiComposeSuggestion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGenerationOutput clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGenerationOutput copyWith(void Function(AiGenerationOutput) updates) =>
      super.copyWith((message) => updates(message as AiGenerationOutput))
          as AiGenerationOutput;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiGenerationOutput() / AiGenerationOutput.new instead')
  static AiGenerationOutput create() => AiGenerationOutput._();
  static $pb.GeneratedMessage $_createMessage() => AiGenerationOutput._();
  @$core.override
  AiGenerationOutput createEmptyInstance() => AiGenerationOutput._();
  @$core.pragma('dart2js:noInline')
  static AiGenerationOutput getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiGenerationOutput>(
          AiGenerationOutput.$_createMessage);
  static AiGenerationOutput? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AiComposeSuggestion> get suggestions => $_getList(0);
}

/// One generation run: the prompt plus its (reviewable) model output.
class AiGeneration extends $pb.GeneratedMessage {
  factory AiGeneration({
    $fixnum.Int64? id,
    $fixnum.Int64? aiSettingId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdBy,
    $core.String? prompt,
    AiGenerationOutput? output,
    $core.String? status,
    $fixnum.Int64? composeId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = AiGeneration._();
    if (id != null) result.id = id;
    if (aiSettingId != null) result.aiSettingId = aiSettingId;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdBy != null) result.createdBy = createdBy;
    if (prompt != null) result.prompt = prompt;
    if (output != null) result.output = output;
    if (status != null) result.status = status;
    if (composeId != null) result.composeId = composeId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  AiGeneration._();

  factory AiGeneration.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGeneration()..mergeFromBuffer(data, registry);
  factory AiGeneration.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGeneration()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGeneration',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGeneration.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'aiSettingId')
    ..aInt64(3, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(4, _omitFieldNames ? '' : 'createdBy')
    ..aOS(5, _omitFieldNames ? '' : 'prompt')
    ..aOM<AiGenerationOutput>(6, _omitFieldNames ? '' : 'output',
        subBuilder: AiGenerationOutput.$_createMessage)
    ..aOS(7, _omitFieldNames ? '' : 'status')
    ..aInt64(8, _omitFieldNames ? '' : 'composeId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGeneration clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGeneration copyWith(void Function(AiGeneration) updates) =>
      super.copyWith((message) => updates(message as AiGeneration))
          as AiGeneration;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiGeneration() / AiGeneration.new instead')
  static AiGeneration create() => AiGeneration._();
  static $pb.GeneratedMessage $_createMessage() => AiGeneration._();
  @$core.override
  AiGeneration createEmptyInstance() => AiGeneration._();
  @$core.pragma('dart2js:noInline')
  static AiGeneration getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AiGeneration>(
          AiGeneration.$_createMessage);
  static AiGeneration? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get aiSettingId => $_getI64(1);
  @$pb.TagNumber(2)
  set aiSettingId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAiSettingId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAiSettingId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get organizationId => $_getI64(2);
  @$pb.TagNumber(3)
  set organizationId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOrganizationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOrganizationId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get createdBy => $_getI64(3);
  @$pb.TagNumber(4)
  set createdBy($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCreatedBy() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreatedBy() => $_clearField(4);

  /// Natural-language prompt echoed back verbatim.
  @$pb.TagNumber(5)
  $core.String get prompt => $_getSZ(4);
  @$pb.TagNumber(5)
  set prompt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPrompt() => $_has(4);
  @$pb.TagNumber(5)
  void clearPrompt() => $_clearField(5);

  @$pb.TagNumber(6)
  AiGenerationOutput get output => $_getN(5);
  @$pb.TagNumber(6)
  set output(AiGenerationOutput value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasOutput() => $_has(5);
  @$pb.TagNumber(6)
  void clearOutput() => $_clearField(6);
  @$pb.TagNumber(6)
  AiGenerationOutput ensureOutput() => $_ensure(5);

  /// Lifecycle status string ("REVIEWED", "DEPLOYING", "DEPLOYED", ...).
  @$pb.TagNumber(7)
  $core.String get status => $_getSZ(6);
  @$pb.TagNumber(7)
  set status($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasStatus() => $_has(6);
  @$pb.TagNumber(7)
  void clearStatus() => $_clearField(7);

  /// Compose service row created by deployment, once deployed.
  @$pb.TagNumber(8)
  $fixnum.Int64 get composeId => $_getI64(7);
  @$pb.TagNumber(8)
  set composeId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasComposeId() => $_has(7);
  @$pb.TagNumber(8)
  void clearComposeId() => $_clearField(8);

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

class AiListSettingsRequest extends $pb.GeneratedMessage {
  factory AiListSettingsRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = AiListSettingsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiListSettingsRequest._();

  factory AiListSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiListSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListSettingsRequest copyWith(
          void Function(AiListSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiListSettingsRequest))
          as AiListSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiListSettingsRequest() / AiListSettingsRequest.new instead')
  static AiListSettingsRequest create() => AiListSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiListSettingsRequest._();
  @$core.override
  AiListSettingsRequest createEmptyInstance() => AiListSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiListSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListSettingsRequest>(
          AiListSettingsRequest.$_createMessage);
  static AiListSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class AiListSettingsResponse extends $pb.GeneratedMessage {
  factory AiListSettingsResponse({
    $core.Iterable<AiSetting>? settings,
  }) {
    final result = AiListSettingsResponse._();
    if (settings != null) result.settings.addAll(settings);
    return result;
  }

  AiListSettingsResponse._();

  factory AiListSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListSettingsResponse()..mergeFromBuffer(data, registry);
  factory AiListSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListSettingsResponse.$_createMessage)
    ..pPM<AiSetting>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: AiSetting.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListSettingsResponse copyWith(
          void Function(AiListSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as AiListSettingsResponse))
          as AiListSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiListSettingsResponse() / AiListSettingsResponse.new instead')
  static AiListSettingsResponse create() => AiListSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiListSettingsResponse._();
  @$core.override
  AiListSettingsResponse createEmptyInstance() => AiListSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiListSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListSettingsResponse>(
          AiListSettingsResponse.$_createMessage);
  static AiListSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AiSetting> get settings => $_getList(0);
}

class AiGetSettingsRequest extends $pb.GeneratedMessage {
  factory AiGetSettingsRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiGetSettingsRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiGetSettingsRequest._();

  factory AiGetSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiGetSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGetSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGetSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetSettingsRequest copyWith(void Function(AiGetSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiGetSettingsRequest))
          as AiGetSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiGetSettingsRequest() / AiGetSettingsRequest.new instead')
  static AiGetSettingsRequest create() => AiGetSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiGetSettingsRequest._();
  @$core.override
  AiGetSettingsRequest createEmptyInstance() => AiGetSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiGetSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiGetSettingsRequest>(
          AiGetSettingsRequest.$_createMessage);
  static AiGetSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class AiGetSettingsResponse extends $pb.GeneratedMessage {
  factory AiGetSettingsResponse({
    AiSetting? setting,
  }) {
    final result = AiGetSettingsResponse._();
    if (setting != null) result.setting = setting;
    return result;
  }

  AiGetSettingsResponse._();

  factory AiGetSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetSettingsResponse()..mergeFromBuffer(data, registry);
  factory AiGetSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGetSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGetSettingsResponse.$_createMessage)
    ..aOM<AiSetting>(1, _omitFieldNames ? '' : 'setting',
        subBuilder: AiSetting.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetSettingsResponse copyWith(
          void Function(AiGetSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as AiGetSettingsResponse))
          as AiGetSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiGetSettingsResponse() / AiGetSettingsResponse.new instead')
  static AiGetSettingsResponse create() => AiGetSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiGetSettingsResponse._();
  @$core.override
  AiGetSettingsResponse createEmptyInstance() => AiGetSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiGetSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiGetSettingsResponse>(
          AiGetSettingsResponse.$_createMessage);
  static AiGetSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiSetting get setting => $_getN(0);
  @$pb.TagNumber(1)
  set setting(AiSetting value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSetting() => $_has(0);
  @$pb.TagNumber(1)
  void clearSetting() => $_clearField(1);
  @$pb.TagNumber(1)
  AiSetting ensureSetting() => $_ensure(0);
}

class AiCreateSettingsRequest extends $pb.GeneratedMessage {
  factory AiCreateSettingsRequest({
    $core.String? name,
    $core.String? apiUrl,
    $core.String? apiKey,
    $core.String? model,
    $core.bool? isEnabled,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiCreateSettingsRequest._();
    if (name != null) result.name = name;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (apiKey != null) result.apiKey = apiKey;
    if (model != null) result.model = model;
    if (isEnabled != null) result.isEnabled = isEnabled;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiCreateSettingsRequest._();

  factory AiCreateSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiCreateSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiCreateSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiCreateSettingsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(3, _omitFieldNames ? '' : 'apiKey')
    ..aOS(4, _omitFieldNames ? '' : 'model')
    ..aOB(5, _omitFieldNames ? '' : 'isEnabled')
    ..aInt64(6, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateSettingsRequest copyWith(
          void Function(AiCreateSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiCreateSettingsRequest))
          as AiCreateSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiCreateSettingsRequest() / AiCreateSettingsRequest.new instead')
  static AiCreateSettingsRequest create() => AiCreateSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiCreateSettingsRequest._();
  @$core.override
  AiCreateSettingsRequest createEmptyInstance() => AiCreateSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiCreateSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiCreateSettingsRequest>(
          AiCreateSettingsRequest.$_createMessage);
  static AiCreateSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get apiUrl => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiUrl($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiUrl() => $_clearField(2);

  /// Secret stored server-side; never echoed back on reads.
  @$pb.TagNumber(3)
  $core.String get apiKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set apiKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasApiKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearApiKey() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get model => $_getSZ(3);
  @$pb.TagNumber(4)
  set model($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasModel() => $_has(3);
  @$pb.TagNumber(4)
  void clearModel() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isEnabled => $_getBF(4);
  @$pb.TagNumber(5)
  set isEnabled($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIsEnabled() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsEnabled() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get organizationId => $_getI64(5);
  @$pb.TagNumber(6)
  set organizationId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasOrganizationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearOrganizationId() => $_clearField(6);
}

class AiCreateSettingsResponse extends $pb.GeneratedMessage {
  factory AiCreateSettingsResponse({
    AiSetting? setting,
  }) {
    final result = AiCreateSettingsResponse._();
    if (setting != null) result.setting = setting;
    return result;
  }

  AiCreateSettingsResponse._();

  factory AiCreateSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateSettingsResponse()..mergeFromBuffer(data, registry);
  factory AiCreateSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiCreateSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiCreateSettingsResponse.$_createMessage)
    ..aOM<AiSetting>(1, _omitFieldNames ? '' : 'setting',
        subBuilder: AiSetting.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateSettingsResponse copyWith(
          void Function(AiCreateSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as AiCreateSettingsResponse))
          as AiCreateSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiCreateSettingsResponse() / AiCreateSettingsResponse.new instead')
  static AiCreateSettingsResponse create() => AiCreateSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiCreateSettingsResponse._();
  @$core.override
  AiCreateSettingsResponse createEmptyInstance() =>
      AiCreateSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiCreateSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiCreateSettingsResponse>(
          AiCreateSettingsResponse.$_createMessage);
  static AiCreateSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiSetting get setting => $_getN(0);
  @$pb.TagNumber(1)
  set setting(AiSetting value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSetting() => $_has(0);
  @$pb.TagNumber(1)
  void clearSetting() => $_clearField(1);
  @$pb.TagNumber(1)
  AiSetting ensureSetting() => $_ensure(0);
}

/// An absent `api_key` keeps the stored key.
class AiUpdateSettingsRequest extends $pb.GeneratedMessage {
  factory AiUpdateSettingsRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? apiUrl,
    $core.String? apiKey,
    $core.String? model,
    $core.bool? isEnabled,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiUpdateSettingsRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (apiKey != null) result.apiKey = apiKey;
    if (model != null) result.model = model;
    if (isEnabled != null) result.isEnabled = isEnabled;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiUpdateSettingsRequest._();

  factory AiUpdateSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiUpdateSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiUpdateSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiUpdateSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiUpdateSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiUpdateSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(4, _omitFieldNames ? '' : 'apiKey')
    ..aOS(5, _omitFieldNames ? '' : 'model')
    ..aOB(6, _omitFieldNames ? '' : 'isEnabled')
    ..aInt64(7, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiUpdateSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiUpdateSettingsRequest copyWith(
          void Function(AiUpdateSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiUpdateSettingsRequest))
          as AiUpdateSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiUpdateSettingsRequest() / AiUpdateSettingsRequest.new instead')
  static AiUpdateSettingsRequest create() => AiUpdateSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiUpdateSettingsRequest._();
  @$core.override
  AiUpdateSettingsRequest createEmptyInstance() => AiUpdateSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiUpdateSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiUpdateSettingsRequest>(
          AiUpdateSettingsRequest.$_createMessage);
  static AiUpdateSettingsRequest? _defaultInstance;

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
  $core.String get apiKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set apiKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasApiKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearApiKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get model => $_getSZ(4);
  @$pb.TagNumber(5)
  set model($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasModel() => $_has(4);
  @$pb.TagNumber(5)
  void clearModel() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get isEnabled => $_getBF(5);
  @$pb.TagNumber(6)
  set isEnabled($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasIsEnabled() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsEnabled() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get organizationId => $_getI64(6);
  @$pb.TagNumber(7)
  set organizationId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasOrganizationId() => $_has(6);
  @$pb.TagNumber(7)
  void clearOrganizationId() => $_clearField(7);
}

class AiUpdateSettingsResponse extends $pb.GeneratedMessage {
  factory AiUpdateSettingsResponse({
    AiSetting? setting,
  }) {
    final result = AiUpdateSettingsResponse._();
    if (setting != null) result.setting = setting;
    return result;
  }

  AiUpdateSettingsResponse._();

  factory AiUpdateSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiUpdateSettingsResponse()..mergeFromBuffer(data, registry);
  factory AiUpdateSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiUpdateSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiUpdateSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiUpdateSettingsResponse.$_createMessage)
    ..aOM<AiSetting>(1, _omitFieldNames ? '' : 'setting',
        subBuilder: AiSetting.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiUpdateSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiUpdateSettingsResponse copyWith(
          void Function(AiUpdateSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as AiUpdateSettingsResponse))
          as AiUpdateSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiUpdateSettingsResponse() / AiUpdateSettingsResponse.new instead')
  static AiUpdateSettingsResponse create() => AiUpdateSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiUpdateSettingsResponse._();
  @$core.override
  AiUpdateSettingsResponse createEmptyInstance() =>
      AiUpdateSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiUpdateSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiUpdateSettingsResponse>(
          AiUpdateSettingsResponse.$_createMessage);
  static AiUpdateSettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiSetting get setting => $_getN(0);
  @$pb.TagNumber(1)
  set setting(AiSetting value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSetting() => $_has(0);
  @$pb.TagNumber(1)
  void clearSetting() => $_clearField(1);
  @$pb.TagNumber(1)
  AiSetting ensureSetting() => $_ensure(0);
}

class AiDeleteSettingsRequest extends $pb.GeneratedMessage {
  factory AiDeleteSettingsRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiDeleteSettingsRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiDeleteSettingsRequest._();

  factory AiDeleteSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeleteSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiDeleteSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeleteSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDeleteSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDeleteSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeleteSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeleteSettingsRequest copyWith(
          void Function(AiDeleteSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiDeleteSettingsRequest))
          as AiDeleteSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiDeleteSettingsRequest() / AiDeleteSettingsRequest.new instead')
  static AiDeleteSettingsRequest create() => AiDeleteSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiDeleteSettingsRequest._();
  @$core.override
  AiDeleteSettingsRequest createEmptyInstance() => AiDeleteSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiDeleteSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDeleteSettingsRequest>(
          AiDeleteSettingsRequest.$_createMessage);
  static AiDeleteSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

/// Model names offered by the stored provider configuration.
class AiListModelsRequest extends $pb.GeneratedMessage {
  factory AiListModelsRequest({
    $fixnum.Int64? settingId,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiListModelsRequest._();
    if (settingId != null) result.settingId = settingId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiListModelsRequest._();

  factory AiListModelsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListModelsRequest()..mergeFromBuffer(data, registry);
  factory AiListModelsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListModelsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListModelsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListModelsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'settingId')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListModelsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListModelsRequest copyWith(void Function(AiListModelsRequest) updates) =>
      super.copyWith((message) => updates(message as AiListModelsRequest))
          as AiListModelsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use AiListModelsRequest() / AiListModelsRequest.new instead')
  static AiListModelsRequest create() => AiListModelsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiListModelsRequest._();
  @$core.override
  AiListModelsRequest createEmptyInstance() => AiListModelsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiListModelsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListModelsRequest>(
          AiListModelsRequest.$_createMessage);
  static AiListModelsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get settingId => $_getI64(0);
  @$pb.TagNumber(1)
  set settingId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSettingId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSettingId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class AiListModelsResponse extends $pb.GeneratedMessage {
  factory AiListModelsResponse({
    $core.Iterable<$core.String>? models,
  }) {
    final result = AiListModelsResponse._();
    if (models != null) result.models.addAll(models);
    return result;
  }

  AiListModelsResponse._();

  factory AiListModelsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListModelsResponse()..mergeFromBuffer(data, registry);
  factory AiListModelsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListModelsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListModelsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListModelsResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'models')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListModelsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListModelsResponse copyWith(void Function(AiListModelsResponse) updates) =>
      super.copyWith((message) => updates(message as AiListModelsResponse))
          as AiListModelsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiListModelsResponse() / AiListModelsResponse.new instead')
  static AiListModelsResponse create() => AiListModelsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiListModelsResponse._();
  @$core.override
  AiListModelsResponse createEmptyInstance() => AiListModelsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiListModelsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListModelsResponse>(
          AiListModelsResponse.$_createMessage);
  static AiListModelsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get models => $_getList(0);
}

class AiTestSettingsRequest extends $pb.GeneratedMessage {
  factory AiTestSettingsRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiTestSettingsRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiTestSettingsRequest._();

  factory AiTestSettingsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestSettingsRequest()..mergeFromBuffer(data, registry);
  factory AiTestSettingsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestSettingsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiTestSettingsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiTestSettingsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestSettingsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestSettingsRequest copyWith(
          void Function(AiTestSettingsRequest) updates) =>
      super.copyWith((message) => updates(message as AiTestSettingsRequest))
          as AiTestSettingsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiTestSettingsRequest() / AiTestSettingsRequest.new instead')
  static AiTestSettingsRequest create() => AiTestSettingsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiTestSettingsRequest._();
  @$core.override
  AiTestSettingsRequest createEmptyInstance() => AiTestSettingsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiTestSettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiTestSettingsRequest>(
          AiTestSettingsRequest.$_createMessage);
  static AiTestSettingsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class AiTestSettingsResponse extends $pb.GeneratedMessage {
  factory AiTestSettingsResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = AiTestSettingsResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  AiTestSettingsResponse._();

  factory AiTestSettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestSettingsResponse()..mergeFromBuffer(data, registry);
  factory AiTestSettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestSettingsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiTestSettingsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiTestSettingsResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestSettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestSettingsResponse copyWith(
          void Function(AiTestSettingsResponse) updates) =>
      super.copyWith((message) => updates(message as AiTestSettingsResponse))
          as AiTestSettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiTestSettingsResponse() / AiTestSettingsResponse.new instead')
  static AiTestSettingsResponse create() => AiTestSettingsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiTestSettingsResponse._();
  @$core.override
  AiTestSettingsResponse createEmptyInstance() => AiTestSettingsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiTestSettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiTestSettingsResponse>(
          AiTestSettingsResponse.$_createMessage);
  static AiTestSettingsResponse? _defaultInstance;

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

/// Model discovery against unsaved credentials (create dialog).
class AiDiscoverModelsRequest extends $pb.GeneratedMessage {
  factory AiDiscoverModelsRequest({
    $core.String? apiUrl,
    $core.String? apiKey,
  }) {
    final result = AiDiscoverModelsRequest._();
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (apiKey != null) result.apiKey = apiKey;
    return result;
  }

  AiDiscoverModelsRequest._();

  factory AiDiscoverModelsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDiscoverModelsRequest()..mergeFromBuffer(data, registry);
  factory AiDiscoverModelsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDiscoverModelsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDiscoverModelsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDiscoverModelsRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(2, _omitFieldNames ? '' : 'apiKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDiscoverModelsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDiscoverModelsRequest copyWith(
          void Function(AiDiscoverModelsRequest) updates) =>
      super.copyWith((message) => updates(message as AiDiscoverModelsRequest))
          as AiDiscoverModelsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiDiscoverModelsRequest() / AiDiscoverModelsRequest.new instead')
  static AiDiscoverModelsRequest create() => AiDiscoverModelsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiDiscoverModelsRequest._();
  @$core.override
  AiDiscoverModelsRequest createEmptyInstance() => AiDiscoverModelsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiDiscoverModelsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDiscoverModelsRequest>(
          AiDiscoverModelsRequest.$_createMessage);
  static AiDiscoverModelsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get apiUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set apiUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApiUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearApiUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get apiKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiKey() => $_clearField(2);
}

class AiDiscoverModelsResponse extends $pb.GeneratedMessage {
  factory AiDiscoverModelsResponse({
    $core.Iterable<$core.String>? models,
  }) {
    final result = AiDiscoverModelsResponse._();
    if (models != null) result.models.addAll(models);
    return result;
  }

  AiDiscoverModelsResponse._();

  factory AiDiscoverModelsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDiscoverModelsResponse()..mergeFromBuffer(data, registry);
  factory AiDiscoverModelsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDiscoverModelsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDiscoverModelsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDiscoverModelsResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'models')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDiscoverModelsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDiscoverModelsResponse copyWith(
          void Function(AiDiscoverModelsResponse) updates) =>
      super.copyWith((message) => updates(message as AiDiscoverModelsResponse))
          as AiDiscoverModelsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiDiscoverModelsResponse() / AiDiscoverModelsResponse.new instead')
  static AiDiscoverModelsResponse create() => AiDiscoverModelsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiDiscoverModelsResponse._();
  @$core.override
  AiDiscoverModelsResponse createEmptyInstance() =>
      AiDiscoverModelsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiDiscoverModelsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDiscoverModelsResponse>(
          AiDiscoverModelsResponse.$_createMessage);
  static AiDiscoverModelsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get models => $_getList(0);
}

/// Connectivity test against unsaved credentials.
class AiTestConnectionRequest extends $pb.GeneratedMessage {
  factory AiTestConnectionRequest({
    $core.String? apiUrl,
    $core.String? apiKey,
    $core.String? model,
  }) {
    final result = AiTestConnectionRequest._();
    if (apiUrl != null) result.apiUrl = apiUrl;
    if (apiKey != null) result.apiKey = apiKey;
    if (model != null) result.model = model;
    return result;
  }

  AiTestConnectionRequest._();

  factory AiTestConnectionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestConnectionRequest()..mergeFromBuffer(data, registry);
  factory AiTestConnectionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestConnectionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiTestConnectionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiTestConnectionRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'apiUrl')
    ..aOS(2, _omitFieldNames ? '' : 'apiKey')
    ..aOS(3, _omitFieldNames ? '' : 'model')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestConnectionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestConnectionRequest copyWith(
          void Function(AiTestConnectionRequest) updates) =>
      super.copyWith((message) => updates(message as AiTestConnectionRequest))
          as AiTestConnectionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiTestConnectionRequest() / AiTestConnectionRequest.new instead')
  static AiTestConnectionRequest create() => AiTestConnectionRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiTestConnectionRequest._();
  @$core.override
  AiTestConnectionRequest createEmptyInstance() => AiTestConnectionRequest._();
  @$core.pragma('dart2js:noInline')
  static AiTestConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiTestConnectionRequest>(
          AiTestConnectionRequest.$_createMessage);
  static AiTestConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get apiUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set apiUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApiUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearApiUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get apiKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set apiKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasApiKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearApiKey() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get model => $_getSZ(2);
  @$pb.TagNumber(3)
  set model($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasModel() => $_has(2);
  @$pb.TagNumber(3)
  void clearModel() => $_clearField(3);
}

class AiTestConnectionResponse extends $pb.GeneratedMessage {
  factory AiTestConnectionResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = AiTestConnectionResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  AiTestConnectionResponse._();

  factory AiTestConnectionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestConnectionResponse()..mergeFromBuffer(data, registry);
  factory AiTestConnectionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiTestConnectionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiTestConnectionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiTestConnectionResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestConnectionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiTestConnectionResponse copyWith(
          void Function(AiTestConnectionResponse) updates) =>
      super.copyWith((message) => updates(message as AiTestConnectionResponse))
          as AiTestConnectionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiTestConnectionResponse() / AiTestConnectionResponse.new instead')
  static AiTestConnectionResponse create() => AiTestConnectionResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiTestConnectionResponse._();
  @$core.override
  AiTestConnectionResponse createEmptyInstance() =>
      AiTestConnectionResponse._();
  @$core.pragma('dart2js:noInline')
  static AiTestConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiTestConnectionResponse>(
          AiTestConnectionResponse.$_createMessage);
  static AiTestConnectionResponse? _defaultInstance;

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

class AiListGenerationsRequest extends $pb.GeneratedMessage {
  factory AiListGenerationsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? limit,
  }) {
    final result = AiListGenerationsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (limit != null) result.limit = limit;
    return result;
  }

  AiListGenerationsRequest._();

  factory AiListGenerationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListGenerationsRequest()..mergeFromBuffer(data, registry);
  factory AiListGenerationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListGenerationsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListGenerationsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListGenerationsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListGenerationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListGenerationsRequest copyWith(
          void Function(AiListGenerationsRequest) updates) =>
      super.copyWith((message) => updates(message as AiListGenerationsRequest))
          as AiListGenerationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiListGenerationsRequest() / AiListGenerationsRequest.new instead')
  static AiListGenerationsRequest create() => AiListGenerationsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiListGenerationsRequest._();
  @$core.override
  AiListGenerationsRequest createEmptyInstance() =>
      AiListGenerationsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiListGenerationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListGenerationsRequest>(
          AiListGenerationsRequest.$_createMessage);
  static AiListGenerationsRequest? _defaultInstance;

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
}

class AiListGenerationsResponse extends $pb.GeneratedMessage {
  factory AiListGenerationsResponse({
    $core.Iterable<AiGeneration>? generations,
  }) {
    final result = AiListGenerationsResponse._();
    if (generations != null) result.generations.addAll(generations);
    return result;
  }

  AiListGenerationsResponse._();

  factory AiListGenerationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListGenerationsResponse()..mergeFromBuffer(data, registry);
  factory AiListGenerationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiListGenerationsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiListGenerationsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiListGenerationsResponse.$_createMessage)
    ..pPM<AiGeneration>(1, _omitFieldNames ? '' : 'generations',
        subBuilder: AiGeneration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListGenerationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiListGenerationsResponse copyWith(
          void Function(AiListGenerationsResponse) updates) =>
      super.copyWith((message) => updates(message as AiListGenerationsResponse))
          as AiListGenerationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiListGenerationsResponse() / AiListGenerationsResponse.new instead')
  static AiListGenerationsResponse create() => AiListGenerationsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiListGenerationsResponse._();
  @$core.override
  AiListGenerationsResponse createEmptyInstance() =>
      AiListGenerationsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiListGenerationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiListGenerationsResponse>(
          AiListGenerationsResponse.$_createMessage);
  static AiListGenerationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AiGeneration> get generations => $_getList(0);
}

class AiGetGenerationRequest extends $pb.GeneratedMessage {
  factory AiGetGenerationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = AiGetGenerationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  AiGetGenerationRequest._();

  factory AiGetGenerationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetGenerationRequest()..mergeFromBuffer(data, registry);
  factory AiGetGenerationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetGenerationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGetGenerationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGetGenerationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetGenerationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetGenerationRequest copyWith(
          void Function(AiGetGenerationRequest) updates) =>
      super.copyWith((message) => updates(message as AiGetGenerationRequest))
          as AiGetGenerationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiGetGenerationRequest() / AiGetGenerationRequest.new instead')
  static AiGetGenerationRequest create() => AiGetGenerationRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiGetGenerationRequest._();
  @$core.override
  AiGetGenerationRequest createEmptyInstance() => AiGetGenerationRequest._();
  @$core.pragma('dart2js:noInline')
  static AiGetGenerationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiGetGenerationRequest>(
          AiGetGenerationRequest.$_createMessage);
  static AiGetGenerationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class AiGetGenerationResponse extends $pb.GeneratedMessage {
  factory AiGetGenerationResponse({
    AiGeneration? generation,
  }) {
    final result = AiGetGenerationResponse._();
    if (generation != null) result.generation = generation;
    return result;
  }

  AiGetGenerationResponse._();

  factory AiGetGenerationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetGenerationResponse()..mergeFromBuffer(data, registry);
  factory AiGetGenerationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiGetGenerationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiGetGenerationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiGetGenerationResponse.$_createMessage)
    ..aOM<AiGeneration>(1, _omitFieldNames ? '' : 'generation',
        subBuilder: AiGeneration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetGenerationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiGetGenerationResponse copyWith(
          void Function(AiGetGenerationResponse) updates) =>
      super.copyWith((message) => updates(message as AiGetGenerationResponse))
          as AiGetGenerationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiGetGenerationResponse() / AiGetGenerationResponse.new instead')
  static AiGetGenerationResponse create() => AiGetGenerationResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiGetGenerationResponse._();
  @$core.override
  AiGetGenerationResponse createEmptyInstance() => AiGetGenerationResponse._();
  @$core.pragma('dart2js:noInline')
  static AiGetGenerationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiGetGenerationResponse>(
          AiGetGenerationResponse.$_createMessage);
  static AiGetGenerationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiGeneration get generation => $_getN(0);
  @$pb.TagNumber(1)
  set generation(AiGeneration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGeneration() => $_has(0);
  @$pb.TagNumber(1)
  void clearGeneration() => $_clearField(1);
  @$pb.TagNumber(1)
  AiGeneration ensureGeneration() => $_ensure(0);
}

class AiCreateGenerationRequest extends $pb.GeneratedMessage {
  factory AiCreateGenerationRequest({
    $fixnum.Int64? aiSettingId,
    $core.String? request,
  }) {
    final result = AiCreateGenerationRequest._();
    if (aiSettingId != null) result.aiSettingId = aiSettingId;
    if (request != null) result.request = request;
    return result;
  }

  AiCreateGenerationRequest._();

  factory AiCreateGenerationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateGenerationRequest()..mergeFromBuffer(data, registry);
  factory AiCreateGenerationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateGenerationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiCreateGenerationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiCreateGenerationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'aiSettingId')
    ..aOS(2, _omitFieldNames ? '' : 'request')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateGenerationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateGenerationRequest copyWith(
          void Function(AiCreateGenerationRequest) updates) =>
      super.copyWith((message) => updates(message as AiCreateGenerationRequest))
          as AiCreateGenerationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiCreateGenerationRequest() / AiCreateGenerationRequest.new instead')
  static AiCreateGenerationRequest create() => AiCreateGenerationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiCreateGenerationRequest._();
  @$core.override
  AiCreateGenerationRequest createEmptyInstance() =>
      AiCreateGenerationRequest._();
  @$core.pragma('dart2js:noInline')
  static AiCreateGenerationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiCreateGenerationRequest>(
          AiCreateGenerationRequest.$_createMessage);
  static AiCreateGenerationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get aiSettingId => $_getI64(0);
  @$pb.TagNumber(1)
  set aiSettingId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAiSettingId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAiSettingId() => $_clearField(1);

  /// Free-form natural-language prompt for the compose generation.
  @$pb.TagNumber(2)
  $core.String get request => $_getSZ(1);
  @$pb.TagNumber(2)
  set request($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearRequest() => $_clearField(2);
}

class AiCreateGenerationResponse extends $pb.GeneratedMessage {
  factory AiCreateGenerationResponse({
    AiGeneration? generation,
  }) {
    final result = AiCreateGenerationResponse._();
    if (generation != null) result.generation = generation;
    return result;
  }

  AiCreateGenerationResponse._();

  factory AiCreateGenerationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateGenerationResponse()..mergeFromBuffer(data, registry);
  factory AiCreateGenerationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiCreateGenerationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiCreateGenerationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiCreateGenerationResponse.$_createMessage)
    ..aOM<AiGeneration>(1, _omitFieldNames ? '' : 'generation',
        subBuilder: AiGeneration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateGenerationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiCreateGenerationResponse copyWith(
          void Function(AiCreateGenerationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as AiCreateGenerationResponse))
          as AiCreateGenerationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiCreateGenerationResponse() / AiCreateGenerationResponse.new instead')
  static AiCreateGenerationResponse create() => AiCreateGenerationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiCreateGenerationResponse._();
  @$core.override
  AiCreateGenerationResponse createEmptyInstance() =>
      AiCreateGenerationResponse._();
  @$core.pragma('dart2js:noInline')
  static AiCreateGenerationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiCreateGenerationResponse>(
          AiCreateGenerationResponse.$_createMessage);
  static AiCreateGenerationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiGeneration get generation => $_getN(0);
  @$pb.TagNumber(1)
  set generation(AiGeneration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGeneration() => $_has(0);
  @$pb.TagNumber(1)
  void clearGeneration() => $_clearField(1);
  @$pb.TagNumber(1)
  AiGeneration ensureGeneration() => $_ensure(0);
}

/// Full replacement of the generation output after human review.
class AiReviewGenerationRequest extends $pb.GeneratedMessage {
  factory AiReviewGenerationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    AiGenerationOutput? output,
  }) {
    final result = AiReviewGenerationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (output != null) result.output = output;
    return result;
  }

  AiReviewGenerationRequest._();

  factory AiReviewGenerationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiReviewGenerationRequest()..mergeFromBuffer(data, registry);
  factory AiReviewGenerationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiReviewGenerationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiReviewGenerationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiReviewGenerationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOM<AiGenerationOutput>(3, _omitFieldNames ? '' : 'output',
        subBuilder: AiGenerationOutput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiReviewGenerationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiReviewGenerationRequest copyWith(
          void Function(AiReviewGenerationRequest) updates) =>
      super.copyWith((message) => updates(message as AiReviewGenerationRequest))
          as AiReviewGenerationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiReviewGenerationRequest() / AiReviewGenerationRequest.new instead')
  static AiReviewGenerationRequest create() => AiReviewGenerationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiReviewGenerationRequest._();
  @$core.override
  AiReviewGenerationRequest createEmptyInstance() =>
      AiReviewGenerationRequest._();
  @$core.pragma('dart2js:noInline')
  static AiReviewGenerationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiReviewGenerationRequest>(
          AiReviewGenerationRequest.$_createMessage);
  static AiReviewGenerationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  AiGenerationOutput get output => $_getN(2);
  @$pb.TagNumber(3)
  set output(AiGenerationOutput value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasOutput() => $_has(2);
  @$pb.TagNumber(3)
  void clearOutput() => $_clearField(3);
  @$pb.TagNumber(3)
  AiGenerationOutput ensureOutput() => $_ensure(2);
}

class AiReviewGenerationResponse extends $pb.GeneratedMessage {
  factory AiReviewGenerationResponse({
    AiGeneration? generation,
  }) {
    final result = AiReviewGenerationResponse._();
    if (generation != null) result.generation = generation;
    return result;
  }

  AiReviewGenerationResponse._();

  factory AiReviewGenerationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiReviewGenerationResponse()..mergeFromBuffer(data, registry);
  factory AiReviewGenerationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiReviewGenerationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiReviewGenerationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiReviewGenerationResponse.$_createMessage)
    ..aOM<AiGeneration>(1, _omitFieldNames ? '' : 'generation',
        subBuilder: AiGeneration.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiReviewGenerationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiReviewGenerationResponse copyWith(
          void Function(AiReviewGenerationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as AiReviewGenerationResponse))
          as AiReviewGenerationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiReviewGenerationResponse() / AiReviewGenerationResponse.new instead')
  static AiReviewGenerationResponse create() => AiReviewGenerationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiReviewGenerationResponse._();
  @$core.override
  AiReviewGenerationResponse createEmptyInstance() =>
      AiReviewGenerationResponse._();
  @$core.pragma('dart2js:noInline')
  static AiReviewGenerationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiReviewGenerationResponse>(
          AiReviewGenerationResponse.$_createMessage);
  static AiReviewGenerationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiGeneration get generation => $_getN(0);
  @$pb.TagNumber(1)
  set generation(AiGeneration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGeneration() => $_has(0);
  @$pb.TagNumber(1)
  void clearGeneration() => $_clearField(1);
  @$pb.TagNumber(1)
  AiGeneration ensureGeneration() => $_ensure(0);
}

class AiDeployGenerationRequest extends $pb.GeneratedMessage {
  factory AiDeployGenerationRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $core.String? suggestionId,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $core.bool? deployNow,
  }) {
    final result = AiDeployGenerationRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (suggestionId != null) result.suggestionId = suggestionId;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (deployNow != null) result.deployNow = deployNow;
    return result;
  }

  AiDeployGenerationRequest._();

  factory AiDeployGenerationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeployGenerationRequest()..mergeFromBuffer(data, registry);
  factory AiDeployGenerationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeployGenerationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDeployGenerationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDeployGenerationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'suggestionId')
    ..aInt64(4, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(5, _omitFieldNames ? '' : 'serverId')
    ..aOB(6, _omitFieldNames ? '' : 'deployNow')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeployGenerationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeployGenerationRequest copyWith(
          void Function(AiDeployGenerationRequest) updates) =>
      super.copyWith((message) => updates(message as AiDeployGenerationRequest))
          as AiDeployGenerationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiDeployGenerationRequest() / AiDeployGenerationRequest.new instead')
  static AiDeployGenerationRequest create() => AiDeployGenerationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiDeployGenerationRequest._();
  @$core.override
  AiDeployGenerationRequest createEmptyInstance() =>
      AiDeployGenerationRequest._();
  @$core.pragma('dart2js:noInline')
  static AiDeployGenerationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDeployGenerationRequest>(
          AiDeployGenerationRequest.$_createMessage);
  static AiDeployGenerationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  /// Selects one suggestion from the reviewed generation output.
  @$pb.TagNumber(3)
  $core.String get suggestionId => $_getSZ(2);
  @$pb.TagNumber(3)
  set suggestionId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSuggestionId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSuggestionId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get environmentId => $_getI64(3);
  @$pb.TagNumber(4)
  set environmentId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnvironmentId() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnvironmentId() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get serverId => $_getI64(4);
  @$pb.TagNumber(5)
  set serverId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServerId() => $_has(4);
  @$pb.TagNumber(5)
  void clearServerId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get deployNow => $_getBF(5);
  @$pb.TagNumber(6)
  set deployNow($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDeployNow() => $_has(5);
  @$pb.TagNumber(6)
  void clearDeployNow() => $_clearField(6);
}

class AiDeployGenerationResponse extends $pb.GeneratedMessage {
  factory AiDeployGenerationResponse({
    AiGeneration? generation,
    $fixnum.Int64? composeId,
    $fixnum.Int64? deploymentId,
  }) {
    final result = AiDeployGenerationResponse._();
    if (generation != null) result.generation = generation;
    if (composeId != null) result.composeId = composeId;
    if (deploymentId != null) result.deploymentId = deploymentId;
    return result;
  }

  AiDeployGenerationResponse._();

  factory AiDeployGenerationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeployGenerationResponse()..mergeFromBuffer(data, registry);
  factory AiDeployGenerationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiDeployGenerationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiDeployGenerationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiDeployGenerationResponse.$_createMessage)
    ..aOM<AiGeneration>(1, _omitFieldNames ? '' : 'generation',
        subBuilder: AiGeneration.$_createMessage)
    ..aInt64(2, _omitFieldNames ? '' : 'composeId')
    ..aInt64(3, _omitFieldNames ? '' : 'deploymentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeployGenerationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiDeployGenerationResponse copyWith(
          void Function(AiDeployGenerationResponse) updates) =>
      super.copyWith(
              (message) => updates(message as AiDeployGenerationResponse))
          as AiDeployGenerationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiDeployGenerationResponse() / AiDeployGenerationResponse.new instead')
  static AiDeployGenerationResponse create() => AiDeployGenerationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      AiDeployGenerationResponse._();
  @$core.override
  AiDeployGenerationResponse createEmptyInstance() =>
      AiDeployGenerationResponse._();
  @$core.pragma('dart2js:noInline')
  static AiDeployGenerationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiDeployGenerationResponse>(
          AiDeployGenerationResponse.$_createMessage);
  static AiDeployGenerationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  AiGeneration get generation => $_getN(0);
  @$pb.TagNumber(1)
  set generation(AiGeneration value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGeneration() => $_has(0);
  @$pb.TagNumber(1)
  void clearGeneration() => $_clearField(1);
  @$pb.TagNumber(1)
  AiGeneration ensureGeneration() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get composeId => $_getI64(1);
  @$pb.TagNumber(2)
  set composeId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasComposeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearComposeId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get deploymentId => $_getI64(2);
  @$pb.TagNumber(3)
  set deploymentId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDeploymentId() => $_has(2);
  @$pb.TagNumber(3)
  void clearDeploymentId() => $_clearField(3);
}

class AiAnalyzeLogsRequest extends $pb.GeneratedMessage {
  factory AiAnalyzeLogsRequest({
    $fixnum.Int64? aiSettingId,
    AiLogContext? context,
    $core.String? logs,
  }) {
    final result = AiAnalyzeLogsRequest._();
    if (aiSettingId != null) result.aiSettingId = aiSettingId;
    if (context != null) result.context = context;
    if (logs != null) result.logs = logs;
    return result;
  }

  AiAnalyzeLogsRequest._();

  factory AiAnalyzeLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAnalyzeLogsRequest()..mergeFromBuffer(data, registry);
  factory AiAnalyzeLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAnalyzeLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiAnalyzeLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiAnalyzeLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'aiSettingId')
    ..aE<AiLogContext>(2, _omitFieldNames ? '' : 'context',
        enumValues: AiLogContext.values)
    ..aOS(3, _omitFieldNames ? '' : 'logs')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAnalyzeLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAnalyzeLogsRequest copyWith(void Function(AiAnalyzeLogsRequest) updates) =>
      super.copyWith((message) => updates(message as AiAnalyzeLogsRequest))
          as AiAnalyzeLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiAnalyzeLogsRequest() / AiAnalyzeLogsRequest.new instead')
  static AiAnalyzeLogsRequest create() => AiAnalyzeLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiAnalyzeLogsRequest._();
  @$core.override
  AiAnalyzeLogsRequest createEmptyInstance() => AiAnalyzeLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static AiAnalyzeLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiAnalyzeLogsRequest>(
          AiAnalyzeLogsRequest.$_createMessage);
  static AiAnalyzeLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get aiSettingId => $_getI64(0);
  @$pb.TagNumber(1)
  set aiSettingId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAiSettingId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAiSettingId() => $_clearField(1);

  @$pb.TagNumber(2)
  AiLogContext get context => $_getN(1);
  @$pb.TagNumber(2)
  set context(AiLogContext value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasContext() => $_has(1);
  @$pb.TagNumber(2)
  void clearContext() => $_clearField(2);

  /// Raw log corpus sent to the model for analysis.
  @$pb.TagNumber(3)
  $core.String get logs => $_getSZ(2);
  @$pb.TagNumber(3)
  set logs($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLogs() => $_has(2);
  @$pb.TagNumber(3)
  void clearLogs() => $_clearField(3);
}

class AiAnalyzeLogsResponse extends $pb.GeneratedMessage {
  factory AiAnalyzeLogsResponse({
    $core.String? analysis,
  }) {
    final result = AiAnalyzeLogsResponse._();
    if (analysis != null) result.analysis = analysis;
    return result;
  }

  AiAnalyzeLogsResponse._();

  factory AiAnalyzeLogsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAnalyzeLogsResponse()..mergeFromBuffer(data, registry);
  factory AiAnalyzeLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAnalyzeLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiAnalyzeLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiAnalyzeLogsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'analysis')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAnalyzeLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAnalyzeLogsResponse copyWith(
          void Function(AiAnalyzeLogsResponse) updates) =>
      super.copyWith((message) => updates(message as AiAnalyzeLogsResponse))
          as AiAnalyzeLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use AiAnalyzeLogsResponse() / AiAnalyzeLogsResponse.new instead')
  static AiAnalyzeLogsResponse create() => AiAnalyzeLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiAnalyzeLogsResponse._();
  @$core.override
  AiAnalyzeLogsResponse createEmptyInstance() => AiAnalyzeLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static AiAnalyzeLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiAnalyzeLogsResponse>(
          AiAnalyzeLogsResponse.$_createMessage);
  static AiAnalyzeLogsResponse? _defaultInstance;

  /// Model-authored analysis text (markdown; not structured JSON).
  @$pb.TagNumber(1)
  $core.String get analysis => $_getSZ(0);
  @$pb.TagNumber(1)
  set analysis($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAnalysis() => $_has(0);
  @$pb.TagNumber(1)
  void clearAnalysis() => $_clearField(1);
}

class AiAssistantRequest extends $pb.GeneratedMessage {
  factory AiAssistantRequest({
    $core.String? prompt,
    $fixnum.Int64? aiSettingId,
    $core.Iterable<AiChatTurn>? history,
    $core.String? pendingActions,
  }) {
    final result = AiAssistantRequest._();
    if (prompt != null) result.prompt = prompt;
    if (aiSettingId != null) result.aiSettingId = aiSettingId;
    if (history != null) result.history.addAll(history);
    if (pendingActions != null) result.pendingActions = pendingActions;
    return result;
  }

  AiAssistantRequest._();

  factory AiAssistantRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAssistantRequest()..mergeFromBuffer(data, registry);
  factory AiAssistantRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAssistantRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiAssistantRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiAssistantRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'prompt')
    ..aInt64(2, _omitFieldNames ? '' : 'aiSettingId')
    ..pPM<AiChatTurn>(3, _omitFieldNames ? '' : 'history',
        subBuilder: AiChatTurn.$_createMessage)
    ..aOS(4, _omitFieldNames ? '' : 'pendingActions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAssistantRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAssistantRequest copyWith(void Function(AiAssistantRequest) updates) =>
      super.copyWith((message) => updates(message as AiAssistantRequest))
          as AiAssistantRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiAssistantRequest() / AiAssistantRequest.new instead')
  static AiAssistantRequest create() => AiAssistantRequest._();
  static $pb.GeneratedMessage $_createMessage() => AiAssistantRequest._();
  @$core.override
  AiAssistantRequest createEmptyInstance() => AiAssistantRequest._();
  @$core.pragma('dart2js:noInline')
  static AiAssistantRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiAssistantRequest>(
          AiAssistantRequest.$_createMessage);
  static AiAssistantRequest? _defaultInstance;

  /// Natural-language instruction, e.g. "deploy blinko".
  @$pb.TagNumber(1)
  $core.String get prompt => $_getSZ(0);
  @$pb.TagNumber(1)
  set prompt($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPrompt() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrompt() => $_clearField(1);

  /// Optional provider override; falls back to the org's first enabled setting.
  @$pb.TagNumber(2)
  $fixnum.Int64 get aiSettingId => $_getI64(1);
  @$pb.TagNumber(2)
  set aiSettingId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAiSettingId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAiSettingId() => $_clearField(2);

  /// Prior turns of this conversation so the assistant keeps context across
  /// messages (e.g. a "yes" that confirms the previous destructive action).
  @$pb.TagNumber(3)
  $pb.PbList<AiChatTurn> get history => $_getList(2);

  /// Opaque JSON carried back from a prior CONFIRM interaction's
  /// `pending_actions`; when the user confirms, the server executes these
  /// deterministically instead of re-asking the model.
  @$pb.TagNumber(4)
  $core.String get pendingActions => $_getSZ(3);
  @$pb.TagNumber(4)
  set pendingActions($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPendingActions() => $_has(3);
  @$pb.TagNumber(4)
  void clearPendingActions() => $_clearField(4);
}

/// One prior conversational turn replayed to the assistant for context.
class AiChatTurn extends $pb.GeneratedMessage {
  factory AiChatTurn({
    $core.String? role,
    $core.String? content,
  }) {
    final result = AiChatTurn._();
    if (role != null) result.role = role;
    if (content != null) result.content = content;
    return result;
  }

  AiChatTurn._();

  factory AiChatTurn.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiChatTurn()..mergeFromBuffer(data, registry);
  factory AiChatTurn.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiChatTurn()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiChatTurn',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiChatTurn.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'role')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiChatTurn clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiChatTurn copyWith(void Function(AiChatTurn) updates) =>
      super.copyWith((message) => updates(message as AiChatTurn)) as AiChatTurn;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiChatTurn() / AiChatTurn.new instead')
  static AiChatTurn create() => AiChatTurn._();
  static $pb.GeneratedMessage $_createMessage() => AiChatTurn._();
  @$core.override
  AiChatTurn createEmptyInstance() => AiChatTurn._();
  @$core.pragma('dart2js:noInline')
  static AiChatTurn getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiChatTurn>(AiChatTurn.$_createMessage);
  static AiChatTurn? _defaultInstance;

  /// "user" or "assistant".
  @$pb.TagNumber(1)
  $core.String get role => $_getSZ(0);
  @$pb.TagNumber(1)
  set role($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);
}

class AiAssistantResponse extends $pb.GeneratedMessage {
  factory AiAssistantResponse({
    $core.String? reply,
    $core.String? pendingConfirmation,
    AiInteraction? interaction,
  }) {
    final result = AiAssistantResponse._();
    if (reply != null) result.reply = reply;
    if (pendingConfirmation != null)
      result.pendingConfirmation = pendingConfirmation;
    if (interaction != null) result.interaction = interaction;
    return result;
  }

  AiAssistantResponse._();

  factory AiAssistantResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAssistantResponse()..mergeFromBuffer(data, registry);
  factory AiAssistantResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiAssistantResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiAssistantResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiAssistantResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'reply')
    ..aOS(2, _omitFieldNames ? '' : 'pendingConfirmation')
    ..aOM<AiInteraction>(3, _omitFieldNames ? '' : 'interaction',
        subBuilder: AiInteraction.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAssistantResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiAssistantResponse copyWith(void Function(AiAssistantResponse) updates) =>
      super.copyWith((message) => updates(message as AiAssistantResponse))
          as AiAssistantResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use AiAssistantResponse() / AiAssistantResponse.new instead')
  static AiAssistantResponse create() => AiAssistantResponse._();
  static $pb.GeneratedMessage $_createMessage() => AiAssistantResponse._();
  @$core.override
  AiAssistantResponse createEmptyInstance() => AiAssistantResponse._();
  @$core.pragma('dart2js:noInline')
  static AiAssistantResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiAssistantResponse>(
          AiAssistantResponse.$_createMessage);
  static AiAssistantResponse? _defaultInstance;

  /// Natural-language reply to show the user.
  @$pb.TagNumber(1)
  $core.String get reply => $_getSZ(0);
  @$pb.TagNumber(1)
  set reply($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasReply() => $_has(0);
  @$pb.TagNumber(1)
  void clearReply() => $_clearField(1);

  /// Set when the model is asking the user to confirm a destructive action.
  @$pb.TagNumber(2)
  $core.String get pendingConfirmation => $_getSZ(1);
  @$pb.TagNumber(2)
  set pendingConfirmation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPendingConfirmation() => $_has(1);
  @$pb.TagNumber(2)
  void clearPendingConfirmation() => $_clearField(2);

  /// Structured interaction the UI should render (Yes/No or option cards)
  /// instead of expecting free text back.
  @$pb.TagNumber(3)
  AiInteraction get interaction => $_getN(2);
  @$pb.TagNumber(3)
  set interaction(AiInteraction value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasInteraction() => $_has(2);
  @$pb.TagNumber(3)
  void clearInteraction() => $_clearField(3);
  @$pb.TagNumber(3)
  AiInteraction ensureInteraction() => $_ensure(2);
}

class AiInteractionOption extends $pb.GeneratedMessage {
  factory AiInteractionOption({
    $core.String? id,
    $core.String? label,
    $core.String? description,
  }) {
    final result = AiInteractionOption._();
    if (id != null) result.id = id;
    if (label != null) result.label = label;
    if (description != null) result.description = description;
    return result;
  }

  AiInteractionOption._();

  factory AiInteractionOption.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiInteractionOption()..mergeFromBuffer(data, registry);
  factory AiInteractionOption.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiInteractionOption()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiInteractionOption',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiInteractionOption.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'label')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiInteractionOption clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiInteractionOption copyWith(void Function(AiInteractionOption) updates) =>
      super.copyWith((message) => updates(message as AiInteractionOption))
          as AiInteractionOption;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use AiInteractionOption() / AiInteractionOption.new instead')
  static AiInteractionOption create() => AiInteractionOption._();
  static $pb.GeneratedMessage $_createMessage() => AiInteractionOption._();
  @$core.override
  AiInteractionOption createEmptyInstance() => AiInteractionOption._();
  @$core.pragma('dart2js:noInline')
  static AiInteractionOption getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AiInteractionOption>(
          AiInteractionOption.$_createMessage);
  static AiInteractionOption? _defaultInstance;

  /// Stable value sent back when chosen (id or canonical name).
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get label => $_getSZ(1);
  @$pb.TagNumber(2)
  set label($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLabel() => $_has(1);
  @$pb.TagNumber(2)
  void clearLabel() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);
}

class AiInteraction extends $pb.GeneratedMessage {
  factory AiInteraction({
    AiInteractionKind? kind,
    $core.String? question,
    $core.Iterable<AiInteractionOption>? options,
    $core.String? pendingActions,
  }) {
    final result = AiInteraction._();
    if (kind != null) result.kind = kind;
    if (question != null) result.question = question;
    if (options != null) result.options.addAll(options);
    if (pendingActions != null) result.pendingActions = pendingActions;
    return result;
  }

  AiInteraction._();

  factory AiInteraction.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiInteraction()..mergeFromBuffer(data, registry);
  factory AiInteraction.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AiInteraction()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AiInteraction',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AiInteraction.$_createMessage)
    ..aE<AiInteractionKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: AiInteractionKind.values)
    ..aOS(2, _omitFieldNames ? '' : 'question')
    ..pPM<AiInteractionOption>(3, _omitFieldNames ? '' : 'options',
        subBuilder: AiInteractionOption.$_createMessage)
    ..aOS(4, _omitFieldNames ? '' : 'pendingActions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiInteraction clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AiInteraction copyWith(void Function(AiInteraction) updates) =>
      super.copyWith((message) => updates(message as AiInteraction))
          as AiInteraction;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AiInteraction() / AiInteraction.new instead')
  static AiInteraction create() => AiInteraction._();
  static $pb.GeneratedMessage $_createMessage() => AiInteraction._();
  @$core.override
  AiInteraction createEmptyInstance() => AiInteraction._();
  @$core.pragma('dart2js:noInline')
  static AiInteraction getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AiInteraction>(
          AiInteraction.$_createMessage);
  static AiInteraction? _defaultInstance;

  @$pb.TagNumber(1)
  AiInteractionKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(AiInteractionKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  /// Question/summary shown above the controls.
  @$pb.TagNumber(2)
  $core.String get question => $_getSZ(1);
  @$pb.TagNumber(2)
  set question($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasQuestion() => $_has(1);
  @$pb.TagNumber(2)
  void clearQuestion() => $_clearField(2);

  /// Selectable options (CHOICE only; empty for CONFIRM).
  @$pb.TagNumber(3)
  $pb.PbList<AiInteractionOption> get options => $_getList(2);

  /// CONFIRM only: opaque JSON the client echoes back as the request's
  /// `pending_actions` when the user clicks Yes.
  @$pb.TagNumber(4)
  $core.String get pendingActions => $_getSZ(3);
  @$pb.TagNumber(4)
  set pendingActions($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPendingActions() => $_has(3);
  @$pb.TagNumber(4)
  void clearPendingActions() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/git_provider.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'git_provider.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'git_provider.pbenum.dart';

/// Non-secret config projection of a stored provider; secrets are masked to `has_*` presence flags server-side.
class GitProviderConfig extends $pb.GeneratedMessage {
  factory GitProviderConfig({
    $core.String? url,
    $core.String? internalUrl,
    $core.String? appName,
    $fixnum.Int64? appId,
    $core.String? clientId,
    $core.String? installationId,
    $core.String? applicationId,
    $core.String? redirectUri,
    $core.String? groupName,
    $core.String? scopes,
    $core.String? username,
    $core.String? email,
    $core.String? workspace,
    $core.bool? hasClientSecret,
    $core.bool? hasPrivateKey,
    $core.bool? hasAccessToken,
    $core.bool? hasAppPassword,
    $core.bool? hasApiToken,
  }) {
    final result = GitProviderConfig._();
    if (url != null) result.url = url;
    if (internalUrl != null) result.internalUrl = internalUrl;
    if (appName != null) result.appName = appName;
    if (appId != null) result.appId = appId;
    if (clientId != null) result.clientId = clientId;
    if (installationId != null) result.installationId = installationId;
    if (applicationId != null) result.applicationId = applicationId;
    if (redirectUri != null) result.redirectUri = redirectUri;
    if (groupName != null) result.groupName = groupName;
    if (scopes != null) result.scopes = scopes;
    if (username != null) result.username = username;
    if (email != null) result.email = email;
    if (workspace != null) result.workspace = workspace;
    if (hasClientSecret != null) result.hasClientSecret = hasClientSecret;
    if (hasPrivateKey != null) result.hasPrivateKey = hasPrivateKey;
    if (hasAccessToken != null) result.hasAccessToken = hasAccessToken;
    if (hasAppPassword != null) result.hasAppPassword = hasAppPassword;
    if (hasApiToken != null) result.hasApiToken = hasApiToken;
    return result;
  }

  GitProviderConfig._();

  factory GitProviderConfig.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderConfig()..mergeFromBuffer(data, registry);
  factory GitProviderConfig.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderConfig()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProviderConfig',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProviderConfig.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'url')
    ..aOS(2, _omitFieldNames ? '' : 'internalUrl')
    ..aOS(3, _omitFieldNames ? '' : 'appName')
    ..aInt64(4, _omitFieldNames ? '' : 'appId')
    ..aOS(5, _omitFieldNames ? '' : 'clientId')
    ..aOS(6, _omitFieldNames ? '' : 'installationId')
    ..aOS(7, _omitFieldNames ? '' : 'applicationId')
    ..aOS(8, _omitFieldNames ? '' : 'redirectUri')
    ..aOS(9, _omitFieldNames ? '' : 'groupName')
    ..aOS(10, _omitFieldNames ? '' : 'scopes')
    ..aOS(11, _omitFieldNames ? '' : 'username')
    ..aOS(12, _omitFieldNames ? '' : 'email')
    ..aOS(13, _omitFieldNames ? '' : 'workspace')
    ..aOB(14, _omitFieldNames ? '' : 'hasClientSecret')
    ..aOB(15, _omitFieldNames ? '' : 'hasPrivateKey')
    ..aOB(16, _omitFieldNames ? '' : 'hasAccessToken')
    ..aOB(17, _omitFieldNames ? '' : 'hasAppPassword')
    ..aOB(18, _omitFieldNames ? '' : 'hasApiToken')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderConfig clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderConfig copyWith(void Function(GitProviderConfig) updates) =>
      super.copyWith((message) => updates(message as GitProviderConfig))
          as GitProviderConfig;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GitProviderConfig() / GitProviderConfig.new instead')
  static GitProviderConfig create() => GitProviderConfig._();
  static $pb.GeneratedMessage $_createMessage() => GitProviderConfig._();
  @$core.override
  GitProviderConfig createEmptyInstance() => GitProviderConfig._();
  @$core.pragma('dart2js:noInline')
  static GitProviderConfig getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GitProviderConfig>(
          GitProviderConfig.$_createMessage);
  static GitProviderConfig? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get url => $_getSZ(0);
  @$pb.TagNumber(1)
  set url($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get internalUrl => $_getSZ(1);
  @$pb.TagNumber(2)
  set internalUrl($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasInternalUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearInternalUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get appName => $_getSZ(2);
  @$pb.TagNumber(3)
  set appName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAppName() => $_has(2);
  @$pb.TagNumber(3)
  void clearAppName() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get appId => $_getI64(3);
  @$pb.TagNumber(4)
  set appId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAppId() => $_has(3);
  @$pb.TagNumber(4)
  void clearAppId() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get clientId => $_getSZ(4);
  @$pb.TagNumber(5)
  set clientId($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasClientId() => $_has(4);
  @$pb.TagNumber(5)
  void clearClientId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get installationId => $_getSZ(5);
  @$pb.TagNumber(6)
  set installationId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasInstallationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearInstallationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get applicationId => $_getSZ(6);
  @$pb.TagNumber(7)
  set applicationId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasApplicationId() => $_has(6);
  @$pb.TagNumber(7)
  void clearApplicationId() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get redirectUri => $_getSZ(7);
  @$pb.TagNumber(8)
  set redirectUri($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasRedirectUri() => $_has(7);
  @$pb.TagNumber(8)
  void clearRedirectUri() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get groupName => $_getSZ(8);
  @$pb.TagNumber(9)
  set groupName($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasGroupName() => $_has(8);
  @$pb.TagNumber(9)
  void clearGroupName() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get scopes => $_getSZ(9);
  @$pb.TagNumber(10)
  set scopes($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasScopes() => $_has(9);
  @$pb.TagNumber(10)
  void clearScopes() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get username => $_getSZ(10);
  @$pb.TagNumber(11)
  set username($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUsername() => $_has(10);
  @$pb.TagNumber(11)
  void clearUsername() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get email => $_getSZ(11);
  @$pb.TagNumber(12)
  set email($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasEmail() => $_has(11);
  @$pb.TagNumber(12)
  void clearEmail() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get workspace => $_getSZ(12);
  @$pb.TagNumber(13)
  set workspace($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasWorkspace() => $_has(12);
  @$pb.TagNumber(13)
  void clearWorkspace() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.bool get hasClientSecret => $_getBF(13);
  @$pb.TagNumber(14)
  set hasClientSecret($core.bool value) => $_setBool(13, value);
  @$pb.TagNumber(14)
  $core.bool hasHasClientSecret() => $_has(13);
  @$pb.TagNumber(14)
  void clearHasClientSecret() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.bool get hasPrivateKey => $_getBF(14);
  @$pb.TagNumber(15)
  set hasPrivateKey($core.bool value) => $_setBool(14, value);
  @$pb.TagNumber(15)
  $core.bool hasHasPrivateKey() => $_has(14);
  @$pb.TagNumber(15)
  void clearHasPrivateKey() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.bool get hasAccessToken => $_getBF(15);
  @$pb.TagNumber(16)
  set hasAccessToken($core.bool value) => $_setBool(15, value);
  @$pb.TagNumber(16)
  $core.bool hasHasAccessToken() => $_has(15);
  @$pb.TagNumber(16)
  void clearHasAccessToken() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.bool get hasAppPassword => $_getBF(16);
  @$pb.TagNumber(17)
  set hasAppPassword($core.bool value) => $_setBool(16, value);
  @$pb.TagNumber(17)
  $core.bool hasHasAppPassword() => $_has(16);
  @$pb.TagNumber(17)
  void clearHasAppPassword() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.bool get hasApiToken => $_getBF(17);
  @$pb.TagNumber(18)
  set hasApiToken($core.bool value) => $_setBool(17, value);
  @$pb.TagNumber(18)
  $core.bool hasHasApiToken() => $_has(17);
  @$pb.TagNumber(18)
  void clearHasApiToken() => $_clearField(18);
}

class GitProvider extends $pb.GeneratedMessage {
  factory GitProvider({
    $fixnum.Int64? id,
    $core.String? name,
    GitProviderType? providerType,
    $core.bool? shared,
    $core.bool? configured,
    $core.bool? webhookConfigured,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    GitProviderConfig? config,
  }) {
    final result = GitProvider._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (providerType != null) result.providerType = providerType;
    if (shared != null) result.shared = shared;
    if (configured != null) result.configured = configured;
    if (webhookConfigured != null) result.webhookConfigured = webhookConfigured;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (config != null) result.config = config;
    return result;
  }

  GitProvider._();

  factory GitProvider.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProvider()..mergeFromBuffer(data, registry);
  factory GitProvider.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProvider()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProvider',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProvider.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aE<GitProviderType>(3, _omitFieldNames ? '' : 'providerType',
        enumValues: GitProviderType.values)
    ..aOB(4, _omitFieldNames ? '' : 'shared')
    ..aOB(5, _omitFieldNames ? '' : 'configured')
    ..aOB(6, _omitFieldNames ? '' : 'webhookConfigured')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..aOM<GitProviderConfig>(9, _omitFieldNames ? '' : 'config',
        subBuilder: GitProviderConfig.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProvider clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProvider copyWith(void Function(GitProvider) updates) =>
      super.copyWith((message) => updates(message as GitProvider))
          as GitProvider;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GitProvider() / GitProvider.new instead')
  static GitProvider create() => GitProvider._();
  static $pb.GeneratedMessage $_createMessage() => GitProvider._();
  @$core.override
  GitProvider createEmptyInstance() => GitProvider._();
  @$core.pragma('dart2js:noInline')
  static GitProvider getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GitProvider>(
          GitProvider.$_createMessage);
  static GitProvider? _defaultInstance;

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
  GitProviderType get providerType => $_getN(2);
  @$pb.TagNumber(3)
  set providerType(GitProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProviderType() => $_has(2);
  @$pb.TagNumber(3)
  void clearProviderType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get shared => $_getBF(3);
  @$pb.TagNumber(4)
  set shared($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasShared() => $_has(3);
  @$pb.TagNumber(4)
  void clearShared() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get configured => $_getBF(4);
  @$pb.TagNumber(5)
  set configured($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasConfigured() => $_has(4);
  @$pb.TagNumber(5)
  void clearConfigured() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get webhookConfigured => $_getBF(5);
  @$pb.TagNumber(6)
  set webhookConfigured($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasWebhookConfigured() => $_has(5);
  @$pb.TagNumber(6)
  void clearWebhookConfigured() => $_clearField(6);

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

  @$pb.TagNumber(9)
  GitProviderConfig get config => $_getN(8);
  @$pb.TagNumber(9)
  set config(GitProviderConfig value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasConfig() => $_has(8);
  @$pb.TagNumber(9)
  void clearConfig() => $_clearField(9);
  @$pb.TagNumber(9)
  GitProviderConfig ensureConfig() => $_ensure(8);
}

/// Repository summary returned by provider discovery (`RepositoryInfo`).
class GitProviderRepository extends $pb.GeneratedMessage {
  factory GitProviderRepository({
    $core.String? owner,
    $core.String? name,
    $core.String? fullName,
    $core.bool? private,
    $core.String? defaultBranch,
    $core.String? cloneUrl,
  }) {
    final result = GitProviderRepository._();
    if (owner != null) result.owner = owner;
    if (name != null) result.name = name;
    if (fullName != null) result.fullName = fullName;
    if (private != null) result.private = private;
    if (defaultBranch != null) result.defaultBranch = defaultBranch;
    if (cloneUrl != null) result.cloneUrl = cloneUrl;
    return result;
  }

  GitProviderRepository._();

  factory GitProviderRepository.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderRepository()..mergeFromBuffer(data, registry);
  factory GitProviderRepository.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderRepository()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProviderRepository',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProviderRepository.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'owner')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'fullName')
    ..aOB(4, _omitFieldNames ? '' : 'private')
    ..aOS(5, _omitFieldNames ? '' : 'defaultBranch')
    ..aOS(6, _omitFieldNames ? '' : 'cloneUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderRepository clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderRepository copyWith(
          void Function(GitProviderRepository) updates) =>
      super.copyWith((message) => updates(message as GitProviderRepository))
          as GitProviderRepository;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitProviderRepository() / GitProviderRepository.new instead')
  static GitProviderRepository create() => GitProviderRepository._();
  static $pb.GeneratedMessage $_createMessage() => GitProviderRepository._();
  @$core.override
  GitProviderRepository createEmptyInstance() => GitProviderRepository._();
  @$core.pragma('dart2js:noInline')
  static GitProviderRepository getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitProviderRepository>(
          GitProviderRepository.$_createMessage);
  static GitProviderRepository? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get owner => $_getSZ(0);
  @$pb.TagNumber(1)
  set owner($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get fullName => $_getSZ(2);
  @$pb.TagNumber(3)
  set fullName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFullName() => $_has(2);
  @$pb.TagNumber(3)
  void clearFullName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get private => $_getBF(3);
  @$pb.TagNumber(4)
  set private($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivate() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivate() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get defaultBranch => $_getSZ(4);
  @$pb.TagNumber(5)
  set defaultBranch($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDefaultBranch() => $_has(4);
  @$pb.TagNumber(5)
  void clearDefaultBranch() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get cloneUrl => $_getSZ(5);
  @$pb.TagNumber(6)
  set cloneUrl($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCloneUrl() => $_has(5);
  @$pb.TagNumber(6)
  void clearCloneUrl() => $_clearField(6);
}

/// Branch or tag reference returned by provider discovery (`GitReferenceInfo`).
class GitProviderReference extends $pb.GeneratedMessage {
  factory GitProviderReference({
    $core.String? name,
    $core.String? commit,
  }) {
    final result = GitProviderReference._();
    if (name != null) result.name = name;
    if (commit != null) result.commit = commit;
    return result;
  }

  GitProviderReference._();

  factory GitProviderReference.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderReference()..mergeFromBuffer(data, registry);
  factory GitProviderReference.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderReference()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProviderReference',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProviderReference.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'commit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderReference clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderReference copyWith(void Function(GitProviderReference) updates) =>
      super.copyWith((message) => updates(message as GitProviderReference))
          as GitProviderReference;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitProviderReference() / GitProviderReference.new instead')
  static GitProviderReference create() => GitProviderReference._();
  static $pb.GeneratedMessage $_createMessage() => GitProviderReference._();
  @$core.override
  GitProviderReference createEmptyInstance() => GitProviderReference._();
  @$core.pragma('dart2js:noInline')
  static GitProviderReference getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitProviderReference>(
          GitProviderReference.$_createMessage);
  static GitProviderReference? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get commit => $_getSZ(1);
  @$pb.TagNumber(2)
  set commit($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCommit() => $_has(1);
  @$pb.TagNumber(2)
  void clearCommit() => $_clearField(2);
}

/// Provider-side webhook registration (`ProviderWebhookInfo`).
class GitProviderWebhookInfo extends $pb.GeneratedMessage {
  factory GitProviderWebhookInfo({
    $core.String? id,
    $core.String? url,
    $core.bool? active,
  }) {
    final result = GitProviderWebhookInfo._();
    if (id != null) result.id = id;
    if (url != null) result.url = url;
    if (active != null) result.active = active;
    return result;
  }

  GitProviderWebhookInfo._();

  factory GitProviderWebhookInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderWebhookInfo()..mergeFromBuffer(data, registry);
  factory GitProviderWebhookInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderWebhookInfo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProviderWebhookInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProviderWebhookInfo.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'url')
    ..aOB(3, _omitFieldNames ? '' : 'active')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderWebhookInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderWebhookInfo copyWith(
          void Function(GitProviderWebhookInfo) updates) =>
      super.copyWith((message) => updates(message as GitProviderWebhookInfo))
          as GitProviderWebhookInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitProviderWebhookInfo() / GitProviderWebhookInfo.new instead')
  static GitProviderWebhookInfo create() => GitProviderWebhookInfo._();
  static $pb.GeneratedMessage $_createMessage() => GitProviderWebhookInfo._();
  @$core.override
  GitProviderWebhookInfo createEmptyInstance() => GitProviderWebhookInfo._();
  @$core.pragma('dart2js:noInline')
  static GitProviderWebhookInfo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitProviderWebhookInfo>(
          GitProviderWebhookInfo.$_createMessage);
  static GitProviderWebhookInfo? _defaultInstance;

  /// Webhook identifier as issued by the provider (not a panel row id).
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
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
  $core.bool get active => $_getBF(2);
  @$pb.TagNumber(3)
  set active($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasActive() => $_has(2);
  @$pb.TagNumber(3)
  void clearActive() => $_clearField(3);
}

/// Collaborator access report (`CollaboratorPermission`); `level` is a provider-specific string (e.g. "admin", "write", "read", "none").
class GitProviderCollaboratorPermission extends $pb.GeneratedMessage {
  factory GitProviderCollaboratorPermission({
    $core.String? username,
    $core.String? level,
    $core.bool? canWrite,
    $core.bool? canAdmin,
  }) {
    final result = GitProviderCollaboratorPermission._();
    if (username != null) result.username = username;
    if (level != null) result.level = level;
    if (canWrite != null) result.canWrite = canWrite;
    if (canAdmin != null) result.canAdmin = canAdmin;
    return result;
  }

  GitProviderCollaboratorPermission._();

  factory GitProviderCollaboratorPermission.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderCollaboratorPermission()..mergeFromBuffer(data, registry);
  factory GitProviderCollaboratorPermission.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitProviderCollaboratorPermission()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitProviderCollaboratorPermission',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitProviderCollaboratorPermission.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'username')
    ..aOS(2, _omitFieldNames ? '' : 'level')
    ..aOB(3, _omitFieldNames ? '' : 'canWrite')
    ..aOB(4, _omitFieldNames ? '' : 'canAdmin')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderCollaboratorPermission clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitProviderCollaboratorPermission copyWith(
          void Function(GitProviderCollaboratorPermission) updates) =>
      super.copyWith((message) =>
              updates(message as GitProviderCollaboratorPermission))
          as GitProviderCollaboratorPermission;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitProviderCollaboratorPermission() / GitProviderCollaboratorPermission.new instead')
  static GitProviderCollaboratorPermission create() =>
      GitProviderCollaboratorPermission._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitProviderCollaboratorPermission._();
  @$core.override
  GitProviderCollaboratorPermission createEmptyInstance() =>
      GitProviderCollaboratorPermission._();
  @$core.pragma('dart2js:noInline')
  static GitProviderCollaboratorPermission getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitProviderCollaboratorPermission>(
          GitProviderCollaboratorPermission.$_createMessage);
  static GitProviderCollaboratorPermission? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get username => $_getSZ(0);
  @$pb.TagNumber(1)
  set username($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUsername() => $_has(0);
  @$pb.TagNumber(1)
  void clearUsername() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get level => $_getSZ(1);
  @$pb.TagNumber(2)
  set level($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLevel() => $_has(1);
  @$pb.TagNumber(2)
  void clearLevel() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get canWrite => $_getBF(2);
  @$pb.TagNumber(3)
  set canWrite($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCanWrite() => $_has(2);
  @$pb.TagNumber(3)
  void clearCanWrite() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get canAdmin => $_getBF(3);
  @$pb.TagNumber(4)
  set canAdmin($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCanAdmin() => $_has(3);
  @$pb.TagNumber(4)
  void clearCanAdmin() => $_clearField(4);
}

class ListGitProvidersRequest extends $pb.GeneratedMessage {
  factory ListGitProvidersRequest() => ListGitProvidersRequest._();

  ListGitProvidersRequest._();

  factory ListGitProvidersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitProvidersRequest()..mergeFromBuffer(data, registry);
  factory ListGitProvidersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitProvidersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListGitProvidersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListGitProvidersRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitProvidersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitProvidersRequest copyWith(
          void Function(ListGitProvidersRequest) updates) =>
      super.copyWith((message) => updates(message as ListGitProvidersRequest))
          as ListGitProvidersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListGitProvidersRequest() / ListGitProvidersRequest.new instead')
  static ListGitProvidersRequest create() => ListGitProvidersRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListGitProvidersRequest._();
  @$core.override
  ListGitProvidersRequest createEmptyInstance() => ListGitProvidersRequest._();
  @$core.pragma('dart2js:noInline')
  static ListGitProvidersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListGitProvidersRequest>(
          ListGitProvidersRequest.$_createMessage);
  static ListGitProvidersRequest? _defaultInstance;
}

class ListGitProvidersResponse extends $pb.GeneratedMessage {
  factory ListGitProvidersResponse({
    $core.Iterable<GitProvider>? providers,
  }) {
    final result = ListGitProvidersResponse._();
    if (providers != null) result.providers.addAll(providers);
    return result;
  }

  ListGitProvidersResponse._();

  factory ListGitProvidersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitProvidersResponse()..mergeFromBuffer(data, registry);
  factory ListGitProvidersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitProvidersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListGitProvidersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListGitProvidersResponse.$_createMessage)
    ..pPM<GitProvider>(1, _omitFieldNames ? '' : 'providers',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitProvidersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitProvidersResponse copyWith(
          void Function(ListGitProvidersResponse) updates) =>
      super.copyWith((message) => updates(message as ListGitProvidersResponse))
          as ListGitProvidersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListGitProvidersResponse() / ListGitProvidersResponse.new instead')
  static ListGitProvidersResponse create() => ListGitProvidersResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListGitProvidersResponse._();
  @$core.override
  ListGitProvidersResponse createEmptyInstance() =>
      ListGitProvidersResponse._();
  @$core.pragma('dart2js:noInline')
  static ListGitProvidersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListGitProvidersResponse>(
          ListGitProvidersResponse.$_createMessage);
  static ListGitProvidersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GitProvider> get providers => $_getList(0);
}

class GetGitProviderRequest extends $pb.GeneratedMessage {
  factory GetGitProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetGitProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetGitProviderRequest._();

  factory GetGitProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetGitProviderRequest()..mergeFromBuffer(data, registry);
  factory GetGitProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetGitProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetGitProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetGitProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetGitProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetGitProviderRequest copyWith(
          void Function(GetGitProviderRequest) updates) =>
      super.copyWith((message) => updates(message as GetGitProviderRequest))
          as GetGitProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetGitProviderRequest() / GetGitProviderRequest.new instead')
  static GetGitProviderRequest create() => GetGitProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetGitProviderRequest._();
  @$core.override
  GetGitProviderRequest createEmptyInstance() => GetGitProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static GetGitProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetGitProviderRequest>(
          GetGitProviderRequest.$_createMessage);
  static GetGitProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetGitProviderResponse extends $pb.GeneratedMessage {
  factory GetGitProviderResponse({
    GitProvider? provider,
  }) {
    final result = GetGitProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  GetGitProviderResponse._();

  factory GetGitProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetGitProviderResponse()..mergeFromBuffer(data, registry);
  factory GetGitProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetGitProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetGitProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetGitProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetGitProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetGitProviderResponse copyWith(
          void Function(GetGitProviderResponse) updates) =>
      super.copyWith((message) => updates(message as GetGitProviderResponse))
          as GetGitProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetGitProviderResponse() / GetGitProviderResponse.new instead')
  static GetGitProviderResponse create() => GetGitProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetGitProviderResponse._();
  @$core.override
  GetGitProviderResponse createEmptyInstance() => GetGitProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static GetGitProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetGitProviderResponse>(
          GetGitProviderResponse.$_createMessage);
  static GetGitProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class CreateGithubProviderRequest extends $pb.GeneratedMessage {
  factory CreateGithubProviderRequest({
    $core.String? name,
    $core.bool? shared,
    $core.String? appName,
    $fixnum.Int64? appId,
    $core.String? clientId,
    $core.String? clientSecret,
    $core.String? installationId,
    $core.String? privateKey,
  }) {
    final result = CreateGithubProviderRequest._();
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (appName != null) result.appName = appName;
    if (appId != null) result.appId = appId;
    if (clientId != null) result.clientId = clientId;
    if (clientSecret != null) result.clientSecret = clientSecret;
    if (installationId != null) result.installationId = installationId;
    if (privateKey != null) result.privateKey = privateKey;
    return result;
  }

  CreateGithubProviderRequest._();

  factory CreateGithubProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGithubProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateGithubProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGithubProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGithubProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGithubProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOB(2, _omitFieldNames ? '' : 'shared')
    ..aOS(3, _omitFieldNames ? '' : 'appName')
    ..aInt64(4, _omitFieldNames ? '' : 'appId')
    ..aOS(5, _omitFieldNames ? '' : 'clientId')
    ..aOS(6, _omitFieldNames ? '' : 'clientSecret')
    ..aOS(7, _omitFieldNames ? '' : 'installationId')
    ..aOS(8, _omitFieldNames ? '' : 'privateKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGithubProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGithubProviderRequest copyWith(
          void Function(CreateGithubProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGithubProviderRequest))
          as CreateGithubProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGithubProviderRequest() / CreateGithubProviderRequest.new instead')
  static CreateGithubProviderRequest create() =>
      CreateGithubProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGithubProviderRequest._();
  @$core.override
  CreateGithubProviderRequest createEmptyInstance() =>
      CreateGithubProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateGithubProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGithubProviderRequest>(
          CreateGithubProviderRequest.$_createMessage);
  static CreateGithubProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// Omitted means shared (server default `true`).
  @$pb.TagNumber(2)
  $core.bool get shared => $_getBF(1);
  @$pb.TagNumber(2)
  set shared($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasShared() => $_has(1);
  @$pb.TagNumber(2)
  void clearShared() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get appName => $_getSZ(2);
  @$pb.TagNumber(3)
  set appName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAppName() => $_has(2);
  @$pb.TagNumber(3)
  void clearAppName() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get appId => $_getI64(3);
  @$pb.TagNumber(4)
  set appId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAppId() => $_has(3);
  @$pb.TagNumber(4)
  void clearAppId() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get clientId => $_getSZ(4);
  @$pb.TagNumber(5)
  set clientId($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasClientId() => $_has(4);
  @$pb.TagNumber(5)
  void clearClientId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get clientSecret => $_getSZ(5);
  @$pb.TagNumber(6)
  set clientSecret($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasClientSecret() => $_has(5);
  @$pb.TagNumber(6)
  void clearClientSecret() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get installationId => $_getSZ(6);
  @$pb.TagNumber(7)
  set installationId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasInstallationId() => $_has(6);
  @$pb.TagNumber(7)
  void clearInstallationId() => $_clearField(7);

  /// GitHub App PEM private key; opaque secret material.
  @$pb.TagNumber(8)
  $core.String get privateKey => $_getSZ(7);
  @$pb.TagNumber(8)
  set privateKey($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasPrivateKey() => $_has(7);
  @$pb.TagNumber(8)
  void clearPrivateKey() => $_clearField(8);
}

class CreateGithubProviderResponse extends $pb.GeneratedMessage {
  factory CreateGithubProviderResponse({
    GitProvider? provider,
    $core.String? webhookSecret,
  }) {
    final result = CreateGithubProviderResponse._();
    if (provider != null) result.provider = provider;
    if (webhookSecret != null) result.webhookSecret = webhookSecret;
    return result;
  }

  CreateGithubProviderResponse._();

  factory CreateGithubProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGithubProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateGithubProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGithubProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGithubProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGithubProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'webhookSecret')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGithubProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGithubProviderResponse copyWith(
          void Function(CreateGithubProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGithubProviderResponse))
          as CreateGithubProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGithubProviderResponse() / CreateGithubProviderResponse.new instead')
  static CreateGithubProviderResponse create() =>
      CreateGithubProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGithubProviderResponse._();
  @$core.override
  CreateGithubProviderResponse createEmptyInstance() =>
      CreateGithubProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateGithubProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGithubProviderResponse>(
          CreateGithubProviderResponse.$_createMessage);
  static CreateGithubProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get webhookSecret => $_getSZ(1);
  @$pb.TagNumber(2)
  set webhookSecret($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasWebhookSecret() => $_has(1);
  @$pb.TagNumber(2)
  void clearWebhookSecret() => $_clearField(2);
}

class CreateGitlabProviderRequest extends $pb.GeneratedMessage {
  factory CreateGitlabProviderRequest({
    $core.String? name,
    $core.bool? shared,
    $core.String? url,
    $core.String? internalUrl,
    $core.String? applicationId,
    $core.String? redirectUri,
    $core.String? secret,
    $core.String? accessToken,
    $core.String? refreshToken,
    $core.String? groupName,
  }) {
    final result = CreateGitlabProviderRequest._();
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (url != null) result.url = url;
    if (internalUrl != null) result.internalUrl = internalUrl;
    if (applicationId != null) result.applicationId = applicationId;
    if (redirectUri != null) result.redirectUri = redirectUri;
    if (secret != null) result.secret = secret;
    if (accessToken != null) result.accessToken = accessToken;
    if (refreshToken != null) result.refreshToken = refreshToken;
    if (groupName != null) result.groupName = groupName;
    return result;
  }

  CreateGitlabProviderRequest._();

  factory CreateGitlabProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGitlabProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateGitlabProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGitlabProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGitlabProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGitlabProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOB(2, _omitFieldNames ? '' : 'shared')
    ..aOS(3, _omitFieldNames ? '' : 'url')
    ..aOS(4, _omitFieldNames ? '' : 'internalUrl')
    ..aOS(5, _omitFieldNames ? '' : 'applicationId')
    ..aOS(6, _omitFieldNames ? '' : 'redirectUri')
    ..aOS(7, _omitFieldNames ? '' : 'secret')
    ..aOS(8, _omitFieldNames ? '' : 'accessToken')
    ..aOS(9, _omitFieldNames ? '' : 'refreshToken')
    ..aOS(10, _omitFieldNames ? '' : 'groupName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGitlabProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGitlabProviderRequest copyWith(
          void Function(CreateGitlabProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGitlabProviderRequest))
          as CreateGitlabProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGitlabProviderRequest() / CreateGitlabProviderRequest.new instead')
  static CreateGitlabProviderRequest create() =>
      CreateGitlabProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGitlabProviderRequest._();
  @$core.override
  CreateGitlabProviderRequest createEmptyInstance() =>
      CreateGitlabProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateGitlabProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGitlabProviderRequest>(
          CreateGitlabProviderRequest.$_createMessage);
  static CreateGitlabProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// Omitted means shared (server default `true`).
  @$pb.TagNumber(2)
  $core.bool get shared => $_getBF(1);
  @$pb.TagNumber(2)
  set shared($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasShared() => $_has(1);
  @$pb.TagNumber(2)
  void clearShared() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get url => $_getSZ(2);
  @$pb.TagNumber(3)
  set url($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearUrl() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get internalUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set internalUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasInternalUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearInternalUrl() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get applicationId => $_getSZ(4);
  @$pb.TagNumber(5)
  set applicationId($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasApplicationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearApplicationId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get redirectUri => $_getSZ(5);
  @$pb.TagNumber(6)
  set redirectUri($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRedirectUri() => $_has(5);
  @$pb.TagNumber(6)
  void clearRedirectUri() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get secret => $_getSZ(6);
  @$pb.TagNumber(7)
  set secret($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSecret() => $_has(6);
  @$pb.TagNumber(7)
  void clearSecret() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get accessToken => $_getSZ(7);
  @$pb.TagNumber(8)
  set accessToken($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasAccessToken() => $_has(7);
  @$pb.TagNumber(8)
  void clearAccessToken() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get refreshToken => $_getSZ(8);
  @$pb.TagNumber(9)
  set refreshToken($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasRefreshToken() => $_has(8);
  @$pb.TagNumber(9)
  void clearRefreshToken() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get groupName => $_getSZ(9);
  @$pb.TagNumber(10)
  set groupName($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasGroupName() => $_has(9);
  @$pb.TagNumber(10)
  void clearGroupName() => $_clearField(10);
}

class CreateGitlabProviderResponse extends $pb.GeneratedMessage {
  factory CreateGitlabProviderResponse({
    GitProvider? provider,
    $core.String? webhookSecret,
  }) {
    final result = CreateGitlabProviderResponse._();
    if (provider != null) result.provider = provider;
    if (webhookSecret != null) result.webhookSecret = webhookSecret;
    return result;
  }

  CreateGitlabProviderResponse._();

  factory CreateGitlabProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGitlabProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateGitlabProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGitlabProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGitlabProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGitlabProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'webhookSecret')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGitlabProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGitlabProviderResponse copyWith(
          void Function(CreateGitlabProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGitlabProviderResponse))
          as CreateGitlabProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGitlabProviderResponse() / CreateGitlabProviderResponse.new instead')
  static CreateGitlabProviderResponse create() =>
      CreateGitlabProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGitlabProviderResponse._();
  @$core.override
  CreateGitlabProviderResponse createEmptyInstance() =>
      CreateGitlabProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateGitlabProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGitlabProviderResponse>(
          CreateGitlabProviderResponse.$_createMessage);
  static CreateGitlabProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get webhookSecret => $_getSZ(1);
  @$pb.TagNumber(2)
  set webhookSecret($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasWebhookSecret() => $_has(1);
  @$pb.TagNumber(2)
  void clearWebhookSecret() => $_clearField(2);
}

class CreateGiteaProviderRequest extends $pb.GeneratedMessage {
  factory CreateGiteaProviderRequest({
    $core.String? name,
    $core.bool? shared,
    $core.String? url,
    $core.String? internalUrl,
    $core.String? redirectUri,
    $core.String? clientId,
    $core.String? clientSecret,
    $core.String? accessToken,
    $core.String? refreshToken,
    $core.String? scopes,
  }) {
    final result = CreateGiteaProviderRequest._();
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (url != null) result.url = url;
    if (internalUrl != null) result.internalUrl = internalUrl;
    if (redirectUri != null) result.redirectUri = redirectUri;
    if (clientId != null) result.clientId = clientId;
    if (clientSecret != null) result.clientSecret = clientSecret;
    if (accessToken != null) result.accessToken = accessToken;
    if (refreshToken != null) result.refreshToken = refreshToken;
    if (scopes != null) result.scopes = scopes;
    return result;
  }

  CreateGiteaProviderRequest._();

  factory CreateGiteaProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGiteaProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateGiteaProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGiteaProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGiteaProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGiteaProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOB(2, _omitFieldNames ? '' : 'shared')
    ..aOS(3, _omitFieldNames ? '' : 'url')
    ..aOS(4, _omitFieldNames ? '' : 'internalUrl')
    ..aOS(5, _omitFieldNames ? '' : 'redirectUri')
    ..aOS(6, _omitFieldNames ? '' : 'clientId')
    ..aOS(7, _omitFieldNames ? '' : 'clientSecret')
    ..aOS(8, _omitFieldNames ? '' : 'accessToken')
    ..aOS(9, _omitFieldNames ? '' : 'refreshToken')
    ..aOS(10, _omitFieldNames ? '' : 'scopes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGiteaProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGiteaProviderRequest copyWith(
          void Function(CreateGiteaProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGiteaProviderRequest))
          as CreateGiteaProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGiteaProviderRequest() / CreateGiteaProviderRequest.new instead')
  static CreateGiteaProviderRequest create() => CreateGiteaProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGiteaProviderRequest._();
  @$core.override
  CreateGiteaProviderRequest createEmptyInstance() =>
      CreateGiteaProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateGiteaProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGiteaProviderRequest>(
          CreateGiteaProviderRequest.$_createMessage);
  static CreateGiteaProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// Omitted means shared (server default `true`).
  @$pb.TagNumber(2)
  $core.bool get shared => $_getBF(1);
  @$pb.TagNumber(2)
  set shared($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasShared() => $_has(1);
  @$pb.TagNumber(2)
  void clearShared() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get url => $_getSZ(2);
  @$pb.TagNumber(3)
  set url($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearUrl() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get internalUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set internalUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasInternalUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearInternalUrl() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get redirectUri => $_getSZ(4);
  @$pb.TagNumber(5)
  set redirectUri($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRedirectUri() => $_has(4);
  @$pb.TagNumber(5)
  void clearRedirectUri() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get clientId => $_getSZ(5);
  @$pb.TagNumber(6)
  set clientId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasClientId() => $_has(5);
  @$pb.TagNumber(6)
  void clearClientId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get clientSecret => $_getSZ(6);
  @$pb.TagNumber(7)
  set clientSecret($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasClientSecret() => $_has(6);
  @$pb.TagNumber(7)
  void clearClientSecret() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get accessToken => $_getSZ(7);
  @$pb.TagNumber(8)
  set accessToken($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasAccessToken() => $_has(7);
  @$pb.TagNumber(8)
  void clearAccessToken() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get refreshToken => $_getSZ(8);
  @$pb.TagNumber(9)
  set refreshToken($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasRefreshToken() => $_has(8);
  @$pb.TagNumber(9)
  void clearRefreshToken() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get scopes => $_getSZ(9);
  @$pb.TagNumber(10)
  set scopes($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasScopes() => $_has(9);
  @$pb.TagNumber(10)
  void clearScopes() => $_clearField(10);
}

class CreateGiteaProviderResponse extends $pb.GeneratedMessage {
  factory CreateGiteaProviderResponse({
    GitProvider? provider,
    $core.String? webhookSecret,
  }) {
    final result = CreateGiteaProviderResponse._();
    if (provider != null) result.provider = provider;
    if (webhookSecret != null) result.webhookSecret = webhookSecret;
    return result;
  }

  CreateGiteaProviderResponse._();

  factory CreateGiteaProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGiteaProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateGiteaProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateGiteaProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateGiteaProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateGiteaProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'webhookSecret')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGiteaProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateGiteaProviderResponse copyWith(
          void Function(CreateGiteaProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateGiteaProviderResponse))
          as CreateGiteaProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateGiteaProviderResponse() / CreateGiteaProviderResponse.new instead')
  static CreateGiteaProviderResponse create() =>
      CreateGiteaProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateGiteaProviderResponse._();
  @$core.override
  CreateGiteaProviderResponse createEmptyInstance() =>
      CreateGiteaProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateGiteaProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateGiteaProviderResponse>(
          CreateGiteaProviderResponse.$_createMessage);
  static CreateGiteaProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get webhookSecret => $_getSZ(1);
  @$pb.TagNumber(2)
  set webhookSecret($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasWebhookSecret() => $_has(1);
  @$pb.TagNumber(2)
  void clearWebhookSecret() => $_clearField(2);
}

class CreateBitbucketProviderRequest extends $pb.GeneratedMessage {
  factory CreateBitbucketProviderRequest({
    $core.String? name,
    $core.bool? shared,
    $core.String? username,
    $core.String? email,
    $core.String? appPassword,
    $core.String? apiToken,
    $core.String? workspace,
  }) {
    final result = CreateBitbucketProviderRequest._();
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (username != null) result.username = username;
    if (email != null) result.email = email;
    if (appPassword != null) result.appPassword = appPassword;
    if (apiToken != null) result.apiToken = apiToken;
    if (workspace != null) result.workspace = workspace;
    return result;
  }

  CreateBitbucketProviderRequest._();

  factory CreateBitbucketProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateBitbucketProviderRequest()..mergeFromBuffer(data, registry);
  factory CreateBitbucketProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateBitbucketProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateBitbucketProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateBitbucketProviderRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOB(2, _omitFieldNames ? '' : 'shared')
    ..aOS(3, _omitFieldNames ? '' : 'username')
    ..aOS(4, _omitFieldNames ? '' : 'email')
    ..aOS(5, _omitFieldNames ? '' : 'appPassword')
    ..aOS(6, _omitFieldNames ? '' : 'apiToken')
    ..aOS(7, _omitFieldNames ? '' : 'workspace')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBitbucketProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBitbucketProviderRequest copyWith(
          void Function(CreateBitbucketProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateBitbucketProviderRequest))
          as CreateBitbucketProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateBitbucketProviderRequest() / CreateBitbucketProviderRequest.new instead')
  static CreateBitbucketProviderRequest create() =>
      CreateBitbucketProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateBitbucketProviderRequest._();
  @$core.override
  CreateBitbucketProviderRequest createEmptyInstance() =>
      CreateBitbucketProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateBitbucketProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateBitbucketProviderRequest>(
          CreateBitbucketProviderRequest.$_createMessage);
  static CreateBitbucketProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// Omitted means shared (server default `true`).
  @$pb.TagNumber(2)
  $core.bool get shared => $_getBF(1);
  @$pb.TagNumber(2)
  set shared($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasShared() => $_has(1);
  @$pb.TagNumber(2)
  void clearShared() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get username => $_getSZ(2);
  @$pb.TagNumber(3)
  set username($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUsername() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsername() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get email => $_getSZ(3);
  @$pb.TagNumber(4)
  set email($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEmail() => $_has(3);
  @$pb.TagNumber(4)
  void clearEmail() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get appPassword => $_getSZ(4);
  @$pb.TagNumber(5)
  set appPassword($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAppPassword() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppPassword() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get apiToken => $_getSZ(5);
  @$pb.TagNumber(6)
  set apiToken($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApiToken() => $_has(5);
  @$pb.TagNumber(6)
  void clearApiToken() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get workspace => $_getSZ(6);
  @$pb.TagNumber(7)
  set workspace($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasWorkspace() => $_has(6);
  @$pb.TagNumber(7)
  void clearWorkspace() => $_clearField(7);
}

class CreateBitbucketProviderResponse extends $pb.GeneratedMessage {
  factory CreateBitbucketProviderResponse({
    GitProvider? provider,
    $core.String? webhookSecret,
  }) {
    final result = CreateBitbucketProviderResponse._();
    if (provider != null) result.provider = provider;
    if (webhookSecret != null) result.webhookSecret = webhookSecret;
    return result;
  }

  CreateBitbucketProviderResponse._();

  factory CreateBitbucketProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateBitbucketProviderResponse()..mergeFromBuffer(data, registry);
  factory CreateBitbucketProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateBitbucketProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateBitbucketProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateBitbucketProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'webhookSecret')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBitbucketProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBitbucketProviderResponse copyWith(
          void Function(CreateBitbucketProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CreateBitbucketProviderResponse))
          as CreateBitbucketProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateBitbucketProviderResponse() / CreateBitbucketProviderResponse.new instead')
  static CreateBitbucketProviderResponse create() =>
      CreateBitbucketProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateBitbucketProviderResponse._();
  @$core.override
  CreateBitbucketProviderResponse createEmptyInstance() =>
      CreateBitbucketProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateBitbucketProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateBitbucketProviderResponse>(
          CreateBitbucketProviderResponse.$_createMessage);
  static CreateBitbucketProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get webhookSecret => $_getSZ(1);
  @$pb.TagNumber(2)
  set webhookSecret($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasWebhookSecret() => $_has(1);
  @$pb.TagNumber(2)
  void clearWebhookSecret() => $_clearField(2);
}

/// The stored DTO requires `name` on every update; remaining credential fields are optional patches (absent keeps the stored value).
class UpdateGithubProviderRequest extends $pb.GeneratedMessage {
  factory UpdateGithubProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.bool? shared,
    $core.String? appName,
    $fixnum.Int64? appId,
    $core.String? clientId,
    $core.String? clientSecret,
    $core.String? installationId,
    $core.String? privateKey,
  }) {
    final result = UpdateGithubProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (appName != null) result.appName = appName;
    if (appId != null) result.appId = appId;
    if (clientId != null) result.clientId = clientId;
    if (clientSecret != null) result.clientSecret = clientSecret;
    if (installationId != null) result.installationId = installationId;
    if (privateKey != null) result.privateKey = privateKey;
    return result;
  }

  UpdateGithubProviderRequest._();

  factory UpdateGithubProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGithubProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateGithubProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGithubProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGithubProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGithubProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOB(3, _omitFieldNames ? '' : 'shared')
    ..aOS(4, _omitFieldNames ? '' : 'appName')
    ..aInt64(5, _omitFieldNames ? '' : 'appId')
    ..aOS(6, _omitFieldNames ? '' : 'clientId')
    ..aOS(7, _omitFieldNames ? '' : 'clientSecret')
    ..aOS(8, _omitFieldNames ? '' : 'installationId')
    ..aOS(9, _omitFieldNames ? '' : 'privateKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGithubProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGithubProviderRequest copyWith(
          void Function(UpdateGithubProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGithubProviderRequest))
          as UpdateGithubProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGithubProviderRequest() / UpdateGithubProviderRequest.new instead')
  static UpdateGithubProviderRequest create() =>
      UpdateGithubProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGithubProviderRequest._();
  @$core.override
  UpdateGithubProviderRequest createEmptyInstance() =>
      UpdateGithubProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateGithubProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGithubProviderRequest>(
          UpdateGithubProviderRequest.$_createMessage);
  static UpdateGithubProviderRequest? _defaultInstance;

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
  $core.bool get shared => $_getBF(2);
  @$pb.TagNumber(3)
  set shared($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShared() => $_has(2);
  @$pb.TagNumber(3)
  void clearShared() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get appName => $_getSZ(3);
  @$pb.TagNumber(4)
  set appName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAppName() => $_has(3);
  @$pb.TagNumber(4)
  void clearAppName() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get appId => $_getI64(4);
  @$pb.TagNumber(5)
  set appId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAppId() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get clientId => $_getSZ(5);
  @$pb.TagNumber(6)
  set clientId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasClientId() => $_has(5);
  @$pb.TagNumber(6)
  void clearClientId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get clientSecret => $_getSZ(6);
  @$pb.TagNumber(7)
  set clientSecret($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasClientSecret() => $_has(6);
  @$pb.TagNumber(7)
  void clearClientSecret() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get installationId => $_getSZ(7);
  @$pb.TagNumber(8)
  set installationId($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasInstallationId() => $_has(7);
  @$pb.TagNumber(8)
  void clearInstallationId() => $_clearField(8);

  /// GitHub App PEM private key; opaque secret material.
  @$pb.TagNumber(9)
  $core.String get privateKey => $_getSZ(8);
  @$pb.TagNumber(9)
  set privateKey($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPrivateKey() => $_has(8);
  @$pb.TagNumber(9)
  void clearPrivateKey() => $_clearField(9);
}

class UpdateGithubProviderResponse extends $pb.GeneratedMessage {
  factory UpdateGithubProviderResponse({
    GitProvider? provider,
  }) {
    final result = UpdateGithubProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateGithubProviderResponse._();

  factory UpdateGithubProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGithubProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateGithubProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGithubProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGithubProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGithubProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGithubProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGithubProviderResponse copyWith(
          void Function(UpdateGithubProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGithubProviderResponse))
          as UpdateGithubProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGithubProviderResponse() / UpdateGithubProviderResponse.new instead')
  static UpdateGithubProviderResponse create() =>
      UpdateGithubProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGithubProviderResponse._();
  @$core.override
  UpdateGithubProviderResponse createEmptyInstance() =>
      UpdateGithubProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateGithubProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGithubProviderResponse>(
          UpdateGithubProviderResponse.$_createMessage);
  static UpdateGithubProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class UpdateGitlabProviderRequest extends $pb.GeneratedMessage {
  factory UpdateGitlabProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.bool? shared,
    $core.String? url,
    $core.String? internalUrl,
    $core.String? applicationId,
    $core.String? redirectUri,
    $core.String? secret,
    $core.String? accessToken,
    $core.String? refreshToken,
    $core.String? groupName,
  }) {
    final result = UpdateGitlabProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (url != null) result.url = url;
    if (internalUrl != null) result.internalUrl = internalUrl;
    if (applicationId != null) result.applicationId = applicationId;
    if (redirectUri != null) result.redirectUri = redirectUri;
    if (secret != null) result.secret = secret;
    if (accessToken != null) result.accessToken = accessToken;
    if (refreshToken != null) result.refreshToken = refreshToken;
    if (groupName != null) result.groupName = groupName;
    return result;
  }

  UpdateGitlabProviderRequest._();

  factory UpdateGitlabProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGitlabProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateGitlabProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGitlabProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGitlabProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGitlabProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOB(3, _omitFieldNames ? '' : 'shared')
    ..aOS(4, _omitFieldNames ? '' : 'url')
    ..aOS(5, _omitFieldNames ? '' : 'internalUrl')
    ..aOS(6, _omitFieldNames ? '' : 'applicationId')
    ..aOS(7, _omitFieldNames ? '' : 'redirectUri')
    ..aOS(8, _omitFieldNames ? '' : 'secret')
    ..aOS(9, _omitFieldNames ? '' : 'accessToken')
    ..aOS(10, _omitFieldNames ? '' : 'refreshToken')
    ..aOS(11, _omitFieldNames ? '' : 'groupName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGitlabProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGitlabProviderRequest copyWith(
          void Function(UpdateGitlabProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGitlabProviderRequest))
          as UpdateGitlabProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGitlabProviderRequest() / UpdateGitlabProviderRequest.new instead')
  static UpdateGitlabProviderRequest create() =>
      UpdateGitlabProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGitlabProviderRequest._();
  @$core.override
  UpdateGitlabProviderRequest createEmptyInstance() =>
      UpdateGitlabProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateGitlabProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGitlabProviderRequest>(
          UpdateGitlabProviderRequest.$_createMessage);
  static UpdateGitlabProviderRequest? _defaultInstance;

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
  $core.bool get shared => $_getBF(2);
  @$pb.TagNumber(3)
  set shared($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShared() => $_has(2);
  @$pb.TagNumber(3)
  void clearShared() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get url => $_getSZ(3);
  @$pb.TagNumber(4)
  set url($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearUrl() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get internalUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set internalUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasInternalUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearInternalUrl() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get applicationId => $_getSZ(5);
  @$pb.TagNumber(6)
  set applicationId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasApplicationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearApplicationId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get redirectUri => $_getSZ(6);
  @$pb.TagNumber(7)
  set redirectUri($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRedirectUri() => $_has(6);
  @$pb.TagNumber(7)
  void clearRedirectUri() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get secret => $_getSZ(7);
  @$pb.TagNumber(8)
  set secret($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasSecret() => $_has(7);
  @$pb.TagNumber(8)
  void clearSecret() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get accessToken => $_getSZ(8);
  @$pb.TagNumber(9)
  set accessToken($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasAccessToken() => $_has(8);
  @$pb.TagNumber(9)
  void clearAccessToken() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get refreshToken => $_getSZ(9);
  @$pb.TagNumber(10)
  set refreshToken($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasRefreshToken() => $_has(9);
  @$pb.TagNumber(10)
  void clearRefreshToken() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get groupName => $_getSZ(10);
  @$pb.TagNumber(11)
  set groupName($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasGroupName() => $_has(10);
  @$pb.TagNumber(11)
  void clearGroupName() => $_clearField(11);
}

class UpdateGitlabProviderResponse extends $pb.GeneratedMessage {
  factory UpdateGitlabProviderResponse({
    GitProvider? provider,
  }) {
    final result = UpdateGitlabProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateGitlabProviderResponse._();

  factory UpdateGitlabProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGitlabProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateGitlabProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGitlabProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGitlabProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGitlabProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGitlabProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGitlabProviderResponse copyWith(
          void Function(UpdateGitlabProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGitlabProviderResponse))
          as UpdateGitlabProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGitlabProviderResponse() / UpdateGitlabProviderResponse.new instead')
  static UpdateGitlabProviderResponse create() =>
      UpdateGitlabProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGitlabProviderResponse._();
  @$core.override
  UpdateGitlabProviderResponse createEmptyInstance() =>
      UpdateGitlabProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateGitlabProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGitlabProviderResponse>(
          UpdateGitlabProviderResponse.$_createMessage);
  static UpdateGitlabProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class UpdateGiteaProviderRequest extends $pb.GeneratedMessage {
  factory UpdateGiteaProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.bool? shared,
    $core.String? url,
    $core.String? internalUrl,
    $core.String? redirectUri,
    $core.String? clientId,
    $core.String? clientSecret,
    $core.String? accessToken,
    $core.String? refreshToken,
    $core.String? scopes,
  }) {
    final result = UpdateGiteaProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (url != null) result.url = url;
    if (internalUrl != null) result.internalUrl = internalUrl;
    if (redirectUri != null) result.redirectUri = redirectUri;
    if (clientId != null) result.clientId = clientId;
    if (clientSecret != null) result.clientSecret = clientSecret;
    if (accessToken != null) result.accessToken = accessToken;
    if (refreshToken != null) result.refreshToken = refreshToken;
    if (scopes != null) result.scopes = scopes;
    return result;
  }

  UpdateGiteaProviderRequest._();

  factory UpdateGiteaProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGiteaProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateGiteaProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGiteaProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGiteaProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGiteaProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOB(3, _omitFieldNames ? '' : 'shared')
    ..aOS(4, _omitFieldNames ? '' : 'url')
    ..aOS(5, _omitFieldNames ? '' : 'internalUrl')
    ..aOS(6, _omitFieldNames ? '' : 'redirectUri')
    ..aOS(7, _omitFieldNames ? '' : 'clientId')
    ..aOS(8, _omitFieldNames ? '' : 'clientSecret')
    ..aOS(9, _omitFieldNames ? '' : 'accessToken')
    ..aOS(10, _omitFieldNames ? '' : 'refreshToken')
    ..aOS(11, _omitFieldNames ? '' : 'scopes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGiteaProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGiteaProviderRequest copyWith(
          void Function(UpdateGiteaProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGiteaProviderRequest))
          as UpdateGiteaProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGiteaProviderRequest() / UpdateGiteaProviderRequest.new instead')
  static UpdateGiteaProviderRequest create() => UpdateGiteaProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGiteaProviderRequest._();
  @$core.override
  UpdateGiteaProviderRequest createEmptyInstance() =>
      UpdateGiteaProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateGiteaProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGiteaProviderRequest>(
          UpdateGiteaProviderRequest.$_createMessage);
  static UpdateGiteaProviderRequest? _defaultInstance;

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
  $core.bool get shared => $_getBF(2);
  @$pb.TagNumber(3)
  set shared($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShared() => $_has(2);
  @$pb.TagNumber(3)
  void clearShared() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get url => $_getSZ(3);
  @$pb.TagNumber(4)
  set url($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearUrl() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get internalUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set internalUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasInternalUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearInternalUrl() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get redirectUri => $_getSZ(5);
  @$pb.TagNumber(6)
  set redirectUri($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRedirectUri() => $_has(5);
  @$pb.TagNumber(6)
  void clearRedirectUri() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get clientId => $_getSZ(6);
  @$pb.TagNumber(7)
  set clientId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasClientId() => $_has(6);
  @$pb.TagNumber(7)
  void clearClientId() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get clientSecret => $_getSZ(7);
  @$pb.TagNumber(8)
  set clientSecret($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasClientSecret() => $_has(7);
  @$pb.TagNumber(8)
  void clearClientSecret() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get accessToken => $_getSZ(8);
  @$pb.TagNumber(9)
  set accessToken($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasAccessToken() => $_has(8);
  @$pb.TagNumber(9)
  void clearAccessToken() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get refreshToken => $_getSZ(9);
  @$pb.TagNumber(10)
  set refreshToken($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasRefreshToken() => $_has(9);
  @$pb.TagNumber(10)
  void clearRefreshToken() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get scopes => $_getSZ(10);
  @$pb.TagNumber(11)
  set scopes($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasScopes() => $_has(10);
  @$pb.TagNumber(11)
  void clearScopes() => $_clearField(11);
}

class UpdateGiteaProviderResponse extends $pb.GeneratedMessage {
  factory UpdateGiteaProviderResponse({
    GitProvider? provider,
  }) {
    final result = UpdateGiteaProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateGiteaProviderResponse._();

  factory UpdateGiteaProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGiteaProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateGiteaProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateGiteaProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateGiteaProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateGiteaProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGiteaProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateGiteaProviderResponse copyWith(
          void Function(UpdateGiteaProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateGiteaProviderResponse))
          as UpdateGiteaProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateGiteaProviderResponse() / UpdateGiteaProviderResponse.new instead')
  static UpdateGiteaProviderResponse create() =>
      UpdateGiteaProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateGiteaProviderResponse._();
  @$core.override
  UpdateGiteaProviderResponse createEmptyInstance() =>
      UpdateGiteaProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateGiteaProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateGiteaProviderResponse>(
          UpdateGiteaProviderResponse.$_createMessage);
  static UpdateGiteaProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class UpdateBitbucketProviderRequest extends $pb.GeneratedMessage {
  factory UpdateBitbucketProviderRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.bool? shared,
    $core.String? username,
    $core.String? email,
    $core.String? appPassword,
    $core.String? apiToken,
    $core.String? workspace,
  }) {
    final result = UpdateBitbucketProviderRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (shared != null) result.shared = shared;
    if (username != null) result.username = username;
    if (email != null) result.email = email;
    if (appPassword != null) result.appPassword = appPassword;
    if (apiToken != null) result.apiToken = apiToken;
    if (workspace != null) result.workspace = workspace;
    return result;
  }

  UpdateBitbucketProviderRequest._();

  factory UpdateBitbucketProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBitbucketProviderRequest()..mergeFromBuffer(data, registry);
  factory UpdateBitbucketProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBitbucketProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateBitbucketProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateBitbucketProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOB(3, _omitFieldNames ? '' : 'shared')
    ..aOS(4, _omitFieldNames ? '' : 'username')
    ..aOS(5, _omitFieldNames ? '' : 'email')
    ..aOS(6, _omitFieldNames ? '' : 'appPassword')
    ..aOS(7, _omitFieldNames ? '' : 'apiToken')
    ..aOS(8, _omitFieldNames ? '' : 'workspace')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBitbucketProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBitbucketProviderRequest copyWith(
          void Function(UpdateBitbucketProviderRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateBitbucketProviderRequest))
          as UpdateBitbucketProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateBitbucketProviderRequest() / UpdateBitbucketProviderRequest.new instead')
  static UpdateBitbucketProviderRequest create() =>
      UpdateBitbucketProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateBitbucketProviderRequest._();
  @$core.override
  UpdateBitbucketProviderRequest createEmptyInstance() =>
      UpdateBitbucketProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateBitbucketProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateBitbucketProviderRequest>(
          UpdateBitbucketProviderRequest.$_createMessage);
  static UpdateBitbucketProviderRequest? _defaultInstance;

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
  $core.bool get shared => $_getBF(2);
  @$pb.TagNumber(3)
  set shared($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShared() => $_has(2);
  @$pb.TagNumber(3)
  void clearShared() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get username => $_getSZ(3);
  @$pb.TagNumber(4)
  set username($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUsername() => $_has(3);
  @$pb.TagNumber(4)
  void clearUsername() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get email => $_getSZ(4);
  @$pb.TagNumber(5)
  set email($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEmail() => $_has(4);
  @$pb.TagNumber(5)
  void clearEmail() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get appPassword => $_getSZ(5);
  @$pb.TagNumber(6)
  set appPassword($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAppPassword() => $_has(5);
  @$pb.TagNumber(6)
  void clearAppPassword() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get apiToken => $_getSZ(6);
  @$pb.TagNumber(7)
  set apiToken($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasApiToken() => $_has(6);
  @$pb.TagNumber(7)
  void clearApiToken() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get workspace => $_getSZ(7);
  @$pb.TagNumber(8)
  set workspace($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasWorkspace() => $_has(7);
  @$pb.TagNumber(8)
  void clearWorkspace() => $_clearField(8);
}

class UpdateBitbucketProviderResponse extends $pb.GeneratedMessage {
  factory UpdateBitbucketProviderResponse({
    GitProvider? provider,
  }) {
    final result = UpdateBitbucketProviderResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  UpdateBitbucketProviderResponse._();

  factory UpdateBitbucketProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBitbucketProviderResponse()..mergeFromBuffer(data, registry);
  factory UpdateBitbucketProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateBitbucketProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateBitbucketProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateBitbucketProviderResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBitbucketProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateBitbucketProviderResponse copyWith(
          void Function(UpdateBitbucketProviderResponse) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateBitbucketProviderResponse))
          as UpdateBitbucketProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateBitbucketProviderResponse() / UpdateBitbucketProviderResponse.new instead')
  static UpdateBitbucketProviderResponse create() =>
      UpdateBitbucketProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateBitbucketProviderResponse._();
  @$core.override
  UpdateBitbucketProviderResponse createEmptyInstance() =>
      UpdateBitbucketProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateBitbucketProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateBitbucketProviderResponse>(
          UpdateBitbucketProviderResponse.$_createMessage);
  static UpdateBitbucketProviderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class DeleteGitProviderRequest extends $pb.GeneratedMessage {
  factory DeleteGitProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteGitProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteGitProviderRequest._();

  factory DeleteGitProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteGitProviderRequest()..mergeFromBuffer(data, registry);
  factory DeleteGitProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteGitProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteGitProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteGitProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteGitProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteGitProviderRequest copyWith(
          void Function(DeleteGitProviderRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteGitProviderRequest))
          as DeleteGitProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteGitProviderRequest() / DeleteGitProviderRequest.new instead')
  static DeleteGitProviderRequest create() => DeleteGitProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteGitProviderRequest._();
  @$core.override
  DeleteGitProviderRequest createEmptyInstance() =>
      DeleteGitProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteGitProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteGitProviderRequest>(
          DeleteGitProviderRequest.$_createMessage);
  static DeleteGitProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestGitProviderRequest extends $pb.GeneratedMessage {
  factory TestGitProviderRequest({
    $fixnum.Int64? id,
  }) {
    final result = TestGitProviderRequest._();
    if (id != null) result.id = id;
    return result;
  }

  TestGitProviderRequest._();

  factory TestGitProviderRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestGitProviderRequest()..mergeFromBuffer(data, registry);
  factory TestGitProviderRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestGitProviderRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestGitProviderRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestGitProviderRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestGitProviderRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestGitProviderRequest copyWith(
          void Function(TestGitProviderRequest) updates) =>
      super.copyWith((message) => updates(message as TestGitProviderRequest))
          as TestGitProviderRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestGitProviderRequest() / TestGitProviderRequest.new instead')
  static TestGitProviderRequest create() => TestGitProviderRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestGitProviderRequest._();
  @$core.override
  TestGitProviderRequest createEmptyInstance() => TestGitProviderRequest._();
  @$core.pragma('dart2js:noInline')
  static TestGitProviderRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestGitProviderRequest>(
          TestGitProviderRequest.$_createMessage);
  static TestGitProviderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class TestGitProviderResponse extends $pb.GeneratedMessage {
  factory TestGitProviderResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestGitProviderResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestGitProviderResponse._();

  factory TestGitProviderResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestGitProviderResponse()..mergeFromBuffer(data, registry);
  factory TestGitProviderResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestGitProviderResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestGitProviderResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestGitProviderResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestGitProviderResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestGitProviderResponse copyWith(
          void Function(TestGitProviderResponse) updates) =>
      super.copyWith((message) => updates(message as TestGitProviderResponse))
          as TestGitProviderResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestGitProviderResponse() / TestGitProviderResponse.new instead')
  static TestGitProviderResponse create() => TestGitProviderResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestGitProviderResponse._();
  @$core.override
  TestGitProviderResponse createEmptyInstance() => TestGitProviderResponse._();
  @$core.pragma('dart2js:noInline')
  static TestGitProviderResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestGitProviderResponse>(
          TestGitProviderResponse.$_createMessage);
  static TestGitProviderResponse? _defaultInstance;

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

class GitRotateWebhookSecretRequest extends $pb.GeneratedMessage {
  factory GitRotateWebhookSecretRequest({
    $fixnum.Int64? id,
  }) {
    final result = GitRotateWebhookSecretRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GitRotateWebhookSecretRequest._();

  factory GitRotateWebhookSecretRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRotateWebhookSecretRequest()..mergeFromBuffer(data, registry);
  factory GitRotateWebhookSecretRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRotateWebhookSecretRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitRotateWebhookSecretRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitRotateWebhookSecretRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRotateWebhookSecretRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRotateWebhookSecretRequest copyWith(
          void Function(GitRotateWebhookSecretRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GitRotateWebhookSecretRequest))
          as GitRotateWebhookSecretRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitRotateWebhookSecretRequest() / GitRotateWebhookSecretRequest.new instead')
  static GitRotateWebhookSecretRequest create() =>
      GitRotateWebhookSecretRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitRotateWebhookSecretRequest._();
  @$core.override
  GitRotateWebhookSecretRequest createEmptyInstance() =>
      GitRotateWebhookSecretRequest._();
  @$core.pragma('dart2js:noInline')
  static GitRotateWebhookSecretRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitRotateWebhookSecretRequest>(
          GitRotateWebhookSecretRequest.$_createMessage);
  static GitRotateWebhookSecretRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GitRotateWebhookSecretResponse extends $pb.GeneratedMessage {
  factory GitRotateWebhookSecretResponse({
    $core.String? webhookSecret,
  }) {
    final result = GitRotateWebhookSecretResponse._();
    if (webhookSecret != null) result.webhookSecret = webhookSecret;
    return result;
  }

  GitRotateWebhookSecretResponse._();

  factory GitRotateWebhookSecretResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRotateWebhookSecretResponse()..mergeFromBuffer(data, registry);
  factory GitRotateWebhookSecretResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRotateWebhookSecretResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitRotateWebhookSecretResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitRotateWebhookSecretResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'webhookSecret')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRotateWebhookSecretResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRotateWebhookSecretResponse copyWith(
          void Function(GitRotateWebhookSecretResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GitRotateWebhookSecretResponse))
          as GitRotateWebhookSecretResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitRotateWebhookSecretResponse() / GitRotateWebhookSecretResponse.new instead')
  static GitRotateWebhookSecretResponse create() =>
      GitRotateWebhookSecretResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitRotateWebhookSecretResponse._();
  @$core.override
  GitRotateWebhookSecretResponse createEmptyInstance() =>
      GitRotateWebhookSecretResponse._();
  @$core.pragma('dart2js:noInline')
  static GitRotateWebhookSecretResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitRotateWebhookSecretResponse>(
          GitRotateWebhookSecretResponse.$_createMessage);
  static GitRotateWebhookSecretResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get webhookSecret => $_getSZ(0);
  @$pb.TagNumber(1)
  set webhookSecret($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhookSecret() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhookSecret() => $_clearField(1);
}

class GitListRepositoriesRequest extends $pb.GeneratedMessage {
  factory GitListRepositoriesRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = GitListRepositoriesRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  GitListRepositoriesRequest._();

  factory GitListRepositoriesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoriesRequest()..mergeFromBuffer(data, registry);
  factory GitListRepositoriesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoriesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListRepositoriesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListRepositoriesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoriesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoriesRequest copyWith(
          void Function(GitListRepositoriesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GitListRepositoriesRequest))
          as GitListRepositoriesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitListRepositoriesRequest() / GitListRepositoriesRequest.new instead')
  static GitListRepositoriesRequest create() => GitListRepositoriesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitListRepositoriesRequest._();
  @$core.override
  GitListRepositoriesRequest createEmptyInstance() =>
      GitListRepositoriesRequest._();
  @$core.pragma('dart2js:noInline')
  static GitListRepositoriesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListRepositoriesRequest>(
          GitListRepositoriesRequest.$_createMessage);
  static GitListRepositoriesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class GitListRepositoriesResponse extends $pb.GeneratedMessage {
  factory GitListRepositoriesResponse({
    $core.Iterable<GitProviderRepository>? repositories,
  }) {
    final result = GitListRepositoriesResponse._();
    if (repositories != null) result.repositories.addAll(repositories);
    return result;
  }

  GitListRepositoriesResponse._();

  factory GitListRepositoriesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoriesResponse()..mergeFromBuffer(data, registry);
  factory GitListRepositoriesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoriesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListRepositoriesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListRepositoriesResponse.$_createMessage)
    ..pPM<GitProviderRepository>(1, _omitFieldNames ? '' : 'repositories',
        subBuilder: GitProviderRepository.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoriesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoriesResponse copyWith(
          void Function(GitListRepositoriesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GitListRepositoriesResponse))
          as GitListRepositoriesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitListRepositoriesResponse() / GitListRepositoriesResponse.new instead')
  static GitListRepositoriesResponse create() =>
      GitListRepositoriesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitListRepositoriesResponse._();
  @$core.override
  GitListRepositoriesResponse createEmptyInstance() =>
      GitListRepositoriesResponse._();
  @$core.pragma('dart2js:noInline')
  static GitListRepositoriesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListRepositoriesResponse>(
          GitListRepositoriesResponse.$_createMessage);
  static GitListRepositoriesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GitProviderRepository> get repositories => $_getList(0);
}

class GitListRepositoryBranchesRequest extends $pb.GeneratedMessage {
  factory GitListRepositoryBranchesRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
  }) {
    final result = GitListRepositoryBranchesRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    return result;
  }

  GitListRepositoryBranchesRequest._();

  factory GitListRepositoryBranchesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoryBranchesRequest()..mergeFromBuffer(data, registry);
  factory GitListRepositoryBranchesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoryBranchesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListRepositoryBranchesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListRepositoryBranchesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoryBranchesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoryBranchesRequest copyWith(
          void Function(GitListRepositoryBranchesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GitListRepositoryBranchesRequest))
          as GitListRepositoryBranchesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitListRepositoryBranchesRequest() / GitListRepositoryBranchesRequest.new instead')
  static GitListRepositoryBranchesRequest create() =>
      GitListRepositoryBranchesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitListRepositoryBranchesRequest._();
  @$core.override
  GitListRepositoryBranchesRequest createEmptyInstance() =>
      GitListRepositoryBranchesRequest._();
  @$core.pragma('dart2js:noInline')
  static GitListRepositoryBranchesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListRepositoryBranchesRequest>(
          GitListRepositoryBranchesRequest.$_createMessage);
  static GitListRepositoryBranchesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);
}

class GitListRepositoryBranchesResponse extends $pb.GeneratedMessage {
  factory GitListRepositoryBranchesResponse({
    $core.Iterable<GitProviderReference>? branches,
  }) {
    final result = GitListRepositoryBranchesResponse._();
    if (branches != null) result.branches.addAll(branches);
    return result;
  }

  GitListRepositoryBranchesResponse._();

  factory GitListRepositoryBranchesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoryBranchesResponse()..mergeFromBuffer(data, registry);
  factory GitListRepositoryBranchesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListRepositoryBranchesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListRepositoryBranchesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListRepositoryBranchesResponse.$_createMessage)
    ..pPM<GitProviderReference>(1, _omitFieldNames ? '' : 'branches',
        subBuilder: GitProviderReference.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoryBranchesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListRepositoryBranchesResponse copyWith(
          void Function(GitListRepositoryBranchesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as GitListRepositoryBranchesResponse))
          as GitListRepositoryBranchesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitListRepositoryBranchesResponse() / GitListRepositoryBranchesResponse.new instead')
  static GitListRepositoryBranchesResponse create() =>
      GitListRepositoryBranchesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitListRepositoryBranchesResponse._();
  @$core.override
  GitListRepositoryBranchesResponse createEmptyInstance() =>
      GitListRepositoryBranchesResponse._();
  @$core.pragma('dart2js:noInline')
  static GitListRepositoryBranchesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListRepositoryBranchesResponse>(
          GitListRepositoryBranchesResponse.$_createMessage);
  static GitListRepositoryBranchesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GitProviderReference> get branches => $_getList(0);
}

class GitListTagsRequest extends $pb.GeneratedMessage {
  factory GitListTagsRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
  }) {
    final result = GitListTagsRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    return result;
  }

  GitListTagsRequest._();

  factory GitListTagsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListTagsRequest()..mergeFromBuffer(data, registry);
  factory GitListTagsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListTagsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListTagsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListTagsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListTagsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListTagsRequest copyWith(void Function(GitListTagsRequest) updates) =>
      super.copyWith((message) => updates(message as GitListTagsRequest))
          as GitListTagsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GitListTagsRequest() / GitListTagsRequest.new instead')
  static GitListTagsRequest create() => GitListTagsRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitListTagsRequest._();
  @$core.override
  GitListTagsRequest createEmptyInstance() => GitListTagsRequest._();
  @$core.pragma('dart2js:noInline')
  static GitListTagsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListTagsRequest>(
          GitListTagsRequest.$_createMessage);
  static GitListTagsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);
}

class GitListTagsResponse extends $pb.GeneratedMessage {
  factory GitListTagsResponse({
    $core.Iterable<GitProviderReference>? tags,
  }) {
    final result = GitListTagsResponse._();
    if (tags != null) result.tags.addAll(tags);
    return result;
  }

  GitListTagsResponse._();

  factory GitListTagsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListTagsResponse()..mergeFromBuffer(data, registry);
  factory GitListTagsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitListTagsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitListTagsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitListTagsResponse.$_createMessage)
    ..pPM<GitProviderReference>(1, _omitFieldNames ? '' : 'tags',
        subBuilder: GitProviderReference.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListTagsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitListTagsResponse copyWith(void Function(GitListTagsResponse) updates) =>
      super.copyWith((message) => updates(message as GitListTagsResponse))
          as GitListTagsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use GitListTagsResponse() / GitListTagsResponse.new instead')
  static GitListTagsResponse create() => GitListTagsResponse._();
  static $pb.GeneratedMessage $_createMessage() => GitListTagsResponse._();
  @$core.override
  GitListTagsResponse createEmptyInstance() => GitListTagsResponse._();
  @$core.pragma('dart2js:noInline')
  static GitListTagsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitListTagsResponse>(
          GitListTagsResponse.$_createMessage);
  static GitListTagsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GitProviderReference> get tags => $_getList(0);
}

class GitAuthorizeRequest extends $pb.GeneratedMessage {
  factory GitAuthorizeRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = GitAuthorizeRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  GitAuthorizeRequest._();

  factory GitAuthorizeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitAuthorizeRequest()..mergeFromBuffer(data, registry);
  factory GitAuthorizeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitAuthorizeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitAuthorizeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitAuthorizeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitAuthorizeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitAuthorizeRequest copyWith(void Function(GitAuthorizeRequest) updates) =>
      super.copyWith((message) => updates(message as GitAuthorizeRequest))
          as GitAuthorizeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use GitAuthorizeRequest() / GitAuthorizeRequest.new instead')
  static GitAuthorizeRequest create() => GitAuthorizeRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitAuthorizeRequest._();
  @$core.override
  GitAuthorizeRequest createEmptyInstance() => GitAuthorizeRequest._();
  @$core.pragma('dart2js:noInline')
  static GitAuthorizeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitAuthorizeRequest>(
          GitAuthorizeRequest.$_createMessage);
  static GitAuthorizeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class GitAuthorizeResponse extends $pb.GeneratedMessage {
  factory GitAuthorizeResponse({
    $core.String? url,
    $core.String? state,
  }) {
    final result = GitAuthorizeResponse._();
    if (url != null) result.url = url;
    if (state != null) result.state = state;
    return result;
  }

  GitAuthorizeResponse._();

  factory GitAuthorizeResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitAuthorizeResponse()..mergeFromBuffer(data, registry);
  factory GitAuthorizeResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitAuthorizeResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitAuthorizeResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitAuthorizeResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'url')
    ..aOS(2, _omitFieldNames ? '' : 'state')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitAuthorizeResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitAuthorizeResponse copyWith(void Function(GitAuthorizeResponse) updates) =>
      super.copyWith((message) => updates(message as GitAuthorizeResponse))
          as GitAuthorizeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitAuthorizeResponse() / GitAuthorizeResponse.new instead')
  static GitAuthorizeResponse create() => GitAuthorizeResponse._();
  static $pb.GeneratedMessage $_createMessage() => GitAuthorizeResponse._();
  @$core.override
  GitAuthorizeResponse createEmptyInstance() => GitAuthorizeResponse._();
  @$core.pragma('dart2js:noInline')
  static GitAuthorizeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitAuthorizeResponse>(
          GitAuthorizeResponse.$_createMessage);
  static GitAuthorizeResponse? _defaultInstance;

  /// Browser authorization URL to redirect the user to.
  @$pb.TagNumber(1)
  $core.String get url => $_getSZ(0);
  @$pb.TagNumber(1)
  set url($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearUrl() => $_clearField(1);

  /// Signed opaque state token to be echoed back by the callback.
  @$pb.TagNumber(2)
  $core.String get state => $_getSZ(1);
  @$pb.TagNumber(2)
  set state($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasState() => $_has(1);
  @$pb.TagNumber(2)
  void clearState() => $_clearField(2);
}

class GitOAuthCallbackRequest extends $pb.GeneratedMessage {
  factory GitOAuthCallbackRequest({
    $fixnum.Int64? providerId,
    $core.String? code,
    $core.String? state,
    $core.String? installationId,
    $core.String? setupAction,
  }) {
    final result = GitOAuthCallbackRequest._();
    if (providerId != null) result.providerId = providerId;
    if (code != null) result.code = code;
    if (state != null) result.state = state;
    if (installationId != null) result.installationId = installationId;
    if (setupAction != null) result.setupAction = setupAction;
    return result;
  }

  GitOAuthCallbackRequest._();

  factory GitOAuthCallbackRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitOAuthCallbackRequest()..mergeFromBuffer(data, registry);
  factory GitOAuthCallbackRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitOAuthCallbackRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitOAuthCallbackRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitOAuthCallbackRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..aOS(3, _omitFieldNames ? '' : 'state')
    ..aOS(4, _omitFieldNames ? '' : 'installationId')
    ..aOS(5, _omitFieldNames ? '' : 'setupAction')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitOAuthCallbackRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitOAuthCallbackRequest copyWith(
          void Function(GitOAuthCallbackRequest) updates) =>
      super.copyWith((message) => updates(message as GitOAuthCallbackRequest))
          as GitOAuthCallbackRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitOAuthCallbackRequest() / GitOAuthCallbackRequest.new instead')
  static GitOAuthCallbackRequest create() => GitOAuthCallbackRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitOAuthCallbackRequest._();
  @$core.override
  GitOAuthCallbackRequest createEmptyInstance() => GitOAuthCallbackRequest._();
  @$core.pragma('dart2js:noInline')
  static GitOAuthCallbackRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitOAuthCallbackRequest>(
          GitOAuthCallbackRequest.$_createMessage);
  static GitOAuthCallbackRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get state => $_getSZ(2);
  @$pb.TagNumber(3)
  set state($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasState() => $_has(2);
  @$pb.TagNumber(3)
  void clearState() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get installationId => $_getSZ(3);
  @$pb.TagNumber(4)
  set installationId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasInstallationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearInstallationId() => $_clearField(4);

  /// GitHub setup-action query parameter; accepted by the DTO but not used.
  @$pb.TagNumber(5)
  $core.String get setupAction => $_getSZ(4);
  @$pb.TagNumber(5)
  set setupAction($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSetupAction() => $_has(4);
  @$pb.TagNumber(5)
  void clearSetupAction() => $_clearField(5);
}

class GitOAuthCallbackResponse extends $pb.GeneratedMessage {
  factory GitOAuthCallbackResponse({
    GitProvider? provider,
  }) {
    final result = GitOAuthCallbackResponse._();
    if (provider != null) result.provider = provider;
    return result;
  }

  GitOAuthCallbackResponse._();

  factory GitOAuthCallbackResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitOAuthCallbackResponse()..mergeFromBuffer(data, registry);
  factory GitOAuthCallbackResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitOAuthCallbackResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitOAuthCallbackResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitOAuthCallbackResponse.$_createMessage)
    ..aOM<GitProvider>(1, _omitFieldNames ? '' : 'provider',
        subBuilder: GitProvider.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitOAuthCallbackResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitOAuthCallbackResponse copyWith(
          void Function(GitOAuthCallbackResponse) updates) =>
      super.copyWith((message) => updates(message as GitOAuthCallbackResponse))
          as GitOAuthCallbackResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitOAuthCallbackResponse() / GitOAuthCallbackResponse.new instead')
  static GitOAuthCallbackResponse create() => GitOAuthCallbackResponse._();
  static $pb.GeneratedMessage $_createMessage() => GitOAuthCallbackResponse._();
  @$core.override
  GitOAuthCallbackResponse createEmptyInstance() =>
      GitOAuthCallbackResponse._();
  @$core.pragma('dart2js:noInline')
  static GitOAuthCallbackResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitOAuthCallbackResponse>(
          GitOAuthCallbackResponse.$_createMessage);
  static GitOAuthCallbackResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProvider get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(GitProvider value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProvider ensureProvider() => $_ensure(0);
}

class GitDisconnectRequest extends $pb.GeneratedMessage {
  factory GitDisconnectRequest({
    $fixnum.Int64? providerId,
  }) {
    final result = GitDisconnectRequest._();
    if (providerId != null) result.providerId = providerId;
    return result;
  }

  GitDisconnectRequest._();

  factory GitDisconnectRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitDisconnectRequest()..mergeFromBuffer(data, registry);
  factory GitDisconnectRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitDisconnectRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitDisconnectRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitDisconnectRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitDisconnectRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitDisconnectRequest copyWith(void Function(GitDisconnectRequest) updates) =>
      super.copyWith((message) => updates(message as GitDisconnectRequest))
          as GitDisconnectRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitDisconnectRequest() / GitDisconnectRequest.new instead')
  static GitDisconnectRequest create() => GitDisconnectRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitDisconnectRequest._();
  @$core.override
  GitDisconnectRequest createEmptyInstance() => GitDisconnectRequest._();
  @$core.pragma('dart2js:noInline')
  static GitDisconnectRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitDisconnectRequest>(
          GitDisconnectRequest.$_createMessage);
  static GitDisconnectRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);
}

class GitGetCollaboratorPermissionRequest extends $pb.GeneratedMessage {
  factory GitGetCollaboratorPermissionRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? username,
  }) {
    final result = GitGetCollaboratorPermissionRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (username != null) result.username = username;
    return result;
  }

  GitGetCollaboratorPermissionRequest._();

  factory GitGetCollaboratorPermissionRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetCollaboratorPermissionRequest()..mergeFromBuffer(data, registry);
  factory GitGetCollaboratorPermissionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetCollaboratorPermissionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitGetCollaboratorPermissionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitGetCollaboratorPermissionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..aOS(4, _omitFieldNames ? '' : 'username')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetCollaboratorPermissionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetCollaboratorPermissionRequest copyWith(
          void Function(GitGetCollaboratorPermissionRequest) updates) =>
      super.copyWith((message) =>
              updates(message as GitGetCollaboratorPermissionRequest))
          as GitGetCollaboratorPermissionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitGetCollaboratorPermissionRequest() / GitGetCollaboratorPermissionRequest.new instead')
  static GitGetCollaboratorPermissionRequest create() =>
      GitGetCollaboratorPermissionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitGetCollaboratorPermissionRequest._();
  @$core.override
  GitGetCollaboratorPermissionRequest createEmptyInstance() =>
      GitGetCollaboratorPermissionRequest._();
  @$core.pragma('dart2js:noInline')
  static GitGetCollaboratorPermissionRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              GitGetCollaboratorPermissionRequest>(
          GitGetCollaboratorPermissionRequest.$_createMessage);
  static GitGetCollaboratorPermissionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get username => $_getSZ(3);
  @$pb.TagNumber(4)
  set username($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasUsername() => $_has(3);
  @$pb.TagNumber(4)
  void clearUsername() => $_clearField(4);
}

class GitGetCollaboratorPermissionResponse extends $pb.GeneratedMessage {
  factory GitGetCollaboratorPermissionResponse({
    GitProviderCollaboratorPermission? permission,
  }) {
    final result = GitGetCollaboratorPermissionResponse._();
    if (permission != null) result.permission = permission;
    return result;
  }

  GitGetCollaboratorPermissionResponse._();

  factory GitGetCollaboratorPermissionResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetCollaboratorPermissionResponse()..mergeFromBuffer(data, registry);
  factory GitGetCollaboratorPermissionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetCollaboratorPermissionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitGetCollaboratorPermissionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitGetCollaboratorPermissionResponse.$_createMessage)
    ..aOM<GitProviderCollaboratorPermission>(
        1, _omitFieldNames ? '' : 'permission',
        subBuilder: GitProviderCollaboratorPermission.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetCollaboratorPermissionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetCollaboratorPermissionResponse copyWith(
          void Function(GitGetCollaboratorPermissionResponse) updates) =>
      super.copyWith((message) =>
              updates(message as GitGetCollaboratorPermissionResponse))
          as GitGetCollaboratorPermissionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitGetCollaboratorPermissionResponse() / GitGetCollaboratorPermissionResponse.new instead')
  static GitGetCollaboratorPermissionResponse create() =>
      GitGetCollaboratorPermissionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitGetCollaboratorPermissionResponse._();
  @$core.override
  GitGetCollaboratorPermissionResponse createEmptyInstance() =>
      GitGetCollaboratorPermissionResponse._();
  @$core.pragma('dart2js:noInline')
  static GitGetCollaboratorPermissionResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              GitGetCollaboratorPermissionResponse>(
          GitGetCollaboratorPermissionResponse.$_createMessage);
  static GitGetCollaboratorPermissionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProviderCollaboratorPermission get permission => $_getN(0);
  @$pb.TagNumber(1)
  set permission(GitProviderCollaboratorPermission value) =>
      $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPermission() => $_has(0);
  @$pb.TagNumber(1)
  void clearPermission() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProviderCollaboratorPermission ensurePermission() => $_ensure(0);
}

class GitGetWebhookStatusRequest extends $pb.GeneratedMessage {
  factory GitGetWebhookStatusRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = GitGetWebhookStatusRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  GitGetWebhookStatusRequest._();

  factory GitGetWebhookStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetWebhookStatusRequest()..mergeFromBuffer(data, registry);
  factory GitGetWebhookStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetWebhookStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitGetWebhookStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitGetWebhookStatusRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..aOS(4, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetWebhookStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetWebhookStatusRequest copyWith(
          void Function(GitGetWebhookStatusRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GitGetWebhookStatusRequest))
          as GitGetWebhookStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitGetWebhookStatusRequest() / GitGetWebhookStatusRequest.new instead')
  static GitGetWebhookStatusRequest create() => GitGetWebhookStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitGetWebhookStatusRequest._();
  @$core.override
  GitGetWebhookStatusRequest createEmptyInstance() =>
      GitGetWebhookStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static GitGetWebhookStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitGetWebhookStatusRequest>(
          GitGetWebhookStatusRequest.$_createMessage);
  static GitGetWebhookStatusRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get callbackUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set callbackUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCallbackUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearCallbackUrl() => $_clearField(4);
}

class GitGetWebhookStatusResponse extends $pb.GeneratedMessage {
  factory GitGetWebhookStatusResponse({
    GitProviderWebhookInfo? webhook,
  }) {
    final result = GitGetWebhookStatusResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  GitGetWebhookStatusResponse._();

  factory GitGetWebhookStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetWebhookStatusResponse()..mergeFromBuffer(data, registry);
  factory GitGetWebhookStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitGetWebhookStatusResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitGetWebhookStatusResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitGetWebhookStatusResponse.$_createMessage)
    ..aOM<GitProviderWebhookInfo>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: GitProviderWebhookInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetWebhookStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitGetWebhookStatusResponse copyWith(
          void Function(GitGetWebhookStatusResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GitGetWebhookStatusResponse))
          as GitGetWebhookStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitGetWebhookStatusResponse() / GitGetWebhookStatusResponse.new instead')
  static GitGetWebhookStatusResponse create() =>
      GitGetWebhookStatusResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitGetWebhookStatusResponse._();
  @$core.override
  GitGetWebhookStatusResponse createEmptyInstance() =>
      GitGetWebhookStatusResponse._();
  @$core.pragma('dart2js:noInline')
  static GitGetWebhookStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitGetWebhookStatusResponse>(
          GitGetWebhookStatusResponse.$_createMessage);
  static GitGetWebhookStatusResponse? _defaultInstance;

  /// Absent when no matching webhook is registered on the repository.
  @$pb.TagNumber(1)
  GitProviderWebhookInfo get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(GitProviderWebhookInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProviderWebhookInfo ensureWebhook() => $_ensure(0);
}

class GitInstallWebhookRequest extends $pb.GeneratedMessage {
  factory GitInstallWebhookRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = GitInstallWebhookRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  GitInstallWebhookRequest._();

  factory GitInstallWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitInstallWebhookRequest()..mergeFromBuffer(data, registry);
  factory GitInstallWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitInstallWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitInstallWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitInstallWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..aOS(4, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitInstallWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitInstallWebhookRequest copyWith(
          void Function(GitInstallWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as GitInstallWebhookRequest))
          as GitInstallWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitInstallWebhookRequest() / GitInstallWebhookRequest.new instead')
  static GitInstallWebhookRequest create() => GitInstallWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitInstallWebhookRequest._();
  @$core.override
  GitInstallWebhookRequest createEmptyInstance() =>
      GitInstallWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static GitInstallWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitInstallWebhookRequest>(
          GitInstallWebhookRequest.$_createMessage);
  static GitInstallWebhookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get callbackUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set callbackUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCallbackUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearCallbackUrl() => $_clearField(4);
}

class GitInstallWebhookResponse extends $pb.GeneratedMessage {
  factory GitInstallWebhookResponse({
    GitProviderWebhookInfo? webhook,
  }) {
    final result = GitInstallWebhookResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  GitInstallWebhookResponse._();

  factory GitInstallWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitInstallWebhookResponse()..mergeFromBuffer(data, registry);
  factory GitInstallWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitInstallWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitInstallWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitInstallWebhookResponse.$_createMessage)
    ..aOM<GitProviderWebhookInfo>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: GitProviderWebhookInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitInstallWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitInstallWebhookResponse copyWith(
          void Function(GitInstallWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as GitInstallWebhookResponse))
          as GitInstallWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitInstallWebhookResponse() / GitInstallWebhookResponse.new instead')
  static GitInstallWebhookResponse create() => GitInstallWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitInstallWebhookResponse._();
  @$core.override
  GitInstallWebhookResponse createEmptyInstance() =>
      GitInstallWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static GitInstallWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitInstallWebhookResponse>(
          GitInstallWebhookResponse.$_createMessage);
  static GitInstallWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProviderWebhookInfo get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(GitProviderWebhookInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProviderWebhookInfo ensureWebhook() => $_ensure(0);
}

class GitRecreateWebhookRequest extends $pb.GeneratedMessage {
  factory GitRecreateWebhookRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = GitRecreateWebhookRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  GitRecreateWebhookRequest._();

  factory GitRecreateWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRecreateWebhookRequest()..mergeFromBuffer(data, registry);
  factory GitRecreateWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRecreateWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitRecreateWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitRecreateWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..aOS(4, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRecreateWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRecreateWebhookRequest copyWith(
          void Function(GitRecreateWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as GitRecreateWebhookRequest))
          as GitRecreateWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitRecreateWebhookRequest() / GitRecreateWebhookRequest.new instead')
  static GitRecreateWebhookRequest create() => GitRecreateWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitRecreateWebhookRequest._();
  @$core.override
  GitRecreateWebhookRequest createEmptyInstance() =>
      GitRecreateWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static GitRecreateWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitRecreateWebhookRequest>(
          GitRecreateWebhookRequest.$_createMessage);
  static GitRecreateWebhookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get callbackUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set callbackUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCallbackUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearCallbackUrl() => $_clearField(4);
}

class GitRecreateWebhookResponse extends $pb.GeneratedMessage {
  factory GitRecreateWebhookResponse({
    GitProviderWebhookInfo? webhook,
  }) {
    final result = GitRecreateWebhookResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  GitRecreateWebhookResponse._();

  factory GitRecreateWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRecreateWebhookResponse()..mergeFromBuffer(data, registry);
  factory GitRecreateWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitRecreateWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitRecreateWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitRecreateWebhookResponse.$_createMessage)
    ..aOM<GitProviderWebhookInfo>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: GitProviderWebhookInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRecreateWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitRecreateWebhookResponse copyWith(
          void Function(GitRecreateWebhookResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GitRecreateWebhookResponse))
          as GitRecreateWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitRecreateWebhookResponse() / GitRecreateWebhookResponse.new instead')
  static GitRecreateWebhookResponse create() => GitRecreateWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GitRecreateWebhookResponse._();
  @$core.override
  GitRecreateWebhookResponse createEmptyInstance() =>
      GitRecreateWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static GitRecreateWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitRecreateWebhookResponse>(
          GitRecreateWebhookResponse.$_createMessage);
  static GitRecreateWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  GitProviderWebhookInfo get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(GitProviderWebhookInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  GitProviderWebhookInfo ensureWebhook() => $_ensure(0);
}

class GitDeleteWebhookRequest extends $pb.GeneratedMessage {
  factory GitDeleteWebhookRequest({
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = GitDeleteWebhookRequest._();
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  GitDeleteWebhookRequest._();

  factory GitDeleteWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitDeleteWebhookRequest()..mergeFromBuffer(data, registry);
  factory GitDeleteWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GitDeleteWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GitDeleteWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GitDeleteWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'providerId')
    ..aOS(2, _omitFieldNames ? '' : 'owner')
    ..aOS(3, _omitFieldNames ? '' : 'repository')
    ..aOS(4, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitDeleteWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GitDeleteWebhookRequest copyWith(
          void Function(GitDeleteWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as GitDeleteWebhookRequest))
          as GitDeleteWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GitDeleteWebhookRequest() / GitDeleteWebhookRequest.new instead')
  static GitDeleteWebhookRequest create() => GitDeleteWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => GitDeleteWebhookRequest._();
  @$core.override
  GitDeleteWebhookRequest createEmptyInstance() => GitDeleteWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static GitDeleteWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GitDeleteWebhookRequest>(
          GitDeleteWebhookRequest.$_createMessage);
  static GitDeleteWebhookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get providerId => $_getI64(0);
  @$pb.TagNumber(1)
  set providerId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProviderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProviderId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get owner => $_getSZ(1);
  @$pb.TagNumber(2)
  set owner($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get repository => $_getSZ(2);
  @$pb.TagNumber(3)
  set repository($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRepository() => $_has(2);
  @$pb.TagNumber(3)
  void clearRepository() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get callbackUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set callbackUrl($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCallbackUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearCallbackUrl() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/compose.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'compose.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'compose.pbenum.dart';

class Compose extends $pb.GeneratedMessage {
  factory Compose({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $core.String? status,
    $core.String? sourceType,
    $core.String? repository,
    $core.String? branch,
    $core.String? composeFile,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
    $core.String? owner,
    $core.String? gitlabRepository,
    $core.String? gitlabOwner,
    $core.String? gitlabBranch,
    $core.String? giteaRepository,
    $core.String? giteaOwner,
    $core.String? giteaBranch,
    $core.String? bitbucketRepository,
    $core.String? bitbucketOwner,
    $core.String? bitbucketBranch,
    $core.String? customGitUrl,
    $core.String? customGitBranch,
    $core.String? command,
    $core.String? composePath,
    $core.String? composeType,
    $core.String? lastError,
    $core.String? buildPath,
    $fixnum.Int64? customGitSshKeyId,
    $core.bool? randomize,
    $core.String? suffix,
    $core.String? envVar,
  }) {
    final result = Compose._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (status != null) result.status = status;
    if (sourceType != null) result.sourceType = sourceType;
    if (repository != null) result.repository = repository;
    if (branch != null) result.branch = branch;
    if (composeFile != null) result.composeFile = composeFile;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (owner != null) result.owner = owner;
    if (gitlabRepository != null) result.gitlabRepository = gitlabRepository;
    if (gitlabOwner != null) result.gitlabOwner = gitlabOwner;
    if (gitlabBranch != null) result.gitlabBranch = gitlabBranch;
    if (giteaRepository != null) result.giteaRepository = giteaRepository;
    if (giteaOwner != null) result.giteaOwner = giteaOwner;
    if (giteaBranch != null) result.giteaBranch = giteaBranch;
    if (bitbucketRepository != null)
      result.bitbucketRepository = bitbucketRepository;
    if (bitbucketOwner != null) result.bitbucketOwner = bitbucketOwner;
    if (bitbucketBranch != null) result.bitbucketBranch = bitbucketBranch;
    if (customGitUrl != null) result.customGitUrl = customGitUrl;
    if (customGitBranch != null) result.customGitBranch = customGitBranch;
    if (command != null) result.command = command;
    if (composePath != null) result.composePath = composePath;
    if (composeType != null) result.composeType = composeType;
    if (lastError != null) result.lastError = lastError;
    if (buildPath != null) result.buildPath = buildPath;
    if (customGitSshKeyId != null) result.customGitSshKeyId = customGitSshKeyId;
    if (randomize != null) result.randomize = randomize;
    if (suffix != null) result.suffix = suffix;
    if (envVar != null) result.envVar = envVar;
    return result;
  }

  Compose._();

  factory Compose.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Compose()..mergeFromBuffer(data, registry);
  factory Compose.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Compose()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Compose',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Compose.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aInt64(4, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(5, _omitFieldNames ? '' : 'serverId')
    ..aOS(6, _omitFieldNames ? '' : 'status')
    ..aOS(7, _omitFieldNames ? '' : 'sourceType')
    ..aOS(8, _omitFieldNames ? '' : 'repository')
    ..aOS(9, _omitFieldNames ? '' : 'branch')
    ..aOS(10, _omitFieldNames ? '' : 'composeFile')
    ..aInt64(11, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(12, _omitFieldNames ? '' : 'updatedAt')
    ..aOS(13, _omitFieldNames ? '' : 'owner')
    ..aOS(14, _omitFieldNames ? '' : 'gitlabRepository')
    ..aOS(15, _omitFieldNames ? '' : 'gitlabOwner')
    ..aOS(16, _omitFieldNames ? '' : 'gitlabBranch')
    ..aOS(17, _omitFieldNames ? '' : 'giteaRepository')
    ..aOS(18, _omitFieldNames ? '' : 'giteaOwner')
    ..aOS(19, _omitFieldNames ? '' : 'giteaBranch')
    ..aOS(20, _omitFieldNames ? '' : 'bitbucketRepository')
    ..aOS(21, _omitFieldNames ? '' : 'bitbucketOwner')
    ..aOS(22, _omitFieldNames ? '' : 'bitbucketBranch')
    ..aOS(23, _omitFieldNames ? '' : 'customGitUrl')
    ..aOS(24, _omitFieldNames ? '' : 'customGitBranch')
    ..aOS(25, _omitFieldNames ? '' : 'command')
    ..aOS(26, _omitFieldNames ? '' : 'composePath')
    ..aOS(27, _omitFieldNames ? '' : 'composeType')
    ..aOS(28, _omitFieldNames ? '' : 'lastError')
    ..aOS(29, _omitFieldNames ? '' : 'buildPath')
    ..aInt64(30, _omitFieldNames ? '' : 'customGitSshKeyId')
    ..aOB(31, _omitFieldNames ? '' : 'randomize')
    ..aOS(32, _omitFieldNames ? '' : 'suffix')
    ..aOS(33, _omitFieldNames ? '' : 'envVar')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Compose clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Compose copyWith(void Function(Compose) updates) =>
      super.copyWith((message) => updates(message as Compose)) as Compose;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Compose() / Compose.new instead')
  static Compose create() => Compose._();
  static $pb.GeneratedMessage $_createMessage() => Compose._();
  @$core.override
  Compose createEmptyInstance() => Compose._();
  @$core.pragma('dart2js:noInline')
  static Compose getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Compose>(Compose.$_createMessage);
  static Compose? _defaultInstance;

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
  $core.String get status => $_getSZ(5);
  @$pb.TagNumber(6)
  set status($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(6)
  void clearStatus() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get sourceType => $_getSZ(6);
  @$pb.TagNumber(7)
  set sourceType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSourceType() => $_has(6);
  @$pb.TagNumber(7)
  void clearSourceType() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get repository => $_getSZ(7);
  @$pb.TagNumber(8)
  set repository($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasRepository() => $_has(7);
  @$pb.TagNumber(8)
  void clearRepository() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get branch => $_getSZ(8);
  @$pb.TagNumber(9)
  set branch($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasBranch() => $_has(8);
  @$pb.TagNumber(9)
  void clearBranch() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get composeFile => $_getSZ(9);
  @$pb.TagNumber(10)
  set composeFile($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasComposeFile() => $_has(9);
  @$pb.TagNumber(10)
  void clearComposeFile() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get createdAt => $_getI64(10);
  @$pb.TagNumber(11)
  set createdAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCreatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCreatedAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get updatedAt => $_getI64(11);
  @$pb.TagNumber(12)
  set updatedAt($fixnum.Int64 value) => $_setInt64(11, value);
  @$pb.TagNumber(12)
  $core.bool hasUpdatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearUpdatedAt() => $_clearField(12);

  /// Source round-trip: the source-settings card re-seeds from these after save/refetch; without them every provider except GitHub looked unset.
  @$pb.TagNumber(13)
  $core.String get owner => $_getSZ(12);
  @$pb.TagNumber(13)
  set owner($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasOwner() => $_has(12);
  @$pb.TagNumber(13)
  void clearOwner() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get gitlabRepository => $_getSZ(13);
  @$pb.TagNumber(14)
  set gitlabRepository($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasGitlabRepository() => $_has(13);
  @$pb.TagNumber(14)
  void clearGitlabRepository() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get gitlabOwner => $_getSZ(14);
  @$pb.TagNumber(15)
  set gitlabOwner($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasGitlabOwner() => $_has(14);
  @$pb.TagNumber(15)
  void clearGitlabOwner() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get gitlabBranch => $_getSZ(15);
  @$pb.TagNumber(16)
  set gitlabBranch($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasGitlabBranch() => $_has(15);
  @$pb.TagNumber(16)
  void clearGitlabBranch() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.String get giteaRepository => $_getSZ(16);
  @$pb.TagNumber(17)
  set giteaRepository($core.String value) => $_setString(16, value);
  @$pb.TagNumber(17)
  $core.bool hasGiteaRepository() => $_has(16);
  @$pb.TagNumber(17)
  void clearGiteaRepository() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.String get giteaOwner => $_getSZ(17);
  @$pb.TagNumber(18)
  set giteaOwner($core.String value) => $_setString(17, value);
  @$pb.TagNumber(18)
  $core.bool hasGiteaOwner() => $_has(17);
  @$pb.TagNumber(18)
  void clearGiteaOwner() => $_clearField(18);

  @$pb.TagNumber(19)
  $core.String get giteaBranch => $_getSZ(18);
  @$pb.TagNumber(19)
  set giteaBranch($core.String value) => $_setString(18, value);
  @$pb.TagNumber(19)
  $core.bool hasGiteaBranch() => $_has(18);
  @$pb.TagNumber(19)
  void clearGiteaBranch() => $_clearField(19);

  @$pb.TagNumber(20)
  $core.String get bitbucketRepository => $_getSZ(19);
  @$pb.TagNumber(20)
  set bitbucketRepository($core.String value) => $_setString(19, value);
  @$pb.TagNumber(20)
  $core.bool hasBitbucketRepository() => $_has(19);
  @$pb.TagNumber(20)
  void clearBitbucketRepository() => $_clearField(20);

  @$pb.TagNumber(21)
  $core.String get bitbucketOwner => $_getSZ(20);
  @$pb.TagNumber(21)
  set bitbucketOwner($core.String value) => $_setString(20, value);
  @$pb.TagNumber(21)
  $core.bool hasBitbucketOwner() => $_has(20);
  @$pb.TagNumber(21)
  void clearBitbucketOwner() => $_clearField(21);

  @$pb.TagNumber(22)
  $core.String get bitbucketBranch => $_getSZ(21);
  @$pb.TagNumber(22)
  set bitbucketBranch($core.String value) => $_setString(21, value);
  @$pb.TagNumber(22)
  $core.bool hasBitbucketBranch() => $_has(21);
  @$pb.TagNumber(22)
  void clearBitbucketBranch() => $_clearField(22);

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
  $core.String get command => $_getSZ(24);
  @$pb.TagNumber(25)
  set command($core.String value) => $_setString(24, value);
  @$pb.TagNumber(25)
  $core.bool hasCommand() => $_has(24);
  @$pb.TagNumber(25)
  void clearCommand() => $_clearField(25);

  @$pb.TagNumber(26)
  $core.String get composePath => $_getSZ(25);
  @$pb.TagNumber(26)
  set composePath($core.String value) => $_setString(25, value);
  @$pb.TagNumber(26)
  $core.bool hasComposePath() => $_has(25);
  @$pb.TagNumber(26)
  void clearComposePath() => $_clearField(26);

  @$pb.TagNumber(27)
  $core.String get composeType => $_getSZ(26);
  @$pb.TagNumber(27)
  set composeType($core.String value) => $_setString(26, value);
  @$pb.TagNumber(27)
  $core.bool hasComposeType() => $_has(26);
  @$pb.TagNumber(27)
  void clearComposeType() => $_clearField(27);

  @$pb.TagNumber(28)
  $core.String get lastError => $_getSZ(27);
  @$pb.TagNumber(28)
  set lastError($core.String value) => $_setString(27, value);
  @$pb.TagNumber(28)
  $core.bool hasLastError() => $_has(27);
  @$pb.TagNumber(28)
  void clearLastError() => $_clearField(28);

  @$pb.TagNumber(29)
  $core.String get buildPath => $_getSZ(28);
  @$pb.TagNumber(29)
  set buildPath($core.String value) => $_setString(28, value);
  @$pb.TagNumber(29)
  $core.bool hasBuildPath() => $_has(28);
  @$pb.TagNumber(29)
  void clearBuildPath() => $_clearField(29);

  @$pb.TagNumber(30)
  $fixnum.Int64 get customGitSshKeyId => $_getI64(29);
  @$pb.TagNumber(30)
  set customGitSshKeyId($fixnum.Int64 value) => $_setInt64(29, value);
  @$pb.TagNumber(30)
  $core.bool hasCustomGitSshKeyId() => $_has(29);
  @$pb.TagNumber(30)
  void clearCustomGitSshKeyId() => $_clearField(30);

  /// Name-randomization state: when true, deploy renames every service to `{name}-{suffix}`.
  @$pb.TagNumber(31)
  $core.bool get randomize => $_getBF(30);
  @$pb.TagNumber(31)
  set randomize($core.bool value) => $_setBool(30, value);
  @$pb.TagNumber(31)
  $core.bool hasRandomize() => $_has(30);
  @$pb.TagNumber(31)
  void clearRandomize() => $_clearField(31);

  @$pb.TagNumber(32)
  $core.String get suffix => $_getSZ(31);
  @$pb.TagNumber(32)
  set suffix($core.String value) => $_setString(31, value);
  @$pb.TagNumber(32)
  $core.bool hasSuffix() => $_has(31);
  @$pb.TagNumber(32)
  void clearSuffix() => $_clearField(32);

  /// Resolved environment (.env-style KEY=VALUE lines), including any secrets a
  /// template generated at install time, so the GUI env editor can display and
  /// edit them.
  @$pb.TagNumber(33)
  $core.String get envVar => $_getSZ(32);
  @$pb.TagNumber(33)
  set envVar($core.String value) => $_setString(32, value);
  @$pb.TagNumber(33)
  $core.bool hasEnvVar() => $_has(32);
  @$pb.TagNumber(33)
  void clearEnvVar() => $_clearField(33);
}

class GetComposeRequest extends $pb.GeneratedMessage {
  factory GetComposeRequest({
    $fixnum.Int64? id,
  }) {
    final result = GetComposeRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetComposeRequest._();

  factory GetComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetComposeRequest()..mergeFromBuffer(data, registry);
  factory GetComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetComposeRequest copyWith(void Function(GetComposeRequest) updates) =>
      super.copyWith((message) => updates(message as GetComposeRequest))
          as GetComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetComposeRequest() / GetComposeRequest.new instead')
  static GetComposeRequest create() => GetComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetComposeRequest._();
  @$core.override
  GetComposeRequest createEmptyInstance() => GetComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static GetComposeRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetComposeRequest>(
          GetComposeRequest.$_createMessage);
  static GetComposeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ListComposesRequest extends $pb.GeneratedMessage {
  factory ListComposesRequest({
    $fixnum.Int64? environmentId,
  }) {
    final result = ListComposesRequest._();
    if (environmentId != null) result.environmentId = environmentId;
    return result;
  }

  ListComposesRequest._();

  factory ListComposesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposesRequest()..mergeFromBuffer(data, registry);
  factory ListComposesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListComposesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListComposesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'environmentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposesRequest copyWith(void Function(ListComposesRequest) updates) =>
      super.copyWith((message) => updates(message as ListComposesRequest))
          as ListComposesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListComposesRequest() / ListComposesRequest.new instead')
  static ListComposesRequest create() => ListComposesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListComposesRequest._();
  @$core.override
  ListComposesRequest createEmptyInstance() => ListComposesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListComposesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListComposesRequest>(
          ListComposesRequest.$_createMessage);
  static ListComposesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get environmentId => $_getI64(0);
  @$pb.TagNumber(1)
  set environmentId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEnvironmentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnvironmentId() => $_clearField(1);
}

class ListComposesResponse extends $pb.GeneratedMessage {
  factory ListComposesResponse({
    $core.Iterable<Compose>? composes,
  }) {
    final result = ListComposesResponse._();
    if (composes != null) result.composes.addAll(composes);
    return result;
  }

  ListComposesResponse._();

  factory ListComposesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposesResponse()..mergeFromBuffer(data, registry);
  factory ListComposesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListComposesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListComposesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListComposesResponse.$_createMessage)
    ..pPM<Compose>(1, _omitFieldNames ? '' : 'composes',
        subBuilder: Compose.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListComposesResponse copyWith(void Function(ListComposesResponse) updates) =>
      super.copyWith((message) => updates(message as ListComposesResponse))
          as ListComposesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListComposesResponse() / ListComposesResponse.new instead')
  static ListComposesResponse create() => ListComposesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListComposesResponse._();
  @$core.override
  ListComposesResponse createEmptyInstance() => ListComposesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListComposesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListComposesResponse>(
          ListComposesResponse.$_createMessage);
  static ListComposesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Compose> get composes => $_getList(0);
}

class CreateComposeRequest extends $pb.GeneratedMessage {
  factory CreateComposeRequest({
    $core.String? name,
    $core.String? description,
    $fixnum.Int64? environmentId,
    $fixnum.Int64? serverId,
    $core.String? composeFile,
    $core.String? composeType,
  }) {
    final result = CreateComposeRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (environmentId != null) result.environmentId = environmentId;
    if (serverId != null) result.serverId = serverId;
    if (composeFile != null) result.composeFile = composeFile;
    if (composeType != null) result.composeType = composeType;
    return result;
  }

  CreateComposeRequest._();

  factory CreateComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateComposeRequest()..mergeFromBuffer(data, registry);
  factory CreateComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateComposeRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aInt64(3, _omitFieldNames ? '' : 'environmentId')
    ..aInt64(4, _omitFieldNames ? '' : 'serverId')
    ..aOS(5, _omitFieldNames ? '' : 'composeFile')
    ..aOS(6, _omitFieldNames ? '' : 'composeType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateComposeRequest copyWith(void Function(CreateComposeRequest) updates) =>
      super.copyWith((message) => updates(message as CreateComposeRequest))
          as CreateComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateComposeRequest() / CreateComposeRequest.new instead')
  static CreateComposeRequest create() => CreateComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateComposeRequest._();
  @$core.override
  CreateComposeRequest createEmptyInstance() => CreateComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateComposeRequest>(
          CreateComposeRequest.$_createMessage);
  static CreateComposeRequest? _defaultInstance;

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
  $core.String get composeFile => $_getSZ(4);
  @$pb.TagNumber(5)
  set composeFile($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasComposeFile() => $_has(4);
  @$pb.TagNumber(5)
  void clearComposeFile() => $_clearField(5);

  /// Canonical ComposeType spelling ("DOCKER-COMPOSE" | "STACK"); unset defaults to DOCKER-COMPOSE. Unknown values are rejected, not coerced.
  @$pb.TagNumber(6)
  $core.String get composeType => $_getSZ(5);
  @$pb.TagNumber(6)
  set composeType($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasComposeType() => $_has(5);
  @$pb.TagNumber(6)
  void clearComposeType() => $_clearField(6);
}

class UpdateComposeRequest extends $pb.GeneratedMessage {
  factory UpdateComposeRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? composeFile,
    $fixnum.Int64? serverId,
    $core.Iterable<$core.String>? networkIds,
    $core.String? command,
    $core.bool? randomize,
    $core.String? envVar,
  }) {
    final result = UpdateComposeRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (composeFile != null) result.composeFile = composeFile;
    if (serverId != null) result.serverId = serverId;
    if (networkIds != null) result.networkIds.addAll(networkIds);
    if (command != null) result.command = command;
    if (randomize != null) result.randomize = randomize;
    if (envVar != null) result.envVar = envVar;
    return result;
  }

  UpdateComposeRequest._();

  factory UpdateComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateComposeRequest()..mergeFromBuffer(data, registry);
  factory UpdateComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'composeFile')
    ..aInt64(5, _omitFieldNames ? '' : 'serverId')
    ..pPS(6, _omitFieldNames ? '' : 'networkIds')
    ..aOS(7, _omitFieldNames ? '' : 'command')
    ..aOB(8, _omitFieldNames ? '' : 'randomize')
    ..aOS(9, _omitFieldNames ? '' : 'envVar')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateComposeRequest copyWith(void Function(UpdateComposeRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateComposeRequest))
          as UpdateComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateComposeRequest() / UpdateComposeRequest.new instead')
  static UpdateComposeRequest create() => UpdateComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateComposeRequest._();
  @$core.override
  UpdateComposeRequest createEmptyInstance() => UpdateComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateComposeRequest>(
          UpdateComposeRequest.$_createMessage);
  static UpdateComposeRequest? _defaultInstance;

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
  $core.String get composeFile => $_getSZ(3);
  @$pb.TagNumber(4)
  set composeFile($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasComposeFile() => $_has(3);
  @$pb.TagNumber(4)
  void clearComposeFile() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get serverId => $_getI64(4);
  @$pb.TagNumber(5)
  set serverId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasServerId() => $_has(4);
  @$pb.TagNumber(5)
  void clearServerId() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<$core.String> get networkIds => $_getList(5);

  @$pb.TagNumber(7)
  $core.String get command => $_getSZ(6);
  @$pb.TagNumber(7)
  set command($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCommand() => $_has(6);
  @$pb.TagNumber(7)
  void clearCommand() => $_clearField(7);

  /// Toggle service-name randomization. false clears the stored suffix so services keep their literal names.
  @$pb.TagNumber(8)
  $core.bool get randomize => $_getBF(7);
  @$pb.TagNumber(8)
  set randomize($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasRandomize() => $_has(7);
  @$pb.TagNumber(8)
  void clearRandomize() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get envVar => $_getSZ(8);
  @$pb.TagNumber(9)
  set envVar($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEnvVar() => $_has(8);
  @$pb.TagNumber(9)
  void clearEnvVar() => $_clearField(9);
}

class DeleteComposeRequest extends $pb.GeneratedMessage {
  factory DeleteComposeRequest({
    $fixnum.Int64? id,
  }) {
    final result = DeleteComposeRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteComposeRequest._();

  factory DeleteComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteComposeRequest()..mergeFromBuffer(data, registry);
  factory DeleteComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteComposeRequest copyWith(void Function(DeleteComposeRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteComposeRequest))
          as DeleteComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteComposeRequest() / DeleteComposeRequest.new instead')
  static DeleteComposeRequest create() => DeleteComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteComposeRequest._();
  @$core.override
  DeleteComposeRequest createEmptyInstance() => DeleteComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteComposeRequest>(
          DeleteComposeRequest.$_createMessage);
  static DeleteComposeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ComposeOperationRequest extends $pb.GeneratedMessage {
  factory ComposeOperationRequest({
    $fixnum.Int64? id,
  }) {
    final result = ComposeOperationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  ComposeOperationRequest._();

  factory ComposeOperationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeOperationRequest()..mergeFromBuffer(data, registry);
  factory ComposeOperationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeOperationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeOperationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeOperationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeOperationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeOperationRequest copyWith(
          void Function(ComposeOperationRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeOperationRequest))
          as ComposeOperationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeOperationRequest() / ComposeOperationRequest.new instead')
  static ComposeOperationRequest create() => ComposeOperationRequest._();
  static $pb.GeneratedMessage $_createMessage() => ComposeOperationRequest._();
  @$core.override
  ComposeOperationRequest createEmptyInstance() => ComposeOperationRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeOperationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeOperationRequest>(
          ComposeOperationRequest.$_createMessage);
  static ComposeOperationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class ComposeOperation extends $pb.GeneratedMessage {
  factory ComposeOperation({
    Compose? compose,
    $core.String? operation,
  }) {
    final result = ComposeOperation._();
    if (compose != null) result.compose = compose;
    if (operation != null) result.operation = operation;
    return result;
  }

  ComposeOperation._();

  factory ComposeOperation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeOperation()..mergeFromBuffer(data, registry);
  factory ComposeOperation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeOperation()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeOperation',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeOperation.$_createMessage)
    ..aOM<Compose>(1, _omitFieldNames ? '' : 'compose',
        subBuilder: Compose.$_createMessage)
    ..aOS(2, _omitFieldNames ? '' : 'operation')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeOperation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeOperation copyWith(void Function(ComposeOperation) updates) =>
      super.copyWith((message) => updates(message as ComposeOperation))
          as ComposeOperation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposeOperation() / ComposeOperation.new instead')
  static ComposeOperation create() => ComposeOperation._();
  static $pb.GeneratedMessage $_createMessage() => ComposeOperation._();
  @$core.override
  ComposeOperation createEmptyInstance() => ComposeOperation._();
  @$core.pragma('dart2js:noInline')
  static ComposeOperation getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ComposeOperation>(
          ComposeOperation.$_createMessage);
  static ComposeOperation? _defaultInstance;

  @$pb.TagNumber(1)
  Compose get compose => $_getN(0);
  @$pb.TagNumber(1)
  set compose(Compose value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCompose() => $_has(0);
  @$pb.TagNumber(1)
  void clearCompose() => $_clearField(1);
  @$pb.TagNumber(1)
  Compose ensureCompose() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get operation => $_getSZ(1);
  @$pb.TagNumber(2)
  set operation($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOperation() => $_has(1);
  @$pb.TagNumber(2)
  void clearOperation() => $_clearField(2);
}

class SetComposeSourceRequest extends $pb.GeneratedMessage {
  factory SetComposeSourceRequest({
    $fixnum.Int64? id,
    $core.String? repository,
    $core.String? owner,
    $core.String? branch,
    $core.String? buildPath,
    $fixnum.Int64? providerId,
    $fixnum.Int64? sshKeyId,
  }) {
    final result = SetComposeSourceRequest._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (owner != null) result.owner = owner;
    if (branch != null) result.branch = branch;
    if (buildPath != null) result.buildPath = buildPath;
    if (providerId != null) result.providerId = providerId;
    if (sshKeyId != null) result.sshKeyId = sshKeyId;
    return result;
  }

  SetComposeSourceRequest._();

  factory SetComposeSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetComposeSourceRequest()..mergeFromBuffer(data, registry);
  factory SetComposeSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetComposeSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetComposeSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetComposeSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'branch')
    ..aOS(5, _omitFieldNames ? '' : 'buildPath')
    ..aInt64(6, _omitFieldNames ? '' : 'providerId')
    ..aInt64(7, _omitFieldNames ? '' : 'sshKeyId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetComposeSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetComposeSourceRequest copyWith(
          void Function(SetComposeSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetComposeSourceRequest))
          as SetComposeSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetComposeSourceRequest() / SetComposeSourceRequest.new instead')
  static SetComposeSourceRequest create() => SetComposeSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetComposeSourceRequest._();
  @$core.override
  SetComposeSourceRequest createEmptyInstance() => SetComposeSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetComposeSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetComposeSourceRequest>(
          SetComposeSourceRequest.$_createMessage);
  static SetComposeSourceRequest? _defaultInstance;

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

  /// CUSTOM provider only: SSH key used to clone a private custom-git repo.
  @$pb.TagNumber(7)
  $fixnum.Int64 get sshKeyId => $_getI64(6);
  @$pb.TagNumber(7)
  set sshKeyId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSshKeyId() => $_has(6);
  @$pb.TagNumber(7)
  void clearSshKeyId() => $_clearField(7);
}

class SetRawSourceRequest extends $pb.GeneratedMessage {
  factory SetRawSourceRequest({
    $fixnum.Int64? id,
    $core.String? content,
    $core.String? fileName,
  }) {
    final result = SetRawSourceRequest._();
    if (id != null) result.id = id;
    if (content != null) result.content = content;
    if (fileName != null) result.fileName = fileName;
    return result;
  }

  SetRawSourceRequest._();

  factory SetRawSourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetRawSourceRequest()..mergeFromBuffer(data, registry);
  factory SetRawSourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetRawSourceRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetRawSourceRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetRawSourceRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..aOS(3, _omitFieldNames ? '' : 'fileName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetRawSourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetRawSourceRequest copyWith(void Function(SetRawSourceRequest) updates) =>
      super.copyWith((message) => updates(message as SetRawSourceRequest))
          as SetRawSourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use SetRawSourceRequest() / SetRawSourceRequest.new instead')
  static SetRawSourceRequest create() => SetRawSourceRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetRawSourceRequest._();
  @$core.override
  SetRawSourceRequest createEmptyInstance() => SetRawSourceRequest._();
  @$core.pragma('dart2js:noInline')
  static SetRawSourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetRawSourceRequest>(
          SetRawSourceRequest.$_createMessage);
  static SetRawSourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get fileName => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFileName() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileName() => $_clearField(3);
}

enum UploadComposeRequest_Payload { composeId, chunk, notSet }

class UploadComposeRequest extends $pb.GeneratedMessage {
  factory UploadComposeRequest({
    $fixnum.Int64? composeId,
    $core.List<$core.int>? chunk,
  }) {
    final result = UploadComposeRequest._();
    if (composeId != null) result.composeId = composeId;
    if (chunk != null) result.chunk = chunk;
    return result;
  }

  UploadComposeRequest._();

  factory UploadComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UploadComposeRequest()..mergeFromBuffer(data, registry);
  factory UploadComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UploadComposeRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, UploadComposeRequest_Payload>
      _UploadComposeRequest_PayloadByTag = {
    1: UploadComposeRequest_Payload.composeId,
    2: UploadComposeRequest_Payload.chunk,
    0: UploadComposeRequest_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UploadComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UploadComposeRequest.$_createMessage)
    ..oo(0, [1, 2])
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'chunk', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UploadComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UploadComposeRequest copyWith(void Function(UploadComposeRequest) updates) =>
      super.copyWith((message) => updates(message as UploadComposeRequest))
          as UploadComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UploadComposeRequest() / UploadComposeRequest.new instead')
  static UploadComposeRequest create() => UploadComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => UploadComposeRequest._();
  @$core.override
  UploadComposeRequest createEmptyInstance() => UploadComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static UploadComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UploadComposeRequest>(
          UploadComposeRequest.$_createMessage);
  static UploadComposeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  UploadComposeRequest_Payload whichPayload() =>
      _UploadComposeRequest_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get chunk => $_getN(1);
  @$pb.TagNumber(2)
  set chunk($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChunk() => $_has(1);
  @$pb.TagNumber(2)
  void clearChunk() => $_clearField(2);
}

class MoveComposeRequest extends $pb.GeneratedMessage {
  factory MoveComposeRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? targetEnvironmentId,
  }) {
    final result = MoveComposeRequest._();
    if (id != null) result.id = id;
    if (targetEnvironmentId != null)
      result.targetEnvironmentId = targetEnvironmentId;
    return result;
  }

  MoveComposeRequest._();

  factory MoveComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MoveComposeRequest()..mergeFromBuffer(data, registry);
  factory MoveComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MoveComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MoveComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MoveComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'targetEnvironmentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MoveComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MoveComposeRequest copyWith(void Function(MoveComposeRequest) updates) =>
      super.copyWith((message) => updates(message as MoveComposeRequest))
          as MoveComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use MoveComposeRequest() / MoveComposeRequest.new instead')
  static MoveComposeRequest create() => MoveComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => MoveComposeRequest._();
  @$core.override
  MoveComposeRequest createEmptyInstance() => MoveComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static MoveComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MoveComposeRequest>(
          MoveComposeRequest.$_createMessage);
  static MoveComposeRequest? _defaultInstance;

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

class ComposeWebhookToken extends $pb.GeneratedMessage {
  factory ComposeWebhookToken({
    $core.String? token,
  }) {
    final result = ComposeWebhookToken._();
    if (token != null) result.token = token;
    return result;
  }

  ComposeWebhookToken._();

  factory ComposeWebhookToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeWebhookToken()..mergeFromBuffer(data, registry);
  factory ComposeWebhookToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeWebhookToken()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeWebhookToken',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeWebhookToken.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeWebhookToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeWebhookToken copyWith(void Function(ComposeWebhookToken) updates) =>
      super.copyWith((message) => updates(message as ComposeWebhookToken))
          as ComposeWebhookToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ComposeWebhookToken() / ComposeWebhookToken.new instead')
  static ComposeWebhookToken create() => ComposeWebhookToken._();
  static $pb.GeneratedMessage $_createMessage() => ComposeWebhookToken._();
  @$core.override
  ComposeWebhookToken createEmptyInstance() => ComposeWebhookToken._();
  @$core.pragma('dart2js:noInline')
  static ComposeWebhookToken getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeWebhookToken>(
          ComposeWebhookToken.$_createMessage);
  static ComposeWebhookToken? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);
}

class ComposeDependencies extends $pb.GeneratedMessage {
  factory ComposeDependencies({
    $core.Iterable<$core.String>? items,
  }) {
    final result = ComposeDependencies._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ComposeDependencies._();

  factory ComposeDependencies.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeDependencies()..mergeFromBuffer(data, registry);
  factory ComposeDependencies.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeDependencies()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeDependencies',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeDependencies.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'items')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeDependencies clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeDependencies copyWith(void Function(ComposeDependencies) updates) =>
      super.copyWith((message) => updates(message as ComposeDependencies))
          as ComposeDependencies;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ComposeDependencies() / ComposeDependencies.new instead')
  static ComposeDependencies create() => ComposeDependencies._();
  static $pb.GeneratedMessage $_createMessage() => ComposeDependencies._();
  @$core.override
  ComposeDependencies createEmptyInstance() => ComposeDependencies._();
  @$core.pragma('dart2js:noInline')
  static ComposeDependencies getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeDependencies>(
          ComposeDependencies.$_createMessage);
  static ComposeDependencies? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get items => $_getList(0);
}

class PreviewComposeRequest extends $pb.GeneratedMessage {
  factory PreviewComposeRequest({
    $core.String? content,
  }) {
    final result = PreviewComposeRequest._();
    if (content != null) result.content = content;
    return result;
  }

  PreviewComposeRequest._();

  factory PreviewComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PreviewComposeRequest()..mergeFromBuffer(data, registry);
  factory PreviewComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PreviewComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PreviewComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PreviewComposeRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewComposeRequest copyWith(
          void Function(PreviewComposeRequest) updates) =>
      super.copyWith((message) => updates(message as PreviewComposeRequest))
          as PreviewComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use PreviewComposeRequest() / PreviewComposeRequest.new instead')
  static PreviewComposeRequest create() => PreviewComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => PreviewComposeRequest._();
  @$core.override
  PreviewComposeRequest createEmptyInstance() => PreviewComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static PreviewComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PreviewComposeRequest>(
          PreviewComposeRequest.$_createMessage);
  static PreviewComposeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

class ComposePreview extends $pb.GeneratedMessage {
  factory ComposePreview({
    $core.String? content,
    $core.Iterable<$core.String>? services,
  }) {
    final result = ComposePreview._();
    if (content != null) result.content = content;
    if (services != null) result.services.addAll(services);
    return result;
  }

  ComposePreview._();

  factory ComposePreview.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePreview()..mergeFromBuffer(data, registry);
  factory ComposePreview.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePreview()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposePreview',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposePreview.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..pPS(2, _omitFieldNames ? '' : 'services')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePreview clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePreview copyWith(void Function(ComposePreview) updates) =>
      super.copyWith((message) => updates(message as ComposePreview))
          as ComposePreview;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposePreview() / ComposePreview.new instead')
  static ComposePreview create() => ComposePreview._();
  static $pb.GeneratedMessage $_createMessage() => ComposePreview._();
  @$core.override
  ComposePreview createEmptyInstance() => ComposePreview._();
  @$core.pragma('dart2js:noInline')
  static ComposePreview getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ComposePreview>(
          ComposePreview.$_createMessage);
  static ComposePreview? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get services => $_getList(1);
}

class InstallTemplateRequest extends $pb.GeneratedMessage {
  factory InstallTemplateRequest({
    $core.String? template,
    $fixnum.Int64? environmentId,
    $core.String? name,
    $fixnum.Int64? serverId,
    $core.String? envVar,
  }) {
    final result = InstallTemplateRequest._();
    if (template != null) result.template = template;
    if (environmentId != null) result.environmentId = environmentId;
    if (name != null) result.name = name;
    if (serverId != null) result.serverId = serverId;
    if (envVar != null) result.envVar = envVar;
    return result;
  }

  InstallTemplateRequest._();

  factory InstallTemplateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InstallTemplateRequest()..mergeFromBuffer(data, registry);
  factory InstallTemplateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InstallTemplateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InstallTemplateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: InstallTemplateRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'template')
    ..aInt64(2, _omitFieldNames ? '' : 'environmentId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aInt64(4, _omitFieldNames ? '' : 'serverId')
    ..aOS(5, _omitFieldNames ? '' : 'envVar')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InstallTemplateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InstallTemplateRequest copyWith(
          void Function(InstallTemplateRequest) updates) =>
      super.copyWith((message) => updates(message as InstallTemplateRequest))
          as InstallTemplateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use InstallTemplateRequest() / InstallTemplateRequest.new instead')
  static InstallTemplateRequest create() => InstallTemplateRequest._();
  static $pb.GeneratedMessage $_createMessage() => InstallTemplateRequest._();
  @$core.override
  InstallTemplateRequest createEmptyInstance() => InstallTemplateRequest._();
  @$core.pragma('dart2js:noInline')
  static InstallTemplateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InstallTemplateRequest>(
          InstallTemplateRequest.$_createMessage);
  static InstallTemplateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get template => $_getSZ(0);
  @$pb.TagNumber(1)
  set template($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTemplate() => $_has(0);
  @$pb.TagNumber(1)
  void clearTemplate() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get environmentId => $_getI64(1);
  @$pb.TagNumber(2)
  set environmentId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvironmentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvironmentId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get serverId => $_getI64(3);
  @$pb.TagNumber(4)
  set serverId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasServerId() => $_has(3);
  @$pb.TagNumber(4)
  void clearServerId() => $_clearField(4);

  /// .env-style KEY=VALUE lines seeded on the new compose after install.
  @$pb.TagNumber(5)
  $core.String get envVar => $_getSZ(4);
  @$pb.TagNumber(5)
  set envVar($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEnvVar() => $_has(4);
  @$pb.TagNumber(5)
  void clearEnvVar() => $_clearField(5);
}

class ExportComposeRequest extends $pb.GeneratedMessage {
  factory ExportComposeRequest({
    $fixnum.Int64? id,
    $core.bool? includeSecrets,
  }) {
    final result = ExportComposeRequest._();
    if (id != null) result.id = id;
    if (includeSecrets != null) result.includeSecrets = includeSecrets;
    return result;
  }

  ExportComposeRequest._();

  factory ExportComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportComposeRequest()..mergeFromBuffer(data, registry);
  factory ExportComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExportComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExportComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ExportComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'includeSecrets')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportComposeRequest copyWith(void Function(ExportComposeRequest) updates) =>
      super.copyWith((message) => updates(message as ExportComposeRequest))
          as ExportComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ExportComposeRequest() / ExportComposeRequest.new instead')
  static ExportComposeRequest create() => ExportComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => ExportComposeRequest._();
  @$core.override
  ExportComposeRequest createEmptyInstance() => ExportComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static ExportComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExportComposeRequest>(
          ExportComposeRequest.$_createMessage);
  static ExportComposeRequest? _defaultInstance;

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

class ComposeExport extends $pb.GeneratedMessage {
  factory ComposeExport({
    $core.String? archive,
  }) {
    final result = ComposeExport._();
    if (archive != null) result.archive = archive;
    return result;
  }

  ComposeExport._();

  factory ComposeExport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeExport()..mergeFromBuffer(data, registry);
  factory ComposeExport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeExport()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeExport',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeExport.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeExport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeExport copyWith(void Function(ComposeExport) updates) =>
      super.copyWith((message) => updates(message as ComposeExport))
          as ComposeExport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposeExport() / ComposeExport.new instead')
  static ComposeExport create() => ComposeExport._();
  static $pb.GeneratedMessage $_createMessage() => ComposeExport._();
  @$core.override
  ComposeExport createEmptyInstance() => ComposeExport._();
  @$core.pragma('dart2js:noInline')
  static ComposeExport getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ComposeExport>(
          ComposeExport.$_createMessage);
  static ComposeExport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get archive => $_getSZ(0);
  @$pb.TagNumber(1)
  set archive($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasArchive() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchive() => $_clearField(1);
}

class ImportComposeRequest extends $pb.GeneratedMessage {
  factory ImportComposeRequest({
    $fixnum.Int64? environmentId,
    $core.String? archive,
  }) {
    final result = ImportComposeRequest._();
    if (environmentId != null) result.environmentId = environmentId;
    if (archive != null) result.archive = archive;
    return result;
  }

  ImportComposeRequest._();

  factory ImportComposeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportComposeRequest()..mergeFromBuffer(data, registry);
  factory ImportComposeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportComposeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportComposeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ImportComposeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'environmentId')
    ..aOS(2, _omitFieldNames ? '' : 'archive')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportComposeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportComposeRequest copyWith(void Function(ImportComposeRequest) updates) =>
      super.copyWith((message) => updates(message as ImportComposeRequest))
          as ImportComposeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportComposeRequest() / ImportComposeRequest.new instead')
  static ImportComposeRequest create() => ImportComposeRequest._();
  static $pb.GeneratedMessage $_createMessage() => ImportComposeRequest._();
  @$core.override
  ImportComposeRequest createEmptyInstance() => ImportComposeRequest._();
  @$core.pragma('dart2js:noInline')
  static ImportComposeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportComposeRequest>(
          ImportComposeRequest.$_createMessage);
  static ImportComposeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get environmentId => $_getI64(0);
  @$pb.TagNumber(1)
  set environmentId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEnvironmentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnvironmentId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get archive => $_getSZ(1);
  @$pb.TagNumber(2)
  set archive($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasArchive() => $_has(1);
  @$pb.TagNumber(2)
  void clearArchive() => $_clearField(2);
}

class ServiceMutationRequest extends $pb.GeneratedMessage {
  factory ServiceMutationRequest({
    $fixnum.Int64? id,
    $core.Iterable<$core.String>? services,
  }) {
    final result = ServiceMutationRequest._();
    if (id != null) result.id = id;
    if (services != null) result.services.addAll(services);
    return result;
  }

  ServiceMutationRequest._();

  factory ServiceMutationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServiceMutationRequest()..mergeFromBuffer(data, registry);
  factory ServiceMutationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServiceMutationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServiceMutationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ServiceMutationRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..pPS(2, _omitFieldNames ? '' : 'services')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServiceMutationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServiceMutationRequest copyWith(
          void Function(ServiceMutationRequest) updates) =>
      super.copyWith((message) => updates(message as ServiceMutationRequest))
          as ServiceMutationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ServiceMutationRequest() / ServiceMutationRequest.new instead')
  static ServiceMutationRequest create() => ServiceMutationRequest._();
  static $pb.GeneratedMessage $_createMessage() => ServiceMutationRequest._();
  @$core.override
  ServiceMutationRequest createEmptyInstance() => ServiceMutationRequest._();
  @$core.pragma('dart2js:noInline')
  static ServiceMutationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServiceMutationRequest>(
          ServiceMutationRequest.$_createMessage);
  static ServiceMutationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get services => $_getList(1);
}

class UpdateComposeResourcesRequest extends $pb.GeneratedMessage {
  factory UpdateComposeResourcesRequest({
    $fixnum.Int64? id,
    $core.String? memoryLimit,
    $core.String? cpuLimit,
    $fixnum.Int64? replicas,
  }) {
    final result = UpdateComposeResourcesRequest._();
    if (id != null) result.id = id;
    if (memoryLimit != null) result.memoryLimit = memoryLimit;
    if (cpuLimit != null) result.cpuLimit = cpuLimit;
    if (replicas != null) result.replicas = replicas;
    return result;
  }

  UpdateComposeResourcesRequest._();

  factory UpdateComposeResourcesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateComposeResourcesRequest()..mergeFromBuffer(data, registry);
  factory UpdateComposeResourcesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateComposeResourcesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateComposeResourcesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateComposeResourcesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'memoryLimit')
    ..aOS(3, _omitFieldNames ? '' : 'cpuLimit')
    ..aInt64(4, _omitFieldNames ? '' : 'replicas')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateComposeResourcesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateComposeResourcesRequest copyWith(
          void Function(UpdateComposeResourcesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as UpdateComposeResourcesRequest))
          as UpdateComposeResourcesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateComposeResourcesRequest() / UpdateComposeResourcesRequest.new instead')
  static UpdateComposeResourcesRequest create() =>
      UpdateComposeResourcesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateComposeResourcesRequest._();
  @$core.override
  UpdateComposeResourcesRequest createEmptyInstance() =>
      UpdateComposeResourcesRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateComposeResourcesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateComposeResourcesRequest>(
          UpdateComposeResourcesRequest.$_createMessage);
  static UpdateComposeResourcesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get memoryLimit => $_getSZ(1);
  @$pb.TagNumber(2)
  set memoryLimit($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMemoryLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearMemoryLimit() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get cpuLimit => $_getSZ(2);
  @$pb.TagNumber(3)
  set cpuLimit($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCpuLimit() => $_has(2);
  @$pb.TagNumber(3)
  void clearCpuLimit() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get replicas => $_getI64(3);
  @$pb.TagNumber(4)
  set replicas($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasReplicas() => $_has(3);
  @$pb.TagNumber(4)
  void clearReplicas() => $_clearField(4);
}

class ComposeMountRequest extends $pb.GeneratedMessage {
  factory ComposeMountRequest({
    $fixnum.Int64? composeId,
    $fixnum.Int64? id,
    $core.String? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = ComposeMountRequest._();
    if (composeId != null) result.composeId = composeId;
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  ComposeMountRequest._();

  factory ComposeMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeMountRequest()..mergeFromBuffer(data, registry);
  factory ComposeMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'mountType')
    ..aOS(4, _omitFieldNames ? '' : 'hostPath')
    ..aOS(5, _omitFieldNames ? '' : 'volumeName')
    ..aOS(6, _omitFieldNames ? '' : 'filePath')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..aOS(8, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeMountRequest copyWith(void Function(ComposeMountRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeMountRequest))
          as ComposeMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ComposeMountRequest() / ComposeMountRequest.new instead')
  static ComposeMountRequest create() => ComposeMountRequest._();
  static $pb.GeneratedMessage $_createMessage() => ComposeMountRequest._();
  @$core.override
  ComposeMountRequest createEmptyInstance() => ComposeMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeMountRequest>(
          ComposeMountRequest.$_createMessage);
  static ComposeMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get mountType => $_getSZ(2);
  @$pb.TagNumber(3)
  set mountType($core.String value) => $_setString(2, value);
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

class ComposePatchRequest extends $pb.GeneratedMessage {
  factory ComposePatchRequest({
    $fixnum.Int64? composeId,
    $fixnum.Int64? id,
    $core.String? patchType,
    $core.String? payload,
  }) {
    final result = ComposePatchRequest._();
    if (composeId != null) result.composeId = composeId;
    if (id != null) result.id = id;
    if (patchType != null) result.patchType = patchType;
    if (payload != null) result.payload = payload;
    return result;
  }

  ComposePatchRequest._();

  factory ComposePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePatchRequest()..mergeFromBuffer(data, registry);
  factory ComposePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'patchType')
    ..aOS(4, _omitFieldNames ? '' : 'payload')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePatchRequest copyWith(void Function(ComposePatchRequest) updates) =>
      super.copyWith((message) => updates(message as ComposePatchRequest))
          as ComposePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ComposePatchRequest() / ComposePatchRequest.new instead')
  static ComposePatchRequest create() => ComposePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() => ComposePatchRequest._();
  @$core.override
  ComposePatchRequest createEmptyInstance() => ComposePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposePatchRequest>(
          ComposePatchRequest.$_createMessage);
  static ComposePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get patchType => $_getSZ(2);
  @$pb.TagNumber(3)
  set patchType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPatchType() => $_has(2);
  @$pb.TagNumber(3)
  void clearPatchType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get payload => $_getSZ(3);
  @$pb.TagNumber(4)
  set payload($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPayload() => $_has(3);
  @$pb.TagNumber(4)
  void clearPayload() => $_clearField(4);
}

class ComposeMount extends $pb.GeneratedMessage {
  factory ComposeMount({
    $fixnum.Int64? id,
    ComposeMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
    $fixnum.Int64? composeId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ComposeMount._();
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    if (composeId != null) result.composeId = composeId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ComposeMount._();

  factory ComposeMount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeMount()..mergeFromBuffer(data, registry);
  factory ComposeMount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeMount()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeMount',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeMount.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<ComposeMountType>(2, _omitFieldNames ? '' : 'mountType',
        enumValues: ComposeMountType.values)
    ..aOS(3, _omitFieldNames ? '' : 'hostPath')
    ..aOS(4, _omitFieldNames ? '' : 'volumeName')
    ..aOS(5, _omitFieldNames ? '' : 'filePath')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..aOS(7, _omitFieldNames ? '' : 'mountPath')
    ..aInt64(8, _omitFieldNames ? '' : 'composeId')
    ..aInt64(9, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(10, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeMount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeMount copyWith(void Function(ComposeMount) updates) =>
      super.copyWith((message) => updates(message as ComposeMount))
          as ComposeMount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposeMount() / ComposeMount.new instead')
  static ComposeMount create() => ComposeMount._();
  static $pb.GeneratedMessage $_createMessage() => ComposeMount._();
  @$core.override
  ComposeMount createEmptyInstance() => ComposeMount._();
  @$core.pragma('dart2js:noInline')
  static ComposeMount getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ComposeMount>(
          ComposeMount.$_createMessage);
  static ComposeMount? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ComposeMountType get mountType => $_getN(1);
  @$pb.TagNumber(2)
  set mountType(ComposeMountType value) => $_setField(2, value);
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

class ComposePatch extends $pb.GeneratedMessage {
  factory ComposePatch({
    $fixnum.Int64? id,
    ComposePatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
    $fixnum.Int64? composeId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = ComposePatch._();
    if (id != null) result.id = id;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    if (composeId != null) result.composeId = composeId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ComposePatch._();

  factory ComposePatch.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePatch()..mergeFromBuffer(data, registry);
  factory ComposePatch.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposePatch()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposePatch',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposePatch.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aE<ComposePatchType>(2, _omitFieldNames ? '' : 'patchType',
        enumValues: ComposePatchType.values)
    ..aOS(3, _omitFieldNames ? '' : 'filePath')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..aInt64(6, _omitFieldNames ? '' : 'composeId')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(8, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePatch clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposePatch copyWith(void Function(ComposePatch) updates) =>
      super.copyWith((message) => updates(message as ComposePatch))
          as ComposePatch;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ComposePatch() / ComposePatch.new instead')
  static ComposePatch create() => ComposePatch._();
  static $pb.GeneratedMessage $_createMessage() => ComposePatch._();
  @$core.override
  ComposePatch createEmptyInstance() => ComposePatch._();
  @$core.pragma('dart2js:noInline')
  static ComposePatch getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ComposePatch>(
          ComposePatch.$_createMessage);
  static ComposePatch? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ComposePatchType get patchType => $_getN(1);
  @$pb.TagNumber(2)
  set patchType(ComposePatchType value) => $_setField(2, value);
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

  /// Patch file body (raw text).
  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get composeId => $_getI64(5);
  @$pb.TagNumber(6)
  set composeId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasComposeId() => $_has(5);
  @$pb.TagNumber(6)
  void clearComposeId() => $_clearField(6);

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

class ComposeListMountsRequest extends $pb.GeneratedMessage {
  factory ComposeListMountsRequest({
    $fixnum.Int64? composeId,
  }) {
    final result = ComposeListMountsRequest._();
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  ComposeListMountsRequest._();

  factory ComposeListMountsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListMountsRequest()..mergeFromBuffer(data, registry);
  factory ComposeListMountsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListMountsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeListMountsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeListMountsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListMountsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListMountsRequest copyWith(
          void Function(ComposeListMountsRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeListMountsRequest))
          as ComposeListMountsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeListMountsRequest() / ComposeListMountsRequest.new instead')
  static ComposeListMountsRequest create() => ComposeListMountsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ComposeListMountsRequest._();
  @$core.override
  ComposeListMountsRequest createEmptyInstance() =>
      ComposeListMountsRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeListMountsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeListMountsRequest>(
          ComposeListMountsRequest.$_createMessage);
  static ComposeListMountsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);
}

class ComposeListMountsResponse extends $pb.GeneratedMessage {
  factory ComposeListMountsResponse({
    $core.Iterable<ComposeMount>? mounts,
  }) {
    final result = ComposeListMountsResponse._();
    if (mounts != null) result.mounts.addAll(mounts);
    return result;
  }

  ComposeListMountsResponse._();

  factory ComposeListMountsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListMountsResponse()..mergeFromBuffer(data, registry);
  factory ComposeListMountsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListMountsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeListMountsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeListMountsResponse.$_createMessage)
    ..pPM<ComposeMount>(1, _omitFieldNames ? '' : 'mounts',
        subBuilder: ComposeMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListMountsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListMountsResponse copyWith(
          void Function(ComposeListMountsResponse) updates) =>
      super.copyWith((message) => updates(message as ComposeListMountsResponse))
          as ComposeListMountsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeListMountsResponse() / ComposeListMountsResponse.new instead')
  static ComposeListMountsResponse create() => ComposeListMountsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeListMountsResponse._();
  @$core.override
  ComposeListMountsResponse createEmptyInstance() =>
      ComposeListMountsResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeListMountsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeListMountsResponse>(
          ComposeListMountsResponse.$_createMessage);
  static ComposeListMountsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ComposeMount> get mounts => $_getList(0);
}

class ComposeCreateMountRequest extends $pb.GeneratedMessage {
  factory ComposeCreateMountRequest({
    $fixnum.Int64? composeId,
    ComposeMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = ComposeCreateMountRequest._();
    if (composeId != null) result.composeId = composeId;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  ComposeCreateMountRequest._();

  factory ComposeCreateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreateMountRequest()..mergeFromBuffer(data, registry);
  factory ComposeCreateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeCreateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeCreateMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aE<ComposeMountType>(2, _omitFieldNames ? '' : 'mountType',
        enumValues: ComposeMountType.values)
    ..aOS(3, _omitFieldNames ? '' : 'hostPath')
    ..aOS(4, _omitFieldNames ? '' : 'volumeName')
    ..aOS(5, _omitFieldNames ? '' : 'filePath')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..aOS(7, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreateMountRequest copyWith(
          void Function(ComposeCreateMountRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeCreateMountRequest))
          as ComposeCreateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeCreateMountRequest() / ComposeCreateMountRequest.new instead')
  static ComposeCreateMountRequest create() => ComposeCreateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeCreateMountRequest._();
  @$core.override
  ComposeCreateMountRequest createEmptyInstance() =>
      ComposeCreateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeCreateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeCreateMountRequest>(
          ComposeCreateMountRequest.$_createMessage);
  static ComposeCreateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  ComposeMountType get mountType => $_getN(1);
  @$pb.TagNumber(2)
  set mountType(ComposeMountType value) => $_setField(2, value);
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

class ComposeCreateMountResponse extends $pb.GeneratedMessage {
  factory ComposeCreateMountResponse({
    ComposeMount? mount,
  }) {
    final result = ComposeCreateMountResponse._();
    if (mount != null) result.mount = mount;
    return result;
  }

  ComposeCreateMountResponse._();

  factory ComposeCreateMountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreateMountResponse()..mergeFromBuffer(data, registry);
  factory ComposeCreateMountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreateMountResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeCreateMountResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeCreateMountResponse.$_createMessage)
    ..aOM<ComposeMount>(1, _omitFieldNames ? '' : 'mount',
        subBuilder: ComposeMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreateMountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreateMountResponse copyWith(
          void Function(ComposeCreateMountResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ComposeCreateMountResponse))
          as ComposeCreateMountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeCreateMountResponse() / ComposeCreateMountResponse.new instead')
  static ComposeCreateMountResponse create() => ComposeCreateMountResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeCreateMountResponse._();
  @$core.override
  ComposeCreateMountResponse createEmptyInstance() =>
      ComposeCreateMountResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeCreateMountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeCreateMountResponse>(
          ComposeCreateMountResponse.$_createMessage);
  static ComposeCreateMountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ComposeMount get mount => $_getN(0);
  @$pb.TagNumber(1)
  set mount(ComposeMount value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMount() => $_has(0);
  @$pb.TagNumber(1)
  void clearMount() => $_clearField(1);
  @$pb.TagNumber(1)
  ComposeMount ensureMount() => $_ensure(0);
}

class ComposeUpdateMountRequest extends $pb.GeneratedMessage {
  factory ComposeUpdateMountRequest({
    $fixnum.Int64? composeId,
    $fixnum.Int64? id,
    ComposeMountType? mountType,
    $core.String? hostPath,
    $core.String? volumeName,
    $core.String? filePath,
    $core.String? content,
    $core.String? mountPath,
  }) {
    final result = ComposeUpdateMountRequest._();
    if (composeId != null) result.composeId = composeId;
    if (id != null) result.id = id;
    if (mountType != null) result.mountType = mountType;
    if (hostPath != null) result.hostPath = hostPath;
    if (volumeName != null) result.volumeName = volumeName;
    if (filePath != null) result.filePath = filePath;
    if (content != null) result.content = content;
    if (mountPath != null) result.mountPath = mountPath;
    return result;
  }

  ComposeUpdateMountRequest._();

  factory ComposeUpdateMountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdateMountRequest()..mergeFromBuffer(data, registry);
  factory ComposeUpdateMountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdateMountRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeUpdateMountRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeUpdateMountRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aE<ComposeMountType>(3, _omitFieldNames ? '' : 'mountType',
        enumValues: ComposeMountType.values)
    ..aOS(4, _omitFieldNames ? '' : 'hostPath')
    ..aOS(5, _omitFieldNames ? '' : 'volumeName')
    ..aOS(6, _omitFieldNames ? '' : 'filePath')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..aOS(8, _omitFieldNames ? '' : 'mountPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdateMountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdateMountRequest copyWith(
          void Function(ComposeUpdateMountRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeUpdateMountRequest))
          as ComposeUpdateMountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeUpdateMountRequest() / ComposeUpdateMountRequest.new instead')
  static ComposeUpdateMountRequest create() => ComposeUpdateMountRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeUpdateMountRequest._();
  @$core.override
  ComposeUpdateMountRequest createEmptyInstance() =>
      ComposeUpdateMountRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeUpdateMountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeUpdateMountRequest>(
          ComposeUpdateMountRequest.$_createMessage);
  static ComposeUpdateMountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  ComposeMountType get mountType => $_getN(2);
  @$pb.TagNumber(3)
  set mountType(ComposeMountType value) => $_setField(3, value);
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

class ComposeUpdateMountResponse extends $pb.GeneratedMessage {
  factory ComposeUpdateMountResponse({
    ComposeMount? mount,
  }) {
    final result = ComposeUpdateMountResponse._();
    if (mount != null) result.mount = mount;
    return result;
  }

  ComposeUpdateMountResponse._();

  factory ComposeUpdateMountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdateMountResponse()..mergeFromBuffer(data, registry);
  factory ComposeUpdateMountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdateMountResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeUpdateMountResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeUpdateMountResponse.$_createMessage)
    ..aOM<ComposeMount>(1, _omitFieldNames ? '' : 'mount',
        subBuilder: ComposeMount.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdateMountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdateMountResponse copyWith(
          void Function(ComposeUpdateMountResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ComposeUpdateMountResponse))
          as ComposeUpdateMountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeUpdateMountResponse() / ComposeUpdateMountResponse.new instead')
  static ComposeUpdateMountResponse create() => ComposeUpdateMountResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeUpdateMountResponse._();
  @$core.override
  ComposeUpdateMountResponse createEmptyInstance() =>
      ComposeUpdateMountResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeUpdateMountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeUpdateMountResponse>(
          ComposeUpdateMountResponse.$_createMessage);
  static ComposeUpdateMountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ComposeMount get mount => $_getN(0);
  @$pb.TagNumber(1)
  set mount(ComposeMount value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMount() => $_has(0);
  @$pb.TagNumber(1)
  void clearMount() => $_clearField(1);
  @$pb.TagNumber(1)
  ComposeMount ensureMount() => $_ensure(0);
}

class ComposeListPatchesRequest extends $pb.GeneratedMessage {
  factory ComposeListPatchesRequest({
    $fixnum.Int64? composeId,
  }) {
    final result = ComposeListPatchesRequest._();
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  ComposeListPatchesRequest._();

  factory ComposeListPatchesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListPatchesRequest()..mergeFromBuffer(data, registry);
  factory ComposeListPatchesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListPatchesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeListPatchesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeListPatchesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListPatchesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListPatchesRequest copyWith(
          void Function(ComposeListPatchesRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeListPatchesRequest))
          as ComposeListPatchesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeListPatchesRequest() / ComposeListPatchesRequest.new instead')
  static ComposeListPatchesRequest create() => ComposeListPatchesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeListPatchesRequest._();
  @$core.override
  ComposeListPatchesRequest createEmptyInstance() =>
      ComposeListPatchesRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeListPatchesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeListPatchesRequest>(
          ComposeListPatchesRequest.$_createMessage);
  static ComposeListPatchesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);
}

class ComposeListPatchesResponse extends $pb.GeneratedMessage {
  factory ComposeListPatchesResponse({
    $core.Iterable<ComposePatch>? patches,
  }) {
    final result = ComposeListPatchesResponse._();
    if (patches != null) result.patches.addAll(patches);
    return result;
  }

  ComposeListPatchesResponse._();

  factory ComposeListPatchesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListPatchesResponse()..mergeFromBuffer(data, registry);
  factory ComposeListPatchesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeListPatchesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeListPatchesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeListPatchesResponse.$_createMessage)
    ..pPM<ComposePatch>(1, _omitFieldNames ? '' : 'patches',
        subBuilder: ComposePatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListPatchesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeListPatchesResponse copyWith(
          void Function(ComposeListPatchesResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ComposeListPatchesResponse))
          as ComposeListPatchesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeListPatchesResponse() / ComposeListPatchesResponse.new instead')
  static ComposeListPatchesResponse create() => ComposeListPatchesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeListPatchesResponse._();
  @$core.override
  ComposeListPatchesResponse createEmptyInstance() =>
      ComposeListPatchesResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeListPatchesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeListPatchesResponse>(
          ComposeListPatchesResponse.$_createMessage);
  static ComposeListPatchesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ComposePatch> get patches => $_getList(0);
}

class ComposeCreatePatchRequest extends $pb.GeneratedMessage {
  factory ComposeCreatePatchRequest({
    $fixnum.Int64? composeId,
    ComposePatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
  }) {
    final result = ComposeCreatePatchRequest._();
    if (composeId != null) result.composeId = composeId;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    return result;
  }

  ComposeCreatePatchRequest._();

  factory ComposeCreatePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreatePatchRequest()..mergeFromBuffer(data, registry);
  factory ComposeCreatePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreatePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeCreatePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeCreatePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aE<ComposePatchType>(2, _omitFieldNames ? '' : 'patchType',
        enumValues: ComposePatchType.values)
    ..aOS(3, _omitFieldNames ? '' : 'filePath')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOS(5, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreatePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreatePatchRequest copyWith(
          void Function(ComposeCreatePatchRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeCreatePatchRequest))
          as ComposeCreatePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeCreatePatchRequest() / ComposeCreatePatchRequest.new instead')
  static ComposeCreatePatchRequest create() => ComposeCreatePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeCreatePatchRequest._();
  @$core.override
  ComposeCreatePatchRequest createEmptyInstance() =>
      ComposeCreatePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeCreatePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeCreatePatchRequest>(
          ComposeCreatePatchRequest.$_createMessage);
  static ComposeCreatePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  ComposePatchType get patchType => $_getN(1);
  @$pb.TagNumber(2)
  set patchType(ComposePatchType value) => $_setField(2, value);
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

class ComposeCreatePatchResponse extends $pb.GeneratedMessage {
  factory ComposeCreatePatchResponse({
    ComposePatch? patch,
  }) {
    final result = ComposeCreatePatchResponse._();
    if (patch != null) result.patch = patch;
    return result;
  }

  ComposeCreatePatchResponse._();

  factory ComposeCreatePatchResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreatePatchResponse()..mergeFromBuffer(data, registry);
  factory ComposeCreatePatchResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeCreatePatchResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeCreatePatchResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeCreatePatchResponse.$_createMessage)
    ..aOM<ComposePatch>(1, _omitFieldNames ? '' : 'patch',
        subBuilder: ComposePatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreatePatchResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeCreatePatchResponse copyWith(
          void Function(ComposeCreatePatchResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ComposeCreatePatchResponse))
          as ComposeCreatePatchResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeCreatePatchResponse() / ComposeCreatePatchResponse.new instead')
  static ComposeCreatePatchResponse create() => ComposeCreatePatchResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeCreatePatchResponse._();
  @$core.override
  ComposeCreatePatchResponse createEmptyInstance() =>
      ComposeCreatePatchResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeCreatePatchResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeCreatePatchResponse>(
          ComposeCreatePatchResponse.$_createMessage);
  static ComposeCreatePatchResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ComposePatch get patch => $_getN(0);
  @$pb.TagNumber(1)
  set patch(ComposePatch value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPatch() => $_has(0);
  @$pb.TagNumber(1)
  void clearPatch() => $_clearField(1);
  @$pb.TagNumber(1)
  ComposePatch ensurePatch() => $_ensure(0);
}

class ComposeUpdatePatchRequest extends $pb.GeneratedMessage {
  factory ComposeUpdatePatchRequest({
    $fixnum.Int64? composeId,
    $fixnum.Int64? id,
    ComposePatchType? patchType,
    $core.String? filePath,
    $core.bool? enabled,
    $core.String? content,
  }) {
    final result = ComposeUpdatePatchRequest._();
    if (composeId != null) result.composeId = composeId;
    if (id != null) result.id = id;
    if (patchType != null) result.patchType = patchType;
    if (filePath != null) result.filePath = filePath;
    if (enabled != null) result.enabled = enabled;
    if (content != null) result.content = content;
    return result;
  }

  ComposeUpdatePatchRequest._();

  factory ComposeUpdatePatchRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdatePatchRequest()..mergeFromBuffer(data, registry);
  factory ComposeUpdatePatchRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdatePatchRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeUpdatePatchRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeUpdatePatchRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'composeId')
    ..aInt64(2, _omitFieldNames ? '' : 'id')
    ..aE<ComposePatchType>(3, _omitFieldNames ? '' : 'patchType',
        enumValues: ComposePatchType.values)
    ..aOS(4, _omitFieldNames ? '' : 'filePath')
    ..aOB(5, _omitFieldNames ? '' : 'enabled')
    ..aOS(6, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdatePatchRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdatePatchRequest copyWith(
          void Function(ComposeUpdatePatchRequest) updates) =>
      super.copyWith((message) => updates(message as ComposeUpdatePatchRequest))
          as ComposeUpdatePatchRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeUpdatePatchRequest() / ComposeUpdatePatchRequest.new instead')
  static ComposeUpdatePatchRequest create() => ComposeUpdatePatchRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeUpdatePatchRequest._();
  @$core.override
  ComposeUpdatePatchRequest createEmptyInstance() =>
      ComposeUpdatePatchRequest._();
  @$core.pragma('dart2js:noInline')
  static ComposeUpdatePatchRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeUpdatePatchRequest>(
          ComposeUpdatePatchRequest.$_createMessage);
  static ComposeUpdatePatchRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get composeId => $_getI64(0);
  @$pb.TagNumber(1)
  set composeId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasComposeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearComposeId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  ComposePatchType get patchType => $_getN(2);
  @$pb.TagNumber(3)
  set patchType(ComposePatchType value) => $_setField(3, value);
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

class ComposeUpdatePatchResponse extends $pb.GeneratedMessage {
  factory ComposeUpdatePatchResponse({
    ComposePatch? patch,
  }) {
    final result = ComposeUpdatePatchResponse._();
    if (patch != null) result.patch = patch;
    return result;
  }

  ComposeUpdatePatchResponse._();

  factory ComposeUpdatePatchResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdatePatchResponse()..mergeFromBuffer(data, registry);
  factory ComposeUpdatePatchResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ComposeUpdatePatchResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ComposeUpdatePatchResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ComposeUpdatePatchResponse.$_createMessage)
    ..aOM<ComposePatch>(1, _omitFieldNames ? '' : 'patch',
        subBuilder: ComposePatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdatePatchResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ComposeUpdatePatchResponse copyWith(
          void Function(ComposeUpdatePatchResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ComposeUpdatePatchResponse))
          as ComposeUpdatePatchResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ComposeUpdatePatchResponse() / ComposeUpdatePatchResponse.new instead')
  static ComposeUpdatePatchResponse create() => ComposeUpdatePatchResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ComposeUpdatePatchResponse._();
  @$core.override
  ComposeUpdatePatchResponse createEmptyInstance() =>
      ComposeUpdatePatchResponse._();
  @$core.pragma('dart2js:noInline')
  static ComposeUpdatePatchResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ComposeUpdatePatchResponse>(
          ComposeUpdatePatchResponse.$_createMessage);
  static ComposeUpdatePatchResponse? _defaultInstance;

  @$pb.TagNumber(1)
  ComposePatch get patch => $_getN(0);
  @$pb.TagNumber(1)
  set patch(ComposePatch value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPatch() => $_has(0);
  @$pb.TagNumber(1)
  void clearPatch() => $_clearField(1);
  @$pb.TagNumber(1)
  ComposePatch ensurePatch() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

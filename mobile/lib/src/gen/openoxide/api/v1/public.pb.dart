// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/public.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ListGitBranchesRequest extends $pb.GeneratedMessage {
  factory ListGitBranchesRequest({
    $core.String? repositoryUrl,
  }) {
    final result = ListGitBranchesRequest._();
    if (repositoryUrl != null) result.repositoryUrl = repositoryUrl;
    return result;
  }

  ListGitBranchesRequest._();

  factory ListGitBranchesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitBranchesRequest()..mergeFromBuffer(data, registry);
  factory ListGitBranchesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitBranchesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListGitBranchesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListGitBranchesRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'repositoryUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitBranchesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitBranchesRequest copyWith(
          void Function(ListGitBranchesRequest) updates) =>
      super.copyWith((message) => updates(message as ListGitBranchesRequest))
          as ListGitBranchesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListGitBranchesRequest() / ListGitBranchesRequest.new instead')
  static ListGitBranchesRequest create() => ListGitBranchesRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListGitBranchesRequest._();
  @$core.override
  ListGitBranchesRequest createEmptyInstance() => ListGitBranchesRequest._();
  @$core.pragma('dart2js:noInline')
  static ListGitBranchesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListGitBranchesRequest>(
          ListGitBranchesRequest.$_createMessage);
  static ListGitBranchesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get repositoryUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set repositoryUrl($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRepositoryUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearRepositoryUrl() => $_clearField(1);
}

class ListGitBranchesResponse extends $pb.GeneratedMessage {
  factory ListGitBranchesResponse({
    $core.Iterable<$core.String>? branches,
  }) {
    final result = ListGitBranchesResponse._();
    if (branches != null) result.branches.addAll(branches);
    return result;
  }

  ListGitBranchesResponse._();

  factory ListGitBranchesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitBranchesResponse()..mergeFromBuffer(data, registry);
  factory ListGitBranchesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListGitBranchesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListGitBranchesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListGitBranchesResponse.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'branches')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitBranchesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListGitBranchesResponse copyWith(
          void Function(ListGitBranchesResponse) updates) =>
      super.copyWith((message) => updates(message as ListGitBranchesResponse))
          as ListGitBranchesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListGitBranchesResponse() / ListGitBranchesResponse.new instead')
  static ListGitBranchesResponse create() => ListGitBranchesResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListGitBranchesResponse._();
  @$core.override
  ListGitBranchesResponse createEmptyInstance() => ListGitBranchesResponse._();
  @$core.pragma('dart2js:noInline')
  static ListGitBranchesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListGitBranchesResponse>(
          ListGitBranchesResponse.$_createMessage);
  static ListGitBranchesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get branches => $_getList(0);
}

class GenerateDomainRequest extends $pb.GeneratedMessage {
  factory GenerateDomainRequest({
    $core.String? name,
  }) {
    final result = GenerateDomainRequest._();
    if (name != null) result.name = name;
    return result;
  }

  GenerateDomainRequest._();

  factory GenerateDomainRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GenerateDomainRequest()..mergeFromBuffer(data, registry);
  factory GenerateDomainRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GenerateDomainRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GenerateDomainRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GenerateDomainRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateDomainRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateDomainRequest copyWith(
          void Function(GenerateDomainRequest) updates) =>
      super.copyWith((message) => updates(message as GenerateDomainRequest))
          as GenerateDomainRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GenerateDomainRequest() / GenerateDomainRequest.new instead')
  static GenerateDomainRequest create() => GenerateDomainRequest._();
  static $pb.GeneratedMessage $_createMessage() => GenerateDomainRequest._();
  @$core.override
  GenerateDomainRequest createEmptyInstance() => GenerateDomainRequest._();
  @$core.pragma('dart2js:noInline')
  static GenerateDomainRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GenerateDomainRequest>(
          GenerateDomainRequest.$_createMessage);
  static GenerateDomainRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);
}

class GenerateDomainResponse extends $pb.GeneratedMessage {
  factory GenerateDomainResponse({
    $core.String? domain,
  }) {
    final result = GenerateDomainResponse._();
    if (domain != null) result.domain = domain;
    return result;
  }

  GenerateDomainResponse._();

  factory GenerateDomainResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GenerateDomainResponse()..mergeFromBuffer(data, registry);
  factory GenerateDomainResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GenerateDomainResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GenerateDomainResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GenerateDomainResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'domain')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateDomainResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateDomainResponse copyWith(
          void Function(GenerateDomainResponse) updates) =>
      super.copyWith((message) => updates(message as GenerateDomainResponse))
          as GenerateDomainResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GenerateDomainResponse() / GenerateDomainResponse.new instead')
  static GenerateDomainResponse create() => GenerateDomainResponse._();
  static $pb.GeneratedMessage $_createMessage() => GenerateDomainResponse._();
  @$core.override
  GenerateDomainResponse createEmptyInstance() => GenerateDomainResponse._();
  @$core.pragma('dart2js:noInline')
  static GenerateDomainResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GenerateDomainResponse>(
          GenerateDomainResponse.$_createMessage);
  static GenerateDomainResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get domain => $_getSZ(0);
  @$pb.TagNumber(1)
  set domain($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDomain() => $_has(0);
  @$pb.TagNumber(1)
  void clearDomain() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

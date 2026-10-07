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

import 'package:protobuf/protobuf.dart' as $pb;

/// Provider kinds, mirroring the `GitProviderType` string enum stored on the provider row (`GITHUB`, `GITLAB`, `GITEA`, `BITBUCKET`).
class GitProviderType extends $pb.ProtobufEnum {
  static const GitProviderType GIT_PROVIDER_TYPE_UNSPECIFIED =
      GitProviderType._(
          0, _omitEnumNames ? '' : 'GIT_PROVIDER_TYPE_UNSPECIFIED');
  static const GitProviderType GIT_PROVIDER_TYPE_GITHUB =
      GitProviderType._(1, _omitEnumNames ? '' : 'GIT_PROVIDER_TYPE_GITHUB');
  static const GitProviderType GIT_PROVIDER_TYPE_GITLAB =
      GitProviderType._(2, _omitEnumNames ? '' : 'GIT_PROVIDER_TYPE_GITLAB');
  static const GitProviderType GIT_PROVIDER_TYPE_GITEA =
      GitProviderType._(3, _omitEnumNames ? '' : 'GIT_PROVIDER_TYPE_GITEA');
  static const GitProviderType GIT_PROVIDER_TYPE_BITBUCKET =
      GitProviderType._(4, _omitEnumNames ? '' : 'GIT_PROVIDER_TYPE_BITBUCKET');

  static const $core.List<GitProviderType> values = <GitProviderType>[
    GIT_PROVIDER_TYPE_UNSPECIFIED,
    GIT_PROVIDER_TYPE_GITHUB,
    GIT_PROVIDER_TYPE_GITLAB,
    GIT_PROVIDER_TYPE_GITEA,
    GIT_PROVIDER_TYPE_BITBUCKET,
  ];

  static final $core.List<GitProviderType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static GitProviderType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const GitProviderType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

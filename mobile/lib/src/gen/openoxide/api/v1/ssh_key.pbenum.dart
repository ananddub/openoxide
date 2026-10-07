// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ssh_key.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Key algorithm selector for generation; the service only accepts these two.
class SshKeyType extends $pb.ProtobufEnum {
  static const SshKeyType SSH_KEY_TYPE_UNSPECIFIED =
      SshKeyType._(0, _omitEnumNames ? '' : 'SSH_KEY_TYPE_UNSPECIFIED');
  static const SshKeyType SSH_KEY_TYPE_ED25519 =
      SshKeyType._(1, _omitEnumNames ? '' : 'SSH_KEY_TYPE_ED25519');
  static const SshKeyType SSH_KEY_TYPE_RSA =
      SshKeyType._(2, _omitEnumNames ? '' : 'SSH_KEY_TYPE_RSA');

  static const $core.List<SshKeyType> values = <SshKeyType>[
    SSH_KEY_TYPE_UNSPECIFIED,
    SSH_KEY_TYPE_ED25519,
    SSH_KEY_TYPE_RSA,
  ];

  static final $core.List<SshKeyType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static SshKeyType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SshKeyType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

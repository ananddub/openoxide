// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/vault.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class VaultProviderType extends $pb.ProtobufEnum {
  static const VaultProviderType VAULT_PROVIDER_TYPE_UNSPECIFIED =
      VaultProviderType._(
          0, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_UNSPECIFIED');
  static const VaultProviderType VAULT_PROVIDER_TYPE_HASHICORP =
      VaultProviderType._(
          1, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_HASHICORP');
  static const VaultProviderType VAULT_PROVIDER_TYPE_INFISICAL =
      VaultProviderType._(
          2, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_INFISICAL');
  static const VaultProviderType VAULT_PROVIDER_TYPE_DOPPLER =
      VaultProviderType._(
          3, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_DOPPLER');
  static const VaultProviderType VAULT_PROVIDER_TYPE_AWS =
      VaultProviderType._(4, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_AWS');
  static const VaultProviderType VAULT_PROVIDER_TYPE_AWS_PARAMETER_STORE =
      VaultProviderType._(
          5, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_AWS_PARAMETER_STORE');
  static const VaultProviderType VAULT_PROVIDER_TYPE_SCALEWAY =
      VaultProviderType._(
          6, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_SCALEWAY');
  static const VaultProviderType VAULT_PROVIDER_TYPE_AZURE =
      VaultProviderType._(7, _omitEnumNames ? '' : 'VAULT_PROVIDER_TYPE_AZURE');

  static const $core.List<VaultProviderType> values = <VaultProviderType>[
    VAULT_PROVIDER_TYPE_UNSPECIFIED,
    VAULT_PROVIDER_TYPE_HASHICORP,
    VAULT_PROVIDER_TYPE_INFISICAL,
    VAULT_PROVIDER_TYPE_DOPPLER,
    VAULT_PROVIDER_TYPE_AWS,
    VAULT_PROVIDER_TYPE_AWS_PARAMETER_STORE,
    VAULT_PROVIDER_TYPE_SCALEWAY,
    VAULT_PROVIDER_TYPE_AZURE,
  ];

  static final $core.List<VaultProviderType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static VaultProviderType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const VaultProviderType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

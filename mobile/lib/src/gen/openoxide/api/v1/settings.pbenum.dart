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

import 'package:protobuf/protobuf.dart' as $pb;

/// Traefik certificate resolution mode (SettingsCertificateTypeDto).
class SettingsCertificateType extends $pb.ProtobufEnum {
  static const SettingsCertificateType SETTINGS_CERTIFICATE_TYPE_UNSPECIFIED =
      SettingsCertificateType._(
          0, _omitEnumNames ? '' : 'SETTINGS_CERTIFICATE_TYPE_UNSPECIFIED');
  static const SettingsCertificateType SETTINGS_CERTIFICATE_TYPE_NONE =
      SettingsCertificateType._(
          1, _omitEnumNames ? '' : 'SETTINGS_CERTIFICATE_TYPE_NONE');
  static const SettingsCertificateType SETTINGS_CERTIFICATE_TYPE_LETSENCRYPT =
      SettingsCertificateType._(
          2, _omitEnumNames ? '' : 'SETTINGS_CERTIFICATE_TYPE_LETSENCRYPT');
  static const SettingsCertificateType SETTINGS_CERTIFICATE_TYPE_CUSTOM =
      SettingsCertificateType._(
          3, _omitEnumNames ? '' : 'SETTINGS_CERTIFICATE_TYPE_CUSTOM');

  static const $core.List<SettingsCertificateType> values =
      <SettingsCertificateType>[
    SETTINGS_CERTIFICATE_TYPE_UNSPECIFIED,
    SETTINGS_CERTIFICATE_TYPE_NONE,
    SETTINGS_CERTIFICATE_TYPE_LETSENCRYPT,
    SETTINGS_CERTIFICATE_TYPE_CUSTOM,
  ];

  static final $core.List<SettingsCertificateType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static SettingsCertificateType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SettingsCertificateType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

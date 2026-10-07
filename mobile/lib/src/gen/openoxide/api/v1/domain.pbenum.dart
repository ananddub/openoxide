// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/domain.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class DomainType extends $pb.ProtobufEnum {
  static const DomainType DOMAIN_TYPE_UNSPECIFIED =
      DomainType._(0, _omitEnumNames ? '' : 'DOMAIN_TYPE_UNSPECIFIED');
  static const DomainType DOMAIN_TYPE_APPLICATION =
      DomainType._(1, _omitEnumNames ? '' : 'DOMAIN_TYPE_APPLICATION');
  static const DomainType DOMAIN_TYPE_COMPOSE =
      DomainType._(2, _omitEnumNames ? '' : 'DOMAIN_TYPE_COMPOSE');

  static const $core.List<DomainType> values = <DomainType>[
    DOMAIN_TYPE_UNSPECIFIED,
    DOMAIN_TYPE_APPLICATION,
    DOMAIN_TYPE_COMPOSE,
  ];

  static final $core.List<DomainType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static DomainType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const DomainType._(super.value, super.name);
}

class DomainCertificateType extends $pb.ProtobufEnum {
  static const DomainCertificateType DOMAIN_CERTIFICATE_TYPE_UNSPECIFIED =
      DomainCertificateType._(
          0, _omitEnumNames ? '' : 'DOMAIN_CERTIFICATE_TYPE_UNSPECIFIED');
  static const DomainCertificateType DOMAIN_CERTIFICATE_TYPE_LETSENCRYPT =
      DomainCertificateType._(
          1, _omitEnumNames ? '' : 'DOMAIN_CERTIFICATE_TYPE_LETSENCRYPT');
  static const DomainCertificateType DOMAIN_CERTIFICATE_TYPE_CUSTOM =
      DomainCertificateType._(
          2, _omitEnumNames ? '' : 'DOMAIN_CERTIFICATE_TYPE_CUSTOM');
  static const DomainCertificateType DOMAIN_CERTIFICATE_TYPE_NONE =
      DomainCertificateType._(
          3, _omitEnumNames ? '' : 'DOMAIN_CERTIFICATE_TYPE_NONE');

  static const $core.List<DomainCertificateType> values =
      <DomainCertificateType>[
    DOMAIN_CERTIFICATE_TYPE_UNSPECIFIED,
    DOMAIN_CERTIFICATE_TYPE_LETSENCRYPT,
    DOMAIN_CERTIFICATE_TYPE_CUSTOM,
    DOMAIN_CERTIFICATE_TYPE_NONE,
  ];

  static final $core.List<DomainCertificateType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static DomainCertificateType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const DomainCertificateType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

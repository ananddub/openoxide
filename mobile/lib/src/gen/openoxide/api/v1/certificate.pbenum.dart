// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/certificate.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CertificateRenewalStatus extends $pb.ProtobufEnum {
  static const CertificateRenewalStatus CERTIFICATE_RENEWAL_STATUS_UNSPECIFIED =
      CertificateRenewalStatus._(
          0, _omitEnumNames ? '' : 'CERTIFICATE_RENEWAL_STATUS_UNSPECIFIED');
  static const CertificateRenewalStatus CERTIFICATE_RENEWAL_STATUS_RUNNING =
      CertificateRenewalStatus._(
          1, _omitEnumNames ? '' : 'CERTIFICATE_RENEWAL_STATUS_RUNNING');
  static const CertificateRenewalStatus CERTIFICATE_RENEWAL_STATUS_SUCCEEDED =
      CertificateRenewalStatus._(
          2, _omitEnumNames ? '' : 'CERTIFICATE_RENEWAL_STATUS_SUCCEEDED');
  static const CertificateRenewalStatus CERTIFICATE_RENEWAL_STATUS_FAILED =
      CertificateRenewalStatus._(
          3, _omitEnumNames ? '' : 'CERTIFICATE_RENEWAL_STATUS_FAILED');

  static const $core.List<CertificateRenewalStatus> values =
      <CertificateRenewalStatus>[
    CERTIFICATE_RENEWAL_STATUS_UNSPECIFIED,
    CERTIFICATE_RENEWAL_STATUS_RUNNING,
    CERTIFICATE_RENEWAL_STATUS_SUCCEEDED,
    CERTIFICATE_RENEWAL_STATUS_FAILED,
  ];

  static final $core.List<CertificateRenewalStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static CertificateRenewalStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const CertificateRenewalStatus._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/dns.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class DnsProviderType extends $pb.ProtobufEnum {
  static const DnsProviderType DNS_PROVIDER_TYPE_UNSPECIFIED =
      DnsProviderType._(
          0, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_UNSPECIFIED');
  static const DnsProviderType DNS_PROVIDER_TYPE_CLOUDFLARE = DnsProviderType._(
      1, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_CLOUDFLARE');
  static const DnsProviderType DNS_PROVIDER_TYPE_ROUTE53 =
      DnsProviderType._(2, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_ROUTE53');
  static const DnsProviderType DNS_PROVIDER_TYPE_PORKBUN =
      DnsProviderType._(6, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_PORKBUN');
  static const DnsProviderType DNS_PROVIDER_TYPE_INFOMANIAK = DnsProviderType._(
      7, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_INFOMANIAK');
  static const DnsProviderType DNS_PROVIDER_TYPE_OVH =
      DnsProviderType._(8, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_OVH');
  static const DnsProviderType DNS_PROVIDER_TYPE_SPACESHIP = DnsProviderType._(
      16, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_SPACESHIP');
  static const DnsProviderType DNS_PROVIDER_TYPE_HOSTINGER = DnsProviderType._(
      38, _omitEnumNames ? '' : 'DNS_PROVIDER_TYPE_HOSTINGER');

  static const $core.List<DnsProviderType> values = <DnsProviderType>[
    DNS_PROVIDER_TYPE_UNSPECIFIED,
    DNS_PROVIDER_TYPE_CLOUDFLARE,
    DNS_PROVIDER_TYPE_ROUTE53,
    DNS_PROVIDER_TYPE_PORKBUN,
    DNS_PROVIDER_TYPE_INFOMANIAK,
    DNS_PROVIDER_TYPE_OVH,
    DNS_PROVIDER_TYPE_SPACESHIP,
    DNS_PROVIDER_TYPE_HOSTINGER,
  ];

  static final $core.Map<$core.int, DnsProviderType> _byValue =
      $pb.ProtobufEnum.initByValue(values);
  static DnsProviderType? valueOf($core.int value) => _byValue[value];

  const DnsProviderType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

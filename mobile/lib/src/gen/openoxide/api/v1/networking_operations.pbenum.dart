// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/networking_operations.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set mirroring `CdnProvider`.
class NetworkingCdnProvider extends $pb.ProtobufEnum {
  static const NetworkingCdnProvider NETWORKING_CDN_PROVIDER_UNSPECIFIED =
      NetworkingCdnProvider._(
          0, _omitEnumNames ? '' : 'NETWORKING_CDN_PROVIDER_UNSPECIFIED');
  static const NetworkingCdnProvider NETWORKING_CDN_PROVIDER_CLOUDFLARE =
      NetworkingCdnProvider._(
          1, _omitEnumNames ? '' : 'NETWORKING_CDN_PROVIDER_CLOUDFLARE');
  static const NetworkingCdnProvider NETWORKING_CDN_PROVIDER_FASTLY =
      NetworkingCdnProvider._(
          2, _omitEnumNames ? '' : 'NETWORKING_CDN_PROVIDER_FASTLY');
  static const NetworkingCdnProvider NETWORKING_CDN_PROVIDER_BUNNY =
      NetworkingCdnProvider._(
          3, _omitEnumNames ? '' : 'NETWORKING_CDN_PROVIDER_BUNNY');

  static const $core.List<NetworkingCdnProvider> values =
      <NetworkingCdnProvider>[
    NETWORKING_CDN_PROVIDER_UNSPECIFIED,
    NETWORKING_CDN_PROVIDER_CLOUDFLARE,
    NETWORKING_CDN_PROVIDER_FASTLY,
    NETWORKING_CDN_PROVIDER_BUNNY,
  ];

  static final $core.List<NetworkingCdnProvider?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static NetworkingCdnProvider? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const NetworkingCdnProvider._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

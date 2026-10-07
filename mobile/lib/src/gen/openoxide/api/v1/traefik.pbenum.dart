// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/traefik.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class TraefikFileNodeType extends $pb.ProtobufEnum {
  static const TraefikFileNodeType TRAEFIK_FILE_NODE_TYPE_UNSPECIFIED =
      TraefikFileNodeType._(
          0, _omitEnumNames ? '' : 'TRAEFIK_FILE_NODE_TYPE_UNSPECIFIED');
  static const TraefikFileNodeType TRAEFIK_FILE_NODE_TYPE_FILE =
      TraefikFileNodeType._(
          1, _omitEnumNames ? '' : 'TRAEFIK_FILE_NODE_TYPE_FILE');
  static const TraefikFileNodeType TRAEFIK_FILE_NODE_TYPE_DIRECTORY =
      TraefikFileNodeType._(
          2, _omitEnumNames ? '' : 'TRAEFIK_FILE_NODE_TYPE_DIRECTORY');

  static const $core.List<TraefikFileNodeType> values = <TraefikFileNodeType>[
    TRAEFIK_FILE_NODE_TYPE_UNSPECIFIED,
    TRAEFIK_FILE_NODE_TYPE_FILE,
    TRAEFIK_FILE_NODE_TYPE_DIRECTORY,
  ];

  static final $core.List<TraefikFileNodeType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static TraefikFileNodeType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const TraefikFileNodeType._(super.value, super.name);
}

/// Closed set mirroring `StructuredMiddlewareKind`.
class TraefikMiddlewareKind extends $pb.ProtobufEnum {
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_UNSPECIFIED =
      TraefikMiddlewareKind._(
          0, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_UNSPECIFIED');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_STRIP_PREFIX =
      TraefikMiddlewareKind._(
          1, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_STRIP_PREFIX');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_ADD_PREFIX =
      TraefikMiddlewareKind._(
          2, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_ADD_PREFIX');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_REDIRECT_SCHEME =
      TraefikMiddlewareKind._(
          3, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_REDIRECT_SCHEME');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_COMPRESS =
      TraefikMiddlewareKind._(
          4, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_COMPRESS');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_RATE_LIMIT =
      TraefikMiddlewareKind._(
          5, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_RATE_LIMIT');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_IP_ALLOW_LIST =
      TraefikMiddlewareKind._(
          6, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_IP_ALLOW_LIST');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_REQUEST_HEADERS =
      TraefikMiddlewareKind._(
          7, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_REQUEST_HEADERS');
  static const TraefikMiddlewareKind TRAEFIK_MIDDLEWARE_KIND_RESPONSE_HEADERS =
      TraefikMiddlewareKind._(
          8, _omitEnumNames ? '' : 'TRAEFIK_MIDDLEWARE_KIND_RESPONSE_HEADERS');

  static const $core.List<TraefikMiddlewareKind> values =
      <TraefikMiddlewareKind>[
    TRAEFIK_MIDDLEWARE_KIND_UNSPECIFIED,
    TRAEFIK_MIDDLEWARE_KIND_STRIP_PREFIX,
    TRAEFIK_MIDDLEWARE_KIND_ADD_PREFIX,
    TRAEFIK_MIDDLEWARE_KIND_REDIRECT_SCHEME,
    TRAEFIK_MIDDLEWARE_KIND_COMPRESS,
    TRAEFIK_MIDDLEWARE_KIND_RATE_LIMIT,
    TRAEFIK_MIDDLEWARE_KIND_IP_ALLOW_LIST,
    TRAEFIK_MIDDLEWARE_KIND_REQUEST_HEADERS,
    TRAEFIK_MIDDLEWARE_KIND_RESPONSE_HEADERS,
  ];

  static final $core.List<TraefikMiddlewareKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 8);
  static TraefikMiddlewareKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const TraefikMiddlewareKind._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

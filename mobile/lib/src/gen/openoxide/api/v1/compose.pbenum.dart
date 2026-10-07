// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/compose.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ComposeMountType extends $pb.ProtobufEnum {
  static const ComposeMountType COMPOSE_MOUNT_TYPE_UNSPECIFIED =
      ComposeMountType._(
          0, _omitEnumNames ? '' : 'COMPOSE_MOUNT_TYPE_UNSPECIFIED');
  static const ComposeMountType COMPOSE_MOUNT_TYPE_VOLUME =
      ComposeMountType._(1, _omitEnumNames ? '' : 'COMPOSE_MOUNT_TYPE_VOLUME');
  static const ComposeMountType COMPOSE_MOUNT_TYPE_BIND =
      ComposeMountType._(2, _omitEnumNames ? '' : 'COMPOSE_MOUNT_TYPE_BIND');
  static const ComposeMountType COMPOSE_MOUNT_TYPE_FILE =
      ComposeMountType._(3, _omitEnumNames ? '' : 'COMPOSE_MOUNT_TYPE_FILE');

  static const $core.List<ComposeMountType> values = <ComposeMountType>[
    COMPOSE_MOUNT_TYPE_UNSPECIFIED,
    COMPOSE_MOUNT_TYPE_VOLUME,
    COMPOSE_MOUNT_TYPE_BIND,
    COMPOSE_MOUNT_TYPE_FILE,
  ];

  static final $core.List<ComposeMountType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static ComposeMountType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ComposeMountType._(super.value, super.name);
}

class ComposePatchType extends $pb.ProtobufEnum {
  static const ComposePatchType COMPOSE_PATCH_TYPE_UNSPECIFIED =
      ComposePatchType._(
          0, _omitEnumNames ? '' : 'COMPOSE_PATCH_TYPE_UNSPECIFIED');
  static const ComposePatchType COMPOSE_PATCH_TYPE_UPDATE =
      ComposePatchType._(1, _omitEnumNames ? '' : 'COMPOSE_PATCH_TYPE_UPDATE');
  static const ComposePatchType COMPOSE_PATCH_TYPE_DELETE =
      ComposePatchType._(2, _omitEnumNames ? '' : 'COMPOSE_PATCH_TYPE_DELETE');

  static const $core.List<ComposePatchType> values = <ComposePatchType>[
    COMPOSE_PATCH_TYPE_UNSPECIFIED,
    COMPOSE_PATCH_TYPE_UPDATE,
    COMPOSE_PATCH_TYPE_DELETE,
  ];

  static final $core.List<ComposePatchType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ComposePatchType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ComposePatchType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/global.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Lifecycle actions accepted by `container_action` (ContainerActionDto).
class GlobalContainerAction extends $pb.ProtobufEnum {
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_UNSPECIFIED =
      GlobalContainerAction._(
          0, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_UNSPECIFIED');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_START =
      GlobalContainerAction._(
          1, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_START');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_STOP =
      GlobalContainerAction._(
          2, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_STOP');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_RESTART =
      GlobalContainerAction._(
          3, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_RESTART');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_KILL =
      GlobalContainerAction._(
          4, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_KILL');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_PAUSE =
      GlobalContainerAction._(
          5, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_PAUSE');
  static const GlobalContainerAction GLOBAL_CONTAINER_ACTION_UNPAUSE =
      GlobalContainerAction._(
          6, _omitEnumNames ? '' : 'GLOBAL_CONTAINER_ACTION_UNPAUSE');

  static const $core.List<GlobalContainerAction> values =
      <GlobalContainerAction>[
    GLOBAL_CONTAINER_ACTION_UNSPECIFIED,
    GLOBAL_CONTAINER_ACTION_START,
    GLOBAL_CONTAINER_ACTION_STOP,
    GLOBAL_CONTAINER_ACTION_RESTART,
    GLOBAL_CONTAINER_ACTION_KILL,
    GLOBAL_CONTAINER_ACTION_PAUSE,
    GLOBAL_CONTAINER_ACTION_UNPAUSE,
  ];

  static final $core.List<GlobalContainerAction?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static GlobalContainerAction? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const GlobalContainerAction._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

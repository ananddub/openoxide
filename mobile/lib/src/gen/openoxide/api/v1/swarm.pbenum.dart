// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/swarm.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SwarmNodeRole extends $pb.ProtobufEnum {
  static const SwarmNodeRole SWARM_NODE_ROLE_UNSPECIFIED =
      SwarmNodeRole._(0, _omitEnumNames ? '' : 'SWARM_NODE_ROLE_UNSPECIFIED');
  static const SwarmNodeRole SWARM_NODE_ROLE_WORKER =
      SwarmNodeRole._(1, _omitEnumNames ? '' : 'SWARM_NODE_ROLE_WORKER');
  static const SwarmNodeRole SWARM_NODE_ROLE_MANAGER =
      SwarmNodeRole._(2, _omitEnumNames ? '' : 'SWARM_NODE_ROLE_MANAGER');

  static const $core.List<SwarmNodeRole> values = <SwarmNodeRole>[
    SWARM_NODE_ROLE_UNSPECIFIED,
    SWARM_NODE_ROLE_WORKER,
    SWARM_NODE_ROLE_MANAGER,
  ];

  static final $core.List<SwarmNodeRole?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static SwarmNodeRole? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SwarmNodeRole._(super.value, super.name);
}

class SwarmRole extends $pb.ProtobufEnum {
  static const SwarmRole SWARM_ROLE_UNSPECIFIED =
      SwarmRole._(0, _omitEnumNames ? '' : 'SWARM_ROLE_UNSPECIFIED');
  static const SwarmRole SWARM_ROLE_WORKER =
      SwarmRole._(1, _omitEnumNames ? '' : 'SWARM_ROLE_WORKER');
  static const SwarmRole SWARM_ROLE_MANAGER =
      SwarmRole._(2, _omitEnumNames ? '' : 'SWARM_ROLE_MANAGER');

  static const $core.List<SwarmRole> values = <SwarmRole>[
    SWARM_ROLE_UNSPECIFIED,
    SWARM_ROLE_WORKER,
    SWARM_ROLE_MANAGER,
  ];

  static final $core.List<SwarmRole?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static SwarmRole? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SwarmRole._(super.value, super.name);
}

class SwarmNodeAvailability extends $pb.ProtobufEnum {
  static const SwarmNodeAvailability SWARM_NODE_AVAILABILITY_UNSPECIFIED =
      SwarmNodeAvailability._(
          0, _omitEnumNames ? '' : 'SWARM_NODE_AVAILABILITY_UNSPECIFIED');
  static const SwarmNodeAvailability SWARM_NODE_AVAILABILITY_ACTIVE =
      SwarmNodeAvailability._(
          1, _omitEnumNames ? '' : 'SWARM_NODE_AVAILABILITY_ACTIVE');
  static const SwarmNodeAvailability SWARM_NODE_AVAILABILITY_PAUSE =
      SwarmNodeAvailability._(
          2, _omitEnumNames ? '' : 'SWARM_NODE_AVAILABILITY_PAUSE');
  static const SwarmNodeAvailability SWARM_NODE_AVAILABILITY_DRAIN =
      SwarmNodeAvailability._(
          3, _omitEnumNames ? '' : 'SWARM_NODE_AVAILABILITY_DRAIN');

  static const $core.List<SwarmNodeAvailability> values =
      <SwarmNodeAvailability>[
    SWARM_NODE_AVAILABILITY_UNSPECIFIED,
    SWARM_NODE_AVAILABILITY_ACTIVE,
    SWARM_NODE_AVAILABILITY_PAUSE,
    SWARM_NODE_AVAILABILITY_DRAIN,
  ];

  static final $core.List<SwarmNodeAvailability?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static SwarmNodeAvailability? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SwarmNodeAvailability._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/terminal.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class TerminalKind extends $pb.ProtobufEnum {
  static const TerminalKind TERMINAL_KIND_UNSPECIFIED =
      TerminalKind._(0, _omitEnumNames ? '' : 'TERMINAL_KIND_UNSPECIFIED');
  static const TerminalKind TERMINAL_KIND_DOCKER =
      TerminalKind._(1, _omitEnumNames ? '' : 'TERMINAL_KIND_DOCKER');
  static const TerminalKind TERMINAL_KIND_REMOTE_SERVER =
      TerminalKind._(2, _omitEnumNames ? '' : 'TERMINAL_KIND_REMOTE_SERVER');

  static const $core.List<TerminalKind> values = <TerminalKind>[
    TERMINAL_KIND_UNSPECIFIED,
    TERMINAL_KIND_DOCKER,
    TERMINAL_KIND_REMOTE_SERVER,
  ];

  static final $core.List<TerminalKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static TerminalKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const TerminalKind._(super.value, super.name);
}

class TerminalOutputStream extends $pb.ProtobufEnum {
  static const TerminalOutputStream TERMINAL_OUTPUT_STREAM_UNSPECIFIED =
      TerminalOutputStream._(
          0, _omitEnumNames ? '' : 'TERMINAL_OUTPUT_STREAM_UNSPECIFIED');
  static const TerminalOutputStream TERMINAL_OUTPUT_STREAM_STDOUT =
      TerminalOutputStream._(
          1, _omitEnumNames ? '' : 'TERMINAL_OUTPUT_STREAM_STDOUT');
  static const TerminalOutputStream TERMINAL_OUTPUT_STREAM_STDERR =
      TerminalOutputStream._(
          2, _omitEnumNames ? '' : 'TERMINAL_OUTPUT_STREAM_STDERR');

  static const $core.List<TerminalOutputStream> values = <TerminalOutputStream>[
    TERMINAL_OUTPUT_STREAM_UNSPECIFIED,
    TERMINAL_OUTPUT_STREAM_STDOUT,
    TERMINAL_OUTPUT_STREAM_STDERR,
  ];

  static final $core.List<TerminalOutputStream?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static TerminalOutputStream? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const TerminalOutputStream._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

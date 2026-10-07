// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/database.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class DatabaseKind extends $pb.ProtobufEnum {
  static const DatabaseKind DATABASE_KIND_UNSPECIFIED =
      DatabaseKind._(0, _omitEnumNames ? '' : 'DATABASE_KIND_UNSPECIFIED');
  static const DatabaseKind DATABASE_KIND_POSTGRES =
      DatabaseKind._(1, _omitEnumNames ? '' : 'DATABASE_KIND_POSTGRES');
  static const DatabaseKind DATABASE_KIND_MYSQL =
      DatabaseKind._(2, _omitEnumNames ? '' : 'DATABASE_KIND_MYSQL');
  static const DatabaseKind DATABASE_KIND_MARIADB =
      DatabaseKind._(3, _omitEnumNames ? '' : 'DATABASE_KIND_MARIADB');
  static const DatabaseKind DATABASE_KIND_MONGO =
      DatabaseKind._(4, _omitEnumNames ? '' : 'DATABASE_KIND_MONGO');
  static const DatabaseKind DATABASE_KIND_REDIS =
      DatabaseKind._(5, _omitEnumNames ? '' : 'DATABASE_KIND_REDIS');
  static const DatabaseKind DATABASE_KIND_LIBSQL =
      DatabaseKind._(6, _omitEnumNames ? '' : 'DATABASE_KIND_LIBSQL');

  static const $core.List<DatabaseKind> values = <DatabaseKind>[
    DATABASE_KIND_UNSPECIFIED,
    DATABASE_KIND_POSTGRES,
    DATABASE_KIND_MYSQL,
    DATABASE_KIND_MARIADB,
    DATABASE_KIND_MONGO,
    DATABASE_KIND_REDIS,
    DATABASE_KIND_LIBSQL,
  ];

  static final $core.List<DatabaseKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static DatabaseKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const DatabaseKind._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

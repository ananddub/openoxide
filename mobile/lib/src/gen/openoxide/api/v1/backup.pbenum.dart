// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/backup.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class BackupKind extends $pb.ProtobufEnum {
  static const BackupKind BACKUP_KIND_UNSPECIFIED =
      BackupKind._(0, _omitEnumNames ? '' : 'BACKUP_KIND_UNSPECIFIED');
  static const BackupKind BACKUP_KIND_DATABASE =
      BackupKind._(1, _omitEnumNames ? '' : 'BACKUP_KIND_DATABASE');
  static const BackupKind BACKUP_KIND_VOLUME =
      BackupKind._(2, _omitEnumNames ? '' : 'BACKUP_KIND_VOLUME');
  static const BackupKind BACKUP_KIND_PANEL =
      BackupKind._(3, _omitEnumNames ? '' : 'BACKUP_KIND_PANEL');
  static const BackupKind BACKUP_KIND_COMPOSE_CONFIG =
      BackupKind._(4, _omitEnumNames ? '' : 'BACKUP_KIND_COMPOSE_CONFIG');

  static const $core.List<BackupKind> values = <BackupKind>[
    BACKUP_KIND_UNSPECIFIED,
    BACKUP_KIND_DATABASE,
    BACKUP_KIND_VOLUME,
    BACKUP_KIND_PANEL,
    BACKUP_KIND_COMPOSE_CONFIG,
  ];

  static final $core.List<BackupKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static BackupKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const BackupKind._(super.value, super.name);
}

class BackupOperation extends $pb.ProtobufEnum {
  static const BackupOperation BACKUP_OPERATION_UNSPECIFIED = BackupOperation._(
      0, _omitEnumNames ? '' : 'BACKUP_OPERATION_UNSPECIFIED');
  static const BackupOperation BACKUP_OPERATION_BACKUP =
      BackupOperation._(1, _omitEnumNames ? '' : 'BACKUP_OPERATION_BACKUP');
  static const BackupOperation BACKUP_OPERATION_RESTORE =
      BackupOperation._(2, _omitEnumNames ? '' : 'BACKUP_OPERATION_RESTORE');

  static const $core.List<BackupOperation> values = <BackupOperation>[
    BACKUP_OPERATION_UNSPECIFIED,
    BACKUP_OPERATION_BACKUP,
    BACKUP_OPERATION_RESTORE,
  ];

  static final $core.List<BackupOperation?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static BackupOperation? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const BackupOperation._(super.value, super.name);
}

class BackupExecutionStatus extends $pb.ProtobufEnum {
  static const BackupExecutionStatus BACKUP_EXECUTION_STATUS_UNSPECIFIED =
      BackupExecutionStatus._(
          0, _omitEnumNames ? '' : 'BACKUP_EXECUTION_STATUS_UNSPECIFIED');
  static const BackupExecutionStatus BACKUP_EXECUTION_STATUS_RUNNING =
      BackupExecutionStatus._(
          1, _omitEnumNames ? '' : 'BACKUP_EXECUTION_STATUS_RUNNING');
  static const BackupExecutionStatus BACKUP_EXECUTION_STATUS_SUCCEEDED =
      BackupExecutionStatus._(
          2, _omitEnumNames ? '' : 'BACKUP_EXECUTION_STATUS_SUCCEEDED');
  static const BackupExecutionStatus BACKUP_EXECUTION_STATUS_FAILED =
      BackupExecutionStatus._(
          3, _omitEnumNames ? '' : 'BACKUP_EXECUTION_STATUS_FAILED');

  static const $core.List<BackupExecutionStatus> values =
      <BackupExecutionStatus>[
    BACKUP_EXECUTION_STATUS_UNSPECIFIED,
    BACKUP_EXECUTION_STATUS_RUNNING,
    BACKUP_EXECUTION_STATUS_SUCCEEDED,
    BACKUP_EXECUTION_STATUS_FAILED,
  ];

  static final $core.List<BackupExecutionStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static BackupExecutionStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const BackupExecutionStatus._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

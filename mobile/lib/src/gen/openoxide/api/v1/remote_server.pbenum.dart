// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/remote_server.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set: `RemoteServerTypeDto` (DEPLOY/BUILD).
class RemoteServerType extends $pb.ProtobufEnum {
  static const RemoteServerType REMOTE_SERVER_TYPE_UNSPECIFIED =
      RemoteServerType._(
          0, _omitEnumNames ? '' : 'REMOTE_SERVER_TYPE_UNSPECIFIED');
  static const RemoteServerType REMOTE_SERVER_TYPE_DEPLOY =
      RemoteServerType._(1, _omitEnumNames ? '' : 'REMOTE_SERVER_TYPE_DEPLOY');
  static const RemoteServerType REMOTE_SERVER_TYPE_BUILD =
      RemoteServerType._(2, _omitEnumNames ? '' : 'REMOTE_SERVER_TYPE_BUILD');

  static const $core.List<RemoteServerType> values = <RemoteServerType>[
    REMOTE_SERVER_TYPE_UNSPECIFIED,
    REMOTE_SERVER_TYPE_DEPLOY,
    REMOTE_SERVER_TYPE_BUILD,
  ];

  static final $core.List<RemoteServerType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static RemoteServerType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerType._(super.value, super.name);
}

/// Closed set: `RemoteServerStatus` / `RemoteServerStatusDto` (ACTIVE/INACTIVE).
class RemoteServerStatus extends $pb.ProtobufEnum {
  static const RemoteServerStatus REMOTE_SERVER_STATUS_UNSPECIFIED =
      RemoteServerStatus._(
          0, _omitEnumNames ? '' : 'REMOTE_SERVER_STATUS_UNSPECIFIED');
  static const RemoteServerStatus REMOTE_SERVER_STATUS_ACTIVE =
      RemoteServerStatus._(
          1, _omitEnumNames ? '' : 'REMOTE_SERVER_STATUS_ACTIVE');
  static const RemoteServerStatus REMOTE_SERVER_STATUS_INACTIVE =
      RemoteServerStatus._(
          2, _omitEnumNames ? '' : 'REMOTE_SERVER_STATUS_INACTIVE');

  static const $core.List<RemoteServerStatus> values = <RemoteServerStatus>[
    REMOTE_SERVER_STATUS_UNSPECIFIED,
    REMOTE_SERVER_STATUS_ACTIVE,
    REMOTE_SERVER_STATUS_INACTIVE,
  ];

  static final $core.List<RemoteServerStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static RemoteServerStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerStatus._(super.value, super.name);
}

/// Closed set: `RemoteServerAction` enum.
class RemoteServerAction extends $pb.ProtobufEnum {
  static const RemoteServerAction REMOTE_SERVER_ACTION_UNSPECIFIED =
      RemoteServerAction._(
          0, _omitEnumNames ? '' : 'REMOTE_SERVER_ACTION_UNSPECIFIED');
  static const RemoteServerAction REMOTE_SERVER_ACTION_ACTIVATE =
      RemoteServerAction._(
          1, _omitEnumNames ? '' : 'REMOTE_SERVER_ACTION_ACTIVATE');
  static const RemoteServerAction REMOTE_SERVER_ACTION_DEACTIVATE =
      RemoteServerAction._(
          2, _omitEnumNames ? '' : 'REMOTE_SERVER_ACTION_DEACTIVATE');
  static const RemoteServerAction REMOTE_SERVER_ACTION_TEST_CONNECTION =
      RemoteServerAction._(
          3, _omitEnumNames ? '' : 'REMOTE_SERVER_ACTION_TEST_CONNECTION');

  static const $core.List<RemoteServerAction> values = <RemoteServerAction>[
    REMOTE_SERVER_ACTION_UNSPECIFIED,
    REMOTE_SERVER_ACTION_ACTIVATE,
    REMOTE_SERVER_ACTION_DEACTIVATE,
    REMOTE_SERVER_ACTION_TEST_CONNECTION,
  ];

  static final $core.List<RemoteServerAction?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static RemoteServerAction? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerAction._(super.value, super.name);
}

/// Closed set: the only status literals written for `server_migrations` rows.
class RemoteServerDependencyMigrationStatus extends $pb.ProtobufEnum {
  static const RemoteServerDependencyMigrationStatus
      REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_UNSPECIFIED =
      RemoteServerDependencyMigrationStatus._(
          0,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_UNSPECIFIED');
  static const RemoteServerDependencyMigrationStatus
      REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_RUNNING =
      RemoteServerDependencyMigrationStatus._(
          1,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_RUNNING');
  static const RemoteServerDependencyMigrationStatus
      REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_SUCCEEDED =
      RemoteServerDependencyMigrationStatus._(
          2,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_SUCCEEDED');
  static const RemoteServerDependencyMigrationStatus
      REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_FAILED =
      RemoteServerDependencyMigrationStatus._(
          3,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_FAILED');
  static const RemoteServerDependencyMigrationStatus
      REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_ROLLED_BACK =
      RemoteServerDependencyMigrationStatus._(
          4,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_ROLLED_BACK');

  static const $core.List<RemoteServerDependencyMigrationStatus> values =
      <RemoteServerDependencyMigrationStatus>[
    REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_UNSPECIFIED,
    REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_RUNNING,
    REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_SUCCEEDED,
    REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_FAILED,
    REMOTE_SERVER_DEPENDENCY_MIGRATION_STATUS_ROLLED_BACK,
  ];

  static final $core.List<RemoteServerDependencyMigrationStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static RemoteServerDependencyMigrationStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerDependencyMigrationStatus._(super.value, super.name);
}

/// Closed set: status literals written for `application_migrations` rows ("RUNNING"/"APPLYING" from the phase setter, plus the terminal updates).
class RemoteServerApplicationMigrationStatus extends $pb.ProtobufEnum {
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_UNSPECIFIED =
      RemoteServerApplicationMigrationStatus._(
          0,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_UNSPECIFIED');
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_RUNNING =
      RemoteServerApplicationMigrationStatus._(
          1,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_RUNNING');
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_APPLYING =
      RemoteServerApplicationMigrationStatus._(
          2,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_APPLYING');
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_SUCCEEDED =
      RemoteServerApplicationMigrationStatus._(
          3,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_SUCCEEDED');
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_FAILED =
      RemoteServerApplicationMigrationStatus._(
          4,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_FAILED');
  static const RemoteServerApplicationMigrationStatus
      REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_CANCELLED =
      RemoteServerApplicationMigrationStatus._(
          5,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_CANCELLED');

  static const $core.List<RemoteServerApplicationMigrationStatus> values =
      <RemoteServerApplicationMigrationStatus>[
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_UNSPECIFIED,
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_RUNNING,
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_APPLYING,
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_SUCCEEDED,
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_FAILED,
    REMOTE_SERVER_APPLICATION_MIGRATION_STATUS_CANCELLED,
  ];

  static final $core.List<RemoteServerApplicationMigrationStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static RemoteServerApplicationMigrationStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerApplicationMigrationStatus._(super.value, super.name);
}

/// Closed set: `MigrationPhase::as_str` match arms plus the terminal DONE stamp.
class RemoteServerMigrationPhase extends $pb.ProtobufEnum {
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_UNSPECIFIED = RemoteServerMigrationPhase._(
          0, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_UNSPECIFIED');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_PREFLIGHT = RemoteServerMigrationPhase._(
          1, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_PREFLIGHT');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_QUIESCE = RemoteServerMigrationPhase._(
          2, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_QUIESCE');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_SYNC =
      RemoteServerMigrationPhase._(
          3, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_SYNC');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_VERIFY =
      RemoteServerMigrationPhase._(
          4, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_VERIFY');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_APPLY =
      RemoteServerMigrationPhase._(
          5, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_APPLY');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_REPOINT = RemoteServerMigrationPhase._(
          6, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_REPOINT');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_REDEPLOY = RemoteServerMigrationPhase._(
          7, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_REDEPLOY');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_HEALTH =
      RemoteServerMigrationPhase._(
          8, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_HEALTH');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_SUPERSEDED = RemoteServerMigrationPhase._(
          9, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_SUPERSEDED');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_FAILED =
      RemoteServerMigrationPhase._(
          10, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_FAILED');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_ROLLED_BACK = RemoteServerMigrationPhase._(
          11,
          _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_ROLLED_BACK');
  static const RemoteServerMigrationPhase
      REMOTE_SERVER_MIGRATION_PHASE_CANCELLED = RemoteServerMigrationPhase._(
          12, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_CANCELLED');
  static const RemoteServerMigrationPhase REMOTE_SERVER_MIGRATION_PHASE_DONE =
      RemoteServerMigrationPhase._(
          13, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_PHASE_DONE');

  static const $core.List<RemoteServerMigrationPhase> values =
      <RemoteServerMigrationPhase>[
    REMOTE_SERVER_MIGRATION_PHASE_UNSPECIFIED,
    REMOTE_SERVER_MIGRATION_PHASE_PREFLIGHT,
    REMOTE_SERVER_MIGRATION_PHASE_QUIESCE,
    REMOTE_SERVER_MIGRATION_PHASE_SYNC,
    REMOTE_SERVER_MIGRATION_PHASE_VERIFY,
    REMOTE_SERVER_MIGRATION_PHASE_APPLY,
    REMOTE_SERVER_MIGRATION_PHASE_REPOINT,
    REMOTE_SERVER_MIGRATION_PHASE_REDEPLOY,
    REMOTE_SERVER_MIGRATION_PHASE_HEALTH,
    REMOTE_SERVER_MIGRATION_PHASE_SUPERSEDED,
    REMOTE_SERVER_MIGRATION_PHASE_FAILED,
    REMOTE_SERVER_MIGRATION_PHASE_ROLLED_BACK,
    REMOTE_SERVER_MIGRATION_PHASE_CANCELLED,
    REMOTE_SERVER_MIGRATION_PHASE_DONE,
  ];

  static final $core.List<RemoteServerMigrationPhase?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 13);
  static RemoteServerMigrationPhase? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerMigrationPhase._(super.value, super.name);
}

/// Closed set: `MigrationObjectKind::as_str` match arms.
class RemoteServerMigrationObjectKind extends $pb.ProtobufEnum {
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_UNSPECIFIED =
      RemoteServerMigrationObjectKind._(
          0,
          _omitEnumNames
              ? ''
              : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_UNSPECIFIED');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_VOLUME =
      RemoteServerMigrationObjectKind._(1,
          _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_VOLUME');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_BIND =
      RemoteServerMigrationObjectKind._(
          2, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_BIND');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_DBDATA =
      RemoteServerMigrationObjectKind._(3,
          _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_DBDATA');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_DUMP =
      RemoteServerMigrationObjectKind._(
          4, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_DUMP');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_SRC =
      RemoteServerMigrationObjectKind._(
          5, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_SRC');
  static const RemoteServerMigrationObjectKind
      REMOTE_SERVER_MIGRATION_OBJECT_KIND_CERT =
      RemoteServerMigrationObjectKind._(
          6, _omitEnumNames ? '' : 'REMOTE_SERVER_MIGRATION_OBJECT_KIND_CERT');

  static const $core.List<RemoteServerMigrationObjectKind> values =
      <RemoteServerMigrationObjectKind>[
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_UNSPECIFIED,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_VOLUME,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_BIND,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_DBDATA,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_DUMP,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_SRC,
    REMOTE_SERVER_MIGRATION_OBJECT_KIND_CERT,
  ];

  static final $core.List<RemoteServerMigrationObjectKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static RemoteServerMigrationObjectKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RemoteServerMigrationObjectKind._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

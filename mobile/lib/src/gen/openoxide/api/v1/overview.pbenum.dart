// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/overview.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Panel-normalised service kind ("APP" | "COMPOSE" | "DATABASE").
class OverviewServiceType extends $pb.ProtobufEnum {
  static const OverviewServiceType OVERVIEW_SERVICE_TYPE_UNSPECIFIED =
      OverviewServiceType._(
          0, _omitEnumNames ? '' : 'OVERVIEW_SERVICE_TYPE_UNSPECIFIED');
  static const OverviewServiceType OVERVIEW_SERVICE_TYPE_APP =
      OverviewServiceType._(
          1, _omitEnumNames ? '' : 'OVERVIEW_SERVICE_TYPE_APP');
  static const OverviewServiceType OVERVIEW_SERVICE_TYPE_COMPOSE =
      OverviewServiceType._(
          2, _omitEnumNames ? '' : 'OVERVIEW_SERVICE_TYPE_COMPOSE');
  static const OverviewServiceType OVERVIEW_SERVICE_TYPE_DATABASE =
      OverviewServiceType._(
          3, _omitEnumNames ? '' : 'OVERVIEW_SERVICE_TYPE_DATABASE');

  static const $core.List<OverviewServiceType> values = <OverviewServiceType>[
    OVERVIEW_SERVICE_TYPE_UNSPECIFIED,
    OVERVIEW_SERVICE_TYPE_APP,
    OVERVIEW_SERVICE_TYPE_COMPOSE,
    OVERVIEW_SERVICE_TYPE_DATABASE,
  ];

  static final $core.List<OverviewServiceType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static OverviewServiceType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const OverviewServiceType._(super.value, super.name);
}

class OverviewBackupType extends $pb.ProtobufEnum {
  static const OverviewBackupType OVERVIEW_BACKUP_TYPE_UNSPECIFIED =
      OverviewBackupType._(
          0, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_TYPE_UNSPECIFIED');
  static const OverviewBackupType OVERVIEW_BACKUP_TYPE_DATABASE =
      OverviewBackupType._(
          1, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_TYPE_DATABASE');
  static const OverviewBackupType OVERVIEW_BACKUP_TYPE_VOLUME =
      OverviewBackupType._(
          2, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_TYPE_VOLUME');

  static const $core.List<OverviewBackupType> values = <OverviewBackupType>[
    OVERVIEW_BACKUP_TYPE_UNSPECIFIED,
    OVERVIEW_BACKUP_TYPE_DATABASE,
    OVERVIEW_BACKUP_TYPE_VOLUME,
  ];

  static final $core.List<OverviewBackupType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static OverviewBackupType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const OverviewBackupType._(super.value, super.name);
}

class OverviewBackupStatus extends $pb.ProtobufEnum {
  static const OverviewBackupStatus OVERVIEW_BACKUP_STATUS_UNSPECIFIED =
      OverviewBackupStatus._(
          0, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_STATUS_UNSPECIFIED');
  static const OverviewBackupStatus OVERVIEW_BACKUP_STATUS_ENABLED =
      OverviewBackupStatus._(
          1, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_STATUS_ENABLED');
  static const OverviewBackupStatus OVERVIEW_BACKUP_STATUS_DISABLED =
      OverviewBackupStatus._(
          2, _omitEnumNames ? '' : 'OVERVIEW_BACKUP_STATUS_DISABLED');

  static const $core.List<OverviewBackupStatus> values = <OverviewBackupStatus>[
    OVERVIEW_BACKUP_STATUS_UNSPECIFIED,
    OVERVIEW_BACKUP_STATUS_ENABLED,
    OVERVIEW_BACKUP_STATUS_DISABLED,
  ];

  static final $core.List<OverviewBackupStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static OverviewBackupStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const OverviewBackupStatus._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

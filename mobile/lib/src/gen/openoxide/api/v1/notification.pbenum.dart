// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/notification.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class NotificationProviderType extends $pb.ProtobufEnum {
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_UNSPECIFIED =
      NotificationProviderType._(
          0, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_UNSPECIFIED');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_SLACK =
      NotificationProviderType._(
          1, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_SLACK');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_TELEGRAM =
      NotificationProviderType._(
          2, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_TELEGRAM');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_DISCORD =
      NotificationProviderType._(
          3, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_DISCORD');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_EMAIL =
      NotificationProviderType._(
          4, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_EMAIL');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_RESEND =
      NotificationProviderType._(
          5, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_RESEND');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_GOTIFY =
      NotificationProviderType._(
          6, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_GOTIFY');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_NTFY =
      NotificationProviderType._(
          7, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_NTFY');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_MATTERMOST =
      NotificationProviderType._(
          8, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_MATTERMOST');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_PUSHOVER =
      NotificationProviderType._(
          9, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_PUSHOVER');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_CUSTOM =
      NotificationProviderType._(
          10, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_CUSTOM');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_LARK =
      NotificationProviderType._(
          11, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_LARK');
  static const NotificationProviderType NOTIFICATION_PROVIDER_TYPE_TEAMS =
      NotificationProviderType._(
          12, _omitEnumNames ? '' : 'NOTIFICATION_PROVIDER_TYPE_TEAMS');

  static const $core.List<NotificationProviderType> values =
      <NotificationProviderType>[
    NOTIFICATION_PROVIDER_TYPE_UNSPECIFIED,
    NOTIFICATION_PROVIDER_TYPE_SLACK,
    NOTIFICATION_PROVIDER_TYPE_TELEGRAM,
    NOTIFICATION_PROVIDER_TYPE_DISCORD,
    NOTIFICATION_PROVIDER_TYPE_EMAIL,
    NOTIFICATION_PROVIDER_TYPE_RESEND,
    NOTIFICATION_PROVIDER_TYPE_GOTIFY,
    NOTIFICATION_PROVIDER_TYPE_NTFY,
    NOTIFICATION_PROVIDER_TYPE_MATTERMOST,
    NOTIFICATION_PROVIDER_TYPE_PUSHOVER,
    NOTIFICATION_PROVIDER_TYPE_CUSTOM,
    NOTIFICATION_PROVIDER_TYPE_LARK,
    NOTIFICATION_PROVIDER_TYPE_TEAMS,
  ];

  static final $core.List<NotificationProviderType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 12);
  static NotificationProviderType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const NotificationProviderType._(super.value, super.name);
}

class NotificationResourceType extends $pb.ProtobufEnum {
  static const NotificationResourceType NOTIFICATION_RESOURCE_TYPE_UNSPECIFIED =
      NotificationResourceType._(
          0, _omitEnumNames ? '' : 'NOTIFICATION_RESOURCE_TYPE_UNSPECIFIED');
  static const NotificationResourceType NOTIFICATION_RESOURCE_TYPE_SERVER =
      NotificationResourceType._(
          1, _omitEnumNames ? '' : 'NOTIFICATION_RESOURCE_TYPE_SERVER');
  static const NotificationResourceType NOTIFICATION_RESOURCE_TYPE_APPLICATION =
      NotificationResourceType._(
          2, _omitEnumNames ? '' : 'NOTIFICATION_RESOURCE_TYPE_APPLICATION');
  static const NotificationResourceType NOTIFICATION_RESOURCE_TYPE_COMPOSE =
      NotificationResourceType._(
          3, _omitEnumNames ? '' : 'NOTIFICATION_RESOURCE_TYPE_COMPOSE');
  static const NotificationResourceType NOTIFICATION_RESOURCE_TYPE_DATABASE =
      NotificationResourceType._(
          4, _omitEnumNames ? '' : 'NOTIFICATION_RESOURCE_TYPE_DATABASE');

  static const $core.List<NotificationResourceType> values =
      <NotificationResourceType>[
    NOTIFICATION_RESOURCE_TYPE_UNSPECIFIED,
    NOTIFICATION_RESOURCE_TYPE_SERVER,
    NOTIFICATION_RESOURCE_TYPE_APPLICATION,
    NOTIFICATION_RESOURCE_TYPE_COMPOSE,
    NOTIFICATION_RESOURCE_TYPE_DATABASE,
  ];

  static final $core.List<NotificationResourceType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static NotificationResourceType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const NotificationResourceType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/realtime.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Every user-facing SQLite table that can invalidate frontend queries; internal churn tables (auth tokens, schedule execution locks) are deliberately absent — they rotate too fast to be useful as signals.
class WatchedTable extends $pb.ProtobufEnum {
  static const WatchedTable WATCHED_TABLE_UNSPECIFIED =
      WatchedTable._(0, _omitEnumNames ? '' : 'WATCHED_TABLE_UNSPECIFIED');
  static const WatchedTable WATCHED_TABLE_AI_GENERATIONS =
      WatchedTable._(1, _omitEnumNames ? '' : 'WATCHED_TABLE_AI_GENERATIONS');
  static const WatchedTable WATCHED_TABLE_AI_SETTINGS =
      WatchedTable._(2, _omitEnumNames ? '' : 'WATCHED_TABLE_AI_SETTINGS');
  static const WatchedTable WATCHED_TABLE_ALERT_EVENTS =
      WatchedTable._(3, _omitEnumNames ? '' : 'WATCHED_TABLE_ALERT_EVENTS');
  static const WatchedTable WATCHED_TABLE_ALERT_RULES =
      WatchedTable._(4, _omitEnumNames ? '' : 'WATCHED_TABLE_ALERT_RULES');
  static const WatchedTable WATCHED_TABLE_APPLICATION_MIDDLEWARES =
      WatchedTable._(
          5, _omitEnumNames ? '' : 'WATCHED_TABLE_APPLICATION_MIDDLEWARES');
  static const WatchedTable WATCHED_TABLE_AUDIT_LOGS =
      WatchedTable._(6, _omitEnumNames ? '' : 'WATCHED_TABLE_AUDIT_LOGS');
  static const WatchedTable WATCHED_TABLE_BACKGROUND_POLICIES = WatchedTable._(
      7, _omitEnumNames ? '' : 'WATCHED_TABLE_BACKGROUND_POLICIES');
  static const WatchedTable WATCHED_TABLE_BACKUP_EXECUTIONS = WatchedTable._(
      8, _omitEnumNames ? '' : 'WATCHED_TABLE_BACKUP_EXECUTIONS');
  static const WatchedTable WATCHED_TABLE_BACKUPS =
      WatchedTable._(9, _omitEnumNames ? '' : 'WATCHED_TABLE_BACKUPS');
  static const WatchedTable WATCHED_TABLE_CERTIFICATE_RENEWALS = WatchedTable._(
      10, _omitEnumNames ? '' : 'WATCHED_TABLE_CERTIFICATE_RENEWALS');
  static const WatchedTable WATCHED_TABLE_CERTIFICATES =
      WatchedTable._(11, _omitEnumNames ? '' : 'WATCHED_TABLE_CERTIFICATES');
  static const WatchedTable WATCHED_TABLE_DATABASE_NETWORKS = WatchedTable._(
      12, _omitEnumNames ? '' : 'WATCHED_TABLE_DATABASE_NETWORKS');
  static const WatchedTable WATCHED_TABLE_DEPLOYMENTS =
      WatchedTable._(13, _omitEnumNames ? '' : 'WATCHED_TABLE_DEPLOYMENTS');
  static const WatchedTable WATCHED_TABLE_DESTINATIONS =
      WatchedTable._(14, _omitEnumNames ? '' : 'WATCHED_TABLE_DESTINATIONS');
  static const WatchedTable WATCHED_TABLE_DNS_PROVIDERS =
      WatchedTable._(15, _omitEnumNames ? '' : 'WATCHED_TABLE_DNS_PROVIDERS');
  static const WatchedTable WATCHED_TABLE_DOMAINS =
      WatchedTable._(16, _omitEnumNames ? '' : 'WATCHED_TABLE_DOMAINS');
  static const WatchedTable WATCHED_TABLE_ENVIRONMENTS =
      WatchedTable._(17, _omitEnumNames ? '' : 'WATCHED_TABLE_ENVIRONMENTS');
  static const WatchedTable WATCHED_TABLE_GIT_PROVIDERS =
      WatchedTable._(18, _omitEnumNames ? '' : 'WATCHED_TABLE_GIT_PROVIDERS');
  static const WatchedTable WATCHED_TABLE_GROUP_POLICY =
      WatchedTable._(19, _omitEnumNames ? '' : 'WATCHED_TABLE_GROUP_POLICY');
  static const WatchedTable WATCHED_TABLE_GROUPS =
      WatchedTable._(20, _omitEnumNames ? '' : 'WATCHED_TABLE_GROUPS');
  static const WatchedTable WATCHED_TABLE_HEALTH_REPORTS =
      WatchedTable._(21, _omitEnumNames ? '' : 'WATCHED_TABLE_HEALTH_REPORTS');
  static const WatchedTable WATCHED_TABLE_MONITORING_AGENTS = WatchedTable._(
      22, _omitEnumNames ? '' : 'WATCHED_TABLE_MONITORING_AGENTS');
  static const WatchedTable WATCHED_TABLE_MONITORING_MAINTENANCE_WINDOWS =
      WatchedTable._(23,
          _omitEnumNames ? '' : 'WATCHED_TABLE_MONITORING_MAINTENANCE_WINDOWS');
  static const WatchedTable WATCHED_TABLE_MONITORING_POLICY = WatchedTable._(
      24, _omitEnumNames ? '' : 'WATCHED_TABLE_MONITORING_POLICY');
  static const WatchedTable WATCHED_TABLE_MOUNTS =
      WatchedTable._(25, _omitEnumNames ? '' : 'WATCHED_TABLE_MOUNTS');
  static const WatchedTable WATCHED_TABLE_NOTIFICATION_DELIVERY_ATTEMPTS =
      WatchedTable._(26,
          _omitEnumNames ? '' : 'WATCHED_TABLE_NOTIFICATION_DELIVERY_ATTEMPTS');
  static const WatchedTable WATCHED_TABLE_NOTIFICATION_RESOURCE_BINDINGS =
      WatchedTable._(27,
          _omitEnumNames ? '' : 'WATCHED_TABLE_NOTIFICATION_RESOURCE_BINDINGS');
  static const WatchedTable WATCHED_TABLE_NOTIFICATIONS =
      WatchedTable._(28, _omitEnumNames ? '' : 'WATCHED_TABLE_NOTIFICATIONS');
  static const WatchedTable WATCHED_TABLE_ORGANIZATION =
      WatchedTable._(29, _omitEnumNames ? '' : 'WATCHED_TABLE_ORGANIZATION');
  static const WatchedTable WATCHED_TABLE_ORGANIZATION_INVITES = WatchedTable._(
      30, _omitEnumNames ? '' : 'WATCHED_TABLE_ORGANIZATION_INVITES');
  static const WatchedTable WATCHED_TABLE_ORGANIZATION_MEMBERS = WatchedTable._(
      31, _omitEnumNames ? '' : 'WATCHED_TABLE_ORGANIZATION_MEMBERS');
  static const WatchedTable WATCHED_TABLE_PATCHES =
      WatchedTable._(32, _omitEnumNames ? '' : 'WATCHED_TABLE_PATCHES');
  static const WatchedTable WATCHED_TABLE_POLICY =
      WatchedTable._(33, _omitEnumNames ? '' : 'WATCHED_TABLE_POLICY');
  static const WatchedTable WATCHED_TABLE_PORTS =
      WatchedTable._(34, _omitEnumNames ? '' : 'WATCHED_TABLE_PORTS');
  static const WatchedTable WATCHED_TABLE_PREVIEW_DEPLOYMENTS = WatchedTable._(
      35, _omitEnumNames ? '' : 'WATCHED_TABLE_PREVIEW_DEPLOYMENTS');
  static const WatchedTable WATCHED_TABLE_PROJECTS =
      WatchedTable._(36, _omitEnumNames ? '' : 'WATCHED_TABLE_PROJECTS');
  static const WatchedTable WATCHED_TABLE_PROJECT_TAGS =
      WatchedTable._(37, _omitEnumNames ? '' : 'WATCHED_TABLE_PROJECT_TAGS');
  static const WatchedTable WATCHED_TABLE_REDIRECTS =
      WatchedTable._(38, _omitEnumNames ? '' : 'WATCHED_TABLE_REDIRECTS');
  static const WatchedTable WATCHED_TABLE_REGISTRIES =
      WatchedTable._(39, _omitEnumNames ? '' : 'WATCHED_TABLE_REGISTRIES');
  static const WatchedTable WATCHED_TABLE_RESOURCE_ACCESS =
      WatchedTable._(40, _omitEnumNames ? '' : 'WATCHED_TABLE_RESOURCE_ACCESS');
  static const WatchedTable WATCHED_TABLE_ROLLBACKS =
      WatchedTable._(41, _omitEnumNames ? '' : 'WATCHED_TABLE_ROLLBACKS');
  static const WatchedTable WATCHED_TABLE_SCHEDULE_EXECUTIONS = WatchedTable._(
      42, _omitEnumNames ? '' : 'WATCHED_TABLE_SCHEDULE_EXECUTIONS');
  static const WatchedTable WATCHED_TABLE_SCHEDULE_RUNTIME_POLICIES =
      WatchedTable._(
          43, _omitEnumNames ? '' : 'WATCHED_TABLE_SCHEDULE_RUNTIME_POLICIES');
  static const WatchedTable WATCHED_TABLE_SCHEDULES =
      WatchedTable._(44, _omitEnumNames ? '' : 'WATCHED_TABLE_SCHEDULES');
  static const WatchedTable WATCHED_TABLE_SECURITY =
      WatchedTable._(45, _omitEnumNames ? '' : 'WATCHED_TABLE_SECURITY');
  static const WatchedTable WATCHED_TABLE_SERVER_CLEANUP_EXECUTIONS =
      WatchedTable._(
          46, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVER_CLEANUP_EXECUTIONS');
  static const WatchedTable WATCHED_TABLE_SERVER_MANAGEMENT = WatchedTable._(
      47, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVER_MANAGEMENT');
  static const WatchedTable WATCHED_TABLE_SERVER_MIGRATIONS = WatchedTable._(
      48, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVER_MIGRATIONS');
  static const WatchedTable WATCHED_TABLE_SERVER_PRIVATE_NETWORKS =
      WatchedTable._(
          49, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVER_PRIVATE_NETWORKS');
  static const WatchedTable WATCHED_TABLE_SERVERS =
      WatchedTable._(50, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVERS');
  static const WatchedTable WATCHED_TABLE_SERVICE_APPLICATION = WatchedTable._(
      51, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVICE_APPLICATION');
  static const WatchedTable WATCHED_TABLE_SERVICE_COMPOSE =
      WatchedTable._(52, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVICE_COMPOSE');
  static const WatchedTable WATCHED_TABLE_SERVICE_DATABASE = WatchedTable._(
      53, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVICE_DATABASE');
  static const WatchedTable WATCHED_TABLE_SERVICE_GIT =
      WatchedTable._(54, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVICE_GIT');
  static const WatchedTable WATCHED_TABLE_SERVICES =
      WatchedTable._(55, _omitEnumNames ? '' : 'WATCHED_TABLE_SERVICES');
  static const WatchedTable WATCHED_TABLE_SETTINGS =
      WatchedTable._(56, _omitEnumNames ? '' : 'WATCHED_TABLE_SETTINGS');
  static const WatchedTable WATCHED_TABLE_SSH_KEYS =
      WatchedTable._(57, _omitEnumNames ? '' : 'WATCHED_TABLE_SSH_KEYS');
  static const WatchedTable WATCHED_TABLE_TAGS =
      WatchedTable._(58, _omitEnumNames ? '' : 'WATCHED_TABLE_TAGS');
  static const WatchedTable WATCHED_TABLE_USER_POLICY =
      WatchedTable._(59, _omitEnumNames ? '' : 'WATCHED_TABLE_USER_POLICY');
  static const WatchedTable WATCHED_TABLE_USERS =
      WatchedTable._(60, _omitEnumNames ? '' : 'WATCHED_TABLE_USERS');
  static const WatchedTable WATCHED_TABLE_VAULT_PROVIDERS =
      WatchedTable._(61, _omitEnumNames ? '' : 'WATCHED_TABLE_VAULT_PROVIDERS');
  static const WatchedTable WATCHED_TABLE_VOLUME_BACKUPS =
      WatchedTable._(62, _omitEnumNames ? '' : 'WATCHED_TABLE_VOLUME_BACKUPS');
  static const WatchedTable WATCHED_TABLE_APPLICATION_MIGRATIONS =
      WatchedTable._(
          63, _omitEnumNames ? '' : 'WATCHED_TABLE_APPLICATION_MIGRATIONS');
  static const WatchedTable WATCHED_TABLE_MIGRATION_OBJECTS = WatchedTable._(
      64, _omitEnumNames ? '' : 'WATCHED_TABLE_MIGRATION_OBJECTS');

  static const $core.List<WatchedTable> values = <WatchedTable>[
    WATCHED_TABLE_UNSPECIFIED,
    WATCHED_TABLE_AI_GENERATIONS,
    WATCHED_TABLE_AI_SETTINGS,
    WATCHED_TABLE_ALERT_EVENTS,
    WATCHED_TABLE_ALERT_RULES,
    WATCHED_TABLE_APPLICATION_MIDDLEWARES,
    WATCHED_TABLE_AUDIT_LOGS,
    WATCHED_TABLE_BACKGROUND_POLICIES,
    WATCHED_TABLE_BACKUP_EXECUTIONS,
    WATCHED_TABLE_BACKUPS,
    WATCHED_TABLE_CERTIFICATE_RENEWALS,
    WATCHED_TABLE_CERTIFICATES,
    WATCHED_TABLE_DATABASE_NETWORKS,
    WATCHED_TABLE_DEPLOYMENTS,
    WATCHED_TABLE_DESTINATIONS,
    WATCHED_TABLE_DNS_PROVIDERS,
    WATCHED_TABLE_DOMAINS,
    WATCHED_TABLE_ENVIRONMENTS,
    WATCHED_TABLE_GIT_PROVIDERS,
    WATCHED_TABLE_GROUP_POLICY,
    WATCHED_TABLE_GROUPS,
    WATCHED_TABLE_HEALTH_REPORTS,
    WATCHED_TABLE_MONITORING_AGENTS,
    WATCHED_TABLE_MONITORING_MAINTENANCE_WINDOWS,
    WATCHED_TABLE_MONITORING_POLICY,
    WATCHED_TABLE_MOUNTS,
    WATCHED_TABLE_NOTIFICATION_DELIVERY_ATTEMPTS,
    WATCHED_TABLE_NOTIFICATION_RESOURCE_BINDINGS,
    WATCHED_TABLE_NOTIFICATIONS,
    WATCHED_TABLE_ORGANIZATION,
    WATCHED_TABLE_ORGANIZATION_INVITES,
    WATCHED_TABLE_ORGANIZATION_MEMBERS,
    WATCHED_TABLE_PATCHES,
    WATCHED_TABLE_POLICY,
    WATCHED_TABLE_PORTS,
    WATCHED_TABLE_PREVIEW_DEPLOYMENTS,
    WATCHED_TABLE_PROJECTS,
    WATCHED_TABLE_PROJECT_TAGS,
    WATCHED_TABLE_REDIRECTS,
    WATCHED_TABLE_REGISTRIES,
    WATCHED_TABLE_RESOURCE_ACCESS,
    WATCHED_TABLE_ROLLBACKS,
    WATCHED_TABLE_SCHEDULE_EXECUTIONS,
    WATCHED_TABLE_SCHEDULE_RUNTIME_POLICIES,
    WATCHED_TABLE_SCHEDULES,
    WATCHED_TABLE_SECURITY,
    WATCHED_TABLE_SERVER_CLEANUP_EXECUTIONS,
    WATCHED_TABLE_SERVER_MANAGEMENT,
    WATCHED_TABLE_SERVER_MIGRATIONS,
    WATCHED_TABLE_SERVER_PRIVATE_NETWORKS,
    WATCHED_TABLE_SERVERS,
    WATCHED_TABLE_SERVICE_APPLICATION,
    WATCHED_TABLE_SERVICE_COMPOSE,
    WATCHED_TABLE_SERVICE_DATABASE,
    WATCHED_TABLE_SERVICE_GIT,
    WATCHED_TABLE_SERVICES,
    WATCHED_TABLE_SETTINGS,
    WATCHED_TABLE_SSH_KEYS,
    WATCHED_TABLE_TAGS,
    WATCHED_TABLE_USER_POLICY,
    WATCHED_TABLE_USERS,
    WATCHED_TABLE_VAULT_PROVIDERS,
    WATCHED_TABLE_VOLUME_BACKUPS,
    WATCHED_TABLE_APPLICATION_MIGRATIONS,
    WATCHED_TABLE_MIGRATION_OBJECTS,
  ];

  static final $core.List<WatchedTable?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 64);
  static WatchedTable? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const WatchedTable._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

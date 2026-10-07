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

import 'realtime.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'realtime.pbenum.dart';

class WatchTablesEvent extends $pb.GeneratedMessage {
  factory WatchTablesEvent({
    $core.Iterable<WatchedTable>? tables,
  }) {
    final result = WatchTablesEvent._();
    if (tables != null) result.tables.addAll(tables);
    return result;
  }

  WatchTablesEvent._();

  factory WatchTablesEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      WatchTablesEvent()..mergeFromBuffer(data, registry);
  factory WatchTablesEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      WatchTablesEvent()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WatchTablesEvent',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: WatchTablesEvent.$_createMessage)
    ..pc<WatchedTable>(1, _omitFieldNames ? '' : 'tables', $pb.PbFieldType.KE,
        valueOf: WatchedTable.valueOf,
        enumValues: WatchedTable.values,
        defaultEnumValue: WatchedTable.WATCHED_TABLE_UNSPECIFIED)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WatchTablesEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WatchTablesEvent copyWith(void Function(WatchTablesEvent) updates) =>
      super.copyWith((message) => updates(message as WatchTablesEvent))
          as WatchTablesEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use WatchTablesEvent() / WatchTablesEvent.new instead')
  static WatchTablesEvent create() => WatchTablesEvent._();
  static $pb.GeneratedMessage $_createMessage() => WatchTablesEvent._();
  @$core.override
  WatchTablesEvent createEmptyInstance() => WatchTablesEvent._();
  @$core.pragma('dart2js:noInline')
  static WatchTablesEvent getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<WatchTablesEvent>(
          WatchTablesEvent.$_createMessage);
  static WatchTablesEvent? _defaultInstance;

  /// Tables whose rows changed since the previous event; an event listing all tables is a resync, so clients should treat every watched query as stale.
  @$pb.TagNumber(1)
  $pb.PbList<WatchedTable> get tables => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

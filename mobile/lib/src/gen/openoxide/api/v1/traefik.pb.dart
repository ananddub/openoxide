// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/traefik.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'traefik.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'traefik.pbenum.dart';

class TraefikFileNode extends $pb.GeneratedMessage {
  factory TraefikFileNode({
    $core.String? name,
    $core.String? relativePath,
    $fixnum.Int64? size,
    $core.bool? isReadonly,
    $fixnum.Int64? modifiedAt,
  }) {
    final result = TraefikFileNode._();
    if (name != null) result.name = name;
    if (relativePath != null) result.relativePath = relativePath;
    if (size != null) result.size = size;
    if (isReadonly != null) result.isReadonly = isReadonly;
    if (modifiedAt != null) result.modifiedAt = modifiedAt;
    return result;
  }

  TraefikFileNode._();

  factory TraefikFileNode.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileNode()..mergeFromBuffer(data, registry);
  factory TraefikFileNode.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileNode()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikFileNode',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikFileNode.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'relativePath')
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'size', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(4, _omitFieldNames ? '' : 'isReadonly')
    ..a<$fixnum.Int64>(
        5, _omitFieldNames ? '' : 'modifiedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileNode clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileNode copyWith(void Function(TraefikFileNode) updates) =>
      super.copyWith((message) => updates(message as TraefikFileNode))
          as TraefikFileNode;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikFileNode() / TraefikFileNode.new instead')
  static TraefikFileNode create() => TraefikFileNode._();
  static $pb.GeneratedMessage $_createMessage() => TraefikFileNode._();
  @$core.override
  TraefikFileNode createEmptyInstance() => TraefikFileNode._();
  @$core.pragma('dart2js:noInline')
  static TraefikFileNode getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraefikFileNode>(
          TraefikFileNode.$_createMessage);
  static TraefikFileNode? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// Path relative to the Traefik config base directory.
  @$pb.TagNumber(2)
  $core.String get relativePath => $_getSZ(1);
  @$pb.TagNumber(2)
  set relativePath($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRelativePath() => $_has(1);
  @$pb.TagNumber(2)
  void clearRelativePath() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get size => $_getI64(2);
  @$pb.TagNumber(3)
  set size($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearSize() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get isReadonly => $_getBF(3);
  @$pb.TagNumber(4)
  set isReadonly($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasIsReadonly() => $_has(3);
  @$pb.TagNumber(4)
  void clearIsReadonly() => $_clearField(4);

  /// Unix epoch seconds as reported by the filesystem.
  @$pb.TagNumber(5)
  $fixnum.Int64 get modifiedAt => $_getI64(4);
  @$pb.TagNumber(5)
  set modifiedAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasModifiedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearModifiedAt() => $_clearField(5);
}

class TraefikFileTreeNode extends $pb.GeneratedMessage {
  factory TraefikFileTreeNode({
    $core.String? name,
    $core.String? relativePath,
    TraefikFileNodeType? nodeType,
    $fixnum.Int64? size,
    $core.bool? isReadonly,
    $fixnum.Int64? modifiedAt,
    $core.Iterable<TraefikFileTreeNode>? children,
  }) {
    final result = TraefikFileTreeNode._();
    if (name != null) result.name = name;
    if (relativePath != null) result.relativePath = relativePath;
    if (nodeType != null) result.nodeType = nodeType;
    if (size != null) result.size = size;
    if (isReadonly != null) result.isReadonly = isReadonly;
    if (modifiedAt != null) result.modifiedAt = modifiedAt;
    if (children != null) result.children.addAll(children);
    return result;
  }

  TraefikFileTreeNode._();

  factory TraefikFileTreeNode.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileTreeNode()..mergeFromBuffer(data, registry);
  factory TraefikFileTreeNode.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileTreeNode()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikFileTreeNode',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikFileTreeNode.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'relativePath')
    ..aE<TraefikFileNodeType>(3, _omitFieldNames ? '' : 'nodeType',
        enumValues: TraefikFileNodeType.values)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'size', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(5, _omitFieldNames ? '' : 'isReadonly')
    ..a<$fixnum.Int64>(
        6, _omitFieldNames ? '' : 'modifiedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..pPM<TraefikFileTreeNode>(7, _omitFieldNames ? '' : 'children',
        subBuilder: TraefikFileTreeNode.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileTreeNode clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileTreeNode copyWith(void Function(TraefikFileTreeNode) updates) =>
      super.copyWith((message) => updates(message as TraefikFileTreeNode))
          as TraefikFileTreeNode;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use TraefikFileTreeNode() / TraefikFileTreeNode.new instead')
  static TraefikFileTreeNode create() => TraefikFileTreeNode._();
  static $pb.GeneratedMessage $_createMessage() => TraefikFileTreeNode._();
  @$core.override
  TraefikFileTreeNode createEmptyInstance() => TraefikFileTreeNode._();
  @$core.pragma('dart2js:noInline')
  static TraefikFileTreeNode getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikFileTreeNode>(
          TraefikFileTreeNode.$_createMessage);
  static TraefikFileTreeNode? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get relativePath => $_getSZ(1);
  @$pb.TagNumber(2)
  set relativePath($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRelativePath() => $_has(1);
  @$pb.TagNumber(2)
  void clearRelativePath() => $_clearField(2);

  @$pb.TagNumber(3)
  TraefikFileNodeType get nodeType => $_getN(2);
  @$pb.TagNumber(3)
  set nodeType(TraefikFileNodeType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasNodeType() => $_has(2);
  @$pb.TagNumber(3)
  void clearNodeType() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get size => $_getI64(3);
  @$pb.TagNumber(4)
  set size($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearSize() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isReadonly => $_getBF(4);
  @$pb.TagNumber(5)
  set isReadonly($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIsReadonly() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsReadonly() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get modifiedAt => $_getI64(5);
  @$pb.TagNumber(6)
  set modifiedAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasModifiedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearModifiedAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $pb.PbList<TraefikFileTreeNode> get children => $_getList(6);
}

class TraefikFileContent extends $pb.GeneratedMessage {
  factory TraefikFileContent({
    $core.String? path,
    $core.String? content,
    $core.bool? isReadonly,
  }) {
    final result = TraefikFileContent._();
    if (path != null) result.path = path;
    if (content != null) result.content = content;
    if (isReadonly != null) result.isReadonly = isReadonly;
    return result;
  }

  TraefikFileContent._();

  factory TraefikFileContent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileContent()..mergeFromBuffer(data, registry);
  factory TraefikFileContent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikFileContent()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikFileContent',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikFileContent.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..aOB(3, _omitFieldNames ? '' : 'isReadonly')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileContent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikFileContent copyWith(void Function(TraefikFileContent) updates) =>
      super.copyWith((message) => updates(message as TraefikFileContent))
          as TraefikFileContent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikFileContent() / TraefikFileContent.new instead')
  static TraefikFileContent create() => TraefikFileContent._();
  static $pb.GeneratedMessage $_createMessage() => TraefikFileContent._();
  @$core.override
  TraefikFileContent createEmptyInstance() => TraefikFileContent._();
  @$core.pragma('dart2js:noInline')
  static TraefikFileContent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikFileContent>(
          TraefikFileContent.$_createMessage);
  static TraefikFileContent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);

  /// Opaque Traefik config document (.yml/.yaml/.json); shape is file-specific.
  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get isReadonly => $_getBF(2);
  @$pb.TagNumber(3)
  set isReadonly($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasIsReadonly() => $_has(2);
  @$pb.TagNumber(3)
  void clearIsReadonly() => $_clearField(3);
}

class TraefikHealth extends $pb.GeneratedMessage {
  factory TraefikHealth({
    $core.bool? isHealthy,
    $core.String? rawdataStatus,
    $core.Iterable<$core.String>? configurationErrors,
  }) {
    final result = TraefikHealth._();
    if (isHealthy != null) result.isHealthy = isHealthy;
    if (rawdataStatus != null) result.rawdataStatus = rawdataStatus;
    if (configurationErrors != null)
      result.configurationErrors.addAll(configurationErrors);
    return result;
  }

  TraefikHealth._();

  factory TraefikHealth.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealth()..mergeFromBuffer(data, registry);
  factory TraefikHealth.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealth()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikHealth',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikHealth.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'isHealthy')
    ..aOS(2, _omitFieldNames ? '' : 'rawdataStatus')
    ..pPS(3, _omitFieldNames ? '' : 'configurationErrors')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealth clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealth copyWith(void Function(TraefikHealth) updates) =>
      super.copyWith((message) => updates(message as TraefikHealth))
          as TraefikHealth;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikHealth() / TraefikHealth.new instead')
  static TraefikHealth create() => TraefikHealth._();
  static $pb.GeneratedMessage $_createMessage() => TraefikHealth._();
  @$core.override
  TraefikHealth createEmptyInstance() => TraefikHealth._();
  @$core.pragma('dart2js:noInline')
  static TraefikHealth getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraefikHealth>(
          TraefikHealth.$_createMessage);
  static TraefikHealth? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get isHealthy => $_getBF(0);
  @$pb.TagNumber(1)
  set isHealthy($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIsHealthy() => $_has(0);
  @$pb.TagNumber(1)
  void clearIsHealthy() => $_clearField(1);

  /// Coarse rawdata endpoint state ("ok" or "unreachable").
  @$pb.TagNumber(2)
  $core.String get rawdataStatus => $_getSZ(1);
  @$pb.TagNumber(2)
  set rawdataStatus($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRawdataStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearRawdataStatus() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get configurationErrors => $_getList(2);
}

class TraefikRequestsStatus extends $pb.GeneratedMessage {
  factory TraefikRequestsStatus({
    $core.bool? isActive,
    $core.String? logPath,
    $core.String? cronExpression,
  }) {
    final result = TraefikRequestsStatus._();
    if (isActive != null) result.isActive = isActive;
    if (logPath != null) result.logPath = logPath;
    if (cronExpression != null) result.cronExpression = cronExpression;
    return result;
  }

  TraefikRequestsStatus._();

  factory TraefikRequestsStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRequestsStatus()..mergeFromBuffer(data, registry);
  factory TraefikRequestsStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRequestsStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikRequestsStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikRequestsStatus.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'isActive')
    ..aOS(2, _omitFieldNames ? '' : 'logPath')
    ..aOS(3, _omitFieldNames ? '' : 'cronExpression')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRequestsStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRequestsStatus copyWith(
          void Function(TraefikRequestsStatus) updates) =>
      super.copyWith((message) => updates(message as TraefikRequestsStatus))
          as TraefikRequestsStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikRequestsStatus() / TraefikRequestsStatus.new instead')
  static TraefikRequestsStatus create() => TraefikRequestsStatus._();
  static $pb.GeneratedMessage $_createMessage() => TraefikRequestsStatus._();
  @$core.override
  TraefikRequestsStatus createEmptyInstance() => TraefikRequestsStatus._();
  @$core.pragma('dart2js:noInline')
  static TraefikRequestsStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikRequestsStatus>(
          TraefikRequestsStatus.$_createMessage);
  static TraefikRequestsStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get isActive => $_getBF(0);
  @$pb.TagNumber(1)
  set isActive($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIsActive() => $_has(0);
  @$pb.TagNumber(1)
  void clearIsActive() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get logPath => $_getSZ(1);
  @$pb.TagNumber(2)
  set logPath($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLogPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearLogPath() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get cronExpression => $_getSZ(2);
  @$pb.TagNumber(3)
  set cronExpression($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCronExpression() => $_has(2);
  @$pb.TagNumber(3)
  void clearCronExpression() => $_clearField(3);
}

class TraefikLogEntry extends $pb.GeneratedMessage {
  factory TraefikLogEntry({
    $core.String? timestamp,
    $core.String? clientIp,
    $core.String? method,
    $core.String? path,
    $core.int? status,
    $core.double? durationMs,
    $core.String? serviceName,
    $core.String? routerName,
  }) {
    final result = TraefikLogEntry._();
    if (timestamp != null) result.timestamp = timestamp;
    if (clientIp != null) result.clientIp = clientIp;
    if (method != null) result.method = method;
    if (path != null) result.path = path;
    if (status != null) result.status = status;
    if (durationMs != null) result.durationMs = durationMs;
    if (serviceName != null) result.serviceName = serviceName;
    if (routerName != null) result.routerName = routerName;
    return result;
  }

  TraefikLogEntry._();

  factory TraefikLogEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikLogEntry()..mergeFromBuffer(data, registry);
  factory TraefikLogEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikLogEntry()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikLogEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikLogEntry.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'timestamp')
    ..aOS(2, _omitFieldNames ? '' : 'clientIp')
    ..aOS(3, _omitFieldNames ? '' : 'method')
    ..aOS(4, _omitFieldNames ? '' : 'path')
    ..aI(5, _omitFieldNames ? '' : 'status', fieldType: $pb.PbFieldType.OU3)
    ..aD(6, _omitFieldNames ? '' : 'durationMs')
    ..aOS(7, _omitFieldNames ? '' : 'serviceName')
    ..aOS(8, _omitFieldNames ? '' : 'routerName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikLogEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikLogEntry copyWith(void Function(TraefikLogEntry) updates) =>
      super.copyWith((message) => updates(message as TraefikLogEntry))
          as TraefikLogEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikLogEntry() / TraefikLogEntry.new instead')
  static TraefikLogEntry create() => TraefikLogEntry._();
  static $pb.GeneratedMessage $_createMessage() => TraefikLogEntry._();
  @$core.override
  TraefikLogEntry createEmptyInstance() => TraefikLogEntry._();
  @$core.pragma('dart2js:noInline')
  static TraefikLogEntry getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraefikLogEntry>(
          TraefikLogEntry.$_createMessage);
  static TraefikLogEntry? _defaultInstance;

  /// Timestamp string as emitted by the Traefik access log (RFC 3339).
  @$pb.TagNumber(1)
  $core.String get timestamp => $_getSZ(0);
  @$pb.TagNumber(1)
  set timestamp($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get clientIp => $_getSZ(1);
  @$pb.TagNumber(2)
  set clientIp($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasClientIp() => $_has(1);
  @$pb.TagNumber(2)
  void clearClientIp() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get method => $_getSZ(2);
  @$pb.TagNumber(3)
  set method($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMethod() => $_has(2);
  @$pb.TagNumber(3)
  void clearMethod() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get path => $_getSZ(3);
  @$pb.TagNumber(4)
  set path($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearPath() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get status => $_getIZ(4);
  @$pb.TagNumber(5)
  set status($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(5)
  void clearStatus() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.double get durationMs => $_getN(5);
  @$pb.TagNumber(6)
  set durationMs($core.double value) => $_setDouble(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDurationMs() => $_has(5);
  @$pb.TagNumber(6)
  void clearDurationMs() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get serviceName => $_getSZ(6);
  @$pb.TagNumber(7)
  set serviceName($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasServiceName() => $_has(6);
  @$pb.TagNumber(7)
  void clearServiceName() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get routerName => $_getSZ(7);
  @$pb.TagNumber(8)
  set routerName($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasRouterName() => $_has(7);
  @$pb.TagNumber(8)
  void clearRouterName() => $_clearField(8);
}

class TraefikVersion extends $pb.GeneratedMessage {
  factory TraefikVersion({
    $fixnum.Int64? serverId,
    $core.String? currentImage,
    $core.String? desiredVersion,
  }) {
    final result = TraefikVersion._();
    if (serverId != null) result.serverId = serverId;
    if (currentImage != null) result.currentImage = currentImage;
    if (desiredVersion != null) result.desiredVersion = desiredVersion;
    return result;
  }

  TraefikVersion._();

  factory TraefikVersion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikVersion()..mergeFromBuffer(data, registry);
  factory TraefikVersion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikVersion()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikVersion',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikVersion.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'currentImage')
    ..aOS(3, _omitFieldNames ? '' : 'desiredVersion')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikVersion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikVersion copyWith(void Function(TraefikVersion) updates) =>
      super.copyWith((message) => updates(message as TraefikVersion))
          as TraefikVersion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikVersion() / TraefikVersion.new instead')
  static TraefikVersion create() => TraefikVersion._();
  static $pb.GeneratedMessage $_createMessage() => TraefikVersion._();
  @$core.override
  TraefikVersion createEmptyInstance() => TraefikVersion._();
  @$core.pragma('dart2js:noInline')
  static TraefikVersion getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraefikVersion>(
          TraefikVersion.$_createMessage);
  static TraefikVersion? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get currentImage => $_getSZ(1);
  @$pb.TagNumber(2)
  set currentImage($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCurrentImage() => $_has(1);
  @$pb.TagNumber(2)
  void clearCurrentImage() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get desiredVersion => $_getSZ(2);
  @$pb.TagNumber(3)
  set desiredVersion($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDesiredVersion() => $_has(2);
  @$pb.TagNumber(3)
  void clearDesiredVersion() => $_clearField(3);
}

class TraefikMiddleware extends $pb.GeneratedMessage {
  factory TraefikMiddleware({
    $core.String? reference,
    $core.Iterable<$core.MapEntry<$core.String, $core.String>>? labels,
  }) {
    final result = TraefikMiddleware._();
    if (reference != null) result.reference = reference;
    if (labels != null) result.labels.addEntries(labels);
    return result;
  }

  TraefikMiddleware._();

  factory TraefikMiddleware.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikMiddleware()..mergeFromBuffer(data, registry);
  factory TraefikMiddleware.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikMiddleware()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikMiddleware',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikMiddleware.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'reference')
    ..m<$core.String, $core.String>(2, _omitFieldNames ? '' : 'labels',
        entryClassName: 'TraefikMiddleware.LabelsEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OS,
        packageName: const $pb.PackageName('openoxide.api.v1'))
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikMiddleware clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikMiddleware copyWith(void Function(TraefikMiddleware) updates) =>
      super.copyWith((message) => updates(message as TraefikMiddleware))
          as TraefikMiddleware;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TraefikMiddleware() / TraefikMiddleware.new instead')
  static TraefikMiddleware create() => TraefikMiddleware._();
  static $pb.GeneratedMessage $_createMessage() => TraefikMiddleware._();
  @$core.override
  TraefikMiddleware createEmptyInstance() => TraefikMiddleware._();
  @$core.pragma('dart2js:noInline')
  static TraefikMiddleware getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraefikMiddleware>(
          TraefikMiddleware.$_createMessage);
  static TraefikMiddleware? _defaultInstance;

  /// Docker-label middleware reference, e.g. `openoxide-auth@docker`.
  @$pb.TagNumber(1)
  $core.String get reference => $_getSZ(0);
  @$pb.TagNumber(1)
  set reference($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasReference() => $_has(0);
  @$pb.TagNumber(1)
  void clearReference() => $_clearField(1);

  /// Rendered Traefik Docker labels; values are opaque YAML fragments.
  @$pb.TagNumber(2)
  $pb.PbMap<$core.String, $core.String> get labels => $_getMap(1);
}

class TraefikListFilesRequest extends $pb.GeneratedMessage {
  factory TraefikListFilesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = TraefikListFilesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  TraefikListFilesRequest._();

  factory TraefikListFilesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikListFilesRequest()..mergeFromBuffer(data, registry);
  factory TraefikListFilesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikListFilesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikListFilesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikListFilesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikListFilesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikListFilesRequest copyWith(
          void Function(TraefikListFilesRequest) updates) =>
      super.copyWith((message) => updates(message as TraefikListFilesRequest))
          as TraefikListFilesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikListFilesRequest() / TraefikListFilesRequest.new instead')
  static TraefikListFilesRequest create() => TraefikListFilesRequest._();
  static $pb.GeneratedMessage $_createMessage() => TraefikListFilesRequest._();
  @$core.override
  TraefikListFilesRequest createEmptyInstance() => TraefikListFilesRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikListFilesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikListFilesRequest>(
          TraefikListFilesRequest.$_createMessage);
  static TraefikListFilesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class TraefikListFilesResponse extends $pb.GeneratedMessage {
  factory TraefikListFilesResponse({
    $core.Iterable<TraefikFileNode>? files,
  }) {
    final result = TraefikListFilesResponse._();
    if (files != null) result.files.addAll(files);
    return result;
  }

  TraefikListFilesResponse._();

  factory TraefikListFilesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikListFilesResponse()..mergeFromBuffer(data, registry);
  factory TraefikListFilesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikListFilesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikListFilesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikListFilesResponse.$_createMessage)
    ..pPM<TraefikFileNode>(1, _omitFieldNames ? '' : 'files',
        subBuilder: TraefikFileNode.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikListFilesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikListFilesResponse copyWith(
          void Function(TraefikListFilesResponse) updates) =>
      super.copyWith((message) => updates(message as TraefikListFilesResponse))
          as TraefikListFilesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikListFilesResponse() / TraefikListFilesResponse.new instead')
  static TraefikListFilesResponse create() => TraefikListFilesResponse._();
  static $pb.GeneratedMessage $_createMessage() => TraefikListFilesResponse._();
  @$core.override
  TraefikListFilesResponse createEmptyInstance() =>
      TraefikListFilesResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikListFilesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikListFilesResponse>(
          TraefikListFilesResponse.$_createMessage);
  static TraefikListFilesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<TraefikFileNode> get files => $_getList(0);
}

class TraefikGetFileTreeRequest extends $pb.GeneratedMessage {
  factory TraefikGetFileTreeRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = TraefikGetFileTreeRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  TraefikGetFileTreeRequest._();

  factory TraefikGetFileTreeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileTreeRequest()..mergeFromBuffer(data, registry);
  factory TraefikGetFileTreeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileTreeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetFileTreeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetFileTreeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileTreeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileTreeRequest copyWith(
          void Function(TraefikGetFileTreeRequest) updates) =>
      super.copyWith((message) => updates(message as TraefikGetFileTreeRequest))
          as TraefikGetFileTreeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetFileTreeRequest() / TraefikGetFileTreeRequest.new instead')
  static TraefikGetFileTreeRequest create() => TraefikGetFileTreeRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetFileTreeRequest._();
  @$core.override
  TraefikGetFileTreeRequest createEmptyInstance() =>
      TraefikGetFileTreeRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetFileTreeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetFileTreeRequest>(
          TraefikGetFileTreeRequest.$_createMessage);
  static TraefikGetFileTreeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class TraefikGetFileTreeResponse extends $pb.GeneratedMessage {
  factory TraefikGetFileTreeResponse({
    $core.Iterable<TraefikFileTreeNode>? nodes,
  }) {
    final result = TraefikGetFileTreeResponse._();
    if (nodes != null) result.nodes.addAll(nodes);
    return result;
  }

  TraefikGetFileTreeResponse._();

  factory TraefikGetFileTreeResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileTreeResponse()..mergeFromBuffer(data, registry);
  factory TraefikGetFileTreeResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileTreeResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetFileTreeResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetFileTreeResponse.$_createMessage)
    ..pPM<TraefikFileTreeNode>(1, _omitFieldNames ? '' : 'nodes',
        subBuilder: TraefikFileTreeNode.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileTreeResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileTreeResponse copyWith(
          void Function(TraefikGetFileTreeResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetFileTreeResponse))
          as TraefikGetFileTreeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetFileTreeResponse() / TraefikGetFileTreeResponse.new instead')
  static TraefikGetFileTreeResponse create() => TraefikGetFileTreeResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetFileTreeResponse._();
  @$core.override
  TraefikGetFileTreeResponse createEmptyInstance() =>
      TraefikGetFileTreeResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetFileTreeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetFileTreeResponse>(
          TraefikGetFileTreeResponse.$_createMessage);
  static TraefikGetFileTreeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<TraefikFileTreeNode> get nodes => $_getList(0);
}

class TraefikGetFileContentRequest extends $pb.GeneratedMessage {
  factory TraefikGetFileContentRequest({
    $fixnum.Int64? serverId,
    $core.String? path,
  }) {
    final result = TraefikGetFileContentRequest._();
    if (serverId != null) result.serverId = serverId;
    if (path != null) result.path = path;
    return result;
  }

  TraefikGetFileContentRequest._();

  factory TraefikGetFileContentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileContentRequest()..mergeFromBuffer(data, registry);
  factory TraefikGetFileContentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileContentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetFileContentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetFileContentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileContentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileContentRequest copyWith(
          void Function(TraefikGetFileContentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetFileContentRequest))
          as TraefikGetFileContentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetFileContentRequest() / TraefikGetFileContentRequest.new instead')
  static TraefikGetFileContentRequest create() =>
      TraefikGetFileContentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetFileContentRequest._();
  @$core.override
  TraefikGetFileContentRequest createEmptyInstance() =>
      TraefikGetFileContentRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetFileContentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetFileContentRequest>(
          TraefikGetFileContentRequest.$_createMessage);
  static TraefikGetFileContentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);
}

class TraefikGetFileContentResponse extends $pb.GeneratedMessage {
  factory TraefikGetFileContentResponse({
    TraefikFileContent? file,
  }) {
    final result = TraefikGetFileContentResponse._();
    if (file != null) result.file = file;
    return result;
  }

  TraefikGetFileContentResponse._();

  factory TraefikGetFileContentResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileContentResponse()..mergeFromBuffer(data, registry);
  factory TraefikGetFileContentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetFileContentResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetFileContentResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetFileContentResponse.$_createMessage)
    ..aOM<TraefikFileContent>(1, _omitFieldNames ? '' : 'file',
        subBuilder: TraefikFileContent.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileContentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetFileContentResponse copyWith(
          void Function(TraefikGetFileContentResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetFileContentResponse))
          as TraefikGetFileContentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetFileContentResponse() / TraefikGetFileContentResponse.new instead')
  static TraefikGetFileContentResponse create() =>
      TraefikGetFileContentResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetFileContentResponse._();
  @$core.override
  TraefikGetFileContentResponse createEmptyInstance() =>
      TraefikGetFileContentResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetFileContentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetFileContentResponse>(
          TraefikGetFileContentResponse.$_createMessage);
  static TraefikGetFileContentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikFileContent get file => $_getN(0);
  @$pb.TagNumber(1)
  set file(TraefikFileContent value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasFile() => $_has(0);
  @$pb.TagNumber(1)
  void clearFile() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikFileContent ensureFile() => $_ensure(0);
}

class TraefikUpdateFileContentRequest extends $pb.GeneratedMessage {
  factory TraefikUpdateFileContentRequest({
    $fixnum.Int64? serverId,
    $core.String? path,
    $core.String? content,
  }) {
    final result = TraefikUpdateFileContentRequest._();
    if (serverId != null) result.serverId = serverId;
    if (path != null) result.path = path;
    if (content != null) result.content = content;
    return result;
  }

  TraefikUpdateFileContentRequest._();

  factory TraefikUpdateFileContentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateFileContentRequest()..mergeFromBuffer(data, registry);
  factory TraefikUpdateFileContentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateFileContentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikUpdateFileContentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikUpdateFileContentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..aOS(3, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateFileContentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateFileContentRequest copyWith(
          void Function(TraefikUpdateFileContentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikUpdateFileContentRequest))
          as TraefikUpdateFileContentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikUpdateFileContentRequest() / TraefikUpdateFileContentRequest.new instead')
  static TraefikUpdateFileContentRequest create() =>
      TraefikUpdateFileContentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikUpdateFileContentRequest._();
  @$core.override
  TraefikUpdateFileContentRequest createEmptyInstance() =>
      TraefikUpdateFileContentRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikUpdateFileContentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikUpdateFileContentRequest>(
          TraefikUpdateFileContentRequest.$_createMessage);
  static TraefikUpdateFileContentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);

  /// Full replacement document; validated as a YAML mapping server-side.
  @$pb.TagNumber(3)
  $core.String get content => $_getSZ(2);
  @$pb.TagNumber(3)
  set content($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearContent() => $_clearField(3);
}

class TraefikHealthRequest extends $pb.GeneratedMessage {
  factory TraefikHealthRequest() => TraefikHealthRequest._();

  TraefikHealthRequest._();

  factory TraefikHealthRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealthRequest()..mergeFromBuffer(data, registry);
  factory TraefikHealthRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealthRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikHealthRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikHealthRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealthRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealthRequest copyWith(void Function(TraefikHealthRequest) updates) =>
      super.copyWith((message) => updates(message as TraefikHealthRequest))
          as TraefikHealthRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikHealthRequest() / TraefikHealthRequest.new instead')
  static TraefikHealthRequest create() => TraefikHealthRequest._();
  static $pb.GeneratedMessage $_createMessage() => TraefikHealthRequest._();
  @$core.override
  TraefikHealthRequest createEmptyInstance() => TraefikHealthRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikHealthRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikHealthRequest>(
          TraefikHealthRequest.$_createMessage);
  static TraefikHealthRequest? _defaultInstance;
}

class TraefikHealthResponse extends $pb.GeneratedMessage {
  factory TraefikHealthResponse({
    TraefikHealth? health,
  }) {
    final result = TraefikHealthResponse._();
    if (health != null) result.health = health;
    return result;
  }

  TraefikHealthResponse._();

  factory TraefikHealthResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealthResponse()..mergeFromBuffer(data, registry);
  factory TraefikHealthResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikHealthResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikHealthResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikHealthResponse.$_createMessage)
    ..aOM<TraefikHealth>(1, _omitFieldNames ? '' : 'health',
        subBuilder: TraefikHealth.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealthResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikHealthResponse copyWith(
          void Function(TraefikHealthResponse) updates) =>
      super.copyWith((message) => updates(message as TraefikHealthResponse))
          as TraefikHealthResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikHealthResponse() / TraefikHealthResponse.new instead')
  static TraefikHealthResponse create() => TraefikHealthResponse._();
  static $pb.GeneratedMessage $_createMessage() => TraefikHealthResponse._();
  @$core.override
  TraefikHealthResponse createEmptyInstance() => TraefikHealthResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikHealthResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikHealthResponse>(
          TraefikHealthResponse.$_createMessage);
  static TraefikHealthResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikHealth get health => $_getN(0);
  @$pb.TagNumber(1)
  set health(TraefikHealth value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHealth() => $_has(0);
  @$pb.TagNumber(1)
  void clearHealth() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikHealth ensureHealth() => $_ensure(0);
}

class TraefikGetRequestStatusRequest extends $pb.GeneratedMessage {
  factory TraefikGetRequestStatusRequest() =>
      TraefikGetRequestStatusRequest._();

  TraefikGetRequestStatusRequest._();

  factory TraefikGetRequestStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestStatusRequest()..mergeFromBuffer(data, registry);
  factory TraefikGetRequestStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetRequestStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetRequestStatusRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestStatusRequest copyWith(
          void Function(TraefikGetRequestStatusRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetRequestStatusRequest))
          as TraefikGetRequestStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetRequestStatusRequest() / TraefikGetRequestStatusRequest.new instead')
  static TraefikGetRequestStatusRequest create() =>
      TraefikGetRequestStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetRequestStatusRequest._();
  @$core.override
  TraefikGetRequestStatusRequest createEmptyInstance() =>
      TraefikGetRequestStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetRequestStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetRequestStatusRequest>(
          TraefikGetRequestStatusRequest.$_createMessage);
  static TraefikGetRequestStatusRequest? _defaultInstance;
}

class TraefikGetRequestStatusResponse extends $pb.GeneratedMessage {
  factory TraefikGetRequestStatusResponse({
    TraefikRequestsStatus? status,
  }) {
    final result = TraefikGetRequestStatusResponse._();
    if (status != null) result.status = status;
    return result;
  }

  TraefikGetRequestStatusResponse._();

  factory TraefikGetRequestStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestStatusResponse()..mergeFromBuffer(data, registry);
  factory TraefikGetRequestStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestStatusResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetRequestStatusResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetRequestStatusResponse.$_createMessage)
    ..aOM<TraefikRequestsStatus>(1, _omitFieldNames ? '' : 'status',
        subBuilder: TraefikRequestsStatus.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestStatusResponse copyWith(
          void Function(TraefikGetRequestStatusResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetRequestStatusResponse))
          as TraefikGetRequestStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetRequestStatusResponse() / TraefikGetRequestStatusResponse.new instead')
  static TraefikGetRequestStatusResponse create() =>
      TraefikGetRequestStatusResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetRequestStatusResponse._();
  @$core.override
  TraefikGetRequestStatusResponse createEmptyInstance() =>
      TraefikGetRequestStatusResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetRequestStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetRequestStatusResponse>(
          TraefikGetRequestStatusResponse.$_createMessage);
  static TraefikGetRequestStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikRequestsStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(TraefikRequestsStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikRequestsStatus ensureStatus() => $_ensure(0);
}

class TraefikToggleRequestsRequest extends $pb.GeneratedMessage {
  factory TraefikToggleRequestsRequest({
    $fixnum.Int64? serverId,
    $core.bool? enable,
  }) {
    final result = TraefikToggleRequestsRequest._();
    if (serverId != null) result.serverId = serverId;
    if (enable != null) result.enable = enable;
    return result;
  }

  TraefikToggleRequestsRequest._();

  factory TraefikToggleRequestsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikToggleRequestsRequest()..mergeFromBuffer(data, registry);
  factory TraefikToggleRequestsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikToggleRequestsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikToggleRequestsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikToggleRequestsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOB(2, _omitFieldNames ? '' : 'enable')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikToggleRequestsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikToggleRequestsRequest copyWith(
          void Function(TraefikToggleRequestsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikToggleRequestsRequest))
          as TraefikToggleRequestsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikToggleRequestsRequest() / TraefikToggleRequestsRequest.new instead')
  static TraefikToggleRequestsRequest create() =>
      TraefikToggleRequestsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikToggleRequestsRequest._();
  @$core.override
  TraefikToggleRequestsRequest createEmptyInstance() =>
      TraefikToggleRequestsRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikToggleRequestsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikToggleRequestsRequest>(
          TraefikToggleRequestsRequest.$_createMessage);
  static TraefikToggleRequestsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get enable => $_getBF(1);
  @$pb.TagNumber(2)
  set enable($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnable() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnable() => $_clearField(2);
}

class TraefikToggleRequestsResponse extends $pb.GeneratedMessage {
  factory TraefikToggleRequestsResponse({
    $core.bool? active,
  }) {
    final result = TraefikToggleRequestsResponse._();
    if (active != null) result.active = active;
    return result;
  }

  TraefikToggleRequestsResponse._();

  factory TraefikToggleRequestsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikToggleRequestsResponse()..mergeFromBuffer(data, registry);
  factory TraefikToggleRequestsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikToggleRequestsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikToggleRequestsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikToggleRequestsResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'active')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikToggleRequestsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikToggleRequestsResponse copyWith(
          void Function(TraefikToggleRequestsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikToggleRequestsResponse))
          as TraefikToggleRequestsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikToggleRequestsResponse() / TraefikToggleRequestsResponse.new instead')
  static TraefikToggleRequestsResponse create() =>
      TraefikToggleRequestsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikToggleRequestsResponse._();
  @$core.override
  TraefikToggleRequestsResponse createEmptyInstance() =>
      TraefikToggleRequestsResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikToggleRequestsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikToggleRequestsResponse>(
          TraefikToggleRequestsResponse.$_createMessage);
  static TraefikToggleRequestsResponse? _defaultInstance;

  /// Whether request logging is active after the toggle.
  @$pb.TagNumber(1)
  $core.bool get active => $_getBF(0);
  @$pb.TagNumber(1)
  set active($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasActive() => $_has(0);
  @$pb.TagNumber(1)
  void clearActive() => $_clearField(1);
}

class TraefikGetRequestLogsRequest extends $pb.GeneratedMessage {
  factory TraefikGetRequestLogsRequest({
    $fixnum.Int64? serverId,
    $fixnum.Int64? page,
    $fixnum.Int64? pageSize,
    $core.String? search,
    $core.int? status,
  }) {
    final result = TraefikGetRequestLogsRequest._();
    if (serverId != null) result.serverId = serverId;
    if (page != null) result.page = page;
    if (pageSize != null) result.pageSize = pageSize;
    if (search != null) result.search = search;
    if (status != null) result.status = status;
    return result;
  }

  TraefikGetRequestLogsRequest._();

  factory TraefikGetRequestLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestLogsRequest()..mergeFromBuffer(data, registry);
  factory TraefikGetRequestLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestLogsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetRequestLogsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetRequestLogsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'page')
    ..aInt64(3, _omitFieldNames ? '' : 'pageSize')
    ..aOS(4, _omitFieldNames ? '' : 'search')
    ..aI(5, _omitFieldNames ? '' : 'status', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestLogsRequest copyWith(
          void Function(TraefikGetRequestLogsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetRequestLogsRequest))
          as TraefikGetRequestLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetRequestLogsRequest() / TraefikGetRequestLogsRequest.new instead')
  static TraefikGetRequestLogsRequest create() =>
      TraefikGetRequestLogsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetRequestLogsRequest._();
  @$core.override
  TraefikGetRequestLogsRequest createEmptyInstance() =>
      TraefikGetRequestLogsRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetRequestLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetRequestLogsRequest>(
          TraefikGetRequestLogsRequest.$_createMessage);
  static TraefikGetRequestLogsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get page => $_getI64(1);
  @$pb.TagNumber(2)
  set page($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(2)
  void clearPage() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get pageSize => $_getI64(2);
  @$pb.TagNumber(3)
  set pageSize($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPageSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearPageSize() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get search => $_getSZ(3);
  @$pb.TagNumber(4)
  set search($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSearch() => $_has(3);
  @$pb.TagNumber(4)
  void clearSearch() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get status => $_getIZ(4);
  @$pb.TagNumber(5)
  set status($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(5)
  void clearStatus() => $_clearField(5);
}

class TraefikGetRequestLogsResponse extends $pb.GeneratedMessage {
  factory TraefikGetRequestLogsResponse({
    $core.Iterable<TraefikLogEntry>? entries,
    $fixnum.Int64? totalCount,
    $fixnum.Int64? page,
    $fixnum.Int64? pageSize,
  }) {
    final result = TraefikGetRequestLogsResponse._();
    if (entries != null) result.entries.addAll(entries);
    if (totalCount != null) result.totalCount = totalCount;
    if (page != null) result.page = page;
    if (pageSize != null) result.pageSize = pageSize;
    return result;
  }

  TraefikGetRequestLogsResponse._();

  factory TraefikGetRequestLogsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestLogsResponse()..mergeFromBuffer(data, registry);
  factory TraefikGetRequestLogsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetRequestLogsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetRequestLogsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetRequestLogsResponse.$_createMessage)
    ..pPM<TraefikLogEntry>(1, _omitFieldNames ? '' : 'entries',
        subBuilder: TraefikLogEntry.$_createMessage)
    ..aInt64(2, _omitFieldNames ? '' : 'totalCount')
    ..aInt64(3, _omitFieldNames ? '' : 'page')
    ..aInt64(4, _omitFieldNames ? '' : 'pageSize')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestLogsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetRequestLogsResponse copyWith(
          void Function(TraefikGetRequestLogsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikGetRequestLogsResponse))
          as TraefikGetRequestLogsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetRequestLogsResponse() / TraefikGetRequestLogsResponse.new instead')
  static TraefikGetRequestLogsResponse create() =>
      TraefikGetRequestLogsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetRequestLogsResponse._();
  @$core.override
  TraefikGetRequestLogsResponse createEmptyInstance() =>
      TraefikGetRequestLogsResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetRequestLogsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetRequestLogsResponse>(
          TraefikGetRequestLogsResponse.$_createMessage);
  static TraefikGetRequestLogsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<TraefikLogEntry> get entries => $_getList(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalCount => $_getI64(1);
  @$pb.TagNumber(2)
  set totalCount($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTotalCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalCount() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get page => $_getI64(2);
  @$pb.TagNumber(3)
  set page($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPage() => $_has(2);
  @$pb.TagNumber(3)
  void clearPage() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get pageSize => $_getI64(3);
  @$pb.TagNumber(4)
  set pageSize($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPageSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearPageSize() => $_clearField(4);
}

class TraefikGetVersionRequest extends $pb.GeneratedMessage {
  factory TraefikGetVersionRequest() => TraefikGetVersionRequest._();

  TraefikGetVersionRequest._();

  factory TraefikGetVersionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetVersionRequest()..mergeFromBuffer(data, registry);
  factory TraefikGetVersionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetVersionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetVersionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetVersionRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetVersionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetVersionRequest copyWith(
          void Function(TraefikGetVersionRequest) updates) =>
      super.copyWith((message) => updates(message as TraefikGetVersionRequest))
          as TraefikGetVersionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetVersionRequest() / TraefikGetVersionRequest.new instead')
  static TraefikGetVersionRequest create() => TraefikGetVersionRequest._();
  static $pb.GeneratedMessage $_createMessage() => TraefikGetVersionRequest._();
  @$core.override
  TraefikGetVersionRequest createEmptyInstance() =>
      TraefikGetVersionRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetVersionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetVersionRequest>(
          TraefikGetVersionRequest.$_createMessage);
  static TraefikGetVersionRequest? _defaultInstance;
}

class TraefikGetVersionResponse extends $pb.GeneratedMessage {
  factory TraefikGetVersionResponse({
    TraefikVersion? version,
  }) {
    final result = TraefikGetVersionResponse._();
    if (version != null) result.version = version;
    return result;
  }

  TraefikGetVersionResponse._();

  factory TraefikGetVersionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetVersionResponse()..mergeFromBuffer(data, registry);
  factory TraefikGetVersionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikGetVersionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikGetVersionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikGetVersionResponse.$_createMessage)
    ..aOM<TraefikVersion>(1, _omitFieldNames ? '' : 'version',
        subBuilder: TraefikVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetVersionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikGetVersionResponse copyWith(
          void Function(TraefikGetVersionResponse) updates) =>
      super.copyWith((message) => updates(message as TraefikGetVersionResponse))
          as TraefikGetVersionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikGetVersionResponse() / TraefikGetVersionResponse.new instead')
  static TraefikGetVersionResponse create() => TraefikGetVersionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikGetVersionResponse._();
  @$core.override
  TraefikGetVersionResponse createEmptyInstance() =>
      TraefikGetVersionResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikGetVersionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikGetVersionResponse>(
          TraefikGetVersionResponse.$_createMessage);
  static TraefikGetVersionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikVersion get version => $_getN(0);
  @$pb.TagNumber(1)
  set version(TraefikVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearVersion() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikVersion ensureVersion() => $_ensure(0);
}

class TraefikUpdateVersionRequest extends $pb.GeneratedMessage {
  factory TraefikUpdateVersionRequest({
    $fixnum.Int64? serverId,
    $core.String? version,
  }) {
    final result = TraefikUpdateVersionRequest._();
    if (serverId != null) result.serverId = serverId;
    if (version != null) result.version = version;
    return result;
  }

  TraefikUpdateVersionRequest._();

  factory TraefikUpdateVersionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateVersionRequest()..mergeFromBuffer(data, registry);
  factory TraefikUpdateVersionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateVersionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikUpdateVersionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikUpdateVersionRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateVersionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateVersionRequest copyWith(
          void Function(TraefikUpdateVersionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikUpdateVersionRequest))
          as TraefikUpdateVersionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikUpdateVersionRequest() / TraefikUpdateVersionRequest.new instead')
  static TraefikUpdateVersionRequest create() =>
      TraefikUpdateVersionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikUpdateVersionRequest._();
  @$core.override
  TraefikUpdateVersionRequest createEmptyInstance() =>
      TraefikUpdateVersionRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikUpdateVersionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikUpdateVersionRequest>(
          TraefikUpdateVersionRequest.$_createMessage);
  static TraefikUpdateVersionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get version => $_getSZ(1);
  @$pb.TagNumber(2)
  set version($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);
}

class TraefikUpdateVersionResponse extends $pb.GeneratedMessage {
  factory TraefikUpdateVersionResponse({
    TraefikVersion? version,
  }) {
    final result = TraefikUpdateVersionResponse._();
    if (version != null) result.version = version;
    return result;
  }

  TraefikUpdateVersionResponse._();

  factory TraefikUpdateVersionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateVersionResponse()..mergeFromBuffer(data, registry);
  factory TraefikUpdateVersionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikUpdateVersionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikUpdateVersionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikUpdateVersionResponse.$_createMessage)
    ..aOM<TraefikVersion>(1, _omitFieldNames ? '' : 'version',
        subBuilder: TraefikVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateVersionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikUpdateVersionResponse copyWith(
          void Function(TraefikUpdateVersionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikUpdateVersionResponse))
          as TraefikUpdateVersionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikUpdateVersionResponse() / TraefikUpdateVersionResponse.new instead')
  static TraefikUpdateVersionResponse create() =>
      TraefikUpdateVersionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikUpdateVersionResponse._();
  @$core.override
  TraefikUpdateVersionResponse createEmptyInstance() =>
      TraefikUpdateVersionResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikUpdateVersionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikUpdateVersionResponse>(
          TraefikUpdateVersionResponse.$_createMessage);
  static TraefikUpdateVersionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikVersion get version => $_getN(0);
  @$pb.TagNumber(1)
  set version(TraefikVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearVersion() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikVersion ensureVersion() => $_ensure(0);
}

class TraefikRenderMiddlewareRequest extends $pb.GeneratedMessage {
  factory TraefikRenderMiddlewareRequest({
    TraefikMiddlewareKind? kind,
    $core.String? name,
    $core.Iterable<$core.MapEntry<$core.String, $core.String>>? values,
    $core.Iterable<$core.String>? list,
  }) {
    final result = TraefikRenderMiddlewareRequest._();
    if (kind != null) result.kind = kind;
    if (name != null) result.name = name;
    if (values != null) result.values.addEntries(values);
    if (list != null) result.list.addAll(list);
    return result;
  }

  TraefikRenderMiddlewareRequest._();

  factory TraefikRenderMiddlewareRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRenderMiddlewareRequest()..mergeFromBuffer(data, registry);
  factory TraefikRenderMiddlewareRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRenderMiddlewareRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikRenderMiddlewareRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikRenderMiddlewareRequest.$_createMessage)
    ..aE<TraefikMiddlewareKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: TraefikMiddlewareKind.values)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..m<$core.String, $core.String>(3, _omitFieldNames ? '' : 'values',
        entryClassName: 'TraefikRenderMiddlewareRequest.ValuesEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OS,
        packageName: const $pb.PackageName('openoxide.api.v1'))
    ..pPS(4, _omitFieldNames ? '' : 'list')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRenderMiddlewareRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRenderMiddlewareRequest copyWith(
          void Function(TraefikRenderMiddlewareRequest) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikRenderMiddlewareRequest))
          as TraefikRenderMiddlewareRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikRenderMiddlewareRequest() / TraefikRenderMiddlewareRequest.new instead')
  static TraefikRenderMiddlewareRequest create() =>
      TraefikRenderMiddlewareRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikRenderMiddlewareRequest._();
  @$core.override
  TraefikRenderMiddlewareRequest createEmptyInstance() =>
      TraefikRenderMiddlewareRequest._();
  @$core.pragma('dart2js:noInline')
  static TraefikRenderMiddlewareRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikRenderMiddlewareRequest>(
          TraefikRenderMiddlewareRequest.$_createMessage);
  static TraefikRenderMiddlewareRequest? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikMiddlewareKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(TraefikMiddlewareKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  /// Middleware name; alphanumerics, `-` and `_` only (server-validated).
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// Kind-specific scalar values (e.g. `prefix`, `scheme`, `average`, `burst`, headers).
  @$pb.TagNumber(3)
  $pb.PbMap<$core.String, $core.String> get values => $_getMap(2);

  /// Kind-specific list input (prefix list for strip-prefix, source ranges for allow-list).
  @$pb.TagNumber(4)
  $pb.PbList<$core.String> get list => $_getList(3);
}

class TraefikRenderMiddlewareResponse extends $pb.GeneratedMessage {
  factory TraefikRenderMiddlewareResponse({
    TraefikMiddleware? middleware,
  }) {
    final result = TraefikRenderMiddlewareResponse._();
    if (middleware != null) result.middleware = middleware;
    return result;
  }

  TraefikRenderMiddlewareResponse._();

  factory TraefikRenderMiddlewareResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRenderMiddlewareResponse()..mergeFromBuffer(data, registry);
  factory TraefikRenderMiddlewareResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TraefikRenderMiddlewareResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TraefikRenderMiddlewareResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TraefikRenderMiddlewareResponse.$_createMessage)
    ..aOM<TraefikMiddleware>(1, _omitFieldNames ? '' : 'middleware',
        subBuilder: TraefikMiddleware.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRenderMiddlewareResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TraefikRenderMiddlewareResponse copyWith(
          void Function(TraefikRenderMiddlewareResponse) updates) =>
      super.copyWith(
              (message) => updates(message as TraefikRenderMiddlewareResponse))
          as TraefikRenderMiddlewareResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TraefikRenderMiddlewareResponse() / TraefikRenderMiddlewareResponse.new instead')
  static TraefikRenderMiddlewareResponse create() =>
      TraefikRenderMiddlewareResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      TraefikRenderMiddlewareResponse._();
  @$core.override
  TraefikRenderMiddlewareResponse createEmptyInstance() =>
      TraefikRenderMiddlewareResponse._();
  @$core.pragma('dart2js:noInline')
  static TraefikRenderMiddlewareResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TraefikRenderMiddlewareResponse>(
          TraefikRenderMiddlewareResponse.$_createMessage);
  static TraefikRenderMiddlewareResponse? _defaultInstance;

  @$pb.TagNumber(1)
  TraefikMiddleware get middleware => $_getN(0);
  @$pb.TagNumber(1)
  set middleware(TraefikMiddleware value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMiddleware() => $_has(0);
  @$pb.TagNumber(1)
  void clearMiddleware() => $_clearField(1);
  @$pb.TagNumber(1)
  TraefikMiddleware ensureMiddleware() => $_ensure(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

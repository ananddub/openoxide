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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'terminal.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'terminal.pbenum.dart';

class OpenTerminalRequest extends $pb.GeneratedMessage {
  factory OpenTerminalRequest({
    TerminalKind? kind,
    $fixnum.Int64? serverId,
    $core.String? container,
    $core.String? shell,
    $core.int? cols,
    $core.int? rows,
    $core.bool? host,
  }) {
    final result = OpenTerminalRequest._();
    if (kind != null) result.kind = kind;
    if (serverId != null) result.serverId = serverId;
    if (container != null) result.container = container;
    if (shell != null) result.shell = shell;
    if (cols != null) result.cols = cols;
    if (rows != null) result.rows = rows;
    if (host != null) result.host = host;
    return result;
  }

  OpenTerminalRequest._();

  factory OpenTerminalRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenTerminalRequest()..mergeFromBuffer(data, registry);
  factory OpenTerminalRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenTerminalRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OpenTerminalRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: OpenTerminalRequest.$_createMessage)
    ..aE<TerminalKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: TerminalKind.values)
    ..aInt64(2, _omitFieldNames ? '' : 'serverId')
    ..aOS(3, _omitFieldNames ? '' : 'container')
    ..aOS(4, _omitFieldNames ? '' : 'shell')
    ..aI(5, _omitFieldNames ? '' : 'cols', fieldType: $pb.PbFieldType.OU3)
    ..aI(6, _omitFieldNames ? '' : 'rows', fieldType: $pb.PbFieldType.OU3)
    ..aOB(7, _omitFieldNames ? '' : 'host')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenTerminalRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenTerminalRequest copyWith(void Function(OpenTerminalRequest) updates) =>
      super.copyWith((message) => updates(message as OpenTerminalRequest))
          as OpenTerminalRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use OpenTerminalRequest() / OpenTerminalRequest.new instead')
  static OpenTerminalRequest create() => OpenTerminalRequest._();
  static $pb.GeneratedMessage $_createMessage() => OpenTerminalRequest._();
  @$core.override
  OpenTerminalRequest createEmptyInstance() => OpenTerminalRequest._();
  @$core.pragma('dart2js:noInline')
  static OpenTerminalRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OpenTerminalRequest>(
          OpenTerminalRequest.$_createMessage);
  static OpenTerminalRequest? _defaultInstance;

  @$pb.TagNumber(1)
  TerminalKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(TerminalKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  /// Required for TERMINAL_KIND_REMOTE_SERVER; optional for TERMINAL_KIND_DOCKER (falls back to the local Agent).
  @$pb.TagNumber(2)
  $fixnum.Int64 get serverId => $_getI64(1);
  @$pb.TagNumber(2)
  set serverId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerId() => $_clearField(2);

  /// Container name for TERMINAL_KIND_DOCKER.
  @$pb.TagNumber(3)
  $core.String get container => $_getSZ(2);
  @$pb.TagNumber(3)
  set container($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasContainer() => $_has(2);
  @$pb.TagNumber(3)
  void clearContainer() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get shell => $_getSZ(3);
  @$pb.TagNumber(4)
  set shell($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasShell() => $_has(3);
  @$pb.TagNumber(4)
  void clearShell() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get cols => $_getIZ(4);
  @$pb.TagNumber(5)
  set cols($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCols() => $_has(4);
  @$pb.TagNumber(5)
  void clearCols() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.int get rows => $_getIZ(5);
  @$pb.TagNumber(6)
  set rows($core.int value) => $_setUnsignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRows() => $_has(5);
  @$pb.TagNumber(6)
  void clearRows() => $_clearField(6);

  /// TERMINAL_KIND_DOCKER only: escape through the Agent to the host itself (privileged helper container + chroot) instead of a workload container.
  @$pb.TagNumber(7)
  $core.bool get host => $_getBF(6);
  @$pb.TagNumber(7)
  set host($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasHost() => $_has(6);
  @$pb.TagNumber(7)
  void clearHost() => $_clearField(7);
}

enum TerminalEvent_Payload { started, output, error, exit, notSet }

class TerminalEvent extends $pb.GeneratedMessage {
  factory TerminalEvent({
    $core.String? sessionId,
    TerminalStarted? started,
    TerminalOutput? output,
    TerminalError? error,
    TerminalExit? exit,
  }) {
    final result = TerminalEvent._();
    if (sessionId != null) result.sessionId = sessionId;
    if (started != null) result.started = started;
    if (output != null) result.output = output;
    if (error != null) result.error = error;
    if (exit != null) result.exit = exit;
    return result;
  }

  TerminalEvent._();

  factory TerminalEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalEvent()..mergeFromBuffer(data, registry);
  factory TerminalEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalEvent()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, TerminalEvent_Payload>
      _TerminalEvent_PayloadByTag = {
    2: TerminalEvent_Payload.started,
    3: TerminalEvent_Payload.output,
    4: TerminalEvent_Payload.error,
    5: TerminalEvent_Payload.exit,
    0: TerminalEvent_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalEvent',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalEvent.$_createMessage)
    ..oo(0, [2, 3, 4, 5])
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOM<TerminalStarted>(2, _omitFieldNames ? '' : 'started',
        subBuilder: TerminalStarted.$_createMessage)
    ..aOM<TerminalOutput>(3, _omitFieldNames ? '' : 'output',
        subBuilder: TerminalOutput.$_createMessage)
    ..aOM<TerminalError>(4, _omitFieldNames ? '' : 'error',
        subBuilder: TerminalError.$_createMessage)
    ..aOM<TerminalExit>(5, _omitFieldNames ? '' : 'exit',
        subBuilder: TerminalExit.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalEvent copyWith(void Function(TerminalEvent) updates) =>
      super.copyWith((message) => updates(message as TerminalEvent))
          as TerminalEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalEvent() / TerminalEvent.new instead')
  static TerminalEvent create() => TerminalEvent._();
  static $pb.GeneratedMessage $_createMessage() => TerminalEvent._();
  @$core.override
  TerminalEvent createEmptyInstance() => TerminalEvent._();
  @$core.pragma('dart2js:noInline')
  static TerminalEvent getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalEvent>(
          TerminalEvent.$_createMessage);
  static TerminalEvent? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  TerminalEvent_Payload whichPayload() =>
      _TerminalEvent_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearPayload() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  TerminalStarted get started => $_getN(1);
  @$pb.TagNumber(2)
  set started(TerminalStarted value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasStarted() => $_has(1);
  @$pb.TagNumber(2)
  void clearStarted() => $_clearField(2);
  @$pb.TagNumber(2)
  TerminalStarted ensureStarted() => $_ensure(1);

  @$pb.TagNumber(3)
  TerminalOutput get output => $_getN(2);
  @$pb.TagNumber(3)
  set output(TerminalOutput value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasOutput() => $_has(2);
  @$pb.TagNumber(3)
  void clearOutput() => $_clearField(3);
  @$pb.TagNumber(3)
  TerminalOutput ensureOutput() => $_ensure(2);

  @$pb.TagNumber(4)
  TerminalError get error => $_getN(3);
  @$pb.TagNumber(4)
  set error(TerminalError value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasError() => $_has(3);
  @$pb.TagNumber(4)
  void clearError() => $_clearField(4);
  @$pb.TagNumber(4)
  TerminalError ensureError() => $_ensure(3);

  @$pb.TagNumber(5)
  TerminalExit get exit => $_getN(4);
  @$pb.TagNumber(5)
  set exit(TerminalExit value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasExit() => $_has(4);
  @$pb.TagNumber(5)
  void clearExit() => $_clearField(5);
  @$pb.TagNumber(5)
  TerminalExit ensureExit() => $_ensure(4);
}

class TerminalStarted extends $pb.GeneratedMessage {
  factory TerminalStarted({
    TerminalKind? kind,
    $core.String? host,
  }) {
    final result = TerminalStarted._();
    if (kind != null) result.kind = kind;
    if (host != null) result.host = host;
    return result;
  }

  TerminalStarted._();

  factory TerminalStarted.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalStarted()..mergeFromBuffer(data, registry);
  factory TerminalStarted.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalStarted()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalStarted',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalStarted.$_createMessage)
    ..aE<TerminalKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: TerminalKind.values)
    ..aOS(2, _omitFieldNames ? '' : 'host')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalStarted clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalStarted copyWith(void Function(TerminalStarted) updates) =>
      super.copyWith((message) => updates(message as TerminalStarted))
          as TerminalStarted;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalStarted() / TerminalStarted.new instead')
  static TerminalStarted create() => TerminalStarted._();
  static $pb.GeneratedMessage $_createMessage() => TerminalStarted._();
  @$core.override
  TerminalStarted createEmptyInstance() => TerminalStarted._();
  @$core.pragma('dart2js:noInline')
  static TerminalStarted getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalStarted>(
          TerminalStarted.$_createMessage);
  static TerminalStarted? _defaultInstance;

  @$pb.TagNumber(1)
  TerminalKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(TerminalKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get host => $_getSZ(1);
  @$pb.TagNumber(2)
  set host($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHost() => $_has(1);
  @$pb.TagNumber(2)
  void clearHost() => $_clearField(2);
}

class TerminalOutput extends $pb.GeneratedMessage {
  factory TerminalOutput({
    TerminalOutputStream? stream,
    $core.List<$core.int>? data,
  }) {
    final result = TerminalOutput._();
    if (stream != null) result.stream = stream;
    if (data != null) result.data = data;
    return result;
  }

  TerminalOutput._();

  factory TerminalOutput.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalOutput()..mergeFromBuffer(data, registry);
  factory TerminalOutput.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalOutput()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalOutput',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalOutput.$_createMessage)
    ..aE<TerminalOutputStream>(1, _omitFieldNames ? '' : 'stream',
        enumValues: TerminalOutputStream.values)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalOutput clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalOutput copyWith(void Function(TerminalOutput) updates) =>
      super.copyWith((message) => updates(message as TerminalOutput))
          as TerminalOutput;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalOutput() / TerminalOutput.new instead')
  static TerminalOutput create() => TerminalOutput._();
  static $pb.GeneratedMessage $_createMessage() => TerminalOutput._();
  @$core.override
  TerminalOutput createEmptyInstance() => TerminalOutput._();
  @$core.pragma('dart2js:noInline')
  static TerminalOutput getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalOutput>(
          TerminalOutput.$_createMessage);
  static TerminalOutput? _defaultInstance;

  @$pb.TagNumber(1)
  TerminalOutputStream get stream => $_getN(0);
  @$pb.TagNumber(1)
  set stream(TerminalOutputStream value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStream() => $_has(0);
  @$pb.TagNumber(1)
  void clearStream() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => $_clearField(2);
}

class TerminalError extends $pb.GeneratedMessage {
  factory TerminalError({
    $core.String? message,
    $core.String? code,
  }) {
    final result = TerminalError._();
    if (message != null) result.message = message;
    if (code != null) result.code = code;
    return result;
  }

  TerminalError._();

  factory TerminalError.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalError()..mergeFromBuffer(data, registry);
  factory TerminalError.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalError()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalError',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalError.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'message')
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalError clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalError copyWith(void Function(TerminalError) updates) =>
      super.copyWith((message) => updates(message as TerminalError))
          as TerminalError;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalError() / TerminalError.new instead')
  static TerminalError create() => TerminalError._();
  static $pb.GeneratedMessage $_createMessage() => TerminalError._();
  @$core.override
  TerminalError createEmptyInstance() => TerminalError._();
  @$core.pragma('dart2js:noInline')
  static TerminalError getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalError>(
          TerminalError.$_createMessage);
  static TerminalError? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(1)
  set message($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => $_clearField(2);
}

class TerminalExit extends $pb.GeneratedMessage {
  factory TerminalExit({
    $core.int? code,
  }) {
    final result = TerminalExit._();
    if (code != null) result.code = code;
    return result;
  }

  TerminalExit._();

  factory TerminalExit.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalExit()..mergeFromBuffer(data, registry);
  factory TerminalExit.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalExit()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalExit',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalExit.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalExit clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalExit copyWith(void Function(TerminalExit) updates) =>
      super.copyWith((message) => updates(message as TerminalExit))
          as TerminalExit;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalExit() / TerminalExit.new instead')
  static TerminalExit create() => TerminalExit._();
  static $pb.GeneratedMessage $_createMessage() => TerminalExit._();
  @$core.override
  TerminalExit createEmptyInstance() => TerminalExit._();
  @$core.pragma('dart2js:noInline')
  static TerminalExit getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalExit>(
          TerminalExit.$_createMessage);
  static TerminalExit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get code => $_getIZ(0);
  @$pb.TagNumber(1)
  set code($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);
}

enum ExecutionCommandRequest_Command { input, resize, stop, notSet }

class ExecutionCommandRequest extends $pb.GeneratedMessage {
  factory ExecutionCommandRequest({
    $core.String? sessionId,
    TerminalInput? input,
    TerminalResize? resize,
    TerminalStop? stop,
  }) {
    final result = ExecutionCommandRequest._();
    if (sessionId != null) result.sessionId = sessionId;
    if (input != null) result.input = input;
    if (resize != null) result.resize = resize;
    if (stop != null) result.stop = stop;
    return result;
  }

  ExecutionCommandRequest._();

  factory ExecutionCommandRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExecutionCommandRequest()..mergeFromBuffer(data, registry);
  factory ExecutionCommandRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExecutionCommandRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ExecutionCommandRequest_Command>
      _ExecutionCommandRequest_CommandByTag = {
    2: ExecutionCommandRequest_Command.input,
    3: ExecutionCommandRequest_Command.resize,
    4: ExecutionCommandRequest_Command.stop,
    0: ExecutionCommandRequest_Command.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExecutionCommandRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ExecutionCommandRequest.$_createMessage)
    ..oo(0, [2, 3, 4])
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOM<TerminalInput>(2, _omitFieldNames ? '' : 'input',
        subBuilder: TerminalInput.$_createMessage)
    ..aOM<TerminalResize>(3, _omitFieldNames ? '' : 'resize',
        subBuilder: TerminalResize.$_createMessage)
    ..aOM<TerminalStop>(4, _omitFieldNames ? '' : 'stop',
        subBuilder: TerminalStop.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExecutionCommandRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExecutionCommandRequest copyWith(
          void Function(ExecutionCommandRequest) updates) =>
      super.copyWith((message) => updates(message as ExecutionCommandRequest))
          as ExecutionCommandRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ExecutionCommandRequest() / ExecutionCommandRequest.new instead')
  static ExecutionCommandRequest create() => ExecutionCommandRequest._();
  static $pb.GeneratedMessage $_createMessage() => ExecutionCommandRequest._();
  @$core.override
  ExecutionCommandRequest createEmptyInstance() => ExecutionCommandRequest._();
  @$core.pragma('dart2js:noInline')
  static ExecutionCommandRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExecutionCommandRequest>(
          ExecutionCommandRequest.$_createMessage);
  static ExecutionCommandRequest? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  ExecutionCommandRequest_Command whichCommand() =>
      _ExecutionCommandRequest_CommandByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  void clearCommand() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  TerminalInput get input => $_getN(1);
  @$pb.TagNumber(2)
  set input(TerminalInput value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasInput() => $_has(1);
  @$pb.TagNumber(2)
  void clearInput() => $_clearField(2);
  @$pb.TagNumber(2)
  TerminalInput ensureInput() => $_ensure(1);

  @$pb.TagNumber(3)
  TerminalResize get resize => $_getN(2);
  @$pb.TagNumber(3)
  set resize(TerminalResize value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasResize() => $_has(2);
  @$pb.TagNumber(3)
  void clearResize() => $_clearField(3);
  @$pb.TagNumber(3)
  TerminalResize ensureResize() => $_ensure(2);

  @$pb.TagNumber(4)
  TerminalStop get stop => $_getN(3);
  @$pb.TagNumber(4)
  set stop(TerminalStop value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStop() => $_has(3);
  @$pb.TagNumber(4)
  void clearStop() => $_clearField(4);
  @$pb.TagNumber(4)
  TerminalStop ensureStop() => $_ensure(3);
}

class TerminalInput extends $pb.GeneratedMessage {
  factory TerminalInput({
    $core.List<$core.int>? data,
  }) {
    final result = TerminalInput._();
    if (data != null) result.data = data;
    return result;
  }

  TerminalInput._();

  factory TerminalInput.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalInput()..mergeFromBuffer(data, registry);
  factory TerminalInput.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalInput()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalInput',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalInput.$_createMessage)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalInput clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalInput copyWith(void Function(TerminalInput) updates) =>
      super.copyWith((message) => updates(message as TerminalInput))
          as TerminalInput;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalInput() / TerminalInput.new instead')
  static TerminalInput create() => TerminalInput._();
  static $pb.GeneratedMessage $_createMessage() => TerminalInput._();
  @$core.override
  TerminalInput createEmptyInstance() => TerminalInput._();
  @$core.pragma('dart2js:noInline')
  static TerminalInput getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalInput>(
          TerminalInput.$_createMessage);
  static TerminalInput? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => $_clearField(1);
}

class TerminalResize extends $pb.GeneratedMessage {
  factory TerminalResize({
    $core.int? cols,
    $core.int? rows,
  }) {
    final result = TerminalResize._();
    if (cols != null) result.cols = cols;
    if (rows != null) result.rows = rows;
    return result;
  }

  TerminalResize._();

  factory TerminalResize.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalResize()..mergeFromBuffer(data, registry);
  factory TerminalResize.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalResize()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalResize',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalResize.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'cols', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'rows', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalResize clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalResize copyWith(void Function(TerminalResize) updates) =>
      super.copyWith((message) => updates(message as TerminalResize))
          as TerminalResize;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalResize() / TerminalResize.new instead')
  static TerminalResize create() => TerminalResize._();
  static $pb.GeneratedMessage $_createMessage() => TerminalResize._();
  @$core.override
  TerminalResize createEmptyInstance() => TerminalResize._();
  @$core.pragma('dart2js:noInline')
  static TerminalResize getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalResize>(
          TerminalResize.$_createMessage);
  static TerminalResize? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get cols => $_getIZ(0);
  @$pb.TagNumber(1)
  set cols($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCols() => $_has(0);
  @$pb.TagNumber(1)
  void clearCols() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get rows => $_getIZ(1);
  @$pb.TagNumber(2)
  set rows($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRows() => $_has(1);
  @$pb.TagNumber(2)
  void clearRows() => $_clearField(2);
}

class TerminalStop extends $pb.GeneratedMessage {
  factory TerminalStop() => TerminalStop._();

  TerminalStop._();

  factory TerminalStop.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalStop()..mergeFromBuffer(data, registry);
  factory TerminalStop.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TerminalStop()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TerminalStop',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TerminalStop.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalStop clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TerminalStop copyWith(void Function(TerminalStop) updates) =>
      super.copyWith((message) => updates(message as TerminalStop))
          as TerminalStop;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TerminalStop() / TerminalStop.new instead')
  static TerminalStop create() => TerminalStop._();
  static $pb.GeneratedMessage $_createMessage() => TerminalStop._();
  @$core.override
  TerminalStop createEmptyInstance() => TerminalStop._();
  @$core.pragma('dart2js:noInline')
  static TerminalStop getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TerminalStop>(
          TerminalStop.$_createMessage);
  static TerminalStop? _defaultInstance;
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/system.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class SystemHealthRequest extends $pb.GeneratedMessage {
  factory SystemHealthRequest() => SystemHealthRequest._();

  SystemHealthRequest._();

  factory SystemHealthRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemHealthRequest()..mergeFromBuffer(data, registry);
  factory SystemHealthRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemHealthRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SystemHealthRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SystemHealthRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemHealthRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemHealthRequest copyWith(void Function(SystemHealthRequest) updates) =>
      super.copyWith((message) => updates(message as SystemHealthRequest))
          as SystemHealthRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use SystemHealthRequest() / SystemHealthRequest.new instead')
  static SystemHealthRequest create() => SystemHealthRequest._();
  static $pb.GeneratedMessage $_createMessage() => SystemHealthRequest._();
  @$core.override
  SystemHealthRequest createEmptyInstance() => SystemHealthRequest._();
  @$core.pragma('dart2js:noInline')
  static SystemHealthRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SystemHealthRequest>(
          SystemHealthRequest.$_createMessage);
  static SystemHealthRequest? _defaultInstance;
}

/// success means every dependency (database, docker) is healthy; message summarises unhealthy ones.
class SystemHealthResponse extends $pb.GeneratedMessage {
  factory SystemHealthResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = SystemHealthResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  SystemHealthResponse._();

  factory SystemHealthResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemHealthResponse()..mergeFromBuffer(data, registry);
  factory SystemHealthResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemHealthResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SystemHealthResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SystemHealthResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemHealthResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemHealthResponse copyWith(void Function(SystemHealthResponse) updates) =>
      super.copyWith((message) => updates(message as SystemHealthResponse))
          as SystemHealthResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SystemHealthResponse() / SystemHealthResponse.new instead')
  static SystemHealthResponse create() => SystemHealthResponse._();
  static $pb.GeneratedMessage $_createMessage() => SystemHealthResponse._();
  @$core.override
  SystemHealthResponse createEmptyInstance() => SystemHealthResponse._();
  @$core.pragma('dart2js:noInline')
  static SystemHealthResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SystemHealthResponse>(
          SystemHealthResponse.$_createMessage);
  static SystemHealthResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

class SystemTestConfigRequest extends $pb.GeneratedMessage {
  factory SystemTestConfigRequest() => SystemTestConfigRequest._();

  SystemTestConfigRequest._();

  factory SystemTestConfigRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemTestConfigRequest()..mergeFromBuffer(data, registry);
  factory SystemTestConfigRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemTestConfigRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SystemTestConfigRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SystemTestConfigRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemTestConfigRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemTestConfigRequest copyWith(
          void Function(SystemTestConfigRequest) updates) =>
      super.copyWith((message) => updates(message as SystemTestConfigRequest))
          as SystemTestConfigRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SystemTestConfigRequest() / SystemTestConfigRequest.new instead')
  static SystemTestConfigRequest create() => SystemTestConfigRequest._();
  static $pb.GeneratedMessage $_createMessage() => SystemTestConfigRequest._();
  @$core.override
  SystemTestConfigRequest createEmptyInstance() => SystemTestConfigRequest._();
  @$core.pragma('dart2js:noInline')
  static SystemTestConfigRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SystemTestConfigRequest>(
          SystemTestConfigRequest.$_createMessage);
  static SystemTestConfigRequest? _defaultInstance;
}

/// success means no configuration errors; warnings are informational and surface through message only.
class SystemTestConfigResponse extends $pb.GeneratedMessage {
  factory SystemTestConfigResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = SystemTestConfigResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  SystemTestConfigResponse._();

  factory SystemTestConfigResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemTestConfigResponse()..mergeFromBuffer(data, registry);
  factory SystemTestConfigResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SystemTestConfigResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SystemTestConfigResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SystemTestConfigResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemTestConfigResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SystemTestConfigResponse copyWith(
          void Function(SystemTestConfigResponse) updates) =>
      super.copyWith((message) => updates(message as SystemTestConfigResponse))
          as SystemTestConfigResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SystemTestConfigResponse() / SystemTestConfigResponse.new instead')
  static SystemTestConfigResponse create() => SystemTestConfigResponse._();
  static $pb.GeneratedMessage $_createMessage() => SystemTestConfigResponse._();
  @$core.override
  SystemTestConfigResponse createEmptyInstance() =>
      SystemTestConfigResponse._();
  @$core.pragma('dart2js:noInline')
  static SystemTestConfigResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SystemTestConfigResponse>(
          SystemTestConfigResponse.$_createMessage);
  static SystemTestConfigResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

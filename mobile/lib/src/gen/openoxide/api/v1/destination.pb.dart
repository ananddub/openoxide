// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/destination.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Destination extends $pb.GeneratedMessage {
  factory Destination({
    $core.String? id,
    $core.String? name,
    $core.String? provider,
    $core.String? accessKey,
    $core.String? secretAccessKey,
    $core.String? bucket,
    $core.String? region,
    $core.String? endpoint,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = Destination._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (provider != null) result.provider = provider;
    if (accessKey != null) result.accessKey = accessKey;
    if (secretAccessKey != null) result.secretAccessKey = secretAccessKey;
    if (bucket != null) result.bucket = bucket;
    if (region != null) result.region = region;
    if (endpoint != null) result.endpoint = endpoint;
    if (organizationId != null) result.organizationId = organizationId;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Destination._();

  factory Destination.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Destination()..mergeFromBuffer(data, registry);
  factory Destination.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Destination()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Destination',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Destination.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'provider')
    ..aOS(4, _omitFieldNames ? '' : 'accessKey')
    ..aOS(5, _omitFieldNames ? '' : 'secretAccessKey')
    ..aOS(6, _omitFieldNames ? '' : 'bucket')
    ..aOS(7, _omitFieldNames ? '' : 'region')
    ..aOS(8, _omitFieldNames ? '' : 'endpoint')
    ..aInt64(9, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(10, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(11, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Destination clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Destination copyWith(void Function(Destination) updates) =>
      super.copyWith((message) => updates(message as Destination))
          as Destination;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Destination() / Destination.new instead')
  static Destination create() => Destination._();
  static $pb.GeneratedMessage $_createMessage() => Destination._();
  @$core.override
  Destination createEmptyInstance() => Destination._();
  @$core.pragma('dart2js:noInline')
  static Destination getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Destination>(
          Destination.$_createMessage);
  static Destination? _defaultInstance;

  /// UUID primary key.
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// rclone provider label (S3, R2, BACKBLAZE, MINIO, WASABI, ...).
  @$pb.TagNumber(3)
  $core.String get provider => $_getSZ(2);
  @$pb.TagNumber(3)
  set provider($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get accessKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set accessKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAccessKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearAccessKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get secretAccessKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set secretAccessKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSecretAccessKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearSecretAccessKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get bucket => $_getSZ(5);
  @$pb.TagNumber(6)
  set bucket($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasBucket() => $_has(5);
  @$pb.TagNumber(6)
  void clearBucket() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get region => $_getSZ(6);
  @$pb.TagNumber(7)
  set region($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRegion() => $_has(6);
  @$pb.TagNumber(7)
  void clearRegion() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get endpoint => $_getSZ(7);
  @$pb.TagNumber(8)
  set endpoint($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEndpoint() => $_has(7);
  @$pb.TagNumber(8)
  void clearEndpoint() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get organizationId => $_getI64(8);
  @$pb.TagNumber(9)
  set organizationId($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasOrganizationId() => $_has(8);
  @$pb.TagNumber(9)
  void clearOrganizationId() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get createdAt => $_getI64(9);
  @$pb.TagNumber(10)
  set createdAt($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCreatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearCreatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get updatedAt => $_getI64(10);
  @$pb.TagNumber(11)
  set updatedAt($fixnum.Int64 value) => $_setInt64(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUpdatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearUpdatedAt() => $_clearField(11);
}

class ListDestinationsRequest extends $pb.GeneratedMessage {
  factory ListDestinationsRequest() => ListDestinationsRequest._();

  ListDestinationsRequest._();

  factory ListDestinationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDestinationsRequest()..mergeFromBuffer(data, registry);
  factory ListDestinationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDestinationsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDestinationsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDestinationsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDestinationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDestinationsRequest copyWith(
          void Function(ListDestinationsRequest) updates) =>
      super.copyWith((message) => updates(message as ListDestinationsRequest))
          as ListDestinationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDestinationsRequest() / ListDestinationsRequest.new instead')
  static ListDestinationsRequest create() => ListDestinationsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListDestinationsRequest._();
  @$core.override
  ListDestinationsRequest createEmptyInstance() => ListDestinationsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListDestinationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDestinationsRequest>(
          ListDestinationsRequest.$_createMessage);
  static ListDestinationsRequest? _defaultInstance;
}

class ListDestinationsResponse extends $pb.GeneratedMessage {
  factory ListDestinationsResponse({
    $core.Iterable<Destination>? destinations,
  }) {
    final result = ListDestinationsResponse._();
    if (destinations != null) result.destinations.addAll(destinations);
    return result;
  }

  ListDestinationsResponse._();

  factory ListDestinationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDestinationsResponse()..mergeFromBuffer(data, registry);
  factory ListDestinationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListDestinationsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDestinationsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListDestinationsResponse.$_createMessage)
    ..pPM<Destination>(1, _omitFieldNames ? '' : 'destinations',
        subBuilder: Destination.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDestinationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDestinationsResponse copyWith(
          void Function(ListDestinationsResponse) updates) =>
      super.copyWith((message) => updates(message as ListDestinationsResponse))
          as ListDestinationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListDestinationsResponse() / ListDestinationsResponse.new instead')
  static ListDestinationsResponse create() => ListDestinationsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListDestinationsResponse._();
  @$core.override
  ListDestinationsResponse createEmptyInstance() =>
      ListDestinationsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListDestinationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDestinationsResponse>(
          ListDestinationsResponse.$_createMessage);
  static ListDestinationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Destination> get destinations => $_getList(0);
}

class GetDestinationRequest extends $pb.GeneratedMessage {
  factory GetDestinationRequest({
    $core.String? id,
  }) {
    final result = GetDestinationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  GetDestinationRequest._();

  factory GetDestinationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDestinationRequest()..mergeFromBuffer(data, registry);
  factory GetDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDestinationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDestinationRequest copyWith(
          void Function(GetDestinationRequest) updates) =>
      super.copyWith((message) => updates(message as GetDestinationRequest))
          as GetDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetDestinationRequest() / GetDestinationRequest.new instead')
  static GetDestinationRequest create() => GetDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetDestinationRequest._();
  @$core.override
  GetDestinationRequest createEmptyInstance() => GetDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static GetDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDestinationRequest>(
          GetDestinationRequest.$_createMessage);
  static GetDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class GetDestinationResponse extends $pb.GeneratedMessage {
  factory GetDestinationResponse({
    Destination? destination,
  }) {
    final result = GetDestinationResponse._();
    if (destination != null) result.destination = destination;
    return result;
  }

  GetDestinationResponse._();

  factory GetDestinationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDestinationResponse()..mergeFromBuffer(data, registry);
  factory GetDestinationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetDestinationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDestinationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetDestinationResponse.$_createMessage)
    ..aOM<Destination>(1, _omitFieldNames ? '' : 'destination',
        subBuilder: Destination.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDestinationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDestinationResponse copyWith(
          void Function(GetDestinationResponse) updates) =>
      super.copyWith((message) => updates(message as GetDestinationResponse))
          as GetDestinationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetDestinationResponse() / GetDestinationResponse.new instead')
  static GetDestinationResponse create() => GetDestinationResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetDestinationResponse._();
  @$core.override
  GetDestinationResponse createEmptyInstance() => GetDestinationResponse._();
  @$core.pragma('dart2js:noInline')
  static GetDestinationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDestinationResponse>(
          GetDestinationResponse.$_createMessage);
  static GetDestinationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Destination get destination => $_getN(0);
  @$pb.TagNumber(1)
  set destination(Destination value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDestination() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestination() => $_clearField(1);
  @$pb.TagNumber(1)
  Destination ensureDestination() => $_ensure(0);
}

/// The authenticated organization always owns the created row.
class CreateDestinationRequest extends $pb.GeneratedMessage {
  factory CreateDestinationRequest({
    $core.String? name,
    $core.String? provider,
    $core.String? accessKey,
    $core.String? secretAccessKey,
    $core.String? bucket,
    $core.String? region,
    $core.String? endpoint,
  }) {
    final result = CreateDestinationRequest._();
    if (name != null) result.name = name;
    if (provider != null) result.provider = provider;
    if (accessKey != null) result.accessKey = accessKey;
    if (secretAccessKey != null) result.secretAccessKey = secretAccessKey;
    if (bucket != null) result.bucket = bucket;
    if (region != null) result.region = region;
    if (endpoint != null) result.endpoint = endpoint;
    return result;
  }

  CreateDestinationRequest._();

  factory CreateDestinationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDestinationRequest()..mergeFromBuffer(data, registry);
  factory CreateDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDestinationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'provider')
    ..aOS(3, _omitFieldNames ? '' : 'accessKey')
    ..aOS(4, _omitFieldNames ? '' : 'secretAccessKey')
    ..aOS(5, _omitFieldNames ? '' : 'bucket')
    ..aOS(6, _omitFieldNames ? '' : 'region')
    ..aOS(7, _omitFieldNames ? '' : 'endpoint')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDestinationRequest copyWith(
          void Function(CreateDestinationRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDestinationRequest))
          as CreateDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDestinationRequest() / CreateDestinationRequest.new instead')
  static CreateDestinationRequest create() => CreateDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateDestinationRequest._();
  @$core.override
  CreateDestinationRequest createEmptyInstance() =>
      CreateDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDestinationRequest>(
          CreateDestinationRequest.$_createMessage);
  static CreateDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get provider => $_getSZ(1);
  @$pb.TagNumber(2)
  set provider($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProvider() => $_has(1);
  @$pb.TagNumber(2)
  void clearProvider() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get accessKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set accessKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAccessKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccessKey() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get secretAccessKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set secretAccessKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSecretAccessKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearSecretAccessKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get bucket => $_getSZ(4);
  @$pb.TagNumber(5)
  set bucket($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBucket() => $_has(4);
  @$pb.TagNumber(5)
  void clearBucket() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get region => $_getSZ(5);
  @$pb.TagNumber(6)
  set region($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRegion() => $_has(5);
  @$pb.TagNumber(6)
  void clearRegion() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get endpoint => $_getSZ(6);
  @$pb.TagNumber(7)
  set endpoint($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasEndpoint() => $_has(6);
  @$pb.TagNumber(7)
  void clearEndpoint() => $_clearField(7);
}

class CreateDestinationResponse extends $pb.GeneratedMessage {
  factory CreateDestinationResponse({
    Destination? destination,
  }) {
    final result = CreateDestinationResponse._();
    if (destination != null) result.destination = destination;
    return result;
  }

  CreateDestinationResponse._();

  factory CreateDestinationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDestinationResponse()..mergeFromBuffer(data, registry);
  factory CreateDestinationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateDestinationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDestinationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateDestinationResponse.$_createMessage)
    ..aOM<Destination>(1, _omitFieldNames ? '' : 'destination',
        subBuilder: Destination.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDestinationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDestinationResponse copyWith(
          void Function(CreateDestinationResponse) updates) =>
      super.copyWith((message) => updates(message as CreateDestinationResponse))
          as CreateDestinationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateDestinationResponse() / CreateDestinationResponse.new instead')
  static CreateDestinationResponse create() => CreateDestinationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreateDestinationResponse._();
  @$core.override
  CreateDestinationResponse createEmptyInstance() =>
      CreateDestinationResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateDestinationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDestinationResponse>(
          CreateDestinationResponse.$_createMessage);
  static CreateDestinationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Destination get destination => $_getN(0);
  @$pb.TagNumber(1)
  set destination(Destination value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDestination() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestination() => $_clearField(1);
  @$pb.TagNumber(1)
  Destination ensureDestination() => $_ensure(0);
}

class UpdateDestinationRequest extends $pb.GeneratedMessage {
  factory UpdateDestinationRequest({
    $core.String? id,
    $core.String? name,
    $core.String? provider,
    $core.String? accessKey,
    $core.String? secretAccessKey,
    $core.String? bucket,
    $core.String? region,
    $core.String? endpoint,
  }) {
    final result = UpdateDestinationRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (provider != null) result.provider = provider;
    if (accessKey != null) result.accessKey = accessKey;
    if (secretAccessKey != null) result.secretAccessKey = secretAccessKey;
    if (bucket != null) result.bucket = bucket;
    if (region != null) result.region = region;
    if (endpoint != null) result.endpoint = endpoint;
    return result;
  }

  UpdateDestinationRequest._();

  factory UpdateDestinationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDestinationRequest()..mergeFromBuffer(data, registry);
  factory UpdateDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDestinationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'provider')
    ..aOS(4, _omitFieldNames ? '' : 'accessKey')
    ..aOS(5, _omitFieldNames ? '' : 'secretAccessKey')
    ..aOS(6, _omitFieldNames ? '' : 'bucket')
    ..aOS(7, _omitFieldNames ? '' : 'region')
    ..aOS(8, _omitFieldNames ? '' : 'endpoint')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDestinationRequest copyWith(
          void Function(UpdateDestinationRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateDestinationRequest))
          as UpdateDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDestinationRequest() / UpdateDestinationRequest.new instead')
  static UpdateDestinationRequest create() => UpdateDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateDestinationRequest._();
  @$core.override
  UpdateDestinationRequest createEmptyInstance() =>
      UpdateDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDestinationRequest>(
          UpdateDestinationRequest.$_createMessage);
  static UpdateDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get provider => $_getSZ(2);
  @$pb.TagNumber(3)
  set provider($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get accessKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set accessKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAccessKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearAccessKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get secretAccessKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set secretAccessKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSecretAccessKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearSecretAccessKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get bucket => $_getSZ(5);
  @$pb.TagNumber(6)
  set bucket($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasBucket() => $_has(5);
  @$pb.TagNumber(6)
  void clearBucket() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get region => $_getSZ(6);
  @$pb.TagNumber(7)
  set region($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasRegion() => $_has(6);
  @$pb.TagNumber(7)
  void clearRegion() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get endpoint => $_getSZ(7);
  @$pb.TagNumber(8)
  set endpoint($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEndpoint() => $_has(7);
  @$pb.TagNumber(8)
  void clearEndpoint() => $_clearField(8);
}

class UpdateDestinationResponse extends $pb.GeneratedMessage {
  factory UpdateDestinationResponse({
    Destination? destination,
  }) {
    final result = UpdateDestinationResponse._();
    if (destination != null) result.destination = destination;
    return result;
  }

  UpdateDestinationResponse._();

  factory UpdateDestinationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDestinationResponse()..mergeFromBuffer(data, registry);
  factory UpdateDestinationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateDestinationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateDestinationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateDestinationResponse.$_createMessage)
    ..aOM<Destination>(1, _omitFieldNames ? '' : 'destination',
        subBuilder: Destination.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDestinationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateDestinationResponse copyWith(
          void Function(UpdateDestinationResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateDestinationResponse))
          as UpdateDestinationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateDestinationResponse() / UpdateDestinationResponse.new instead')
  static UpdateDestinationResponse create() => UpdateDestinationResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      UpdateDestinationResponse._();
  @$core.override
  UpdateDestinationResponse createEmptyInstance() =>
      UpdateDestinationResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateDestinationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateDestinationResponse>(
          UpdateDestinationResponse.$_createMessage);
  static UpdateDestinationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Destination get destination => $_getN(0);
  @$pb.TagNumber(1)
  set destination(Destination value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDestination() => $_has(0);
  @$pb.TagNumber(1)
  void clearDestination() => $_clearField(1);
  @$pb.TagNumber(1)
  Destination ensureDestination() => $_ensure(0);
}

class DeleteDestinationRequest extends $pb.GeneratedMessage {
  factory DeleteDestinationRequest({
    $core.String? id,
  }) {
    final result = DeleteDestinationRequest._();
    if (id != null) result.id = id;
    return result;
  }

  DeleteDestinationRequest._();

  factory DeleteDestinationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDestinationRequest()..mergeFromBuffer(data, registry);
  factory DeleteDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteDestinationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteDestinationRequest copyWith(
          void Function(DeleteDestinationRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteDestinationRequest))
          as DeleteDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteDestinationRequest() / DeleteDestinationRequest.new instead')
  static DeleteDestinationRequest create() => DeleteDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteDestinationRequest._();
  @$core.override
  DeleteDestinationRequest createEmptyInstance() =>
      DeleteDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteDestinationRequest>(
          DeleteDestinationRequest.$_createMessage);
  static DeleteDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Raw credentials test used before a destination row exists.
class TestDestinationRequest extends $pb.GeneratedMessage {
  factory TestDestinationRequest({
    $core.String? provider,
    $core.String? accessKey,
    $core.String? secretAccessKey,
    $core.String? bucket,
    $core.String? region,
    $core.String? endpoint,
  }) {
    final result = TestDestinationRequest._();
    if (provider != null) result.provider = provider;
    if (accessKey != null) result.accessKey = accessKey;
    if (secretAccessKey != null) result.secretAccessKey = secretAccessKey;
    if (bucket != null) result.bucket = bucket;
    if (region != null) result.region = region;
    if (endpoint != null) result.endpoint = endpoint;
    return result;
  }

  TestDestinationRequest._();

  factory TestDestinationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDestinationRequest()..mergeFromBuffer(data, registry);
  factory TestDestinationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDestinationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDestinationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDestinationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'provider')
    ..aOS(2, _omitFieldNames ? '' : 'accessKey')
    ..aOS(3, _omitFieldNames ? '' : 'secretAccessKey')
    ..aOS(4, _omitFieldNames ? '' : 'bucket')
    ..aOS(5, _omitFieldNames ? '' : 'region')
    ..aOS(6, _omitFieldNames ? '' : 'endpoint')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDestinationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDestinationRequest copyWith(
          void Function(TestDestinationRequest) updates) =>
      super.copyWith((message) => updates(message as TestDestinationRequest))
          as TestDestinationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDestinationRequest() / TestDestinationRequest.new instead')
  static TestDestinationRequest create() => TestDestinationRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestDestinationRequest._();
  @$core.override
  TestDestinationRequest createEmptyInstance() => TestDestinationRequest._();
  @$core.pragma('dart2js:noInline')
  static TestDestinationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDestinationRequest>(
          TestDestinationRequest.$_createMessage);
  static TestDestinationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get provider => $_getSZ(0);
  @$pb.TagNumber(1)
  set provider($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get accessKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set accessKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAccessKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccessKey() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get secretAccessKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set secretAccessKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSecretAccessKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearSecretAccessKey() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get bucket => $_getSZ(3);
  @$pb.TagNumber(4)
  set bucket($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBucket() => $_has(3);
  @$pb.TagNumber(4)
  void clearBucket() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get region => $_getSZ(4);
  @$pb.TagNumber(5)
  set region($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRegion() => $_has(4);
  @$pb.TagNumber(5)
  void clearRegion() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get endpoint => $_getSZ(5);
  @$pb.TagNumber(6)
  set endpoint($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasEndpoint() => $_has(5);
  @$pb.TagNumber(6)
  void clearEndpoint() => $_clearField(6);
}

class TestDestinationResponse extends $pb.GeneratedMessage {
  factory TestDestinationResponse({
    $core.bool? success,
  }) {
    final result = TestDestinationResponse._();
    if (success != null) result.success = success;
    return result;
  }

  TestDestinationResponse._();

  factory TestDestinationResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDestinationResponse()..mergeFromBuffer(data, registry);
  factory TestDestinationResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestDestinationResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestDestinationResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestDestinationResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDestinationResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestDestinationResponse copyWith(
          void Function(TestDestinationResponse) updates) =>
      super.copyWith((message) => updates(message as TestDestinationResponse))
          as TestDestinationResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TestDestinationResponse() / TestDestinationResponse.new instead')
  static TestDestinationResponse create() => TestDestinationResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestDestinationResponse._();
  @$core.override
  TestDestinationResponse createEmptyInstance() => TestDestinationResponse._();
  @$core.pragma('dart2js:noInline')
  static TestDestinationResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestDestinationResponse>(
          TestDestinationResponse.$_createMessage);
  static TestDestinationResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ssh_key.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'ssh_key.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'ssh_key.pbenum.dart';

class SshKey extends $pb.GeneratedMessage {
  factory SshKey({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? publicKey,
    $core.String? privateKey,
    $core.bool? hasPrivateKey_6,
    $fixnum.Int64? lastUsedAt,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? updatedAt,
  }) {
    final result = SshKey._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (publicKey != null) result.publicKey = publicKey;
    if (privateKey != null) result.privateKey = privateKey;
    if (hasPrivateKey_6 != null) result.hasPrivateKey_6 = hasPrivateKey_6;
    if (lastUsedAt != null) result.lastUsedAt = lastUsedAt;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  SshKey._();

  factory SshKey.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKey()..mergeFromBuffer(data, registry);
  factory SshKey.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKey()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKey',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKey.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'publicKey')
    ..aOS(5, _omitFieldNames ? '' : 'privateKey')
    ..aOB(6, _omitFieldNames ? '' : 'hasPrivateKey')
    ..aInt64(7, _omitFieldNames ? '' : 'lastUsedAt')
    ..aInt64(8, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(9, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKey clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKey copyWith(void Function(SshKey) updates) =>
      super.copyWith((message) => updates(message as SshKey)) as SshKey;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SshKey() / SshKey.new instead')
  static SshKey create() => SshKey._();
  static $pb.GeneratedMessage $_createMessage() => SshKey._();
  @$core.override
  SshKey createEmptyInstance() => SshKey._();
  @$core.pragma('dart2js:noInline')
  static SshKey getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKey>(SshKey.$_createMessage);
  static SshKey? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  /// OpenSSH public key line.
  @$pb.TagNumber(4)
  $core.String get publicKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set publicKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPublicKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearPublicKey() => $_clearField(4);

  /// OpenSSH/PEM private key material; present when a key pair is stored.
  @$pb.TagNumber(5)
  $core.String get privateKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set privateKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPrivateKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearPrivateKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get hasPrivateKey_6 => $_getBF(5);
  @$pb.TagNumber(6)
  set hasPrivateKey_6($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHasPrivateKey_6() => $_has(5);
  @$pb.TagNumber(6)
  void clearHasPrivateKey_6() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get lastUsedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set lastUsedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasLastUsedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearLastUsedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get createdAt => $_getI64(7);
  @$pb.TagNumber(8)
  set createdAt($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCreatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedAt() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get updatedAt => $_getI64(8);
  @$pb.TagNumber(9)
  set updatedAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUpdatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearUpdatedAt() => $_clearField(9);
}

class SshKeyListKeysRequest extends $pb.GeneratedMessage {
  factory SshKeyListKeysRequest() => SshKeyListKeysRequest._();

  SshKeyListKeysRequest._();

  factory SshKeyListKeysRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyListKeysRequest()..mergeFromBuffer(data, registry);
  factory SshKeyListKeysRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyListKeysRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyListKeysRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyListKeysRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyListKeysRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyListKeysRequest copyWith(
          void Function(SshKeyListKeysRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyListKeysRequest))
          as SshKeyListKeysRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyListKeysRequest() / SshKeyListKeysRequest.new instead')
  static SshKeyListKeysRequest create() => SshKeyListKeysRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyListKeysRequest._();
  @$core.override
  SshKeyListKeysRequest createEmptyInstance() => SshKeyListKeysRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyListKeysRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyListKeysRequest>(
          SshKeyListKeysRequest.$_createMessage);
  static SshKeyListKeysRequest? _defaultInstance;
}

class SshKeyListKeysResponse extends $pb.GeneratedMessage {
  factory SshKeyListKeysResponse({
    $core.Iterable<SshKey>? keys,
  }) {
    final result = SshKeyListKeysResponse._();
    if (keys != null) result.keys.addAll(keys);
    return result;
  }

  SshKeyListKeysResponse._();

  factory SshKeyListKeysResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyListKeysResponse()..mergeFromBuffer(data, registry);
  factory SshKeyListKeysResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyListKeysResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyListKeysResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyListKeysResponse.$_createMessage)
    ..pPM<SshKey>(1, _omitFieldNames ? '' : 'keys',
        subBuilder: SshKey.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyListKeysResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyListKeysResponse copyWith(
          void Function(SshKeyListKeysResponse) updates) =>
      super.copyWith((message) => updates(message as SshKeyListKeysResponse))
          as SshKeyListKeysResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyListKeysResponse() / SshKeyListKeysResponse.new instead')
  static SshKeyListKeysResponse create() => SshKeyListKeysResponse._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyListKeysResponse._();
  @$core.override
  SshKeyListKeysResponse createEmptyInstance() => SshKeyListKeysResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyListKeysResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyListKeysResponse>(
          SshKeyListKeysResponse.$_createMessage);
  static SshKeyListKeysResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SshKey> get keys => $_getList(0);
}

class SshKeyGetKeyRequest extends $pb.GeneratedMessage {
  factory SshKeyGetKeyRequest({
    $fixnum.Int64? id,
  }) {
    final result = SshKeyGetKeyRequest._();
    if (id != null) result.id = id;
    return result;
  }

  SshKeyGetKeyRequest._();

  factory SshKeyGetKeyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGetKeyRequest()..mergeFromBuffer(data, registry);
  factory SshKeyGetKeyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGetKeyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGetKeyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGetKeyRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGetKeyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGetKeyRequest copyWith(void Function(SshKeyGetKeyRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyGetKeyRequest))
          as SshKeyGetKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use SshKeyGetKeyRequest() / SshKeyGetKeyRequest.new instead')
  static SshKeyGetKeyRequest create() => SshKeyGetKeyRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyGetKeyRequest._();
  @$core.override
  SshKeyGetKeyRequest createEmptyInstance() => SshKeyGetKeyRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGetKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGetKeyRequest>(
          SshKeyGetKeyRequest.$_createMessage);
  static SshKeyGetKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class SshKeyGetKeyResponse extends $pb.GeneratedMessage {
  factory SshKeyGetKeyResponse({
    SshKey? key,
  }) {
    final result = SshKeyGetKeyResponse._();
    if (key != null) result.key = key;
    return result;
  }

  SshKeyGetKeyResponse._();

  factory SshKeyGetKeyResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGetKeyResponse()..mergeFromBuffer(data, registry);
  factory SshKeyGetKeyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGetKeyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGetKeyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGetKeyResponse.$_createMessage)
    ..aOM<SshKey>(1, _omitFieldNames ? '' : 'key',
        subBuilder: SshKey.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGetKeyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGetKeyResponse copyWith(void Function(SshKeyGetKeyResponse) updates) =>
      super.copyWith((message) => updates(message as SshKeyGetKeyResponse))
          as SshKeyGetKeyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyGetKeyResponse() / SshKeyGetKeyResponse.new instead')
  static SshKeyGetKeyResponse create() => SshKeyGetKeyResponse._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyGetKeyResponse._();
  @$core.override
  SshKeyGetKeyResponse createEmptyInstance() => SshKeyGetKeyResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGetKeyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGetKeyResponse>(
          SshKeyGetKeyResponse.$_createMessage);
  static SshKeyGetKeyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SshKey get key => $_getN(0);
  @$pb.TagNumber(1)
  set key(SshKey value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => $_clearField(1);
  @$pb.TagNumber(1)
  SshKey ensureKey() => $_ensure(0);
}

class SshKeyCreateKeyRequest extends $pb.GeneratedMessage {
  factory SshKeyCreateKeyRequest({
    $core.String? name,
    $core.String? description,
    $core.String? publicKey,
    $core.String? privateKey,
  }) {
    final result = SshKeyCreateKeyRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (publicKey != null) result.publicKey = publicKey;
    if (privateKey != null) result.privateKey = privateKey;
    return result;
  }

  SshKeyCreateKeyRequest._();

  factory SshKeyCreateKeyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyCreateKeyRequest()..mergeFromBuffer(data, registry);
  factory SshKeyCreateKeyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyCreateKeyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyCreateKeyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyCreateKeyRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aOS(3, _omitFieldNames ? '' : 'publicKey')
    ..aOS(4, _omitFieldNames ? '' : 'privateKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyCreateKeyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyCreateKeyRequest copyWith(
          void Function(SshKeyCreateKeyRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyCreateKeyRequest))
          as SshKeyCreateKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyCreateKeyRequest() / SshKeyCreateKeyRequest.new instead')
  static SshKeyCreateKeyRequest create() => SshKeyCreateKeyRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyCreateKeyRequest._();
  @$core.override
  SshKeyCreateKeyRequest createEmptyInstance() => SshKeyCreateKeyRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyCreateKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyCreateKeyRequest>(
          SshKeyCreateKeyRequest.$_createMessage);
  static SshKeyCreateKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  /// OpenSSH public key line.
  @$pb.TagNumber(3)
  $core.String get publicKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set publicKey($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPublicKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearPublicKey() => $_clearField(3);

  /// Optional OpenSSH/PEM private key material; empty means no private key.
  @$pb.TagNumber(4)
  $core.String get privateKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set privateKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPrivateKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearPrivateKey() => $_clearField(4);
}

class SshKeyCreateKeyResponse extends $pb.GeneratedMessage {
  factory SshKeyCreateKeyResponse({
    SshKey? key,
  }) {
    final result = SshKeyCreateKeyResponse._();
    if (key != null) result.key = key;
    return result;
  }

  SshKeyCreateKeyResponse._();

  factory SshKeyCreateKeyResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyCreateKeyResponse()..mergeFromBuffer(data, registry);
  factory SshKeyCreateKeyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyCreateKeyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyCreateKeyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyCreateKeyResponse.$_createMessage)
    ..aOM<SshKey>(1, _omitFieldNames ? '' : 'key',
        subBuilder: SshKey.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyCreateKeyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyCreateKeyResponse copyWith(
          void Function(SshKeyCreateKeyResponse) updates) =>
      super.copyWith((message) => updates(message as SshKeyCreateKeyResponse))
          as SshKeyCreateKeyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyCreateKeyResponse() / SshKeyCreateKeyResponse.new instead')
  static SshKeyCreateKeyResponse create() => SshKeyCreateKeyResponse._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyCreateKeyResponse._();
  @$core.override
  SshKeyCreateKeyResponse createEmptyInstance() => SshKeyCreateKeyResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyCreateKeyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyCreateKeyResponse>(
          SshKeyCreateKeyResponse.$_createMessage);
  static SshKeyCreateKeyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SshKey get key => $_getN(0);
  @$pb.TagNumber(1)
  set key(SshKey value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => $_clearField(1);
  @$pb.TagNumber(1)
  SshKey ensureKey() => $_ensure(0);
}

class SshKeyGenerateKeyRequest extends $pb.GeneratedMessage {
  factory SshKeyGenerateKeyRequest({
    $core.String? name,
    $core.String? description,
    SshKeyType? keyType,
  }) {
    final result = SshKeyGenerateKeyRequest._();
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (keyType != null) result.keyType = keyType;
    return result;
  }

  SshKeyGenerateKeyRequest._();

  factory SshKeyGenerateKeyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGenerateKeyRequest()..mergeFromBuffer(data, registry);
  factory SshKeyGenerateKeyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGenerateKeyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGenerateKeyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGenerateKeyRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'description')
    ..aE<SshKeyType>(3, _omitFieldNames ? '' : 'keyType',
        enumValues: SshKeyType.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGenerateKeyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGenerateKeyRequest copyWith(
          void Function(SshKeyGenerateKeyRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyGenerateKeyRequest))
          as SshKeyGenerateKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyGenerateKeyRequest() / SshKeyGenerateKeyRequest.new instead')
  static SshKeyGenerateKeyRequest create() => SshKeyGenerateKeyRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyGenerateKeyRequest._();
  @$core.override
  SshKeyGenerateKeyRequest createEmptyInstance() =>
      SshKeyGenerateKeyRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGenerateKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGenerateKeyRequest>(
          SshKeyGenerateKeyRequest.$_createMessage);
  static SshKeyGenerateKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get description => $_getSZ(1);
  @$pb.TagNumber(2)
  set description($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDescription() => $_has(1);
  @$pb.TagNumber(2)
  void clearDescription() => $_clearField(2);

  @$pb.TagNumber(3)
  SshKeyType get keyType => $_getN(2);
  @$pb.TagNumber(3)
  set keyType(SshKeyType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasKeyType() => $_has(2);
  @$pb.TagNumber(3)
  void clearKeyType() => $_clearField(3);
}

class SshKeyGenerateKeyResponse extends $pb.GeneratedMessage {
  factory SshKeyGenerateKeyResponse({
    SshKey? key,
  }) {
    final result = SshKeyGenerateKeyResponse._();
    if (key != null) result.key = key;
    return result;
  }

  SshKeyGenerateKeyResponse._();

  factory SshKeyGenerateKeyResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGenerateKeyResponse()..mergeFromBuffer(data, registry);
  factory SshKeyGenerateKeyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGenerateKeyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGenerateKeyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGenerateKeyResponse.$_createMessage)
    ..aOM<SshKey>(1, _omitFieldNames ? '' : 'key',
        subBuilder: SshKey.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGenerateKeyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGenerateKeyResponse copyWith(
          void Function(SshKeyGenerateKeyResponse) updates) =>
      super.copyWith((message) => updates(message as SshKeyGenerateKeyResponse))
          as SshKeyGenerateKeyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyGenerateKeyResponse() / SshKeyGenerateKeyResponse.new instead')
  static SshKeyGenerateKeyResponse create() => SshKeyGenerateKeyResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SshKeyGenerateKeyResponse._();
  @$core.override
  SshKeyGenerateKeyResponse createEmptyInstance() =>
      SshKeyGenerateKeyResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGenerateKeyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGenerateKeyResponse>(
          SshKeyGenerateKeyResponse.$_createMessage);
  static SshKeyGenerateKeyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SshKey get key => $_getN(0);
  @$pb.TagNumber(1)
  set key(SshKey value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => $_clearField(1);
  @$pb.TagNumber(1)
  SshKey ensureKey() => $_ensure(0);
}

/// Omit `key_type` to use the server default (ed25519).
class SshKeyGeneratePairRequest extends $pb.GeneratedMessage {
  factory SshKeyGeneratePairRequest({
    SshKeyType? keyType,
  }) {
    final result = SshKeyGeneratePairRequest._();
    if (keyType != null) result.keyType = keyType;
    return result;
  }

  SshKeyGeneratePairRequest._();

  factory SshKeyGeneratePairRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGeneratePairRequest()..mergeFromBuffer(data, registry);
  factory SshKeyGeneratePairRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGeneratePairRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGeneratePairRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGeneratePairRequest.$_createMessage)
    ..aE<SshKeyType>(1, _omitFieldNames ? '' : 'keyType',
        enumValues: SshKeyType.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGeneratePairRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGeneratePairRequest copyWith(
          void Function(SshKeyGeneratePairRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyGeneratePairRequest))
          as SshKeyGeneratePairRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyGeneratePairRequest() / SshKeyGeneratePairRequest.new instead')
  static SshKeyGeneratePairRequest create() => SshKeyGeneratePairRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      SshKeyGeneratePairRequest._();
  @$core.override
  SshKeyGeneratePairRequest createEmptyInstance() =>
      SshKeyGeneratePairRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGeneratePairRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGeneratePairRequest>(
          SshKeyGeneratePairRequest.$_createMessage);
  static SshKeyGeneratePairRequest? _defaultInstance;

  @$pb.TagNumber(1)
  SshKeyType get keyType => $_getN(0);
  @$pb.TagNumber(1)
  set keyType(SshKeyType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKeyType() => $_has(0);
  @$pb.TagNumber(1)
  void clearKeyType() => $_clearField(1);
}

class SshKeyGeneratePairResponse extends $pb.GeneratedMessage {
  factory SshKeyGeneratePairResponse({
    $core.String? privateKey,
    $core.String? publicKey,
  }) {
    final result = SshKeyGeneratePairResponse._();
    if (privateKey != null) result.privateKey = privateKey;
    if (publicKey != null) result.publicKey = publicKey;
    return result;
  }

  SshKeyGeneratePairResponse._();

  factory SshKeyGeneratePairResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGeneratePairResponse()..mergeFromBuffer(data, registry);
  factory SshKeyGeneratePairResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyGeneratePairResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyGeneratePairResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyGeneratePairResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'privateKey')
    ..aOS(2, _omitFieldNames ? '' : 'publicKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGeneratePairResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyGeneratePairResponse copyWith(
          void Function(SshKeyGeneratePairResponse) updates) =>
      super.copyWith(
              (message) => updates(message as SshKeyGeneratePairResponse))
          as SshKeyGeneratePairResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyGeneratePairResponse() / SshKeyGeneratePairResponse.new instead')
  static SshKeyGeneratePairResponse create() => SshKeyGeneratePairResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      SshKeyGeneratePairResponse._();
  @$core.override
  SshKeyGeneratePairResponse createEmptyInstance() =>
      SshKeyGeneratePairResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyGeneratePairResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyGeneratePairResponse>(
          SshKeyGeneratePairResponse.$_createMessage);
  static SshKeyGeneratePairResponse? _defaultInstance;

  /// Freshly generated OpenSSH/PEM private key material.
  @$pb.TagNumber(1)
  $core.String get privateKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set privateKey($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPrivateKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrivateKey() => $_clearField(1);

  /// Matching OpenSSH public key line.
  @$pb.TagNumber(2)
  $core.String get publicKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set publicKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPublicKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearPublicKey() => $_clearField(2);
}

class SshKeyUpdateKeyRequest extends $pb.GeneratedMessage {
  factory SshKeyUpdateKeyRequest({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? description,
    $core.String? publicKey,
    $core.String? privateKey,
  }) {
    final result = SshKeyUpdateKeyRequest._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (publicKey != null) result.publicKey = publicKey;
    if (privateKey != null) result.privateKey = privateKey;
    return result;
  }

  SshKeyUpdateKeyRequest._();

  factory SshKeyUpdateKeyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyUpdateKeyRequest()..mergeFromBuffer(data, registry);
  factory SshKeyUpdateKeyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyUpdateKeyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyUpdateKeyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyUpdateKeyRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'publicKey')
    ..aOS(5, _omitFieldNames ? '' : 'privateKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyUpdateKeyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyUpdateKeyRequest copyWith(
          void Function(SshKeyUpdateKeyRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyUpdateKeyRequest))
          as SshKeyUpdateKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyUpdateKeyRequest() / SshKeyUpdateKeyRequest.new instead')
  static SshKeyUpdateKeyRequest create() => SshKeyUpdateKeyRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyUpdateKeyRequest._();
  @$core.override
  SshKeyUpdateKeyRequest createEmptyInstance() => SshKeyUpdateKeyRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyUpdateKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyUpdateKeyRequest>(
          SshKeyUpdateKeyRequest.$_createMessage);
  static SshKeyUpdateKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get publicKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set publicKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPublicKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearPublicKey() => $_clearField(4);

  /// OpenSSH/PEM private key material.
  @$pb.TagNumber(5)
  $core.String get privateKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set privateKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPrivateKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearPrivateKey() => $_clearField(5);
}

class SshKeyUpdateKeyResponse extends $pb.GeneratedMessage {
  factory SshKeyUpdateKeyResponse({
    SshKey? key,
  }) {
    final result = SshKeyUpdateKeyResponse._();
    if (key != null) result.key = key;
    return result;
  }

  SshKeyUpdateKeyResponse._();

  factory SshKeyUpdateKeyResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyUpdateKeyResponse()..mergeFromBuffer(data, registry);
  factory SshKeyUpdateKeyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyUpdateKeyResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyUpdateKeyResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyUpdateKeyResponse.$_createMessage)
    ..aOM<SshKey>(1, _omitFieldNames ? '' : 'key',
        subBuilder: SshKey.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyUpdateKeyResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyUpdateKeyResponse copyWith(
          void Function(SshKeyUpdateKeyResponse) updates) =>
      super.copyWith((message) => updates(message as SshKeyUpdateKeyResponse))
          as SshKeyUpdateKeyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyUpdateKeyResponse() / SshKeyUpdateKeyResponse.new instead')
  static SshKeyUpdateKeyResponse create() => SshKeyUpdateKeyResponse._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyUpdateKeyResponse._();
  @$core.override
  SshKeyUpdateKeyResponse createEmptyInstance() => SshKeyUpdateKeyResponse._();
  @$core.pragma('dart2js:noInline')
  static SshKeyUpdateKeyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyUpdateKeyResponse>(
          SshKeyUpdateKeyResponse.$_createMessage);
  static SshKeyUpdateKeyResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SshKey get key => $_getN(0);
  @$pb.TagNumber(1)
  set key(SshKey value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => $_clearField(1);
  @$pb.TagNumber(1)
  SshKey ensureKey() => $_ensure(0);
}

class SshKeyMarkUsedRequest extends $pb.GeneratedMessage {
  factory SshKeyMarkUsedRequest({
    $fixnum.Int64? id,
  }) {
    final result = SshKeyMarkUsedRequest._();
    if (id != null) result.id = id;
    return result;
  }

  SshKeyMarkUsedRequest._();

  factory SshKeyMarkUsedRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyMarkUsedRequest()..mergeFromBuffer(data, registry);
  factory SshKeyMarkUsedRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyMarkUsedRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyMarkUsedRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyMarkUsedRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyMarkUsedRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyMarkUsedRequest copyWith(
          void Function(SshKeyMarkUsedRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyMarkUsedRequest))
          as SshKeyMarkUsedRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyMarkUsedRequest() / SshKeyMarkUsedRequest.new instead')
  static SshKeyMarkUsedRequest create() => SshKeyMarkUsedRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyMarkUsedRequest._();
  @$core.override
  SshKeyMarkUsedRequest createEmptyInstance() => SshKeyMarkUsedRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyMarkUsedRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyMarkUsedRequest>(
          SshKeyMarkUsedRequest.$_createMessage);
  static SshKeyMarkUsedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class SshKeyDeleteKeyRequest extends $pb.GeneratedMessage {
  factory SshKeyDeleteKeyRequest({
    $fixnum.Int64? id,
  }) {
    final result = SshKeyDeleteKeyRequest._();
    if (id != null) result.id = id;
    return result;
  }

  SshKeyDeleteKeyRequest._();

  factory SshKeyDeleteKeyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyDeleteKeyRequest()..mergeFromBuffer(data, registry);
  factory SshKeyDeleteKeyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SshKeyDeleteKeyRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SshKeyDeleteKeyRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SshKeyDeleteKeyRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyDeleteKeyRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SshKeyDeleteKeyRequest copyWith(
          void Function(SshKeyDeleteKeyRequest) updates) =>
      super.copyWith((message) => updates(message as SshKeyDeleteKeyRequest))
          as SshKeyDeleteKeyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SshKeyDeleteKeyRequest() / SshKeyDeleteKeyRequest.new instead')
  static SshKeyDeleteKeyRequest create() => SshKeyDeleteKeyRequest._();
  static $pb.GeneratedMessage $_createMessage() => SshKeyDeleteKeyRequest._();
  @$core.override
  SshKeyDeleteKeyRequest createEmptyInstance() => SshKeyDeleteKeyRequest._();
  @$core.pragma('dart2js:noInline')
  static SshKeyDeleteKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SshKeyDeleteKeyRequest>(
          SshKeyDeleteKeyRequest.$_createMessage);
  static SshKeyDeleteKeyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

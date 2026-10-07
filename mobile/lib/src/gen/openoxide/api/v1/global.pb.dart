// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/global.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'global.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'global.pbenum.dart';

/// `docker ps` projection; every column arrives as the CLI-formatted string.
class GlobalContainer extends $pb.GeneratedMessage {
  factory GlobalContainer({
    $core.String? id,
    $core.String? names,
    $core.String? image,
    $core.String? command,
    $core.String? createdAt,
    $core.String? runningFor,
    $core.String? ports,
    $core.String? status,
    $core.String? state,
    $core.String? size,
    $core.String? labels,
    $core.String? mounts,
    $core.String? networks,
  }) {
    final result = GlobalContainer._();
    if (id != null) result.id = id;
    if (names != null) result.names = names;
    if (image != null) result.image = image;
    if (command != null) result.command = command;
    if (createdAt != null) result.createdAt = createdAt;
    if (runningFor != null) result.runningFor = runningFor;
    if (ports != null) result.ports = ports;
    if (status != null) result.status = status;
    if (state != null) result.state = state;
    if (size != null) result.size = size;
    if (labels != null) result.labels = labels;
    if (mounts != null) result.mounts = mounts;
    if (networks != null) result.networks = networks;
    return result;
  }

  GlobalContainer._();

  factory GlobalContainer.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainer()..mergeFromBuffer(data, registry);
  factory GlobalContainer.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainer()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalContainer',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalContainer.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'names')
    ..aOS(3, _omitFieldNames ? '' : 'image')
    ..aOS(4, _omitFieldNames ? '' : 'command')
    ..aOS(5, _omitFieldNames ? '' : 'createdAt')
    ..aOS(6, _omitFieldNames ? '' : 'runningFor')
    ..aOS(7, _omitFieldNames ? '' : 'ports')
    ..aOS(8, _omitFieldNames ? '' : 'status')
    ..aOS(9, _omitFieldNames ? '' : 'state')
    ..aOS(10, _omitFieldNames ? '' : 'size')
    ..aOS(11, _omitFieldNames ? '' : 'labels')
    ..aOS(12, _omitFieldNames ? '' : 'mounts')
    ..aOS(13, _omitFieldNames ? '' : 'networks')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainer clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainer copyWith(void Function(GlobalContainer) updates) =>
      super.copyWith((message) => updates(message as GlobalContainer))
          as GlobalContainer;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalContainer() / GlobalContainer.new instead')
  static GlobalContainer create() => GlobalContainer._();
  static $pb.GeneratedMessage $_createMessage() => GlobalContainer._();
  @$core.override
  GlobalContainer createEmptyInstance() => GlobalContainer._();
  @$core.pragma('dart2js:noInline')
  static GlobalContainer getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GlobalContainer>(
          GlobalContainer.$_createMessage);
  static GlobalContainer? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get names => $_getSZ(1);
  @$pb.TagNumber(2)
  set names($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNames() => $_has(1);
  @$pb.TagNumber(2)
  void clearNames() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get image => $_getSZ(2);
  @$pb.TagNumber(3)
  set image($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasImage() => $_has(2);
  @$pb.TagNumber(3)
  void clearImage() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get command => $_getSZ(3);
  @$pb.TagNumber(4)
  set command($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCommand() => $_has(3);
  @$pb.TagNumber(4)
  void clearCommand() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get createdAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set createdAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get runningFor => $_getSZ(5);
  @$pb.TagNumber(6)
  set runningFor($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRunningFor() => $_has(5);
  @$pb.TagNumber(6)
  void clearRunningFor() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get ports => $_getSZ(6);
  @$pb.TagNumber(7)
  set ports($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasPorts() => $_has(6);
  @$pb.TagNumber(7)
  void clearPorts() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get status => $_getSZ(7);
  @$pb.TagNumber(8)
  set status($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasStatus() => $_has(7);
  @$pb.TagNumber(8)
  void clearStatus() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get state => $_getSZ(8);
  @$pb.TagNumber(9)
  set state($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasState() => $_has(8);
  @$pb.TagNumber(9)
  void clearState() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get size => $_getSZ(9);
  @$pb.TagNumber(10)
  set size($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasSize() => $_has(9);
  @$pb.TagNumber(10)
  void clearSize() => $_clearField(10);

  /// Labels as the CLI-formatted string, not a JSON document.
  @$pb.TagNumber(11)
  $core.String get labels => $_getSZ(10);
  @$pb.TagNumber(11)
  set labels($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLabels() => $_has(10);
  @$pb.TagNumber(11)
  void clearLabels() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get mounts => $_getSZ(11);
  @$pb.TagNumber(12)
  set mounts($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasMounts() => $_has(11);
  @$pb.TagNumber(12)
  void clearMounts() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get networks => $_getSZ(12);
  @$pb.TagNumber(13)
  set networks($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasNetworks() => $_has(12);
  @$pb.TagNumber(13)
  void clearNetworks() => $_clearField(13);
}

/// `docker images` projection; every column is a CLI-formatted string.
class GlobalImage extends $pb.GeneratedMessage {
  factory GlobalImage({
    $core.String? id,
    $core.String? repository,
    $core.String? tag,
    $core.String? digest,
    $core.String? createdAt,
    $core.String? createdSince,
    $core.String? size,
    $core.String? sharedSize,
    $core.String? uniqueSize,
    $core.String? containers,
  }) {
    final result = GlobalImage._();
    if (id != null) result.id = id;
    if (repository != null) result.repository = repository;
    if (tag != null) result.tag = tag;
    if (digest != null) result.digest = digest;
    if (createdAt != null) result.createdAt = createdAt;
    if (createdSince != null) result.createdSince = createdSince;
    if (size != null) result.size = size;
    if (sharedSize != null) result.sharedSize = sharedSize;
    if (uniqueSize != null) result.uniqueSize = uniqueSize;
    if (containers != null) result.containers = containers;
    return result;
  }

  GlobalImage._();

  factory GlobalImage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalImage()..mergeFromBuffer(data, registry);
  factory GlobalImage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalImage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalImage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalImage.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'repository')
    ..aOS(3, _omitFieldNames ? '' : 'tag')
    ..aOS(4, _omitFieldNames ? '' : 'digest')
    ..aOS(5, _omitFieldNames ? '' : 'createdAt')
    ..aOS(6, _omitFieldNames ? '' : 'createdSince')
    ..aOS(7, _omitFieldNames ? '' : 'size')
    ..aOS(8, _omitFieldNames ? '' : 'sharedSize')
    ..aOS(9, _omitFieldNames ? '' : 'uniqueSize')
    ..aOS(10, _omitFieldNames ? '' : 'containers')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalImage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalImage copyWith(void Function(GlobalImage) updates) =>
      super.copyWith((message) => updates(message as GlobalImage))
          as GlobalImage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalImage() / GlobalImage.new instead')
  static GlobalImage create() => GlobalImage._();
  static $pb.GeneratedMessage $_createMessage() => GlobalImage._();
  @$core.override
  GlobalImage createEmptyInstance() => GlobalImage._();
  @$core.pragma('dart2js:noInline')
  static GlobalImage getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GlobalImage>(
          GlobalImage.$_createMessage);
  static GlobalImage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get repository => $_getSZ(1);
  @$pb.TagNumber(2)
  set repository($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRepository() => $_has(1);
  @$pb.TagNumber(2)
  void clearRepository() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get tag => $_getSZ(2);
  @$pb.TagNumber(3)
  set tag($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTag() => $_has(2);
  @$pb.TagNumber(3)
  void clearTag() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get digest => $_getSZ(3);
  @$pb.TagNumber(4)
  set digest($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDigest() => $_has(3);
  @$pb.TagNumber(4)
  void clearDigest() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get createdAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set createdAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCreatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get createdSince => $_getSZ(5);
  @$pb.TagNumber(6)
  set createdSince($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCreatedSince() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedSince() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get size => $_getSZ(6);
  @$pb.TagNumber(7)
  set size($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSize() => $_has(6);
  @$pb.TagNumber(7)
  void clearSize() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get sharedSize => $_getSZ(7);
  @$pb.TagNumber(8)
  set sharedSize($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasSharedSize() => $_has(7);
  @$pb.TagNumber(8)
  void clearSharedSize() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get uniqueSize => $_getSZ(8);
  @$pb.TagNumber(9)
  set uniqueSize($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUniqueSize() => $_has(8);
  @$pb.TagNumber(9)
  void clearUniqueSize() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get containers => $_getSZ(9);
  @$pb.TagNumber(10)
  set containers($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasContainers() => $_has(9);
  @$pb.TagNumber(10)
  void clearContainers() => $_clearField(10);
}

/// `docker network ls` projection; every column is a CLI-formatted string.
class GlobalNetwork extends $pb.GeneratedMessage {
  factory GlobalNetwork({
    $core.String? id,
    $core.String? name,
    $core.String? driver,
    $core.String? scope,
    $core.String? ipv6,
    $core.String? internal,
    $core.String? labels,
    $core.String? createdAt,
  }) {
    final result = GlobalNetwork._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (driver != null) result.driver = driver;
    if (scope != null) result.scope = scope;
    if (ipv6 != null) result.ipv6 = ipv6;
    if (internal != null) result.internal = internal;
    if (labels != null) result.labels = labels;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  GlobalNetwork._();

  factory GlobalNetwork.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalNetwork()..mergeFromBuffer(data, registry);
  factory GlobalNetwork.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalNetwork()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalNetwork',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalNetwork.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'driver')
    ..aOS(4, _omitFieldNames ? '' : 'scope')
    ..aOS(5, _omitFieldNames ? '' : 'ipv6')
    ..aOS(6, _omitFieldNames ? '' : 'internal')
    ..aOS(7, _omitFieldNames ? '' : 'labels')
    ..aOS(8, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalNetwork clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalNetwork copyWith(void Function(GlobalNetwork) updates) =>
      super.copyWith((message) => updates(message as GlobalNetwork))
          as GlobalNetwork;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalNetwork() / GlobalNetwork.new instead')
  static GlobalNetwork create() => GlobalNetwork._();
  static $pb.GeneratedMessage $_createMessage() => GlobalNetwork._();
  @$core.override
  GlobalNetwork createEmptyInstance() => GlobalNetwork._();
  @$core.pragma('dart2js:noInline')
  static GlobalNetwork getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GlobalNetwork>(
          GlobalNetwork.$_createMessage);
  static GlobalNetwork? _defaultInstance;

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
  $core.String get driver => $_getSZ(2);
  @$pb.TagNumber(3)
  set driver($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDriver() => $_has(2);
  @$pb.TagNumber(3)
  void clearDriver() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get scope => $_getSZ(3);
  @$pb.TagNumber(4)
  set scope($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasScope() => $_has(3);
  @$pb.TagNumber(4)
  void clearScope() => $_clearField(4);

  /// i64-as-bool rendered by the CLI ("true"/"false" strings).
  @$pb.TagNumber(5)
  $core.String get ipv6 => $_getSZ(4);
  @$pb.TagNumber(5)
  set ipv6($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIpv6() => $_has(4);
  @$pb.TagNumber(5)
  void clearIpv6() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get internal => $_getSZ(5);
  @$pb.TagNumber(6)
  set internal($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasInternal() => $_has(5);
  @$pb.TagNumber(6)
  void clearInternal() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get labels => $_getSZ(6);
  @$pb.TagNumber(7)
  set labels($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasLabels() => $_has(6);
  @$pb.TagNumber(7)
  void clearLabels() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get createdAt => $_getSZ(7);
  @$pb.TagNumber(8)
  set createdAt($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCreatedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedAt() => $_clearField(8);
}

/// `docker volume ls` projection; every column is a CLI-formatted string.
class GlobalVolume extends $pb.GeneratedMessage {
  factory GlobalVolume({
    $core.String? name,
    $core.String? driver,
    $core.String? scope,
    $core.String? mountpoint,
    $core.String? labels,
    $core.String? availability,
    $core.String? group,
    $core.String? links,
    $core.String? size,
  }) {
    final result = GlobalVolume._();
    if (name != null) result.name = name;
    if (driver != null) result.driver = driver;
    if (scope != null) result.scope = scope;
    if (mountpoint != null) result.mountpoint = mountpoint;
    if (labels != null) result.labels = labels;
    if (availability != null) result.availability = availability;
    if (group != null) result.group = group;
    if (links != null) result.links = links;
    if (size != null) result.size = size;
    return result;
  }

  GlobalVolume._();

  factory GlobalVolume.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalVolume()..mergeFromBuffer(data, registry);
  factory GlobalVolume.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalVolume()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalVolume',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalVolume.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'driver')
    ..aOS(3, _omitFieldNames ? '' : 'scope')
    ..aOS(4, _omitFieldNames ? '' : 'mountpoint')
    ..aOS(5, _omitFieldNames ? '' : 'labels')
    ..aOS(6, _omitFieldNames ? '' : 'availability')
    ..aOS(7, _omitFieldNames ? '' : 'group')
    ..aOS(8, _omitFieldNames ? '' : 'links')
    ..aOS(9, _omitFieldNames ? '' : 'size')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalVolume clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalVolume copyWith(void Function(GlobalVolume) updates) =>
      super.copyWith((message) => updates(message as GlobalVolume))
          as GlobalVolume;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalVolume() / GlobalVolume.new instead')
  static GlobalVolume create() => GlobalVolume._();
  static $pb.GeneratedMessage $_createMessage() => GlobalVolume._();
  @$core.override
  GlobalVolume createEmptyInstance() => GlobalVolume._();
  @$core.pragma('dart2js:noInline')
  static GlobalVolume getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GlobalVolume>(
          GlobalVolume.$_createMessage);
  static GlobalVolume? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get driver => $_getSZ(1);
  @$pb.TagNumber(2)
  set driver($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDriver() => $_has(1);
  @$pb.TagNumber(2)
  void clearDriver() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get scope => $_getSZ(2);
  @$pb.TagNumber(3)
  set scope($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasScope() => $_has(2);
  @$pb.TagNumber(3)
  void clearScope() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get mountpoint => $_getSZ(3);
  @$pb.TagNumber(4)
  set mountpoint($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMountpoint() => $_has(3);
  @$pb.TagNumber(4)
  void clearMountpoint() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get labels => $_getSZ(4);
  @$pb.TagNumber(5)
  set labels($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLabels() => $_has(4);
  @$pb.TagNumber(5)
  void clearLabels() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get availability => $_getSZ(5);
  @$pb.TagNumber(6)
  set availability($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAvailability() => $_has(5);
  @$pb.TagNumber(6)
  void clearAvailability() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get group => $_getSZ(6);
  @$pb.TagNumber(7)
  set group($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasGroup() => $_has(6);
  @$pb.TagNumber(7)
  void clearGroup() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get links => $_getSZ(7);
  @$pb.TagNumber(8)
  set links($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasLinks() => $_has(7);
  @$pb.TagNumber(8)
  void clearLinks() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get size => $_getSZ(8);
  @$pb.TagNumber(9)
  set size($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasSize() => $_has(8);
  @$pb.TagNumber(9)
  void clearSize() => $_clearField(9);
}

class GlobalDockerOutput extends $pb.GeneratedMessage {
  factory GlobalDockerOutput({
    $core.String? stdout,
    $core.String? stderr,
  }) {
    final result = GlobalDockerOutput._();
    if (stdout != null) result.stdout = stdout;
    if (stderr != null) result.stderr = stderr;
    return result;
  }

  GlobalDockerOutput._();

  factory GlobalDockerOutput.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDockerOutput()..mergeFromBuffer(data, registry);
  factory GlobalDockerOutput.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDockerOutput()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDockerOutput',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDockerOutput.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'stdout')
    ..aOS(2, _omitFieldNames ? '' : 'stderr')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDockerOutput clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDockerOutput copyWith(void Function(GlobalDockerOutput) updates) =>
      super.copyWith((message) => updates(message as GlobalDockerOutput))
          as GlobalDockerOutput;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalDockerOutput() / GlobalDockerOutput.new instead')
  static GlobalDockerOutput create() => GlobalDockerOutput._();
  static $pb.GeneratedMessage $_createMessage() => GlobalDockerOutput._();
  @$core.override
  GlobalDockerOutput createEmptyInstance() => GlobalDockerOutput._();
  @$core.pragma('dart2js:noInline')
  static GlobalDockerOutput getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDockerOutput>(
          GlobalDockerOutput.$_createMessage);
  static GlobalDockerOutput? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get stdout => $_getSZ(0);
  @$pb.TagNumber(1)
  set stdout($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStdout() => $_has(0);
  @$pb.TagNumber(1)
  void clearStdout() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get stderr => $_getSZ(1);
  @$pb.TagNumber(2)
  set stderr($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStderr() => $_has(1);
  @$pb.TagNumber(2)
  void clearStderr() => $_clearField(2);
}

/// First (and only metadata) chunk of an upload stream; the destination inside the container is joined as `<destination>/<filename>`.
class GlobalUploadFileHeader extends $pb.GeneratedMessage {
  factory GlobalUploadFileHeader({
    $fixnum.Int64? serverId,
    $core.String? containerId,
    $core.String? destination,
    $core.String? filename,
  }) {
    final result = GlobalUploadFileHeader._();
    if (serverId != null) result.serverId = serverId;
    if (containerId != null) result.containerId = containerId;
    if (destination != null) result.destination = destination;
    if (filename != null) result.filename = filename;
    return result;
  }

  GlobalUploadFileHeader._();

  factory GlobalUploadFileHeader.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileHeader()..mergeFromBuffer(data, registry);
  factory GlobalUploadFileHeader.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileHeader()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadFileHeader',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadFileHeader.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'containerId')
    ..aOS(3, _omitFieldNames ? '' : 'destination')
    ..aOS(4, _omitFieldNames ? '' : 'filename')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileHeader clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileHeader copyWith(
          void Function(GlobalUploadFileHeader) updates) =>
      super.copyWith((message) => updates(message as GlobalUploadFileHeader))
          as GlobalUploadFileHeader;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadFileHeader() / GlobalUploadFileHeader.new instead')
  static GlobalUploadFileHeader create() => GlobalUploadFileHeader._();
  static $pb.GeneratedMessage $_createMessage() => GlobalUploadFileHeader._();
  @$core.override
  GlobalUploadFileHeader createEmptyInstance() => GlobalUploadFileHeader._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadFileHeader getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadFileHeader>(
          GlobalUploadFileHeader.$_createMessage);
  static GlobalUploadFileHeader? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get containerId => $_getSZ(1);
  @$pb.TagNumber(2)
  set containerId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContainerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContainerId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get destination => $_getSZ(2);
  @$pb.TagNumber(3)
  set destination($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDestination() => $_has(2);
  @$pb.TagNumber(3)
  void clearDestination() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get filename => $_getSZ(3);
  @$pb.TagNumber(4)
  set filename($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFilename() => $_has(3);
  @$pb.TagNumber(4)
  void clearFilename() => $_clearField(4);
}

enum GlobalUploadFileRequest_Payload { header, chunk, notSet }

class GlobalUploadFileRequest extends $pb.GeneratedMessage {
  factory GlobalUploadFileRequest({
    GlobalUploadFileHeader? header,
    $core.List<$core.int>? chunk,
  }) {
    final result = GlobalUploadFileRequest._();
    if (header != null) result.header = header;
    if (chunk != null) result.chunk = chunk;
    return result;
  }

  GlobalUploadFileRequest._();

  factory GlobalUploadFileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileRequest()..mergeFromBuffer(data, registry);
  factory GlobalUploadFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, GlobalUploadFileRequest_Payload>
      _GlobalUploadFileRequest_PayloadByTag = {
    1: GlobalUploadFileRequest_Payload.header,
    2: GlobalUploadFileRequest_Payload.chunk,
    0: GlobalUploadFileRequest_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadFileRequest.$_createMessage)
    ..oo(0, [1, 2])
    ..aOM<GlobalUploadFileHeader>(1, _omitFieldNames ? '' : 'header',
        subBuilder: GlobalUploadFileHeader.$_createMessage)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'chunk', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileRequest copyWith(
          void Function(GlobalUploadFileRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalUploadFileRequest))
          as GlobalUploadFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadFileRequest() / GlobalUploadFileRequest.new instead')
  static GlobalUploadFileRequest create() => GlobalUploadFileRequest._();
  static $pb.GeneratedMessage $_createMessage() => GlobalUploadFileRequest._();
  @$core.override
  GlobalUploadFileRequest createEmptyInstance() => GlobalUploadFileRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadFileRequest>(
          GlobalUploadFileRequest.$_createMessage);
  static GlobalUploadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  GlobalUploadFileRequest_Payload whichPayload() =>
      _GlobalUploadFileRequest_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  GlobalUploadFileHeader get header => $_getN(0);
  @$pb.TagNumber(1)
  set header(GlobalUploadFileHeader value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHeader() => $_has(0);
  @$pb.TagNumber(1)
  void clearHeader() => $_clearField(1);
  @$pb.TagNumber(1)
  GlobalUploadFileHeader ensureHeader() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get chunk => $_getN(1);
  @$pb.TagNumber(2)
  set chunk($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChunk() => $_has(1);
  @$pb.TagNumber(2)
  void clearChunk() => $_clearField(2);
}

class GlobalUploadFileResponse extends $pb.GeneratedMessage {
  factory GlobalUploadFileResponse({
    $core.bool? success,
    GlobalDockerOutput? output,
  }) {
    final result = GlobalUploadFileResponse._();
    if (success != null) result.success = success;
    if (output != null) result.output = output;
    return result;
  }

  GlobalUploadFileResponse._();

  factory GlobalUploadFileResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileResponse()..mergeFromBuffer(data, registry);
  factory GlobalUploadFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadFileResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadFileResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOM<GlobalDockerOutput>(2, _omitFieldNames ? '' : 'output',
        subBuilder: GlobalDockerOutput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadFileResponse copyWith(
          void Function(GlobalUploadFileResponse) updates) =>
      super.copyWith((message) => updates(message as GlobalUploadFileResponse))
          as GlobalUploadFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadFileResponse() / GlobalUploadFileResponse.new instead')
  static GlobalUploadFileResponse create() => GlobalUploadFileResponse._();
  static $pb.GeneratedMessage $_createMessage() => GlobalUploadFileResponse._();
  @$core.override
  GlobalUploadFileResponse createEmptyInstance() =>
      GlobalUploadFileResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadFileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadFileResponse>(
          GlobalUploadFileResponse.$_createMessage);
  static GlobalUploadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  GlobalDockerOutput get output => $_getN(1);
  @$pb.TagNumber(2)
  set output(GlobalDockerOutput value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOutput() => $_has(1);
  @$pb.TagNumber(2)
  void clearOutput() => $_clearField(2);
  @$pb.TagNumber(2)
  GlobalDockerOutput ensureOutput() => $_ensure(1);
}

class GlobalDownloadFileRequest extends $pb.GeneratedMessage {
  factory GlobalDownloadFileRequest({
    $fixnum.Int64? serverId,
    $core.String? containerId,
    $core.String? path,
  }) {
    final result = GlobalDownloadFileRequest._();
    if (serverId != null) result.serverId = serverId;
    if (containerId != null) result.containerId = containerId;
    if (path != null) result.path = path;
    return result;
  }

  GlobalDownloadFileRequest._();

  factory GlobalDownloadFileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadFileRequest()..mergeFromBuffer(data, registry);
  factory GlobalDownloadFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadFileRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDownloadFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDownloadFileRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'containerId')
    ..aOS(3, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadFileRequest copyWith(
          void Function(GlobalDownloadFileRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalDownloadFileRequest))
          as GlobalDownloadFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDownloadFileRequest() / GlobalDownloadFileRequest.new instead')
  static GlobalDownloadFileRequest create() => GlobalDownloadFileRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalDownloadFileRequest._();
  @$core.override
  GlobalDownloadFileRequest createEmptyInstance() =>
      GlobalDownloadFileRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalDownloadFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDownloadFileRequest>(
          GlobalDownloadFileRequest.$_createMessage);
  static GlobalDownloadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get containerId => $_getSZ(1);
  @$pb.TagNumber(2)
  set containerId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContainerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContainerId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get path => $_getSZ(2);
  @$pb.TagNumber(3)
  set path($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearPath() => $_clearField(3);
}

class GlobalDownloadFileResponse extends $pb.GeneratedMessage {
  factory GlobalDownloadFileResponse({
    $core.List<$core.int>? content,
    $core.String? path,
  }) {
    final result = GlobalDownloadFileResponse._();
    if (content != null) result.content = content;
    if (path != null) result.path = path;
    return result;
  }

  GlobalDownloadFileResponse._();

  factory GlobalDownloadFileResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadFileResponse()..mergeFromBuffer(data, registry);
  factory GlobalDownloadFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadFileResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDownloadFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDownloadFileResponse.$_createMessage)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadFileResponse copyWith(
          void Function(GlobalDownloadFileResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalDownloadFileResponse))
          as GlobalDownloadFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDownloadFileResponse() / GlobalDownloadFileResponse.new instead')
  static GlobalDownloadFileResponse create() => GlobalDownloadFileResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalDownloadFileResponse._();
  @$core.override
  GlobalDownloadFileResponse createEmptyInstance() =>
      GlobalDownloadFileResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalDownloadFileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDownloadFileResponse>(
          GlobalDownloadFileResponse.$_createMessage);
  static GlobalDownloadFileResponse? _defaultInstance;

  /// Raw file bytes; the old handler base64-wrapped them in a JSON payload.
  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);
}

class GlobalUploadContainerFileHeader extends $pb.GeneratedMessage {
  factory GlobalUploadContainerFileHeader({
    $fixnum.Int64? serverId,
    $core.String? containerId,
    $core.String? destination,
    $core.String? filename,
  }) {
    final result = GlobalUploadContainerFileHeader._();
    if (serverId != null) result.serverId = serverId;
    if (containerId != null) result.containerId = containerId;
    if (destination != null) result.destination = destination;
    if (filename != null) result.filename = filename;
    return result;
  }

  GlobalUploadContainerFileHeader._();

  factory GlobalUploadContainerFileHeader.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileHeader()..mergeFromBuffer(data, registry);
  factory GlobalUploadContainerFileHeader.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileHeader()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadContainerFileHeader',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadContainerFileHeader.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'containerId')
    ..aOS(3, _omitFieldNames ? '' : 'destination')
    ..aOS(4, _omitFieldNames ? '' : 'filename')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileHeader clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileHeader copyWith(
          void Function(GlobalUploadContainerFileHeader) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalUploadContainerFileHeader))
          as GlobalUploadContainerFileHeader;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadContainerFileHeader() / GlobalUploadContainerFileHeader.new instead')
  static GlobalUploadContainerFileHeader create() =>
      GlobalUploadContainerFileHeader._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalUploadContainerFileHeader._();
  @$core.override
  GlobalUploadContainerFileHeader createEmptyInstance() =>
      GlobalUploadContainerFileHeader._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadContainerFileHeader getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadContainerFileHeader>(
          GlobalUploadContainerFileHeader.$_createMessage);
  static GlobalUploadContainerFileHeader? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get containerId => $_getSZ(1);
  @$pb.TagNumber(2)
  set containerId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContainerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContainerId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get destination => $_getSZ(2);
  @$pb.TagNumber(3)
  set destination($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDestination() => $_has(2);
  @$pb.TagNumber(3)
  void clearDestination() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get filename => $_getSZ(3);
  @$pb.TagNumber(4)
  set filename($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFilename() => $_has(3);
  @$pb.TagNumber(4)
  void clearFilename() => $_clearField(4);
}

enum GlobalUploadContainerFileRequest_Payload { header, chunk, notSet }

class GlobalUploadContainerFileRequest extends $pb.GeneratedMessage {
  factory GlobalUploadContainerFileRequest({
    GlobalUploadContainerFileHeader? header,
    $core.List<$core.int>? chunk,
  }) {
    final result = GlobalUploadContainerFileRequest._();
    if (header != null) result.header = header;
    if (chunk != null) result.chunk = chunk;
    return result;
  }

  GlobalUploadContainerFileRequest._();

  factory GlobalUploadContainerFileRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileRequest()..mergeFromBuffer(data, registry);
  factory GlobalUploadContainerFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileRequest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, GlobalUploadContainerFileRequest_Payload>
      _GlobalUploadContainerFileRequest_PayloadByTag = {
    1: GlobalUploadContainerFileRequest_Payload.header,
    2: GlobalUploadContainerFileRequest_Payload.chunk,
    0: GlobalUploadContainerFileRequest_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadContainerFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadContainerFileRequest.$_createMessage)
    ..oo(0, [1, 2])
    ..aOM<GlobalUploadContainerFileHeader>(1, _omitFieldNames ? '' : 'header',
        subBuilder: GlobalUploadContainerFileHeader.$_createMessage)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'chunk', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileRequest copyWith(
          void Function(GlobalUploadContainerFileRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalUploadContainerFileRequest))
          as GlobalUploadContainerFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadContainerFileRequest() / GlobalUploadContainerFileRequest.new instead')
  static GlobalUploadContainerFileRequest create() =>
      GlobalUploadContainerFileRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalUploadContainerFileRequest._();
  @$core.override
  GlobalUploadContainerFileRequest createEmptyInstance() =>
      GlobalUploadContainerFileRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadContainerFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadContainerFileRequest>(
          GlobalUploadContainerFileRequest.$_createMessage);
  static GlobalUploadContainerFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  GlobalUploadContainerFileRequest_Payload whichPayload() =>
      _GlobalUploadContainerFileRequest_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  GlobalUploadContainerFileHeader get header => $_getN(0);
  @$pb.TagNumber(1)
  set header(GlobalUploadContainerFileHeader value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHeader() => $_has(0);
  @$pb.TagNumber(1)
  void clearHeader() => $_clearField(1);
  @$pb.TagNumber(1)
  GlobalUploadContainerFileHeader ensureHeader() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get chunk => $_getN(1);
  @$pb.TagNumber(2)
  set chunk($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasChunk() => $_has(1);
  @$pb.TagNumber(2)
  void clearChunk() => $_clearField(2);
}

class GlobalUploadContainerFileResponse extends $pb.GeneratedMessage {
  factory GlobalUploadContainerFileResponse({
    $core.bool? success,
    GlobalDockerOutput? output,
  }) {
    final result = GlobalUploadContainerFileResponse._();
    if (success != null) result.success = success;
    if (output != null) result.output = output;
    return result;
  }

  GlobalUploadContainerFileResponse._();

  factory GlobalUploadContainerFileResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileResponse()..mergeFromBuffer(data, registry);
  factory GlobalUploadContainerFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalUploadContainerFileResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalUploadContainerFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalUploadContainerFileResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOM<GlobalDockerOutput>(2, _omitFieldNames ? '' : 'output',
        subBuilder: GlobalDockerOutput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalUploadContainerFileResponse copyWith(
          void Function(GlobalUploadContainerFileResponse) updates) =>
      super.copyWith((message) =>
              updates(message as GlobalUploadContainerFileResponse))
          as GlobalUploadContainerFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalUploadContainerFileResponse() / GlobalUploadContainerFileResponse.new instead')
  static GlobalUploadContainerFileResponse create() =>
      GlobalUploadContainerFileResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalUploadContainerFileResponse._();
  @$core.override
  GlobalUploadContainerFileResponse createEmptyInstance() =>
      GlobalUploadContainerFileResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalUploadContainerFileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalUploadContainerFileResponse>(
          GlobalUploadContainerFileResponse.$_createMessage);
  static GlobalUploadContainerFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  GlobalDockerOutput get output => $_getN(1);
  @$pb.TagNumber(2)
  set output(GlobalDockerOutput value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOutput() => $_has(1);
  @$pb.TagNumber(2)
  void clearOutput() => $_clearField(2);
  @$pb.TagNumber(2)
  GlobalDockerOutput ensureOutput() => $_ensure(1);
}

class GlobalDownloadContainerFileRequest extends $pb.GeneratedMessage {
  factory GlobalDownloadContainerFileRequest({
    $fixnum.Int64? serverId,
    $core.String? containerId,
    $core.String? path,
  }) {
    final result = GlobalDownloadContainerFileRequest._();
    if (serverId != null) result.serverId = serverId;
    if (containerId != null) result.containerId = containerId;
    if (path != null) result.path = path;
    return result;
  }

  GlobalDownloadContainerFileRequest._();

  factory GlobalDownloadContainerFileRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadContainerFileRequest()..mergeFromBuffer(data, registry);
  factory GlobalDownloadContainerFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadContainerFileRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDownloadContainerFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDownloadContainerFileRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'containerId')
    ..aOS(3, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadContainerFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadContainerFileRequest copyWith(
          void Function(GlobalDownloadContainerFileRequest) updates) =>
      super.copyWith((message) =>
              updates(message as GlobalDownloadContainerFileRequest))
          as GlobalDownloadContainerFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDownloadContainerFileRequest() / GlobalDownloadContainerFileRequest.new instead')
  static GlobalDownloadContainerFileRequest create() =>
      GlobalDownloadContainerFileRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalDownloadContainerFileRequest._();
  @$core.override
  GlobalDownloadContainerFileRequest createEmptyInstance() =>
      GlobalDownloadContainerFileRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalDownloadContainerFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDownloadContainerFileRequest>(
          GlobalDownloadContainerFileRequest.$_createMessage);
  static GlobalDownloadContainerFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get containerId => $_getSZ(1);
  @$pb.TagNumber(2)
  set containerId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContainerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContainerId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get path => $_getSZ(2);
  @$pb.TagNumber(3)
  set path($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearPath() => $_clearField(3);
}

class GlobalDownloadContainerFileResponse extends $pb.GeneratedMessage {
  factory GlobalDownloadContainerFileResponse({
    $core.List<$core.int>? content,
    $core.String? path,
  }) {
    final result = GlobalDownloadContainerFileResponse._();
    if (content != null) result.content = content;
    if (path != null) result.path = path;
    return result;
  }

  GlobalDownloadContainerFileResponse._();

  factory GlobalDownloadContainerFileResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadContainerFileResponse()..mergeFromBuffer(data, registry);
  factory GlobalDownloadContainerFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDownloadContainerFileResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDownloadContainerFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDownloadContainerFileResponse.$_createMessage)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadContainerFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDownloadContainerFileResponse copyWith(
          void Function(GlobalDownloadContainerFileResponse) updates) =>
      super.copyWith((message) =>
              updates(message as GlobalDownloadContainerFileResponse))
          as GlobalDownloadContainerFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDownloadContainerFileResponse() / GlobalDownloadContainerFileResponse.new instead')
  static GlobalDownloadContainerFileResponse create() =>
      GlobalDownloadContainerFileResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalDownloadContainerFileResponse._();
  @$core.override
  GlobalDownloadContainerFileResponse createEmptyInstance() =>
      GlobalDownloadContainerFileResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalDownloadContainerFileResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              GlobalDownloadContainerFileResponse>(
          GlobalDownloadContainerFileResponse.$_createMessage);
  static GlobalDownloadContainerFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);
}

class GlobalListContainersRequest extends $pb.GeneratedMessage {
  factory GlobalListContainersRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = GlobalListContainersRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  GlobalListContainersRequest._();

  factory GlobalListContainersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListContainersRequest()..mergeFromBuffer(data, registry);
  factory GlobalListContainersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListContainersRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListContainersRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListContainersRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListContainersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListContainersRequest copyWith(
          void Function(GlobalListContainersRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalListContainersRequest))
          as GlobalListContainersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListContainersRequest() / GlobalListContainersRequest.new instead')
  static GlobalListContainersRequest create() =>
      GlobalListContainersRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalListContainersRequest._();
  @$core.override
  GlobalListContainersRequest createEmptyInstance() =>
      GlobalListContainersRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalListContainersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListContainersRequest>(
          GlobalListContainersRequest.$_createMessage);
  static GlobalListContainersRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class GlobalListContainersResponse extends $pb.GeneratedMessage {
  factory GlobalListContainersResponse({
    $core.Iterable<GlobalContainer>? containers,
  }) {
    final result = GlobalListContainersResponse._();
    if (containers != null) result.containers.addAll(containers);
    return result;
  }

  GlobalListContainersResponse._();

  factory GlobalListContainersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListContainersResponse()..mergeFromBuffer(data, registry);
  factory GlobalListContainersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListContainersResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListContainersResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListContainersResponse.$_createMessage)
    ..pPM<GlobalContainer>(1, _omitFieldNames ? '' : 'containers',
        subBuilder: GlobalContainer.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListContainersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListContainersResponse copyWith(
          void Function(GlobalListContainersResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalListContainersResponse))
          as GlobalListContainersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListContainersResponse() / GlobalListContainersResponse.new instead')
  static GlobalListContainersResponse create() =>
      GlobalListContainersResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalListContainersResponse._();
  @$core.override
  GlobalListContainersResponse createEmptyInstance() =>
      GlobalListContainersResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalListContainersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListContainersResponse>(
          GlobalListContainersResponse.$_createMessage);
  static GlobalListContainersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GlobalContainer> get containers => $_getList(0);
}

class GlobalListImagesRequest extends $pb.GeneratedMessage {
  factory GlobalListImagesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = GlobalListImagesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  GlobalListImagesRequest._();

  factory GlobalListImagesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListImagesRequest()..mergeFromBuffer(data, registry);
  factory GlobalListImagesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListImagesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListImagesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListImagesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListImagesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListImagesRequest copyWith(
          void Function(GlobalListImagesRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalListImagesRequest))
          as GlobalListImagesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListImagesRequest() / GlobalListImagesRequest.new instead')
  static GlobalListImagesRequest create() => GlobalListImagesRequest._();
  static $pb.GeneratedMessage $_createMessage() => GlobalListImagesRequest._();
  @$core.override
  GlobalListImagesRequest createEmptyInstance() => GlobalListImagesRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalListImagesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListImagesRequest>(
          GlobalListImagesRequest.$_createMessage);
  static GlobalListImagesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class GlobalListImagesResponse extends $pb.GeneratedMessage {
  factory GlobalListImagesResponse({
    $core.Iterable<GlobalImage>? images,
  }) {
    final result = GlobalListImagesResponse._();
    if (images != null) result.images.addAll(images);
    return result;
  }

  GlobalListImagesResponse._();

  factory GlobalListImagesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListImagesResponse()..mergeFromBuffer(data, registry);
  factory GlobalListImagesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListImagesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListImagesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListImagesResponse.$_createMessage)
    ..pPM<GlobalImage>(1, _omitFieldNames ? '' : 'images',
        subBuilder: GlobalImage.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListImagesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListImagesResponse copyWith(
          void Function(GlobalListImagesResponse) updates) =>
      super.copyWith((message) => updates(message as GlobalListImagesResponse))
          as GlobalListImagesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListImagesResponse() / GlobalListImagesResponse.new instead')
  static GlobalListImagesResponse create() => GlobalListImagesResponse._();
  static $pb.GeneratedMessage $_createMessage() => GlobalListImagesResponse._();
  @$core.override
  GlobalListImagesResponse createEmptyInstance() =>
      GlobalListImagesResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalListImagesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListImagesResponse>(
          GlobalListImagesResponse.$_createMessage);
  static GlobalListImagesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GlobalImage> get images => $_getList(0);
}

class GlobalListNetworksRequest extends $pb.GeneratedMessage {
  factory GlobalListNetworksRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = GlobalListNetworksRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  GlobalListNetworksRequest._();

  factory GlobalListNetworksRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListNetworksRequest()..mergeFromBuffer(data, registry);
  factory GlobalListNetworksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListNetworksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListNetworksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListNetworksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListNetworksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListNetworksRequest copyWith(
          void Function(GlobalListNetworksRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalListNetworksRequest))
          as GlobalListNetworksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListNetworksRequest() / GlobalListNetworksRequest.new instead')
  static GlobalListNetworksRequest create() => GlobalListNetworksRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalListNetworksRequest._();
  @$core.override
  GlobalListNetworksRequest createEmptyInstance() =>
      GlobalListNetworksRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalListNetworksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListNetworksRequest>(
          GlobalListNetworksRequest.$_createMessage);
  static GlobalListNetworksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class GlobalListNetworksResponse extends $pb.GeneratedMessage {
  factory GlobalListNetworksResponse({
    $core.Iterable<GlobalNetwork>? networks,
  }) {
    final result = GlobalListNetworksResponse._();
    if (networks != null) result.networks.addAll(networks);
    return result;
  }

  GlobalListNetworksResponse._();

  factory GlobalListNetworksResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListNetworksResponse()..mergeFromBuffer(data, registry);
  factory GlobalListNetworksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListNetworksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListNetworksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListNetworksResponse.$_createMessage)
    ..pPM<GlobalNetwork>(1, _omitFieldNames ? '' : 'networks',
        subBuilder: GlobalNetwork.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListNetworksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListNetworksResponse copyWith(
          void Function(GlobalListNetworksResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalListNetworksResponse))
          as GlobalListNetworksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListNetworksResponse() / GlobalListNetworksResponse.new instead')
  static GlobalListNetworksResponse create() => GlobalListNetworksResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalListNetworksResponse._();
  @$core.override
  GlobalListNetworksResponse createEmptyInstance() =>
      GlobalListNetworksResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalListNetworksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListNetworksResponse>(
          GlobalListNetworksResponse.$_createMessage);
  static GlobalListNetworksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GlobalNetwork> get networks => $_getList(0);
}

class GlobalListVolumesRequest extends $pb.GeneratedMessage {
  factory GlobalListVolumesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = GlobalListVolumesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  GlobalListVolumesRequest._();

  factory GlobalListVolumesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListVolumesRequest()..mergeFromBuffer(data, registry);
  factory GlobalListVolumesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListVolumesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListVolumesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListVolumesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListVolumesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListVolumesRequest copyWith(
          void Function(GlobalListVolumesRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalListVolumesRequest))
          as GlobalListVolumesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListVolumesRequest() / GlobalListVolumesRequest.new instead')
  static GlobalListVolumesRequest create() => GlobalListVolumesRequest._();
  static $pb.GeneratedMessage $_createMessage() => GlobalListVolumesRequest._();
  @$core.override
  GlobalListVolumesRequest createEmptyInstance() =>
      GlobalListVolumesRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalListVolumesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListVolumesRequest>(
          GlobalListVolumesRequest.$_createMessage);
  static GlobalListVolumesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class GlobalListVolumesResponse extends $pb.GeneratedMessage {
  factory GlobalListVolumesResponse({
    $core.Iterable<GlobalVolume>? volumes,
  }) {
    final result = GlobalListVolumesResponse._();
    if (volumes != null) result.volumes.addAll(volumes);
    return result;
  }

  GlobalListVolumesResponse._();

  factory GlobalListVolumesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListVolumesResponse()..mergeFromBuffer(data, registry);
  factory GlobalListVolumesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalListVolumesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalListVolumesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalListVolumesResponse.$_createMessage)
    ..pPM<GlobalVolume>(1, _omitFieldNames ? '' : 'volumes',
        subBuilder: GlobalVolume.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListVolumesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalListVolumesResponse copyWith(
          void Function(GlobalListVolumesResponse) updates) =>
      super.copyWith((message) => updates(message as GlobalListVolumesResponse))
          as GlobalListVolumesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalListVolumesResponse() / GlobalListVolumesResponse.new instead')
  static GlobalListVolumesResponse create() => GlobalListVolumesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalListVolumesResponse._();
  @$core.override
  GlobalListVolumesResponse createEmptyInstance() =>
      GlobalListVolumesResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalListVolumesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalListVolumesResponse>(
          GlobalListVolumesResponse.$_createMessage);
  static GlobalListVolumesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<GlobalVolume> get volumes => $_getList(0);
}

class GlobalInspectContainerRequest extends $pb.GeneratedMessage {
  factory GlobalInspectContainerRequest({
    $fixnum.Int64? serverId,
    $core.String? containerId,
  }) {
    final result = GlobalInspectContainerRequest._();
    if (serverId != null) result.serverId = serverId;
    if (containerId != null) result.containerId = containerId;
    return result;
  }

  GlobalInspectContainerRequest._();

  factory GlobalInspectContainerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectContainerRequest()..mergeFromBuffer(data, registry);
  factory GlobalInspectContainerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectContainerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectContainerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectContainerRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'containerId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectContainerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectContainerRequest copyWith(
          void Function(GlobalInspectContainerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectContainerRequest))
          as GlobalInspectContainerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectContainerRequest() / GlobalInspectContainerRequest.new instead')
  static GlobalInspectContainerRequest create() =>
      GlobalInspectContainerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectContainerRequest._();
  @$core.override
  GlobalInspectContainerRequest createEmptyInstance() =>
      GlobalInspectContainerRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectContainerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectContainerRequest>(
          GlobalInspectContainerRequest.$_createMessage);
  static GlobalInspectContainerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get containerId => $_getSZ(1);
  @$pb.TagNumber(2)
  set containerId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContainerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContainerId() => $_clearField(2);
}

class GlobalInspectContainerResponse extends $pb.GeneratedMessage {
  factory GlobalInspectContainerResponse({
    $core.String? dataJson,
  }) {
    final result = GlobalInspectContainerResponse._();
    if (dataJson != null) result.dataJson = dataJson;
    return result;
  }

  GlobalInspectContainerResponse._();

  factory GlobalInspectContainerResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectContainerResponse()..mergeFromBuffer(data, registry);
  factory GlobalInspectContainerResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectContainerResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectContainerResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectContainerResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'dataJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectContainerResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectContainerResponse copyWith(
          void Function(GlobalInspectContainerResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectContainerResponse))
          as GlobalInspectContainerResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectContainerResponse() / GlobalInspectContainerResponse.new instead')
  static GlobalInspectContainerResponse create() =>
      GlobalInspectContainerResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectContainerResponse._();
  @$core.override
  GlobalInspectContainerResponse createEmptyInstance() =>
      GlobalInspectContainerResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectContainerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectContainerResponse>(
          GlobalInspectContainerResponse.$_createMessage);
  static GlobalInspectContainerResponse? _defaultInstance;

  /// Raw `docker inspect` JSON document for the container.
  @$pb.TagNumber(1)
  $core.String get dataJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDataJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataJson() => $_clearField(1);
}

class GlobalInspectImageRequest extends $pb.GeneratedMessage {
  factory GlobalInspectImageRequest({
    $fixnum.Int64? serverId,
    $core.String? imageId,
  }) {
    final result = GlobalInspectImageRequest._();
    if (serverId != null) result.serverId = serverId;
    if (imageId != null) result.imageId = imageId;
    return result;
  }

  GlobalInspectImageRequest._();

  factory GlobalInspectImageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectImageRequest()..mergeFromBuffer(data, registry);
  factory GlobalInspectImageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectImageRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectImageRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectImageRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'imageId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectImageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectImageRequest copyWith(
          void Function(GlobalInspectImageRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalInspectImageRequest))
          as GlobalInspectImageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectImageRequest() / GlobalInspectImageRequest.new instead')
  static GlobalInspectImageRequest create() => GlobalInspectImageRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectImageRequest._();
  @$core.override
  GlobalInspectImageRequest createEmptyInstance() =>
      GlobalInspectImageRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectImageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectImageRequest>(
          GlobalInspectImageRequest.$_createMessage);
  static GlobalInspectImageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  /// Image id or reference (validate_resource_name charset).
  @$pb.TagNumber(2)
  $core.String get imageId => $_getSZ(1);
  @$pb.TagNumber(2)
  set imageId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasImageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearImageId() => $_clearField(2);
}

class GlobalInspectImageResponse extends $pb.GeneratedMessage {
  factory GlobalInspectImageResponse({
    $core.String? dataJson,
  }) {
    final result = GlobalInspectImageResponse._();
    if (dataJson != null) result.dataJson = dataJson;
    return result;
  }

  GlobalInspectImageResponse._();

  factory GlobalInspectImageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectImageResponse()..mergeFromBuffer(data, registry);
  factory GlobalInspectImageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectImageResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectImageResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectImageResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'dataJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectImageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectImageResponse copyWith(
          void Function(GlobalInspectImageResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectImageResponse))
          as GlobalInspectImageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectImageResponse() / GlobalInspectImageResponse.new instead')
  static GlobalInspectImageResponse create() => GlobalInspectImageResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectImageResponse._();
  @$core.override
  GlobalInspectImageResponse createEmptyInstance() =>
      GlobalInspectImageResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectImageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectImageResponse>(
          GlobalInspectImageResponse.$_createMessage);
  static GlobalInspectImageResponse? _defaultInstance;

  /// Raw `docker inspect` JSON document for the image.
  @$pb.TagNumber(1)
  $core.String get dataJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDataJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataJson() => $_clearField(1);
}

class GlobalInspectNetworkRequest extends $pb.GeneratedMessage {
  factory GlobalInspectNetworkRequest({
    $fixnum.Int64? serverId,
    $core.String? networkId,
  }) {
    final result = GlobalInspectNetworkRequest._();
    if (serverId != null) result.serverId = serverId;
    if (networkId != null) result.networkId = networkId;
    return result;
  }

  GlobalInspectNetworkRequest._();

  factory GlobalInspectNetworkRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectNetworkRequest()..mergeFromBuffer(data, registry);
  factory GlobalInspectNetworkRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectNetworkRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectNetworkRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectNetworkRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'networkId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectNetworkRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectNetworkRequest copyWith(
          void Function(GlobalInspectNetworkRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectNetworkRequest))
          as GlobalInspectNetworkRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectNetworkRequest() / GlobalInspectNetworkRequest.new instead')
  static GlobalInspectNetworkRequest create() =>
      GlobalInspectNetworkRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectNetworkRequest._();
  @$core.override
  GlobalInspectNetworkRequest createEmptyInstance() =>
      GlobalInspectNetworkRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectNetworkRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectNetworkRequest>(
          GlobalInspectNetworkRequest.$_createMessage);
  static GlobalInspectNetworkRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get networkId => $_getSZ(1);
  @$pb.TagNumber(2)
  set networkId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNetworkId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNetworkId() => $_clearField(2);
}

class GlobalInspectNetworkResponse extends $pb.GeneratedMessage {
  factory GlobalInspectNetworkResponse({
    $core.String? dataJson,
  }) {
    final result = GlobalInspectNetworkResponse._();
    if (dataJson != null) result.dataJson = dataJson;
    return result;
  }

  GlobalInspectNetworkResponse._();

  factory GlobalInspectNetworkResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectNetworkResponse()..mergeFromBuffer(data, registry);
  factory GlobalInspectNetworkResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectNetworkResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectNetworkResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectNetworkResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'dataJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectNetworkResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectNetworkResponse copyWith(
          void Function(GlobalInspectNetworkResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectNetworkResponse))
          as GlobalInspectNetworkResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectNetworkResponse() / GlobalInspectNetworkResponse.new instead')
  static GlobalInspectNetworkResponse create() =>
      GlobalInspectNetworkResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectNetworkResponse._();
  @$core.override
  GlobalInspectNetworkResponse createEmptyInstance() =>
      GlobalInspectNetworkResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectNetworkResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectNetworkResponse>(
          GlobalInspectNetworkResponse.$_createMessage);
  static GlobalInspectNetworkResponse? _defaultInstance;

  /// Raw `docker inspect` JSON document for the network.
  @$pb.TagNumber(1)
  $core.String get dataJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDataJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataJson() => $_clearField(1);
}

class GlobalInspectVolumeRequest extends $pb.GeneratedMessage {
  factory GlobalInspectVolumeRequest({
    $fixnum.Int64? serverId,
    $core.String? volumeId,
  }) {
    final result = GlobalInspectVolumeRequest._();
    if (serverId != null) result.serverId = serverId;
    if (volumeId != null) result.volumeId = volumeId;
    return result;
  }

  GlobalInspectVolumeRequest._();

  factory GlobalInspectVolumeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectVolumeRequest()..mergeFromBuffer(data, registry);
  factory GlobalInspectVolumeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectVolumeRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectVolumeRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectVolumeRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aOS(2, _omitFieldNames ? '' : 'volumeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectVolumeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectVolumeRequest copyWith(
          void Function(GlobalInspectVolumeRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectVolumeRequest))
          as GlobalInspectVolumeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectVolumeRequest() / GlobalInspectVolumeRequest.new instead')
  static GlobalInspectVolumeRequest create() => GlobalInspectVolumeRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectVolumeRequest._();
  @$core.override
  GlobalInspectVolumeRequest createEmptyInstance() =>
      GlobalInspectVolumeRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectVolumeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectVolumeRequest>(
          GlobalInspectVolumeRequest.$_createMessage);
  static GlobalInspectVolumeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get volumeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set volumeId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVolumeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearVolumeId() => $_clearField(2);
}

class GlobalInspectVolumeResponse extends $pb.GeneratedMessage {
  factory GlobalInspectVolumeResponse({
    $core.String? dataJson,
  }) {
    final result = GlobalInspectVolumeResponse._();
    if (dataJson != null) result.dataJson = dataJson;
    return result;
  }

  GlobalInspectVolumeResponse._();

  factory GlobalInspectVolumeResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectVolumeResponse()..mergeFromBuffer(data, registry);
  factory GlobalInspectVolumeResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalInspectVolumeResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalInspectVolumeResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalInspectVolumeResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'dataJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectVolumeResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalInspectVolumeResponse copyWith(
          void Function(GlobalInspectVolumeResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalInspectVolumeResponse))
          as GlobalInspectVolumeResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalInspectVolumeResponse() / GlobalInspectVolumeResponse.new instead')
  static GlobalInspectVolumeResponse create() =>
      GlobalInspectVolumeResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalInspectVolumeResponse._();
  @$core.override
  GlobalInspectVolumeResponse createEmptyInstance() =>
      GlobalInspectVolumeResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalInspectVolumeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalInspectVolumeResponse>(
          GlobalInspectVolumeResponse.$_createMessage);
  static GlobalInspectVolumeResponse? _defaultInstance;

  /// Raw `docker inspect` JSON document for the volume.
  @$pb.TagNumber(1)
  $core.String get dataJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDataJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataJson() => $_clearField(1);
}

class GlobalContainerActionRequest extends $pb.GeneratedMessage {
  factory GlobalContainerActionRequest({
    $core.String? containerId,
    GlobalContainerAction? action,
  }) {
    final result = GlobalContainerActionRequest._();
    if (containerId != null) result.containerId = containerId;
    if (action != null) result.action = action;
    return result;
  }

  GlobalContainerActionRequest._();

  factory GlobalContainerActionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainerActionRequest()..mergeFromBuffer(data, registry);
  factory GlobalContainerActionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainerActionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalContainerActionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalContainerActionRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'containerId')
    ..aE<GlobalContainerAction>(2, _omitFieldNames ? '' : 'action',
        enumValues: GlobalContainerAction.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainerActionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainerActionRequest copyWith(
          void Function(GlobalContainerActionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalContainerActionRequest))
          as GlobalContainerActionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalContainerActionRequest() / GlobalContainerActionRequest.new instead')
  static GlobalContainerActionRequest create() =>
      GlobalContainerActionRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalContainerActionRequest._();
  @$core.override
  GlobalContainerActionRequest createEmptyInstance() =>
      GlobalContainerActionRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalContainerActionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalContainerActionRequest>(
          GlobalContainerActionRequest.$_createMessage);
  static GlobalContainerActionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get containerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set containerId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainerId() => $_clearField(1);

  @$pb.TagNumber(2)
  GlobalContainerAction get action => $_getN(1);
  @$pb.TagNumber(2)
  set action(GlobalContainerAction value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);
}

class GlobalContainerActionResponse extends $pb.GeneratedMessage {
  factory GlobalContainerActionResponse({
    $core.bool? success,
    GlobalDockerOutput? output,
  }) {
    final result = GlobalContainerActionResponse._();
    if (success != null) result.success = success;
    if (output != null) result.output = output;
    return result;
  }

  GlobalContainerActionResponse._();

  factory GlobalContainerActionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainerActionResponse()..mergeFromBuffer(data, registry);
  factory GlobalContainerActionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalContainerActionResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalContainerActionResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalContainerActionResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOM<GlobalDockerOutput>(2, _omitFieldNames ? '' : 'output',
        subBuilder: GlobalDockerOutput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainerActionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalContainerActionResponse copyWith(
          void Function(GlobalContainerActionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalContainerActionResponse))
          as GlobalContainerActionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalContainerActionResponse() / GlobalContainerActionResponse.new instead')
  static GlobalContainerActionResponse create() =>
      GlobalContainerActionResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalContainerActionResponse._();
  @$core.override
  GlobalContainerActionResponse createEmptyInstance() =>
      GlobalContainerActionResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalContainerActionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalContainerActionResponse>(
          GlobalContainerActionResponse.$_createMessage);
  static GlobalContainerActionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  GlobalDockerOutput get output => $_getN(1);
  @$pb.TagNumber(2)
  set output(GlobalDockerOutput value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOutput() => $_has(1);
  @$pb.TagNumber(2)
  void clearOutput() => $_clearField(2);
  @$pb.TagNumber(2)
  GlobalDockerOutput ensureOutput() => $_ensure(1);
}

class GlobalRemoveContainerRequest extends $pb.GeneratedMessage {
  factory GlobalRemoveContainerRequest({
    $core.String? containerId,
    $core.bool? force,
    $core.bool? volumes,
  }) {
    final result = GlobalRemoveContainerRequest._();
    if (containerId != null) result.containerId = containerId;
    if (force != null) result.force = force;
    if (volumes != null) result.volumes = volumes;
    return result;
  }

  GlobalRemoveContainerRequest._();

  factory GlobalRemoveContainerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalRemoveContainerRequest()..mergeFromBuffer(data, registry);
  factory GlobalRemoveContainerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalRemoveContainerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalRemoveContainerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalRemoveContainerRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'containerId')
    ..aOB(2, _omitFieldNames ? '' : 'force')
    ..aOB(3, _omitFieldNames ? '' : 'volumes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalRemoveContainerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalRemoveContainerRequest copyWith(
          void Function(GlobalRemoveContainerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalRemoveContainerRequest))
          as GlobalRemoveContainerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalRemoveContainerRequest() / GlobalRemoveContainerRequest.new instead')
  static GlobalRemoveContainerRequest create() =>
      GlobalRemoveContainerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalRemoveContainerRequest._();
  @$core.override
  GlobalRemoveContainerRequest createEmptyInstance() =>
      GlobalRemoveContainerRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalRemoveContainerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalRemoveContainerRequest>(
          GlobalRemoveContainerRequest.$_createMessage);
  static GlobalRemoveContainerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get containerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set containerId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get force => $_getBF(1);
  @$pb.TagNumber(2)
  set force($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasForce() => $_has(1);
  @$pb.TagNumber(2)
  void clearForce() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get volumes => $_getBF(2);
  @$pb.TagNumber(3)
  set volumes($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVolumes() => $_has(2);
  @$pb.TagNumber(3)
  void clearVolumes() => $_clearField(3);
}

class GlobalDeleteContainerRequest extends $pb.GeneratedMessage {
  factory GlobalDeleteContainerRequest({
    $core.String? containerId,
    $core.bool? force,
    $core.bool? volumes,
  }) {
    final result = GlobalDeleteContainerRequest._();
    if (containerId != null) result.containerId = containerId;
    if (force != null) result.force = force;
    if (volumes != null) result.volumes = volumes;
    return result;
  }

  GlobalDeleteContainerRequest._();

  factory GlobalDeleteContainerRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDeleteContainerRequest()..mergeFromBuffer(data, registry);
  factory GlobalDeleteContainerRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDeleteContainerRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDeleteContainerRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDeleteContainerRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'containerId')
    ..aOB(2, _omitFieldNames ? '' : 'force')
    ..aOB(3, _omitFieldNames ? '' : 'volumes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDeleteContainerRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDeleteContainerRequest copyWith(
          void Function(GlobalDeleteContainerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GlobalDeleteContainerRequest))
          as GlobalDeleteContainerRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDeleteContainerRequest() / GlobalDeleteContainerRequest.new instead')
  static GlobalDeleteContainerRequest create() =>
      GlobalDeleteContainerRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GlobalDeleteContainerRequest._();
  @$core.override
  GlobalDeleteContainerRequest createEmptyInstance() =>
      GlobalDeleteContainerRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalDeleteContainerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDeleteContainerRequest>(
          GlobalDeleteContainerRequest.$_createMessage);
  static GlobalDeleteContainerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get containerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set containerId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get force => $_getBF(1);
  @$pb.TagNumber(2)
  set force($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasForce() => $_has(1);
  @$pb.TagNumber(2)
  void clearForce() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get volumes => $_getBF(2);
  @$pb.TagNumber(3)
  set volumes($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVolumes() => $_has(2);
  @$pb.TagNumber(3)
  void clearVolumes() => $_clearField(3);
}

class GlobalDiskUsageRequest extends $pb.GeneratedMessage {
  factory GlobalDiskUsageRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = GlobalDiskUsageRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  GlobalDiskUsageRequest._();

  factory GlobalDiskUsageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDiskUsageRequest()..mergeFromBuffer(data, registry);
  factory GlobalDiskUsageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDiskUsageRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDiskUsageRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDiskUsageRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDiskUsageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDiskUsageRequest copyWith(
          void Function(GlobalDiskUsageRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalDiskUsageRequest))
          as GlobalDiskUsageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDiskUsageRequest() / GlobalDiskUsageRequest.new instead')
  static GlobalDiskUsageRequest create() => GlobalDiskUsageRequest._();
  static $pb.GeneratedMessage $_createMessage() => GlobalDiskUsageRequest._();
  @$core.override
  GlobalDiskUsageRequest createEmptyInstance() => GlobalDiskUsageRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalDiskUsageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDiskUsageRequest>(
          GlobalDiskUsageRequest.$_createMessage);
  static GlobalDiskUsageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class GlobalDiskUsageResponse extends $pb.GeneratedMessage {
  factory GlobalDiskUsageResponse({
    $core.String? dataJson,
  }) {
    final result = GlobalDiskUsageResponse._();
    if (dataJson != null) result.dataJson = dataJson;
    return result;
  }

  GlobalDiskUsageResponse._();

  factory GlobalDiskUsageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDiskUsageResponse()..mergeFromBuffer(data, registry);
  factory GlobalDiskUsageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalDiskUsageResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalDiskUsageResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalDiskUsageResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'dataJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDiskUsageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalDiskUsageResponse copyWith(
          void Function(GlobalDiskUsageResponse) updates) =>
      super.copyWith((message) => updates(message as GlobalDiskUsageResponse))
          as GlobalDiskUsageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GlobalDiskUsageResponse() / GlobalDiskUsageResponse.new instead')
  static GlobalDiskUsageResponse create() => GlobalDiskUsageResponse._();
  static $pb.GeneratedMessage $_createMessage() => GlobalDiskUsageResponse._();
  @$core.override
  GlobalDiskUsageResponse createEmptyInstance() => GlobalDiskUsageResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalDiskUsageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalDiskUsageResponse>(
          GlobalDiskUsageResponse.$_createMessage);
  static GlobalDiskUsageResponse? _defaultInstance;

  /// Raw `docker system df` JSON document.
  @$pb.TagNumber(1)
  $core.String get dataJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDataJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataJson() => $_clearField(1);
}

class GlobalPruneRequest extends $pb.GeneratedMessage {
  factory GlobalPruneRequest({
    $core.bool? containers,
    $core.bool? images,
    $core.bool? networks,
    $core.bool? volumes,
  }) {
    final result = GlobalPruneRequest._();
    if (containers != null) result.containers = containers;
    if (images != null) result.images = images;
    if (networks != null) result.networks = networks;
    if (volumes != null) result.volumes = volumes;
    return result;
  }

  GlobalPruneRequest._();

  factory GlobalPruneRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalPruneRequest()..mergeFromBuffer(data, registry);
  factory GlobalPruneRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalPruneRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalPruneRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalPruneRequest.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'containers')
    ..aOB(2, _omitFieldNames ? '' : 'images')
    ..aOB(3, _omitFieldNames ? '' : 'networks')
    ..aOB(4, _omitFieldNames ? '' : 'volumes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalPruneRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalPruneRequest copyWith(void Function(GlobalPruneRequest) updates) =>
      super.copyWith((message) => updates(message as GlobalPruneRequest))
          as GlobalPruneRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GlobalPruneRequest() / GlobalPruneRequest.new instead')
  static GlobalPruneRequest create() => GlobalPruneRequest._();
  static $pb.GeneratedMessage $_createMessage() => GlobalPruneRequest._();
  @$core.override
  GlobalPruneRequest createEmptyInstance() => GlobalPruneRequest._();
  @$core.pragma('dart2js:noInline')
  static GlobalPruneRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalPruneRequest>(
          GlobalPruneRequest.$_createMessage);
  static GlobalPruneRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get containers => $_getBF(0);
  @$pb.TagNumber(1)
  set containers($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainers() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainers() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get images => $_getBF(1);
  @$pb.TagNumber(2)
  set images($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasImages() => $_has(1);
  @$pb.TagNumber(2)
  void clearImages() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get networks => $_getBF(2);
  @$pb.TagNumber(3)
  set networks($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasNetworks() => $_has(2);
  @$pb.TagNumber(3)
  void clearNetworks() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get volumes => $_getBF(3);
  @$pb.TagNumber(4)
  set volumes($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasVolumes() => $_has(3);
  @$pb.TagNumber(4)
  void clearVolumes() => $_clearField(4);
}

class GlobalPruneResponse extends $pb.GeneratedMessage {
  factory GlobalPruneResponse({
    $core.Iterable<GlobalDockerOutput>? operations,
  }) {
    final result = GlobalPruneResponse._();
    if (operations != null) result.operations.addAll(operations);
    return result;
  }

  GlobalPruneResponse._();

  factory GlobalPruneResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalPruneResponse()..mergeFromBuffer(data, registry);
  factory GlobalPruneResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GlobalPruneResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GlobalPruneResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GlobalPruneResponse.$_createMessage)
    ..pPM<GlobalDockerOutput>(1, _omitFieldNames ? '' : 'operations',
        subBuilder: GlobalDockerOutput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalPruneResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GlobalPruneResponse copyWith(void Function(GlobalPruneResponse) updates) =>
      super.copyWith((message) => updates(message as GlobalPruneResponse))
          as GlobalPruneResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use GlobalPruneResponse() / GlobalPruneResponse.new instead')
  static GlobalPruneResponse create() => GlobalPruneResponse._();
  static $pb.GeneratedMessage $_createMessage() => GlobalPruneResponse._();
  @$core.override
  GlobalPruneResponse createEmptyInstance() => GlobalPruneResponse._();
  @$core.pragma('dart2js:noInline')
  static GlobalPruneResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GlobalPruneResponse>(
          GlobalPruneResponse.$_createMessage);
  static GlobalPruneResponse? _defaultInstance;

  /// One output entry per enabled prune operation, in request order.
  @$pb.TagNumber(1)
  $pb.PbList<GlobalDockerOutput> get operations => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

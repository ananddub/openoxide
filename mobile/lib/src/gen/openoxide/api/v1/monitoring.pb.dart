// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/monitoring.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'monitoring.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'monitoring.pbenum.dart';

/// One server metric sample returned by the agent (host facts repeated per row).
class MonitoringServerMetric extends $pb.GeneratedMessage {
  factory MonitoringServerMetric({
    $fixnum.Int64? timestamp,
    $fixnum.Int64? serverId,
    $core.double? cpu,
    $core.String? cpuModel,
    $fixnum.Int64? cpuCores,
    $fixnum.Int64? cpuPhysicalCores,
    $core.double? cpuSpeed,
    $core.String? os,
    $core.String? distro,
    $core.String? kernel,
    $core.String? arch,
    $core.double? memUsed,
    $core.double? memUsedGb,
    $core.double? memTotal,
    $fixnum.Int64? uptime,
    $core.double? diskUsed,
    $core.double? totalDisk,
    $core.double? networkIn,
    $core.double? networkOut,
  }) {
    final result = MonitoringServerMetric._();
    if (timestamp != null) result.timestamp = timestamp;
    if (serverId != null) result.serverId = serverId;
    if (cpu != null) result.cpu = cpu;
    if (cpuModel != null) result.cpuModel = cpuModel;
    if (cpuCores != null) result.cpuCores = cpuCores;
    if (cpuPhysicalCores != null) result.cpuPhysicalCores = cpuPhysicalCores;
    if (cpuSpeed != null) result.cpuSpeed = cpuSpeed;
    if (os != null) result.os = os;
    if (distro != null) result.distro = distro;
    if (kernel != null) result.kernel = kernel;
    if (arch != null) result.arch = arch;
    if (memUsed != null) result.memUsed = memUsed;
    if (memUsedGb != null) result.memUsedGb = memUsedGb;
    if (memTotal != null) result.memTotal = memTotal;
    if (uptime != null) result.uptime = uptime;
    if (diskUsed != null) result.diskUsed = diskUsed;
    if (totalDisk != null) result.totalDisk = totalDisk;
    if (networkIn != null) result.networkIn = networkIn;
    if (networkOut != null) result.networkOut = networkOut;
    return result;
  }

  MonitoringServerMetric._();

  factory MonitoringServerMetric.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringServerMetric()..mergeFromBuffer(data, registry);
  factory MonitoringServerMetric.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringServerMetric()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringServerMetric',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringServerMetric.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'timestamp')
    ..aInt64(2, _omitFieldNames ? '' : 'serverId')
    ..aD(3, _omitFieldNames ? '' : 'cpu')
    ..aOS(4, _omitFieldNames ? '' : 'cpuModel')
    ..aInt64(5, _omitFieldNames ? '' : 'cpuCores')
    ..aInt64(6, _omitFieldNames ? '' : 'cpuPhysicalCores')
    ..aD(7, _omitFieldNames ? '' : 'cpuSpeed')
    ..aOS(8, _omitFieldNames ? '' : 'os')
    ..aOS(9, _omitFieldNames ? '' : 'distro')
    ..aOS(10, _omitFieldNames ? '' : 'kernel')
    ..aOS(11, _omitFieldNames ? '' : 'arch')
    ..aD(12, _omitFieldNames ? '' : 'memUsed')
    ..aD(13, _omitFieldNames ? '' : 'memUsedGb')
    ..aD(14, _omitFieldNames ? '' : 'memTotal')
    ..aInt64(15, _omitFieldNames ? '' : 'uptime')
    ..aD(16, _omitFieldNames ? '' : 'diskUsed')
    ..aD(17, _omitFieldNames ? '' : 'totalDisk')
    ..aD(18, _omitFieldNames ? '' : 'networkIn')
    ..aD(19, _omitFieldNames ? '' : 'networkOut')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringServerMetric clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringServerMetric copyWith(
          void Function(MonitoringServerMetric) updates) =>
      super.copyWith((message) => updates(message as MonitoringServerMetric))
          as MonitoringServerMetric;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringServerMetric() / MonitoringServerMetric.new instead')
  static MonitoringServerMetric create() => MonitoringServerMetric._();
  static $pb.GeneratedMessage $_createMessage() => MonitoringServerMetric._();
  @$core.override
  MonitoringServerMetric createEmptyInstance() => MonitoringServerMetric._();
  @$core.pragma('dart2js:noInline')
  static MonitoringServerMetric getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringServerMetric>(
          MonitoringServerMetric.$_createMessage);
  static MonitoringServerMetric? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get serverId => $_getI64(1);
  @$pb.TagNumber(2)
  set serverId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerId() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerId() => $_clearField(2);

  /// CPU utilisation percent.
  @$pb.TagNumber(3)
  $core.double get cpu => $_getN(2);
  @$pb.TagNumber(3)
  set cpu($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCpu() => $_has(2);
  @$pb.TagNumber(3)
  void clearCpu() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get cpuModel => $_getSZ(3);
  @$pb.TagNumber(4)
  set cpuModel($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCpuModel() => $_has(3);
  @$pb.TagNumber(4)
  void clearCpuModel() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get cpuCores => $_getI64(4);
  @$pb.TagNumber(5)
  set cpuCores($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCpuCores() => $_has(4);
  @$pb.TagNumber(5)
  void clearCpuCores() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get cpuPhysicalCores => $_getI64(5);
  @$pb.TagNumber(6)
  set cpuPhysicalCores($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasCpuPhysicalCores() => $_has(5);
  @$pb.TagNumber(6)
  void clearCpuPhysicalCores() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get cpuSpeed => $_getN(6);
  @$pb.TagNumber(7)
  set cpuSpeed($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCpuSpeed() => $_has(6);
  @$pb.TagNumber(7)
  void clearCpuSpeed() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get os => $_getSZ(7);
  @$pb.TagNumber(8)
  set os($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasOs() => $_has(7);
  @$pb.TagNumber(8)
  void clearOs() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get distro => $_getSZ(8);
  @$pb.TagNumber(9)
  set distro($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasDistro() => $_has(8);
  @$pb.TagNumber(9)
  void clearDistro() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get kernel => $_getSZ(9);
  @$pb.TagNumber(10)
  set kernel($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasKernel() => $_has(9);
  @$pb.TagNumber(10)
  void clearKernel() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get arch => $_getSZ(10);
  @$pb.TagNumber(11)
  set arch($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasArch() => $_has(10);
  @$pb.TagNumber(11)
  void clearArch() => $_clearField(11);

  /// Memory / disk utilisation percents.
  @$pb.TagNumber(12)
  $core.double get memUsed => $_getN(11);
  @$pb.TagNumber(12)
  set memUsed($core.double value) => $_setDouble(11, value);
  @$pb.TagNumber(12)
  $core.bool hasMemUsed() => $_has(11);
  @$pb.TagNumber(12)
  void clearMemUsed() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.double get memUsedGb => $_getN(12);
  @$pb.TagNumber(13)
  set memUsedGb($core.double value) => $_setDouble(12, value);
  @$pb.TagNumber(13)
  $core.bool hasMemUsedGb() => $_has(12);
  @$pb.TagNumber(13)
  void clearMemUsedGb() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.double get memTotal => $_getN(13);
  @$pb.TagNumber(14)
  set memTotal($core.double value) => $_setDouble(13, value);
  @$pb.TagNumber(14)
  $core.bool hasMemTotal() => $_has(13);
  @$pb.TagNumber(14)
  void clearMemTotal() => $_clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get uptime => $_getI64(14);
  @$pb.TagNumber(15)
  set uptime($fixnum.Int64 value) => $_setInt64(14, value);
  @$pb.TagNumber(15)
  $core.bool hasUptime() => $_has(14);
  @$pb.TagNumber(15)
  void clearUptime() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.double get diskUsed => $_getN(15);
  @$pb.TagNumber(16)
  set diskUsed($core.double value) => $_setDouble(15, value);
  @$pb.TagNumber(16)
  $core.bool hasDiskUsed() => $_has(15);
  @$pb.TagNumber(16)
  void clearDiskUsed() => $_clearField(16);

  @$pb.TagNumber(17)
  $core.double get totalDisk => $_getN(16);
  @$pb.TagNumber(17)
  set totalDisk($core.double value) => $_setDouble(16, value);
  @$pb.TagNumber(17)
  $core.bool hasTotalDisk() => $_has(16);
  @$pb.TagNumber(17)
  void clearTotalDisk() => $_clearField(17);

  @$pb.TagNumber(18)
  $core.double get networkIn => $_getN(17);
  @$pb.TagNumber(18)
  set networkIn($core.double value) => $_setDouble(17, value);
  @$pb.TagNumber(18)
  $core.bool hasNetworkIn() => $_has(17);
  @$pb.TagNumber(18)
  void clearNetworkIn() => $_clearField(18);

  @$pb.TagNumber(19)
  $core.double get networkOut => $_getN(18);
  @$pb.TagNumber(19)
  set networkOut($core.double value) => $_setDouble(18, value);
  @$pb.TagNumber(19)
  $core.bool hasNetworkOut() => $_has(18);
  @$pb.TagNumber(19)
  void clearNetworkOut() => $_clearField(19);
}

/// Fixed-key projection of the agent's per-container metrics_json document.
class MonitoringContainerStats extends $pb.GeneratedMessage {
  factory MonitoringContainerStats({
    $core.double? cpuPercent,
    $core.double? memoryPercent,
    $core.double? memoryUsedMb,
    $core.double? memoryLimitMb,
    $core.double? netRxKbps,
    $core.double? netTxKbps,
    $core.double? blockReadMb,
    $core.double? blockWriteMb,
  }) {
    final result = MonitoringContainerStats._();
    if (cpuPercent != null) result.cpuPercent = cpuPercent;
    if (memoryPercent != null) result.memoryPercent = memoryPercent;
    if (memoryUsedMb != null) result.memoryUsedMb = memoryUsedMb;
    if (memoryLimitMb != null) result.memoryLimitMb = memoryLimitMb;
    if (netRxKbps != null) result.netRxKbps = netRxKbps;
    if (netTxKbps != null) result.netTxKbps = netTxKbps;
    if (blockReadMb != null) result.blockReadMb = blockReadMb;
    if (blockWriteMb != null) result.blockWriteMb = blockWriteMb;
    return result;
  }

  MonitoringContainerStats._();

  factory MonitoringContainerStats.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerStats()..mergeFromBuffer(data, registry);
  factory MonitoringContainerStats.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerStats()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringContainerStats',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringContainerStats.$_createMessage)
    ..aD(1, _omitFieldNames ? '' : 'cpuPercent')
    ..aD(2, _omitFieldNames ? '' : 'memoryPercent')
    ..aD(3, _omitFieldNames ? '' : 'memoryUsedMb')
    ..aD(4, _omitFieldNames ? '' : 'memoryLimitMb')
    ..aD(5, _omitFieldNames ? '' : 'netRxKbps')
    ..aD(6, _omitFieldNames ? '' : 'netTxKbps')
    ..aD(7, _omitFieldNames ? '' : 'blockReadMb')
    ..aD(8, _omitFieldNames ? '' : 'blockWriteMb')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerStats clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerStats copyWith(
          void Function(MonitoringContainerStats) updates) =>
      super.copyWith((message) => updates(message as MonitoringContainerStats))
          as MonitoringContainerStats;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringContainerStats() / MonitoringContainerStats.new instead')
  static MonitoringContainerStats create() => MonitoringContainerStats._();
  static $pb.GeneratedMessage $_createMessage() => MonitoringContainerStats._();
  @$core.override
  MonitoringContainerStats createEmptyInstance() =>
      MonitoringContainerStats._();
  @$core.pragma('dart2js:noInline')
  static MonitoringContainerStats getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringContainerStats>(
          MonitoringContainerStats.$_createMessage);
  static MonitoringContainerStats? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get cpuPercent => $_getN(0);
  @$pb.TagNumber(1)
  set cpuPercent($core.double value) => $_setDouble(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCpuPercent() => $_has(0);
  @$pb.TagNumber(1)
  void clearCpuPercent() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get memoryPercent => $_getN(1);
  @$pb.TagNumber(2)
  set memoryPercent($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMemoryPercent() => $_has(1);
  @$pb.TagNumber(2)
  void clearMemoryPercent() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.double get memoryUsedMb => $_getN(2);
  @$pb.TagNumber(3)
  set memoryUsedMb($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMemoryUsedMb() => $_has(2);
  @$pb.TagNumber(3)
  void clearMemoryUsedMb() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.double get memoryLimitMb => $_getN(3);
  @$pb.TagNumber(4)
  set memoryLimitMb($core.double value) => $_setDouble(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMemoryLimitMb() => $_has(3);
  @$pb.TagNumber(4)
  void clearMemoryLimitMb() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.double get netRxKbps => $_getN(4);
  @$pb.TagNumber(5)
  set netRxKbps($core.double value) => $_setDouble(4, value);
  @$pb.TagNumber(5)
  $core.bool hasNetRxKbps() => $_has(4);
  @$pb.TagNumber(5)
  void clearNetRxKbps() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.double get netTxKbps => $_getN(5);
  @$pb.TagNumber(6)
  set netTxKbps($core.double value) => $_setDouble(5, value);
  @$pb.TagNumber(6)
  $core.bool hasNetTxKbps() => $_has(5);
  @$pb.TagNumber(6)
  void clearNetTxKbps() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get blockReadMb => $_getN(6);
  @$pb.TagNumber(7)
  set blockReadMb($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBlockReadMb() => $_has(6);
  @$pb.TagNumber(7)
  void clearBlockReadMb() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.double get blockWriteMb => $_getN(7);
  @$pb.TagNumber(8)
  set blockWriteMb($core.double value) => $_setDouble(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBlockWriteMb() => $_has(7);
  @$pb.TagNumber(8)
  void clearBlockWriteMb() => $_clearField(8);
}

class MonitoringContainerMetric extends $pb.GeneratedMessage {
  factory MonitoringContainerMetric({
    $fixnum.Int64? id,
    $fixnum.Int64? timestamp,
    $core.String? containerId,
    $core.String? containerName,
    MonitoringContainerStats? stats,
    $fixnum.Int64? serverId,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
  }) {
    final result = MonitoringContainerMetric._();
    if (id != null) result.id = id;
    if (timestamp != null) result.timestamp = timestamp;
    if (containerId != null) result.containerId = containerId;
    if (containerName != null) result.containerName = containerName;
    if (stats != null) result.stats = stats;
    if (serverId != null) result.serverId = serverId;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    return result;
  }

  MonitoringContainerMetric._();

  factory MonitoringContainerMetric.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerMetric()..mergeFromBuffer(data, registry);
  factory MonitoringContainerMetric.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerMetric()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringContainerMetric',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringContainerMetric.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'timestamp')
    ..aOS(3, _omitFieldNames ? '' : 'containerId')
    ..aOS(4, _omitFieldNames ? '' : 'containerName')
    ..aOM<MonitoringContainerStats>(5, _omitFieldNames ? '' : 'stats',
        subBuilder: MonitoringContainerStats.$_createMessage)
    ..aInt64(6, _omitFieldNames ? '' : 'serverId')
    ..aInt64(7, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(8, _omitFieldNames ? '' : 'composeId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerMetric clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerMetric copyWith(
          void Function(MonitoringContainerMetric) updates) =>
      super.copyWith((message) => updates(message as MonitoringContainerMetric))
          as MonitoringContainerMetric;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringContainerMetric() / MonitoringContainerMetric.new instead')
  static MonitoringContainerMetric create() => MonitoringContainerMetric._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringContainerMetric._();
  @$core.override
  MonitoringContainerMetric createEmptyInstance() =>
      MonitoringContainerMetric._();
  @$core.pragma('dart2js:noInline')
  static MonitoringContainerMetric getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringContainerMetric>(
          MonitoringContainerMetric.$_createMessage);
  static MonitoringContainerMetric? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get timestamp => $_getI64(1);
  @$pb.TagNumber(2)
  set timestamp($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get containerId => $_getSZ(2);
  @$pb.TagNumber(3)
  set containerId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasContainerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearContainerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get containerName => $_getSZ(3);
  @$pb.TagNumber(4)
  set containerName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasContainerName() => $_has(3);
  @$pb.TagNumber(4)
  void clearContainerName() => $_clearField(4);

  @$pb.TagNumber(5)
  MonitoringContainerStats get stats => $_getN(4);
  @$pb.TagNumber(5)
  set stats(MonitoringContainerStats value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasStats() => $_has(4);
  @$pb.TagNumber(5)
  void clearStats() => $_clearField(5);
  @$pb.TagNumber(5)
  MonitoringContainerStats ensureStats() => $_ensure(4);

  @$pb.TagNumber(6)
  $fixnum.Int64 get serverId => $_getI64(5);
  @$pb.TagNumber(6)
  set serverId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasServerId() => $_has(5);
  @$pb.TagNumber(6)
  void clearServerId() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get applicationId => $_getI64(6);
  @$pb.TagNumber(7)
  set applicationId($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasApplicationId() => $_has(6);
  @$pb.TagNumber(7)
  void clearApplicationId() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get composeId => $_getI64(7);
  @$pb.TagNumber(8)
  set composeId($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasComposeId() => $_has(7);
  @$pb.TagNumber(8)
  void clearComposeId() => $_clearField(8);
}

/// Container state point as reported by the agent; `state` is the Docker-native status string (no panel-side closed set).
class MonitoringContainerState extends $pb.GeneratedMessage {
  factory MonitoringContainerState({
    $core.String? containerId,
    $core.String? name,
    $core.String? state,
    $core.String? status,
    $fixnum.Int64? applicationId,
    $fixnum.Int64? composeId,
    $core.String? composeProject,
    $core.String? composeService,
  }) {
    final result = MonitoringContainerState._();
    if (containerId != null) result.containerId = containerId;
    if (name != null) result.name = name;
    if (state != null) result.state = state;
    if (status != null) result.status = status;
    if (applicationId != null) result.applicationId = applicationId;
    if (composeId != null) result.composeId = composeId;
    if (composeProject != null) result.composeProject = composeProject;
    if (composeService != null) result.composeService = composeService;
    return result;
  }

  MonitoringContainerState._();

  factory MonitoringContainerState.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerState()..mergeFromBuffer(data, registry);
  factory MonitoringContainerState.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringContainerState()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringContainerState',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringContainerState.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'containerId')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'state')
    ..aOS(4, _omitFieldNames ? '' : 'status')
    ..aInt64(5, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(6, _omitFieldNames ? '' : 'composeId')
    ..aOS(7, _omitFieldNames ? '' : 'composeProject')
    ..aOS(8, _omitFieldNames ? '' : 'composeService')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerState clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringContainerState copyWith(
          void Function(MonitoringContainerState) updates) =>
      super.copyWith((message) => updates(message as MonitoringContainerState))
          as MonitoringContainerState;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringContainerState() / MonitoringContainerState.new instead')
  static MonitoringContainerState create() => MonitoringContainerState._();
  static $pb.GeneratedMessage $_createMessage() => MonitoringContainerState._();
  @$core.override
  MonitoringContainerState createEmptyInstance() =>
      MonitoringContainerState._();
  @$core.pragma('dart2js:noInline')
  static MonitoringContainerState getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringContainerState>(
          MonitoringContainerState.$_createMessage);
  static MonitoringContainerState? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get containerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set containerId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get state => $_getSZ(2);
  @$pb.TagNumber(3)
  set state($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasState() => $_has(2);
  @$pb.TagNumber(3)
  void clearState() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get status => $_getSZ(3);
  @$pb.TagNumber(4)
  set status($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get applicationId => $_getI64(4);
  @$pb.TagNumber(5)
  set applicationId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasApplicationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearApplicationId() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get composeId => $_getI64(5);
  @$pb.TagNumber(6)
  set composeId($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasComposeId() => $_has(5);
  @$pb.TagNumber(6)
  void clearComposeId() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get composeProject => $_getSZ(6);
  @$pb.TagNumber(7)
  set composeProject($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasComposeProject() => $_has(6);
  @$pb.TagNumber(7)
  void clearComposeProject() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get composeService => $_getSZ(7);
  @$pb.TagNumber(8)
  set composeService($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasComposeService() => $_has(7);
  @$pb.TagNumber(8)
  void clearComposeService() => $_clearField(8);
}

class MonitoringAgentStatus extends $pb.GeneratedMessage {
  factory MonitoringAgentStatus({
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? lastSeenAt,
    $core.String? agentVersion,
    MonitoringAgentState? state,
  }) {
    final result = MonitoringAgentStatus._();
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    if (lastSeenAt != null) result.lastSeenAt = lastSeenAt;
    if (agentVersion != null) result.agentVersion = agentVersion;
    if (state != null) result.state = state;
    return result;
  }

  MonitoringAgentStatus._();

  factory MonitoringAgentStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringAgentStatus()..mergeFromBuffer(data, registry);
  factory MonitoringAgentStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringAgentStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringAgentStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringAgentStatus.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(3, _omitFieldNames ? '' : 'lastSeenAt')
    ..aOS(4, _omitFieldNames ? '' : 'agentVersion')
    ..aE<MonitoringAgentState>(5, _omitFieldNames ? '' : 'state',
        enumValues: MonitoringAgentState.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringAgentStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringAgentStatus copyWith(
          void Function(MonitoringAgentStatus) updates) =>
      super.copyWith((message) => updates(message as MonitoringAgentStatus))
          as MonitoringAgentStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringAgentStatus() / MonitoringAgentStatus.new instead')
  static MonitoringAgentStatus create() => MonitoringAgentStatus._();
  static $pb.GeneratedMessage $_createMessage() => MonitoringAgentStatus._();
  @$core.override
  MonitoringAgentStatus createEmptyInstance() => MonitoringAgentStatus._();
  @$core.pragma('dart2js:noInline')
  static MonitoringAgentStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringAgentStatus>(
          MonitoringAgentStatus.$_createMessage);
  static MonitoringAgentStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get lastSeenAt => $_getI64(2);
  @$pb.TagNumber(3)
  set lastSeenAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLastSeenAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastSeenAt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get agentVersion => $_getSZ(3);
  @$pb.TagNumber(4)
  set agentVersion($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAgentVersion() => $_has(3);
  @$pb.TagNumber(4)
  void clearAgentVersion() => $_clearField(4);

  @$pb.TagNumber(5)
  MonitoringAgentState get state => $_getN(4);
  @$pb.TagNumber(5)
  set state(MonitoringAgentState value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasState() => $_has(4);
  @$pb.TagNumber(5)
  void clearState() => $_clearField(5);
}

/// Result of an agent lifecycle command; `action` echoes the verb ("start", "stop", "restart", "reinstall").
class MonitoringAgentAction extends $pb.GeneratedMessage {
  factory MonitoringAgentAction({
    $core.String? action,
    $core.bool? success,
    $core.String? message,
  }) {
    final result = MonitoringAgentAction._();
    if (action != null) result.action = action;
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  MonitoringAgentAction._();

  factory MonitoringAgentAction.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringAgentAction()..mergeFromBuffer(data, registry);
  factory MonitoringAgentAction.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringAgentAction()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringAgentAction',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringAgentAction.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'action')
    ..aOB(2, _omitFieldNames ? '' : 'success')
    ..aOS(3, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringAgentAction clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringAgentAction copyWith(
          void Function(MonitoringAgentAction) updates) =>
      super.copyWith((message) => updates(message as MonitoringAgentAction))
          as MonitoringAgentAction;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringAgentAction() / MonitoringAgentAction.new instead')
  static MonitoringAgentAction create() => MonitoringAgentAction._();
  static $pb.GeneratedMessage $_createMessage() => MonitoringAgentAction._();
  @$core.override
  MonitoringAgentAction createEmptyInstance() => MonitoringAgentAction._();
  @$core.pragma('dart2js:noInline')
  static MonitoringAgentAction getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringAgentAction>(
          MonitoringAgentAction.$_createMessage);
  static MonitoringAgentAction? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get action => $_getSZ(0);
  @$pb.TagNumber(1)
  set action($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get message => $_getSZ(2);
  @$pb.TagNumber(3)
  set message($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessage() => $_clearField(3);
}

class MonitoringMaintenanceWindow extends $pb.GeneratedMessage {
  factory MonitoringMaintenanceWindow({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $fixnum.Int64? serverId,
    $fixnum.Int64? startsAt,
    $fixnum.Int64? endsAt,
    $core.String? reason,
    $fixnum.Int64? createdAt,
  }) {
    final result = MonitoringMaintenanceWindow._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (serverId != null) result.serverId = serverId;
    if (startsAt != null) result.startsAt = startsAt;
    if (endsAt != null) result.endsAt = endsAt;
    if (reason != null) result.reason = reason;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  MonitoringMaintenanceWindow._();

  factory MonitoringMaintenanceWindow.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringMaintenanceWindow()..mergeFromBuffer(data, registry);
  factory MonitoringMaintenanceWindow.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringMaintenanceWindow()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringMaintenanceWindow',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringMaintenanceWindow.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(3, _omitFieldNames ? '' : 'serverId')
    ..aInt64(4, _omitFieldNames ? '' : 'startsAt')
    ..aInt64(5, _omitFieldNames ? '' : 'endsAt')
    ..aOS(6, _omitFieldNames ? '' : 'reason')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringMaintenanceWindow clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringMaintenanceWindow copyWith(
          void Function(MonitoringMaintenanceWindow) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringMaintenanceWindow))
          as MonitoringMaintenanceWindow;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringMaintenanceWindow() / MonitoringMaintenanceWindow.new instead')
  static MonitoringMaintenanceWindow create() =>
      MonitoringMaintenanceWindow._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringMaintenanceWindow._();
  @$core.override
  MonitoringMaintenanceWindow createEmptyInstance() =>
      MonitoringMaintenanceWindow._();
  @$core.pragma('dart2js:noInline')
  static MonitoringMaintenanceWindow getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringMaintenanceWindow>(
          MonitoringMaintenanceWindow.$_createMessage);
  static MonitoringMaintenanceWindow? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get serverId => $_getI64(2);
  @$pb.TagNumber(3)
  set serverId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasServerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearServerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get startsAt => $_getI64(3);
  @$pb.TagNumber(4)
  set startsAt($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasStartsAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartsAt() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get endsAt => $_getI64(4);
  @$pb.TagNumber(5)
  set endsAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEndsAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearEndsAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get reason => $_getSZ(5);
  @$pb.TagNumber(6)
  set reason($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasReason() => $_has(5);
  @$pb.TagNumber(6)
  void clearReason() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);
}

/// Legacy stub: no server-side implementation yet; envelope carries the addressed resource only.
class MonitoringGetOverviewRequest extends $pb.GeneratedMessage {
  factory MonitoringGetOverviewRequest({
    $fixnum.Int64? id,
  }) {
    final result = MonitoringGetOverviewRequest._();
    if (id != null) result.id = id;
    return result;
  }

  MonitoringGetOverviewRequest._();

  factory MonitoringGetOverviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetOverviewRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetOverviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetOverviewRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetOverviewRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetOverviewRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetOverviewRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetOverviewRequest copyWith(
          void Function(MonitoringGetOverviewRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringGetOverviewRequest))
          as MonitoringGetOverviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetOverviewRequest() / MonitoringGetOverviewRequest.new instead')
  static MonitoringGetOverviewRequest create() =>
      MonitoringGetOverviewRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetOverviewRequest._();
  @$core.override
  MonitoringGetOverviewRequest createEmptyInstance() =>
      MonitoringGetOverviewRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetOverviewRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetOverviewRequest>(
          MonitoringGetOverviewRequest.$_createMessage);
  static MonitoringGetOverviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class MonitoringGetMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringGetMetricsRequest({
    $fixnum.Int64? id,
  }) {
    final result = MonitoringGetMetricsRequest._();
    if (id != null) result.id = id;
    return result;
  }

  MonitoringGetMetricsRequest._();

  factory MonitoringGetMetricsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetMetricsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetMetricsRequest copyWith(
          void Function(MonitoringGetMetricsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringGetMetricsRequest))
          as MonitoringGetMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetMetricsRequest() / MonitoringGetMetricsRequest.new instead')
  static MonitoringGetMetricsRequest create() =>
      MonitoringGetMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetMetricsRequest._();
  @$core.override
  MonitoringGetMetricsRequest createEmptyInstance() =>
      MonitoringGetMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetMetricsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetMetricsRequest>(
          MonitoringGetMetricsRequest.$_createMessage);
  static MonitoringGetMetricsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class MonitoringGetContainerStateRequest extends $pb.GeneratedMessage {
  factory MonitoringGetContainerStateRequest({
    $fixnum.Int64? id,
  }) {
    final result = MonitoringGetContainerStateRequest._();
    if (id != null) result.id = id;
    return result;
  }

  MonitoringGetContainerStateRequest._();

  factory MonitoringGetContainerStateRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStateRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetContainerStateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStateRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetContainerStateRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetContainerStateRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStateRequest copyWith(
          void Function(MonitoringGetContainerStateRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetContainerStateRequest))
          as MonitoringGetContainerStateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetContainerStateRequest() / MonitoringGetContainerStateRequest.new instead')
  static MonitoringGetContainerStateRequest create() =>
      MonitoringGetContainerStateRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetContainerStateRequest._();
  @$core.override
  MonitoringGetContainerStateRequest createEmptyInstance() =>
      MonitoringGetContainerStateRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetContainerStateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetContainerStateRequest>(
          MonitoringGetContainerStateRequest.$_createMessage);
  static MonitoringGetContainerStateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class MonitoringGetLocalMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringGetLocalMetricsRequest() =>
      MonitoringGetLocalMetricsRequest._();

  MonitoringGetLocalMetricsRequest._();

  factory MonitoringGetLocalMetricsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetLocalMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetLocalMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetLocalMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetLocalMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetLocalMetricsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetLocalMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetLocalMetricsRequest copyWith(
          void Function(MonitoringGetLocalMetricsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringGetLocalMetricsRequest))
          as MonitoringGetLocalMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetLocalMetricsRequest() / MonitoringGetLocalMetricsRequest.new instead')
  static MonitoringGetLocalMetricsRequest create() =>
      MonitoringGetLocalMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetLocalMetricsRequest._();
  @$core.override
  MonitoringGetLocalMetricsRequest createEmptyInstance() =>
      MonitoringGetLocalMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetLocalMetricsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetLocalMetricsRequest>(
          MonitoringGetLocalMetricsRequest.$_createMessage);
  static MonitoringGetLocalMetricsRequest? _defaultInstance;
}

class MonitoringGetLocalMetricsResponse extends $pb.GeneratedMessage {
  factory MonitoringGetLocalMetricsResponse({
    $core.String? status,
    $core.String? service,
    $core.Iterable<$core.String>? endpoints,
  }) {
    final result = MonitoringGetLocalMetricsResponse._();
    if (status != null) result.status = status;
    if (service != null) result.service = service;
    if (endpoints != null) result.endpoints.addAll(endpoints);
    return result;
  }

  MonitoringGetLocalMetricsResponse._();

  factory MonitoringGetLocalMetricsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetLocalMetricsResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetLocalMetricsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetLocalMetricsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetLocalMetricsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetLocalMetricsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'status')
    ..aOS(2, _omitFieldNames ? '' : 'service')
    ..pPS(3, _omitFieldNames ? '' : 'endpoints')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetLocalMetricsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetLocalMetricsResponse copyWith(
          void Function(MonitoringGetLocalMetricsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetLocalMetricsResponse))
          as MonitoringGetLocalMetricsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetLocalMetricsResponse() / MonitoringGetLocalMetricsResponse.new instead')
  static MonitoringGetLocalMetricsResponse create() =>
      MonitoringGetLocalMetricsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetLocalMetricsResponse._();
  @$core.override
  MonitoringGetLocalMetricsResponse createEmptyInstance() =>
      MonitoringGetLocalMetricsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetLocalMetricsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetLocalMetricsResponse>(
          MonitoringGetLocalMetricsResponse.$_createMessage);
  static MonitoringGetLocalMetricsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get status => $_getSZ(0);
  @$pb.TagNumber(1)
  set status($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get service => $_getSZ(1);
  @$pb.TagNumber(2)
  set service($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasService() => $_has(1);
  @$pb.TagNumber(2)
  void clearService() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get endpoints => $_getList(2);
}

class MonitoringGetServerMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringGetServerMetricsRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringGetServerMetricsRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringGetServerMetricsRequest._();

  factory MonitoringGetServerMetricsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetServerMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetServerMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetServerMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetServerMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetServerMetricsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetServerMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetServerMetricsRequest copyWith(
          void Function(MonitoringGetServerMetricsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetServerMetricsRequest))
          as MonitoringGetServerMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetServerMetricsRequest() / MonitoringGetServerMetricsRequest.new instead')
  static MonitoringGetServerMetricsRequest create() =>
      MonitoringGetServerMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetServerMetricsRequest._();
  @$core.override
  MonitoringGetServerMetricsRequest createEmptyInstance() =>
      MonitoringGetServerMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetServerMetricsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetServerMetricsRequest>(
          MonitoringGetServerMetricsRequest.$_createMessage);
  static MonitoringGetServerMetricsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

/// History capped server-side at 50 samples.
class MonitoringGetServerMetricsResponse extends $pb.GeneratedMessage {
  factory MonitoringGetServerMetricsResponse({
    $core.Iterable<MonitoringServerMetric>? metrics,
  }) {
    final result = MonitoringGetServerMetricsResponse._();
    if (metrics != null) result.metrics.addAll(metrics);
    return result;
  }

  MonitoringGetServerMetricsResponse._();

  factory MonitoringGetServerMetricsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetServerMetricsResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetServerMetricsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetServerMetricsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetServerMetricsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetServerMetricsResponse.$_createMessage)
    ..pPM<MonitoringServerMetric>(1, _omitFieldNames ? '' : 'metrics',
        subBuilder: MonitoringServerMetric.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetServerMetricsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetServerMetricsResponse copyWith(
          void Function(MonitoringGetServerMetricsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetServerMetricsResponse))
          as MonitoringGetServerMetricsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetServerMetricsResponse() / MonitoringGetServerMetricsResponse.new instead')
  static MonitoringGetServerMetricsResponse create() =>
      MonitoringGetServerMetricsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetServerMetricsResponse._();
  @$core.override
  MonitoringGetServerMetricsResponse createEmptyInstance() =>
      MonitoringGetServerMetricsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetServerMetricsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetServerMetricsResponse>(
          MonitoringGetServerMetricsResponse.$_createMessage);
  static MonitoringGetServerMetricsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<MonitoringServerMetric> get metrics => $_getList(0);
}

class MonitoringGetContainerMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringGetContainerMetricsRequest({
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
    $core.String? appName,
    $fixnum.Int64? limit,
  }) {
    final result = MonitoringGetContainerMetricsRequest._();
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    if (appName != null) result.appName = appName;
    if (limit != null) result.limit = limit;
    return result;
  }

  MonitoringGetContainerMetricsRequest._();

  factory MonitoringGetContainerMetricsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetContainerMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetContainerMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetContainerMetricsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'appName')
    ..aInt64(4, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerMetricsRequest copyWith(
          void Function(MonitoringGetContainerMetricsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetContainerMetricsRequest))
          as MonitoringGetContainerMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetContainerMetricsRequest() / MonitoringGetContainerMetricsRequest.new instead')
  static MonitoringGetContainerMetricsRequest create() =>
      MonitoringGetContainerMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetContainerMetricsRequest._();
  @$core.override
  MonitoringGetContainerMetricsRequest createEmptyInstance() =>
      MonitoringGetContainerMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetContainerMetricsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetContainerMetricsRequest>(
          MonitoringGetContainerMetricsRequest.$_createMessage);
  static MonitoringGetContainerMetricsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get appName => $_getSZ(2);
  @$pb.TagNumber(3)
  set appName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAppName() => $_has(2);
  @$pb.TagNumber(3)
  void clearAppName() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get limit => $_getI64(3);
  @$pb.TagNumber(4)
  set limit($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLimit() => $_has(3);
  @$pb.TagNumber(4)
  void clearLimit() => $_clearField(4);
}

class MonitoringGetContainerMetricsResponse extends $pb.GeneratedMessage {
  factory MonitoringGetContainerMetricsResponse({
    $core.Iterable<MonitoringContainerMetric>? metrics,
  }) {
    final result = MonitoringGetContainerMetricsResponse._();
    if (metrics != null) result.metrics.addAll(metrics);
    return result;
  }

  MonitoringGetContainerMetricsResponse._();

  factory MonitoringGetContainerMetricsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerMetricsResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetContainerMetricsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerMetricsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetContainerMetricsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringGetContainerMetricsResponse.$_createMessage)
    ..pPM<MonitoringContainerMetric>(1, _omitFieldNames ? '' : 'metrics',
        subBuilder: MonitoringContainerMetric.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerMetricsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerMetricsResponse copyWith(
          void Function(MonitoringGetContainerMetricsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetContainerMetricsResponse))
          as MonitoringGetContainerMetricsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetContainerMetricsResponse() / MonitoringGetContainerMetricsResponse.new instead')
  static MonitoringGetContainerMetricsResponse create() =>
      MonitoringGetContainerMetricsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetContainerMetricsResponse._();
  @$core.override
  MonitoringGetContainerMetricsResponse createEmptyInstance() =>
      MonitoringGetContainerMetricsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetContainerMetricsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetContainerMetricsResponse>(
          MonitoringGetContainerMetricsResponse.$_createMessage);
  static MonitoringGetContainerMetricsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<MonitoringContainerMetric> get metrics => $_getList(0);
}

class MonitoringGetContainerStatesRequest extends $pb.GeneratedMessage {
  factory MonitoringGetContainerStatesRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringGetContainerStatesRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringGetContainerStatesRequest._();

  factory MonitoringGetContainerStatesRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStatesRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetContainerStatesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStatesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetContainerStatesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetContainerStatesRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStatesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStatesRequest copyWith(
          void Function(MonitoringGetContainerStatesRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetContainerStatesRequest))
          as MonitoringGetContainerStatesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetContainerStatesRequest() / MonitoringGetContainerStatesRequest.new instead')
  static MonitoringGetContainerStatesRequest create() =>
      MonitoringGetContainerStatesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetContainerStatesRequest._();
  @$core.override
  MonitoringGetContainerStatesRequest createEmptyInstance() =>
      MonitoringGetContainerStatesRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetContainerStatesRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetContainerStatesRequest>(
          MonitoringGetContainerStatesRequest.$_createMessage);
  static MonitoringGetContainerStatesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringGetContainerStatesResponse extends $pb.GeneratedMessage {
  factory MonitoringGetContainerStatesResponse({
    $core.Iterable<MonitoringContainerState>? states,
  }) {
    final result = MonitoringGetContainerStatesResponse._();
    if (states != null) result.states.addAll(states);
    return result;
  }

  MonitoringGetContainerStatesResponse._();

  factory MonitoringGetContainerStatesResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStatesResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetContainerStatesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetContainerStatesResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetContainerStatesResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetContainerStatesResponse.$_createMessage)
    ..pPM<MonitoringContainerState>(1, _omitFieldNames ? '' : 'states',
        subBuilder: MonitoringContainerState.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStatesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetContainerStatesResponse copyWith(
          void Function(MonitoringGetContainerStatesResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetContainerStatesResponse))
          as MonitoringGetContainerStatesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetContainerStatesResponse() / MonitoringGetContainerStatesResponse.new instead')
  static MonitoringGetContainerStatesResponse create() =>
      MonitoringGetContainerStatesResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetContainerStatesResponse._();
  @$core.override
  MonitoringGetContainerStatesResponse createEmptyInstance() =>
      MonitoringGetContainerStatesResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetContainerStatesResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetContainerStatesResponse>(
          MonitoringGetContainerStatesResponse.$_createMessage);
  static MonitoringGetContainerStatesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<MonitoringContainerState> get states => $_getList(0);
}

/// Rotates the server's agent token; the plaintext token is returned once.
class MonitoringCreateAgentTokenRequest extends $pb.GeneratedMessage {
  factory MonitoringCreateAgentTokenRequest({
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
  }) {
    final result = MonitoringCreateAgentTokenRequest._();
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  MonitoringCreateAgentTokenRequest._();

  factory MonitoringCreateAgentTokenRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateAgentTokenRequest()..mergeFromBuffer(data, registry);
  factory MonitoringCreateAgentTokenRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateAgentTokenRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringCreateAgentTokenRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringCreateAgentTokenRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateAgentTokenRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateAgentTokenRequest copyWith(
          void Function(MonitoringCreateAgentTokenRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringCreateAgentTokenRequest))
          as MonitoringCreateAgentTokenRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringCreateAgentTokenRequest() / MonitoringCreateAgentTokenRequest.new instead')
  static MonitoringCreateAgentTokenRequest create() =>
      MonitoringCreateAgentTokenRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringCreateAgentTokenRequest._();
  @$core.override
  MonitoringCreateAgentTokenRequest createEmptyInstance() =>
      MonitoringCreateAgentTokenRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringCreateAgentTokenRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringCreateAgentTokenRequest>(
          MonitoringCreateAgentTokenRequest.$_createMessage);
  static MonitoringCreateAgentTokenRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

class MonitoringCreateAgentTokenResponse extends $pb.GeneratedMessage {
  factory MonitoringCreateAgentTokenResponse({
    $fixnum.Int64? serverId,
    $fixnum.Int64? organizationId,
    $core.String? token,
  }) {
    final result = MonitoringCreateAgentTokenResponse._();
    if (serverId != null) result.serverId = serverId;
    if (organizationId != null) result.organizationId = organizationId;
    if (token != null) result.token = token;
    return result;
  }

  MonitoringCreateAgentTokenResponse._();

  factory MonitoringCreateAgentTokenResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateAgentTokenResponse()..mergeFromBuffer(data, registry);
  factory MonitoringCreateAgentTokenResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateAgentTokenResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringCreateAgentTokenResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringCreateAgentTokenResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateAgentTokenResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateAgentTokenResponse copyWith(
          void Function(MonitoringCreateAgentTokenResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringCreateAgentTokenResponse))
          as MonitoringCreateAgentTokenResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringCreateAgentTokenResponse() / MonitoringCreateAgentTokenResponse.new instead')
  static MonitoringCreateAgentTokenResponse create() =>
      MonitoringCreateAgentTokenResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringCreateAgentTokenResponse._();
  @$core.override
  MonitoringCreateAgentTokenResponse createEmptyInstance() =>
      MonitoringCreateAgentTokenResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringCreateAgentTokenResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringCreateAgentTokenResponse>(
          MonitoringCreateAgentTokenResponse.$_createMessage);
  static MonitoringCreateAgentTokenResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get token => $_getSZ(2);
  @$pb.TagNumber(3)
  set token($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearToken() => $_clearField(3);
}

class MonitoringGetAgentStatusRequest extends $pb.GeneratedMessage {
  factory MonitoringGetAgentStatusRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringGetAgentStatusRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringGetAgentStatusRequest._();

  factory MonitoringGetAgentStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetAgentStatusRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetAgentStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetAgentStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetAgentStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetAgentStatusRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetAgentStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetAgentStatusRequest copyWith(
          void Function(MonitoringGetAgentStatusRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringGetAgentStatusRequest))
          as MonitoringGetAgentStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetAgentStatusRequest() / MonitoringGetAgentStatusRequest.new instead')
  static MonitoringGetAgentStatusRequest create() =>
      MonitoringGetAgentStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetAgentStatusRequest._();
  @$core.override
  MonitoringGetAgentStatusRequest createEmptyInstance() =>
      MonitoringGetAgentStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetAgentStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetAgentStatusRequest>(
          MonitoringGetAgentStatusRequest.$_createMessage);
  static MonitoringGetAgentStatusRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringGetAgentStatusResponse extends $pb.GeneratedMessage {
  factory MonitoringGetAgentStatusResponse({
    MonitoringAgentStatus? status,
  }) {
    final result = MonitoringGetAgentStatusResponse._();
    if (status != null) result.status = status;
    return result;
  }

  MonitoringGetAgentStatusResponse._();

  factory MonitoringGetAgentStatusResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetAgentStatusResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetAgentStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetAgentStatusResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetAgentStatusResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringGetAgentStatusResponse.$_createMessage)
    ..aOM<MonitoringAgentStatus>(1, _omitFieldNames ? '' : 'status',
        subBuilder: MonitoringAgentStatus.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetAgentStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetAgentStatusResponse copyWith(
          void Function(MonitoringGetAgentStatusResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringGetAgentStatusResponse))
          as MonitoringGetAgentStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetAgentStatusResponse() / MonitoringGetAgentStatusResponse.new instead')
  static MonitoringGetAgentStatusResponse create() =>
      MonitoringGetAgentStatusResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetAgentStatusResponse._();
  @$core.override
  MonitoringGetAgentStatusResponse createEmptyInstance() =>
      MonitoringGetAgentStatusResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetAgentStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringGetAgentStatusResponse>(
          MonitoringGetAgentStatusResponse.$_createMessage);
  static MonitoringGetAgentStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  MonitoringAgentStatus get status => $_getN(0);
  @$pb.TagNumber(1)
  set status(MonitoringAgentStatus value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
  @$pb.TagNumber(1)
  MonitoringAgentStatus ensureStatus() => $_ensure(0);
}

class MonitoringStartAgentRequest extends $pb.GeneratedMessage {
  factory MonitoringStartAgentRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringStartAgentRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringStartAgentRequest._();

  factory MonitoringStartAgentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStartAgentRequest()..mergeFromBuffer(data, registry);
  factory MonitoringStartAgentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStartAgentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringStartAgentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringStartAgentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStartAgentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStartAgentRequest copyWith(
          void Function(MonitoringStartAgentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringStartAgentRequest))
          as MonitoringStartAgentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringStartAgentRequest() / MonitoringStartAgentRequest.new instead')
  static MonitoringStartAgentRequest create() =>
      MonitoringStartAgentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringStartAgentRequest._();
  @$core.override
  MonitoringStartAgentRequest createEmptyInstance() =>
      MonitoringStartAgentRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringStartAgentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringStartAgentRequest>(
          MonitoringStartAgentRequest.$_createMessage);
  static MonitoringStartAgentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringStartAgentResponse extends $pb.GeneratedMessage {
  factory MonitoringStartAgentResponse({
    MonitoringAgentAction? action,
  }) {
    final result = MonitoringStartAgentResponse._();
    if (action != null) result.action = action;
    return result;
  }

  MonitoringStartAgentResponse._();

  factory MonitoringStartAgentResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStartAgentResponse()..mergeFromBuffer(data, registry);
  factory MonitoringStartAgentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStartAgentResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringStartAgentResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringStartAgentResponse.$_createMessage)
    ..aOM<MonitoringAgentAction>(1, _omitFieldNames ? '' : 'action',
        subBuilder: MonitoringAgentAction.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStartAgentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStartAgentResponse copyWith(
          void Function(MonitoringStartAgentResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringStartAgentResponse))
          as MonitoringStartAgentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringStartAgentResponse() / MonitoringStartAgentResponse.new instead')
  static MonitoringStartAgentResponse create() =>
      MonitoringStartAgentResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringStartAgentResponse._();
  @$core.override
  MonitoringStartAgentResponse createEmptyInstance() =>
      MonitoringStartAgentResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringStartAgentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringStartAgentResponse>(
          MonitoringStartAgentResponse.$_createMessage);
  static MonitoringStartAgentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  MonitoringAgentAction get action => $_getN(0);
  @$pb.TagNumber(1)
  set action(MonitoringAgentAction value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);
  @$pb.TagNumber(1)
  MonitoringAgentAction ensureAction() => $_ensure(0);
}

class MonitoringStopAgentRequest extends $pb.GeneratedMessage {
  factory MonitoringStopAgentRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringStopAgentRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringStopAgentRequest._();

  factory MonitoringStopAgentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStopAgentRequest()..mergeFromBuffer(data, registry);
  factory MonitoringStopAgentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStopAgentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringStopAgentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringStopAgentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStopAgentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStopAgentRequest copyWith(
          void Function(MonitoringStopAgentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringStopAgentRequest))
          as MonitoringStopAgentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringStopAgentRequest() / MonitoringStopAgentRequest.new instead')
  static MonitoringStopAgentRequest create() => MonitoringStopAgentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringStopAgentRequest._();
  @$core.override
  MonitoringStopAgentRequest createEmptyInstance() =>
      MonitoringStopAgentRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringStopAgentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringStopAgentRequest>(
          MonitoringStopAgentRequest.$_createMessage);
  static MonitoringStopAgentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringStopAgentResponse extends $pb.GeneratedMessage {
  factory MonitoringStopAgentResponse({
    MonitoringAgentAction? action,
  }) {
    final result = MonitoringStopAgentResponse._();
    if (action != null) result.action = action;
    return result;
  }

  MonitoringStopAgentResponse._();

  factory MonitoringStopAgentResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStopAgentResponse()..mergeFromBuffer(data, registry);
  factory MonitoringStopAgentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringStopAgentResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringStopAgentResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringStopAgentResponse.$_createMessage)
    ..aOM<MonitoringAgentAction>(1, _omitFieldNames ? '' : 'action',
        subBuilder: MonitoringAgentAction.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStopAgentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringStopAgentResponse copyWith(
          void Function(MonitoringStopAgentResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringStopAgentResponse))
          as MonitoringStopAgentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringStopAgentResponse() / MonitoringStopAgentResponse.new instead')
  static MonitoringStopAgentResponse create() =>
      MonitoringStopAgentResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringStopAgentResponse._();
  @$core.override
  MonitoringStopAgentResponse createEmptyInstance() =>
      MonitoringStopAgentResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringStopAgentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringStopAgentResponse>(
          MonitoringStopAgentResponse.$_createMessage);
  static MonitoringStopAgentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  MonitoringAgentAction get action => $_getN(0);
  @$pb.TagNumber(1)
  set action(MonitoringAgentAction value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);
  @$pb.TagNumber(1)
  MonitoringAgentAction ensureAction() => $_ensure(0);
}

class MonitoringRestartAgentRequest extends $pb.GeneratedMessage {
  factory MonitoringRestartAgentRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringRestartAgentRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringRestartAgentRequest._();

  factory MonitoringRestartAgentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringRestartAgentRequest()..mergeFromBuffer(data, registry);
  factory MonitoringRestartAgentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringRestartAgentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringRestartAgentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringRestartAgentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringRestartAgentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringRestartAgentRequest copyWith(
          void Function(MonitoringRestartAgentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringRestartAgentRequest))
          as MonitoringRestartAgentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringRestartAgentRequest() / MonitoringRestartAgentRequest.new instead')
  static MonitoringRestartAgentRequest create() =>
      MonitoringRestartAgentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringRestartAgentRequest._();
  @$core.override
  MonitoringRestartAgentRequest createEmptyInstance() =>
      MonitoringRestartAgentRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringRestartAgentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringRestartAgentRequest>(
          MonitoringRestartAgentRequest.$_createMessage);
  static MonitoringRestartAgentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringRestartAgentResponse extends $pb.GeneratedMessage {
  factory MonitoringRestartAgentResponse({
    MonitoringAgentAction? action,
  }) {
    final result = MonitoringRestartAgentResponse._();
    if (action != null) result.action = action;
    return result;
  }

  MonitoringRestartAgentResponse._();

  factory MonitoringRestartAgentResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringRestartAgentResponse()..mergeFromBuffer(data, registry);
  factory MonitoringRestartAgentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringRestartAgentResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringRestartAgentResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringRestartAgentResponse.$_createMessage)
    ..aOM<MonitoringAgentAction>(1, _omitFieldNames ? '' : 'action',
        subBuilder: MonitoringAgentAction.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringRestartAgentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringRestartAgentResponse copyWith(
          void Function(MonitoringRestartAgentResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringRestartAgentResponse))
          as MonitoringRestartAgentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringRestartAgentResponse() / MonitoringRestartAgentResponse.new instead')
  static MonitoringRestartAgentResponse create() =>
      MonitoringRestartAgentResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringRestartAgentResponse._();
  @$core.override
  MonitoringRestartAgentResponse createEmptyInstance() =>
      MonitoringRestartAgentResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringRestartAgentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringRestartAgentResponse>(
          MonitoringRestartAgentResponse.$_createMessage);
  static MonitoringRestartAgentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  MonitoringAgentAction get action => $_getN(0);
  @$pb.TagNumber(1)
  set action(MonitoringAgentAction value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);
  @$pb.TagNumber(1)
  MonitoringAgentAction ensureAction() => $_ensure(0);
}

class MonitoringReinstallAgentRequest extends $pb.GeneratedMessage {
  factory MonitoringReinstallAgentRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringReinstallAgentRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringReinstallAgentRequest._();

  factory MonitoringReinstallAgentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringReinstallAgentRequest()..mergeFromBuffer(data, registry);
  factory MonitoringReinstallAgentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringReinstallAgentRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringReinstallAgentRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringReinstallAgentRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringReinstallAgentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringReinstallAgentRequest copyWith(
          void Function(MonitoringReinstallAgentRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringReinstallAgentRequest))
          as MonitoringReinstallAgentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringReinstallAgentRequest() / MonitoringReinstallAgentRequest.new instead')
  static MonitoringReinstallAgentRequest create() =>
      MonitoringReinstallAgentRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringReinstallAgentRequest._();
  @$core.override
  MonitoringReinstallAgentRequest createEmptyInstance() =>
      MonitoringReinstallAgentRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringReinstallAgentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringReinstallAgentRequest>(
          MonitoringReinstallAgentRequest.$_createMessage);
  static MonitoringReinstallAgentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringReinstallAgentResponse extends $pb.GeneratedMessage {
  factory MonitoringReinstallAgentResponse({
    MonitoringAgentAction? action,
  }) {
    final result = MonitoringReinstallAgentResponse._();
    if (action != null) result.action = action;
    return result;
  }

  MonitoringReinstallAgentResponse._();

  factory MonitoringReinstallAgentResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringReinstallAgentResponse()..mergeFromBuffer(data, registry);
  factory MonitoringReinstallAgentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringReinstallAgentResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringReinstallAgentResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringReinstallAgentResponse.$_createMessage)
    ..aOM<MonitoringAgentAction>(1, _omitFieldNames ? '' : 'action',
        subBuilder: MonitoringAgentAction.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringReinstallAgentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringReinstallAgentResponse copyWith(
          void Function(MonitoringReinstallAgentResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MonitoringReinstallAgentResponse))
          as MonitoringReinstallAgentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringReinstallAgentResponse() / MonitoringReinstallAgentResponse.new instead')
  static MonitoringReinstallAgentResponse create() =>
      MonitoringReinstallAgentResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringReinstallAgentResponse._();
  @$core.override
  MonitoringReinstallAgentResponse createEmptyInstance() =>
      MonitoringReinstallAgentResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringReinstallAgentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MonitoringReinstallAgentResponse>(
          MonitoringReinstallAgentResponse.$_createMessage);
  static MonitoringReinstallAgentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  MonitoringAgentAction get action => $_getN(0);
  @$pb.TagNumber(1)
  set action(MonitoringAgentAction value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAction() => $_has(0);
  @$pb.TagNumber(1)
  void clearAction() => $_clearField(1);
  @$pb.TagNumber(1)
  MonitoringAgentAction ensureAction() => $_ensure(0);
}

class MonitoringExportServerMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringExportServerMetricsRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringExportServerMetricsRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringExportServerMetricsRequest._();

  factory MonitoringExportServerMetricsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringExportServerMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringExportServerMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringExportServerMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringExportServerMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: MonitoringExportServerMetricsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringExportServerMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringExportServerMetricsRequest copyWith(
          void Function(MonitoringExportServerMetricsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringExportServerMetricsRequest))
          as MonitoringExportServerMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringExportServerMetricsRequest() / MonitoringExportServerMetricsRequest.new instead')
  static MonitoringExportServerMetricsRequest create() =>
      MonitoringExportServerMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringExportServerMetricsRequest._();
  @$core.override
  MonitoringExportServerMetricsRequest createEmptyInstance() =>
      MonitoringExportServerMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringExportServerMetricsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringExportServerMetricsRequest>(
          MonitoringExportServerMetricsRequest.$_createMessage);
  static MonitoringExportServerMetricsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

/// CSV export (up to 1000 samples) with header timestamp,server_id,cpu,mem_used,mem_total,disk_used,total_disk,network_in,network_out.
class MonitoringExportServerMetricsResponse extends $pb.GeneratedMessage {
  factory MonitoringExportServerMetricsResponse({
    $core.String? csv,
  }) {
    final result = MonitoringExportServerMetricsResponse._();
    if (csv != null) result.csv = csv;
    return result;
  }

  MonitoringExportServerMetricsResponse._();

  factory MonitoringExportServerMetricsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringExportServerMetricsResponse()..mergeFromBuffer(data, registry);
  factory MonitoringExportServerMetricsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringExportServerMetricsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringExportServerMetricsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringExportServerMetricsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'csv')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringExportServerMetricsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringExportServerMetricsResponse copyWith(
          void Function(MonitoringExportServerMetricsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringExportServerMetricsResponse))
          as MonitoringExportServerMetricsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringExportServerMetricsResponse() / MonitoringExportServerMetricsResponse.new instead')
  static MonitoringExportServerMetricsResponse create() =>
      MonitoringExportServerMetricsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringExportServerMetricsResponse._();
  @$core.override
  MonitoringExportServerMetricsResponse createEmptyInstance() =>
      MonitoringExportServerMetricsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringExportServerMetricsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringExportServerMetricsResponse>(
          MonitoringExportServerMetricsResponse.$_createMessage);
  static MonitoringExportServerMetricsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get csv => $_getSZ(0);
  @$pb.TagNumber(1)
  set csv($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCsv() => $_has(0);
  @$pb.TagNumber(1)
  void clearCsv() => $_clearField(1);
}

/// Single latest sample rendered in Prometheus exposition format.
class MonitoringGetPrometheusMetricsRequest extends $pb.GeneratedMessage {
  factory MonitoringGetPrometheusMetricsRequest({
    $fixnum.Int64? serverId,
  }) {
    final result = MonitoringGetPrometheusMetricsRequest._();
    if (serverId != null) result.serverId = serverId;
    return result;
  }

  MonitoringGetPrometheusMetricsRequest._();

  factory MonitoringGetPrometheusMetricsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetPrometheusMetricsRequest()..mergeFromBuffer(data, registry);
  factory MonitoringGetPrometheusMetricsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetPrometheusMetricsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetPrometheusMetricsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringGetPrometheusMetricsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetPrometheusMetricsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetPrometheusMetricsRequest copyWith(
          void Function(MonitoringGetPrometheusMetricsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetPrometheusMetricsRequest))
          as MonitoringGetPrometheusMetricsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetPrometheusMetricsRequest() / MonitoringGetPrometheusMetricsRequest.new instead')
  static MonitoringGetPrometheusMetricsRequest create() =>
      MonitoringGetPrometheusMetricsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetPrometheusMetricsRequest._();
  @$core.override
  MonitoringGetPrometheusMetricsRequest createEmptyInstance() =>
      MonitoringGetPrometheusMetricsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetPrometheusMetricsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetPrometheusMetricsRequest>(
          MonitoringGetPrometheusMetricsRequest.$_createMessage);
  static MonitoringGetPrometheusMetricsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);
}

class MonitoringGetPrometheusMetricsResponse extends $pb.GeneratedMessage {
  factory MonitoringGetPrometheusMetricsResponse({
    $core.String? prometheusText,
  }) {
    final result = MonitoringGetPrometheusMetricsResponse._();
    if (prometheusText != null) result.prometheusText = prometheusText;
    return result;
  }

  MonitoringGetPrometheusMetricsResponse._();

  factory MonitoringGetPrometheusMetricsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetPrometheusMetricsResponse()..mergeFromBuffer(data, registry);
  factory MonitoringGetPrometheusMetricsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringGetPrometheusMetricsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringGetPrometheusMetricsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringGetPrometheusMetricsResponse.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'prometheusText')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetPrometheusMetricsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringGetPrometheusMetricsResponse copyWith(
          void Function(MonitoringGetPrometheusMetricsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringGetPrometheusMetricsResponse))
          as MonitoringGetPrometheusMetricsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringGetPrometheusMetricsResponse() / MonitoringGetPrometheusMetricsResponse.new instead')
  static MonitoringGetPrometheusMetricsResponse create() =>
      MonitoringGetPrometheusMetricsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringGetPrometheusMetricsResponse._();
  @$core.override
  MonitoringGetPrometheusMetricsResponse createEmptyInstance() =>
      MonitoringGetPrometheusMetricsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringGetPrometheusMetricsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringGetPrometheusMetricsResponse>(
          MonitoringGetPrometheusMetricsResponse.$_createMessage);
  static MonitoringGetPrometheusMetricsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get prometheusText => $_getSZ(0);
  @$pb.TagNumber(1)
  set prometheusText($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPrometheusText() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrometheusText() => $_clearField(1);
}

/// `ends_at` must follow `starts_at`; absent organization_id uses the authenticated scope.
class MonitoringCreateMaintenanceWindowRequest extends $pb.GeneratedMessage {
  factory MonitoringCreateMaintenanceWindowRequest({
    $fixnum.Int64? serverId,
    $fixnum.Int64? startsAt,
    $fixnum.Int64? endsAt,
    $core.String? reason,
    $fixnum.Int64? organizationId,
  }) {
    final result = MonitoringCreateMaintenanceWindowRequest._();
    if (serverId != null) result.serverId = serverId;
    if (startsAt != null) result.startsAt = startsAt;
    if (endsAt != null) result.endsAt = endsAt;
    if (reason != null) result.reason = reason;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  MonitoringCreateMaintenanceWindowRequest._();

  factory MonitoringCreateMaintenanceWindowRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateMaintenanceWindowRequest()
        ..mergeFromBuffer(data, registry);
  factory MonitoringCreateMaintenanceWindowRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateMaintenanceWindowRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringCreateMaintenanceWindowRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringCreateMaintenanceWindowRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'serverId')
    ..aInt64(2, _omitFieldNames ? '' : 'startsAt')
    ..aInt64(3, _omitFieldNames ? '' : 'endsAt')
    ..aOS(4, _omitFieldNames ? '' : 'reason')
    ..aInt64(5, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateMaintenanceWindowRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateMaintenanceWindowRequest copyWith(
          void Function(MonitoringCreateMaintenanceWindowRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringCreateMaintenanceWindowRequest))
          as MonitoringCreateMaintenanceWindowRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringCreateMaintenanceWindowRequest() / MonitoringCreateMaintenanceWindowRequest.new instead')
  static MonitoringCreateMaintenanceWindowRequest create() =>
      MonitoringCreateMaintenanceWindowRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringCreateMaintenanceWindowRequest._();
  @$core.override
  MonitoringCreateMaintenanceWindowRequest createEmptyInstance() =>
      MonitoringCreateMaintenanceWindowRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringCreateMaintenanceWindowRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringCreateMaintenanceWindowRequest>(
          MonitoringCreateMaintenanceWindowRequest.$_createMessage);
  static MonitoringCreateMaintenanceWindowRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get serverId => $_getI64(0);
  @$pb.TagNumber(1)
  set serverId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get startsAt => $_getI64(1);
  @$pb.TagNumber(2)
  set startsAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStartsAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearStartsAt() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get endsAt => $_getI64(2);
  @$pb.TagNumber(3)
  set endsAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasEndsAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearEndsAt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get reason => $_getSZ(3);
  @$pb.TagNumber(4)
  set reason($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasReason() => $_has(3);
  @$pb.TagNumber(4)
  void clearReason() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get organizationId => $_getI64(4);
  @$pb.TagNumber(5)
  set organizationId($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOrganizationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearOrganizationId() => $_clearField(5);
}

class MonitoringCreateMaintenanceWindowResponse extends $pb.GeneratedMessage {
  factory MonitoringCreateMaintenanceWindowResponse({
    $fixnum.Int64? id,
  }) {
    final result = MonitoringCreateMaintenanceWindowResponse._();
    if (id != null) result.id = id;
    return result;
  }

  MonitoringCreateMaintenanceWindowResponse._();

  factory MonitoringCreateMaintenanceWindowResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateMaintenanceWindowResponse()
        ..mergeFromBuffer(data, registry);
  factory MonitoringCreateMaintenanceWindowResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringCreateMaintenanceWindowResponse()
        ..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringCreateMaintenanceWindowResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringCreateMaintenanceWindowResponse.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateMaintenanceWindowResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringCreateMaintenanceWindowResponse copyWith(
          void Function(MonitoringCreateMaintenanceWindowResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringCreateMaintenanceWindowResponse))
          as MonitoringCreateMaintenanceWindowResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringCreateMaintenanceWindowResponse() / MonitoringCreateMaintenanceWindowResponse.new instead')
  static MonitoringCreateMaintenanceWindowResponse create() =>
      MonitoringCreateMaintenanceWindowResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringCreateMaintenanceWindowResponse._();
  @$core.override
  MonitoringCreateMaintenanceWindowResponse createEmptyInstance() =>
      MonitoringCreateMaintenanceWindowResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringCreateMaintenanceWindowResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringCreateMaintenanceWindowResponse>(
          MonitoringCreateMaintenanceWindowResponse.$_createMessage);
  static MonitoringCreateMaintenanceWindowResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class MonitoringListMaintenanceWindowsRequest extends $pb.GeneratedMessage {
  factory MonitoringListMaintenanceWindowsRequest({
    $fixnum.Int64? organizationId,
    $fixnum.Int64? limit,
    $fixnum.Int64? offset,
  }) {
    final result = MonitoringListMaintenanceWindowsRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    if (limit != null) result.limit = limit;
    if (offset != null) result.offset = offset;
    return result;
  }

  MonitoringListMaintenanceWindowsRequest._();

  factory MonitoringListMaintenanceWindowsRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringListMaintenanceWindowsRequest()
        ..mergeFromBuffer(data, registry);
  factory MonitoringListMaintenanceWindowsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringListMaintenanceWindowsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringListMaintenanceWindowsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringListMaintenanceWindowsRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..aInt64(2, _omitFieldNames ? '' : 'limit')
    ..aInt64(3, _omitFieldNames ? '' : 'offset')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringListMaintenanceWindowsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringListMaintenanceWindowsRequest copyWith(
          void Function(MonitoringListMaintenanceWindowsRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringListMaintenanceWindowsRequest))
          as MonitoringListMaintenanceWindowsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringListMaintenanceWindowsRequest() / MonitoringListMaintenanceWindowsRequest.new instead')
  static MonitoringListMaintenanceWindowsRequest create() =>
      MonitoringListMaintenanceWindowsRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringListMaintenanceWindowsRequest._();
  @$core.override
  MonitoringListMaintenanceWindowsRequest createEmptyInstance() =>
      MonitoringListMaintenanceWindowsRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringListMaintenanceWindowsRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringListMaintenanceWindowsRequest>(
          MonitoringListMaintenanceWindowsRequest.$_createMessage);
  static MonitoringListMaintenanceWindowsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(2)
  set limit($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get offset => $_getI64(2);
  @$pb.TagNumber(3)
  set offset($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOffset() => $_has(2);
  @$pb.TagNumber(3)
  void clearOffset() => $_clearField(3);
}

class MonitoringListMaintenanceWindowsResponse extends $pb.GeneratedMessage {
  factory MonitoringListMaintenanceWindowsResponse({
    $core.Iterable<MonitoringMaintenanceWindow>? windows,
  }) {
    final result = MonitoringListMaintenanceWindowsResponse._();
    if (windows != null) result.windows.addAll(windows);
    return result;
  }

  MonitoringListMaintenanceWindowsResponse._();

  factory MonitoringListMaintenanceWindowsResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringListMaintenanceWindowsResponse()
        ..mergeFromBuffer(data, registry);
  factory MonitoringListMaintenanceWindowsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringListMaintenanceWindowsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringListMaintenanceWindowsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringListMaintenanceWindowsResponse.$_createMessage)
    ..pPM<MonitoringMaintenanceWindow>(1, _omitFieldNames ? '' : 'windows',
        subBuilder: MonitoringMaintenanceWindow.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringListMaintenanceWindowsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringListMaintenanceWindowsResponse copyWith(
          void Function(MonitoringListMaintenanceWindowsResponse) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringListMaintenanceWindowsResponse))
          as MonitoringListMaintenanceWindowsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringListMaintenanceWindowsResponse() / MonitoringListMaintenanceWindowsResponse.new instead')
  static MonitoringListMaintenanceWindowsResponse create() =>
      MonitoringListMaintenanceWindowsResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringListMaintenanceWindowsResponse._();
  @$core.override
  MonitoringListMaintenanceWindowsResponse createEmptyInstance() =>
      MonitoringListMaintenanceWindowsResponse._();
  @$core.pragma('dart2js:noInline')
  static MonitoringListMaintenanceWindowsResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringListMaintenanceWindowsResponse>(
          MonitoringListMaintenanceWindowsResponse.$_createMessage);
  static MonitoringListMaintenanceWindowsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<MonitoringMaintenanceWindow> get windows => $_getList(0);
}

class MonitoringDeleteMaintenanceWindowRequest extends $pb.GeneratedMessage {
  factory MonitoringDeleteMaintenanceWindowRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = MonitoringDeleteMaintenanceWindowRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  MonitoringDeleteMaintenanceWindowRequest._();

  factory MonitoringDeleteMaintenanceWindowRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringDeleteMaintenanceWindowRequest()
        ..mergeFromBuffer(data, registry);
  factory MonitoringDeleteMaintenanceWindowRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      MonitoringDeleteMaintenanceWindowRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MonitoringDeleteMaintenanceWindowRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance:
          MonitoringDeleteMaintenanceWindowRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringDeleteMaintenanceWindowRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MonitoringDeleteMaintenanceWindowRequest copyWith(
          void Function(MonitoringDeleteMaintenanceWindowRequest) updates) =>
      super.copyWith((message) =>
              updates(message as MonitoringDeleteMaintenanceWindowRequest))
          as MonitoringDeleteMaintenanceWindowRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use MonitoringDeleteMaintenanceWindowRequest() / MonitoringDeleteMaintenanceWindowRequest.new instead')
  static MonitoringDeleteMaintenanceWindowRequest create() =>
      MonitoringDeleteMaintenanceWindowRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      MonitoringDeleteMaintenanceWindowRequest._();
  @$core.override
  MonitoringDeleteMaintenanceWindowRequest createEmptyInstance() =>
      MonitoringDeleteMaintenanceWindowRequest._();
  @$core.pragma('dart2js:noInline')
  static MonitoringDeleteMaintenanceWindowRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
              MonitoringDeleteMaintenanceWindowRequest>(
          MonitoringDeleteMaintenanceWindowRequest.$_createMessage);
  static MonitoringDeleteMaintenanceWindowRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get organizationId => $_getI64(1);
  @$pb.TagNumber(2)
  set organizationId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOrganizationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOrganizationId() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

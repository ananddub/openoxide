// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/incoming_webhook.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'webhook.pbenum.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class IncomingWebhookRequest extends $pb.GeneratedMessage {
  factory IncomingWebhookRequest({
    $1.WebhookProviderType? provider,
    $fixnum.Int64? providerId,
    $core.String? payload,
    $core.String? signature,
  }) {
    final result = IncomingWebhookRequest._();
    if (provider != null) result.provider = provider;
    if (providerId != null) result.providerId = providerId;
    if (payload != null) result.payload = payload;
    if (signature != null) result.signature = signature;
    return result;
  }

  IncomingWebhookRequest._();

  factory IncomingWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookRequest()..mergeFromBuffer(data, registry);
  factory IncomingWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IncomingWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: IncomingWebhookRequest.$_createMessage)
    ..aE<$1.WebhookProviderType>(1, _omitFieldNames ? '' : 'provider',
        enumValues: $1.WebhookProviderType.values)
    ..aInt64(2, _omitFieldNames ? '' : 'providerId')
    ..aOS(3, _omitFieldNames ? '' : 'payload')
    ..aOS(4, _omitFieldNames ? '' : 'signature')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookRequest copyWith(
          void Function(IncomingWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as IncomingWebhookRequest))
          as IncomingWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use IncomingWebhookRequest() / IncomingWebhookRequest.new instead')
  static IncomingWebhookRequest create() => IncomingWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => IncomingWebhookRequest._();
  @$core.override
  IncomingWebhookRequest createEmptyInstance() => IncomingWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static IncomingWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<IncomingWebhookRequest>(
          IncomingWebhookRequest.$_createMessage);
  static IncomingWebhookRequest? _defaultInstance;

  /// Git provider the event was posted by (Rust GitProviderType).
  @$pb.TagNumber(1)
  $1.WebhookProviderType get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider($1.WebhookProviderType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);

  /// Row id of the git provider instance whose stored secret verifies this event.
  @$pb.TagNumber(2)
  $fixnum.Int64 get providerId => $_getI64(1);
  @$pb.TagNumber(2)
  set providerId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProviderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearProviderId() => $_clearField(2);

  /// Raw provider event body, forwarded verbatim for signature verification and parsing.
  @$pb.TagNumber(3)
  $core.String get payload => $_getSZ(2);
  @$pb.TagNumber(3)
  set payload($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPayload() => $_has(2);
  @$pb.TagNumber(3)
  void clearPayload() => $_clearField(3);

  /// Provider signature header (HMAC digest or token); opaque to the panel.
  @$pb.TagNumber(4)
  $core.String get signature => $_getSZ(3);
  @$pb.TagNumber(4)
  set signature($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSignature() => $_has(3);
  @$pb.TagNumber(4)
  void clearSignature() => $_clearField(4);
}

/// Result of the auto-deploy dispatch triggered by one accepted webhook event.
class IncomingWebhookDispatchOutcome extends $pb.GeneratedMessage {
  factory IncomingWebhookDispatchOutcome({
    $core.String? eventType,
    $core.String? provider,
    $core.String? owner,
    $core.String? repository,
    $fixnum.Int64? applicationsQueued,
    $fixnum.Int64? composeProjectsQueued,
    $fixnum.Int64? skippedByWatchPaths,
    $fixnum.Int64? alreadyRunning,
    $core.String? pullRequestNumber,
    $core.String? pullRequestAction,
    $core.String? sourceBranch,
    $core.String? sourceOwner,
    $core.String? sourceRepository,
    $core.String? targetBranch,
    $core.String? commit,
    $core.String? author,
    $fixnum.Int64? previewsCreated,
    $fixnum.Int64? previewsRedeployed,
    $fixnum.Int64? previewsRemoved,
    $fixnum.Int64? previewsSkippedPermission,
    $fixnum.Int64? previewsSkippedLimit,
  }) {
    final result = IncomingWebhookDispatchOutcome._();
    if (eventType != null) result.eventType = eventType;
    if (provider != null) result.provider = provider;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (applicationsQueued != null)
      result.applicationsQueued = applicationsQueued;
    if (composeProjectsQueued != null)
      result.composeProjectsQueued = composeProjectsQueued;
    if (skippedByWatchPaths != null)
      result.skippedByWatchPaths = skippedByWatchPaths;
    if (alreadyRunning != null) result.alreadyRunning = alreadyRunning;
    if (pullRequestNumber != null) result.pullRequestNumber = pullRequestNumber;
    if (pullRequestAction != null) result.pullRequestAction = pullRequestAction;
    if (sourceBranch != null) result.sourceBranch = sourceBranch;
    if (sourceOwner != null) result.sourceOwner = sourceOwner;
    if (sourceRepository != null) result.sourceRepository = sourceRepository;
    if (targetBranch != null) result.targetBranch = targetBranch;
    if (commit != null) result.commit = commit;
    if (author != null) result.author = author;
    if (previewsCreated != null) result.previewsCreated = previewsCreated;
    if (previewsRedeployed != null)
      result.previewsRedeployed = previewsRedeployed;
    if (previewsRemoved != null) result.previewsRemoved = previewsRemoved;
    if (previewsSkippedPermission != null)
      result.previewsSkippedPermission = previewsSkippedPermission;
    if (previewsSkippedLimit != null)
      result.previewsSkippedLimit = previewsSkippedLimit;
    return result;
  }

  IncomingWebhookDispatchOutcome._();

  factory IncomingWebhookDispatchOutcome.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookDispatchOutcome()..mergeFromBuffer(data, registry);
  factory IncomingWebhookDispatchOutcome.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookDispatchOutcome()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IncomingWebhookDispatchOutcome',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: IncomingWebhookDispatchOutcome.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'eventType')
    ..aOS(2, _omitFieldNames ? '' : 'provider')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'repository')
    ..aInt64(5, _omitFieldNames ? '' : 'applicationsQueued')
    ..aInt64(6, _omitFieldNames ? '' : 'composeProjectsQueued')
    ..aInt64(7, _omitFieldNames ? '' : 'skippedByWatchPaths')
    ..aInt64(8, _omitFieldNames ? '' : 'alreadyRunning')
    ..aOS(9, _omitFieldNames ? '' : 'pullRequestNumber')
    ..aOS(10, _omitFieldNames ? '' : 'pullRequestAction')
    ..aOS(11, _omitFieldNames ? '' : 'sourceBranch')
    ..aOS(12, _omitFieldNames ? '' : 'sourceOwner')
    ..aOS(13, _omitFieldNames ? '' : 'sourceRepository')
    ..aOS(14, _omitFieldNames ? '' : 'targetBranch')
    ..aOS(15, _omitFieldNames ? '' : 'commit')
    ..aOS(16, _omitFieldNames ? '' : 'author')
    ..aInt64(17, _omitFieldNames ? '' : 'previewsCreated')
    ..aInt64(18, _omitFieldNames ? '' : 'previewsRedeployed')
    ..aInt64(19, _omitFieldNames ? '' : 'previewsRemoved')
    ..aInt64(20, _omitFieldNames ? '' : 'previewsSkippedPermission')
    ..aInt64(21, _omitFieldNames ? '' : 'previewsSkippedLimit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookDispatchOutcome clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookDispatchOutcome copyWith(
          void Function(IncomingWebhookDispatchOutcome) updates) =>
      super.copyWith(
              (message) => updates(message as IncomingWebhookDispatchOutcome))
          as IncomingWebhookDispatchOutcome;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use IncomingWebhookDispatchOutcome() / IncomingWebhookDispatchOutcome.new instead')
  static IncomingWebhookDispatchOutcome create() =>
      IncomingWebhookDispatchOutcome._();
  static $pb.GeneratedMessage $_createMessage() =>
      IncomingWebhookDispatchOutcome._();
  @$core.override
  IncomingWebhookDispatchOutcome createEmptyInstance() =>
      IncomingWebhookDispatchOutcome._();
  @$core.pragma('dart2js:noInline')
  static IncomingWebhookDispatchOutcome getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<IncomingWebhookDispatchOutcome>(
          IncomingWebhookDispatchOutcome.$_createMessage);
  static IncomingWebhookDispatchOutcome? _defaultInstance;

  /// Normalized event class; provider-dependent spelling ("push", "tag", "ping", ...).
  @$pb.TagNumber(1)
  $core.String get eventType => $_getSZ(0);
  @$pb.TagNumber(1)
  set eventType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get provider => $_getSZ(1);
  @$pb.TagNumber(2)
  set provider($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProvider() => $_has(1);
  @$pb.TagNumber(2)
  void clearProvider() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get owner => $_getSZ(2);
  @$pb.TagNumber(3)
  set owner($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwner() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwner() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get repository => $_getSZ(3);
  @$pb.TagNumber(4)
  set repository($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRepository() => $_has(3);
  @$pb.TagNumber(4)
  void clearRepository() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get applicationsQueued => $_getI64(4);
  @$pb.TagNumber(5)
  set applicationsQueued($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasApplicationsQueued() => $_has(4);
  @$pb.TagNumber(5)
  void clearApplicationsQueued() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get composeProjectsQueued => $_getI64(5);
  @$pb.TagNumber(6)
  set composeProjectsQueued($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasComposeProjectsQueued() => $_has(5);
  @$pb.TagNumber(6)
  void clearComposeProjectsQueued() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get skippedByWatchPaths => $_getI64(6);
  @$pb.TagNumber(7)
  set skippedByWatchPaths($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSkippedByWatchPaths() => $_has(6);
  @$pb.TagNumber(7)
  void clearSkippedByWatchPaths() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get alreadyRunning => $_getI64(7);
  @$pb.TagNumber(8)
  set alreadyRunning($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasAlreadyRunning() => $_has(7);
  @$pb.TagNumber(8)
  void clearAlreadyRunning() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get pullRequestNumber => $_getSZ(8);
  @$pb.TagNumber(9)
  set pullRequestNumber($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPullRequestNumber() => $_has(8);
  @$pb.TagNumber(9)
  void clearPullRequestNumber() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get pullRequestAction => $_getSZ(9);
  @$pb.TagNumber(10)
  set pullRequestAction($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasPullRequestAction() => $_has(9);
  @$pb.TagNumber(10)
  void clearPullRequestAction() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get sourceBranch => $_getSZ(10);
  @$pb.TagNumber(11)
  set sourceBranch($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasSourceBranch() => $_has(10);
  @$pb.TagNumber(11)
  void clearSourceBranch() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get sourceOwner => $_getSZ(11);
  @$pb.TagNumber(12)
  set sourceOwner($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasSourceOwner() => $_has(11);
  @$pb.TagNumber(12)
  void clearSourceOwner() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get sourceRepository => $_getSZ(12);
  @$pb.TagNumber(13)
  set sourceRepository($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasSourceRepository() => $_has(12);
  @$pb.TagNumber(13)
  void clearSourceRepository() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get targetBranch => $_getSZ(13);
  @$pb.TagNumber(14)
  set targetBranch($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasTargetBranch() => $_has(13);
  @$pb.TagNumber(14)
  void clearTargetBranch() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get commit => $_getSZ(14);
  @$pb.TagNumber(15)
  set commit($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasCommit() => $_has(14);
  @$pb.TagNumber(15)
  void clearCommit() => $_clearField(15);

  @$pb.TagNumber(16)
  $core.String get author => $_getSZ(15);
  @$pb.TagNumber(16)
  set author($core.String value) => $_setString(15, value);
  @$pb.TagNumber(16)
  $core.bool hasAuthor() => $_has(15);
  @$pb.TagNumber(16)
  void clearAuthor() => $_clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get previewsCreated => $_getI64(16);
  @$pb.TagNumber(17)
  set previewsCreated($fixnum.Int64 value) => $_setInt64(16, value);
  @$pb.TagNumber(17)
  $core.bool hasPreviewsCreated() => $_has(16);
  @$pb.TagNumber(17)
  void clearPreviewsCreated() => $_clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get previewsRedeployed => $_getI64(17);
  @$pb.TagNumber(18)
  set previewsRedeployed($fixnum.Int64 value) => $_setInt64(17, value);
  @$pb.TagNumber(18)
  $core.bool hasPreviewsRedeployed() => $_has(17);
  @$pb.TagNumber(18)
  void clearPreviewsRedeployed() => $_clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get previewsRemoved => $_getI64(18);
  @$pb.TagNumber(19)
  set previewsRemoved($fixnum.Int64 value) => $_setInt64(18, value);
  @$pb.TagNumber(19)
  $core.bool hasPreviewsRemoved() => $_has(18);
  @$pb.TagNumber(19)
  void clearPreviewsRemoved() => $_clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get previewsSkippedPermission => $_getI64(19);
  @$pb.TagNumber(20)
  set previewsSkippedPermission($fixnum.Int64 value) => $_setInt64(19, value);
  @$pb.TagNumber(20)
  $core.bool hasPreviewsSkippedPermission() => $_has(19);
  @$pb.TagNumber(20)
  void clearPreviewsSkippedPermission() => $_clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get previewsSkippedLimit => $_getI64(20);
  @$pb.TagNumber(21)
  set previewsSkippedLimit($fixnum.Int64 value) => $_setInt64(20, value);
  @$pb.TagNumber(21)
  $core.bool hasPreviewsSkippedLimit() => $_has(20);
  @$pb.TagNumber(21)
  void clearPreviewsSkippedLimit() => $_clearField(21);
}

class IncomingWebhookResponse extends $pb.GeneratedMessage {
  factory IncomingWebhookResponse({
    $core.bool? success,
    IncomingWebhookDispatchOutcome? outcome,
  }) {
    final result = IncomingWebhookResponse._();
    if (success != null) result.success = success;
    if (outcome != null) result.outcome = outcome;
    return result;
  }

  IncomingWebhookResponse._();

  factory IncomingWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookResponse()..mergeFromBuffer(data, registry);
  factory IncomingWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IncomingWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IncomingWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: IncomingWebhookResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOM<IncomingWebhookDispatchOutcome>(2, _omitFieldNames ? '' : 'outcome',
        subBuilder: IncomingWebhookDispatchOutcome.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IncomingWebhookResponse copyWith(
          void Function(IncomingWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as IncomingWebhookResponse))
          as IncomingWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use IncomingWebhookResponse() / IncomingWebhookResponse.new instead')
  static IncomingWebhookResponse create() => IncomingWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() => IncomingWebhookResponse._();
  @$core.override
  IncomingWebhookResponse createEmptyInstance() => IncomingWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static IncomingWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<IncomingWebhookResponse>(
          IncomingWebhookResponse.$_createMessage);
  static IncomingWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  IncomingWebhookDispatchOutcome get outcome => $_getN(1);
  @$pb.TagNumber(2)
  set outcome(IncomingWebhookDispatchOutcome value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOutcome() => $_has(1);
  @$pb.TagNumber(2)
  void clearOutcome() => $_clearField(2);
  @$pb.TagNumber(2)
  IncomingWebhookDispatchOutcome ensureOutcome() => $_ensure(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

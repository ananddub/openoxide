// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/webhook.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'webhook.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'webhook.pbenum.dart';

class Webhook extends $pb.GeneratedMessage {
  factory Webhook({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    WebhookProviderType? provider,
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = Webhook._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (provider != null) result.provider = provider;
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  Webhook._();

  factory Webhook.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Webhook()..mergeFromBuffer(data, registry);
  factory Webhook.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Webhook()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Webhook',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: Webhook.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aE<WebhookProviderType>(3, _omitFieldNames ? '' : 'provider',
        enumValues: WebhookProviderType.values)
    ..aInt64(4, _omitFieldNames ? '' : 'providerId')
    ..aOS(5, _omitFieldNames ? '' : 'owner')
    ..aOS(6, _omitFieldNames ? '' : 'repository')
    ..aOS(7, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Webhook clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Webhook copyWith(void Function(Webhook) updates) =>
      super.copyWith((message) => updates(message as Webhook)) as Webhook;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Webhook() / Webhook.new instead')
  static Webhook create() => Webhook._();
  static $pb.GeneratedMessage $_createMessage() => Webhook._();
  @$core.override
  Webhook createEmptyInstance() => Webhook._();
  @$core.pragma('dart2js:noInline')
  static Webhook getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Webhook>(Webhook.$_createMessage);
  static Webhook? _defaultInstance;

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
  WebhookProviderType get provider => $_getN(2);
  @$pb.TagNumber(3)
  set provider(WebhookProviderType value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasProvider() => $_has(2);
  @$pb.TagNumber(3)
  void clearProvider() => $_clearField(3);

  /// Row id of the git provider instance the webhook is installed through.
  @$pb.TagNumber(4)
  $fixnum.Int64 get providerId => $_getI64(3);
  @$pb.TagNumber(4)
  set providerId($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasProviderId() => $_has(3);
  @$pb.TagNumber(4)
  void clearProviderId() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get owner => $_getSZ(4);
  @$pb.TagNumber(5)
  set owner($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOwner() => $_has(4);
  @$pb.TagNumber(5)
  void clearOwner() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get repository => $_getSZ(5);
  @$pb.TagNumber(6)
  set repository($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRepository() => $_has(5);
  @$pb.TagNumber(6)
  void clearRepository() => $_clearField(6);

  /// Panel callback URL the provider posts events to.
  @$pb.TagNumber(7)
  $core.String get callbackUrl => $_getSZ(6);
  @$pb.TagNumber(7)
  set callbackUrl($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCallbackUrl() => $_has(6);
  @$pb.TagNumber(7)
  void clearCallbackUrl() => $_clearField(7);
}

class ListWebhooksRequest extends $pb.GeneratedMessage {
  factory ListWebhooksRequest({
    $fixnum.Int64? organizationId,
  }) {
    final result = ListWebhooksRequest._();
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  ListWebhooksRequest._();

  factory ListWebhooksRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListWebhooksRequest()..mergeFromBuffer(data, registry);
  factory ListWebhooksRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListWebhooksRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListWebhooksRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListWebhooksRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWebhooksRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWebhooksRequest copyWith(void Function(ListWebhooksRequest) updates) =>
      super.copyWith((message) => updates(message as ListWebhooksRequest))
          as ListWebhooksRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListWebhooksRequest() / ListWebhooksRequest.new instead')
  static ListWebhooksRequest create() => ListWebhooksRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListWebhooksRequest._();
  @$core.override
  ListWebhooksRequest createEmptyInstance() => ListWebhooksRequest._();
  @$core.pragma('dart2js:noInline')
  static ListWebhooksRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListWebhooksRequest>(
          ListWebhooksRequest.$_createMessage);
  static ListWebhooksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get organizationId => $_getI64(0);
  @$pb.TagNumber(1)
  set organizationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOrganizationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrganizationId() => $_clearField(1);
}

class ListWebhooksResponse extends $pb.GeneratedMessage {
  factory ListWebhooksResponse({
    $core.Iterable<Webhook>? webhooks,
  }) {
    final result = ListWebhooksResponse._();
    if (webhooks != null) result.webhooks.addAll(webhooks);
    return result;
  }

  ListWebhooksResponse._();

  factory ListWebhooksResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListWebhooksResponse()..mergeFromBuffer(data, registry);
  factory ListWebhooksResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListWebhooksResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListWebhooksResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListWebhooksResponse.$_createMessage)
    ..pPM<Webhook>(1, _omitFieldNames ? '' : 'webhooks',
        subBuilder: Webhook.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWebhooksResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWebhooksResponse copyWith(void Function(ListWebhooksResponse) updates) =>
      super.copyWith((message) => updates(message as ListWebhooksResponse))
          as ListWebhooksResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListWebhooksResponse() / ListWebhooksResponse.new instead')
  static ListWebhooksResponse create() => ListWebhooksResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListWebhooksResponse._();
  @$core.override
  ListWebhooksResponse createEmptyInstance() => ListWebhooksResponse._();
  @$core.pragma('dart2js:noInline')
  static ListWebhooksResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListWebhooksResponse>(
          ListWebhooksResponse.$_createMessage);
  static ListWebhooksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Webhook> get webhooks => $_getList(0);
}

class GetWebhookRequest extends $pb.GeneratedMessage {
  factory GetWebhookRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = GetWebhookRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  GetWebhookRequest._();

  factory GetWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetWebhookRequest()..mergeFromBuffer(data, registry);
  factory GetWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWebhookRequest copyWith(void Function(GetWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as GetWebhookRequest))
          as GetWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetWebhookRequest() / GetWebhookRequest.new instead')
  static GetWebhookRequest create() => GetWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetWebhookRequest._();
  @$core.override
  GetWebhookRequest createEmptyInstance() => GetWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static GetWebhookRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetWebhookRequest>(
          GetWebhookRequest.$_createMessage);
  static GetWebhookRequest? _defaultInstance;

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

class GetWebhookResponse extends $pb.GeneratedMessage {
  factory GetWebhookResponse({
    Webhook? webhook,
  }) {
    final result = GetWebhookResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  GetWebhookResponse._();

  factory GetWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetWebhookResponse()..mergeFromBuffer(data, registry);
  factory GetWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetWebhookResponse.$_createMessage)
    ..aOM<Webhook>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: Webhook.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWebhookResponse copyWith(void Function(GetWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as GetWebhookResponse))
          as GetWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use GetWebhookResponse() / GetWebhookResponse.new instead')
  static GetWebhookResponse create() => GetWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() => GetWebhookResponse._();
  @$core.override
  GetWebhookResponse createEmptyInstance() => GetWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static GetWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetWebhookResponse>(
          GetWebhookResponse.$_createMessage);
  static GetWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Webhook get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(Webhook value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  Webhook ensureWebhook() => $_ensure(0);
}

class CreateWebhookRequest extends $pb.GeneratedMessage {
  factory CreateWebhookRequest({
    WebhookProviderType? provider,
    $fixnum.Int64? providerId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = CreateWebhookRequest._();
    if (provider != null) result.provider = provider;
    if (providerId != null) result.providerId = providerId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  CreateWebhookRequest._();

  factory CreateWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateWebhookRequest()..mergeFromBuffer(data, registry);
  factory CreateWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateWebhookRequest.$_createMessage)
    ..aE<WebhookProviderType>(1, _omitFieldNames ? '' : 'provider',
        enumValues: WebhookProviderType.values)
    ..aInt64(2, _omitFieldNames ? '' : 'providerId')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'repository')
    ..aOS(5, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWebhookRequest copyWith(void Function(CreateWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as CreateWebhookRequest))
          as CreateWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateWebhookRequest() / CreateWebhookRequest.new instead')
  static CreateWebhookRequest create() => CreateWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => CreateWebhookRequest._();
  @$core.override
  CreateWebhookRequest createEmptyInstance() => CreateWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static CreateWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateWebhookRequest>(
          CreateWebhookRequest.$_createMessage);
  static CreateWebhookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  WebhookProviderType get provider => $_getN(0);
  @$pb.TagNumber(1)
  set provider(WebhookProviderType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProvider() => $_has(0);
  @$pb.TagNumber(1)
  void clearProvider() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get providerId => $_getI64(1);
  @$pb.TagNumber(2)
  set providerId($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProviderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearProviderId() => $_clearField(2);

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
  $core.String get callbackUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set callbackUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCallbackUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearCallbackUrl() => $_clearField(5);
}

class CreateWebhookResponse extends $pb.GeneratedMessage {
  factory CreateWebhookResponse({
    Webhook? webhook,
  }) {
    final result = CreateWebhookResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  CreateWebhookResponse._();

  factory CreateWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateWebhookResponse()..mergeFromBuffer(data, registry);
  factory CreateWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreateWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreateWebhookResponse.$_createMessage)
    ..aOM<Webhook>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: Webhook.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWebhookResponse copyWith(
          void Function(CreateWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as CreateWebhookResponse))
          as CreateWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreateWebhookResponse() / CreateWebhookResponse.new instead')
  static CreateWebhookResponse create() => CreateWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() => CreateWebhookResponse._();
  @$core.override
  CreateWebhookResponse createEmptyInstance() => CreateWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static CreateWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateWebhookResponse>(
          CreateWebhookResponse.$_createMessage);
  static CreateWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Webhook get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(Webhook value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  Webhook ensureWebhook() => $_ensure(0);
}

/// PATCH semantics: absent fields keep their stored values.
class UpdateWebhookRequest extends $pb.GeneratedMessage {
  factory UpdateWebhookRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
    $core.String? owner,
    $core.String? repository,
    $core.String? callbackUrl,
  }) {
    final result = UpdateWebhookRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    if (owner != null) result.owner = owner;
    if (repository != null) result.repository = repository;
    if (callbackUrl != null) result.callbackUrl = callbackUrl;
    return result;
  }

  UpdateWebhookRequest._();

  factory UpdateWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateWebhookRequest()..mergeFromBuffer(data, registry);
  factory UpdateWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..aOS(3, _omitFieldNames ? '' : 'owner')
    ..aOS(4, _omitFieldNames ? '' : 'repository')
    ..aOS(5, _omitFieldNames ? '' : 'callbackUrl')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateWebhookRequest copyWith(void Function(UpdateWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateWebhookRequest))
          as UpdateWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateWebhookRequest() / UpdateWebhookRequest.new instead')
  static UpdateWebhookRequest create() => UpdateWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateWebhookRequest._();
  @$core.override
  UpdateWebhookRequest createEmptyInstance() => UpdateWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateWebhookRequest>(
          UpdateWebhookRequest.$_createMessage);
  static UpdateWebhookRequest? _defaultInstance;

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
  $core.String get callbackUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set callbackUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCallbackUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearCallbackUrl() => $_clearField(5);
}

class UpdateWebhookResponse extends $pb.GeneratedMessage {
  factory UpdateWebhookResponse({
    Webhook? webhook,
  }) {
    final result = UpdateWebhookResponse._();
    if (webhook != null) result.webhook = webhook;
    return result;
  }

  UpdateWebhookResponse._();

  factory UpdateWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateWebhookResponse()..mergeFromBuffer(data, registry);
  factory UpdateWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateWebhookResponse.$_createMessage)
    ..aOM<Webhook>(1, _omitFieldNames ? '' : 'webhook',
        subBuilder: Webhook.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateWebhookResponse copyWith(
          void Function(UpdateWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as UpdateWebhookResponse))
          as UpdateWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use UpdateWebhookResponse() / UpdateWebhookResponse.new instead')
  static UpdateWebhookResponse create() => UpdateWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() => UpdateWebhookResponse._();
  @$core.override
  UpdateWebhookResponse createEmptyInstance() => UpdateWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static UpdateWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UpdateWebhookResponse>(
          UpdateWebhookResponse.$_createMessage);
  static UpdateWebhookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Webhook get webhook => $_getN(0);
  @$pb.TagNumber(1)
  set webhook(Webhook value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasWebhook() => $_has(0);
  @$pb.TagNumber(1)
  void clearWebhook() => $_clearField(1);
  @$pb.TagNumber(1)
  Webhook ensureWebhook() => $_ensure(0);
}

class DeleteWebhookRequest extends $pb.GeneratedMessage {
  factory DeleteWebhookRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = DeleteWebhookRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  DeleteWebhookRequest._();

  factory DeleteWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteWebhookRequest()..mergeFromBuffer(data, registry);
  factory DeleteWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DeleteWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DeleteWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteWebhookRequest copyWith(void Function(DeleteWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteWebhookRequest))
          as DeleteWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DeleteWebhookRequest() / DeleteWebhookRequest.new instead')
  static DeleteWebhookRequest create() => DeleteWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => DeleteWebhookRequest._();
  @$core.override
  DeleteWebhookRequest createEmptyInstance() => DeleteWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static DeleteWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteWebhookRequest>(
          DeleteWebhookRequest.$_createMessage);
  static DeleteWebhookRequest? _defaultInstance;

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

class TestWebhookRequest extends $pb.GeneratedMessage {
  factory TestWebhookRequest({
    $fixnum.Int64? id,
    $fixnum.Int64? organizationId,
  }) {
    final result = TestWebhookRequest._();
    if (id != null) result.id = id;
    if (organizationId != null) result.organizationId = organizationId;
    return result;
  }

  TestWebhookRequest._();

  factory TestWebhookRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestWebhookRequest()..mergeFromBuffer(data, registry);
  factory TestWebhookRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestWebhookRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestWebhookRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestWebhookRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aInt64(2, _omitFieldNames ? '' : 'organizationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestWebhookRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestWebhookRequest copyWith(void Function(TestWebhookRequest) updates) =>
      super.copyWith((message) => updates(message as TestWebhookRequest))
          as TestWebhookRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TestWebhookRequest() / TestWebhookRequest.new instead')
  static TestWebhookRequest create() => TestWebhookRequest._();
  static $pb.GeneratedMessage $_createMessage() => TestWebhookRequest._();
  @$core.override
  TestWebhookRequest createEmptyInstance() => TestWebhookRequest._();
  @$core.pragma('dart2js:noInline')
  static TestWebhookRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestWebhookRequest>(
          TestWebhookRequest.$_createMessage);
  static TestWebhookRequest? _defaultInstance;

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

class TestWebhookResponse extends $pb.GeneratedMessage {
  factory TestWebhookResponse({
    $core.bool? success,
    $core.String? message,
  }) {
    final result = TestWebhookResponse._();
    if (success != null) result.success = success;
    if (message != null) result.message = message;
    return result;
  }

  TestWebhookResponse._();

  factory TestWebhookResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestWebhookResponse()..mergeFromBuffer(data, registry);
  factory TestWebhookResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TestWebhookResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TestWebhookResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TestWebhookResponse.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestWebhookResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TestWebhookResponse copyWith(void Function(TestWebhookResponse) updates) =>
      super.copyWith((message) => updates(message as TestWebhookResponse))
          as TestWebhookResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use TestWebhookResponse() / TestWebhookResponse.new instead')
  static TestWebhookResponse create() => TestWebhookResponse._();
  static $pb.GeneratedMessage $_createMessage() => TestWebhookResponse._();
  @$core.override
  TestWebhookResponse createEmptyInstance() => TestWebhookResponse._();
  @$core.pragma('dart2js:noInline')
  static TestWebhookResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TestWebhookResponse>(
          TestWebhookResponse.$_createMessage);
  static TestWebhookResponse? _defaultInstance;

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

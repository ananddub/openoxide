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

import 'package:protobuf/protobuf.dart' as $pb;

/// Closed set shared with IncomingWebhookRequest.provider (Rust GitProviderType).
class WebhookProviderType extends $pb.ProtobufEnum {
  static const WebhookProviderType WEBHOOK_PROVIDER_TYPE_UNSPECIFIED =
      WebhookProviderType._(
          0, _omitEnumNames ? '' : 'WEBHOOK_PROVIDER_TYPE_UNSPECIFIED');
  static const WebhookProviderType WEBHOOK_PROVIDER_TYPE_GITHUB =
      WebhookProviderType._(
          1, _omitEnumNames ? '' : 'WEBHOOK_PROVIDER_TYPE_GITHUB');
  static const WebhookProviderType WEBHOOK_PROVIDER_TYPE_GITLAB =
      WebhookProviderType._(
          2, _omitEnumNames ? '' : 'WEBHOOK_PROVIDER_TYPE_GITLAB');
  static const WebhookProviderType WEBHOOK_PROVIDER_TYPE_GITEA =
      WebhookProviderType._(
          3, _omitEnumNames ? '' : 'WEBHOOK_PROVIDER_TYPE_GITEA');
  static const WebhookProviderType WEBHOOK_PROVIDER_TYPE_BITBUCKET =
      WebhookProviderType._(
          4, _omitEnumNames ? '' : 'WEBHOOK_PROVIDER_TYPE_BITBUCKET');

  static const $core.List<WebhookProviderType> values = <WebhookProviderType>[
    WEBHOOK_PROVIDER_TYPE_UNSPECIFIED,
    WEBHOOK_PROVIDER_TYPE_GITHUB,
    WEBHOOK_PROVIDER_TYPE_GITLAB,
    WEBHOOK_PROVIDER_TYPE_GITEA,
    WEBHOOK_PROVIDER_TYPE_BITBUCKET,
  ];

  static final $core.List<WebhookProviderType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static WebhookProviderType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const WebhookProviderType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

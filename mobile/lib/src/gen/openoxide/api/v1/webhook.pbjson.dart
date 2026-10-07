// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/webhook.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use webhookProviderTypeDescriptor instead')
const WebhookProviderType$json = {
  '1': 'WebhookProviderType',
  '2': [
    {'1': 'WEBHOOK_PROVIDER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'WEBHOOK_PROVIDER_TYPE_GITHUB', '2': 1},
    {'1': 'WEBHOOK_PROVIDER_TYPE_GITLAB', '2': 2},
    {'1': 'WEBHOOK_PROVIDER_TYPE_GITEA', '2': 3},
    {'1': 'WEBHOOK_PROVIDER_TYPE_BITBUCKET', '2': 4},
  ],
};

/// Descriptor for `WebhookProviderType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List webhookProviderTypeDescriptor = $convert.base64Decode(
    'ChNXZWJob29rUHJvdmlkZXJUeXBlEiUKIVdFQkhPT0tfUFJPVklERVJfVFlQRV9VTlNQRUNJRk'
    'lFRBAAEiAKHFdFQkhPT0tfUFJPVklERVJfVFlQRV9HSVRIVUIQARIgChxXRUJIT09LX1BST1ZJ'
    'REVSX1RZUEVfR0lUTEFCEAISHwobV0VCSE9PS19QUk9WSURFUl9UWVBFX0dJVEVBEAMSIwofV0'
    'VCSE9PS19QUk9WSURFUl9UWVBFX0JJVEJVQ0tFVBAE');

@$core.Deprecated('Use webhookDescriptor instead')
const Webhook$json = {
  '1': 'Webhook',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'organization_id', '3': 2, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'provider',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.WebhookProviderType',
      '10': 'provider'
    },
    {'1': 'provider_id', '3': 4, '4': 1, '5': 3, '10': 'providerId'},
    {'1': 'owner', '3': 5, '4': 1, '5': 9, '10': 'owner'},
    {'1': 'repository', '3': 6, '4': 1, '5': 9, '10': 'repository'},
    {'1': 'callback_url', '3': 7, '4': 1, '5': 9, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `Webhook`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List webhookDescriptor = $convert.base64Decode(
    'CgdXZWJob29rEg4KAmlkGAEgASgDUgJpZBInCg9vcmdhbml6YXRpb25faWQYAiABKANSDm9yZ2'
    'FuaXphdGlvbklkEkEKCHByb3ZpZGVyGAMgASgOMiUub3Blbm94aWRlLmFwaS52MS5XZWJob29r'
    'UHJvdmlkZXJUeXBlUghwcm92aWRlchIfCgtwcm92aWRlcl9pZBgEIAEoA1IKcHJvdmlkZXJJZB'
    'IUCgVvd25lchgFIAEoCVIFb3duZXISHgoKcmVwb3NpdG9yeRgGIAEoCVIKcmVwb3NpdG9yeRIh'
    'CgxjYWxsYmFja191cmwYByABKAlSC2NhbGxiYWNrVXJs');

@$core.Deprecated('Use listWebhooksRequestDescriptor instead')
const ListWebhooksRequest$json = {
  '1': 'ListWebhooksRequest',
  '2': [
    {
      '1': 'organization_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `ListWebhooksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listWebhooksRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0V2ViaG9va3NSZXF1ZXN0EjUKD29yZ2FuaXphdGlvbl9pZBgBIAEoA0IHukgEIgIgAE'
    'gAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use listWebhooksResponseDescriptor instead')
const ListWebhooksResponse$json = {
  '1': 'ListWebhooksResponse',
  '2': [
    {
      '1': 'webhooks',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Webhook',
      '10': 'webhooks'
    },
  ],
};

/// Descriptor for `ListWebhooksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listWebhooksResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0V2ViaG9va3NSZXNwb25zZRI1Cgh3ZWJob29rcxgBIAMoCzIZLm9wZW5veGlkZS5hcG'
    'kudjEuV2ViaG9va1IId2ViaG9va3M=');

@$core.Deprecated('Use getWebhookRequestDescriptor instead')
const GetWebhookRequest$json = {
  '1': 'GetWebhookRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `GetWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getWebhookRequestDescriptor = $convert.base64Decode(
    'ChFHZXRXZWJob29rUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3JnYW5pem'
    'F0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdhbml6'
    'YXRpb25faWQ=');

@$core.Deprecated('Use getWebhookResponseDescriptor instead')
const GetWebhookResponse$json = {
  '1': 'GetWebhookResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Webhook',
      '10': 'webhook'
    },
  ],
};

/// Descriptor for `GetWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getWebhookResponseDescriptor = $convert.base64Decode(
    'ChJHZXRXZWJob29rUmVzcG9uc2USMwoHd2ViaG9vaxgBIAEoCzIZLm9wZW5veGlkZS5hcGkudj'
    'EuV2ViaG9va1IHd2ViaG9vaw==');

@$core.Deprecated('Use createWebhookRequestDescriptor instead')
const CreateWebhookRequest$json = {
  '1': 'CreateWebhookRequest',
  '2': [
    {
      '1': 'provider',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.WebhookProviderType',
      '8': {},
      '10': 'provider'
    },
    {'1': 'provider_id', '3': 2, '4': 1, '5': 3, '8': {}, '10': 'providerId'},
    {'1': 'owner', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'owner'},
    {'1': 'repository', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'repository'},
    {'1': 'callback_url', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'callbackUrl'},
  ],
};

/// Descriptor for `CreateWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createWebhookRequestDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVXZWJob29rUmVxdWVzdBJNCghwcm92aWRlchgBIAEoDjIlLm9wZW5veGlkZS5hcG'
    'kudjEuV2ViaG9va1Byb3ZpZGVyVHlwZUIKukgHggEEEAEgAFIIcHJvdmlkZXISKAoLcHJvdmlk'
    'ZXJfaWQYAiABKANCB7pIBCICIABSCnByb3ZpZGVySWQSIAoFb3duZXIYAyABKAlCCrpIB3IFEA'
    'EY/wFSBW93bmVyEioKCnJlcG9zaXRvcnkYBCABKAlCCrpIB3IFEAEY/wFSCnJlcG9zaXRvcnkS'
    'KgoMY2FsbGJhY2tfdXJsGAUgASgJQge6SARyAhABUgtjYWxsYmFja1VybA==');

@$core.Deprecated('Use createWebhookResponseDescriptor instead')
const CreateWebhookResponse$json = {
  '1': 'CreateWebhookResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Webhook',
      '10': 'webhook'
    },
  ],
};

/// Descriptor for `CreateWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createWebhookResponseDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVXZWJob29rUmVzcG9uc2USMwoHd2ViaG9vaxgBIAEoCzIZLm9wZW5veGlkZS5hcG'
    'kudjEuV2ViaG9va1IHd2ViaG9vaw==');

@$core.Deprecated('Use updateWebhookRequestDescriptor instead')
const UpdateWebhookRequest$json = {
  '1': 'UpdateWebhookRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
    {
      '1': 'owner',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'owner',
      '17': true
    },
    {
      '1': 'repository',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'repository',
      '17': true
    },
    {
      '1': 'callback_url',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'callbackUrl',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_owner'},
    {'1': '_repository'},
    {'1': '_callback_url'},
  ],
};

/// Descriptor for `UpdateWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateWebhookRequestDescriptor = $convert.base64Decode(
    'ChRVcGRhdGVXZWJob29rUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3JnYW'
    '5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBEiMKBW93bmVy'
    'GAMgASgJQgi6SAVyAxj/AUgBUgVvd25lcogBARItCgpyZXBvc2l0b3J5GAQgASgJQgi6SAVyAx'
    'j/AUgCUgpyZXBvc2l0b3J5iAEBEiYKDGNhbGxiYWNrX3VybBgFIAEoCUgDUgtjYWxsYmFja1Vy'
    'bIgBAUISChBfb3JnYW5pemF0aW9uX2lkQggKBl9vd25lckINCgtfcmVwb3NpdG9yeUIPCg1fY2'
    'FsbGJhY2tfdXJs');

@$core.Deprecated('Use updateWebhookResponseDescriptor instead')
const UpdateWebhookResponse$json = {
  '1': 'UpdateWebhookResponse',
  '2': [
    {
      '1': 'webhook',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Webhook',
      '10': 'webhook'
    },
  ],
};

/// Descriptor for `UpdateWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateWebhookResponseDescriptor = $convert.base64Decode(
    'ChVVcGRhdGVXZWJob29rUmVzcG9uc2USMwoHd2ViaG9vaxgBIAEoCzIZLm9wZW5veGlkZS5hcG'
    'kudjEuV2ViaG9va1IHd2ViaG9vaw==');

@$core.Deprecated('Use deleteWebhookRequestDescriptor instead')
const DeleteWebhookRequest$json = {
  '1': 'DeleteWebhookRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `DeleteWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteWebhookRequestDescriptor = $convert.base64Decode(
    'ChREZWxldGVXZWJob29rUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3JnYW'
    '5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdh'
    'bml6YXRpb25faWQ=');

@$core.Deprecated('Use testWebhookRequestDescriptor instead')
const TestWebhookRequest$json = {
  '1': 'TestWebhookRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `TestWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testWebhookRequestDescriptor = $convert.base64Decode(
    'ChJUZXN0V2ViaG9va1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjUKD29yZ2FuaX'
    'phdGlvbl9pZBgCIAEoA0IHukgEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3JnYW5p'
    'emF0aW9uX2lk');

@$core.Deprecated('Use testWebhookResponseDescriptor instead')
const TestWebhookResponse$json = {
  '1': 'TestWebhookResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testWebhookResponseDescriptor = $convert.base64Decode(
    'ChNUZXN0V2ViaG9va1Jlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSGAoHbWVzc2'
    'FnZRgCIAEoCVIHbWVzc2FnZQ==');

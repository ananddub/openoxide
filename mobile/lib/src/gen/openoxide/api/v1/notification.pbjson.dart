// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/notification.proto.

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

@$core.Deprecated('Use notificationProviderTypeDescriptor instead')
const NotificationProviderType$json = {
  '1': 'NotificationProviderType',
  '2': [
    {'1': 'NOTIFICATION_PROVIDER_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_SLACK', '2': 1},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_TELEGRAM', '2': 2},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_DISCORD', '2': 3},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_EMAIL', '2': 4},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_RESEND', '2': 5},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_GOTIFY', '2': 6},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_NTFY', '2': 7},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_MATTERMOST', '2': 8},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_PUSHOVER', '2': 9},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_CUSTOM', '2': 10},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_LARK', '2': 11},
    {'1': 'NOTIFICATION_PROVIDER_TYPE_TEAMS', '2': 12},
  ],
};

/// Descriptor for `NotificationProviderType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List notificationProviderTypeDescriptor = $convert.base64Decode(
    'ChhOb3RpZmljYXRpb25Qcm92aWRlclR5cGUSKgomTk9USUZJQ0FUSU9OX1BST1ZJREVSX1RZUE'
    'VfVU5TUEVDSUZJRUQQABIkCiBOT1RJRklDQVRJT05fUFJPVklERVJfVFlQRV9TTEFDSxABEicK'
    'I05PVElGSUNBVElPTl9QUk9WSURFUl9UWVBFX1RFTEVHUkFNEAISJgoiTk9USUZJQ0FUSU9OX1'
    'BST1ZJREVSX1RZUEVfRElTQ09SRBADEiQKIE5PVElGSUNBVElPTl9QUk9WSURFUl9UWVBFX0VN'
    'QUlMEAQSJQohTk9USUZJQ0FUSU9OX1BST1ZJREVSX1RZUEVfUkVTRU5EEAUSJQohTk9USUZJQ0'
    'FUSU9OX1BST1ZJREVSX1RZUEVfR09USUZZEAYSIwofTk9USUZJQ0FUSU9OX1BST1ZJREVSX1RZ'
    'UEVfTlRGWRAHEikKJU5PVElGSUNBVElPTl9QUk9WSURFUl9UWVBFX01BVFRFUk1PU1QQCBInCi'
    'NOT1RJRklDQVRJT05fUFJPVklERVJfVFlQRV9QVVNIT1ZFUhAJEiUKIU5PVElGSUNBVElPTl9Q'
    'Uk9WSURFUl9UWVBFX0NVU1RPTRAKEiMKH05PVElGSUNBVElPTl9QUk9WSURFUl9UWVBFX0xBUk'
    'sQCxIkCiBOT1RJRklDQVRJT05fUFJPVklERVJfVFlQRV9URUFNUxAM');

@$core.Deprecated('Use notificationResourceTypeDescriptor instead')
const NotificationResourceType$json = {
  '1': 'NotificationResourceType',
  '2': [
    {'1': 'NOTIFICATION_RESOURCE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'NOTIFICATION_RESOURCE_TYPE_SERVER', '2': 1},
    {'1': 'NOTIFICATION_RESOURCE_TYPE_APPLICATION', '2': 2},
    {'1': 'NOTIFICATION_RESOURCE_TYPE_COMPOSE', '2': 3},
    {'1': 'NOTIFICATION_RESOURCE_TYPE_DATABASE', '2': 4},
  ],
};

/// Descriptor for `NotificationResourceType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List notificationResourceTypeDescriptor = $convert.base64Decode(
    'ChhOb3RpZmljYXRpb25SZXNvdXJjZVR5cGUSKgomTk9USUZJQ0FUSU9OX1JFU09VUkNFX1RZUE'
    'VfVU5TUEVDSUZJRUQQABIlCiFOT1RJRklDQVRJT05fUkVTT1VSQ0VfVFlQRV9TRVJWRVIQARIq'
    'CiZOT1RJRklDQVRJT05fUkVTT1VSQ0VfVFlQRV9BUFBMSUNBVElPThACEiYKIk5PVElGSUNBVE'
    'lPTl9SRVNPVVJDRV9UWVBFX0NPTVBPU0UQAxInCiNOT1RJRklDQVRJT05fUkVTT1VSQ0VfVFlQ'
    'RV9EQVRBQkFTRRAE');

@$core.Deprecated('Use notificationDescriptor instead')
const Notification$json = {
  '1': 'Notification',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'notification_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.NotificationProviderType',
      '10': 'notificationType'
    },
    {'1': 'on_app_deploy', '3': 4, '4': 1, '5': 8, '10': 'onAppDeploy'},
    {
      '1': 'on_app_build_error',
      '3': 5,
      '4': 1,
      '5': 8,
      '10': 'onAppBuildError'
    },
    {
      '1': 'on_database_backup',
      '3': 6,
      '4': 1,
      '5': 8,
      '10': 'onDatabaseBackup'
    },
    {'1': 'on_volume_backup', '3': 7, '4': 1, '5': 8, '10': 'onVolumeBackup'},
    {'1': 'on_panel_restart', '3': 8, '4': 1, '5': 8, '10': 'onPanelRestart'},
    {'1': 'on_docker_cleanup', '3': 9, '4': 1, '5': 8, '10': 'onDockerCleanup'},
    {
      '1': 'on_server_threshold',
      '3': 10,
      '4': 1,
      '5': 8,
      '10': 'onServerThreshold'
    },
    {'1': 'on_panel_backup', '3': 11, '4': 1, '5': 8, '10': 'onPanelBackup'},
    {
      '1': 'on_schedule_success',
      '3': 12,
      '4': 1,
      '5': 8,
      '10': 'onScheduleSuccess'
    },
    {
      '1': 'on_schedule_failure',
      '3': 13,
      '4': 1,
      '5': 8,
      '10': 'onScheduleFailure'
    },
    {'1': 'organization_id', '3': 14, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 15, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 16, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Notification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationDescriptor = $convert.base64Decode(
    'CgxOb3RpZmljYXRpb24SDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSVwoRbm'
    '90aWZpY2F0aW9uX3R5cGUYAyABKA4yKi5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblBy'
    'b3ZpZGVyVHlwZVIQbm90aWZpY2F0aW9uVHlwZRIiCg1vbl9hcHBfZGVwbG95GAQgASgIUgtvbk'
    'FwcERlcGxveRIrChJvbl9hcHBfYnVpbGRfZXJyb3IYBSABKAhSD29uQXBwQnVpbGRFcnJvchIs'
    'ChJvbl9kYXRhYmFzZV9iYWNrdXAYBiABKAhSEG9uRGF0YWJhc2VCYWNrdXASKAoQb25fdm9sdW'
    '1lX2JhY2t1cBgHIAEoCFIOb25Wb2x1bWVCYWNrdXASKAoQb25fcGFuZWxfcmVzdGFydBgIIAEo'
    'CFIOb25QYW5lbFJlc3RhcnQSKgoRb25fZG9ja2VyX2NsZWFudXAYCSABKAhSD29uRG9ja2VyQ2'
    'xlYW51cBIuChNvbl9zZXJ2ZXJfdGhyZXNob2xkGAogASgIUhFvblNlcnZlclRocmVzaG9sZBIm'
    'Cg9vbl9wYW5lbF9iYWNrdXAYCyABKAhSDW9uUGFuZWxCYWNrdXASLgoTb25fc2NoZWR1bGVfc3'
    'VjY2VzcxgMIAEoCFIRb25TY2hlZHVsZVN1Y2Nlc3MSLgoTb25fc2NoZWR1bGVfZmFpbHVyZRgN'
    'IAEoCFIRb25TY2hlZHVsZUZhaWx1cmUSJwoPb3JnYW5pemF0aW9uX2lkGA4gASgDUg5vcmdhbm'
    'l6YXRpb25JZBIdCgpjcmVhdGVkX2F0GA8gASgDUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgQ'
    'IAEoA1IJdXBkYXRlZEF0');

@$core.Deprecated('Use notificationDeliveryAttemptDescriptor instead')
const NotificationDeliveryAttempt$json = {
  '1': 'NotificationDeliveryAttempt',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'notification_id', '3': 2, '4': 1, '5': 3, '10': 'notificationId'},
    {'1': 'organization_id', '3': 3, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'trigger_name', '3': 4, '4': 1, '5': 9, '10': 'triggerName'},
    {'1': 'correlation_id', '3': 5, '4': 1, '5': 9, '10': 'correlationId'},
    {'1': 'status', '3': 6, '4': 1, '5': 9, '10': 'status'},
    {'1': 'attempt', '3': 7, '4': 1, '5': 3, '10': 'attempt'},
    {'1': 'title', '3': 8, '4': 1, '5': 9, '10': 'title'},
    {'1': 'body', '3': 9, '4': 1, '5': 9, '10': 'body'},
    {'1': 'error', '3': 10, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
    {'1': 'created_at', '3': 11, '4': 1, '5': 3, '10': 'createdAt'},
    {
      '1': 'finished_at',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'finishedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_error'},
    {'1': '_finished_at'},
  ],
};

/// Descriptor for `NotificationDeliveryAttempt`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationDeliveryAttemptDescriptor = $convert.base64Decode(
    'ChtOb3RpZmljYXRpb25EZWxpdmVyeUF0dGVtcHQSDgoCaWQYASABKANSAmlkEicKD25vdGlmaW'
    'NhdGlvbl9pZBgCIAEoA1IObm90aWZpY2F0aW9uSWQSJwoPb3JnYW5pemF0aW9uX2lkGAMgASgD'
    'Ug5vcmdhbml6YXRpb25JZBIhCgx0cmlnZ2VyX25hbWUYBCABKAlSC3RyaWdnZXJOYW1lEiUKDm'
    'NvcnJlbGF0aW9uX2lkGAUgASgJUg1jb3JyZWxhdGlvbklkEhYKBnN0YXR1cxgGIAEoCVIGc3Rh'
    'dHVzEhgKB2F0dGVtcHQYByABKANSB2F0dGVtcHQSFAoFdGl0bGUYCCABKAlSBXRpdGxlEhIKBG'
    'JvZHkYCSABKAlSBGJvZHkSGQoFZXJyb3IYCiABKAlIAFIFZXJyb3KIAQESHQoKY3JlYXRlZF9h'
    'dBgLIAEoA1IJY3JlYXRlZEF0EiQKC2ZpbmlzaGVkX2F0GAwgASgDSAFSCmZpbmlzaGVkQXSIAQ'
    'FCCAoGX2Vycm9yQg4KDF9maW5pc2hlZF9hdA==');

@$core.Deprecated('Use notificationBindingDescriptor instead')
const NotificationBinding$json = {
  '1': 'NotificationBinding',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'notification_id', '3': 2, '4': 1, '5': 3, '10': 'notificationId'},
    {'1': 'organization_id', '3': 3, '4': 1, '5': 3, '10': 'organizationId'},
    {
      '1': 'resource_type',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.NotificationResourceType',
      '10': 'resourceType'
    },
    {'1': 'resource_id', '3': 5, '4': 1, '5': 3, '10': 'resourceId'},
    {'1': 'created_at', '3': 6, '4': 1, '5': 3, '10': 'createdAt'},
  ],
};

/// Descriptor for `NotificationBinding`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationBindingDescriptor = $convert.base64Decode(
    'ChNOb3RpZmljYXRpb25CaW5kaW5nEg4KAmlkGAEgASgDUgJpZBInCg9ub3RpZmljYXRpb25faW'
    'QYAiABKANSDm5vdGlmaWNhdGlvbklkEicKD29yZ2FuaXphdGlvbl9pZBgDIAEoA1IOb3JnYW5p'
    'emF0aW9uSWQSTwoNcmVzb3VyY2VfdHlwZRgEIAEoDjIqLm9wZW5veGlkZS5hcGkudjEuTm90aW'
    'ZpY2F0aW9uUmVzb3VyY2VUeXBlUgxyZXNvdXJjZVR5cGUSHwoLcmVzb3VyY2VfaWQYBSABKANS'
    'CnJlc291cmNlSWQSHQoKY3JlYXRlZF9hdBgGIAEoA1IJY3JlYXRlZEF0');

@$core.Deprecated('Use notificationSlackConfigDescriptor instead')
const NotificationSlackConfig$json = {
  '1': 'NotificationSlackConfig',
  '2': [
    {'1': 'webhook_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'webhookUrl'},
    {
      '1': 'channel',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'channel',
      '17': true
    },
  ],
  '8': [
    {'1': '_channel'},
  ],
};

/// Descriptor for `NotificationSlackConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationSlackConfigDescriptor = $convert.base64Decode(
    'ChdOb3RpZmljYXRpb25TbGFja0NvbmZpZxIoCgt3ZWJob29rX3VybBgBIAEoCUIHukgEcgIQAV'
    'IKd2ViaG9va1VybBIdCgdjaGFubmVsGAIgASgJSABSB2NoYW5uZWyIAQFCCgoIX2NoYW5uZWw=');

@$core.Deprecated('Use notificationTelegramConfigDescriptor instead')
const NotificationTelegramConfig$json = {
  '1': 'NotificationTelegramConfig',
  '2': [
    {'1': 'bot_token', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'botToken'},
    {'1': 'chat_id', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'chatId'},
    {
      '1': 'message_thread_id',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'messageThreadId',
      '17': true
    },
  ],
  '8': [
    {'1': '_message_thread_id'},
  ],
};

/// Descriptor for `NotificationTelegramConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationTelegramConfigDescriptor = $convert.base64Decode(
    'ChpOb3RpZmljYXRpb25UZWxlZ3JhbUNvbmZpZxIkCglib3RfdG9rZW4YASABKAlCB7pIBHICEA'
    'FSCGJvdFRva2VuEiAKB2NoYXRfaWQYAiABKAlCB7pIBHICEAFSBmNoYXRJZBIvChFtZXNzYWdl'
    'X3RocmVhZF9pZBgDIAEoCUgAUg9tZXNzYWdlVGhyZWFkSWSIAQFCFAoSX21lc3NhZ2VfdGhyZW'
    'FkX2lk');

@$core.Deprecated('Use notificationDiscordConfigDescriptor instead')
const NotificationDiscordConfig$json = {
  '1': 'NotificationDiscordConfig',
  '2': [
    {'1': 'webhook_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'webhookUrl'},
    {'1': 'decoration', '3': 2, '4': 1, '5': 8, '10': 'decoration'},
  ],
};

/// Descriptor for `NotificationDiscordConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationDiscordConfigDescriptor =
    $convert.base64Decode(
        'ChlOb3RpZmljYXRpb25EaXNjb3JkQ29uZmlnEigKC3dlYmhvb2tfdXJsGAEgASgJQge6SARyAh'
        'ABUgp3ZWJob29rVXJsEh4KCmRlY29yYXRpb24YAiABKAhSCmRlY29yYXRpb24=');

@$core.Deprecated('Use notificationEmailConfigDescriptor instead')
const NotificationEmailConfig$json = {
  '1': 'NotificationEmailConfig',
  '2': [
    {'1': 'smtp_server', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'smtpServer'},
    {'1': 'smtp_port', '3': 2, '4': 1, '5': 3, '10': 'smtpPort'},
    {'1': 'username', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'username'},
    {'1': 'password', '3': 4, '4': 1, '5': 9, '10': 'password'},
    {'1': 'from_address', '3': 5, '4': 1, '5': 9, '8': {}, '10': 'fromAddress'},
    {'1': 'to_addresses', '3': 6, '4': 3, '5': 9, '8': {}, '10': 'toAddresses'},
  ],
};

/// Descriptor for `NotificationEmailConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationEmailConfigDescriptor = $convert.base64Decode(
    'ChdOb3RpZmljYXRpb25FbWFpbENvbmZpZxIoCgtzbXRwX3NlcnZlchgBIAEoCUIHukgEcgIQAV'
    'IKc210cFNlcnZlchIbCglzbXRwX3BvcnQYAiABKANSCHNtdHBQb3J0EiMKCHVzZXJuYW1lGAMg'
    'ASgJQge6SARyAhABUgh1c2VybmFtZRIaCghwYXNzd29yZBgEIAEoCVIIcGFzc3dvcmQSKgoMZn'
    'JvbV9hZGRyZXNzGAUgASgJQge6SARyAhABUgtmcm9tQWRkcmVzcxIrCgx0b19hZGRyZXNzZXMY'
    'BiADKAlCCLpIBZIBAggBUgt0b0FkZHJlc3Nlcw==');

@$core.Deprecated('Use notificationResendConfigDescriptor instead')
const NotificationResendConfig$json = {
  '1': 'NotificationResendConfig',
  '2': [
    {'1': 'api_key', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'apiKey'},
    {'1': 'from_address', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'fromAddress'},
    {'1': 'to_addresses', '3': 3, '4': 3, '5': 9, '8': {}, '10': 'toAddresses'},
  ],
};

/// Descriptor for `NotificationResendConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationResendConfigDescriptor = $convert.base64Decode(
    'ChhOb3RpZmljYXRpb25SZXNlbmRDb25maWcSIAoHYXBpX2tleRgBIAEoCUIHukgEcgIQAVIGYX'
    'BpS2V5EioKDGZyb21fYWRkcmVzcxgCIAEoCUIHukgEcgIQAVILZnJvbUFkZHJlc3MSKwoMdG9f'
    'YWRkcmVzc2VzGAMgAygJQgi6SAWSAQIIAVILdG9BZGRyZXNzZXM=');

@$core.Deprecated('Use notificationGotifyConfigDescriptor instead')
const NotificationGotifyConfig$json = {
  '1': 'NotificationGotifyConfig',
  '2': [
    {'1': 'server_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'serverUrl'},
    {'1': 'app_token', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'appToken'},
    {'1': 'priority', '3': 3, '4': 1, '5': 3, '10': 'priority'},
    {'1': 'decoration', '3': 4, '4': 1, '5': 8, '10': 'decoration'},
  ],
};

/// Descriptor for `NotificationGotifyConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationGotifyConfigDescriptor = $convert.base64Decode(
    'ChhOb3RpZmljYXRpb25Hb3RpZnlDb25maWcSJgoKc2VydmVyX3VybBgBIAEoCUIHukgEcgIQAV'
    'IJc2VydmVyVXJsEiQKCWFwcF90b2tlbhgCIAEoCUIHukgEcgIQAVIIYXBwVG9rZW4SGgoIcHJp'
    'b3JpdHkYAyABKANSCHByaW9yaXR5Eh4KCmRlY29yYXRpb24YBCABKAhSCmRlY29yYXRpb24=');

@$core.Deprecated('Use notificationNtfyConfigDescriptor instead')
const NotificationNtfyConfig$json = {
  '1': 'NotificationNtfyConfig',
  '2': [
    {'1': 'server_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'serverUrl'},
    {'1': 'topic', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'topic'},
    {
      '1': 'access_token',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'accessToken',
      '17': true
    },
    {'1': 'priority', '3': 4, '4': 1, '5': 3, '10': 'priority'},
  ],
  '8': [
    {'1': '_access_token'},
  ],
};

/// Descriptor for `NotificationNtfyConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationNtfyConfigDescriptor = $convert.base64Decode(
    'ChZOb3RpZmljYXRpb25OdGZ5Q29uZmlnEiYKCnNlcnZlcl91cmwYASABKAlCB7pIBHICEAFSCX'
    'NlcnZlclVybBIdCgV0b3BpYxgCIAEoCUIHukgEcgIQAVIFdG9waWMSJgoMYWNjZXNzX3Rva2Vu'
    'GAMgASgJSABSC2FjY2Vzc1Rva2VuiAEBEhoKCHByaW9yaXR5GAQgASgDUghwcmlvcml0eUIPCg'
    '1fYWNjZXNzX3Rva2Vu');

@$core.Deprecated('Use notificationMattermostConfigDescriptor instead')
const NotificationMattermostConfig$json = {
  '1': 'NotificationMattermostConfig',
  '2': [
    {'1': 'webhook_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'webhookUrl'},
    {
      '1': 'channel',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'channel',
      '17': true
    },
    {
      '1': 'username',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'username',
      '17': true
    },
  ],
  '8': [
    {'1': '_channel'},
    {'1': '_username'},
  ],
};

/// Descriptor for `NotificationMattermostConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationMattermostConfigDescriptor =
    $convert.base64Decode(
        'ChxOb3RpZmljYXRpb25NYXR0ZXJtb3N0Q29uZmlnEigKC3dlYmhvb2tfdXJsGAEgASgJQge6SA'
        'RyAhABUgp3ZWJob29rVXJsEh0KB2NoYW5uZWwYAiABKAlIAFIHY2hhbm5lbIgBARIfCgh1c2Vy'
        'bmFtZRgDIAEoCUgBUgh1c2VybmFtZYgBAUIKCghfY2hhbm5lbEILCglfdXNlcm5hbWU=');

@$core.Deprecated('Use notificationCustomConfigDescriptor instead')
const NotificationCustomConfig$json = {
  '1': 'NotificationCustomConfig',
  '2': [
    {'1': 'endpoint', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'endpoint'},
    {
      '1': 'headers',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'headers',
      '17': true
    },
  ],
  '8': [
    {'1': '_headers'},
  ],
};

/// Descriptor for `NotificationCustomConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationCustomConfigDescriptor =
    $convert.base64Decode(
        'ChhOb3RpZmljYXRpb25DdXN0b21Db25maWcSIwoIZW5kcG9pbnQYASABKAlCB7pIBHICEAFSCG'
        'VuZHBvaW50Eh0KB2hlYWRlcnMYAiABKAlIAFIHaGVhZGVyc4gBAUIKCghfaGVhZGVycw==');

@$core.Deprecated('Use notificationWebhookUrlConfigDescriptor instead')
const NotificationWebhookUrlConfig$json = {
  '1': 'NotificationWebhookUrlConfig',
  '2': [
    {'1': 'webhook_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'webhookUrl'},
  ],
};

/// Descriptor for `NotificationWebhookUrlConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationWebhookUrlConfigDescriptor =
    $convert.base64Decode(
        'ChxOb3RpZmljYXRpb25XZWJob29rVXJsQ29uZmlnEigKC3dlYmhvb2tfdXJsGAEgASgJQge6SA'
        'RyAhABUgp3ZWJob29rVXJs');

@$core.Deprecated('Use notificationPushoverConfigDescriptor instead')
const NotificationPushoverConfig$json = {
  '1': 'NotificationPushoverConfig',
  '2': [
    {'1': 'user_key', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'userKey'},
    {'1': 'api_token', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'apiToken'},
    {'1': 'priority', '3': 3, '4': 1, '5': 3, '10': 'priority'},
    {'1': 'retry', '3': 4, '4': 1, '5': 3, '9': 0, '10': 'retry', '17': true},
    {'1': 'expire', '3': 5, '4': 1, '5': 3, '9': 1, '10': 'expire', '17': true},
  ],
  '8': [
    {'1': '_retry'},
    {'1': '_expire'},
  ],
};

/// Descriptor for `NotificationPushoverConfig`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationPushoverConfigDescriptor = $convert.base64Decode(
    'ChpOb3RpZmljYXRpb25QdXNob3ZlckNvbmZpZxIiCgh1c2VyX2tleRgBIAEoCUIHukgEcgIQAV'
    'IHdXNlcktleRIkCglhcGlfdG9rZW4YAiABKAlCB7pIBHICEAFSCGFwaVRva2VuEhoKCHByaW9y'
    'aXR5GAMgASgDUghwcmlvcml0eRIZCgVyZXRyeRgEIAEoA0gAUgVyZXRyeYgBARIbCgZleHBpcm'
    'UYBSABKANIAVIGZXhwaXJliAEBQggKBl9yZXRyeUIJCgdfZXhwaXJl');

@$core.Deprecated('Use listNotificationsRequestDescriptor instead')
const ListNotificationsRequest$json = {
  '1': 'ListNotificationsRequest',
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

/// Descriptor for `ListNotificationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationsRequestDescriptor =
    $convert.base64Decode(
        'ChhMaXN0Tm90aWZpY2F0aW9uc1JlcXVlc3QSNQoPb3JnYW5pemF0aW9uX2lkGAEgASgDQge6SA'
        'QiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use listNotificationsResponseDescriptor instead')
const ListNotificationsResponse$json = {
  '1': 'ListNotificationsResponse',
  '2': [
    {
      '1': 'notifications',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.Notification',
      '10': 'notifications'
    },
  ],
};

/// Descriptor for `ListNotificationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationsResponseDescriptor =
    $convert.base64Decode(
        'ChlMaXN0Tm90aWZpY2F0aW9uc1Jlc3BvbnNlEkQKDW5vdGlmaWNhdGlvbnMYASADKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblINbm90aWZpY2F0aW9ucw==');

@$core.Deprecated('Use getNotificationRequestDescriptor instead')
const GetNotificationRequest$json = {
  '1': 'GetNotificationRequest',
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

/// Descriptor for `GetNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getNotificationRequestDescriptor = $convert.base64Decode(
    'ChZHZXROb3RpZmljYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg9vcm'
    'dhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQX29y'
    'Z2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use getNotificationResponseDescriptor instead')
const GetNotificationResponse$json = {
  '1': 'GetNotificationResponse',
  '2': [
    {
      '1': 'notification',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Notification',
      '10': 'notification'
    },
  ],
};

/// Descriptor for `GetNotificationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getNotificationResponseDescriptor =
    $convert.base64Decode(
        'ChdHZXROb3RpZmljYXRpb25SZXNwb25zZRJCCgxub3RpZmljYXRpb24YASABKAsyHi5vcGVub3'
        'hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblIMbm90aWZpY2F0aW9u');

@$core.Deprecated('Use createNotificationRequestDescriptor instead')
const CreateNotificationRequest$json = {
  '1': 'CreateNotificationRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
    {'1': 'on_app_deploy', '3': 3, '4': 1, '5': 8, '10': 'onAppDeploy'},
    {
      '1': 'on_app_build_error',
      '3': 4,
      '4': 1,
      '5': 8,
      '10': 'onAppBuildError'
    },
    {
      '1': 'on_database_backup',
      '3': 5,
      '4': 1,
      '5': 8,
      '10': 'onDatabaseBackup'
    },
    {'1': 'on_volume_backup', '3': 6, '4': 1, '5': 8, '10': 'onVolumeBackup'},
    {'1': 'on_panel_restart', '3': 7, '4': 1, '5': 8, '10': 'onPanelRestart'},
    {'1': 'on_docker_cleanup', '3': 8, '4': 1, '5': 8, '10': 'onDockerCleanup'},
    {
      '1': 'on_server_threshold',
      '3': 9,
      '4': 1,
      '5': 8,
      '10': 'onServerThreshold'
    },
    {'1': 'on_panel_backup', '3': 10, '4': 1, '5': 8, '10': 'onPanelBackup'},
    {
      '1': 'on_schedule_success',
      '3': 11,
      '4': 1,
      '5': 8,
      '10': 'onScheduleSuccess'
    },
    {
      '1': 'on_schedule_failure',
      '3': 12,
      '4': 1,
      '5': 8,
      '10': 'onScheduleFailure'
    },
    {
      '1': 'slack',
      '3': 13,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationSlackConfig',
      '9': 0,
      '10': 'slack'
    },
    {
      '1': 'telegram',
      '3': 14,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationTelegramConfig',
      '9': 0,
      '10': 'telegram'
    },
    {
      '1': 'discord',
      '3': 15,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationDiscordConfig',
      '9': 0,
      '10': 'discord'
    },
    {
      '1': 'email',
      '3': 16,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationEmailConfig',
      '9': 0,
      '10': 'email'
    },
    {
      '1': 'resend',
      '3': 17,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationResendConfig',
      '9': 0,
      '10': 'resend'
    },
    {
      '1': 'gotify',
      '3': 18,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationGotifyConfig',
      '9': 0,
      '10': 'gotify'
    },
    {
      '1': 'ntfy',
      '3': 19,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationNtfyConfig',
      '9': 0,
      '10': 'ntfy'
    },
    {
      '1': 'mattermost',
      '3': 20,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationMattermostConfig',
      '9': 0,
      '10': 'mattermost'
    },
    {
      '1': 'custom',
      '3': 21,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationCustomConfig',
      '9': 0,
      '10': 'custom'
    },
    {
      '1': 'lark',
      '3': 22,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationWebhookUrlConfig',
      '9': 0,
      '10': 'lark'
    },
    {
      '1': 'pushover',
      '3': 23,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationPushoverConfig',
      '9': 0,
      '10': 'pushover'
    },
    {
      '1': 'teams',
      '3': 24,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationWebhookUrlConfig',
      '9': 0,
      '10': 'teams'
    },
  ],
  '8': [
    {'1': 'provider'},
  ],
};

/// Descriptor for `CreateNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createNotificationRequestDescriptor = $convert.base64Decode(
    'ChlDcmVhdGVOb3RpZmljYXRpb25SZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG'
    '5hbWUSMAoPb3JnYW5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAAUg5vcmdhbml6YXRpb25JZBIi'
    'Cg1vbl9hcHBfZGVwbG95GAMgASgIUgtvbkFwcERlcGxveRIrChJvbl9hcHBfYnVpbGRfZXJyb3'
    'IYBCABKAhSD29uQXBwQnVpbGRFcnJvchIsChJvbl9kYXRhYmFzZV9iYWNrdXAYBSABKAhSEG9u'
    'RGF0YWJhc2VCYWNrdXASKAoQb25fdm9sdW1lX2JhY2t1cBgGIAEoCFIOb25Wb2x1bWVCYWNrdX'
    'ASKAoQb25fcGFuZWxfcmVzdGFydBgHIAEoCFIOb25QYW5lbFJlc3RhcnQSKgoRb25fZG9ja2Vy'
    'X2NsZWFudXAYCCABKAhSD29uRG9ja2VyQ2xlYW51cBIuChNvbl9zZXJ2ZXJfdGhyZXNob2xkGA'
    'kgASgIUhFvblNlcnZlclRocmVzaG9sZBImCg9vbl9wYW5lbF9iYWNrdXAYCiABKAhSDW9uUGFu'
    'ZWxCYWNrdXASLgoTb25fc2NoZWR1bGVfc3VjY2VzcxgLIAEoCFIRb25TY2hlZHVsZVN1Y2Nlc3'
    'MSLgoTb25fc2NoZWR1bGVfZmFpbHVyZRgMIAEoCFIRb25TY2hlZHVsZUZhaWx1cmUSQQoFc2xh'
    'Y2sYDSABKAsyKS5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblNsYWNrQ29uZmlnSABSBX'
    'NsYWNrEkoKCHRlbGVncmFtGA4gASgLMiwub3Blbm94aWRlLmFwaS52MS5Ob3RpZmljYXRpb25U'
    'ZWxlZ3JhbUNvbmZpZ0gAUgh0ZWxlZ3JhbRJHCgdkaXNjb3JkGA8gASgLMisub3Blbm94aWRlLm'
    'FwaS52MS5Ob3RpZmljYXRpb25EaXNjb3JkQ29uZmlnSABSB2Rpc2NvcmQSQQoFZW1haWwYECAB'
    'KAsyKS5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbkVtYWlsQ29uZmlnSABSBWVtYWlsEk'
    'QKBnJlc2VuZBgRIAEoCzIqLm9wZW5veGlkZS5hcGkudjEuTm90aWZpY2F0aW9uUmVzZW5kQ29u'
    'ZmlnSABSBnJlc2VuZBJECgZnb3RpZnkYEiABKAsyKi5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaW'
    'NhdGlvbkdvdGlmeUNvbmZpZ0gAUgZnb3RpZnkSPgoEbnRmeRgTIAEoCzIoLm9wZW5veGlkZS5h'
    'cGkudjEuTm90aWZpY2F0aW9uTnRmeUNvbmZpZ0gAUgRudGZ5ElAKCm1hdHRlcm1vc3QYFCABKA'
    'syLi5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbk1hdHRlcm1vc3RDb25maWdIAFIKbWF0'
    'dGVybW9zdBJECgZjdXN0b20YFSABKAsyKi5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbk'
    'N1c3RvbUNvbmZpZ0gAUgZjdXN0b20SRAoEbGFyaxgWIAEoCzIuLm9wZW5veGlkZS5hcGkudjEu'
    'Tm90aWZpY2F0aW9uV2ViaG9va1VybENvbmZpZ0gAUgRsYXJrEkoKCHB1c2hvdmVyGBcgASgLMi'
    'wub3Blbm94aWRlLmFwaS52MS5Ob3RpZmljYXRpb25QdXNob3ZlckNvbmZpZ0gAUghwdXNob3Zl'
    'chJGCgV0ZWFtcxgYIAEoCzIuLm9wZW5veGlkZS5hcGkudjEuTm90aWZpY2F0aW9uV2ViaG9va1'
    'VybENvbmZpZ0gAUgV0ZWFtc0IKCghwcm92aWRlcg==');

@$core.Deprecated('Use createNotificationResponseDescriptor instead')
const CreateNotificationResponse$json = {
  '1': 'CreateNotificationResponse',
  '2': [
    {
      '1': 'notification',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Notification',
      '10': 'notification'
    },
  ],
};

/// Descriptor for `CreateNotificationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createNotificationResponseDescriptor =
    $convert.base64Decode(
        'ChpDcmVhdGVOb3RpZmljYXRpb25SZXNwb25zZRJCCgxub3RpZmljYXRpb24YASABKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblIMbm90aWZpY2F0aW9u');

@$core.Deprecated('Use updateNotificationRequestDescriptor instead')
const UpdateNotificationRequest$json = {
  '1': 'UpdateNotificationRequest',
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
      '1': 'name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'name',
      '17': true
    },
    {
      '1': 'on_app_deploy',
      '3': 4,
      '4': 1,
      '5': 8,
      '9': 2,
      '10': 'onAppDeploy',
      '17': true
    },
    {
      '1': 'on_app_build_error',
      '3': 5,
      '4': 1,
      '5': 8,
      '9': 3,
      '10': 'onAppBuildError',
      '17': true
    },
    {
      '1': 'on_database_backup',
      '3': 6,
      '4': 1,
      '5': 8,
      '9': 4,
      '10': 'onDatabaseBackup',
      '17': true
    },
    {
      '1': 'on_volume_backup',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 5,
      '10': 'onVolumeBackup',
      '17': true
    },
    {
      '1': 'on_panel_restart',
      '3': 8,
      '4': 1,
      '5': 8,
      '9': 6,
      '10': 'onPanelRestart',
      '17': true
    },
    {
      '1': 'on_docker_cleanup',
      '3': 9,
      '4': 1,
      '5': 8,
      '9': 7,
      '10': 'onDockerCleanup',
      '17': true
    },
    {
      '1': 'on_server_threshold',
      '3': 10,
      '4': 1,
      '5': 8,
      '9': 8,
      '10': 'onServerThreshold',
      '17': true
    },
    {
      '1': 'on_panel_backup',
      '3': 11,
      '4': 1,
      '5': 8,
      '9': 9,
      '10': 'onPanelBackup',
      '17': true
    },
    {
      '1': 'on_schedule_success',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 10,
      '10': 'onScheduleSuccess',
      '17': true
    },
    {
      '1': 'on_schedule_failure',
      '3': 13,
      '4': 1,
      '5': 8,
      '9': 11,
      '10': 'onScheduleFailure',
      '17': true
    },
    {
      '1': 'webhook_url',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 12,
      '10': 'webhookUrl',
      '17': true
    },
    {
      '1': 'channel',
      '3': 15,
      '4': 1,
      '5': 9,
      '9': 13,
      '10': 'channel',
      '17': true
    },
    {
      '1': 'username',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 14,
      '10': 'username',
      '17': true
    },
    {
      '1': 'decoration',
      '3': 17,
      '4': 1,
      '5': 3,
      '9': 15,
      '10': 'decoration',
      '17': true
    },
    {
      '1': 'bot_token',
      '3': 18,
      '4': 1,
      '5': 9,
      '9': 16,
      '10': 'botToken',
      '17': true
    },
    {
      '1': 'chat_id',
      '3': 19,
      '4': 1,
      '5': 9,
      '9': 17,
      '10': 'chatId',
      '17': true
    },
    {
      '1': 'message_thread_id',
      '3': 20,
      '4': 1,
      '5': 9,
      '9': 18,
      '10': 'messageThreadId',
      '17': true
    },
    {
      '1': 'smtp_server',
      '3': 21,
      '4': 1,
      '5': 9,
      '9': 19,
      '10': 'smtpServer',
      '17': true
    },
    {
      '1': 'smtp_port',
      '3': 22,
      '4': 1,
      '5': 3,
      '9': 20,
      '10': 'smtpPort',
      '17': true
    },
    {
      '1': 'password',
      '3': 23,
      '4': 1,
      '5': 9,
      '9': 21,
      '10': 'password',
      '17': true
    },
    {
      '1': 'from_address',
      '3': 24,
      '4': 1,
      '5': 9,
      '9': 22,
      '10': 'fromAddress',
      '17': true
    },
    {'1': 'to_addresses', '3': 25, '4': 3, '5': 9, '10': 'toAddresses'},
    {
      '1': 'api_key',
      '3': 26,
      '4': 1,
      '5': 9,
      '9': 23,
      '10': 'apiKey',
      '17': true
    },
    {
      '1': 'server_url',
      '3': 27,
      '4': 1,
      '5': 9,
      '9': 24,
      '10': 'serverUrl',
      '17': true
    },
    {
      '1': 'app_token',
      '3': 28,
      '4': 1,
      '5': 9,
      '9': 25,
      '10': 'appToken',
      '17': true
    },
    {
      '1': 'priority',
      '3': 29,
      '4': 1,
      '5': 3,
      '9': 26,
      '10': 'priority',
      '17': true
    },
    {'1': 'topic', '3': 30, '4': 1, '5': 9, '9': 27, '10': 'topic', '17': true},
    {
      '1': 'access_token',
      '3': 31,
      '4': 1,
      '5': 9,
      '9': 28,
      '10': 'accessToken',
      '17': true
    },
    {
      '1': 'endpoint',
      '3': 32,
      '4': 1,
      '5': 9,
      '9': 29,
      '10': 'endpoint',
      '17': true
    },
    {
      '1': 'headers',
      '3': 33,
      '4': 1,
      '5': 9,
      '9': 30,
      '10': 'headers',
      '17': true
    },
    {
      '1': 'user_key',
      '3': 34,
      '4': 1,
      '5': 9,
      '9': 31,
      '10': 'userKey',
      '17': true
    },
    {
      '1': 'api_token',
      '3': 35,
      '4': 1,
      '5': 9,
      '9': 32,
      '10': 'apiToken',
      '17': true
    },
    {'1': 'retry', '3': 36, '4': 1, '5': 3, '9': 33, '10': 'retry', '17': true},
    {
      '1': 'expire',
      '3': 37,
      '4': 1,
      '5': 3,
      '9': 34,
      '10': 'expire',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_name'},
    {'1': '_on_app_deploy'},
    {'1': '_on_app_build_error'},
    {'1': '_on_database_backup'},
    {'1': '_on_volume_backup'},
    {'1': '_on_panel_restart'},
    {'1': '_on_docker_cleanup'},
    {'1': '_on_server_threshold'},
    {'1': '_on_panel_backup'},
    {'1': '_on_schedule_success'},
    {'1': '_on_schedule_failure'},
    {'1': '_webhook_url'},
    {'1': '_channel'},
    {'1': '_username'},
    {'1': '_decoration'},
    {'1': '_bot_token'},
    {'1': '_chat_id'},
    {'1': '_message_thread_id'},
    {'1': '_smtp_server'},
    {'1': '_smtp_port'},
    {'1': '_password'},
    {'1': '_from_address'},
    {'1': '_api_key'},
    {'1': '_server_url'},
    {'1': '_app_token'},
    {'1': '_priority'},
    {'1': '_topic'},
    {'1': '_access_token'},
    {'1': '_endpoint'},
    {'1': '_headers'},
    {'1': '_user_key'},
    {'1': '_api_token'},
    {'1': '_retry'},
    {'1': '_expire'},
  ],
};

/// Descriptor for `UpdateNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateNotificationRequestDescriptor = $convert.base64Decode(
    'ChlVcGRhdGVOb3RpZmljYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg'
    '9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESIQoE'
    'bmFtZRgDIAEoCUIIukgFcgMY/wFIAVIEbmFtZYgBARInCg1vbl9hcHBfZGVwbG95GAQgASgISA'
    'JSC29uQXBwRGVwbG95iAEBEjAKEm9uX2FwcF9idWlsZF9lcnJvchgFIAEoCEgDUg9vbkFwcEJ1'
    'aWxkRXJyb3KIAQESMQoSb25fZGF0YWJhc2VfYmFja3VwGAYgASgISARSEG9uRGF0YWJhc2VCYW'
    'NrdXCIAQESLQoQb25fdm9sdW1lX2JhY2t1cBgHIAEoCEgFUg5vblZvbHVtZUJhY2t1cIgBARIt'
    'ChBvbl9wYW5lbF9yZXN0YXJ0GAggASgISAZSDm9uUGFuZWxSZXN0YXJ0iAEBEi8KEW9uX2RvY2'
    'tlcl9jbGVhbnVwGAkgASgISAdSD29uRG9ja2VyQ2xlYW51cIgBARIzChNvbl9zZXJ2ZXJfdGhy'
    'ZXNob2xkGAogASgISAhSEW9uU2VydmVyVGhyZXNob2xkiAEBEisKD29uX3BhbmVsX2JhY2t1cB'
    'gLIAEoCEgJUg1vblBhbmVsQmFja3VwiAEBEjMKE29uX3NjaGVkdWxlX3N1Y2Nlc3MYDCABKAhI'
    'ClIRb25TY2hlZHVsZVN1Y2Nlc3OIAQESMwoTb25fc2NoZWR1bGVfZmFpbHVyZRgNIAEoCEgLUh'
    'FvblNjaGVkdWxlRmFpbHVyZYgBARIkCgt3ZWJob29rX3VybBgOIAEoCUgMUgp3ZWJob29rVXJs'
    'iAEBEh0KB2NoYW5uZWwYDyABKAlIDVIHY2hhbm5lbIgBARIfCgh1c2VybmFtZRgQIAEoCUgOUg'
    'h1c2VybmFtZYgBARIjCgpkZWNvcmF0aW9uGBEgASgDSA9SCmRlY29yYXRpb26IAQESIAoJYm90'
    'X3Rva2VuGBIgASgJSBBSCGJvdFRva2VuiAEBEhwKB2NoYXRfaWQYEyABKAlIEVIGY2hhdElkiA'
    'EBEi8KEW1lc3NhZ2VfdGhyZWFkX2lkGBQgASgJSBJSD21lc3NhZ2VUaHJlYWRJZIgBARIkCgtz'
    'bXRwX3NlcnZlchgVIAEoCUgTUgpzbXRwU2VydmVyiAEBEiAKCXNtdHBfcG9ydBgWIAEoA0gUUg'
    'hzbXRwUG9ydIgBARIfCghwYXNzd29yZBgXIAEoCUgVUghwYXNzd29yZIgBARImCgxmcm9tX2Fk'
    'ZHJlc3MYGCABKAlIFlILZnJvbUFkZHJlc3OIAQESIQoMdG9fYWRkcmVzc2VzGBkgAygJUgt0b0'
    'FkZHJlc3NlcxIcCgdhcGlfa2V5GBogASgJSBdSBmFwaUtleYgBARIiCgpzZXJ2ZXJfdXJsGBsg'
    'ASgJSBhSCXNlcnZlclVybIgBARIgCglhcHBfdG9rZW4YHCABKAlIGVIIYXBwVG9rZW6IAQESHw'
    'oIcHJpb3JpdHkYHSABKANIGlIIcHJpb3JpdHmIAQESGQoFdG9waWMYHiABKAlIG1IFdG9waWOI'
    'AQESJgoMYWNjZXNzX3Rva2VuGB8gASgJSBxSC2FjY2Vzc1Rva2VuiAEBEh8KCGVuZHBvaW50GC'
    'AgASgJSB1SCGVuZHBvaW50iAEBEh0KB2hlYWRlcnMYISABKAlIHlIHaGVhZGVyc4gBARIeCgh1'
    'c2VyX2tleRgiIAEoCUgfUgd1c2VyS2V5iAEBEiAKCWFwaV90b2tlbhgjIAEoCUggUghhcGlUb2'
    'tlbogBARIZCgVyZXRyeRgkIAEoA0ghUgVyZXRyeYgBARIbCgZleHBpcmUYJSABKANIIlIGZXhw'
    'aXJliAEBQhIKEF9vcmdhbml6YXRpb25faWRCBwoFX25hbWVCEAoOX29uX2FwcF9kZXBsb3lCFQ'
    'oTX29uX2FwcF9idWlsZF9lcnJvckIVChNfb25fZGF0YWJhc2VfYmFja3VwQhMKEV9vbl92b2x1'
    'bWVfYmFja3VwQhMKEV9vbl9wYW5lbF9yZXN0YXJ0QhQKEl9vbl9kb2NrZXJfY2xlYW51cEIWCh'
    'Rfb25fc2VydmVyX3RocmVzaG9sZEISChBfb25fcGFuZWxfYmFja3VwQhYKFF9vbl9zY2hlZHVs'
    'ZV9zdWNjZXNzQhYKFF9vbl9zY2hlZHVsZV9mYWlsdXJlQg4KDF93ZWJob29rX3VybEIKCghfY2'
    'hhbm5lbEILCglfdXNlcm5hbWVCDQoLX2RlY29yYXRpb25CDAoKX2JvdF90b2tlbkIKCghfY2hh'
    'dF9pZEIUChJfbWVzc2FnZV90aHJlYWRfaWRCDgoMX3NtdHBfc2VydmVyQgwKCl9zbXRwX3Bvcn'
    'RCCwoJX3Bhc3N3b3JkQg8KDV9mcm9tX2FkZHJlc3NCCgoIX2FwaV9rZXlCDQoLX3NlcnZlcl91'
    'cmxCDAoKX2FwcF90b2tlbkILCglfcHJpb3JpdHlCCAoGX3RvcGljQg8KDV9hY2Nlc3NfdG9rZW'
    '5CCwoJX2VuZHBvaW50QgoKCF9oZWFkZXJzQgsKCV91c2VyX2tleUIMCgpfYXBpX3Rva2VuQggK'
    'Bl9yZXRyeUIJCgdfZXhwaXJl');

@$core.Deprecated('Use updateNotificationResponseDescriptor instead')
const UpdateNotificationResponse$json = {
  '1': 'UpdateNotificationResponse',
  '2': [
    {
      '1': 'notification',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Notification',
      '10': 'notification'
    },
  ],
};

/// Descriptor for `UpdateNotificationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateNotificationResponseDescriptor =
    $convert.base64Decode(
        'ChpVcGRhdGVOb3RpZmljYXRpb25SZXNwb25zZRJCCgxub3RpZmljYXRpb24YASABKAsyHi5vcG'
        'Vub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblIMbm90aWZpY2F0aW9u');

@$core.Deprecated('Use replaceNotificationRequestDescriptor instead')
const ReplaceNotificationRequest$json = {
  '1': 'ReplaceNotificationRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'organization_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'organizationId',
      '17': true
    },
    {'1': 'name', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'on_app_deploy', '3': 4, '4': 1, '5': 8, '10': 'onAppDeploy'},
    {
      '1': 'on_app_build_error',
      '3': 5,
      '4': 1,
      '5': 8,
      '10': 'onAppBuildError'
    },
    {
      '1': 'on_database_backup',
      '3': 6,
      '4': 1,
      '5': 8,
      '10': 'onDatabaseBackup'
    },
    {'1': 'on_volume_backup', '3': 7, '4': 1, '5': 8, '10': 'onVolumeBackup'},
    {'1': 'on_panel_restart', '3': 8, '4': 1, '5': 8, '10': 'onPanelRestart'},
    {'1': 'on_docker_cleanup', '3': 9, '4': 1, '5': 8, '10': 'onDockerCleanup'},
    {
      '1': 'on_server_threshold',
      '3': 10,
      '4': 1,
      '5': 8,
      '10': 'onServerThreshold'
    },
    {'1': 'on_panel_backup', '3': 11, '4': 1, '5': 8, '10': 'onPanelBackup'},
    {
      '1': 'on_schedule_success',
      '3': 12,
      '4': 1,
      '5': 8,
      '10': 'onScheduleSuccess'
    },
    {
      '1': 'on_schedule_failure',
      '3': 13,
      '4': 1,
      '5': 8,
      '10': 'onScheduleFailure'
    },
    {
      '1': 'slack',
      '3': 14,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationSlackConfig',
      '9': 0,
      '10': 'slack'
    },
    {
      '1': 'telegram',
      '3': 15,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationTelegramConfig',
      '9': 0,
      '10': 'telegram'
    },
    {
      '1': 'discord',
      '3': 16,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationDiscordConfig',
      '9': 0,
      '10': 'discord'
    },
    {
      '1': 'email',
      '3': 17,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationEmailConfig',
      '9': 0,
      '10': 'email'
    },
    {
      '1': 'resend',
      '3': 18,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationResendConfig',
      '9': 0,
      '10': 'resend'
    },
    {
      '1': 'gotify',
      '3': 19,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationGotifyConfig',
      '9': 0,
      '10': 'gotify'
    },
    {
      '1': 'ntfy',
      '3': 20,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationNtfyConfig',
      '9': 0,
      '10': 'ntfy'
    },
    {
      '1': 'mattermost',
      '3': 21,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationMattermostConfig',
      '9': 0,
      '10': 'mattermost'
    },
    {
      '1': 'custom',
      '3': 22,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationCustomConfig',
      '9': 0,
      '10': 'custom'
    },
    {
      '1': 'lark',
      '3': 23,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationWebhookUrlConfig',
      '9': 0,
      '10': 'lark'
    },
    {
      '1': 'pushover',
      '3': 24,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationPushoverConfig',
      '9': 0,
      '10': 'pushover'
    },
    {
      '1': 'teams',
      '3': 25,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationWebhookUrlConfig',
      '9': 0,
      '10': 'teams'
    },
  ],
  '8': [
    {'1': 'provider'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `ReplaceNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List replaceNotificationRequestDescriptor = $convert.base64Decode(
    'ChpSZXBsYWNlTm90aWZpY2F0aW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQ'
    'oPb3JnYW5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASAFSDm9yZ2FuaXphdGlvbklkiAEBEh4K'
    'BG5hbWUYAyABKAlCCrpIB3IFEAEY/wFSBG5hbWUSIgoNb25fYXBwX2RlcGxveRgEIAEoCFILb2'
    '5BcHBEZXBsb3kSKwoSb25fYXBwX2J1aWxkX2Vycm9yGAUgASgIUg9vbkFwcEJ1aWxkRXJyb3IS'
    'LAoSb25fZGF0YWJhc2VfYmFja3VwGAYgASgIUhBvbkRhdGFiYXNlQmFja3VwEigKEG9uX3ZvbH'
    'VtZV9iYWNrdXAYByABKAhSDm9uVm9sdW1lQmFja3VwEigKEG9uX3BhbmVsX3Jlc3RhcnQYCCAB'
    'KAhSDm9uUGFuZWxSZXN0YXJ0EioKEW9uX2RvY2tlcl9jbGVhbnVwGAkgASgIUg9vbkRvY2tlck'
    'NsZWFudXASLgoTb25fc2VydmVyX3RocmVzaG9sZBgKIAEoCFIRb25TZXJ2ZXJUaHJlc2hvbGQS'
    'JgoPb25fcGFuZWxfYmFja3VwGAsgASgIUg1vblBhbmVsQmFja3VwEi4KE29uX3NjaGVkdWxlX3'
    'N1Y2Nlc3MYDCABKAhSEW9uU2NoZWR1bGVTdWNjZXNzEi4KE29uX3NjaGVkdWxlX2ZhaWx1cmUY'
    'DSABKAhSEW9uU2NoZWR1bGVGYWlsdXJlEkEKBXNsYWNrGA4gASgLMikub3Blbm94aWRlLmFwaS'
    '52MS5Ob3RpZmljYXRpb25TbGFja0NvbmZpZ0gAUgVzbGFjaxJKCgh0ZWxlZ3JhbRgPIAEoCzIs'
    'Lm9wZW5veGlkZS5hcGkudjEuTm90aWZpY2F0aW9uVGVsZWdyYW1Db25maWdIAFIIdGVsZWdyYW'
    '0SRwoHZGlzY29yZBgQIAEoCzIrLm9wZW5veGlkZS5hcGkudjEuTm90aWZpY2F0aW9uRGlzY29y'
    'ZENvbmZpZ0gAUgdkaXNjb3JkEkEKBWVtYWlsGBEgASgLMikub3Blbm94aWRlLmFwaS52MS5Ob3'
    'RpZmljYXRpb25FbWFpbENvbmZpZ0gAUgVlbWFpbBJECgZyZXNlbmQYEiABKAsyKi5vcGVub3hp'
    'ZGUuYXBpLnYxLk5vdGlmaWNhdGlvblJlc2VuZENvbmZpZ0gAUgZyZXNlbmQSRAoGZ290aWZ5GB'
    'MgASgLMioub3Blbm94aWRlLmFwaS52MS5Ob3RpZmljYXRpb25Hb3RpZnlDb25maWdIAFIGZ290'
    'aWZ5Ej4KBG50ZnkYFCABKAsyKC5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbk50ZnlDb2'
    '5maWdIAFIEbnRmeRJQCgptYXR0ZXJtb3N0GBUgASgLMi4ub3Blbm94aWRlLmFwaS52MS5Ob3Rp'
    'ZmljYXRpb25NYXR0ZXJtb3N0Q29uZmlnSABSCm1hdHRlcm1vc3QSRAoGY3VzdG9tGBYgASgLMi'
    'oub3Blbm94aWRlLmFwaS52MS5Ob3RpZmljYXRpb25DdXN0b21Db25maWdIAFIGY3VzdG9tEkQK'
    'BGxhcmsYFyABKAsyLi5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbldlYmhvb2tVcmxDb2'
    '5maWdIAFIEbGFyaxJKCghwdXNob3ZlchgYIAEoCzIsLm9wZW5veGlkZS5hcGkudjEuTm90aWZp'
    'Y2F0aW9uUHVzaG92ZXJDb25maWdIAFIIcHVzaG92ZXISRgoFdGVhbXMYGSABKAsyLi5vcGVub3'
    'hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbldlYmhvb2tVcmxDb25maWdIAFIFdGVhbXNCCgoIcHJv'
    'dmlkZXJCEgoQX29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use replaceNotificationResponseDescriptor instead')
const ReplaceNotificationResponse$json = {
  '1': 'ReplaceNotificationResponse',
  '2': [
    {
      '1': 'notification',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Notification',
      '10': 'notification'
    },
  ],
};

/// Descriptor for `ReplaceNotificationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List replaceNotificationResponseDescriptor =
    $convert.base64Decode(
        'ChtSZXBsYWNlTm90aWZpY2F0aW9uUmVzcG9uc2USQgoMbm90aWZpY2F0aW9uGAEgASgLMh4ub3'
        'Blbm94aWRlLmFwaS52MS5Ob3RpZmljYXRpb25SDG5vdGlmaWNhdGlvbg==');

@$core.Deprecated('Use deleteNotificationRequestDescriptor instead')
const DeleteNotificationRequest$json = {
  '1': 'DeleteNotificationRequest',
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

/// Descriptor for `DeleteNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteNotificationRequestDescriptor = $convert.base64Decode(
    'ChlEZWxldGVOb3RpZmljYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg'
    '9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQ'
    'X29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use testNotificationRequestDescriptor instead')
const TestNotificationRequest$json = {
  '1': 'TestNotificationRequest',
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

/// Descriptor for `TestNotificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testNotificationRequestDescriptor = $convert.base64Decode(
    'ChdUZXN0Tm90aWZpY2F0aW9uUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3'
    'JnYW5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9v'
    'cmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use testNotificationResponseDescriptor instead')
const TestNotificationResponse$json = {
  '1': 'TestNotificationResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestNotificationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testNotificationResponseDescriptor =
    $convert.base64Decode(
        'ChhUZXN0Tm90aWZpY2F0aW9uUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2VzcxIYCg'
        'dtZXNzYWdlGAIgASgJUgdtZXNzYWdl');

@$core
    .Deprecated('Use listNotificationDeliveryHistoryRequestDescriptor instead')
const ListNotificationDeliveryHistoryRequest$json = {
  '1': 'ListNotificationDeliveryHistoryRequest',
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

/// Descriptor for `ListNotificationDeliveryHistoryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationDeliveryHistoryRequestDescriptor =
    $convert.base64Decode(
        'CiZMaXN0Tm90aWZpY2F0aW9uRGVsaXZlcnlIaXN0b3J5UmVxdWVzdBI1Cg9vcmdhbml6YXRpb2'
        '5faWQYASABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQX29yZ2FuaXphdGlv'
        'bl9pZA==');

@$core
    .Deprecated('Use listNotificationDeliveryHistoryResponseDescriptor instead')
const ListNotificationDeliveryHistoryResponse$json = {
  '1': 'ListNotificationDeliveryHistoryResponse',
  '2': [
    {
      '1': 'attempts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationDeliveryAttempt',
      '10': 'attempts'
    },
  ],
};

/// Descriptor for `ListNotificationDeliveryHistoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationDeliveryHistoryResponseDescriptor =
    $convert.base64Decode(
        'CidMaXN0Tm90aWZpY2F0aW9uRGVsaXZlcnlIaXN0b3J5UmVzcG9uc2USSQoIYXR0ZW1wdHMYAS'
        'ADKAsyLS5vcGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvbkRlbGl2ZXJ5QXR0ZW1wdFIIYXR0'
        'ZW1wdHM=');

@$core.Deprecated('Use createNotificationBindingRequestDescriptor instead')
const CreateNotificationBindingRequest$json = {
  '1': 'CreateNotificationBindingRequest',
  '2': [
    {
      '1': 'notification_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'notificationId'
    },
    {
      '1': 'resource_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.NotificationResourceType',
      '8': {},
      '10': 'resourceType'
    },
    {'1': 'resource_id', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'resourceId'},
    {
      '1': 'organization_id',
      '3': 4,
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

/// Descriptor for `CreateNotificationBindingRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createNotificationBindingRequestDescriptor = $convert.base64Decode(
    'CiBDcmVhdGVOb3RpZmljYXRpb25CaW5kaW5nUmVxdWVzdBIwCg9ub3RpZmljYXRpb25faWQYAS'
    'ABKANCB7pIBCICIABSDm5vdGlmaWNhdGlvbklkElsKDXJlc291cmNlX3R5cGUYAiABKA4yKi5v'
    'cGVub3hpZGUuYXBpLnYxLk5vdGlmaWNhdGlvblJlc291cmNlVHlwZUIKukgHggEEEAEgAFIMcm'
    'Vzb3VyY2VUeXBlEigKC3Jlc291cmNlX2lkGAMgASgDQge6SAQiAiAAUgpyZXNvdXJjZUlkEjUK'
    'D29yZ2FuaXphdGlvbl9pZBgEIAEoA0IHukgEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISCh'
    'Bfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use createNotificationBindingResponseDescriptor instead')
const CreateNotificationBindingResponse$json = {
  '1': 'CreateNotificationBindingResponse',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
  ],
};

/// Descriptor for `CreateNotificationBindingResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createNotificationBindingResponseDescriptor =
    $convert.base64Decode(
        'CiFDcmVhdGVOb3RpZmljYXRpb25CaW5kaW5nUmVzcG9uc2USDgoCaWQYASABKANSAmlk');

@$core.Deprecated('Use listNotificationBindingsRequestDescriptor instead')
const ListNotificationBindingsRequest$json = {
  '1': 'ListNotificationBindingsRequest',
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

/// Descriptor for `ListNotificationBindingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationBindingsRequestDescriptor =
    $convert.base64Decode(
        'Ch9MaXN0Tm90aWZpY2F0aW9uQmluZGluZ3NSZXF1ZXN0EjUKD29yZ2FuaXphdGlvbl9pZBgBIA'
        'EoA0IHukgEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use listNotificationBindingsResponseDescriptor instead')
const ListNotificationBindingsResponse$json = {
  '1': 'ListNotificationBindingsResponse',
  '2': [
    {
      '1': 'bindings',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.NotificationBinding',
      '10': 'bindings'
    },
  ],
};

/// Descriptor for `ListNotificationBindingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationBindingsResponseDescriptor =
    $convert.base64Decode(
        'CiBMaXN0Tm90aWZpY2F0aW9uQmluZGluZ3NSZXNwb25zZRJBCghiaW5kaW5ncxgBIAMoCzIlLm'
        '9wZW5veGlkZS5hcGkudjEuTm90aWZpY2F0aW9uQmluZGluZ1IIYmluZGluZ3M=');

@$core.Deprecated('Use deleteNotificationBindingRequestDescriptor instead')
const DeleteNotificationBindingRequest$json = {
  '1': 'DeleteNotificationBindingRequest',
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

/// Descriptor for `DeleteNotificationBindingRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteNotificationBindingRequestDescriptor =
    $convert.base64Decode(
        'CiBEZWxldGVOb3RpZmljYXRpb25CaW5kaW5nUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAF'
        'ICaWQSNQoPb3JnYW5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklk'
        'iAEBQhIKEF9vcmdhbml6YXRpb25faWQ=');

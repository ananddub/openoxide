// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/settings.proto.

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

@$core.Deprecated('Use settingsCertificateTypeDescriptor instead')
const SettingsCertificateType$json = {
  '1': 'SettingsCertificateType',
  '2': [
    {'1': 'SETTINGS_CERTIFICATE_TYPE_UNSPECIFIED', '2': 0},
    {'1': 'SETTINGS_CERTIFICATE_TYPE_NONE', '2': 1},
    {'1': 'SETTINGS_CERTIFICATE_TYPE_LETSENCRYPT', '2': 2},
    {'1': 'SETTINGS_CERTIFICATE_TYPE_CUSTOM', '2': 3},
  ],
};

/// Descriptor for `SettingsCertificateType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List settingsCertificateTypeDescriptor = $convert.base64Decode(
    'ChdTZXR0aW5nc0NlcnRpZmljYXRlVHlwZRIpCiVTRVRUSU5HU19DRVJUSUZJQ0FURV9UWVBFX1'
    'VOU1BFQ0lGSUVEEAASIgoeU0VUVElOR1NfQ0VSVElGSUNBVEVfVFlQRV9OT05FEAESKQolU0VU'
    'VElOR1NfQ0VSVElGSUNBVEVfVFlQRV9MRVRTRU5DUllQVBACEiQKIFNFVFRJTkdTX0NFUlRJRk'
    'lDQVRFX1RZUEVfQ1VTVE9NEAM=');

@$core.Deprecated('Use settingsDescriptor instead')
const Settings$json = {
  '1': 'Settings',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {
      '1': 'server_ip',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'serverIp',
      '17': true
    },
    {
      '1': 'certificate_type',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SettingsCertificateType',
      '10': 'certificateType'
    },
    {
      '1': 'custom_cert_resolver',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'https', '3': 5, '4': 1, '5': 8, '10': 'https'},
    {'1': 'host', '3': 6, '4': 1, '5': 9, '9': 2, '10': 'host', '17': true},
    {
      '1': 'lets_encrypt_email',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'letsEncryptEmail',
      '17': true
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 8,
      '4': 1,
      '5': 8,
      '10': 'enableDockerCleanup'
    },
    {
      '1': 'log_cleanup_cron',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'logCleanupCron',
      '17': true
    },
    {'1': 'metrics_config', '3': 10, '4': 1, '5': 9, '10': 'metricsConfig'},
    {
      '1': 'panel_backup_cron',
      '3': 11,
      '4': 1,
      '5': 9,
      '10': 'panelBackupCron'
    },
    {
      '1': 'log_retention_days',
      '3': 12,
      '4': 1,
      '5': 3,
      '10': 'logRetentionDays'
    },
    {
      '1': 'panel_backup_enabled',
      '3': 13,
      '4': 1,
      '5': 8,
      '10': 'panelBackupEnabled'
    },
    {
      '1': 'log_cleanup_enabled',
      '3': 14,
      '4': 1,
      '5': 8,
      '10': 'logCleanupEnabled'
    },
    {'1': 'updated_at', '3': 15, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_server_ip'},
    {'1': '_custom_cert_resolver'},
    {'1': '_host'},
    {'1': '_lets_encrypt_email'},
    {'1': '_log_cleanup_cron'},
  ],
};

/// Descriptor for `Settings`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsDescriptor = $convert.base64Decode(
    'CghTZXR0aW5ncxIOCgJpZBgBIAEoA1ICaWQSIAoJc2VydmVyX2lwGAIgASgJSABSCHNlcnZlck'
    'lwiAEBElQKEGNlcnRpZmljYXRlX3R5cGUYAyABKA4yKS5vcGVub3hpZGUuYXBpLnYxLlNldHRp'
    'bmdzQ2VydGlmaWNhdGVUeXBlUg9jZXJ0aWZpY2F0ZVR5cGUSNQoUY3VzdG9tX2NlcnRfcmVzb2'
    'x2ZXIYBCABKAlIAVISY3VzdG9tQ2VydFJlc29sdmVyiAEBEhQKBWh0dHBzGAUgASgIUgVodHRw'
    'cxIXCgRob3N0GAYgASgJSAJSBGhvc3SIAQESMQoSbGV0c19lbmNyeXB0X2VtYWlsGAcgASgJSA'
    'NSEGxldHNFbmNyeXB0RW1haWyIAQESMgoVZW5hYmxlX2RvY2tlcl9jbGVhbnVwGAggASgIUhNl'
    'bmFibGVEb2NrZXJDbGVhbnVwEi0KEGxvZ19jbGVhbnVwX2Nyb24YCSABKAlIBFIObG9nQ2xlYW'
    '51cENyb26IAQESJQoObWV0cmljc19jb25maWcYCiABKAlSDW1ldHJpY3NDb25maWcSKgoRcGFu'
    'ZWxfYmFja3VwX2Nyb24YCyABKAlSD3BhbmVsQmFja3VwQ3JvbhIsChJsb2dfcmV0ZW50aW9uX2'
    'RheXMYDCABKANSEGxvZ1JldGVudGlvbkRheXMSMAoUcGFuZWxfYmFja3VwX2VuYWJsZWQYDSAB'
    'KAhSEnBhbmVsQmFja3VwRW5hYmxlZBIuChNsb2dfY2xlYW51cF9lbmFibGVkGA4gASgIUhFsb2'
    'dDbGVhbnVwRW5hYmxlZBIdCgp1cGRhdGVkX2F0GA8gASgDUgl1cGRhdGVkQXRCDAoKX3NlcnZl'
    'cl9pcEIXChVfY3VzdG9tX2NlcnRfcmVzb2x2ZXJCBwoFX2hvc3RCFQoTX2xldHNfZW5jcnlwdF'
    '9lbWFpbEITChFfbG9nX2NsZWFudXBfY3Jvbg==');

@$core.Deprecated('Use settingsGetSettingsRequestDescriptor instead')
const SettingsGetSettingsRequest$json = {
  '1': 'SettingsGetSettingsRequest',
};

/// Descriptor for `SettingsGetSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsGetSettingsRequestDescriptor =
    $convert.base64Decode('ChpTZXR0aW5nc0dldFNldHRpbmdzUmVxdWVzdA==');

@$core.Deprecated('Use settingsGetSettingsResponseDescriptor instead')
const SettingsGetSettingsResponse$json = {
  '1': 'SettingsGetSettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Settings',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `SettingsGetSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsGetSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChtTZXR0aW5nc0dldFNldHRpbmdzUmVzcG9uc2USNgoIc2V0dGluZ3MYASABKAsyGi5vcGVub3'
        'hpZGUuYXBpLnYxLlNldHRpbmdzUghzZXR0aW5ncw==');

@$core.Deprecated('Use settingsUpdateSettingsRequestDescriptor instead')
const SettingsUpdateSettingsRequest$json = {
  '1': 'SettingsUpdateSettingsRequest',
  '2': [
    {
      '1': 'server_ip',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'serverIp',
      '17': true
    },
    {
      '1': 'certificate_type',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.SettingsCertificateType',
      '8': {},
      '9': 1,
      '10': 'certificateType',
      '17': true
    },
    {
      '1': 'custom_cert_resolver',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'customCertResolver',
      '17': true
    },
    {'1': 'https', '3': 4, '4': 1, '5': 8, '9': 3, '10': 'https', '17': true},
    {'1': 'host', '3': 5, '4': 1, '5': 9, '9': 4, '10': 'host', '17': true},
    {
      '1': 'lets_encrypt_email',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'letsEncryptEmail',
      '17': true
    },
    {
      '1': 'enable_docker_cleanup',
      '3': 7,
      '4': 1,
      '5': 8,
      '9': 6,
      '10': 'enableDockerCleanup',
      '17': true
    },
    {
      '1': 'log_cleanup_cron',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'logCleanupCron',
      '17': true
    },
    {
      '1': 'metrics_config',
      '3': 9,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 8,
      '10': 'metricsConfig',
      '17': true
    },
    {
      '1': 'panel_backup_cron',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'panelBackupCron',
      '17': true
    },
    {
      '1': 'log_retention_days',
      '3': 11,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 10,
      '10': 'logRetentionDays',
      '17': true
    },
    {
      '1': 'panel_backup_enabled',
      '3': 12,
      '4': 1,
      '5': 8,
      '9': 11,
      '10': 'panelBackupEnabled',
      '17': true
    },
    {
      '1': 'log_cleanup_enabled',
      '3': 13,
      '4': 1,
      '5': 8,
      '9': 12,
      '10': 'logCleanupEnabled',
      '17': true
    },
  ],
  '8': [
    {'1': '_server_ip'},
    {'1': '_certificate_type'},
    {'1': '_custom_cert_resolver'},
    {'1': '_https'},
    {'1': '_host'},
    {'1': '_lets_encrypt_email'},
    {'1': '_enable_docker_cleanup'},
    {'1': '_log_cleanup_cron'},
    {'1': '_metrics_config'},
    {'1': '_panel_backup_cron'},
    {'1': '_log_retention_days'},
    {'1': '_panel_backup_enabled'},
    {'1': '_log_cleanup_enabled'},
  ],
};

/// Descriptor for `SettingsUpdateSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsUpdateSettingsRequestDescriptor = $convert.base64Decode(
    'Ch1TZXR0aW5nc1VwZGF0ZVNldHRpbmdzUmVxdWVzdBIgCglzZXJ2ZXJfaXAYASABKAlIAFIIc2'
    'VydmVySXCIAQESZQoQY2VydGlmaWNhdGVfdHlwZRgCIAEoDjIpLm9wZW5veGlkZS5hcGkudjEu'
    'U2V0dGluZ3NDZXJ0aWZpY2F0ZVR5cGVCCrpIB4IBBBABIABIAVIPY2VydGlmaWNhdGVUeXBliA'
    'EBEjUKFGN1c3RvbV9jZXJ0X3Jlc29sdmVyGAMgASgJSAJSEmN1c3RvbUNlcnRSZXNvbHZlcogB'
    'ARIZCgVodHRwcxgEIAEoCEgDUgVodHRwc4gBARIXCgRob3N0GAUgASgJSARSBGhvc3SIAQESMQ'
    'oSbGV0c19lbmNyeXB0X2VtYWlsGAYgASgJSAVSEGxldHNFbmNyeXB0RW1haWyIAQESNwoVZW5h'
    'YmxlX2RvY2tlcl9jbGVhbnVwGAcgASgISAZSE2VuYWJsZURvY2tlckNsZWFudXCIAQESLQoQbG'
    '9nX2NsZWFudXBfY3JvbhgIIAEoCUgHUg5sb2dDbGVhbnVwQ3JvbogBARI1Cg5tZXRyaWNzX2Nv'
    'bmZpZxgJIAEoCUIJukgGcgQYgIBASAhSDW1ldHJpY3NDb25maWeIAQESLwoRcGFuZWxfYmFja3'
    'VwX2Nyb24YCiABKAlICVIPcGFuZWxCYWNrdXBDcm9uiAEBEjoKEmxvZ19yZXRlbnRpb25fZGF5'
    'cxgLIAEoA0IHukgEIgIoAEgKUhBsb2dSZXRlbnRpb25EYXlziAEBEjUKFHBhbmVsX2JhY2t1cF'
    '9lbmFibGVkGAwgASgISAtSEnBhbmVsQmFja3VwRW5hYmxlZIgBARIzChNsb2dfY2xlYW51cF9l'
    'bmFibGVkGA0gASgISAxSEWxvZ0NsZWFudXBFbmFibGVkiAEBQgwKCl9zZXJ2ZXJfaXBCEwoRX2'
    'NlcnRpZmljYXRlX3R5cGVCFwoVX2N1c3RvbV9jZXJ0X3Jlc29sdmVyQggKBl9odHRwc0IHCgVf'
    'aG9zdEIVChNfbGV0c19lbmNyeXB0X2VtYWlsQhgKFl9lbmFibGVfZG9ja2VyX2NsZWFudXBCEw'
    'oRX2xvZ19jbGVhbnVwX2Nyb25CEQoPX21ldHJpY3NfY29uZmlnQhQKEl9wYW5lbF9iYWNrdXBf'
    'Y3JvbkIVChNfbG9nX3JldGVudGlvbl9kYXlzQhcKFV9wYW5lbF9iYWNrdXBfZW5hYmxlZEIWCh'
    'RfbG9nX2NsZWFudXBfZW5hYmxlZA==');

@$core.Deprecated('Use settingsUpdateSettingsResponseDescriptor instead')
const SettingsUpdateSettingsResponse$json = {
  '1': 'SettingsUpdateSettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.Settings',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `SettingsUpdateSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsUpdateSettingsResponseDescriptor =
    $convert.base64Decode(
        'Ch5TZXR0aW5nc1VwZGF0ZVNldHRpbmdzUmVzcG9uc2USNgoIc2V0dGluZ3MYASABKAsyGi5vcG'
        'Vub3hpZGUuYXBpLnYxLlNldHRpbmdzUghzZXR0aW5ncw==');

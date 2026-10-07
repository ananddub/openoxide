// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/alert_rule.proto.

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

@$core.Deprecated('Use alertTargetKindDescriptor instead')
const AlertTargetKind$json = {
  '1': 'AlertTargetKind',
  '2': [
    {'1': 'ALERT_TARGET_KIND_UNSPECIFIED', '2': 0},
    {'1': 'ALERT_TARGET_KIND_SERVER', '2': 1},
    {'1': 'ALERT_TARGET_KIND_APPLICATION', '2': 2},
    {'1': 'ALERT_TARGET_KIND_DATABASE', '2': 3},
    {'1': 'ALERT_TARGET_KIND_COMPOSE', '2': 4},
  ],
};

/// Descriptor for `AlertTargetKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List alertTargetKindDescriptor = $convert.base64Decode(
    'Cg9BbGVydFRhcmdldEtpbmQSIQodQUxFUlRfVEFSR0VUX0tJTkRfVU5TUEVDSUZJRUQQABIcCh'
    'hBTEVSVF9UQVJHRVRfS0lORF9TRVJWRVIQARIhCh1BTEVSVF9UQVJHRVRfS0lORF9BUFBMSUNB'
    'VElPThACEh4KGkFMRVJUX1RBUkdFVF9LSU5EX0RBVEFCQVNFEAMSHQoZQUxFUlRfVEFSR0VUX0'
    'tJTkRfQ09NUE9TRRAE');

@$core.Deprecated('Use alertMetricKindDescriptor instead')
const AlertMetricKind$json = {
  '1': 'AlertMetricKind',
  '2': [
    {'1': 'ALERT_METRIC_KIND_UNSPECIFIED', '2': 0},
    {'1': 'ALERT_METRIC_KIND_CPU', '2': 1},
    {'1': 'ALERT_METRIC_KIND_MEMORY', '2': 2},
    {'1': 'ALERT_METRIC_KIND_DISK', '2': 3},
  ],
};

/// Descriptor for `AlertMetricKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List alertMetricKindDescriptor = $convert.base64Decode(
    'Cg9BbGVydE1ldHJpY0tpbmQSIQodQUxFUlRfTUVUUklDX0tJTkRfVU5TUEVDSUZJRUQQABIZCh'
    'VBTEVSVF9NRVRSSUNfS0lORF9DUFUQARIcChhBTEVSVF9NRVRSSUNfS0lORF9NRU1PUlkQAhIa'
    'ChZBTEVSVF9NRVRSSUNfS0lORF9ESVNLEAM=');

@$core.Deprecated('Use alertOperatorDescriptor instead')
const AlertOperator$json = {
  '1': 'AlertOperator',
  '2': [
    {'1': 'ALERT_OPERATOR_UNSPECIFIED', '2': 0},
    {'1': 'ALERT_OPERATOR_GREATER_THAN', '2': 1},
    {'1': 'ALERT_OPERATOR_GREATER_OR_EQUAL', '2': 2},
    {'1': 'ALERT_OPERATOR_LESS_THAN', '2': 3},
    {'1': 'ALERT_OPERATOR_LESS_OR_EQUAL', '2': 4},
    {'1': 'ALERT_OPERATOR_EQUAL', '2': 5},
  ],
};

/// Descriptor for `AlertOperator`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List alertOperatorDescriptor = $convert.base64Decode(
    'Cg1BbGVydE9wZXJhdG9yEh4KGkFMRVJUX09QRVJBVE9SX1VOU1BFQ0lGSUVEEAASHwobQUxFUl'
    'RfT1BFUkFUT1JfR1JFQVRFUl9USEFOEAESIwofQUxFUlRfT1BFUkFUT1JfR1JFQVRFUl9PUl9F'
    'UVVBTBACEhwKGEFMRVJUX09QRVJBVE9SX0xFU1NfVEhBThADEiAKHEFMRVJUX09QRVJBVE9SX0'
    'xFU1NfT1JfRVFVQUwQBBIYChRBTEVSVF9PUEVSQVRPUl9FUVVBTBAF');

@$core.Deprecated('Use alertRuleDescriptor instead')
const AlertRule$json = {
  '1': 'AlertRule',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'target_kind',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertTargetKind',
      '10': 'targetKind'
    },
    {'1': 'target_id', '3': 4, '4': 1, '5': 3, '10': 'targetId'},
    {
      '1': 'metric',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertMetricKind',
      '10': 'metric'
    },
    {
      '1': 'operator',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertOperator',
      '10': 'operator'
    },
    {'1': 'threshold', '3': 7, '4': 1, '5': 1, '10': 'threshold'},
    {'1': 'duration_seconds', '3': 8, '4': 1, '5': 3, '10': 'durationSeconds'},
    {'1': 'enabled', '3': 9, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'organization_id', '3': 10, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 11, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 12, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `AlertRule`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List alertRuleDescriptor = $convert.base64Decode(
    'CglBbGVydFJ1bGUSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSQgoLdGFyZ2'
    'V0X2tpbmQYAyABKA4yIS5vcGVub3hpZGUuYXBpLnYxLkFsZXJ0VGFyZ2V0S2luZFIKdGFyZ2V0'
    'S2luZBIbCgl0YXJnZXRfaWQYBCABKANSCHRhcmdldElkEjkKBm1ldHJpYxgFIAEoDjIhLm9wZW'
    '5veGlkZS5hcGkudjEuQWxlcnRNZXRyaWNLaW5kUgZtZXRyaWMSOwoIb3BlcmF0b3IYBiABKA4y'
    'Hy5vcGVub3hpZGUuYXBpLnYxLkFsZXJ0T3BlcmF0b3JSCG9wZXJhdG9yEhwKCXRocmVzaG9sZB'
    'gHIAEoAVIJdGhyZXNob2xkEikKEGR1cmF0aW9uX3NlY29uZHMYCCABKANSD2R1cmF0aW9uU2Vj'
    'b25kcxIYCgdlbmFibGVkGAkgASgIUgdlbmFibGVkEicKD29yZ2FuaXphdGlvbl9pZBgKIAEoA1'
    'IOb3JnYW5pemF0aW9uSWQSHQoKY3JlYXRlZF9hdBgLIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0'
    'ZWRfYXQYDCABKANSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use listAlertRulesRequestDescriptor instead')
const ListAlertRulesRequest$json = {
  '1': 'ListAlertRulesRequest',
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

/// Descriptor for `ListAlertRulesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAlertRulesRequestDescriptor = $convert.base64Decode(
    'ChVMaXN0QWxlcnRSdWxlc1JlcXVlc3QSNQoPb3JnYW5pemF0aW9uX2lkGAEgASgDQge6SAQiAi'
    'AASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use listAlertRulesResponseDescriptor instead')
const ListAlertRulesResponse$json = {
  '1': 'ListAlertRulesResponse',
  '2': [
    {
      '1': 'rules',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AlertRule',
      '10': 'rules'
    },
  ],
};

/// Descriptor for `ListAlertRulesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAlertRulesResponseDescriptor =
    $convert.base64Decode(
        'ChZMaXN0QWxlcnRSdWxlc1Jlc3BvbnNlEjEKBXJ1bGVzGAEgAygLMhsub3Blbm94aWRlLmFwaS'
        '52MS5BbGVydFJ1bGVSBXJ1bGVz');

@$core.Deprecated('Use getAlertRuleRequestDescriptor instead')
const GetAlertRuleRequest$json = {
  '1': 'GetAlertRuleRequest',
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

/// Descriptor for `GetAlertRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAlertRuleRequestDescriptor = $convert.base64Decode(
    'ChNHZXRBbGVydFJ1bGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg9vcmdhbm'
    'l6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQX29yZ2Fu'
    'aXphdGlvbl9pZA==');

@$core.Deprecated('Use getAlertRuleResponseDescriptor instead')
const GetAlertRuleResponse$json = {
  '1': 'GetAlertRuleResponse',
  '2': [
    {
      '1': 'rule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AlertRule',
      '10': 'rule'
    },
  ],
};

/// Descriptor for `GetAlertRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAlertRuleResponseDescriptor = $convert.base64Decode(
    'ChRHZXRBbGVydFJ1bGVSZXNwb25zZRIvCgRydWxlGAEgASgLMhsub3Blbm94aWRlLmFwaS52MS'
    '5BbGVydFJ1bGVSBHJ1bGU=');

@$core.Deprecated('Use createAlertRuleRequestDescriptor instead')
const CreateAlertRuleRequest$json = {
  '1': 'CreateAlertRuleRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'target_kind',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertTargetKind',
      '8': {},
      '10': 'targetKind'
    },
    {'1': 'target_id', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'targetId'},
    {
      '1': 'metric',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertMetricKind',
      '8': {},
      '10': 'metric'
    },
    {
      '1': 'operator',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertOperator',
      '8': {},
      '10': 'operator'
    },
    {'1': 'threshold', '3': 6, '4': 1, '5': 1, '10': 'threshold'},
    {
      '1': 'duration_seconds',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'durationSeconds'
    },
    {'1': 'enabled', '3': 8, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'organization_id',
      '3': 9,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'organizationId'
    },
  ],
};

/// Descriptor for `CreateAlertRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createAlertRuleRequestDescriptor = $convert.base64Decode(
    'ChZDcmVhdGVBbGVydFJ1bGVSZXF1ZXN0Eh4KBG5hbWUYASABKAlCCrpIB3IFEAEY/wFSBG5hbW'
    'USTgoLdGFyZ2V0X2tpbmQYAiABKA4yIS5vcGVub3hpZGUuYXBpLnYxLkFsZXJ0VGFyZ2V0S2lu'
    'ZEIKukgHggEEEAEgAFIKdGFyZ2V0S2luZBIkCgl0YXJnZXRfaWQYAyABKANCB7pIBCICIABSCH'
    'RhcmdldElkEkUKBm1ldHJpYxgEIAEoDjIhLm9wZW5veGlkZS5hcGkudjEuQWxlcnRNZXRyaWNL'
    'aW5kQgq6SAeCAQQQASAAUgZtZXRyaWMSRwoIb3BlcmF0b3IYBSABKA4yHy5vcGVub3hpZGUuYX'
    'BpLnYxLkFsZXJ0T3BlcmF0b3JCCrpIB4IBBBABIABSCG9wZXJhdG9yEhwKCXRocmVzaG9sZBgG'
    'IAEoAVIJdGhyZXNob2xkEjIKEGR1cmF0aW9uX3NlY29uZHMYByABKANCB7pIBCICIABSD2R1cm'
    'F0aW9uU2Vjb25kcxIYCgdlbmFibGVkGAggASgIUgdlbmFibGVkEjAKD29yZ2FuaXphdGlvbl9p'
    'ZBgJIAEoA0IHukgEIgIgAFIOb3JnYW5pemF0aW9uSWQ=');

@$core.Deprecated('Use createAlertRuleResponseDescriptor instead')
const CreateAlertRuleResponse$json = {
  '1': 'CreateAlertRuleResponse',
  '2': [
    {
      '1': 'rule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AlertRule',
      '10': 'rule'
    },
  ],
};

/// Descriptor for `CreateAlertRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createAlertRuleResponseDescriptor =
    $convert.base64Decode(
        'ChdDcmVhdGVBbGVydFJ1bGVSZXNwb25zZRIvCgRydWxlGAEgASgLMhsub3Blbm94aWRlLmFwaS'
        '52MS5BbGVydFJ1bGVSBHJ1bGU=');

@$core.Deprecated('Use updateAlertRuleRequestDescriptor instead')
const UpdateAlertRuleRequest$json = {
  '1': 'UpdateAlertRuleRequest',
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
    {'1': 'name', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {
      '1': 'target_kind',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertTargetKind',
      '8': {},
      '10': 'targetKind'
    },
    {'1': 'target_id', '3': 5, '4': 1, '5': 3, '8': {}, '10': 'targetId'},
    {
      '1': 'metric',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertMetricKind',
      '8': {},
      '10': 'metric'
    },
    {
      '1': 'operator',
      '3': 7,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertOperator',
      '8': {},
      '10': 'operator'
    },
    {'1': 'threshold', '3': 8, '4': 1, '5': 1, '10': 'threshold'},
    {
      '1': 'duration_seconds',
      '3': 9,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'durationSeconds'
    },
    {'1': 'enabled', '3': 10, '4': 1, '5': 8, '10': 'enabled'},
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `UpdateAlertRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateAlertRuleRequestDescriptor = $convert.base64Decode(
    'ChZVcGRhdGVBbGVydFJ1bGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg9vcm'
    'dhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESHgoEbmFt'
    'ZRgDIAEoCUIKukgHcgUQARj/AVIEbmFtZRJOCgt0YXJnZXRfa2luZBgEIAEoDjIhLm9wZW5veG'
    'lkZS5hcGkudjEuQWxlcnRUYXJnZXRLaW5kQgq6SAeCAQQQASAAUgp0YXJnZXRLaW5kEiQKCXRh'
    'cmdldF9pZBgFIAEoA0IHukgEIgIgAFIIdGFyZ2V0SWQSRQoGbWV0cmljGAYgASgOMiEub3Blbm'
    '94aWRlLmFwaS52MS5BbGVydE1ldHJpY0tpbmRCCrpIB4IBBBABIABSBm1ldHJpYxJHCghvcGVy'
    'YXRvchgHIAEoDjIfLm9wZW5veGlkZS5hcGkudjEuQWxlcnRPcGVyYXRvckIKukgHggEEEAEgAF'
    'IIb3BlcmF0b3ISHAoJdGhyZXNob2xkGAggASgBUgl0aHJlc2hvbGQSMgoQZHVyYXRpb25fc2Vj'
    'b25kcxgJIAEoA0IHukgEIgIgAFIPZHVyYXRpb25TZWNvbmRzEhgKB2VuYWJsZWQYCiABKAhSB2'
    'VuYWJsZWRCEgoQX29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use updateAlertRuleResponseDescriptor instead')
const UpdateAlertRuleResponse$json = {
  '1': 'UpdateAlertRuleResponse',
  '2': [
    {
      '1': 'rule',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AlertRule',
      '10': 'rule'
    },
  ],
};

/// Descriptor for `UpdateAlertRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateAlertRuleResponseDescriptor =
    $convert.base64Decode(
        'ChdVcGRhdGVBbGVydFJ1bGVSZXNwb25zZRIvCgRydWxlGAEgASgLMhsub3Blbm94aWRlLmFwaS'
        '52MS5BbGVydFJ1bGVSBHJ1bGU=');

@$core.Deprecated('Use deleteAlertRuleRequestDescriptor instead')
const DeleteAlertRuleRequest$json = {
  '1': 'DeleteAlertRuleRequest',
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

/// Descriptor for `DeleteAlertRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteAlertRuleRequestDescriptor = $convert.base64Decode(
    'ChZEZWxldGVBbGVydFJ1bGVSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg9vcm'
    'dhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQX29y'
    'Z2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use testAlertRuleRequestDescriptor instead')
const TestAlertRuleRequest$json = {
  '1': 'TestAlertRuleRequest',
  '2': [
    {
      '1': 'notification_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'notificationId'
    },
  ],
};

/// Descriptor for `TestAlertRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testAlertRuleRequestDescriptor = $convert.base64Decode(
    'ChRUZXN0QWxlcnRSdWxlUmVxdWVzdBIwCg9ub3RpZmljYXRpb25faWQYASABKANCB7pIBCICIA'
    'BSDm5vdGlmaWNhdGlvbklk');

@$core.Deprecated('Use testAlertRuleResponseDescriptor instead')
const TestAlertRuleResponse$json = {
  '1': 'TestAlertRuleResponse',
  '2': [
    {'1': 'delivered', '3': 1, '4': 1, '5': 8, '10': 'delivered'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `TestAlertRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List testAlertRuleResponseDescriptor = $convert.base64Decode(
    'ChVUZXN0QWxlcnRSdWxlUmVzcG9uc2USHAoJZGVsaXZlcmVkGAEgASgIUglkZWxpdmVyZWQSGA'
    'oHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');

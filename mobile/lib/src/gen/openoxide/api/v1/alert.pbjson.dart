// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/alert.proto.

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

@$core.Deprecated('Use alertEventStateDescriptor instead')
const AlertEventState$json = {
  '1': 'AlertEventState',
  '2': [
    {'1': 'ALERT_EVENT_STATE_UNSPECIFIED', '2': 0},
    {'1': 'ALERT_EVENT_STATE_FIRING', '2': 1},
    {'1': 'ALERT_EVENT_STATE_RESOLVED', '2': 2},
    {'1': 'ALERT_EVENT_STATE_NO_DATA', '2': 3},
  ],
};

/// Descriptor for `AlertEventState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List alertEventStateDescriptor = $convert.base64Decode(
    'Cg9BbGVydEV2ZW50U3RhdGUSIQodQUxFUlRfRVZFTlRfU1RBVEVfVU5TUEVDSUZJRUQQABIcCh'
    'hBTEVSVF9FVkVOVF9TVEFURV9GSVJJTkcQARIeChpBTEVSVF9FVkVOVF9TVEFURV9SRVNPTFZF'
    'RBACEh0KGUFMRVJUX0VWRU5UX1NUQVRFX05PX0RBVEEQAw==');

@$core.Deprecated('Use alertEventDescriptor instead')
const AlertEvent$json = {
  '1': 'AlertEvent',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'alert_rule_id', '3': 2, '4': 1, '5': 3, '10': 'alertRuleId'},
    {'1': 'organization_id', '3': 3, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'target_key', '3': 4, '4': 1, '5': 9, '10': 'targetKey'},
    {
      '1': 'state',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AlertEventState',
      '10': 'state'
    },
    {'1': 'value', '3': 6, '4': 1, '5': 1, '9': 0, '10': 'value', '17': true},
    {
      '1': 'threshold',
      '3': 7,
      '4': 1,
      '5': 1,
      '9': 1,
      '10': 'threshold',
      '17': true
    },
    {'1': 'message', '3': 8, '4': 1, '5': 9, '10': 'message'},
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {
      '1': 'acknowledged_at',
      '3': 10,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'acknowledgedAt',
      '17': true
    },
    {
      '1': 'acknowledged_by',
      '3': 11,
      '4': 1,
      '5': 3,
      '9': 3,
      '10': 'acknowledgedBy',
      '17': true
    },
    {
      '1': 'silenced_until',
      '3': 12,
      '4': 1,
      '5': 3,
      '9': 4,
      '10': 'silencedUntil',
      '17': true
    },
    {
      '1': 'resolved_at',
      '3': 13,
      '4': 1,
      '5': 3,
      '9': 5,
      '10': 'resolvedAt',
      '17': true
    },
    {
      '1': 'notification_correlation_id',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'notificationCorrelationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_value'},
    {'1': '_threshold'},
    {'1': '_acknowledged_at'},
    {'1': '_acknowledged_by'},
    {'1': '_silenced_until'},
    {'1': '_resolved_at'},
    {'1': '_notification_correlation_id'},
  ],
};

/// Descriptor for `AlertEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List alertEventDescriptor = $convert.base64Decode(
    'CgpBbGVydEV2ZW50Eg4KAmlkGAEgASgDUgJpZBIiCg1hbGVydF9ydWxlX2lkGAIgASgDUgthbG'
    'VydFJ1bGVJZBInCg9vcmdhbml6YXRpb25faWQYAyABKANSDm9yZ2FuaXphdGlvbklkEh0KCnRh'
    'cmdldF9rZXkYBCABKAlSCXRhcmdldEtleRI3CgVzdGF0ZRgFIAEoDjIhLm9wZW5veGlkZS5hcG'
    'kudjEuQWxlcnRFdmVudFN0YXRlUgVzdGF0ZRIZCgV2YWx1ZRgGIAEoAUgAUgV2YWx1ZYgBARIh'
    'Cgl0aHJlc2hvbGQYByABKAFIAVIJdGhyZXNob2xkiAEBEhgKB21lc3NhZ2UYCCABKAlSB21lc3'
    'NhZ2USHQoKY3JlYXRlZF9hdBgJIAEoA1IJY3JlYXRlZEF0EiwKD2Fja25vd2xlZGdlZF9hdBgK'
    'IAEoA0gCUg5hY2tub3dsZWRnZWRBdIgBARIsCg9hY2tub3dsZWRnZWRfYnkYCyABKANIA1IOYW'
    'Nrbm93bGVkZ2VkQnmIAQESKgoOc2lsZW5jZWRfdW50aWwYDCABKANIBFINc2lsZW5jZWRVbnRp'
    'bIgBARIkCgtyZXNvbHZlZF9hdBgNIAEoA0gFUgpyZXNvbHZlZEF0iAEBEkMKG25vdGlmaWNhdG'
    'lvbl9jb3JyZWxhdGlvbl9pZBgOIAEoCUgGUhlub3RpZmljYXRpb25Db3JyZWxhdGlvbklkiAEB'
    'QggKBl92YWx1ZUIMCgpfdGhyZXNob2xkQhIKEF9hY2tub3dsZWRnZWRfYXRCEgoQX2Fja25vd2'
    'xlZGdlZF9ieUIRCg9fc2lsZW5jZWRfdW50aWxCDgoMX3Jlc29sdmVkX2F0Qh4KHF9ub3RpZmlj'
    'YXRpb25fY29ycmVsYXRpb25faWQ=');

@$core.Deprecated('Use listAlertEventsRequestDescriptor instead')
const ListAlertEventsRequest$json = {
  '1': 'ListAlertEventsRequest',
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

/// Descriptor for `ListAlertEventsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAlertEventsRequestDescriptor =
    $convert.base64Decode(
        'ChZMaXN0QWxlcnRFdmVudHNSZXF1ZXN0EjUKD29yZ2FuaXphdGlvbl9pZBgBIAEoA0IHukgEIg'
        'IgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use listAlertEventsResponseDescriptor instead')
const ListAlertEventsResponse$json = {
  '1': 'ListAlertEventsResponse',
  '2': [
    {
      '1': 'events',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AlertEvent',
      '10': 'events'
    },
  ],
};

/// Descriptor for `ListAlertEventsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAlertEventsResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0QWxlcnRFdmVudHNSZXNwb25zZRI0CgZldmVudHMYASADKAsyHC5vcGVub3hpZGUuYX'
        'BpLnYxLkFsZXJ0RXZlbnRSBmV2ZW50cw==');

@$core.Deprecated('Use acknowledgeAlertEventRequestDescriptor instead')
const AcknowledgeAlertEventRequest$json = {
  '1': 'AcknowledgeAlertEventRequest',
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

/// Descriptor for `AcknowledgeAlertEventRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List acknowledgeAlertEventRequestDescriptor =
    $convert.base64Decode(
        'ChxBY2tub3dsZWRnZUFsZXJ0RXZlbnRSZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZB'
        'I1Cg9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFC'
        'EgoQX29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use silenceAlertEventRequestDescriptor instead')
const SilenceAlertEventRequest$json = {
  '1': 'SilenceAlertEventRequest',
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
    {'1': 'until', '3': 3, '4': 1, '5': 3, '8': {}, '10': 'until'},
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `SilenceAlertEventRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List silenceAlertEventRequestDescriptor = $convert.base64Decode(
    'ChhTaWxlbmNlQWxlcnRFdmVudFJlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjUKD2'
    '9yZ2FuaXphdGlvbl9pZBgCIAEoA0IHukgEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBARIdCgV1'
    'bnRpbBgDIAEoA0IHukgEIgIgAFIFdW50aWxCEgoQX29yZ2FuaXphdGlvbl9pZA==');

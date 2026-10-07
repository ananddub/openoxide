// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/incoming_webhook.proto.

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

@$core.Deprecated('Use incomingWebhookRequestDescriptor instead')
const IncomingWebhookRequest$json = {
  '1': 'IncomingWebhookRequest',
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
    {'1': 'payload', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'payload'},
    {'1': 'signature', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'signature'},
  ],
};

/// Descriptor for `IncomingWebhookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List incomingWebhookRequestDescriptor = $convert.base64Decode(
    'ChZJbmNvbWluZ1dlYmhvb2tSZXF1ZXN0Ek0KCHByb3ZpZGVyGAEgASgOMiUub3Blbm94aWRlLm'
    'FwaS52MS5XZWJob29rUHJvdmlkZXJUeXBlQgq6SAeCAQQQASAAUghwcm92aWRlchIoCgtwcm92'
    'aWRlcl9pZBgCIAEoA0IHukgEIgIgAFIKcHJvdmlkZXJJZBImCgdwYXlsb2FkGAMgASgJQgy6SA'
    'lyBxABGICAgAVSB3BheWxvYWQSJgoJc2lnbmF0dXJlGAQgASgJQgi6SAVyAxiACFIJc2lnbmF0'
    'dXJl');

@$core.Deprecated('Use incomingWebhookDispatchOutcomeDescriptor instead')
const IncomingWebhookDispatchOutcome$json = {
  '1': 'IncomingWebhookDispatchOutcome',
  '2': [
    {'1': 'event_type', '3': 1, '4': 1, '5': 9, '10': 'eventType'},
    {
      '1': 'provider',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'provider',
      '17': true
    },
    {'1': 'owner', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'owner', '17': true},
    {
      '1': 'repository',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'repository',
      '17': true
    },
    {
      '1': 'applications_queued',
      '3': 5,
      '4': 1,
      '5': 3,
      '10': 'applicationsQueued'
    },
    {
      '1': 'compose_projects_queued',
      '3': 6,
      '4': 1,
      '5': 3,
      '10': 'composeProjectsQueued'
    },
    {
      '1': 'skipped_by_watch_paths',
      '3': 7,
      '4': 1,
      '5': 3,
      '10': 'skippedByWatchPaths'
    },
    {'1': 'already_running', '3': 8, '4': 1, '5': 3, '10': 'alreadyRunning'},
    {
      '1': 'pull_request_number',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'pullRequestNumber',
      '17': true
    },
    {
      '1': 'pull_request_action',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'pullRequestAction',
      '17': true
    },
    {
      '1': 'source_branch',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'sourceBranch',
      '17': true
    },
    {
      '1': 'source_owner',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'sourceOwner',
      '17': true
    },
    {
      '1': 'source_repository',
      '3': 13,
      '4': 1,
      '5': 9,
      '9': 7,
      '10': 'sourceRepository',
      '17': true
    },
    {
      '1': 'target_branch',
      '3': 14,
      '4': 1,
      '5': 9,
      '9': 8,
      '10': 'targetBranch',
      '17': true
    },
    {
      '1': 'commit',
      '3': 15,
      '4': 1,
      '5': 9,
      '9': 9,
      '10': 'commit',
      '17': true
    },
    {
      '1': 'author',
      '3': 16,
      '4': 1,
      '5': 9,
      '9': 10,
      '10': 'author',
      '17': true
    },
    {'1': 'previews_created', '3': 17, '4': 1, '5': 3, '10': 'previewsCreated'},
    {
      '1': 'previews_redeployed',
      '3': 18,
      '4': 1,
      '5': 3,
      '10': 'previewsRedeployed'
    },
    {'1': 'previews_removed', '3': 19, '4': 1, '5': 3, '10': 'previewsRemoved'},
    {
      '1': 'previews_skipped_permission',
      '3': 20,
      '4': 1,
      '5': 3,
      '10': 'previewsSkippedPermission'
    },
    {
      '1': 'previews_skipped_limit',
      '3': 21,
      '4': 1,
      '5': 3,
      '10': 'previewsSkippedLimit'
    },
  ],
  '8': [
    {'1': '_provider'},
    {'1': '_owner'},
    {'1': '_repository'},
    {'1': '_pull_request_number'},
    {'1': '_pull_request_action'},
    {'1': '_source_branch'},
    {'1': '_source_owner'},
    {'1': '_source_repository'},
    {'1': '_target_branch'},
    {'1': '_commit'},
    {'1': '_author'},
  ],
};

/// Descriptor for `IncomingWebhookDispatchOutcome`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List incomingWebhookDispatchOutcomeDescriptor = $convert.base64Decode(
    'Ch5JbmNvbWluZ1dlYmhvb2tEaXNwYXRjaE91dGNvbWUSHQoKZXZlbnRfdHlwZRgBIAEoCVIJZX'
    'ZlbnRUeXBlEh8KCHByb3ZpZGVyGAIgASgJSABSCHByb3ZpZGVyiAEBEhkKBW93bmVyGAMgASgJ'
    'SAFSBW93bmVyiAEBEiMKCnJlcG9zaXRvcnkYBCABKAlIAlIKcmVwb3NpdG9yeYgBARIvChNhcH'
    'BsaWNhdGlvbnNfcXVldWVkGAUgASgDUhJhcHBsaWNhdGlvbnNRdWV1ZWQSNgoXY29tcG9zZV9w'
    'cm9qZWN0c19xdWV1ZWQYBiABKANSFWNvbXBvc2VQcm9qZWN0c1F1ZXVlZBIzChZza2lwcGVkX2'
    'J5X3dhdGNoX3BhdGhzGAcgASgDUhNza2lwcGVkQnlXYXRjaFBhdGhzEicKD2FscmVhZHlfcnVu'
    'bmluZxgIIAEoA1IOYWxyZWFkeVJ1bm5pbmcSMwoTcHVsbF9yZXF1ZXN0X251bWJlchgJIAEoCU'
    'gDUhFwdWxsUmVxdWVzdE51bWJlcogBARIzChNwdWxsX3JlcXVlc3RfYWN0aW9uGAogASgJSARS'
    'EXB1bGxSZXF1ZXN0QWN0aW9uiAEBEigKDXNvdXJjZV9icmFuY2gYCyABKAlIBVIMc291cmNlQn'
    'JhbmNoiAEBEiYKDHNvdXJjZV9vd25lchgMIAEoCUgGUgtzb3VyY2VPd25lcogBARIwChFzb3Vy'
    'Y2VfcmVwb3NpdG9yeRgNIAEoCUgHUhBzb3VyY2VSZXBvc2l0b3J5iAEBEigKDXRhcmdldF9icm'
    'FuY2gYDiABKAlICFIMdGFyZ2V0QnJhbmNoiAEBEhsKBmNvbW1pdBgPIAEoCUgJUgZjb21taXSI'
    'AQESGwoGYXV0aG9yGBAgASgJSApSBmF1dGhvcogBARIpChBwcmV2aWV3c19jcmVhdGVkGBEgAS'
    'gDUg9wcmV2aWV3c0NyZWF0ZWQSLwoTcHJldmlld3NfcmVkZXBsb3llZBgSIAEoA1IScHJldmll'
    'd3NSZWRlcGxveWVkEikKEHByZXZpZXdzX3JlbW92ZWQYEyABKANSD3ByZXZpZXdzUmVtb3ZlZB'
    'I+ChtwcmV2aWV3c19za2lwcGVkX3Blcm1pc3Npb24YFCABKANSGXByZXZpZXdzU2tpcHBlZFBl'
    'cm1pc3Npb24SNAoWcHJldmlld3Nfc2tpcHBlZF9saW1pdBgVIAEoA1IUcHJldmlld3NTa2lwcG'
    'VkTGltaXRCCwoJX3Byb3ZpZGVyQggKBl9vd25lckINCgtfcmVwb3NpdG9yeUIWChRfcHVsbF9y'
    'ZXF1ZXN0X251bWJlckIWChRfcHVsbF9yZXF1ZXN0X2FjdGlvbkIQCg5fc291cmNlX2JyYW5jaE'
    'IPCg1fc291cmNlX293bmVyQhQKEl9zb3VyY2VfcmVwb3NpdG9yeUIQCg5fdGFyZ2V0X2JyYW5j'
    'aEIJCgdfY29tbWl0QgkKB19hdXRob3I=');

@$core.Deprecated('Use incomingWebhookResponseDescriptor instead')
const IncomingWebhookResponse$json = {
  '1': 'IncomingWebhookResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {
      '1': 'outcome',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.IncomingWebhookDispatchOutcome',
      '10': 'outcome'
    },
  ],
};

/// Descriptor for `IncomingWebhookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List incomingWebhookResponseDescriptor = $convert.base64Decode(
    'ChdJbmNvbWluZ1dlYmhvb2tSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEkoKB2'
    '91dGNvbWUYAiABKAsyMC5vcGVub3hpZGUuYXBpLnYxLkluY29taW5nV2ViaG9va0Rpc3BhdGNo'
    'T3V0Y29tZVIHb3V0Y29tZQ==');

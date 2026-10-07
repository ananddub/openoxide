// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ai.proto.

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

@$core.Deprecated('Use aiProviderDescriptor instead')
const AiProvider$json = {
  '1': 'AiProvider',
  '2': [
    {'1': 'AI_PROVIDER_UNSPECIFIED', '2': 0},
    {'1': 'AI_PROVIDER_OPENAI', '2': 1},
    {'1': 'AI_PROVIDER_ANTHROPIC', '2': 2},
    {'1': 'AI_PROVIDER_GEMINI', '2': 3},
    {'1': 'AI_PROVIDER_OLLAMA', '2': 4},
    {'1': 'AI_PROVIDER_OPENAI_COMPATIBLE', '2': 5},
  ],
};

/// Descriptor for `AiProvider`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List aiProviderDescriptor = $convert.base64Decode(
    'CgpBaVByb3ZpZGVyEhsKF0FJX1BST1ZJREVSX1VOU1BFQ0lGSUVEEAASFgoSQUlfUFJPVklERV'
    'JfT1BFTkFJEAESGQoVQUlfUFJPVklERVJfQU5USFJPUElDEAISFgoSQUlfUFJPVklERVJfR0VN'
    'SU5JEAMSFgoSQUlfUFJPVklERVJfT0xMQU1BEAQSIQodQUlfUFJPVklERVJfT1BFTkFJX0NPTV'
    'BBVElCTEUQBQ==');

@$core.Deprecated('Use aiLogContextDescriptor instead')
const AiLogContext$json = {
  '1': 'AiLogContext',
  '2': [
    {'1': 'AI_LOG_CONTEXT_UNSPECIFIED', '2': 0},
    {'1': 'AI_LOG_CONTEXT_BUILD', '2': 1},
    {'1': 'AI_LOG_CONTEXT_RUNTIME', '2': 2},
  ],
};

/// Descriptor for `AiLogContext`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List aiLogContextDescriptor = $convert.base64Decode(
    'CgxBaUxvZ0NvbnRleHQSHgoaQUlfTE9HX0NPTlRFWFRfVU5TUEVDSUZJRUQQABIYChRBSV9MT0'
    'dfQ09OVEVYVF9CVUlMRBABEhoKFkFJX0xPR19DT05URVhUX1JVTlRJTUUQAg==');

@$core.Deprecated('Use aiInteractionKindDescriptor instead')
const AiInteractionKind$json = {
  '1': 'AiInteractionKind',
  '2': [
    {'1': 'AI_INTERACTION_KIND_UNSPECIFIED', '2': 0},
    {'1': 'AI_INTERACTION_KIND_CONFIRM', '2': 1},
    {'1': 'AI_INTERACTION_KIND_CHOICE', '2': 2},
  ],
};

/// Descriptor for `AiInteractionKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List aiInteractionKindDescriptor = $convert.base64Decode(
    'ChFBaUludGVyYWN0aW9uS2luZBIjCh9BSV9JTlRFUkFDVElPTl9LSU5EX1VOU1BFQ0lGSUVEEA'
    'ASHwobQUlfSU5URVJBQ1RJT05fS0lORF9DT05GSVJNEAESHgoaQUlfSU5URVJBQ1RJT05fS0lO'
    'RF9DSE9JQ0UQAg==');

@$core.Deprecated('Use aiSettingDescriptor instead')
const AiSetting$json = {
  '1': 'AiSetting',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'api_url', '3': 3, '4': 1, '5': 9, '10': 'apiUrl'},
    {'1': 'model', '3': 4, '4': 1, '5': 9, '10': 'model'},
    {'1': 'is_enabled', '3': 5, '4': 1, '5': 8, '10': 'isEnabled'},
    {
      '1': 'provider',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AiProvider',
      '10': 'provider'
    },
    {'1': 'has_api_key', '3': 7, '4': 1, '5': 8, '10': 'hasApiKey'},
    {'1': 'organization_id', '3': 8, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
};

/// Descriptor for `AiSetting`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiSettingDescriptor = $convert.base64Decode(
    'CglBaVNldHRpbmcSDgoCaWQYASABKANSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSFwoHYXBpX3'
    'VybBgDIAEoCVIGYXBpVXJsEhQKBW1vZGVsGAQgASgJUgVtb2RlbBIdCgppc19lbmFibGVkGAUg'
    'ASgIUglpc0VuYWJsZWQSOAoIcHJvdmlkZXIYBiABKA4yHC5vcGVub3hpZGUuYXBpLnYxLkFpUH'
    'JvdmlkZXJSCHByb3ZpZGVyEh4KC2hhc19hcGlfa2V5GAcgASgIUgloYXNBcGlLZXkSJwoPb3Jn'
    'YW5pemF0aW9uX2lkGAggASgDUg5vcmdhbml6YXRpb25JZBIdCgpjcmVhdGVkX2F0GAkgASgDUg'
    'ljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgKIAEoA1IJdXBkYXRlZEF0');

@$core.Deprecated('Use aiEnvironmentVariableDescriptor instead')
const AiEnvironmentVariable$json = {
  '1': 'AiEnvironmentVariable',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
};

/// Descriptor for `AiEnvironmentVariable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiEnvironmentVariableDescriptor = $convert.base64Decode(
    'ChVBaUVudmlyb25tZW50VmFyaWFibGUSEgoEbmFtZRgBIAEoCVIEbmFtZRIUCgV2YWx1ZRgCIA'
    'EoCVIFdmFsdWU=');

@$core.Deprecated('Use aiDomainDescriptor instead')
const AiDomain$json = {
  '1': 'AiDomain',
  '2': [
    {'1': 'host', '3': 1, '4': 1, '5': 9, '10': 'host'},
    {'1': 'port', '3': 2, '4': 1, '5': 3, '10': 'port'},
    {'1': 'service_name', '3': 3, '4': 1, '5': 9, '10': 'serviceName'},
  ],
};

/// Descriptor for `AiDomain`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDomainDescriptor = $convert.base64Decode(
    'CghBaURvbWFpbhISCgRob3N0GAEgASgJUgRob3N0EhIKBHBvcnQYAiABKANSBHBvcnQSIQoMc2'
    'VydmljZV9uYW1lGAMgASgJUgtzZXJ2aWNlTmFtZQ==');

@$core.Deprecated('Use aiGeneratedFileDescriptor instead')
const AiGeneratedFile$json = {
  '1': 'AiGeneratedFile',
  '2': [
    {'1': 'file_path', '3': 1, '4': 1, '5': 9, '10': 'filePath'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `AiGeneratedFile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGeneratedFileDescriptor = $convert.base64Decode(
    'Cg9BaUdlbmVyYXRlZEZpbGUSGwoJZmlsZV9wYXRoGAEgASgJUghmaWxlUGF0aBIYCgdjb250ZW'
    '50GAIgASgJUgdjb250ZW50');

@$core.Deprecated('Use aiComposeSuggestionDescriptor instead')
const AiComposeSuggestion$json = {
  '1': 'AiComposeSuggestion',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'short_description',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'shortDescription'
    },
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
    {'1': 'docker_compose', '3': 5, '4': 1, '5': 9, '10': 'dockerCompose'},
    {
      '1': 'env_variables',
      '3': 6,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiEnvironmentVariable',
      '10': 'envVariables'
    },
    {
      '1': 'domains',
      '3': 7,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiDomain',
      '10': 'domains'
    },
    {
      '1': 'config_files',
      '3': 8,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneratedFile',
      '10': 'configFiles'
    },
  ],
};

/// Descriptor for `AiComposeSuggestion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiComposeSuggestionDescriptor = $convert.base64Decode(
    'ChNBaUNvbXBvc2VTdWdnZXN0aW9uEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW'
    '1lEisKEXNob3J0X2Rlc2NyaXB0aW9uGAMgASgJUhBzaG9ydERlc2NyaXB0aW9uEiAKC2Rlc2Ny'
    'aXB0aW9uGAQgASgJUgtkZXNjcmlwdGlvbhIlCg5kb2NrZXJfY29tcG9zZRgFIAEoCVINZG9ja2'
    'VyQ29tcG9zZRJMCg1lbnZfdmFyaWFibGVzGAYgAygLMicub3Blbm94aWRlLmFwaS52MS5BaUVu'
    'dmlyb25tZW50VmFyaWFibGVSDGVudlZhcmlhYmxlcxI0Cgdkb21haW5zGAcgAygLMhoub3Blbm'
    '94aWRlLmFwaS52MS5BaURvbWFpblIHZG9tYWlucxJECgxjb25maWdfZmlsZXMYCCADKAsyIS5v'
    'cGVub3hpZGUuYXBpLnYxLkFpR2VuZXJhdGVkRmlsZVILY29uZmlnRmlsZXM=');

@$core.Deprecated('Use aiGenerationOutputDescriptor instead')
const AiGenerationOutput$json = {
  '1': 'AiGenerationOutput',
  '2': [
    {
      '1': 'suggestions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiComposeSuggestion',
      '10': 'suggestions'
    },
  ],
};

/// Descriptor for `AiGenerationOutput`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGenerationOutputDescriptor = $convert.base64Decode(
    'ChJBaUdlbmVyYXRpb25PdXRwdXQSRwoLc3VnZ2VzdGlvbnMYASADKAsyJS5vcGVub3hpZGUuYX'
    'BpLnYxLkFpQ29tcG9zZVN1Z2dlc3Rpb25SC3N1Z2dlc3Rpb25z');

@$core.Deprecated('Use aiGenerationDescriptor instead')
const AiGeneration$json = {
  '1': 'AiGeneration',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'ai_setting_id', '3': 2, '4': 1, '5': 3, '10': 'aiSettingId'},
    {'1': 'organization_id', '3': 3, '4': 1, '5': 3, '10': 'organizationId'},
    {'1': 'created_by', '3': 4, '4': 1, '5': 3, '10': 'createdBy'},
    {'1': 'prompt', '3': 5, '4': 1, '5': 9, '10': 'prompt'},
    {
      '1': 'output',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGenerationOutput',
      '10': 'output'
    },
    {'1': 'status', '3': 7, '4': 1, '5': 9, '10': 'status'},
    {
      '1': 'compose_id',
      '3': 8,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'composeId',
      '17': true
    },
    {'1': 'created_at', '3': 9, '4': 1, '5': 3, '10': 'createdAt'},
    {'1': 'updated_at', '3': 10, '4': 1, '5': 3, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_compose_id'},
  ],
};

/// Descriptor for `AiGeneration`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGenerationDescriptor = $convert.base64Decode(
    'CgxBaUdlbmVyYXRpb24SDgoCaWQYASABKANSAmlkEiIKDWFpX3NldHRpbmdfaWQYAiABKANSC2'
    'FpU2V0dGluZ0lkEicKD29yZ2FuaXphdGlvbl9pZBgDIAEoA1IOb3JnYW5pemF0aW9uSWQSHQoK'
    'Y3JlYXRlZF9ieRgEIAEoA1IJY3JlYXRlZEJ5EhYKBnByb21wdBgFIAEoCVIGcHJvbXB0EjwKBm'
    '91dHB1dBgGIAEoCzIkLm9wZW5veGlkZS5hcGkudjEuQWlHZW5lcmF0aW9uT3V0cHV0UgZvdXRw'
    'dXQSFgoGc3RhdHVzGAcgASgJUgZzdGF0dXMSIgoKY29tcG9zZV9pZBgIIAEoA0gAUgljb21wb3'
    'NlSWSIAQESHQoKY3JlYXRlZF9hdBgJIAEoA1IJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYCiAB'
    'KANSCXVwZGF0ZWRBdEINCgtfY29tcG9zZV9pZA==');

@$core.Deprecated('Use aiListSettingsRequestDescriptor instead')
const AiListSettingsRequest$json = {
  '1': 'AiListSettingsRequest',
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

/// Descriptor for `AiListSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListSettingsRequestDescriptor = $convert.base64Decode(
    'ChVBaUxpc3RTZXR0aW5nc1JlcXVlc3QSNQoPb3JnYW5pemF0aW9uX2lkGAEgASgDQge6SAQiAi'
    'AASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use aiListSettingsResponseDescriptor instead')
const AiListSettingsResponse$json = {
  '1': 'AiListSettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiSetting',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `AiListSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChZBaUxpc3RTZXR0aW5nc1Jlc3BvbnNlEjcKCHNldHRpbmdzGAEgAygLMhsub3Blbm94aWRlLm'
        'FwaS52MS5BaVNldHRpbmdSCHNldHRpbmdz');

@$core.Deprecated('Use aiGetSettingsRequestDescriptor instead')
const AiGetSettingsRequest$json = {
  '1': 'AiGetSettingsRequest',
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

/// Descriptor for `AiGetSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGetSettingsRequestDescriptor = $convert.base64Decode(
    'ChRBaUdldFNldHRpbmdzUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3JnYW'
    '5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9vcmdh'
    'bml6YXRpb25faWQ=');

@$core.Deprecated('Use aiGetSettingsResponseDescriptor instead')
const AiGetSettingsResponse$json = {
  '1': 'AiGetSettingsResponse',
  '2': [
    {
      '1': 'setting',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiSetting',
      '10': 'setting'
    },
  ],
};

/// Descriptor for `AiGetSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGetSettingsResponseDescriptor = $convert.base64Decode(
    'ChVBaUdldFNldHRpbmdzUmVzcG9uc2USNQoHc2V0dGluZxgBIAEoCzIbLm9wZW5veGlkZS5hcG'
    'kudjEuQWlTZXR0aW5nUgdzZXR0aW5n');

@$core.Deprecated('Use aiCreateSettingsRequestDescriptor instead')
const AiCreateSettingsRequest$json = {
  '1': 'AiCreateSettingsRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'api_url', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'apiUrl'},
    {'1': 'api_key', '3': 3, '4': 1, '5': 9, '10': 'apiKey'},
    {'1': 'model', '3': 4, '4': 1, '5': 9, '8': {}, '10': 'model'},
    {'1': 'is_enabled', '3': 5, '4': 1, '5': 8, '10': 'isEnabled'},
    {
      '1': 'organization_id',
      '3': 6,
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

/// Descriptor for `AiCreateSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiCreateSettingsRequestDescriptor = $convert.base64Decode(
    'ChdBaUNyZWF0ZVNldHRpbmdzUmVxdWVzdBIdCgRuYW1lGAEgASgJQgm6SAZyBBABGGRSBG5hbW'
    'USIAoHYXBpX3VybBgCIAEoCUIHukgEcgIQAVIGYXBpVXJsEhcKB2FwaV9rZXkYAyABKAlSBmFw'
    'aUtleRIgCgVtb2RlbBgEIAEoCUIKukgHcgUQARjIAVIFbW9kZWwSHQoKaXNfZW5hYmxlZBgFIA'
    'EoCFIJaXNFbmFibGVkEjUKD29yZ2FuaXphdGlvbl9pZBgGIAEoA0IHukgEIgIgAEgAUg5vcmdh'
    'bml6YXRpb25JZIgBAUISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use aiCreateSettingsResponseDescriptor instead')
const AiCreateSettingsResponse$json = {
  '1': 'AiCreateSettingsResponse',
  '2': [
    {
      '1': 'setting',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiSetting',
      '10': 'setting'
    },
  ],
};

/// Descriptor for `AiCreateSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiCreateSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChhBaUNyZWF0ZVNldHRpbmdzUmVzcG9uc2USNQoHc2V0dGluZxgBIAEoCzIbLm9wZW5veGlkZS'
        '5hcGkudjEuQWlTZXR0aW5nUgdzZXR0aW5n');

@$core.Deprecated('Use aiUpdateSettingsRequestDescriptor instead')
const AiUpdateSettingsRequest$json = {
  '1': 'AiUpdateSettingsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
    {
      '1': 'name',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'name',
      '17': true
    },
    {
      '1': 'api_url',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'apiUrl',
      '17': true
    },
    {
      '1': 'api_key',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'apiKey',
      '17': true
    },
    {
      '1': 'model',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'model',
      '17': true
    },
    {
      '1': 'is_enabled',
      '3': 6,
      '4': 1,
      '5': 8,
      '9': 4,
      '10': 'isEnabled',
      '17': true
    },
    {
      '1': 'organization_id',
      '3': 7,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 5,
      '10': 'organizationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_name'},
    {'1': '_api_url'},
    {'1': '_api_key'},
    {'1': '_model'},
    {'1': '_is_enabled'},
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `AiUpdateSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiUpdateSettingsRequestDescriptor = $convert.base64Decode(
    'ChdBaVVwZGF0ZVNldHRpbmdzUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSIAoEbm'
    'FtZRgCIAEoCUIHukgEcgIYZEgAUgRuYW1liAEBEhwKB2FwaV91cmwYAyABKAlIAVIGYXBpVXJs'
    'iAEBEhwKB2FwaV9rZXkYBCABKAlIAlIGYXBpS2V5iAEBEiMKBW1vZGVsGAUgASgJQgi6SAVyAx'
    'jIAUgDUgVtb2RlbIgBARIiCgppc19lbmFibGVkGAYgASgISARSCWlzRW5hYmxlZIgBARI1Cg9v'
    'cmdhbml6YXRpb25faWQYByABKANCB7pIBCICIABIBVIOb3JnYW5pemF0aW9uSWSIAQFCBwoFX2'
    '5hbWVCCgoIX2FwaV91cmxCCgoIX2FwaV9rZXlCCAoGX21vZGVsQg0KC19pc19lbmFibGVkQhIK'
    'EF9vcmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use aiUpdateSettingsResponseDescriptor instead')
const AiUpdateSettingsResponse$json = {
  '1': 'AiUpdateSettingsResponse',
  '2': [
    {
      '1': 'setting',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiSetting',
      '10': 'setting'
    },
  ],
};

/// Descriptor for `AiUpdateSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiUpdateSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChhBaVVwZGF0ZVNldHRpbmdzUmVzcG9uc2USNQoHc2V0dGluZxgBIAEoCzIbLm9wZW5veGlkZS'
        '5hcGkudjEuQWlTZXR0aW5nUgdzZXR0aW5n');

@$core.Deprecated('Use aiDeleteSettingsRequestDescriptor instead')
const AiDeleteSettingsRequest$json = {
  '1': 'AiDeleteSettingsRequest',
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

/// Descriptor for `AiDeleteSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDeleteSettingsRequestDescriptor = $convert.base64Decode(
    'ChdBaURlbGV0ZVNldHRpbmdzUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAFICaWQSNQoPb3'
    'JnYW5pemF0aW9uX2lkGAIgASgDQge6SAQiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBQhIKEF9v'
    'cmdhbml6YXRpb25faWQ=');

@$core.Deprecated('Use aiListModelsRequestDescriptor instead')
const AiListModelsRequest$json = {
  '1': 'AiListModelsRequest',
  '2': [
    {'1': 'setting_id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'settingId'},
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

/// Descriptor for `AiListModelsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListModelsRequestDescriptor = $convert.base64Decode(
    'ChNBaUxpc3RNb2RlbHNSZXF1ZXN0EiYKCnNldHRpbmdfaWQYASABKANCB7pIBCICIABSCXNldH'
    'RpbmdJZBI1Cg9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9u'
    'SWSIAQFCEgoQX29yZ2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use aiListModelsResponseDescriptor instead')
const AiListModelsResponse$json = {
  '1': 'AiListModelsResponse',
  '2': [
    {'1': 'models', '3': 1, '4': 3, '5': 9, '10': 'models'},
  ],
};

/// Descriptor for `AiListModelsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListModelsResponseDescriptor =
    $convert.base64Decode(
        'ChRBaUxpc3RNb2RlbHNSZXNwb25zZRIWCgZtb2RlbHMYASADKAlSBm1vZGVscw==');

@$core.Deprecated('Use aiTestSettingsRequestDescriptor instead')
const AiTestSettingsRequest$json = {
  '1': 'AiTestSettingsRequest',
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

/// Descriptor for `AiTestSettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiTestSettingsRequestDescriptor = $convert.base64Decode(
    'ChVBaVRlc3RTZXR0aW5nc1JlcXVlc3QSFwoCaWQYASABKANCB7pIBCICIABSAmlkEjUKD29yZ2'
    'FuaXphdGlvbl9pZBgCIAEoA0IHukgEIgIgAEgAUg5vcmdhbml6YXRpb25JZIgBAUISChBfb3Jn'
    'YW5pemF0aW9uX2lk');

@$core.Deprecated('Use aiTestSettingsResponseDescriptor instead')
const AiTestSettingsResponse$json = {
  '1': 'AiTestSettingsResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `AiTestSettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiTestSettingsResponseDescriptor =
    $convert.base64Decode(
        'ChZBaVRlc3RTZXR0aW5nc1Jlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSGAoHbW'
        'Vzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');

@$core.Deprecated('Use aiDiscoverModelsRequestDescriptor instead')
const AiDiscoverModelsRequest$json = {
  '1': 'AiDiscoverModelsRequest',
  '2': [
    {'1': 'api_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'apiUrl'},
    {'1': 'api_key', '3': 2, '4': 1, '5': 9, '10': 'apiKey'},
  ],
};

/// Descriptor for `AiDiscoverModelsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDiscoverModelsRequestDescriptor =
    $convert.base64Decode(
        'ChdBaURpc2NvdmVyTW9kZWxzUmVxdWVzdBIgCgdhcGlfdXJsGAEgASgJQge6SARyAhABUgZhcG'
        'lVcmwSFwoHYXBpX2tleRgCIAEoCVIGYXBpS2V5');

@$core.Deprecated('Use aiDiscoverModelsResponseDescriptor instead')
const AiDiscoverModelsResponse$json = {
  '1': 'AiDiscoverModelsResponse',
  '2': [
    {'1': 'models', '3': 1, '4': 3, '5': 9, '10': 'models'},
  ],
};

/// Descriptor for `AiDiscoverModelsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDiscoverModelsResponseDescriptor =
    $convert.base64Decode(
        'ChhBaURpc2NvdmVyTW9kZWxzUmVzcG9uc2USFgoGbW9kZWxzGAEgAygJUgZtb2RlbHM=');

@$core.Deprecated('Use aiTestConnectionRequestDescriptor instead')
const AiTestConnectionRequest$json = {
  '1': 'AiTestConnectionRequest',
  '2': [
    {'1': 'api_url', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'apiUrl'},
    {'1': 'api_key', '3': 2, '4': 1, '5': 9, '10': 'apiKey'},
    {'1': 'model', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'model'},
  ],
};

/// Descriptor for `AiTestConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiTestConnectionRequestDescriptor = $convert.base64Decode(
    'ChdBaVRlc3RDb25uZWN0aW9uUmVxdWVzdBIgCgdhcGlfdXJsGAEgASgJQge6SARyAhABUgZhcG'
    'lVcmwSFwoHYXBpX2tleRgCIAEoCVIGYXBpS2V5Eh0KBW1vZGVsGAMgASgJQge6SARyAhABUgVt'
    'b2RlbA==');

@$core.Deprecated('Use aiTestConnectionResponseDescriptor instead')
const AiTestConnectionResponse$json = {
  '1': 'AiTestConnectionResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `AiTestConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiTestConnectionResponseDescriptor =
    $convert.base64Decode(
        'ChhBaVRlc3RDb25uZWN0aW9uUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2VzcxIYCg'
        'dtZXNzYWdlGAIgASgJUgdtZXNzYWdl');

@$core.Deprecated('Use aiListGenerationsRequestDescriptor instead')
const AiListGenerationsRequest$json = {
  '1': 'AiListGenerationsRequest',
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
    {
      '1': 'limit',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'limit',
      '17': true
    },
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_limit'},
  ],
};

/// Descriptor for `AiListGenerationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListGenerationsRequestDescriptor = $convert.base64Decode(
    'ChhBaUxpc3RHZW5lcmF0aW9uc1JlcXVlc3QSNQoPb3JnYW5pemF0aW9uX2lkGAEgASgDQge6SA'
    'QiAiAASABSDm9yZ2FuaXphdGlvbklkiAEBEiQKBWxpbWl0GAIgASgDQgm6SAYiBBhkKAFIAVIF'
    'bGltaXSIAQFCEgoQX29yZ2FuaXphdGlvbl9pZEIICgZfbGltaXQ=');

@$core.Deprecated('Use aiListGenerationsResponseDescriptor instead')
const AiListGenerationsResponse$json = {
  '1': 'AiListGenerationsResponse',
  '2': [
    {
      '1': 'generations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneration',
      '10': 'generations'
    },
  ],
};

/// Descriptor for `AiListGenerationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiListGenerationsResponseDescriptor =
    $convert.base64Decode(
        'ChlBaUxpc3RHZW5lcmF0aW9uc1Jlc3BvbnNlEkAKC2dlbmVyYXRpb25zGAEgAygLMh4ub3Blbm'
        '94aWRlLmFwaS52MS5BaUdlbmVyYXRpb25SC2dlbmVyYXRpb25z');

@$core.Deprecated('Use aiGetGenerationRequestDescriptor instead')
const AiGetGenerationRequest$json = {
  '1': 'AiGetGenerationRequest',
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

/// Descriptor for `AiGetGenerationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGetGenerationRequestDescriptor = $convert.base64Decode(
    'ChZBaUdldEdlbmVyYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg9vcm'
    'dhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQFCEgoQX29y'
    'Z2FuaXphdGlvbl9pZA==');

@$core.Deprecated('Use aiGetGenerationResponseDescriptor instead')
const AiGetGenerationResponse$json = {
  '1': 'AiGetGenerationResponse',
  '2': [
    {
      '1': 'generation',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneration',
      '10': 'generation'
    },
  ],
};

/// Descriptor for `AiGetGenerationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiGetGenerationResponseDescriptor =
    $convert.base64Decode(
        'ChdBaUdldEdlbmVyYXRpb25SZXNwb25zZRI+CgpnZW5lcmF0aW9uGAEgASgLMh4ub3Blbm94aW'
        'RlLmFwaS52MS5BaUdlbmVyYXRpb25SCmdlbmVyYXRpb24=');

@$core.Deprecated('Use aiCreateGenerationRequestDescriptor instead')
const AiCreateGenerationRequest$json = {
  '1': 'AiCreateGenerationRequest',
  '2': [
    {
      '1': 'ai_setting_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'aiSettingId'
    },
    {'1': 'request', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'request'},
  ],
};

/// Descriptor for `AiCreateGenerationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiCreateGenerationRequestDescriptor = $convert.base64Decode(
    'ChlBaUNyZWF0ZUdlbmVyYXRpb25SZXF1ZXN0EisKDWFpX3NldHRpbmdfaWQYASABKANCB7pIBC'
    'ICIABSC2FpU2V0dGluZ0lkEiQKB3JlcXVlc3QYAiABKAlCCrpIB3IFEAEYwD5SB3JlcXVlc3Q=');

@$core.Deprecated('Use aiCreateGenerationResponseDescriptor instead')
const AiCreateGenerationResponse$json = {
  '1': 'AiCreateGenerationResponse',
  '2': [
    {
      '1': 'generation',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneration',
      '10': 'generation'
    },
  ],
};

/// Descriptor for `AiCreateGenerationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiCreateGenerationResponseDescriptor =
    $convert.base64Decode(
        'ChpBaUNyZWF0ZUdlbmVyYXRpb25SZXNwb25zZRI+CgpnZW5lcmF0aW9uGAEgASgLMh4ub3Blbm'
        '94aWRlLmFwaS52MS5BaUdlbmVyYXRpb25SCmdlbmVyYXRpb24=');

@$core.Deprecated('Use aiReviewGenerationRequestDescriptor instead')
const AiReviewGenerationRequest$json = {
  '1': 'AiReviewGenerationRequest',
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
      '1': 'output',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGenerationOutput',
      '10': 'output'
    },
  ],
  '8': [
    {'1': '_organization_id'},
  ],
};

/// Descriptor for `AiReviewGenerationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiReviewGenerationRequestDescriptor = $convert.base64Decode(
    'ChlBaVJldmlld0dlbmVyYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg'
    '9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESPAoG'
    'b3V0cHV0GAMgASgLMiQub3Blbm94aWRlLmFwaS52MS5BaUdlbmVyYXRpb25PdXRwdXRSBm91dH'
    'B1dEISChBfb3JnYW5pemF0aW9uX2lk');

@$core.Deprecated('Use aiReviewGenerationResponseDescriptor instead')
const AiReviewGenerationResponse$json = {
  '1': 'AiReviewGenerationResponse',
  '2': [
    {
      '1': 'generation',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneration',
      '10': 'generation'
    },
  ],
};

/// Descriptor for `AiReviewGenerationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiReviewGenerationResponseDescriptor =
    $convert.base64Decode(
        'ChpBaVJldmlld0dlbmVyYXRpb25SZXNwb25zZRI+CgpnZW5lcmF0aW9uGAEgASgLMh4ub3Blbm'
        '94aWRlLmFwaS52MS5BaUdlbmVyYXRpb25SCmdlbmVyYXRpb24=');

@$core.Deprecated('Use aiDeployGenerationRequestDescriptor instead')
const AiDeployGenerationRequest$json = {
  '1': 'AiDeployGenerationRequest',
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
      '1': 'suggestion_id',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'suggestionId'
    },
    {
      '1': 'environment_id',
      '3': 4,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'environmentId'
    },
    {
      '1': 'server_id',
      '3': 5,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 1,
      '10': 'serverId',
      '17': true
    },
    {'1': 'deploy_now', '3': 6, '4': 1, '5': 8, '10': 'deployNow'},
  ],
  '8': [
    {'1': '_organization_id'},
    {'1': '_server_id'},
  ],
};

/// Descriptor for `AiDeployGenerationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDeployGenerationRequestDescriptor = $convert.base64Decode(
    'ChlBaURlcGxveUdlbmVyYXRpb25SZXF1ZXN0EhcKAmlkGAEgASgDQge6SAQiAiAAUgJpZBI1Cg'
    '9vcmdhbml6YXRpb25faWQYAiABKANCB7pIBCICIABIAFIOb3JnYW5pemF0aW9uSWSIAQESLAoN'
    'c3VnZ2VzdGlvbl9pZBgDIAEoCUIHukgEcgIQAVIMc3VnZ2VzdGlvbklkEi4KDmVudmlyb25tZW'
    '50X2lkGAQgASgDQge6SAQiAiAAUg1lbnZpcm9ubWVudElkEikKCXNlcnZlcl9pZBgFIAEoA0IH'
    'ukgEIgIgAEgBUghzZXJ2ZXJJZIgBARIdCgpkZXBsb3lfbm93GAYgASgIUglkZXBsb3lOb3dCEg'
    'oQX29yZ2FuaXphdGlvbl9pZEIMCgpfc2VydmVyX2lk');

@$core.Deprecated('Use aiDeployGenerationResponseDescriptor instead')
const AiDeployGenerationResponse$json = {
  '1': 'AiDeployGenerationResponse',
  '2': [
    {
      '1': 'generation',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiGeneration',
      '10': 'generation'
    },
    {'1': 'compose_id', '3': 2, '4': 1, '5': 3, '10': 'composeId'},
    {
      '1': 'deployment_id',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'deploymentId',
      '17': true
    },
  ],
  '8': [
    {'1': '_deployment_id'},
  ],
};

/// Descriptor for `AiDeployGenerationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiDeployGenerationResponseDescriptor = $convert.base64Decode(
    'ChpBaURlcGxveUdlbmVyYXRpb25SZXNwb25zZRI+CgpnZW5lcmF0aW9uGAEgASgLMh4ub3Blbm'
    '94aWRlLmFwaS52MS5BaUdlbmVyYXRpb25SCmdlbmVyYXRpb24SHQoKY29tcG9zZV9pZBgCIAEo'
    'A1IJY29tcG9zZUlkEigKDWRlcGxveW1lbnRfaWQYAyABKANIAFIMZGVwbG95bWVudElkiAEBQh'
    'AKDl9kZXBsb3ltZW50X2lk');

@$core.Deprecated('Use aiAnalyzeLogsRequestDescriptor instead')
const AiAnalyzeLogsRequest$json = {
  '1': 'AiAnalyzeLogsRequest',
  '2': [
    {
      '1': 'ai_setting_id',
      '3': 1,
      '4': 1,
      '5': 3,
      '8': {},
      '10': 'aiSettingId'
    },
    {
      '1': 'context',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AiLogContext',
      '8': {},
      '10': 'context'
    },
    {'1': 'logs', '3': 3, '4': 1, '5': 9, '8': {}, '10': 'logs'},
  ],
};

/// Descriptor for `AiAnalyzeLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiAnalyzeLogsRequestDescriptor = $convert.base64Decode(
    'ChRBaUFuYWx5emVMb2dzUmVxdWVzdBIrCg1haV9zZXR0aW5nX2lkGAEgASgDQge6SAQiAiAAUg'
    'thaVNldHRpbmdJZBJECgdjb250ZXh0GAIgASgOMh4ub3Blbm94aWRlLmFwaS52MS5BaUxvZ0Nv'
    'bnRleHRCCrpIB4IBBBABIABSB2NvbnRleHQSGwoEbG9ncxgDIAEoCUIHukgEcgIQAVIEbG9ncw'
    '==');

@$core.Deprecated('Use aiAnalyzeLogsResponseDescriptor instead')
const AiAnalyzeLogsResponse$json = {
  '1': 'AiAnalyzeLogsResponse',
  '2': [
    {'1': 'analysis', '3': 1, '4': 1, '5': 9, '10': 'analysis'},
  ],
};

/// Descriptor for `AiAnalyzeLogsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiAnalyzeLogsResponseDescriptor =
    $convert.base64Decode(
        'ChVBaUFuYWx5emVMb2dzUmVzcG9uc2USGgoIYW5hbHlzaXMYASABKAlSCGFuYWx5c2lz');

@$core.Deprecated('Use aiAssistantRequestDescriptor instead')
const AiAssistantRequest$json = {
  '1': 'AiAssistantRequest',
  '2': [
    {'1': 'prompt', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'prompt'},
    {
      '1': 'ai_setting_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'aiSettingId',
      '17': true
    },
    {
      '1': 'history',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiChatTurn',
      '10': 'history'
    },
    {
      '1': 'pending_actions',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'pendingActions',
      '17': true
    },
  ],
  '8': [
    {'1': '_ai_setting_id'},
    {'1': '_pending_actions'},
  ],
};

/// Descriptor for `AiAssistantRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiAssistantRequestDescriptor = $convert.base64Decode(
    'ChJBaUFzc2lzdGFudFJlcXVlc3QSIgoGcHJvbXB0GAEgASgJQgq6SAdyBRABGMA+UgZwcm9tcH'
    'QSMAoNYWlfc2V0dGluZ19pZBgCIAEoA0IHukgEIgIgAEgAUgthaVNldHRpbmdJZIgBARI2Cgdo'
    'aXN0b3J5GAMgAygLMhwub3Blbm94aWRlLmFwaS52MS5BaUNoYXRUdXJuUgdoaXN0b3J5EiwKD3'
    'BlbmRpbmdfYWN0aW9ucxgEIAEoCUgBUg5wZW5kaW5nQWN0aW9uc4gBAUIQCg5fYWlfc2V0dGlu'
    'Z19pZEISChBfcGVuZGluZ19hY3Rpb25z');

@$core.Deprecated('Use aiChatTurnDescriptor instead')
const AiChatTurn$json = {
  '1': 'AiChatTurn',
  '2': [
    {'1': 'role', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'role'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `AiChatTurn`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiChatTurnDescriptor = $convert.base64Decode(
    'CgpBaUNoYXRUdXJuEhsKBHJvbGUYASABKAlCB7pIBHICEAFSBHJvbGUSGAoHY29udGVudBgCIA'
    'EoCVIHY29udGVudA==');

@$core.Deprecated('Use aiAssistantResponseDescriptor instead')
const AiAssistantResponse$json = {
  '1': 'AiAssistantResponse',
  '2': [
    {'1': 'reply', '3': 1, '4': 1, '5': 9, '10': 'reply'},
    {
      '1': 'pending_confirmation',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'pendingConfirmation',
      '17': true
    },
    {
      '1': 'interaction',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.AiInteraction',
      '9': 1,
      '10': 'interaction',
      '17': true
    },
  ],
  '8': [
    {'1': '_pending_confirmation'},
    {'1': '_interaction'},
  ],
};

/// Descriptor for `AiAssistantResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiAssistantResponseDescriptor = $convert.base64Decode(
    'ChNBaUFzc2lzdGFudFJlc3BvbnNlEhQKBXJlcGx5GAEgASgJUgVyZXBseRI2ChRwZW5kaW5nX2'
    'NvbmZpcm1hdGlvbhgCIAEoCUgAUhNwZW5kaW5nQ29uZmlybWF0aW9uiAEBEkYKC2ludGVyYWN0'
    'aW9uGAMgASgLMh8ub3Blbm94aWRlLmFwaS52MS5BaUludGVyYWN0aW9uSAFSC2ludGVyYWN0aW'
    '9uiAEBQhcKFV9wZW5kaW5nX2NvbmZpcm1hdGlvbkIOCgxfaW50ZXJhY3Rpb24=');

@$core.Deprecated('Use aiInteractionOptionDescriptor instead')
const AiInteractionOption$json = {
  '1': 'AiInteractionOption',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'label', '3': 2, '4': 1, '5': 9, '10': 'label'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `AiInteractionOption`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiInteractionOptionDescriptor = $convert.base64Decode(
    'ChNBaUludGVyYWN0aW9uT3B0aW9uEg4KAmlkGAEgASgJUgJpZBIUCgVsYWJlbBgCIAEoCVIFbG'
    'FiZWwSIAoLZGVzY3JpcHRpb24YAyABKAlSC2Rlc2NyaXB0aW9u');

@$core.Deprecated('Use aiInteractionDescriptor instead')
const AiInteraction$json = {
  '1': 'AiInteraction',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.AiInteractionKind',
      '10': 'kind'
    },
    {'1': 'question', '3': 2, '4': 1, '5': 9, '10': 'question'},
    {
      '1': 'options',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AiInteractionOption',
      '10': 'options'
    },
    {'1': 'pending_actions', '3': 4, '4': 1, '5': 9, '10': 'pendingActions'},
  ],
};

/// Descriptor for `AiInteraction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aiInteractionDescriptor = $convert.base64Decode(
    'Cg1BaUludGVyYWN0aW9uEjcKBGtpbmQYASABKA4yIy5vcGVub3hpZGUuYXBpLnYxLkFpSW50ZX'
    'JhY3Rpb25LaW5kUgRraW5kEhoKCHF1ZXN0aW9uGAIgASgJUghxdWVzdGlvbhI/CgdvcHRpb25z'
    'GAMgAygLMiUub3Blbm94aWRlLmFwaS52MS5BaUludGVyYWN0aW9uT3B0aW9uUgdvcHRpb25zEi'
    'cKD3BlbmRpbmdfYWN0aW9ucxgEIAEoCVIOcGVuZGluZ0FjdGlvbnM=');

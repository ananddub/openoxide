// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/ai.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Provider detected from the configured api_url (AiProviderKind::detect).
class AiProvider extends $pb.ProtobufEnum {
  static const AiProvider AI_PROVIDER_UNSPECIFIED =
      AiProvider._(0, _omitEnumNames ? '' : 'AI_PROVIDER_UNSPECIFIED');
  static const AiProvider AI_PROVIDER_OPENAI =
      AiProvider._(1, _omitEnumNames ? '' : 'AI_PROVIDER_OPENAI');
  static const AiProvider AI_PROVIDER_ANTHROPIC =
      AiProvider._(2, _omitEnumNames ? '' : 'AI_PROVIDER_ANTHROPIC');
  static const AiProvider AI_PROVIDER_GEMINI =
      AiProvider._(3, _omitEnumNames ? '' : 'AI_PROVIDER_GEMINI');
  static const AiProvider AI_PROVIDER_OLLAMA =
      AiProvider._(4, _omitEnumNames ? '' : 'AI_PROVIDER_OLLAMA');
  static const AiProvider AI_PROVIDER_OPENAI_COMPATIBLE =
      AiProvider._(5, _omitEnumNames ? '' : 'AI_PROVIDER_OPENAI_COMPATIBLE');

  static const $core.List<AiProvider> values = <AiProvider>[
    AI_PROVIDER_UNSPECIFIED,
    AI_PROVIDER_OPENAI,
    AI_PROVIDER_ANTHROPIC,
    AI_PROVIDER_GEMINI,
    AI_PROVIDER_OLLAMA,
    AI_PROVIDER_OPENAI_COMPATIBLE,
  ];

  static final $core.List<AiProvider?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static AiProvider? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AiProvider._(super.value, super.name);
}

/// Log corpus fed to the analyzer (AiLogContextDto).
class AiLogContext extends $pb.ProtobufEnum {
  static const AiLogContext AI_LOG_CONTEXT_UNSPECIFIED =
      AiLogContext._(0, _omitEnumNames ? '' : 'AI_LOG_CONTEXT_UNSPECIFIED');
  static const AiLogContext AI_LOG_CONTEXT_BUILD =
      AiLogContext._(1, _omitEnumNames ? '' : 'AI_LOG_CONTEXT_BUILD');
  static const AiLogContext AI_LOG_CONTEXT_RUNTIME =
      AiLogContext._(2, _omitEnumNames ? '' : 'AI_LOG_CONTEXT_RUNTIME');

  static const $core.List<AiLogContext> values = <AiLogContext>[
    AI_LOG_CONTEXT_UNSPECIFIED,
    AI_LOG_CONTEXT_BUILD,
    AI_LOG_CONTEXT_RUNTIME,
  ];

  static final $core.List<AiLogContext?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static AiLogContext? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AiLogContext._(super.value, super.name);
}

class AiInteractionKind extends $pb.ProtobufEnum {
  static const AiInteractionKind AI_INTERACTION_KIND_UNSPECIFIED =
      AiInteractionKind._(
          0, _omitEnumNames ? '' : 'AI_INTERACTION_KIND_UNSPECIFIED');

  /// Yes/No confirmation of a (usually destructive) action.
  static const AiInteractionKind AI_INTERACTION_KIND_CONFIRM =
      AiInteractionKind._(
          1, _omitEnumNames ? '' : 'AI_INTERACTION_KIND_CONFIRM');

  /// Pick one of several options (projects, environments, servers, ...).
  static const AiInteractionKind AI_INTERACTION_KIND_CHOICE =
      AiInteractionKind._(
          2, _omitEnumNames ? '' : 'AI_INTERACTION_KIND_CHOICE');

  static const $core.List<AiInteractionKind> values = <AiInteractionKind>[
    AI_INTERACTION_KIND_UNSPECIFIED,
    AI_INTERACTION_KIND_CONFIRM,
    AI_INTERACTION_KIND_CHOICE,
  ];

  static final $core.List<AiInteractionKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static AiInteractionKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const AiInteractionKind._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');

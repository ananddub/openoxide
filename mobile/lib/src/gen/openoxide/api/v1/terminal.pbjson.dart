// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/terminal.proto.

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

@$core.Deprecated('Use terminalKindDescriptor instead')
const TerminalKind$json = {
  '1': 'TerminalKind',
  '2': [
    {'1': 'TERMINAL_KIND_UNSPECIFIED', '2': 0},
    {'1': 'TERMINAL_KIND_DOCKER', '2': 1},
    {'1': 'TERMINAL_KIND_REMOTE_SERVER', '2': 2},
  ],
};

/// Descriptor for `TerminalKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List terminalKindDescriptor = $convert.base64Decode(
    'CgxUZXJtaW5hbEtpbmQSHQoZVEVSTUlOQUxfS0lORF9VTlNQRUNJRklFRBAAEhgKFFRFUk1JTk'
    'FMX0tJTkRfRE9DS0VSEAESHwobVEVSTUlOQUxfS0lORF9SRU1PVEVfU0VSVkVSEAI=');

@$core.Deprecated('Use terminalOutputStreamDescriptor instead')
const TerminalOutputStream$json = {
  '1': 'TerminalOutputStream',
  '2': [
    {'1': 'TERMINAL_OUTPUT_STREAM_UNSPECIFIED', '2': 0},
    {'1': 'TERMINAL_OUTPUT_STREAM_STDOUT', '2': 1},
    {'1': 'TERMINAL_OUTPUT_STREAM_STDERR', '2': 2},
  ],
};

/// Descriptor for `TerminalOutputStream`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List terminalOutputStreamDescriptor = $convert.base64Decode(
    'ChRUZXJtaW5hbE91dHB1dFN0cmVhbRImCiJURVJNSU5BTF9PVVRQVVRfU1RSRUFNX1VOU1BFQ0'
    'lGSUVEEAASIQodVEVSTUlOQUxfT1VUUFVUX1NUUkVBTV9TVERPVVQQARIhCh1URVJNSU5BTF9P'
    'VVRQVVRfU1RSRUFNX1NUREVSUhAC');

@$core.Deprecated('Use openTerminalRequestDescriptor instead')
const OpenTerminalRequest$json = {
  '1': 'OpenTerminalRequest',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.TerminalKind',
      '8': {},
      '10': 'kind'
    },
    {
      '1': 'server_id',
      '3': 2,
      '4': 1,
      '5': 3,
      '8': {},
      '9': 0,
      '10': 'serverId',
      '17': true
    },
    {
      '1': 'container',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'container',
      '17': true
    },
    {
      '1': 'shell',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'shell',
      '17': true
    },
    {
      '1': 'cols',
      '3': 5,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 3,
      '10': 'cols',
      '17': true
    },
    {
      '1': 'rows',
      '3': 6,
      '4': 1,
      '5': 13,
      '8': {},
      '9': 4,
      '10': 'rows',
      '17': true
    },
    {'1': 'host', '3': 7, '4': 1, '5': 8, '10': 'host'},
  ],
  '8': [
    {'1': '_server_id'},
    {'1': '_container'},
    {'1': '_shell'},
    {'1': '_cols'},
    {'1': '_rows'},
  ],
};

/// Descriptor for `OpenTerminalRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openTerminalRequestDescriptor = $convert.base64Decode(
    'ChNPcGVuVGVybWluYWxSZXF1ZXN0Ej4KBGtpbmQYASABKA4yHi5vcGVub3hpZGUuYXBpLnYxLl'
    'Rlcm1pbmFsS2luZEIKukgHggEEEAEgAFIEa2luZBIpCglzZXJ2ZXJfaWQYAiABKANCB7pIBCIC'
    'IABIAFIIc2VydmVySWSIAQESKwoJY29udGFpbmVyGAMgASgJQgi6SAVyAxj/AUgBUgljb250YW'
    'luZXKIAQESIwoFc2hlbGwYBCABKAlCCLpIBXIDGP8BSAJSBXNoZWxsiAEBEiMKBGNvbHMYBSAB'
    'KA1CCrpIByoFGOgHKAFIA1IEY29sc4gBARIjCgRyb3dzGAYgASgNQgq6SAcqBRjoBygBSARSBH'
    'Jvd3OIAQESEgoEaG9zdBgHIAEoCFIEaG9zdEIMCgpfc2VydmVyX2lkQgwKCl9jb250YWluZXJC'
    'CAoGX3NoZWxsQgcKBV9jb2xzQgcKBV9yb3dz');

@$core.Deprecated('Use terminalEventDescriptor instead')
const TerminalEvent$json = {
  '1': 'TerminalEvent',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {
      '1': 'started',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalStarted',
      '9': 0,
      '10': 'started'
    },
    {
      '1': 'output',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalOutput',
      '9': 0,
      '10': 'output'
    },
    {
      '1': 'error',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalError',
      '9': 0,
      '10': 'error'
    },
    {
      '1': 'exit',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalExit',
      '9': 0,
      '10': 'exit'
    },
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `TerminalEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalEventDescriptor = $convert.base64Decode(
    'Cg1UZXJtaW5hbEV2ZW50Eh0KCnNlc3Npb25faWQYASABKAlSCXNlc3Npb25JZBI9CgdzdGFydG'
    'VkGAIgASgLMiEub3Blbm94aWRlLmFwaS52MS5UZXJtaW5hbFN0YXJ0ZWRIAFIHc3RhcnRlZBI6'
    'CgZvdXRwdXQYAyABKAsyIC5vcGVub3hpZGUuYXBpLnYxLlRlcm1pbmFsT3V0cHV0SABSBm91dH'
    'B1dBI3CgVlcnJvchgEIAEoCzIfLm9wZW5veGlkZS5hcGkudjEuVGVybWluYWxFcnJvckgAUgVl'
    'cnJvchI0CgRleGl0GAUgASgLMh4ub3Blbm94aWRlLmFwaS52MS5UZXJtaW5hbEV4aXRIAFIEZX'
    'hpdEIJCgdwYXlsb2Fk');

@$core.Deprecated('Use terminalStartedDescriptor instead')
const TerminalStarted$json = {
  '1': 'TerminalStarted',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.TerminalKind',
      '10': 'kind'
    },
    {'1': 'host', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'host', '17': true},
  ],
  '8': [
    {'1': '_host'},
  ],
};

/// Descriptor for `TerminalStarted`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalStartedDescriptor = $convert.base64Decode(
    'Cg9UZXJtaW5hbFN0YXJ0ZWQSMgoEa2luZBgBIAEoDjIeLm9wZW5veGlkZS5hcGkudjEuVGVybW'
    'luYWxLaW5kUgRraW5kEhcKBGhvc3QYAiABKAlIAFIEaG9zdIgBAUIHCgVfaG9zdA==');

@$core.Deprecated('Use terminalOutputDescriptor instead')
const TerminalOutput$json = {
  '1': 'TerminalOutput',
  '2': [
    {
      '1': 'stream',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.openoxide.api.v1.TerminalOutputStream',
      '8': {},
      '10': 'stream'
    },
    {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `TerminalOutput`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalOutputDescriptor = $convert.base64Decode(
    'Cg5UZXJtaW5hbE91dHB1dBJKCgZzdHJlYW0YASABKA4yJi5vcGVub3hpZGUuYXBpLnYxLlRlcm'
    '1pbmFsT3V0cHV0U3RyZWFtQgq6SAeCAQQQASAAUgZzdHJlYW0SEgoEZGF0YRgCIAEoDFIEZGF0'
    'YQ==');

@$core.Deprecated('Use terminalErrorDescriptor instead')
const TerminalError$json = {
  '1': 'TerminalError',
  '2': [
    {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'code', '17': true},
  ],
  '8': [
    {'1': '_code'},
  ],
};

/// Descriptor for `TerminalError`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalErrorDescriptor = $convert.base64Decode(
    'Cg1UZXJtaW5hbEVycm9yEhgKB21lc3NhZ2UYASABKAlSB21lc3NhZ2USFwoEY29kZRgCIAEoCU'
    'gAUgRjb2RliAEBQgcKBV9jb2Rl');

@$core.Deprecated('Use terminalExitDescriptor instead')
const TerminalExit$json = {
  '1': 'TerminalExit',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'code', '17': true},
  ],
  '8': [
    {'1': '_code'},
  ],
};

/// Descriptor for `TerminalExit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalExitDescriptor = $convert.base64Decode(
    'CgxUZXJtaW5hbEV4aXQSFwoEY29kZRgBIAEoBUgAUgRjb2RliAEBQgcKBV9jb2Rl');

@$core.Deprecated('Use executionCommandRequestDescriptor instead')
const ExecutionCommandRequest$json = {
  '1': 'ExecutionCommandRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'sessionId'},
    {
      '1': 'input',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalInput',
      '9': 0,
      '10': 'input'
    },
    {
      '1': 'resize',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalResize',
      '9': 0,
      '10': 'resize'
    },
    {
      '1': 'stop',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TerminalStop',
      '9': 0,
      '10': 'stop'
    },
  ],
  '8': [
    {'1': 'command'},
  ],
};

/// Descriptor for `ExecutionCommandRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List executionCommandRequestDescriptor = $convert.base64Decode(
    'ChdFeGVjdXRpb25Db21tYW5kUmVxdWVzdBImCgpzZXNzaW9uX2lkGAEgASgJQge6SARyAhABUg'
    'lzZXNzaW9uSWQSNwoFaW5wdXQYAiABKAsyHy5vcGVub3hpZGUuYXBpLnYxLlRlcm1pbmFsSW5w'
    'dXRIAFIFaW5wdXQSOgoGcmVzaXplGAMgASgLMiAub3Blbm94aWRlLmFwaS52MS5UZXJtaW5hbF'
    'Jlc2l6ZUgAUgZyZXNpemUSNAoEc3RvcBgEIAEoCzIeLm9wZW5veGlkZS5hcGkudjEuVGVybWlu'
    'YWxTdG9wSABSBHN0b3BCCQoHY29tbWFuZA==');

@$core.Deprecated('Use terminalInputDescriptor instead')
const TerminalInput$json = {
  '1': 'TerminalInput',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 12, '8': {}, '10': 'data'},
  ],
};

/// Descriptor for `TerminalInput`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalInputDescriptor = $convert.base64Decode(
    'Cg1UZXJtaW5hbElucHV0Eh0KBGRhdGEYASABKAxCCbpIBnoEGICABFIEZGF0YQ==');

@$core.Deprecated('Use terminalResizeDescriptor instead')
const TerminalResize$json = {
  '1': 'TerminalResize',
  '2': [
    {'1': 'cols', '3': 1, '4': 1, '5': 13, '8': {}, '10': 'cols'},
    {'1': 'rows', '3': 2, '4': 1, '5': 13, '8': {}, '10': 'rows'},
  ],
};

/// Descriptor for `TerminalResize`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalResizeDescriptor = $convert.base64Decode(
    'Cg5UZXJtaW5hbFJlc2l6ZRIeCgRjb2xzGAEgASgNQgq6SAcqBRjoBygBUgRjb2xzEh4KBHJvd3'
    'MYAiABKA1CCrpIByoFGOgHKAFSBHJvd3M=');

@$core.Deprecated('Use terminalStopDescriptor instead')
const TerminalStop$json = {
  '1': 'TerminalStop',
};

/// Descriptor for `TerminalStop`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List terminalStopDescriptor =
    $convert.base64Decode('CgxUZXJtaW5hbFN0b3A=');

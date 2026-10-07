// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/auth.proto.

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

@$core.Deprecated('Use signupRequestDescriptor instead')
const SignupRequest$json = {
  '1': 'SignupRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'email'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'password'},
    {
      '1': 'first_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'firstName',
      '17': true
    },
    {
      '1': 'last_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'lastName',
      '17': true
    },
    {'1': 'avatar', '3': 5, '4': 1, '5': 9, '9': 2, '10': 'avatar', '17': true},
  ],
  '8': [
    {'1': '_first_name'},
    {'1': '_last_name'},
    {'1': '_avatar'},
  ],
};

/// Descriptor for `SignupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signupRequestDescriptor = $convert.base64Decode(
    'Cg1TaWdudXBSZXF1ZXN0Eh0KBWVtYWlsGAEgASgJQge6SARyAmABUgVlbWFpbBIjCghwYXNzd2'
    '9yZBgCIAEoCUIHukgEcgIQCFIIcGFzc3dvcmQSIgoKZmlyc3RfbmFtZRgDIAEoCUgAUglmaXJz'
    'dE5hbWWIAQESIAoJbGFzdF9uYW1lGAQgASgJSAFSCGxhc3ROYW1liAEBEhsKBmF2YXRhchgFIA'
    'EoCUgCUgZhdmF0YXKIAQFCDQoLX2ZpcnN0X25hbWVCDAoKX2xhc3RfbmFtZUIJCgdfYXZhdGFy');

@$core.Deprecated('Use loginRequestDescriptor instead')
const LoginRequest$json = {
  '1': 'LoginRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'email'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'password'},
    {
      '1': 'two_factor_code',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'twoFactorCode',
      '17': true
    },
    {
      '1': 'recovery_code',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'recoveryCode',
      '17': true
    },
  ],
  '8': [
    {'1': '_two_factor_code'},
    {'1': '_recovery_code'},
  ],
};

/// Descriptor for `LoginRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginRequestDescriptor = $convert.base64Decode(
    'CgxMb2dpblJlcXVlc3QSHQoFZW1haWwYASABKAlCB7pIBHICYAFSBWVtYWlsEiMKCHBhc3N3b3'
    'JkGAIgASgJQge6SARyAhABUghwYXNzd29yZBIrCg90d29fZmFjdG9yX2NvZGUYAyABKAlIAFIN'
    'dHdvRmFjdG9yQ29kZYgBARIoCg1yZWNvdmVyeV9jb2RlGAQgASgJSAFSDHJlY292ZXJ5Q29kZY'
    'gBAUISChBfdHdvX2ZhY3Rvcl9jb2RlQhAKDl9yZWNvdmVyeV9jb2Rl');

@$core.Deprecated('Use refreshRequestDescriptor instead')
const RefreshRequest$json = {
  '1': 'RefreshRequest',
  '2': [
    {
      '1': 'refresh_token',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '10': 'refreshToken'
    },
  ],
};

/// Descriptor for `RefreshRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List refreshRequestDescriptor = $convert.base64Decode(
    'Cg5SZWZyZXNoUmVxdWVzdBIsCg1yZWZyZXNoX3Rva2VuGAEgASgJQge6SARyAhABUgxyZWZyZX'
    'NoVG9rZW4=');

@$core.Deprecated('Use authResponseDescriptor instead')
const AuthResponse$json = {
  '1': 'AuthResponse',
  '2': [
    {
      '1': 'user',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.User',
      '10': 'user'
    },
    {
      '1': 'tokens',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.TokenPair',
      '10': 'tokens'
    },
  ],
};

/// Descriptor for `AuthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authResponseDescriptor = $convert.base64Decode(
    'CgxBdXRoUmVzcG9uc2USKgoEdXNlchgBIAEoCzIWLm9wZW5veGlkZS5hcGkudjEuVXNlclIEdX'
    'NlchIzCgZ0b2tlbnMYAiABKAsyGy5vcGVub3hpZGUuYXBpLnYxLlRva2VuUGFpclIGdG9rZW5z');

@$core.Deprecated('Use whoAmIRequestDescriptor instead')
const WhoAmIRequest$json = {
  '1': 'WhoAmIRequest',
};

/// Descriptor for `WhoAmIRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List whoAmIRequestDescriptor =
    $convert.base64Decode('Cg1XaG9BbUlSZXF1ZXN0');

@$core.Deprecated('Use logoutRequestDescriptor instead')
const LogoutRequest$json = {
  '1': 'LogoutRequest',
};

/// Descriptor for `LogoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutRequestDescriptor =
    $convert.base64Decode('Cg1Mb2dvdXRSZXF1ZXN0');

@$core.Deprecated('Use getSetupStatusRequestDescriptor instead')
const GetSetupStatusRequest$json = {
  '1': 'GetSetupStatusRequest',
};

/// Descriptor for `GetSetupStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getSetupStatusRequestDescriptor =
    $convert.base64Decode('ChVHZXRTZXR1cFN0YXR1c1JlcXVlc3Q=');

@$core.Deprecated('Use setupStatusDescriptor instead')
const SetupStatus$json = {
  '1': 'SetupStatus',
  '2': [
    {'1': 'owner_present', '3': 1, '4': 1, '5': 8, '10': 'ownerPresent'},
  ],
};

/// Descriptor for `SetupStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setupStatusDescriptor = $convert.base64Decode(
    'CgtTZXR1cFN0YXR1cxIjCg1vd25lcl9wcmVzZW50GAEgASgIUgxvd25lclByZXNlbnQ=');

@$core.Deprecated('Use updateUserRequestDescriptor instead')
const UpdateUserRequest$json = {
  '1': 'UpdateUserRequest',
  '2': [
    {
      '1': 'email',
      '3': 1,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 0,
      '10': 'email',
      '17': true
    },
    {
      '1': 'password',
      '3': 2,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 1,
      '10': 'password',
      '17': true
    },
    {
      '1': 'first_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 2,
      '10': 'firstName',
      '17': true
    },
    {
      '1': 'last_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 3,
      '10': 'lastName',
      '17': true
    },
    {
      '1': 'avatar',
      '3': 5,
      '4': 1,
      '5': 9,
      '8': {},
      '9': 4,
      '10': 'avatar',
      '17': true
    },
  ],
  '8': [
    {'1': '_email'},
    {'1': '_password'},
    {'1': '_first_name'},
    {'1': '_last_name'},
    {'1': '_avatar'},
  ],
};

/// Descriptor for `UpdateUserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateUserRequestDescriptor = $convert.base64Decode(
    'ChFVcGRhdGVVc2VyUmVxdWVzdBIiCgVlbWFpbBgBIAEoCUIHukgEcgJgAUgAUgVlbWFpbIgBAR'
    'IoCghwYXNzd29yZBgCIAEoCUIHukgEcgIQCEgBUghwYXNzd29yZIgBARIsCgpmaXJzdF9uYW1l'
    'GAMgASgJQgi6SAVyAxj/AUgCUglmaXJzdE5hbWWIAQESKgoJbGFzdF9uYW1lGAQgASgJQgi6SA'
    'VyAxj/AUgDUghsYXN0TmFtZYgBARIlCgZhdmF0YXIYBSABKAlCCLpIBXIDiAEBSARSBmF2YXRh'
    'cogBAUIICgZfZW1haWxCCwoJX3Bhc3N3b3JkQg0KC19maXJzdF9uYW1lQgwKCl9sYXN0X25hbW'
    'VCCQoHX2F2YXRhcg==');

@$core.Deprecated('Use requestPasswordResetRequestDescriptor instead')
const RequestPasswordResetRequest$json = {
  '1': 'RequestPasswordResetRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'email'},
  ],
};

/// Descriptor for `RequestPasswordResetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestPasswordResetRequestDescriptor =
    $convert.base64Decode(
        'ChtSZXF1ZXN0UGFzc3dvcmRSZXNldFJlcXVlc3QSHQoFZW1haWwYASABKAlCB7pIBHICYAFSBW'
        'VtYWls');

@$core.Deprecated('Use resetPasswordRequestDescriptor instead')
const ResetPasswordRequest$json = {
  '1': 'ResetPasswordRequest',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'token'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'password'},
  ],
};

/// Descriptor for `ResetPasswordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resetPasswordRequestDescriptor = $convert.base64Decode(
    'ChRSZXNldFBhc3N3b3JkUmVxdWVzdBIdCgV0b2tlbhgBIAEoCUIHukgEcgIQAVIFdG9rZW4SIw'
    'oIcGFzc3dvcmQYAiABKAlCB7pIBHICEAhSCHBhc3N3b3Jk');

@$core.Deprecated('Use getEmailVerificationStatusRequestDescriptor instead')
const GetEmailVerificationStatusRequest$json = {
  '1': 'GetEmailVerificationStatusRequest',
};

/// Descriptor for `GetEmailVerificationStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEmailVerificationStatusRequestDescriptor =
    $convert.base64Decode('CiFHZXRFbWFpbFZlcmlmaWNhdGlvblN0YXR1c1JlcXVlc3Q=');

@$core.Deprecated('Use requestEmailVerificationRequestDescriptor instead')
const RequestEmailVerificationRequest$json = {
  '1': 'RequestEmailVerificationRequest',
};

/// Descriptor for `RequestEmailVerificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestEmailVerificationRequestDescriptor =
    $convert.base64Decode('Ch9SZXF1ZXN0RW1haWxWZXJpZmljYXRpb25SZXF1ZXN0');

@$core.Deprecated('Use confirmEmailVerificationRequestDescriptor instead')
const ConfirmEmailVerificationRequest$json = {
  '1': 'ConfirmEmailVerificationRequest',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'token'},
  ],
};

/// Descriptor for `ConfirmEmailVerificationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List confirmEmailVerificationRequestDescriptor =
    $convert.base64Decode(
        'Ch9Db25maXJtRW1haWxWZXJpZmljYXRpb25SZXF1ZXN0Eh0KBXRva2VuGAEgASgJQge6SARyAh'
        'ABUgV0b2tlbg==');

@$core.Deprecated('Use emailVerificationStatusDescriptor instead')
const EmailVerificationStatus$json = {
  '1': 'EmailVerificationStatus',
  '2': [
    {'1': 'verified', '3': 1, '4': 1, '5': 8, '10': 'verified'},
    {
      '1': 'verified_at',
      '3': 2,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'verifiedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_verified_at'},
  ],
};

/// Descriptor for `EmailVerificationStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emailVerificationStatusDescriptor = $convert.base64Decode(
    'ChdFbWFpbFZlcmlmaWNhdGlvblN0YXR1cxIaCgh2ZXJpZmllZBgBIAEoCFIIdmVyaWZpZWQSJA'
    'oLdmVyaWZpZWRfYXQYAiABKANIAFIKdmVyaWZpZWRBdIgBAUIOCgxfdmVyaWZpZWRfYXQ=');

@$core.Deprecated('Use getTwoFactorStatusRequestDescriptor instead')
const GetTwoFactorStatusRequest$json = {
  '1': 'GetTwoFactorStatusRequest',
};

/// Descriptor for `GetTwoFactorStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTwoFactorStatusRequestDescriptor =
    $convert.base64Decode('ChlHZXRUd29GYWN0b3JTdGF0dXNSZXF1ZXN0');

@$core.Deprecated('Use twoFactorStatusDescriptor instead')
const TwoFactorStatus$json = {
  '1': 'TwoFactorStatus',
  '2': [
    {'1': 'enabled', '3': 1, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'configured', '3': 2, '4': 1, '5': 8, '10': 'configured'},
  ],
};

/// Descriptor for `TwoFactorStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List twoFactorStatusDescriptor = $convert.base64Decode(
    'Cg9Ud29GYWN0b3JTdGF0dXMSGAoHZW5hYmxlZBgBIAEoCFIHZW5hYmxlZBIeCgpjb25maWd1cm'
    'VkGAIgASgIUgpjb25maWd1cmVk');

@$core.Deprecated('Use setupTwoFactorRequestDescriptor instead')
const SetupTwoFactorRequest$json = {
  '1': 'SetupTwoFactorRequest',
  '2': [
    {'1': 'password', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'password'},
  ],
};

/// Descriptor for `SetupTwoFactorRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setupTwoFactorRequestDescriptor = $convert.base64Decode(
    'ChVTZXR1cFR3b0ZhY3RvclJlcXVlc3QSIwoIcGFzc3dvcmQYASABKAlCB7pIBHICEAFSCHBhc3'
    'N3b3Jk');

@$core.Deprecated('Use twoFactorSetupDescriptor instead')
const TwoFactorSetup$json = {
  '1': 'TwoFactorSetup',
  '2': [
    {'1': 'secret', '3': 1, '4': 1, '5': 9, '10': 'secret'},
    {'1': 'otpauth_url', '3': 2, '4': 1, '5': 9, '10': 'otpauthUrl'},
    {'1': 'recovery_codes', '3': 3, '4': 3, '5': 9, '10': 'recoveryCodes'},
  ],
};

/// Descriptor for `TwoFactorSetup`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List twoFactorSetupDescriptor = $convert.base64Decode(
    'Cg5Ud29GYWN0b3JTZXR1cBIWCgZzZWNyZXQYASABKAlSBnNlY3JldBIfCgtvdHBhdXRoX3VybB'
    'gCIAEoCVIKb3RwYXV0aFVybBIlCg5yZWNvdmVyeV9jb2RlcxgDIAMoCVINcmVjb3ZlcnlDb2Rl'
    'cw==');

@$core.Deprecated('Use enableTwoFactorRequestDescriptor instead')
const EnableTwoFactorRequest$json = {
  '1': 'EnableTwoFactorRequest',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'code'},
  ],
};

/// Descriptor for `EnableTwoFactorRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List enableTwoFactorRequestDescriptor =
    $convert.base64Decode(
        'ChZFbmFibGVUd29GYWN0b3JSZXF1ZXN0Eh0KBGNvZGUYASABKAlCCbpIBnIEEAYYClIEY29kZQ'
        '==');

@$core.Deprecated('Use disableTwoFactorRequestDescriptor instead')
const DisableTwoFactorRequest$json = {
  '1': 'DisableTwoFactorRequest',
  '2': [
    {'1': 'password', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'password'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'code'},
  ],
};

/// Descriptor for `DisableTwoFactorRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List disableTwoFactorRequestDescriptor =
    $convert.base64Decode(
        'ChdEaXNhYmxlVHdvRmFjdG9yUmVxdWVzdBIjCghwYXNzd29yZBgBIAEoCUIHukgEcgIQAVIIcG'
        'Fzc3dvcmQSHQoEY29kZRgCIAEoCUIJukgGcgQQBhgKUgRjb2Rl');

@$core.Deprecated('Use regenerateRecoveryCodesRequestDescriptor instead')
const RegenerateRecoveryCodesRequest$json = {
  '1': 'RegenerateRecoveryCodesRequest',
  '2': [
    {'1': 'password', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'password'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'code'},
  ],
};

/// Descriptor for `RegenerateRecoveryCodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List regenerateRecoveryCodesRequestDescriptor =
    $convert.base64Decode(
        'Ch5SZWdlbmVyYXRlUmVjb3ZlcnlDb2Rlc1JlcXVlc3QSIwoIcGFzc3dvcmQYASABKAlCB7pIBH'
        'ICEAFSCHBhc3N3b3JkEh0KBGNvZGUYAiABKAlCCbpIBnIEEAYYClIEY29kZQ==');

@$core.Deprecated('Use recoveryCodesDescriptor instead')
const RecoveryCodes$json = {
  '1': 'RecoveryCodes',
  '2': [
    {'1': 'recovery_codes', '3': 1, '4': 3, '5': 9, '10': 'recoveryCodes'},
  ],
};

/// Descriptor for `RecoveryCodes`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List recoveryCodesDescriptor = $convert.base64Decode(
    'Cg1SZWNvdmVyeUNvZGVzEiUKDnJlY292ZXJ5X2NvZGVzGAEgAygJUg1yZWNvdmVyeUNvZGVz');

@$core.Deprecated('Use listSessionsRequestDescriptor instead')
const ListSessionsRequest$json = {
  '1': 'ListSessionsRequest',
};

/// Descriptor for `ListSessionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listSessionsRequestDescriptor =
    $convert.base64Decode('ChNMaXN0U2Vzc2lvbnNSZXF1ZXN0');

@$core.Deprecated('Use listSessionsResponseDescriptor instead')
const ListSessionsResponse$json = {
  '1': 'ListSessionsResponse',
  '2': [
    {
      '1': 'sessions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.AuthSession',
      '10': 'sessions'
    },
  ],
};

/// Descriptor for `ListSessionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listSessionsResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0U2Vzc2lvbnNSZXNwb25zZRI5CghzZXNzaW9ucxgBIAMoCzIdLm9wZW5veGlkZS5hcG'
    'kudjEuQXV0aFNlc3Npb25SCHNlc3Npb25z');

@$core.Deprecated('Use authSessionDescriptor instead')
const AuthSession$json = {
  '1': 'AuthSession',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'created_at', '3': 2, '4': 1, '5': 3, '10': 'createdAt'},
    {
      '1': 'expires_at',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'expiresAt',
      '17': true
    },
    {'1': 'active', '3': 4, '4': 1, '5': 8, '10': 'active'},
    {'1': 'current', '3': 5, '4': 1, '5': 8, '10': 'current'},
  ],
  '8': [
    {'1': '_expires_at'},
  ],
};

/// Descriptor for `AuthSession`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authSessionDescriptor = $convert.base64Decode(
    'CgtBdXRoU2Vzc2lvbhIdCgpzZXNzaW9uX2lkGAEgASgJUglzZXNzaW9uSWQSHQoKY3JlYXRlZF'
    '9hdBgCIAEoA1IJY3JlYXRlZEF0EiIKCmV4cGlyZXNfYXQYAyABKANIAFIJZXhwaXJlc0F0iAEB'
    'EhYKBmFjdGl2ZRgEIAEoCFIGYWN0aXZlEhgKB2N1cnJlbnQYBSABKAhSB2N1cnJlbnRCDQoLX2'
    'V4cGlyZXNfYXQ=');

@$core.Deprecated('Use revokeSessionRequestDescriptor instead')
const RevokeSessionRequest$json = {
  '1': 'RevokeSessionRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'sessionId'},
  ],
};

/// Descriptor for `RevokeSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List revokeSessionRequestDescriptor = $convert.base64Decode(
    'ChRSZXZva2VTZXNzaW9uUmVxdWVzdBImCgpzZXNzaW9uX2lkGAEgASgJQge6SARyAhABUglzZX'
    'NzaW9uSWQ=');

@$core.Deprecated('Use listPersonalAccessTokensRequestDescriptor instead')
const ListPersonalAccessTokensRequest$json = {
  '1': 'ListPersonalAccessTokensRequest',
};

/// Descriptor for `ListPersonalAccessTokensRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPersonalAccessTokensRequestDescriptor =
    $convert.base64Decode('Ch9MaXN0UGVyc29uYWxBY2Nlc3NUb2tlbnNSZXF1ZXN0');

@$core.Deprecated('Use listPersonalAccessTokensResponseDescriptor instead')
const ListPersonalAccessTokensResponse$json = {
  '1': 'ListPersonalAccessTokensResponse',
  '2': [
    {
      '1': 'tokens',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.openoxide.api.v1.PersonalAccessToken',
      '10': 'tokens'
    },
  ],
};

/// Descriptor for `ListPersonalAccessTokensResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPersonalAccessTokensResponseDescriptor =
    $convert.base64Decode(
        'CiBMaXN0UGVyc29uYWxBY2Nlc3NUb2tlbnNSZXNwb25zZRI9CgZ0b2tlbnMYASADKAsyJS5vcG'
        'Vub3hpZGUuYXBpLnYxLlBlcnNvbmFsQWNjZXNzVG9rZW5SBnRva2Vucw==');

@$core.Deprecated('Use createPersonalAccessTokenRequestDescriptor instead')
const CreatePersonalAccessTokenRequest$json = {
  '1': 'CreatePersonalAccessTokenRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '8': {}, '10': 'name'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '8': {}, '10': 'password'},
    {
      '1': 'expires_at',
      '3': 3,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'expiresAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_expires_at'},
  ],
};

/// Descriptor for `CreatePersonalAccessTokenRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPersonalAccessTokenRequestDescriptor =
    $convert.base64Decode(
        'CiBDcmVhdGVQZXJzb25hbEFjY2Vzc1Rva2VuUmVxdWVzdBIeCgRuYW1lGAEgASgJQgq6SAdyBR'
        'ABGP8BUgRuYW1lEiMKCHBhc3N3b3JkGAIgASgJQge6SARyAhABUghwYXNzd29yZBIiCgpleHBp'
        'cmVzX2F0GAMgASgDSABSCWV4cGlyZXNBdIgBAUINCgtfZXhwaXJlc19hdA==');

@$core.Deprecated('Use personalAccessTokenDescriptor instead')
const PersonalAccessToken$json = {
  '1': 'PersonalAccessToken',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'token_prefix', '3': 3, '4': 1, '5': 9, '10': 'tokenPrefix'},
    {
      '1': 'expires_at',
      '3': 4,
      '4': 1,
      '5': 3,
      '9': 0,
      '10': 'expiresAt',
      '17': true
    },
    {
      '1': 'last_used_at',
      '3': 5,
      '4': 1,
      '5': 3,
      '9': 1,
      '10': 'lastUsedAt',
      '17': true
    },
    {
      '1': 'revoked_at',
      '3': 6,
      '4': 1,
      '5': 3,
      '9': 2,
      '10': 'revokedAt',
      '17': true
    },
    {'1': 'created_at', '3': 7, '4': 1, '5': 3, '10': 'createdAt'},
  ],
  '8': [
    {'1': '_expires_at'},
    {'1': '_last_used_at'},
    {'1': '_revoked_at'},
  ],
};

/// Descriptor for `PersonalAccessToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personalAccessTokenDescriptor = $convert.base64Decode(
    'ChNQZXJzb25hbEFjY2Vzc1Rva2VuEg4KAmlkGAEgASgDUgJpZBISCgRuYW1lGAIgASgJUgRuYW'
    '1lEiEKDHRva2VuX3ByZWZpeBgDIAEoCVILdG9rZW5QcmVmaXgSIgoKZXhwaXJlc19hdBgEIAEo'
    'A0gAUglleHBpcmVzQXSIAQESJQoMbGFzdF91c2VkX2F0GAUgASgDSAFSCmxhc3RVc2VkQXSIAQ'
    'ESIgoKcmV2b2tlZF9hdBgGIAEoA0gCUglyZXZva2VkQXSIAQESHQoKY3JlYXRlZF9hdBgHIAEo'
    'A1IJY3JlYXRlZEF0Qg0KC19leHBpcmVzX2F0Qg8KDV9sYXN0X3VzZWRfYXRCDQoLX3Jldm9rZW'
    'RfYXQ=');

@$core.Deprecated('Use createdPersonalAccessTokenDescriptor instead')
const CreatedPersonalAccessToken$json = {
  '1': 'CreatedPersonalAccessToken',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 9, '10': 'token'},
    {
      '1': 'details',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.openoxide.api.v1.PersonalAccessToken',
      '10': 'details'
    },
  ],
};

/// Descriptor for `CreatedPersonalAccessToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createdPersonalAccessTokenDescriptor =
    $convert.base64Decode(
        'ChpDcmVhdGVkUGVyc29uYWxBY2Nlc3NUb2tlbhIUCgV0b2tlbhgBIAEoCVIFdG9rZW4SPwoHZG'
        'V0YWlscxgCIAEoCzIlLm9wZW5veGlkZS5hcGkudjEuUGVyc29uYWxBY2Nlc3NUb2tlblIHZGV0'
        'YWlscw==');

@$core.Deprecated('Use revokePersonalAccessTokenRequestDescriptor instead')
const RevokePersonalAccessTokenRequest$json = {
  '1': 'RevokePersonalAccessTokenRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 3, '8': {}, '10': 'id'},
  ],
};

/// Descriptor for `RevokePersonalAccessTokenRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List revokePersonalAccessTokenRequestDescriptor =
    $convert.base64Decode(
        'CiBSZXZva2VQZXJzb25hbEFjY2Vzc1Rva2VuUmVxdWVzdBIXCgJpZBgBIAEoA0IHukgEIgIgAF'
        'ICaWQ=');

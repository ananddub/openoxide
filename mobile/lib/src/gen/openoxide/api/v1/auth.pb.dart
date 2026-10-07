// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/auth.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'common.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class SignupRequest extends $pb.GeneratedMessage {
  factory SignupRequest({
    $core.String? email,
    $core.String? password,
    $core.String? firstName,
    $core.String? lastName,
    $core.String? avatar,
  }) {
    final result = SignupRequest._();
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    if (firstName != null) result.firstName = firstName;
    if (lastName != null) result.lastName = lastName;
    if (avatar != null) result.avatar = avatar;
    return result;
  }

  SignupRequest._();

  factory SignupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SignupRequest()..mergeFromBuffer(data, registry);
  factory SignupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SignupRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SignupRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SignupRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..aOS(3, _omitFieldNames ? '' : 'firstName')
    ..aOS(4, _omitFieldNames ? '' : 'lastName')
    ..aOS(5, _omitFieldNames ? '' : 'avatar')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SignupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SignupRequest copyWith(void Function(SignupRequest) updates) =>
      super.copyWith((message) => updates(message as SignupRequest))
          as SignupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SignupRequest() / SignupRequest.new instead')
  static SignupRequest create() => SignupRequest._();
  static $pb.GeneratedMessage $_createMessage() => SignupRequest._();
  @$core.override
  SignupRequest createEmptyInstance() => SignupRequest._();
  @$core.pragma('dart2js:noInline')
  static SignupRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SignupRequest>(
          SignupRequest.$_createMessage);
  static SignupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get firstName => $_getSZ(2);
  @$pb.TagNumber(3)
  set firstName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFirstName() => $_has(2);
  @$pb.TagNumber(3)
  void clearFirstName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get lastName => $_getSZ(3);
  @$pb.TagNumber(4)
  set lastName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLastName() => $_has(3);
  @$pb.TagNumber(4)
  void clearLastName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get avatar => $_getSZ(4);
  @$pb.TagNumber(5)
  set avatar($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAvatar() => $_has(4);
  @$pb.TagNumber(5)
  void clearAvatar() => $_clearField(5);
}

class LoginRequest extends $pb.GeneratedMessage {
  factory LoginRequest({
    $core.String? email,
    $core.String? password,
    $core.String? twoFactorCode,
    $core.String? recoveryCode,
  }) {
    final result = LoginRequest._();
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    if (twoFactorCode != null) result.twoFactorCode = twoFactorCode;
    if (recoveryCode != null) result.recoveryCode = recoveryCode;
    return result;
  }

  LoginRequest._();

  factory LoginRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LoginRequest()..mergeFromBuffer(data, registry);
  factory LoginRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LoginRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LoginRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: LoginRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..aOS(3, _omitFieldNames ? '' : 'twoFactorCode')
    ..aOS(4, _omitFieldNames ? '' : 'recoveryCode')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LoginRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LoginRequest copyWith(void Function(LoginRequest) updates) =>
      super.copyWith((message) => updates(message as LoginRequest))
          as LoginRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use LoginRequest() / LoginRequest.new instead')
  static LoginRequest create() => LoginRequest._();
  static $pb.GeneratedMessage $_createMessage() => LoginRequest._();
  @$core.override
  LoginRequest createEmptyInstance() => LoginRequest._();
  @$core.pragma('dart2js:noInline')
  static LoginRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LoginRequest>(
          LoginRequest.$_createMessage);
  static LoginRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get twoFactorCode => $_getSZ(2);
  @$pb.TagNumber(3)
  set twoFactorCode($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTwoFactorCode() => $_has(2);
  @$pb.TagNumber(3)
  void clearTwoFactorCode() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get recoveryCode => $_getSZ(3);
  @$pb.TagNumber(4)
  set recoveryCode($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRecoveryCode() => $_has(3);
  @$pb.TagNumber(4)
  void clearRecoveryCode() => $_clearField(4);
}

class RefreshRequest extends $pb.GeneratedMessage {
  factory RefreshRequest({
    $core.String? refreshToken,
  }) {
    final result = RefreshRequest._();
    if (refreshToken != null) result.refreshToken = refreshToken;
    return result;
  }

  RefreshRequest._();

  factory RefreshRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RefreshRequest()..mergeFromBuffer(data, registry);
  factory RefreshRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RefreshRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RefreshRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RefreshRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'refreshToken')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RefreshRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RefreshRequest copyWith(void Function(RefreshRequest) updates) =>
      super.copyWith((message) => updates(message as RefreshRequest))
          as RefreshRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use RefreshRequest() / RefreshRequest.new instead')
  static RefreshRequest create() => RefreshRequest._();
  static $pb.GeneratedMessage $_createMessage() => RefreshRequest._();
  @$core.override
  RefreshRequest createEmptyInstance() => RefreshRequest._();
  @$core.pragma('dart2js:noInline')
  static RefreshRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RefreshRequest>(
          RefreshRequest.$_createMessage);
  static RefreshRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get refreshToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set refreshToken($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRefreshToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearRefreshToken() => $_clearField(1);
}

class AuthResponse extends $pb.GeneratedMessage {
  factory AuthResponse({
    $1.User? user,
    $1.TokenPair? tokens,
  }) {
    final result = AuthResponse._();
    if (user != null) result.user = user;
    if (tokens != null) result.tokens = tokens;
    return result;
  }

  AuthResponse._();

  factory AuthResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuthResponse()..mergeFromBuffer(data, registry);
  factory AuthResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuthResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuthResponse.$_createMessage)
    ..aOM<$1.User>(1, _omitFieldNames ? '' : 'user',
        subBuilder: $1.User.$_createMessage)
    ..aOM<$1.TokenPair>(2, _omitFieldNames ? '' : 'tokens',
        subBuilder: $1.TokenPair.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResponse copyWith(void Function(AuthResponse) updates) =>
      super.copyWith((message) => updates(message as AuthResponse))
          as AuthResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AuthResponse() / AuthResponse.new instead')
  static AuthResponse create() => AuthResponse._();
  static $pb.GeneratedMessage $_createMessage() => AuthResponse._();
  @$core.override
  AuthResponse createEmptyInstance() => AuthResponse._();
  @$core.pragma('dart2js:noInline')
  static AuthResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AuthResponse>(
          AuthResponse.$_createMessage);
  static AuthResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $1.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($1.User value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => $_clearField(1);
  @$pb.TagNumber(1)
  $1.User ensureUser() => $_ensure(0);

  @$pb.TagNumber(2)
  $1.TokenPair get tokens => $_getN(1);
  @$pb.TagNumber(2)
  set tokens($1.TokenPair value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasTokens() => $_has(1);
  @$pb.TagNumber(2)
  void clearTokens() => $_clearField(2);
  @$pb.TagNumber(2)
  $1.TokenPair ensureTokens() => $_ensure(1);
}

class WhoAmIRequest extends $pb.GeneratedMessage {
  factory WhoAmIRequest() => WhoAmIRequest._();

  WhoAmIRequest._();

  factory WhoAmIRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      WhoAmIRequest()..mergeFromBuffer(data, registry);
  factory WhoAmIRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      WhoAmIRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WhoAmIRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: WhoAmIRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WhoAmIRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WhoAmIRequest copyWith(void Function(WhoAmIRequest) updates) =>
      super.copyWith((message) => updates(message as WhoAmIRequest))
          as WhoAmIRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use WhoAmIRequest() / WhoAmIRequest.new instead')
  static WhoAmIRequest create() => WhoAmIRequest._();
  static $pb.GeneratedMessage $_createMessage() => WhoAmIRequest._();
  @$core.override
  WhoAmIRequest createEmptyInstance() => WhoAmIRequest._();
  @$core.pragma('dart2js:noInline')
  static WhoAmIRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<WhoAmIRequest>(
          WhoAmIRequest.$_createMessage);
  static WhoAmIRequest? _defaultInstance;
}

class LogoutRequest extends $pb.GeneratedMessage {
  factory LogoutRequest() => LogoutRequest._();

  LogoutRequest._();

  factory LogoutRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogoutRequest()..mergeFromBuffer(data, registry);
  factory LogoutRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      LogoutRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogoutRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: LogoutRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest copyWith(void Function(LogoutRequest) updates) =>
      super.copyWith((message) => updates(message as LogoutRequest))
          as LogoutRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use LogoutRequest() / LogoutRequest.new instead')
  static LogoutRequest create() => LogoutRequest._();
  static $pb.GeneratedMessage $_createMessage() => LogoutRequest._();
  @$core.override
  LogoutRequest createEmptyInstance() => LogoutRequest._();
  @$core.pragma('dart2js:noInline')
  static LogoutRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogoutRequest>(
          LogoutRequest.$_createMessage);
  static LogoutRequest? _defaultInstance;
}

class GetSetupStatusRequest extends $pb.GeneratedMessage {
  factory GetSetupStatusRequest() => GetSetupStatusRequest._();

  GetSetupStatusRequest._();

  factory GetSetupStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetSetupStatusRequest()..mergeFromBuffer(data, registry);
  factory GetSetupStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetSetupStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetSetupStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetSetupStatusRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetSetupStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetSetupStatusRequest copyWith(
          void Function(GetSetupStatusRequest) updates) =>
      super.copyWith((message) => updates(message as GetSetupStatusRequest))
          as GetSetupStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetSetupStatusRequest() / GetSetupStatusRequest.new instead')
  static GetSetupStatusRequest create() => GetSetupStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() => GetSetupStatusRequest._();
  @$core.override
  GetSetupStatusRequest createEmptyInstance() => GetSetupStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static GetSetupStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetSetupStatusRequest>(
          GetSetupStatusRequest.$_createMessage);
  static GetSetupStatusRequest? _defaultInstance;
}

class SetupStatus extends $pb.GeneratedMessage {
  factory SetupStatus({
    $core.bool? ownerPresent,
  }) {
    final result = SetupStatus._();
    if (ownerPresent != null) result.ownerPresent = ownerPresent;
    return result;
  }

  SetupStatus._();

  factory SetupStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetupStatus()..mergeFromBuffer(data, registry);
  factory SetupStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetupStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetupStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetupStatus.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'ownerPresent')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetupStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetupStatus copyWith(void Function(SetupStatus) updates) =>
      super.copyWith((message) => updates(message as SetupStatus))
          as SetupStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SetupStatus() / SetupStatus.new instead')
  static SetupStatus create() => SetupStatus._();
  static $pb.GeneratedMessage $_createMessage() => SetupStatus._();
  @$core.override
  SetupStatus createEmptyInstance() => SetupStatus._();
  @$core.pragma('dart2js:noInline')
  static SetupStatus getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetupStatus>(
          SetupStatus.$_createMessage);
  static SetupStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get ownerPresent => $_getBF(0);
  @$pb.TagNumber(1)
  set ownerPresent($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOwnerPresent() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwnerPresent() => $_clearField(1);
}

class UpdateUserRequest extends $pb.GeneratedMessage {
  factory UpdateUserRequest({
    $core.String? email,
    $core.String? password,
    $core.String? firstName,
    $core.String? lastName,
    $core.String? avatar,
  }) {
    final result = UpdateUserRequest._();
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    if (firstName != null) result.firstName = firstName;
    if (lastName != null) result.lastName = lastName;
    if (avatar != null) result.avatar = avatar;
    return result;
  }

  UpdateUserRequest._();

  factory UpdateUserRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateUserRequest()..mergeFromBuffer(data, registry);
  factory UpdateUserRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      UpdateUserRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UpdateUserRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: UpdateUserRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..aOS(3, _omitFieldNames ? '' : 'firstName')
    ..aOS(4, _omitFieldNames ? '' : 'lastName')
    ..aOS(5, _omitFieldNames ? '' : 'avatar')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateUserRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UpdateUserRequest copyWith(void Function(UpdateUserRequest) updates) =>
      super.copyWith((message) => updates(message as UpdateUserRequest))
          as UpdateUserRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use UpdateUserRequest() / UpdateUserRequest.new instead')
  static UpdateUserRequest create() => UpdateUserRequest._();
  static $pb.GeneratedMessage $_createMessage() => UpdateUserRequest._();
  @$core.override
  UpdateUserRequest createEmptyInstance() => UpdateUserRequest._();
  @$core.pragma('dart2js:noInline')
  static UpdateUserRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateUserRequest>(
          UpdateUserRequest.$_createMessage);
  static UpdateUserRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get firstName => $_getSZ(2);
  @$pb.TagNumber(3)
  set firstName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFirstName() => $_has(2);
  @$pb.TagNumber(3)
  void clearFirstName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get lastName => $_getSZ(3);
  @$pb.TagNumber(4)
  set lastName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLastName() => $_has(3);
  @$pb.TagNumber(4)
  void clearLastName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get avatar => $_getSZ(4);
  @$pb.TagNumber(5)
  set avatar($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAvatar() => $_has(4);
  @$pb.TagNumber(5)
  void clearAvatar() => $_clearField(5);
}

class RequestPasswordResetRequest extends $pb.GeneratedMessage {
  factory RequestPasswordResetRequest({
    $core.String? email,
  }) {
    final result = RequestPasswordResetRequest._();
    if (email != null) result.email = email;
    return result;
  }

  RequestPasswordResetRequest._();

  factory RequestPasswordResetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RequestPasswordResetRequest()..mergeFromBuffer(data, registry);
  factory RequestPasswordResetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RequestPasswordResetRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RequestPasswordResetRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RequestPasswordResetRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestPasswordResetRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestPasswordResetRequest copyWith(
          void Function(RequestPasswordResetRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RequestPasswordResetRequest))
          as RequestPasswordResetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RequestPasswordResetRequest() / RequestPasswordResetRequest.new instead')
  static RequestPasswordResetRequest create() =>
      RequestPasswordResetRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RequestPasswordResetRequest._();
  @$core.override
  RequestPasswordResetRequest createEmptyInstance() =>
      RequestPasswordResetRequest._();
  @$core.pragma('dart2js:noInline')
  static RequestPasswordResetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RequestPasswordResetRequest>(
          RequestPasswordResetRequest.$_createMessage);
  static RequestPasswordResetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);
}

class ResetPasswordRequest extends $pb.GeneratedMessage {
  factory ResetPasswordRequest({
    $core.String? token,
    $core.String? password,
  }) {
    final result = ResetPasswordRequest._();
    if (token != null) result.token = token;
    if (password != null) result.password = password;
    return result;
  }

  ResetPasswordRequest._();

  factory ResetPasswordRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResetPasswordRequest()..mergeFromBuffer(data, registry);
  factory ResetPasswordRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ResetPasswordRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ResetPasswordRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ResetPasswordRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResetPasswordRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResetPasswordRequest copyWith(void Function(ResetPasswordRequest) updates) =>
      super.copyWith((message) => updates(message as ResetPasswordRequest))
          as ResetPasswordRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ResetPasswordRequest() / ResetPasswordRequest.new instead')
  static ResetPasswordRequest create() => ResetPasswordRequest._();
  static $pb.GeneratedMessage $_createMessage() => ResetPasswordRequest._();
  @$core.override
  ResetPasswordRequest createEmptyInstance() => ResetPasswordRequest._();
  @$core.pragma('dart2js:noInline')
  static ResetPasswordRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ResetPasswordRequest>(
          ResetPasswordRequest.$_createMessage);
  static ResetPasswordRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);
}

class GetEmailVerificationStatusRequest extends $pb.GeneratedMessage {
  factory GetEmailVerificationStatusRequest() =>
      GetEmailVerificationStatusRequest._();

  GetEmailVerificationStatusRequest._();

  factory GetEmailVerificationStatusRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetEmailVerificationStatusRequest()..mergeFromBuffer(data, registry);
  factory GetEmailVerificationStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetEmailVerificationStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetEmailVerificationStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetEmailVerificationStatusRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetEmailVerificationStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetEmailVerificationStatusRequest copyWith(
          void Function(GetEmailVerificationStatusRequest) updates) =>
      super.copyWith((message) =>
              updates(message as GetEmailVerificationStatusRequest))
          as GetEmailVerificationStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetEmailVerificationStatusRequest() / GetEmailVerificationStatusRequest.new instead')
  static GetEmailVerificationStatusRequest create() =>
      GetEmailVerificationStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GetEmailVerificationStatusRequest._();
  @$core.override
  GetEmailVerificationStatusRequest createEmptyInstance() =>
      GetEmailVerificationStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static GetEmailVerificationStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEmailVerificationStatusRequest>(
          GetEmailVerificationStatusRequest.$_createMessage);
  static GetEmailVerificationStatusRequest? _defaultInstance;
}

class RequestEmailVerificationRequest extends $pb.GeneratedMessage {
  factory RequestEmailVerificationRequest() =>
      RequestEmailVerificationRequest._();

  RequestEmailVerificationRequest._();

  factory RequestEmailVerificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RequestEmailVerificationRequest()..mergeFromBuffer(data, registry);
  factory RequestEmailVerificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RequestEmailVerificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RequestEmailVerificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RequestEmailVerificationRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestEmailVerificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestEmailVerificationRequest copyWith(
          void Function(RequestEmailVerificationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RequestEmailVerificationRequest))
          as RequestEmailVerificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RequestEmailVerificationRequest() / RequestEmailVerificationRequest.new instead')
  static RequestEmailVerificationRequest create() =>
      RequestEmailVerificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RequestEmailVerificationRequest._();
  @$core.override
  RequestEmailVerificationRequest createEmptyInstance() =>
      RequestEmailVerificationRequest._();
  @$core.pragma('dart2js:noInline')
  static RequestEmailVerificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RequestEmailVerificationRequest>(
          RequestEmailVerificationRequest.$_createMessage);
  static RequestEmailVerificationRequest? _defaultInstance;
}

class ConfirmEmailVerificationRequest extends $pb.GeneratedMessage {
  factory ConfirmEmailVerificationRequest({
    $core.String? token,
  }) {
    final result = ConfirmEmailVerificationRequest._();
    if (token != null) result.token = token;
    return result;
  }

  ConfirmEmailVerificationRequest._();

  factory ConfirmEmailVerificationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ConfirmEmailVerificationRequest()..mergeFromBuffer(data, registry);
  factory ConfirmEmailVerificationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ConfirmEmailVerificationRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConfirmEmailVerificationRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ConfirmEmailVerificationRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConfirmEmailVerificationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConfirmEmailVerificationRequest copyWith(
          void Function(ConfirmEmailVerificationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ConfirmEmailVerificationRequest))
          as ConfirmEmailVerificationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ConfirmEmailVerificationRequest() / ConfirmEmailVerificationRequest.new instead')
  static ConfirmEmailVerificationRequest create() =>
      ConfirmEmailVerificationRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ConfirmEmailVerificationRequest._();
  @$core.override
  ConfirmEmailVerificationRequest createEmptyInstance() =>
      ConfirmEmailVerificationRequest._();
  @$core.pragma('dart2js:noInline')
  static ConfirmEmailVerificationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConfirmEmailVerificationRequest>(
          ConfirmEmailVerificationRequest.$_createMessage);
  static ConfirmEmailVerificationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);
}

class EmailVerificationStatus extends $pb.GeneratedMessage {
  factory EmailVerificationStatus({
    $core.bool? verified,
    $fixnum.Int64? verifiedAt,
  }) {
    final result = EmailVerificationStatus._();
    if (verified != null) result.verified = verified;
    if (verifiedAt != null) result.verifiedAt = verifiedAt;
    return result;
  }

  EmailVerificationStatus._();

  factory EmailVerificationStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EmailVerificationStatus()..mergeFromBuffer(data, registry);
  factory EmailVerificationStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EmailVerificationStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EmailVerificationStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: EmailVerificationStatus.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'verified')
    ..aInt64(2, _omitFieldNames ? '' : 'verifiedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EmailVerificationStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EmailVerificationStatus copyWith(
          void Function(EmailVerificationStatus) updates) =>
      super.copyWith((message) => updates(message as EmailVerificationStatus))
          as EmailVerificationStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use EmailVerificationStatus() / EmailVerificationStatus.new instead')
  static EmailVerificationStatus create() => EmailVerificationStatus._();
  static $pb.GeneratedMessage $_createMessage() => EmailVerificationStatus._();
  @$core.override
  EmailVerificationStatus createEmptyInstance() => EmailVerificationStatus._();
  @$core.pragma('dart2js:noInline')
  static EmailVerificationStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EmailVerificationStatus>(
          EmailVerificationStatus.$_createMessage);
  static EmailVerificationStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get verified => $_getBF(0);
  @$pb.TagNumber(1)
  set verified($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasVerified() => $_has(0);
  @$pb.TagNumber(1)
  void clearVerified() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get verifiedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set verifiedAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVerifiedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearVerifiedAt() => $_clearField(2);
}

class GetTwoFactorStatusRequest extends $pb.GeneratedMessage {
  factory GetTwoFactorStatusRequest() => GetTwoFactorStatusRequest._();

  GetTwoFactorStatusRequest._();

  factory GetTwoFactorStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTwoFactorStatusRequest()..mergeFromBuffer(data, registry);
  factory GetTwoFactorStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      GetTwoFactorStatusRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetTwoFactorStatusRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: GetTwoFactorStatusRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTwoFactorStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetTwoFactorStatusRequest copyWith(
          void Function(GetTwoFactorStatusRequest) updates) =>
      super.copyWith((message) => updates(message as GetTwoFactorStatusRequest))
          as GetTwoFactorStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use GetTwoFactorStatusRequest() / GetTwoFactorStatusRequest.new instead')
  static GetTwoFactorStatusRequest create() => GetTwoFactorStatusRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      GetTwoFactorStatusRequest._();
  @$core.override
  GetTwoFactorStatusRequest createEmptyInstance() =>
      GetTwoFactorStatusRequest._();
  @$core.pragma('dart2js:noInline')
  static GetTwoFactorStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetTwoFactorStatusRequest>(
          GetTwoFactorStatusRequest.$_createMessage);
  static GetTwoFactorStatusRequest? _defaultInstance;
}

class TwoFactorStatus extends $pb.GeneratedMessage {
  factory TwoFactorStatus({
    $core.bool? enabled,
    $core.bool? configured,
  }) {
    final result = TwoFactorStatus._();
    if (enabled != null) result.enabled = enabled;
    if (configured != null) result.configured = configured;
    return result;
  }

  TwoFactorStatus._();

  factory TwoFactorStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TwoFactorStatus()..mergeFromBuffer(data, registry);
  factory TwoFactorStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TwoFactorStatus()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TwoFactorStatus',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TwoFactorStatus.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'enabled')
    ..aOB(2, _omitFieldNames ? '' : 'configured')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TwoFactorStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TwoFactorStatus copyWith(void Function(TwoFactorStatus) updates) =>
      super.copyWith((message) => updates(message as TwoFactorStatus))
          as TwoFactorStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TwoFactorStatus() / TwoFactorStatus.new instead')
  static TwoFactorStatus create() => TwoFactorStatus._();
  static $pb.GeneratedMessage $_createMessage() => TwoFactorStatus._();
  @$core.override
  TwoFactorStatus createEmptyInstance() => TwoFactorStatus._();
  @$core.pragma('dart2js:noInline')
  static TwoFactorStatus getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TwoFactorStatus>(
          TwoFactorStatus.$_createMessage);
  static TwoFactorStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get enabled => $_getBF(0);
  @$pb.TagNumber(1)
  set enabled($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEnabled() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnabled() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get configured => $_getBF(1);
  @$pb.TagNumber(2)
  set configured($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConfigured() => $_has(1);
  @$pb.TagNumber(2)
  void clearConfigured() => $_clearField(2);
}

class SetupTwoFactorRequest extends $pb.GeneratedMessage {
  factory SetupTwoFactorRequest({
    $core.String? password,
  }) {
    final result = SetupTwoFactorRequest._();
    if (password != null) result.password = password;
    return result;
  }

  SetupTwoFactorRequest._();

  factory SetupTwoFactorRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetupTwoFactorRequest()..mergeFromBuffer(data, registry);
  factory SetupTwoFactorRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SetupTwoFactorRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetupTwoFactorRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: SetupTwoFactorRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetupTwoFactorRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetupTwoFactorRequest copyWith(
          void Function(SetupTwoFactorRequest) updates) =>
      super.copyWith((message) => updates(message as SetupTwoFactorRequest))
          as SetupTwoFactorRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SetupTwoFactorRequest() / SetupTwoFactorRequest.new instead')
  static SetupTwoFactorRequest create() => SetupTwoFactorRequest._();
  static $pb.GeneratedMessage $_createMessage() => SetupTwoFactorRequest._();
  @$core.override
  SetupTwoFactorRequest createEmptyInstance() => SetupTwoFactorRequest._();
  @$core.pragma('dart2js:noInline')
  static SetupTwoFactorRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetupTwoFactorRequest>(
          SetupTwoFactorRequest.$_createMessage);
  static SetupTwoFactorRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get password => $_getSZ(0);
  @$pb.TagNumber(1)
  set password($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPassword() => $_has(0);
  @$pb.TagNumber(1)
  void clearPassword() => $_clearField(1);
}

class TwoFactorSetup extends $pb.GeneratedMessage {
  factory TwoFactorSetup({
    $core.String? secret,
    $core.String? otpauthUrl,
    $core.Iterable<$core.String>? recoveryCodes,
  }) {
    final result = TwoFactorSetup._();
    if (secret != null) result.secret = secret;
    if (otpauthUrl != null) result.otpauthUrl = otpauthUrl;
    if (recoveryCodes != null) result.recoveryCodes.addAll(recoveryCodes);
    return result;
  }

  TwoFactorSetup._();

  factory TwoFactorSetup.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TwoFactorSetup()..mergeFromBuffer(data, registry);
  factory TwoFactorSetup.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TwoFactorSetup()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TwoFactorSetup',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: TwoFactorSetup.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'secret')
    ..aOS(2, _omitFieldNames ? '' : 'otpauthUrl')
    ..pPS(3, _omitFieldNames ? '' : 'recoveryCodes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TwoFactorSetup clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TwoFactorSetup copyWith(void Function(TwoFactorSetup) updates) =>
      super.copyWith((message) => updates(message as TwoFactorSetup))
          as TwoFactorSetup;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TwoFactorSetup() / TwoFactorSetup.new instead')
  static TwoFactorSetup create() => TwoFactorSetup._();
  static $pb.GeneratedMessage $_createMessage() => TwoFactorSetup._();
  @$core.override
  TwoFactorSetup createEmptyInstance() => TwoFactorSetup._();
  @$core.pragma('dart2js:noInline')
  static TwoFactorSetup getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TwoFactorSetup>(
          TwoFactorSetup.$_createMessage);
  static TwoFactorSetup? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get secret => $_getSZ(0);
  @$pb.TagNumber(1)
  set secret($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSecret() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecret() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get otpauthUrl => $_getSZ(1);
  @$pb.TagNumber(2)
  set otpauthUrl($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOtpauthUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearOtpauthUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<$core.String> get recoveryCodes => $_getList(2);
}

class EnableTwoFactorRequest extends $pb.GeneratedMessage {
  factory EnableTwoFactorRequest({
    $core.String? code,
  }) {
    final result = EnableTwoFactorRequest._();
    if (code != null) result.code = code;
    return result;
  }

  EnableTwoFactorRequest._();

  factory EnableTwoFactorRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EnableTwoFactorRequest()..mergeFromBuffer(data, registry);
  factory EnableTwoFactorRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EnableTwoFactorRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EnableTwoFactorRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: EnableTwoFactorRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EnableTwoFactorRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EnableTwoFactorRequest copyWith(
          void Function(EnableTwoFactorRequest) updates) =>
      super.copyWith((message) => updates(message as EnableTwoFactorRequest))
          as EnableTwoFactorRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use EnableTwoFactorRequest() / EnableTwoFactorRequest.new instead')
  static EnableTwoFactorRequest create() => EnableTwoFactorRequest._();
  static $pb.GeneratedMessage $_createMessage() => EnableTwoFactorRequest._();
  @$core.override
  EnableTwoFactorRequest createEmptyInstance() => EnableTwoFactorRequest._();
  @$core.pragma('dart2js:noInline')
  static EnableTwoFactorRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EnableTwoFactorRequest>(
          EnableTwoFactorRequest.$_createMessage);
  static EnableTwoFactorRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);
}

class DisableTwoFactorRequest extends $pb.GeneratedMessage {
  factory DisableTwoFactorRequest({
    $core.String? password,
    $core.String? code,
  }) {
    final result = DisableTwoFactorRequest._();
    if (password != null) result.password = password;
    if (code != null) result.code = code;
    return result;
  }

  DisableTwoFactorRequest._();

  factory DisableTwoFactorRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DisableTwoFactorRequest()..mergeFromBuffer(data, registry);
  factory DisableTwoFactorRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DisableTwoFactorRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DisableTwoFactorRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: DisableTwoFactorRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'password')
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DisableTwoFactorRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DisableTwoFactorRequest copyWith(
          void Function(DisableTwoFactorRequest) updates) =>
      super.copyWith((message) => updates(message as DisableTwoFactorRequest))
          as DisableTwoFactorRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DisableTwoFactorRequest() / DisableTwoFactorRequest.new instead')
  static DisableTwoFactorRequest create() => DisableTwoFactorRequest._();
  static $pb.GeneratedMessage $_createMessage() => DisableTwoFactorRequest._();
  @$core.override
  DisableTwoFactorRequest createEmptyInstance() => DisableTwoFactorRequest._();
  @$core.pragma('dart2js:noInline')
  static DisableTwoFactorRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DisableTwoFactorRequest>(
          DisableTwoFactorRequest.$_createMessage);
  static DisableTwoFactorRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get password => $_getSZ(0);
  @$pb.TagNumber(1)
  set password($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPassword() => $_has(0);
  @$pb.TagNumber(1)
  void clearPassword() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => $_clearField(2);
}

class RegenerateRecoveryCodesRequest extends $pb.GeneratedMessage {
  factory RegenerateRecoveryCodesRequest({
    $core.String? password,
    $core.String? code,
  }) {
    final result = RegenerateRecoveryCodesRequest._();
    if (password != null) result.password = password;
    if (code != null) result.code = code;
    return result;
  }

  RegenerateRecoveryCodesRequest._();

  factory RegenerateRecoveryCodesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RegenerateRecoveryCodesRequest()..mergeFromBuffer(data, registry);
  factory RegenerateRecoveryCodesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RegenerateRecoveryCodesRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RegenerateRecoveryCodesRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RegenerateRecoveryCodesRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'password')
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegenerateRecoveryCodesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegenerateRecoveryCodesRequest copyWith(
          void Function(RegenerateRecoveryCodesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RegenerateRecoveryCodesRequest))
          as RegenerateRecoveryCodesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RegenerateRecoveryCodesRequest() / RegenerateRecoveryCodesRequest.new instead')
  static RegenerateRecoveryCodesRequest create() =>
      RegenerateRecoveryCodesRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RegenerateRecoveryCodesRequest._();
  @$core.override
  RegenerateRecoveryCodesRequest createEmptyInstance() =>
      RegenerateRecoveryCodesRequest._();
  @$core.pragma('dart2js:noInline')
  static RegenerateRecoveryCodesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegenerateRecoveryCodesRequest>(
          RegenerateRecoveryCodesRequest.$_createMessage);
  static RegenerateRecoveryCodesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get password => $_getSZ(0);
  @$pb.TagNumber(1)
  set password($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPassword() => $_has(0);
  @$pb.TagNumber(1)
  void clearPassword() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => $_clearField(2);
}

class RecoveryCodes extends $pb.GeneratedMessage {
  factory RecoveryCodes({
    $core.Iterable<$core.String>? recoveryCodes,
  }) {
    final result = RecoveryCodes._();
    if (recoveryCodes != null) result.recoveryCodes.addAll(recoveryCodes);
    return result;
  }

  RecoveryCodes._();

  factory RecoveryCodes.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RecoveryCodes()..mergeFromBuffer(data, registry);
  factory RecoveryCodes.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RecoveryCodes()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RecoveryCodes',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RecoveryCodes.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'recoveryCodes')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RecoveryCodes clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RecoveryCodes copyWith(void Function(RecoveryCodes) updates) =>
      super.copyWith((message) => updates(message as RecoveryCodes))
          as RecoveryCodes;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use RecoveryCodes() / RecoveryCodes.new instead')
  static RecoveryCodes create() => RecoveryCodes._();
  static $pb.GeneratedMessage $_createMessage() => RecoveryCodes._();
  @$core.override
  RecoveryCodes createEmptyInstance() => RecoveryCodes._();
  @$core.pragma('dart2js:noInline')
  static RecoveryCodes getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RecoveryCodes>(
          RecoveryCodes.$_createMessage);
  static RecoveryCodes? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get recoveryCodes => $_getList(0);
}

class ListSessionsRequest extends $pb.GeneratedMessage {
  factory ListSessionsRequest() => ListSessionsRequest._();

  ListSessionsRequest._();

  factory ListSessionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListSessionsRequest()..mergeFromBuffer(data, registry);
  factory ListSessionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListSessionsRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListSessionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListSessionsRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListSessionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListSessionsRequest copyWith(void Function(ListSessionsRequest) updates) =>
      super.copyWith((message) => updates(message as ListSessionsRequest))
          as ListSessionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ListSessionsRequest() / ListSessionsRequest.new instead')
  static ListSessionsRequest create() => ListSessionsRequest._();
  static $pb.GeneratedMessage $_createMessage() => ListSessionsRequest._();
  @$core.override
  ListSessionsRequest createEmptyInstance() => ListSessionsRequest._();
  @$core.pragma('dart2js:noInline')
  static ListSessionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListSessionsRequest>(
          ListSessionsRequest.$_createMessage);
  static ListSessionsRequest? _defaultInstance;
}

class ListSessionsResponse extends $pb.GeneratedMessage {
  factory ListSessionsResponse({
    $core.Iterable<AuthSession>? sessions,
  }) {
    final result = ListSessionsResponse._();
    if (sessions != null) result.sessions.addAll(sessions);
    return result;
  }

  ListSessionsResponse._();

  factory ListSessionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListSessionsResponse()..mergeFromBuffer(data, registry);
  factory ListSessionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListSessionsResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListSessionsResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListSessionsResponse.$_createMessage)
    ..pPM<AuthSession>(1, _omitFieldNames ? '' : 'sessions',
        subBuilder: AuthSession.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListSessionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListSessionsResponse copyWith(void Function(ListSessionsResponse) updates) =>
      super.copyWith((message) => updates(message as ListSessionsResponse))
          as ListSessionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListSessionsResponse() / ListSessionsResponse.new instead')
  static ListSessionsResponse create() => ListSessionsResponse._();
  static $pb.GeneratedMessage $_createMessage() => ListSessionsResponse._();
  @$core.override
  ListSessionsResponse createEmptyInstance() => ListSessionsResponse._();
  @$core.pragma('dart2js:noInline')
  static ListSessionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListSessionsResponse>(
          ListSessionsResponse.$_createMessage);
  static ListSessionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AuthSession> get sessions => $_getList(0);
}

class AuthSession extends $pb.GeneratedMessage {
  factory AuthSession({
    $core.String? sessionId,
    $fixnum.Int64? createdAt,
    $fixnum.Int64? expiresAt,
    $core.bool? active,
    $core.bool? current,
  }) {
    final result = AuthSession._();
    if (sessionId != null) result.sessionId = sessionId;
    if (createdAt != null) result.createdAt = createdAt;
    if (expiresAt != null) result.expiresAt = expiresAt;
    if (active != null) result.active = active;
    if (current != null) result.current = current;
    return result;
  }

  AuthSession._();

  factory AuthSession.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuthSession()..mergeFromBuffer(data, registry);
  factory AuthSession.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AuthSession()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthSession',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: AuthSession.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aInt64(2, _omitFieldNames ? '' : 'createdAt')
    ..aInt64(3, _omitFieldNames ? '' : 'expiresAt')
    ..aOB(4, _omitFieldNames ? '' : 'active')
    ..aOB(5, _omitFieldNames ? '' : 'current')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthSession clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthSession copyWith(void Function(AuthSession) updates) =>
      super.copyWith((message) => updates(message as AuthSession))
          as AuthSession;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AuthSession() / AuthSession.new instead')
  static AuthSession create() => AuthSession._();
  static $pb.GeneratedMessage $_createMessage() => AuthSession._();
  @$core.override
  AuthSession createEmptyInstance() => AuthSession._();
  @$core.pragma('dart2js:noInline')
  static AuthSession getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AuthSession>(
          AuthSession.$_createMessage);
  static AuthSession? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get createdAt => $_getI64(1);
  @$pb.TagNumber(2)
  set createdAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCreatedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearCreatedAt() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get expiresAt => $_getI64(2);
  @$pb.TagNumber(3)
  set expiresAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasExpiresAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearExpiresAt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get active => $_getBF(3);
  @$pb.TagNumber(4)
  set active($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasActive() => $_has(3);
  @$pb.TagNumber(4)
  void clearActive() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get current => $_getBF(4);
  @$pb.TagNumber(5)
  set current($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCurrent() => $_has(4);
  @$pb.TagNumber(5)
  void clearCurrent() => $_clearField(5);
}

class RevokeSessionRequest extends $pb.GeneratedMessage {
  factory RevokeSessionRequest({
    $core.String? sessionId,
  }) {
    final result = RevokeSessionRequest._();
    if (sessionId != null) result.sessionId = sessionId;
    return result;
  }

  RevokeSessionRequest._();

  factory RevokeSessionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RevokeSessionRequest()..mergeFromBuffer(data, registry);
  factory RevokeSessionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RevokeSessionRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RevokeSessionRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RevokeSessionRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeSessionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeSessionRequest copyWith(void Function(RevokeSessionRequest) updates) =>
      super.copyWith((message) => updates(message as RevokeSessionRequest))
          as RevokeSessionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RevokeSessionRequest() / RevokeSessionRequest.new instead')
  static RevokeSessionRequest create() => RevokeSessionRequest._();
  static $pb.GeneratedMessage $_createMessage() => RevokeSessionRequest._();
  @$core.override
  RevokeSessionRequest createEmptyInstance() => RevokeSessionRequest._();
  @$core.pragma('dart2js:noInline')
  static RevokeSessionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RevokeSessionRequest>(
          RevokeSessionRequest.$_createMessage);
  static RevokeSessionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);
}

class ListPersonalAccessTokensRequest extends $pb.GeneratedMessage {
  factory ListPersonalAccessTokensRequest() =>
      ListPersonalAccessTokensRequest._();

  ListPersonalAccessTokensRequest._();

  factory ListPersonalAccessTokensRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPersonalAccessTokensRequest()..mergeFromBuffer(data, registry);
  factory ListPersonalAccessTokensRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPersonalAccessTokensRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPersonalAccessTokensRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPersonalAccessTokensRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPersonalAccessTokensRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPersonalAccessTokensRequest copyWith(
          void Function(ListPersonalAccessTokensRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListPersonalAccessTokensRequest))
          as ListPersonalAccessTokensRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListPersonalAccessTokensRequest() / ListPersonalAccessTokensRequest.new instead')
  static ListPersonalAccessTokensRequest create() =>
      ListPersonalAccessTokensRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListPersonalAccessTokensRequest._();
  @$core.override
  ListPersonalAccessTokensRequest createEmptyInstance() =>
      ListPersonalAccessTokensRequest._();
  @$core.pragma('dart2js:noInline')
  static ListPersonalAccessTokensRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPersonalAccessTokensRequest>(
          ListPersonalAccessTokensRequest.$_createMessage);
  static ListPersonalAccessTokensRequest? _defaultInstance;
}

class ListPersonalAccessTokensResponse extends $pb.GeneratedMessage {
  factory ListPersonalAccessTokensResponse({
    $core.Iterable<PersonalAccessToken>? tokens,
  }) {
    final result = ListPersonalAccessTokensResponse._();
    if (tokens != null) result.tokens.addAll(tokens);
    return result;
  }

  ListPersonalAccessTokensResponse._();

  factory ListPersonalAccessTokensResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPersonalAccessTokensResponse()..mergeFromBuffer(data, registry);
  factory ListPersonalAccessTokensResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListPersonalAccessTokensResponse()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPersonalAccessTokensResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: ListPersonalAccessTokensResponse.$_createMessage)
    ..pPM<PersonalAccessToken>(1, _omitFieldNames ? '' : 'tokens',
        subBuilder: PersonalAccessToken.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPersonalAccessTokensResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPersonalAccessTokensResponse copyWith(
          void Function(ListPersonalAccessTokensResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListPersonalAccessTokensResponse))
          as ListPersonalAccessTokensResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ListPersonalAccessTokensResponse() / ListPersonalAccessTokensResponse.new instead')
  static ListPersonalAccessTokensResponse create() =>
      ListPersonalAccessTokensResponse._();
  static $pb.GeneratedMessage $_createMessage() =>
      ListPersonalAccessTokensResponse._();
  @$core.override
  ListPersonalAccessTokensResponse createEmptyInstance() =>
      ListPersonalAccessTokensResponse._();
  @$core.pragma('dart2js:noInline')
  static ListPersonalAccessTokensResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPersonalAccessTokensResponse>(
          ListPersonalAccessTokensResponse.$_createMessage);
  static ListPersonalAccessTokensResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PersonalAccessToken> get tokens => $_getList(0);
}

class CreatePersonalAccessTokenRequest extends $pb.GeneratedMessage {
  factory CreatePersonalAccessTokenRequest({
    $core.String? name,
    $core.String? password,
    $fixnum.Int64? expiresAt,
  }) {
    final result = CreatePersonalAccessTokenRequest._();
    if (name != null) result.name = name;
    if (password != null) result.password = password;
    if (expiresAt != null) result.expiresAt = expiresAt;
    return result;
  }

  CreatePersonalAccessTokenRequest._();

  factory CreatePersonalAccessTokenRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePersonalAccessTokenRequest()..mergeFromBuffer(data, registry);
  factory CreatePersonalAccessTokenRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatePersonalAccessTokenRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePersonalAccessTokenRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatePersonalAccessTokenRequest.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..aInt64(3, _omitFieldNames ? '' : 'expiresAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePersonalAccessTokenRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePersonalAccessTokenRequest copyWith(
          void Function(CreatePersonalAccessTokenRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreatePersonalAccessTokenRequest))
          as CreatePersonalAccessTokenRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatePersonalAccessTokenRequest() / CreatePersonalAccessTokenRequest.new instead')
  static CreatePersonalAccessTokenRequest create() =>
      CreatePersonalAccessTokenRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreatePersonalAccessTokenRequest._();
  @$core.override
  CreatePersonalAccessTokenRequest createEmptyInstance() =>
      CreatePersonalAccessTokenRequest._();
  @$core.pragma('dart2js:noInline')
  static CreatePersonalAccessTokenRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePersonalAccessTokenRequest>(
          CreatePersonalAccessTokenRequest.$_createMessage);
  static CreatePersonalAccessTokenRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get expiresAt => $_getI64(2);
  @$pb.TagNumber(3)
  set expiresAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasExpiresAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearExpiresAt() => $_clearField(3);
}

class PersonalAccessToken extends $pb.GeneratedMessage {
  factory PersonalAccessToken({
    $fixnum.Int64? id,
    $core.String? name,
    $core.String? tokenPrefix,
    $fixnum.Int64? expiresAt,
    $fixnum.Int64? lastUsedAt,
    $fixnum.Int64? revokedAt,
    $fixnum.Int64? createdAt,
  }) {
    final result = PersonalAccessToken._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (tokenPrefix != null) result.tokenPrefix = tokenPrefix;
    if (expiresAt != null) result.expiresAt = expiresAt;
    if (lastUsedAt != null) result.lastUsedAt = lastUsedAt;
    if (revokedAt != null) result.revokedAt = revokedAt;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  PersonalAccessToken._();

  factory PersonalAccessToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PersonalAccessToken()..mergeFromBuffer(data, registry);
  factory PersonalAccessToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PersonalAccessToken()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonalAccessToken',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: PersonalAccessToken.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'tokenPrefix')
    ..aInt64(4, _omitFieldNames ? '' : 'expiresAt')
    ..aInt64(5, _omitFieldNames ? '' : 'lastUsedAt')
    ..aInt64(6, _omitFieldNames ? '' : 'revokedAt')
    ..aInt64(7, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonalAccessToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonalAccessToken copyWith(void Function(PersonalAccessToken) updates) =>
      super.copyWith((message) => updates(message as PersonalAccessToken))
          as PersonalAccessToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use PersonalAccessToken() / PersonalAccessToken.new instead')
  static PersonalAccessToken create() => PersonalAccessToken._();
  static $pb.GeneratedMessage $_createMessage() => PersonalAccessToken._();
  @$core.override
  PersonalAccessToken createEmptyInstance() => PersonalAccessToken._();
  @$core.pragma('dart2js:noInline')
  static PersonalAccessToken getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonalAccessToken>(
          PersonalAccessToken.$_createMessage);
  static PersonalAccessToken? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get tokenPrefix => $_getSZ(2);
  @$pb.TagNumber(3)
  set tokenPrefix($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTokenPrefix() => $_has(2);
  @$pb.TagNumber(3)
  void clearTokenPrefix() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get expiresAt => $_getI64(3);
  @$pb.TagNumber(4)
  set expiresAt($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasExpiresAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearExpiresAt() => $_clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get lastUsedAt => $_getI64(4);
  @$pb.TagNumber(5)
  set lastUsedAt($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLastUsedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearLastUsedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get revokedAt => $_getI64(5);
  @$pb.TagNumber(6)
  set revokedAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasRevokedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearRevokedAt() => $_clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get createdAt => $_getI64(6);
  @$pb.TagNumber(7)
  set createdAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCreatedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedAt() => $_clearField(7);
}

class CreatedPersonalAccessToken extends $pb.GeneratedMessage {
  factory CreatedPersonalAccessToken({
    $core.String? token,
    PersonalAccessToken? details,
  }) {
    final result = CreatedPersonalAccessToken._();
    if (token != null) result.token = token;
    if (details != null) result.details = details;
    return result;
  }

  CreatedPersonalAccessToken._();

  factory CreatedPersonalAccessToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatedPersonalAccessToken()..mergeFromBuffer(data, registry);
  factory CreatedPersonalAccessToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CreatedPersonalAccessToken()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatedPersonalAccessToken',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: CreatedPersonalAccessToken.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'token')
    ..aOM<PersonalAccessToken>(2, _omitFieldNames ? '' : 'details',
        subBuilder: PersonalAccessToken.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatedPersonalAccessToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatedPersonalAccessToken copyWith(
          void Function(CreatedPersonalAccessToken) updates) =>
      super.copyWith(
              (message) => updates(message as CreatedPersonalAccessToken))
          as CreatedPersonalAccessToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CreatedPersonalAccessToken() / CreatedPersonalAccessToken.new instead')
  static CreatedPersonalAccessToken create() => CreatedPersonalAccessToken._();
  static $pb.GeneratedMessage $_createMessage() =>
      CreatedPersonalAccessToken._();
  @$core.override
  CreatedPersonalAccessToken createEmptyInstance() =>
      CreatedPersonalAccessToken._();
  @$core.pragma('dart2js:noInline')
  static CreatedPersonalAccessToken getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatedPersonalAccessToken>(
          CreatedPersonalAccessToken.$_createMessage);
  static CreatedPersonalAccessToken? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get token => $_getSZ(0);
  @$pb.TagNumber(1)
  set token($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => $_clearField(1);

  @$pb.TagNumber(2)
  PersonalAccessToken get details => $_getN(1);
  @$pb.TagNumber(2)
  set details(PersonalAccessToken value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasDetails() => $_has(1);
  @$pb.TagNumber(2)
  void clearDetails() => $_clearField(2);
  @$pb.TagNumber(2)
  PersonalAccessToken ensureDetails() => $_ensure(1);
}

class RevokePersonalAccessTokenRequest extends $pb.GeneratedMessage {
  factory RevokePersonalAccessTokenRequest({
    $fixnum.Int64? id,
  }) {
    final result = RevokePersonalAccessTokenRequest._();
    if (id != null) result.id = id;
    return result;
  }

  RevokePersonalAccessTokenRequest._();

  factory RevokePersonalAccessTokenRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RevokePersonalAccessTokenRequest()..mergeFromBuffer(data, registry);
  factory RevokePersonalAccessTokenRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      RevokePersonalAccessTokenRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RevokePersonalAccessTokenRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'openoxide.api.v1'),
      createEmptyInstance: RevokePersonalAccessTokenRequest.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokePersonalAccessTokenRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokePersonalAccessTokenRequest copyWith(
          void Function(RevokePersonalAccessTokenRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RevokePersonalAccessTokenRequest))
          as RevokePersonalAccessTokenRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use RevokePersonalAccessTokenRequest() / RevokePersonalAccessTokenRequest.new instead')
  static RevokePersonalAccessTokenRequest create() =>
      RevokePersonalAccessTokenRequest._();
  static $pb.GeneratedMessage $_createMessage() =>
      RevokePersonalAccessTokenRequest._();
  @$core.override
  RevokePersonalAccessTokenRequest createEmptyInstance() =>
      RevokePersonalAccessTokenRequest._();
  @$core.pragma('dart2js:noInline')
  static RevokePersonalAccessTokenRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RevokePersonalAccessTokenRequest>(
          RevokePersonalAccessTokenRequest.$_createMessage);
  static RevokePersonalAccessTokenRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

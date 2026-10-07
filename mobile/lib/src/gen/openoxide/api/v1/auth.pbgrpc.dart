// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/auth.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'auth.pb.dart' as $0;
import 'common.pb.dart' as $1;

export 'auth.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.AuthService')
class AuthServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AuthServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.AuthResponse> signup(
    $0.SignupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$signup, request, options: options);
  }

  $grpc.ResponseFuture<$0.AuthResponse> login(
    $0.LoginRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$login, request, options: options);
  }

  $grpc.ResponseFuture<$0.AuthResponse> refresh(
    $0.RefreshRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$refresh, request, options: options);
  }

  $grpc.ResponseFuture<$1.User> whoAmI(
    $0.WhoAmIRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$whoAmI, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> logout(
    $0.LogoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$logout, request, options: options);
  }

  $grpc.ResponseFuture<$0.SetupStatus> getSetupStatus(
    $0.GetSetupStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSetupStatus, request, options: options);
  }

  $grpc.ResponseFuture<$1.User> updateUser(
    $0.UpdateUserRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateUser, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> requestPasswordReset(
    $0.RequestPasswordResetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$requestPasswordReset, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> resetPassword(
    $0.ResetPasswordRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$resetPassword, request, options: options);
  }

  $grpc.ResponseFuture<$0.EmailVerificationStatus> getEmailVerificationStatus(
    $0.GetEmailVerificationStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getEmailVerificationStatus, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> requestEmailVerification(
    $0.RequestEmailVerificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$requestEmailVerification, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> confirmEmailVerification(
    $0.ConfirmEmailVerificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$confirmEmailVerification, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.TwoFactorStatus> getTwoFactorStatus(
    $0.GetTwoFactorStatusRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getTwoFactorStatus, request, options: options);
  }

  $grpc.ResponseFuture<$0.TwoFactorSetup> setupTwoFactor(
    $0.SetupTwoFactorRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setupTwoFactor, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> enableTwoFactor(
    $0.EnableTwoFactorRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$enableTwoFactor, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> disableTwoFactor(
    $0.DisableTwoFactorRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$disableTwoFactor, request, options: options);
  }

  $grpc.ResponseFuture<$0.RecoveryCodes> regenerateRecoveryCodes(
    $0.RegenerateRecoveryCodesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$regenerateRecoveryCodes, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.ListSessionsResponse> listSessions(
    $0.ListSessionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listSessions, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> revokeSession(
    $0.RevokeSessionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$revokeSession, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListPersonalAccessTokensResponse>
      listPersonalAccessTokens(
    $0.ListPersonalAccessTokensRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPersonalAccessTokens, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.CreatedPersonalAccessToken> createPersonalAccessToken(
    $0.CreatePersonalAccessTokenRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPersonalAccessToken, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.Empty> revokePersonalAccessToken(
    $0.RevokePersonalAccessTokenRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$revokePersonalAccessToken, request,
        options: options);
  }

  // method descriptors

  static final _$signup = $grpc.ClientMethod<$0.SignupRequest, $0.AuthResponse>(
      '/openoxide.api.v1.AuthService/Signup',
      ($0.SignupRequest value) => value.writeToBuffer(),
      $0.AuthResponse.fromBuffer);
  static final _$login = $grpc.ClientMethod<$0.LoginRequest, $0.AuthResponse>(
      '/openoxide.api.v1.AuthService/Login',
      ($0.LoginRequest value) => value.writeToBuffer(),
      $0.AuthResponse.fromBuffer);
  static final _$refresh =
      $grpc.ClientMethod<$0.RefreshRequest, $0.AuthResponse>(
          '/openoxide.api.v1.AuthService/Refresh',
          ($0.RefreshRequest value) => value.writeToBuffer(),
          $0.AuthResponse.fromBuffer);
  static final _$whoAmI = $grpc.ClientMethod<$0.WhoAmIRequest, $1.User>(
      '/openoxide.api.v1.AuthService/WhoAmI',
      ($0.WhoAmIRequest value) => value.writeToBuffer(),
      $1.User.fromBuffer);
  static final _$logout = $grpc.ClientMethod<$0.LogoutRequest, $1.Empty>(
      '/openoxide.api.v1.AuthService/Logout',
      ($0.LogoutRequest value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
  static final _$getSetupStatus =
      $grpc.ClientMethod<$0.GetSetupStatusRequest, $0.SetupStatus>(
          '/openoxide.api.v1.AuthService/GetSetupStatus',
          ($0.GetSetupStatusRequest value) => value.writeToBuffer(),
          $0.SetupStatus.fromBuffer);
  static final _$updateUser = $grpc.ClientMethod<$0.UpdateUserRequest, $1.User>(
      '/openoxide.api.v1.AuthService/UpdateUser',
      ($0.UpdateUserRequest value) => value.writeToBuffer(),
      $1.User.fromBuffer);
  static final _$requestPasswordReset =
      $grpc.ClientMethod<$0.RequestPasswordResetRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/RequestPasswordReset',
          ($0.RequestPasswordResetRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$resetPassword =
      $grpc.ClientMethod<$0.ResetPasswordRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/ResetPassword',
          ($0.ResetPasswordRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getEmailVerificationStatus = $grpc.ClientMethod<
          $0.GetEmailVerificationStatusRequest, $0.EmailVerificationStatus>(
      '/openoxide.api.v1.AuthService/GetEmailVerificationStatus',
      ($0.GetEmailVerificationStatusRequest value) => value.writeToBuffer(),
      $0.EmailVerificationStatus.fromBuffer);
  static final _$requestEmailVerification =
      $grpc.ClientMethod<$0.RequestEmailVerificationRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/RequestEmailVerification',
          ($0.RequestEmailVerificationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$confirmEmailVerification =
      $grpc.ClientMethod<$0.ConfirmEmailVerificationRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/ConfirmEmailVerification',
          ($0.ConfirmEmailVerificationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$getTwoFactorStatus =
      $grpc.ClientMethod<$0.GetTwoFactorStatusRequest, $0.TwoFactorStatus>(
          '/openoxide.api.v1.AuthService/GetTwoFactorStatus',
          ($0.GetTwoFactorStatusRequest value) => value.writeToBuffer(),
          $0.TwoFactorStatus.fromBuffer);
  static final _$setupTwoFactor =
      $grpc.ClientMethod<$0.SetupTwoFactorRequest, $0.TwoFactorSetup>(
          '/openoxide.api.v1.AuthService/SetupTwoFactor',
          ($0.SetupTwoFactorRequest value) => value.writeToBuffer(),
          $0.TwoFactorSetup.fromBuffer);
  static final _$enableTwoFactor =
      $grpc.ClientMethod<$0.EnableTwoFactorRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/EnableTwoFactor',
          ($0.EnableTwoFactorRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$disableTwoFactor =
      $grpc.ClientMethod<$0.DisableTwoFactorRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/DisableTwoFactor',
          ($0.DisableTwoFactorRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$regenerateRecoveryCodes =
      $grpc.ClientMethod<$0.RegenerateRecoveryCodesRequest, $0.RecoveryCodes>(
          '/openoxide.api.v1.AuthService/RegenerateRecoveryCodes',
          ($0.RegenerateRecoveryCodesRequest value) => value.writeToBuffer(),
          $0.RecoveryCodes.fromBuffer);
  static final _$listSessions =
      $grpc.ClientMethod<$0.ListSessionsRequest, $0.ListSessionsResponse>(
          '/openoxide.api.v1.AuthService/ListSessions',
          ($0.ListSessionsRequest value) => value.writeToBuffer(),
          $0.ListSessionsResponse.fromBuffer);
  static final _$revokeSession =
      $grpc.ClientMethod<$0.RevokeSessionRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/RevokeSession',
          ($0.RevokeSessionRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$listPersonalAccessTokens = $grpc.ClientMethod<
          $0.ListPersonalAccessTokensRequest,
          $0.ListPersonalAccessTokensResponse>(
      '/openoxide.api.v1.AuthService/ListPersonalAccessTokens',
      ($0.ListPersonalAccessTokensRequest value) => value.writeToBuffer(),
      $0.ListPersonalAccessTokensResponse.fromBuffer);
  static final _$createPersonalAccessToken = $grpc.ClientMethod<
          $0.CreatePersonalAccessTokenRequest, $0.CreatedPersonalAccessToken>(
      '/openoxide.api.v1.AuthService/CreatePersonalAccessToken',
      ($0.CreatePersonalAccessTokenRequest value) => value.writeToBuffer(),
      $0.CreatedPersonalAccessToken.fromBuffer);
  static final _$revokePersonalAccessToken =
      $grpc.ClientMethod<$0.RevokePersonalAccessTokenRequest, $1.Empty>(
          '/openoxide.api.v1.AuthService/RevokePersonalAccessToken',
          ($0.RevokePersonalAccessTokenRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.AuthService')
abstract class AuthServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.AuthService';

  AuthServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SignupRequest, $0.AuthResponse>(
        'Signup',
        signup_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SignupRequest.fromBuffer(value),
        ($0.AuthResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LoginRequest, $0.AuthResponse>(
        'Login',
        login_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LoginRequest.fromBuffer(value),
        ($0.AuthResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RefreshRequest, $0.AuthResponse>(
        'Refresh',
        refresh_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RefreshRequest.fromBuffer(value),
        ($0.AuthResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WhoAmIRequest, $1.User>(
        'WhoAmI',
        whoAmI_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WhoAmIRequest.fromBuffer(value),
        ($1.User value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogoutRequest, $1.Empty>(
        'Logout',
        logout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LogoutRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetSetupStatusRequest, $0.SetupStatus>(
        'GetSetupStatus',
        getSetupStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetSetupStatusRequest.fromBuffer(value),
        ($0.SetupStatus value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateUserRequest, $1.User>(
        'UpdateUser',
        updateUser_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.UpdateUserRequest.fromBuffer(value),
        ($1.User value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestPasswordResetRequest, $1.Empty>(
        'RequestPasswordReset',
        requestPasswordReset_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RequestPasswordResetRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ResetPasswordRequest, $1.Empty>(
        'ResetPassword',
        resetPassword_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ResetPasswordRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetEmailVerificationStatusRequest,
            $0.EmailVerificationStatus>(
        'GetEmailVerificationStatus',
        getEmailVerificationStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetEmailVerificationStatusRequest.fromBuffer(value),
        ($0.EmailVerificationStatus value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEmailVerificationRequest, $1.Empty>(
            'RequestEmailVerification',
            requestEmailVerification_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RequestEmailVerificationRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ConfirmEmailVerificationRequest, $1.Empty>(
            'ConfirmEmailVerification',
            confirmEmailVerification_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ConfirmEmailVerificationRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetTwoFactorStatusRequest, $0.TwoFactorStatus>(
            'GetTwoFactorStatus',
            getTwoFactorStatus_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetTwoFactorStatusRequest.fromBuffer(value),
            ($0.TwoFactorStatus value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetupTwoFactorRequest, $0.TwoFactorSetup>(
        'SetupTwoFactor',
        setupTwoFactor_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SetupTwoFactorRequest.fromBuffer(value),
        ($0.TwoFactorSetup value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.EnableTwoFactorRequest, $1.Empty>(
        'EnableTwoFactor',
        enableTwoFactor_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.EnableTwoFactorRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DisableTwoFactorRequest, $1.Empty>(
        'DisableTwoFactor',
        disableTwoFactor_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DisableTwoFactorRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RegenerateRecoveryCodesRequest,
            $0.RecoveryCodes>(
        'RegenerateRecoveryCodes',
        regenerateRecoveryCodes_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RegenerateRecoveryCodesRequest.fromBuffer(value),
        ($0.RecoveryCodes value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListSessionsRequest, $0.ListSessionsResponse>(
            'ListSessions',
            listSessions_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListSessionsRequest.fromBuffer(value),
            ($0.ListSessionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RevokeSessionRequest, $1.Empty>(
        'RevokeSession',
        revokeSession_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RevokeSessionRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListPersonalAccessTokensRequest,
            $0.ListPersonalAccessTokensResponse>(
        'ListPersonalAccessTokens',
        listPersonalAccessTokens_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListPersonalAccessTokensRequest.fromBuffer(value),
        ($0.ListPersonalAccessTokensResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreatePersonalAccessTokenRequest,
            $0.CreatedPersonalAccessToken>(
        'CreatePersonalAccessToken',
        createPersonalAccessToken_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreatePersonalAccessTokenRequest.fromBuffer(value),
        ($0.CreatedPersonalAccessToken value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RevokePersonalAccessTokenRequest, $1.Empty>(
            'RevokePersonalAccessToken',
            revokePersonalAccessToken_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RevokePersonalAccessTokenRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.AuthResponse> signup_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.SignupRequest> $request) async {
    return signup($call, await $request);
  }

  $async.Future<$0.AuthResponse> signup(
      $grpc.ServiceCall call, $0.SignupRequest request);

  $async.Future<$0.AuthResponse> login_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.LoginRequest> $request) async {
    return login($call, await $request);
  }

  $async.Future<$0.AuthResponse> login(
      $grpc.ServiceCall call, $0.LoginRequest request);

  $async.Future<$0.AuthResponse> refresh_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RefreshRequest> $request) async {
    return refresh($call, await $request);
  }

  $async.Future<$0.AuthResponse> refresh(
      $grpc.ServiceCall call, $0.RefreshRequest request);

  $async.Future<$1.User> whoAmI_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.WhoAmIRequest> $request) async {
    return whoAmI($call, await $request);
  }

  $async.Future<$1.User> whoAmI(
      $grpc.ServiceCall call, $0.WhoAmIRequest request);

  $async.Future<$1.Empty> logout_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.LogoutRequest> $request) async {
    return logout($call, await $request);
  }

  $async.Future<$1.Empty> logout(
      $grpc.ServiceCall call, $0.LogoutRequest request);

  $async.Future<$0.SetupStatus> getSetupStatus_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetSetupStatusRequest> $request) async {
    return getSetupStatus($call, await $request);
  }

  $async.Future<$0.SetupStatus> getSetupStatus(
      $grpc.ServiceCall call, $0.GetSetupStatusRequest request);

  $async.Future<$1.User> updateUser_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateUserRequest> $request) async {
    return updateUser($call, await $request);
  }

  $async.Future<$1.User> updateUser(
      $grpc.ServiceCall call, $0.UpdateUserRequest request);

  $async.Future<$1.Empty> requestPasswordReset_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RequestPasswordResetRequest> $request) async {
    return requestPasswordReset($call, await $request);
  }

  $async.Future<$1.Empty> requestPasswordReset(
      $grpc.ServiceCall call, $0.RequestPasswordResetRequest request);

  $async.Future<$1.Empty> resetPassword_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ResetPasswordRequest> $request) async {
    return resetPassword($call, await $request);
  }

  $async.Future<$1.Empty> resetPassword(
      $grpc.ServiceCall call, $0.ResetPasswordRequest request);

  $async.Future<$0.EmailVerificationStatus> getEmailVerificationStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetEmailVerificationStatusRequest> $request) async {
    return getEmailVerificationStatus($call, await $request);
  }

  $async.Future<$0.EmailVerificationStatus> getEmailVerificationStatus(
      $grpc.ServiceCall call, $0.GetEmailVerificationStatusRequest request);

  $async.Future<$1.Empty> requestEmailVerification_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RequestEmailVerificationRequest> $request) async {
    return requestEmailVerification($call, await $request);
  }

  $async.Future<$1.Empty> requestEmailVerification(
      $grpc.ServiceCall call, $0.RequestEmailVerificationRequest request);

  $async.Future<$1.Empty> confirmEmailVerification_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ConfirmEmailVerificationRequest> $request) async {
    return confirmEmailVerification($call, await $request);
  }

  $async.Future<$1.Empty> confirmEmailVerification(
      $grpc.ServiceCall call, $0.ConfirmEmailVerificationRequest request);

  $async.Future<$0.TwoFactorStatus> getTwoFactorStatus_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetTwoFactorStatusRequest> $request) async {
    return getTwoFactorStatus($call, await $request);
  }

  $async.Future<$0.TwoFactorStatus> getTwoFactorStatus(
      $grpc.ServiceCall call, $0.GetTwoFactorStatusRequest request);

  $async.Future<$0.TwoFactorSetup> setupTwoFactor_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetupTwoFactorRequest> $request) async {
    return setupTwoFactor($call, await $request);
  }

  $async.Future<$0.TwoFactorSetup> setupTwoFactor(
      $grpc.ServiceCall call, $0.SetupTwoFactorRequest request);

  $async.Future<$1.Empty> enableTwoFactor_Pre($grpc.ServiceCall $call,
      $async.Future<$0.EnableTwoFactorRequest> $request) async {
    return enableTwoFactor($call, await $request);
  }

  $async.Future<$1.Empty> enableTwoFactor(
      $grpc.ServiceCall call, $0.EnableTwoFactorRequest request);

  $async.Future<$1.Empty> disableTwoFactor_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DisableTwoFactorRequest> $request) async {
    return disableTwoFactor($call, await $request);
  }

  $async.Future<$1.Empty> disableTwoFactor(
      $grpc.ServiceCall call, $0.DisableTwoFactorRequest request);

  $async.Future<$0.RecoveryCodes> regenerateRecoveryCodes_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RegenerateRecoveryCodesRequest> $request) async {
    return regenerateRecoveryCodes($call, await $request);
  }

  $async.Future<$0.RecoveryCodes> regenerateRecoveryCodes(
      $grpc.ServiceCall call, $0.RegenerateRecoveryCodesRequest request);

  $async.Future<$0.ListSessionsResponse> listSessions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListSessionsRequest> $request) async {
    return listSessions($call, await $request);
  }

  $async.Future<$0.ListSessionsResponse> listSessions(
      $grpc.ServiceCall call, $0.ListSessionsRequest request);

  $async.Future<$1.Empty> revokeSession_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RevokeSessionRequest> $request) async {
    return revokeSession($call, await $request);
  }

  $async.Future<$1.Empty> revokeSession(
      $grpc.ServiceCall call, $0.RevokeSessionRequest request);

  $async.Future<$0.ListPersonalAccessTokensResponse>
      listPersonalAccessTokens_Pre($grpc.ServiceCall $call,
          $async.Future<$0.ListPersonalAccessTokensRequest> $request) async {
    return listPersonalAccessTokens($call, await $request);
  }

  $async.Future<$0.ListPersonalAccessTokensResponse> listPersonalAccessTokens(
      $grpc.ServiceCall call, $0.ListPersonalAccessTokensRequest request);

  $async.Future<$0.CreatedPersonalAccessToken> createPersonalAccessToken_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreatePersonalAccessTokenRequest> $request) async {
    return createPersonalAccessToken($call, await $request);
  }

  $async.Future<$0.CreatedPersonalAccessToken> createPersonalAccessToken(
      $grpc.ServiceCall call, $0.CreatePersonalAccessTokenRequest request);

  $async.Future<$1.Empty> revokePersonalAccessToken_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RevokePersonalAccessTokenRequest> $request) async {
    return revokePersonalAccessToken($call, await $request);
  }

  $async.Future<$1.Empty> revokePersonalAccessToken(
      $grpc.ServiceCall call, $0.RevokePersonalAccessTokenRequest request);
}


import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/session/stored_session.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/common.pb.dart' show User;
/// Auth calls: the only public one, the login, and session housekeeping.
mixin AuthSection on PanelApiBase {
  /// Public: no credentials. The sign-in screen gates on it.
  Future<SetupStatus> getSetupStatus() => calls.unary(
    () => authClient.getSetupStatus(GetSetupStatusRequest()),
    allowRetry: false,
  );

  /// Verifies credentials and hands back the session to persist. A login the
  /// panel answers without a token pair counts as a rejection, exactly as an
  /// empty refresh response does.
  Future<StoredSession> signIn({
    required String email,
    required String password,
    String? twoFactorCode,
    String? recoveryCode,
  }) async {
    final response = await calls.unary(
      () => authClient.login(
        LoginRequest(
          email: email,
          password: password,
          twoFactorCode: twoFactorCode,
          recoveryCode: recoveryCode,
        ),
      ),
      allowRetry: false,
    );
    if (!response.hasTokens()) {
      throw const SessionRejected('the panel returned no tokens');
    }
    final tokens = response.tokens;
    final user = response.hasUser() ? response.user : null;
    return StoredSession(
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
      userId: user?.id.toInt(),
      email: _present(user?.email),
      firstName: _present(user?.firstName),
      lastName: _present(user?.lastName),
      avatar: _present(user?.avatar),
    );
  }

  static String? _present(String? value) =>
      value == null || value.isEmpty ? null : value;

  Future<User> whoAmI() =>
      calls.unary(() => authClient.whoAmI(WhoAmIRequest()));

  Future<void> logout() =>
      calls.unary(() => authClient.logout(LogoutRequest()));
}

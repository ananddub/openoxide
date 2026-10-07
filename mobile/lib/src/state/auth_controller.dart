import 'package:flutter/foundation.dart';

import '../api/panel_api.dart';
import '../gen/openoxide/api/v1/organization.pb.dart';
import '../grpc/app_error.dart';
import '../session/session.dart';

/// Where the app stands between launch and a usable screen.
enum AppStage {
  /// Asking the panel whether it exists and whether it has an owner yet.
  probing,

  /// The address is wrong or the panel is down. Retriable.
  unreachable,

  /// The panel answers but nobody owns it: there is no account to sign in to.
  needsSetup,

  signedOut,
  signedIn,
}

/// Owns the sign-in state and the selected organization.
///
/// Both are app-wide because both change what every RPC means: the bearer token
/// and `x-organization-id` are read synchronously by the metadata provider on
/// each call, so a screen never passes them — it asks this object to change them
/// and the next call picks the new values up.
class AuthController extends ChangeNotifier {
  AuthController({required this.session, required this.api});

  final Session session;
  final PanelApi api;

  AppStage _stage = AppStage.probing;
  AppError? _error;
  List<Organization> _organizations = const [];
  bool _busy = false;

  AppStage get stage => _stage;
  AppError? get error => _error;
  List<Organization> get organizations => _organizations;
  bool get busy => _busy;
  bool get hasOrganization => (session.organizationId ?? 0) > 0;
  int? get organizationId => hasOrganization ? session.organizationId : null;

  Organization? get currentOrganization {
    final id = organizationId;
    if (id == null) return null;
    for (final organization in _organizations) {
      if (organization.id.toInt() == id) return organization;
    }
    return null;
  }

  /// One round trip to the public `GetSetupStatus` decides everything: whether
  /// the panel is there at all, whether it is usable yet, and whether a stored
  /// session is worth keeping.
  Future<void> boot() async {
    _stage = AppStage.probing;
    _error = null;
    notifyListeners();
    try {
      final status = await api.getSetupStatus();
      if (!status.ownerPresent) {
        _stage = AppStage.needsSetup;
      } else if (!session.isSignedIn) {
        _stage = AppStage.signedOut;
      } else {
        _stage = AppStage.signedIn;
      }
      notifyListeners();
      // A stored session may predate the organization switcher, and every screen
      // lists resources inside an organization, so resolve one before the shell
      // makes its first list call.
      if (_stage == AppStage.signedIn) await loadOrganizations();
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.unreachable);
    }
  }

  Future<bool> signIn({
    required String email,
    required String password,
    String? twoFactorCode,
    String? recoveryCode,
  }) async {
    if (_busy) return false;
    _busy = true;
    _error = null;
    notifyListeners();
    try {
      final stored = await api.signIn(
        email: email,
        password: password,
        twoFactorCode: twoFactorCode,
        recoveryCode: recoveryCode,
      );
      await session.save(stored);
      _stage = AppStage.signedIn;
      notifyListeners();
      await loadOrganizations();
      return true;
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.signedOut);
      return false;
    } finally {
      _busy = false;
      notifyListeners();
    }
  }

  Future<void> loadOrganizations() async {
    try {
      final response = await api.listOrganizations();
      _organizations = List<Organization>.unmodifiable(response.organizations);
      final selected = organizationId;
      final stillValid =
          selected != null &&
          _organizations.any((o) => o.id.toInt() == selected);
      if (!stillValid && _organizations.isNotEmpty) {
        await selectOrganization(_organizations.first.id.toInt());
      }
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.signedOut);
    }
    notifyListeners();
  }

  Future<void> selectOrganization(int organizationId) async {
    final current = session.current;
    if (current == null) return;
    await session.update(
      (stored) => stored.withOrganization(organizationId),
    );
    notifyListeners();
  }

  Future<void> signOut() async {
    // Best effort: the server-side session is nice to drop, but a phone that
    // cannot reach the panel still has to be able to forget its tokens.
    try {
      await api.logout();
    } on Object {
      // Ignored on purpose.
    }
    await session.signOut();
    _organizations = const [];
    _error = null;
    _stage = AppStage.signedOut;
    notifyListeners();
  }

  /// Records a failure and lands on the stage that still shows something
  /// useful. A rejected session is the one case that changes the tokens
  /// themselves — keeping them would fail the same way on the next launch.
  Future<void> _report(Object cause, {required AppStage fallback}) async {
    final appError = toAppError(cause);
    if (appError is SessionRejected && session.isSignedIn) {
      await session.signOut();
      _organizations = const [];
      _stage = AppStage.signedOut;
    } else {
      _error = appError;
      _stage = session.isSignedIn ? AppStage.signedIn : fallback;
    }
    notifyListeners();
  }
}

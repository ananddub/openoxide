export 'package:mobile/src/core/session/session_vault.dart';
export 'package:mobile/src/core/session/stored_session.dart';
import 'package:mobile/src/core/session/session_vault.dart';
import 'package:mobile/src/core/session/stored_session.dart';

/// The live session.
///
/// Deliberately inert: no listeners, no notifications. The bearer token and
/// organization id are read synchronously by the gRPC metadata provider on each
/// call, and the UI learns about sign-in and organization changes from
/// [AuthBloc], which performs every change through this class.
class Session {
  Session({required this._vault, StoredSession? initial}) : _current = initial;

  final SessionVault _vault;
  StoredSession? _current;

  StoredSession? get current => _current;

  String get accessToken => _current?.accessToken ?? '';

  String get refreshToken => _current?.refreshToken ?? '';

  int? get organizationId => _current?.organizationId;

  bool get isSignedIn => _current != null;

  String get displayName {
    final names = [
      _current?.firstName,
      _current?.lastName,
    ].whereType<String>().where((n) => n.trim().isNotEmpty).join(' ');
    if (names.isNotEmpty) return names;
    return _current?.email ?? '';
  }

  /// Loads the persisted session; false when nothing usable is stored.
  Future<bool> restore() async {
    final stored = StoredSession.fromMap(await _vault.read());
    if (stored == null) return false;
    _current = stored;
    return true;
  }

  Future<void> save(StoredSession session) async {
    _current = session;
    await _vault.write(session.toMap());
  }

  /// Used after a refresh and after switching organization.
  Future<void> update(StoredSession Function(StoredSession) change) async {
    final current = _current;
    if (current == null) return;
    _current = change(current);
    await _vault.write(_current!.toMap());
  }

  Future<void> signOut() async {
    _current = null;
    await _vault.clear();
  }
}

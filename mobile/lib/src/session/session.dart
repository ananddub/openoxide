import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

/// Everything the app needs to talk as the signed-in user.
@immutable
class StoredSession {
  const StoredSession({
    required this.accessToken,
    required this.refreshToken,
    this.organizationId,
    this.userId,
    this.email,
    this.firstName,
    this.lastName,
    this.avatar,
  });

  final String accessToken;
  final String refreshToken;
  final int? organizationId;
  final int? userId;
  final String? email;
  final String? firstName;
  final String? lastName;
  final String? avatar;

  StoredSession copyWith({
    String? accessToken,
    String? refreshToken,
    int? organizationId,
    int? userId,
    String? email,
    String? firstName,
    String? lastName,
    String? avatar,
  }) => StoredSession(
    accessToken: accessToken ?? this.accessToken,
    refreshToken: refreshToken ?? this.refreshToken,
    organizationId: organizationId ?? this.organizationId,
    userId: userId ?? this.userId,
    email: email ?? this.email,
    firstName: firstName ?? this.firstName,
    lastName: lastName ?? this.lastName,
    avatar: avatar ?? this.avatar,
  );

  /// Setting an organization needs its own method: `copyWith` cannot tell
  /// "leave it alone" from "clear it", and the switcher does both.
  StoredSession withOrganization(int? organizationId) => StoredSession(
    accessToken: accessToken,
    refreshToken: refreshToken,
    organizationId: organizationId,
    userId: userId,
    email: email,
    firstName: firstName,
    lastName: lastName,
    avatar: avatar,
  );

  Map<String, String> toMap() => {
    'accessToken': accessToken,
    'refreshToken': refreshToken,
    if (organizationId != null) 'organizationId': '$organizationId',
    if (userId != null) 'userId': '$userId',
    'email': ?email,
    'firstName': ?firstName,
    'lastName': ?lastName,
    'avatar': ?avatar,
  };

  static StoredSession? fromMap(Map<String, String>? map) {
    final access = map?[keyAccessToken] ?? '';
    final refresh = map?[keyRefreshToken] ?? '';
    if (access.isEmpty || refresh.isEmpty) return null;
    return StoredSession(
      accessToken: access,
      refreshToken: refresh,
      organizationId: int.tryParse(map?[keyOrganizationId] ?? ''),
      userId: int.tryParse(map?[keyUserId] ?? ''),
      email: map?[keyEmail],
      firstName: map?[keyFirstName],
      lastName: map?[keyLastName],
      avatar: map?[keyAvatar],
    );
  }

  static const keyAccessToken = 'accessToken';
  static const keyRefreshToken = 'refreshToken';
  static const keyOrganizationId = 'organizationId';
  static const keyUserId = 'userId';
  static const keyEmail = 'email';
  static const keyFirstName = 'firstName';
  static const keyLastName = 'lastName';
  static const keyAvatar = 'avatar';
}

abstract interface class SessionVault {
  Future<Map<String, String>?> read();
  Future<void> write(Map<String, String> session);
  Future<void> clear();
}

/// AndroidKeyStore-backed storage; the tokens never touch SharedPreferences.
class SecureSessionVault implements SessionVault {
  SecureSessionVault([this._storage = const FlutterSecureStorage()]);

  /// Every key this vault owns; anything else in storage belongs to a caller.
  static const String _prefix = 'openoxide.session.';

  final FlutterSecureStorage _storage;

  @override
  Future<Map<String, String>?> read() async {
    final all = await _storage.readAll();
    final session = <String, String>{
      for (final MapEntry(:key, :value) in all.entries)
        if (key.startsWith(_prefix)) key.substring(_prefix.length): value,
    };
    return session.isEmpty ? null : session;
  }

  @override
  Future<void> write(Map<String, String> session) async {
    await clear();
    for (final MapEntry(:key, :value) in session.entries) {
      await _storage.write(key: '$_prefix$key', value: value);
    }
  }

  @override
  Future<void> clear() async {
    final all = await _storage.readAll();
    for (final key in all.keys.where((key) => key.startsWith(_prefix))) {
      await _storage.delete(key: key);
    }
  }
}

class InMemorySessionVault implements SessionVault {
  Map<String, String>? _data;

  @override
  Future<Map<String, String>?> read() async =>
      _data == null ? null : {..._data!};

  @override
  Future<void> write(Map<String, String> session) async => _data = {...session};

  @override
  Future<void> clear() async => _data = null;
}

/// The live session, observable by the UI and readable synchronously by the
/// gRPC metadata provider.
class Session extends ChangeNotifier {
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
    notifyListeners();
    return true;
  }

  Future<void> save(StoredSession session) async {
    _current = session;
    await _vault.write(session.toMap());
    notifyListeners();
  }

  /// Used after a refresh and after switching organization.
  Future<void> update(StoredSession Function(StoredSession) change) async {
    final current = _current;
    if (current == null) return;
    _current = change(current);
    await _vault.write(_current!.toMap());
    notifyListeners();
  }

  Future<void> signOut() async {
    _current = null;
    await _vault.clear();
    notifyListeners();
  }
}

import 'package:flutter_secure_storage/flutter_secure_storage.dart';

/// Where a stored session lives.
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

/// A vault in RAM, for tests.
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

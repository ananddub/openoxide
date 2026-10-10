import 'package:flutter/foundation.dart';

/// Compile-time configuration, injected with `--dart-define`.
class AppConfig {
  const AppConfig._();

  /// Host of the panel's ConnectRPC/gRPC listener. `10.0.2.2` is the host
  /// machine as seen from the Android emulator.
  static const String panelHost = String.fromEnvironment(
    'PANEL_URL',
    defaultValue: '10.0.2.2',
  );

  static const int panelPort = int.fromEnvironment(
    'PANEL_PORT',
    defaultValue: 4000,
  );

  /// Dev-only: accept any TLS certificate. The dev stack serves a certificate
  /// valid for `localhost/127.0.0.1/::1` only, so the emulator's `10.0.2.2`
  /// address can never validate against it. Folds to `false` — and the bypass
  /// path with it — in a release build.
  static const bool allowAnyCert =
      bool.fromEnvironment('DEV_ALLOW_ANY_CERT') && !kReleaseMode;

  /// Asset path of the pinned panel certificate (PEM). Set it for a panel whose
  /// TLS certificate no system trust store can validate — a private CA or a
  /// self-signed dev cert reached by an address it was not issued for. Empty
  /// means: rely on the platform trust store only.
  static const String pinnedCertAsset = String.fromEnvironment(
    'PANEL_CERT_ASSET',
  );

  static const String secureStoragePrefix = 'openoxide.mobile';

  static const Duration refreshRetryCooldown = Duration(seconds: 15);
}

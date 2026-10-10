import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:grpc/grpc.dart';
import 'package:mobile/src/core/config/app_config.dart';

/// Opens the single gRPC channel the app talks to the panel over.
///
/// The panel's listener is TLS + HTTP/2 only: it advertises `h2` through ALPN,
/// rejects HTTP/1.1, and has no cleartext fallback, so credentials are always
/// secure and grpc-dart's own ALPN setup is what makes the handshake succeed.
Future<ClientChannel> openPanelChannel() async {
  return ClientChannel(
    AppConfig.panelHost,
    port: AppConfig.panelPort,
    options: ChannelOptions(
      credentials: await panelCredentials(),
      connectTimeout: const Duration(seconds: 10),
      connectionTimeout: const Duration(seconds: 10),
    ),
  );
}

@visibleForTesting
Future<ChannelCredentials> panelCredentials() async {
  final pinnedPem = await _pinnedCertificate();
  return ChannelCredentials.secure(
    // `authority` keeps the SNI name the certificate was issued for, separate
    // from the address we connect to — the emulator reaches the host as an IP.
    authority: AppConfig.panelHost,
    onBadCertificate: (certificate, host) =>
        // Constant-folded to `false` in release, which removes the bypass with
        // it; a release build can only trust a pinned certificate.
        AppConfig.allowAnyCert ||
        (pinnedPem != null &&
            _matchesPinnedCertificate(certificate, pinnedPem)),
  );
}

/// Pinned leaf certificate, when the build was told which asset holds it.
Future<String?> _pinnedCertificate() async {
  if (AppConfig.pinnedCertAsset.isEmpty) return null;
  return (await rootBundle.loadString(AppConfig.pinnedCertAsset))
      .replaceAll(RegExp(r'\s+'), '');
}

bool _matchesPinnedCertificate(
  X509Certificate certificate,
  String normalizedPem,
) {
  final presented = certificate.pem.replaceAll(RegExp(r'\s+'), '');
  // The leaf the server presents must be byte-identical to the bundled one:
  // rotating the dev certificate means re-copying the asset, not loosening this.
  return presented == normalizedPem;
}

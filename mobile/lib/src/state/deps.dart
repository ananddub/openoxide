import 'package:grpc/grpc.dart' show ClientChannel, GrpcError, StatusCode;
import 'package:grpc/service_api.dart' show CallOptions;

import '../api/panel_api.dart';
import '../gen/openoxide/api/v1/auth.pbgrpc.dart';
import '../grpc/auth_coordinator.dart';
import '../grpc/auth_retry.dart';
import '../grpc/channel.dart';
import '../grpc/metadata.dart';
import '../session/session.dart';
import 'realtime_watch.dart';

/// Everything a screen may need, assembled once at startup and handed down with
/// `provider`. Tests build it from an in-memory vault and a fake channel-free
/// API, so no test needs a running panel.
class Deps {
  Deps({
    required this.session,
    required this.api,
    required this.realtime,
    required this.channel,
  });

  final Session session;
  final PanelApi api;
  final RealtimeWatch realtime;
  final ClientChannel channel;

  static Future<Deps> live({SessionVault? vault}) async {
    final session = Session(vault: vault ?? SecureSessionVault());
    await session.restore();

    final channel = await openPanelChannel();
    // Refresh authenticates with the refresh token in the body, so this client
    // deliberately gets no bearer metadata. It is also unguarded: a rejection
    // here is the answer about the session, not a signal to try again.
    final auth = AuthServiceClient(
      channel,
      options: CallOptions(providers: [refreshMetadataProvider(session)]),
    );

    final coordinator = AuthCoordinator(
      session: session,
      isRejected: _serverRejectedRefresh,
      caller: (refreshToken) async {
        final response = await auth.refresh(
          RefreshRequest(refreshToken: refreshToken),
        );
        if (!response.hasTokens()) {
          // Answered but empty: the session is over, and the coordinator has to
          // see that as a rejection rather than a retryable transport failure.
          throw GrpcError.unauthenticated('refresh returned no access token');
        }
        final tokens = response.tokens;
        if (tokens.accessToken.isEmpty) {
          throw GrpcError.unauthenticated('refresh returned no access token');
        }
        return RefreshedTokens(
          accessToken: tokens.accessToken,
          refreshToken: tokens.refreshToken,
        );
      },
    );

    final api = PanelApi(
      session: session,
      calls: GuardedCalls(coordinator: coordinator),
      channel: channel,
    );

    return Deps(
      session: session,
      api: api,
      realtime: RealtimeWatch(open: api.watchTables),
      channel: channel,
    );
  }

  Future<void> dispose() async {
    realtime.stop();
    await channel.terminate();
  }
}

/// Only a rejection the panel itself marked `unauthenticated` ends the session;
/// a transport failure leaves the stored refresh token alone.
bool _serverRejectedRefresh(Object error) =>
    error is GrpcError && error.code == StatusCode.unauthenticated;

export 'package:mobile/src/core/data/query_bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart' show WatchedTable;
import 'package:mobile/src/gen/openoxide/api/v1/remote_server.pb.dart';

/// One server's health. A server whose agent is down yields an error for that
/// server only — one dead agent must not blank the whole screen.
class ServerHealth extends Equatable {
  const ServerHealth({
    required this.server,
    required this.containers,
    this.metric,
    this.error,
  });

  final RemoteServer server;
  final MonitoringServerMetric? metric;
  final List<MonitoringContainerState> containers;
  final Object? error;

  @override
  List<Object?> get props => [server, metric, containers, error];
}

/// Server health, one card per server.
///
/// Metrics and container states are per-server RPCs, so the bloc loads the list
/// and then asks for each server's numbers.
class ServersHealthBloc extends QueryBloc<List<ServerHealth>> {
  ServersHealthBloc({required PanelApi api, super.realtime})
    : super(
        load: () async {
          final servers = await api.listServers();
          return [
            for (final server in servers.servers) await _one(api, server),
          ];
        },
        listensOn: const [
          WatchedTable.WATCHED_TABLE_SERVERS,
          WatchedTable.WATCHED_TABLE_MONITORING_AGENTS,
        ],
      );

  static Future<ServerHealth> _one(PanelApi api, RemoteServer server) async {
    final id = server.id.toInt();
    final metrics = api.getServerMetrics(id);
    final states = api.getContainerStates(id);
    try {
      // Both calls are in flight and the wait listens to each: an agent that is
      // down rejects both, and a rejection nobody listened to would surface as
      // an unhandled error rather than the one card that says so.
      await Future.wait<Object?>([metrics, states]);
      final answered = await metrics;
      final running = await states;
      return ServerHealth(
        server: server,
        metric: answered.metrics.isEmpty ? null : answered.metrics.first,
        containers: running.states.toList(),
      );
    } on Object catch (cause) {
      return ServerHealth(
        server: server,
        containers: const <MonitoringContainerState>[],
        error: cause,
      );
    }
  }
}

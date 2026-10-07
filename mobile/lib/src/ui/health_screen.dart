import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/monitoring.pb.dart';
import '../gen/openoxide/api/v1/realtime.pbenum.dart';
import '../gen/openoxide/api/v1/remote_server.pb.dart';
import '../state/deps.dart';
import '../state/query.dart';
import 'common.dart';
import 'tokens.dart';

/// Server health, one card per server.
///
/// Metrics and container states are per-server RPCs, so this loads the list and
/// then asks for each server's numbers. A server whose agent is down yields an
/// error for that server only — one dead agent must not blank the whole screen.
class HealthScreen extends StatefulWidget {
  const HealthScreen({super.key});

  @override
  State<HealthScreen> createState() => _HealthScreenState();
}

typedef _ServerHealth = ({
  RemoteServer server,
  MonitoringServerMetric? metric,
  List<MonitoringContainerState> containers,
  Object? error,
});

class _HealthScreenState extends State<HealthScreen> {
  late final Query<List<_ServerHealth>> _query;

  @override
  void initState() {
    super.initState();
    final deps = context.read<Deps>();
    _query = Query<List<_ServerHealth>>(
      load: () async {
        final servers = await deps.api.listServers();
        return [
          for (final server in servers.servers)
            await _one(deps, server),
        ];
      },
      realtime: deps.realtime,
      listensOn: const [
        WatchedTable.WATCHED_TABLE_SERVERS,
        WatchedTable.WATCHED_TABLE_MONITORING_AGENTS,
      ],
    )..start();
  }

  static Future<_ServerHealth> _one(Deps deps, RemoteServer server) async {
    final id = server.id.toInt();
    final metrics = deps.api.getServerMetrics(id);
    final states = deps.api.getContainerStates(id);
    try {
      // Both calls are in flight and the wait listens to each: an agent that is
      // down rejects both, and a rejection nobody listened to would surface as
      // an unhandled error rather than the one card that says so.
      await Future.wait<Object?>([metrics, states]);
      final answered = await metrics;
      final running = await states;
      return (
        server: server,
        metric: answered.metrics.isEmpty ? null : answered.metrics.first,
        containers: running.states.toList(),
        error: null,
      );
    } on Object catch (cause) {
      return (
        server: server,
        metric: null,
        containers: const <MonitoringContainerState>[],
        error: cause,
      );
    }
  }

  @override
  void dispose() {
    _query.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _query,
      builder: (context, _) {
        final servers = _query.data;
        return SectionScaffold(
          onRefresh: _query.refresh,
          loading: servers == null && _query.loading,
          error: servers == null ? _query.error : null,
          onRetry: _query.refresh,
          emptyTitle: 'No servers connected',
          emptySubtitle: 'Add a server in the web dashboard to monitor it here.',
          children: [
            if (servers != null)
              for (var index = 0; index < servers.length; index++)
                _ServerCard(
                  health: servers[index],
                  delay: rowDelay(index),
                ),
          ],
        );
      },
    );
  }
}

class _ServerCard extends StatelessWidget {
  const _ServerCard({required this.health, this.delay = Duration.zero});

  final _ServerHealth health;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final server = health.server;
    final metric = health.metric;
    final text = Theme.of(context).textTheme;
    final status = wireLabel(server.serverStatus.name, 'REMOTE_SERVER_STATUS_');
    final running = health.containers
        .where((container) => container.state.toUpperCase() == 'RUNNING')
        .length;
    return Card(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(
          Spacing.lg,
          Spacing.md,
          Spacing.lg,
          Spacing.lg,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    server.name.isEmpty ? server.ipAddress : server.name,
                    style: text.titleMedium,
                  ),
                ),
                const SizedBox(width: Spacing.sm),
                Text(
                  status,
                  style: text.labelSmall?.copyWith(
                    color: _statusColor(context, status),
                  ),
                ),
              ],
            ),
            if (server.ipAddress.isNotEmpty)
              Text(server.ipAddress, style: text.labelSmall),
            const SizedBox(height: Spacing.md),
            if (metric == null)
              Text(
                health.error == null
                    ? 'No metrics reported yet'
                    : 'Agent not answering',
                style: text.bodySmall,
              )
            else
              _Gauges(metric: metric),
            if (health.containers.isNotEmpty) ...[
              const SizedBox(height: Spacing.md),
              Text(
                '$running of ${health.containers.length} containers running',
                style: text.labelMedium,
              ),
              const SizedBox(height: Spacing.xs),
              Wrap(
                spacing: Spacing.xs,
                runSpacing: Spacing.xs,
                children: [
                  for (final container in health.containers.take(12))
                    _ContainerChip(container: container),
                ],
              ),
            ],
          ],
        ),
      ),
    ).animate(delay: delay).fadeIn(duration: Motion.fast).slideY(
      begin: Motion.liftRow,
    );
  }
}

/// Takes the label the card shows, not the wire name: `ACTIVE` and `INACTIVE`
/// are the only two statuses the panel sends for a server, and the web's server
/// row reads that same pair as online green / offline red.
Color _statusColor(BuildContext context, String label) {
  final ox = context.ox;
  return switch (label) {
    'Active' => ox.success,
    'Inactive' => ox.danger,
    _ => ox.textSubtle,
  };
}

class _Gauges extends StatelessWidget {
  const _Gauges({required this.metric});

  final MonitoringServerMetric metric;

  @override
  Widget build(BuildContext context) {
    // The agent sends `mem_used` and `disk_used` as percentages and the totals
    // in GB — the same reading the web's overview tab takes. Dividing a percent
    // by a GB total fills the bar at any size of machine.
    final memPct = metric.memUsed.clamp(0.0, 100.0);
    final diskPct = metric.diskUsed.clamp(0.0, 100.0);
    final diskGb = metric.totalDisk * diskPct / 100;
    return Column(
      children: [
        _GaugeRow(
          label: 'CPU',
          value: '${metric.cpu.clamp(0.0, 100.0).toStringAsFixed(0)}%',
          fraction: metric.cpu / 100,
        ),
        _GaugeRow(
          label: 'Memory',
          value: '${metric.memUsedGb.toStringAsFixed(1)} GB',
          fraction: memPct / 100,
          detail: '${metric.memTotal.toStringAsFixed(1)} GB total',
        ),
        _GaugeRow(
          label: 'Disk',
          value: '${diskGb.toStringAsFixed(0)} GB',
          fraction: diskPct / 100,
          detail: '${metric.totalDisk.toStringAsFixed(0)} GB total',
        ),
      ],
    );
  }
}

class _GaugeRow extends StatelessWidget {
  const _GaugeRow({
    required this.label,
    required this.value,
    required this.fraction,
    this.detail,
  });

  final String label;
  final String value;
  final double fraction;
  final String? detail;

  @override
  Widget build(BuildContext context) {
    final clamped = fraction.clamp(0.0, 1.0);
    final text = Theme.of(context).textTheme;
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: Spacing.xs),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              SizedBox(
                width: 60,
                child: Text(label, style: text.labelMedium),
              ),
              Text(value, style: text.labelMedium),
              if (detail != null) ...[
                const SizedBox(width: Spacing.xs),
                Expanded(
                  child: Text(
                    detail!,
                    style: text.labelSmall,
                    textAlign: TextAlign.end,
                  ),
                ),
              ],
            ],
          ),
          const SizedBox(height: Spacing.xs),
          // A metric refresh moves the bar; sliding to the new value keeps the
          // number and the fill readable as one change.
          TweenAnimationBuilder<double>(
            tween: Tween(begin: 0, end: clamped),
            duration: Motion.base,
            curve: Motion.move,
            builder: (context, value, _) => ClipRRect(
              borderRadius: BorderRadius.circular(Radii.dot),
              child: LinearProgressIndicator(
                value: value,
                minHeight: Mark.bar,
                backgroundColor: context.ox.subtleFill,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _ContainerChip extends StatelessWidget {
  const _ContainerChip({required this.container});

  final MonitoringContainerState container;

  /// A chip stays a chip: compose names run long enough to push the row past the
  /// card, so the name ellipsizes inside a width the narrowest screen can give.
  static const double _nameMaxWidth = 180;

  @override
  Widget build(BuildContext context) {
    final running = container.state.toUpperCase() == 'RUNNING';
    final color = running ? context.ox.success : context.ox.textSubtle;
    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: Spacing.sm,
        vertical: Spacing.xs,
      ),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.12),
        borderRadius: Radii.controlR,
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.circle, size: Mark.dot, color: color),
          const SizedBox(width: Spacing.xs),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: _nameMaxWidth),
            child: Text(
              container.name.isEmpty ? container.containerId : container.name,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: Theme.of(context).textTheme.labelSmall,
            ),
          ),
        ],
      ),
    );
  }
}

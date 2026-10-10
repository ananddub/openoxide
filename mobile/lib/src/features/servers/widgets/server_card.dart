import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/wire_label.dart';
import 'package:mobile/src/features/servers/bloc/servers_health_bloc.dart';
import 'package:mobile/src/features/servers/widgets/container_chip.dart';
import 'package:mobile/src/features/servers/widgets/metric_gauges.dart';

/// One server: name, status, gauges, container chips.
class ServerCard extends StatelessWidget {
  const ServerCard({
    super.key,
    required this.health,
    this.delay = Duration.zero,
  });

  final ServerHealth health;
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
                  MetricGauges(metric: metric),
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
                        ContainerChip(container: container),
                    ],
                  ),
                ],
              ],
            ),
          ),
        )
        .animate(delay: delay)
        .fadeIn(duration: Motion.fast)
        .slideY(begin: Motion.liftRow);
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

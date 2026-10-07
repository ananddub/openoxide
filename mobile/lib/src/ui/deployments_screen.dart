import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/deployment.pb.dart' hide Container;
import '../gen/openoxide/api/v1/realtime.pbenum.dart';
import '../state/deps.dart';
import '../state/deployment_status.dart';
import '../state/query.dart';
import 'common.dart';
import 'deployment_logs_screen.dart';
import 'theme.dart';
import 'tokens.dart';

/// The deploy queue: what is running now, then the recent history.
///
/// Two calls, not one, because `ListActiveDeployments` answers from the running
/// set while `ListDeployments` answers from the table; a single list would either
/// hide a stuck build or show fifty finished rows above it.
class DeploymentsScreen extends StatefulWidget {
  const DeploymentsScreen({super.key});

  @override
  State<DeploymentsScreen> createState() => _DeploymentsScreenState();
}

class _DeploymentsScreenState extends State<DeploymentsScreen> {
  late final Query<_DeploymentRows> _query;

  @override
  void initState() {
    super.initState();
    final deps = context.read<Deps>();
    _query = Query<_DeploymentRows>(
      load: () => _load(deps),
      realtime: deps.realtime,
      listensOn: const [WatchedTable.WATCHED_TABLE_DEPLOYMENTS],
    )..start();
  }

  static Future<_DeploymentRows> _load(Deps deps) async {
    final history = await deps.api.listDeployments(limit: 50);
    final active = await deps.api.listActiveDeployments();
    return (
      active: active.deployments.toList(),
      recent: history.deployments.toList(),
    );
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
        final rows = _query.data;
        return SectionScaffold(
          onRefresh: _query.refresh,
          loading: rows == null && _query.loading,
          error: rows == null ? _query.error : null,
          onRetry: _query.refresh,
          emptyTitle: 'Nothing has been deployed here yet',
          emptySubtitle: 'Deploys appear as soon as the panel starts one.',
          children: rows == null ? const [] : _tiles(rows),
        );
      },
    );
  }

  /// Two groups, each entering top-down. The gap between them is the next
  /// heading's own top inset, so it is not a spacer that has to agree with one.
  static List<Widget> _tiles(_DeploymentRows rows) {
    final tiles = <Widget>[];
    if (rows.active.isNotEmpty) {
      tiles.add(const SectionLabel('In progress', first: true));
      for (var index = 0; index < rows.active.length; index++) {
        tiles.add(
          _DeploymentTile(
            deployment: rows.active[index],
            emphasize: true,
            delay: rowDelay(index),
          ),
        );
      }
    }
    if (rows.recent.isNotEmpty) {
      tiles.add(SectionLabel('Recent', first: rows.active.isEmpty));
      for (var index = 0; index < rows.recent.length; index++) {
        tiles.add(
          _DeploymentTile(
            deployment: rows.recent[index],
            emphasize: false,
            delay: rowDelay(index),
          ),
        );
      }
    }
    return tiles;
  }
}

typedef _DeploymentRows = ({List<Deployment> active, List<Deployment> recent});

class _DeploymentTile extends StatelessWidget {
  const _DeploymentTile({
    required this.deployment,
    required this.emphasize,
    required this.delay,
  });

  final Deployment deployment;
  final bool emphasize;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final brightness = Theme.of(context).brightness;
    final text = Theme.of(context).textTheme;
    final phase = deploymentPhase(deployment.status);
    final colors = phaseColors(phase, brightness);
    final id = deployment.id.toInt();
    final title = deployment.resourceType.isEmpty
        ? 'Deployment $id'
        : deployment.resourceType;

    return Card(
      child: ListTile(
        leading: _StatusDot(colors: colors),
        title: Text(
          title,
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(fontWeight: emphasize ? FontWeight.w600 : null),
        ),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: Spacing.xxs),
            Text(
              deployment.status,
              style: text.bodySmall?.copyWith(color: colors.text),
            ),
            if (deployment.userEmail.isNotEmpty)
              Text(deployment.userEmail, style: text.labelSmall),
            if (deployment.error.isNotEmpty)
              Padding(
                padding: const EdgeInsets.only(top: Spacing.xxs),
                child: Text(
                  deployment.error,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: text.labelSmall?.copyWith(color: colors.text),
                ),
              ),
          ],
        ),
        trailing: Text(_ago(deployment.updatedAt.toInt()), style: text.labelSmall),
        onTap: () => Navigator.of(context).push(
          MaterialPageRoute(
            builder: (_) => DeploymentLogsScreen(deploymentId: id, title: title),
          ),
        ),
      ),
    ).animate(delay: delay).fadeIn(duration: Motion.fast).slideY(
      begin: Motion.liftRow,
    );
  }

  /// The panel has written these as seconds in some rows and milliseconds in
  /// others, so the unit is inferred from the magnitude — the same test the web
  /// deployment list makes.
  static String _ago(int epoch) {
    if (epoch <= 0) return '';
    final millis = epoch < 100000000000 ? epoch * 1000 : epoch;
    final then = DateTime.fromMillisecondsSinceEpoch(millis, isUtc: true);
    final seconds = DateTime.now().toUtc().difference(then).inSeconds;
    if (seconds < 60) return '${seconds}s';
    if (seconds < 3600) return '${seconds ~/ 60}m';
    if (seconds < 86400) return '${seconds ~/ 3600}h';
    return '${seconds ~/ 86400}d';
  }
}

class _StatusDot extends StatelessWidget {
  const _StatusDot({required this.colors});

  final ({Color dot, Color text, bool pulse}) colors;

  @override
  Widget build(BuildContext context) {
    final dot = Container(
      width: Mark.dot,
      height: Mark.dot,
      decoration: BoxDecoration(color: colors.dot, shape: BoxShape.circle),
    );
    // No Center here: ListTile sizes `leading` with the whole tile width
    // available, so a filler widget would claim all of it and the tile
    // asserts. The slot already centres vertically.
    if (!colors.pulse) return dot;
    return _Pulse(child: dot);
  }
}

/// Breathing opacity while a deploy is in flight — the phone equivalent of the
/// web's `animate-pulse` on an active status pill.
class _Pulse extends StatefulWidget {
  const _Pulse({required this.child});

  final Widget child;

  @override
  State<_Pulse> createState() => _PulseState();
}

class _PulseState extends State<_Pulse> with SingleTickerProviderStateMixin {
  late final AnimationController _controller = AnimationController(
    vsync: this,
    duration: Motion.pulse,
  )..repeat(reverse: true);

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: Tween<double>(
        begin: Motion.pulseOpacity,
        end: 1,
      ).animate(_controller),
      child: widget.child,
    );
  }
}

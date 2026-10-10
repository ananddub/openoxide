import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/domain/deployment_status.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/theme/phase_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/features/deployments/screens/deployment_logs_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart' hide Container;

/// One deployment as a tappable row of the queue.
class DeploymentTile extends StatelessWidget {
  const DeploymentTile({
    super.key,
    required this.deployment,
    required this.emphasize,
    required this.delay,
  });

  final Deployment deployment;
  final bool emphasize;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final phase = deploymentPhase(deployment.status);
    final colors = phaseColors(phase, context.brightness);
    final id = deployment.id.toInt();
    final title = deployment.resourceType.isEmpty
        ? 'Deployment $id'
        : deployment.resourceType;

    return Card(
          child: ListTile(
            leading: StatusDot(colors: colors),
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
            trailing: Text(
              deploymentAge(deployment.updatedAt.toInt()),
              style: text.labelSmall,
            ),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) =>
                    DeploymentLogsScreen(deploymentId: id, title: title),
              ),
            ),
          ),
        )
        .animate(delay: delay)
        .fadeIn(duration: Motion.fast)
        .slideY(begin: Motion.liftRow);
  }

  /// The panel has written these as seconds in some rows and milliseconds in
  /// others, so the unit is inferred from the magnitude — the same test the web
  /// deployment list makes.
  static String deploymentAge(int epoch) {
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

/// The one status dot, breathing while a deploy is in flight.
class StatusDot extends StatelessWidget {
  const StatusDot({super.key, required this.colors});

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

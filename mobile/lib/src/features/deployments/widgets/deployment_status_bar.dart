import 'package:flutter/material.dart';
import 'package:mobile/src/core/domain/deployment_status.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/theme/phase_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart' hide Container;

/// The deployment's status row above the log pane.
class DeploymentStatusBar extends StatelessWidget {
  const DeploymentStatusBar({super.key, required this.deployment});

  final Deployment? deployment;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final item = deployment;
    final status = item?.status ?? '';
    final phase = deploymentPhase(status);
    final colors = phaseColors(phase, context.brightness);
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(
        Spacing.md,
        Spacing.sm,
        Spacing.md,
        Spacing.md,
      ),
      color: context.ox.card,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.circle, size: Mark.dot, color: colors.dot),
              const SizedBox(width: Spacing.sm),
              Expanded(
                child: Text(
                  status.isEmpty ? 'unknown' : status,
                  style: text.bodySmall?.copyWith(color: colors.text),
                ),
              ),
              if (item != null && item.resourceType.isNotEmpty)
                Flexible(
                  child: Text(
                    item.resourceType,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: text.labelSmall,
                  ),
                ),
            ],
          ),
          if (item != null && item.error.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(top: Spacing.xs),
              child: Text(
                item.error,
                style: text.labelSmall?.copyWith(color: colors.text),
              ),
            ),
        ],
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pb.dart';

/// One container state as a coloured chip.
class ContainerChip extends StatelessWidget {
  const ContainerChip({super.key, required this.container});

  final MonitoringContainerState container;

  /// A chip stays a chip: compose names run long enough to push the row past
  /// the card, so the name ellipsizes inside a width the narrowest screen can
  /// give.
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

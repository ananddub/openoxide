import 'package:flutter/material.dart';

import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

import 'package:mobile/src/core/ui/widgets/action_button.dart';

/// Shared pieces every screen needs, so "failed" and "empty" look the same
/// wherever they appear.
class Loading extends StatelessWidget {
  const Loading({super.key, this.label});

  final String? label;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const CircularProgressIndicator(strokeWidth: Mark.stroke),
          if (label != null) ...[
            const SizedBox(height: Spacing.md),
            Text(label!, style: Theme.of(context).textTheme.labelMedium),
          ],
        ],
      ),
    );
  }
}

/// A failure as a contained banner: danger-tinted card, icon, the panel's own
/// words, and — when there is something to retry — the retry beside them.
class ErrorNotice extends StatelessWidget {
  const ErrorNotice({super.key, required this.error, this.onRetry});

  final AppError error;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) {
    final ox = context.ox;
    final text = Theme.of(context).textTheme;
    return Container(
      padding: const EdgeInsets.all(Spacing.md),
      decoration: BoxDecoration(
        color: ox.danger.withValues(alpha: 0.1),
        borderRadius: Radii.controlR,
        border: Border.all(color: ox.danger.withValues(alpha: 0.35)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(Icons.error_outline, size: Mark.glyph, color: ox.danger),
              const SizedBox(width: Spacing.sm),
              Expanded(
                child: Text(
                  error.userMessage,
                  style: text.bodyMedium?.copyWith(color: ox.text),
                ),
              ),
            ],
          ),
          if (onRetry != null) ...[
            const SizedBox(height: Spacing.sm),
            Align(
              alignment: Alignment.centerRight,
              child: ActionButton(
                label: 'Try again',
                tone: ButtonTone.danger,
                size: ButtonSize.small,
                onPressed: onRetry,
              ),
            ),
          ],
        ],
      ),
    );
  }
}

class EmptyNotice extends StatelessWidget {
  const EmptyNotice({super.key, required this.title, this.subtitle});

  final String title;
  final String? subtitle;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(Spacing.xxl),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              title,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            if (subtitle != null) ...[
              const SizedBox(height: Spacing.xs),
              Text(
                subtitle!,
                textAlign: TextAlign.center,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ],
          ],
        ),
      ),
    );
  }
}

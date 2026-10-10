import 'package:flutter_animate/flutter_animate.dart';
import 'package:flutter/material.dart';

import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// The wordmark and tagline of the sign-in screen.
class SignInBrand extends StatelessWidget {
  const SignInBrand({super.key});

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final ox = context.ox;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // The one accent-marked object on the screen: everything else stays
        // neutral so the fields and the primary button own the reader's eye.
        Container(
          width: 44,
          height: 44,
          decoration: BoxDecoration(
            color: ox.accent,
            borderRadius: Radii.controlR,
          ),
          child: Icon(Icons.dns_outlined, size: 26, color: ox.onAccent),
        ),
        const SizedBox(height: Spacing.lg),
        Text(
          'Openoxide',
          key: const ValueKey('brand'),
          style: text.headlineMedium,
        ),
        const SizedBox(height: Spacing.xs),
        Text(
          'Your servers, deployments and logs in your pocket.',
          style: text.bodyMedium?.copyWith(color: context.ox.textMuted),
        ),
      ],
    );
  }
}

/// Entrance for one item of the sign-in form. `step` is its position in the
/// stagger, so the fields arrive top-down rather than all at once.
Widget signInEnter(int step, Widget child) => child
    .animate(delay: Motion.delay(step))
    .fadeIn(duration: Motion.base)
    .slideY(begin: Motion.liftBlock, curve: Motion.enter);

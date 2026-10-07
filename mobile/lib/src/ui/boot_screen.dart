import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';

import '../grpc/app_error.dart';
import '../state/auth_controller.dart';
import 'common.dart';
import 'shell_screen.dart';
import 'sign_in_screen.dart';
import 'tokens.dart';

/// Chooses the first real screen from [AppStage]. Every branch is a state the
/// panel can actually be in, so a phone that lands on the wrong address gets an
/// explanation instead of a spinner forever.
class BootScreen extends StatelessWidget {
  const BootScreen({super.key, required this.controller});

  final AuthController controller;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) => switch (controller.stage) {
        AppStage.probing => const _Placeholder(
          child: Loading(label: 'Contacting the panel'),
        ),
        // The transport's own words go in `detail`, not `body`: a handshake
        // failure is a paragraph of C++ file names, and reading that instead of
        // a sentence is what made this screen useless.
        AppStage.unreachable => _Gate(
          title: 'Cannot reach the panel',
          body:
              'Nothing answered at the configured address. Check that the panel '
              'is running and that this phone can reach it.',
          detail: controller.error?.userMessage,
          actionLabel: 'Try again',
          onAction: controller.boot,
        ),
        AppStage.needsSetup => _Gate(
          title: 'This panel has no owner yet',
          body:
              'Finish the initial setup in the web dashboard, then reopen the '
              'app and sign in.',
          actionLabel: 'Check again',
          onAction: controller.boot,
        ),
        AppStage.signedOut => const SignInScreen(),
        AppStage.signedIn => const ShellScreen(),
      },
    );
  }
}

class _Placeholder extends StatelessWidget {
  const _Placeholder({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: SafeArea(child: Center(child: child)));
  }
}

class _Gate extends StatelessWidget {
  const _Gate({
    required this.title,
    required this.body,
    this.detail,
    required this.actionLabel,
    required this.onAction,
  });

  final String title;
  final String body;

  /// Secondary diagnostics — the failure's own words — kept quiet and short so
  /// the reader gets an actionable sentence first.
  final String? detail;
  final String actionLabel;
  final Future<void> Function() onAction;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final ox = context.ox;
    final content = Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(
          Icons.cloud_off_outlined,
          size: 32,
          color: ox.textSubtle,
        ),
        const SizedBox(height: Spacing.md),
        Text(
          title,
          textAlign: TextAlign.center,
          style: text.titleMedium,
        ),
        const SizedBox(height: Spacing.sm),
        Text(body, textAlign: TextAlign.center, style: text.bodyMedium),
        if (detail != null && detail!.isNotEmpty) ...[
          const SizedBox(height: Spacing.md),
          Text(
            detail!,
            textAlign: TextAlign.center,
            maxLines: 3,
            overflow: TextOverflow.ellipsis,
            style: text.bodySmall?.copyWith(color: ox.textSubtle),
          ),
        ],
        const SizedBox(height: Spacing.md),
        const PanelTargetLabel(),
        const SizedBox(height: Spacing.xl),
        ActionButton(label: actionLabel, onPressed: () => onAction()),
      ],
    ).animate().fadeIn(duration: Motion.slow).slideY(begin: Motion.liftRow);

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: Padding(
            padding: const EdgeInsets.all(Spacing.xl),
            child: content,
          ),
        ),
      ),
    );
  }
}

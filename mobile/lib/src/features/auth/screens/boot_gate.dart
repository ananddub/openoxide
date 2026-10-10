import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/config/app_config.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/panel_target_label.dart';
import 'package:mobile/src/core/ui/widgets/status_notices.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/features/auth/screens/sign_in_screen.dart';
import 'package:mobile/src/features/shell/shell_screen.dart';

/// Chooses the first real screen from [AppStage]. Every branch is a state the
/// panel can actually be in, so a phone that lands on the wrong address gets an
/// explanation instead of a spinner forever.
class BootGate extends StatelessWidget {
  const BootGate({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<AuthBloc, AuthState>(
      builder: (context, state) => switch (state.stage) {
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
          detail: state.error?.userMessage,
          hint: tlsHint(state.error?.userMessage),
          actionLabel: 'Try again',
          onAction: () =>
              context.read<AuthBloc>().add(const AuthBootRequested()),
        ),
        AppStage.needsSetup => _Gate(
          title: 'This panel has no owner yet',
          body:
              'Finish the initial setup in the web dashboard, then reopen the '
              'app and sign in.',
          actionLabel: 'Check again',
          onAction: () =>
              context.read<AuthBloc>().add(const AuthBootRequested()),
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
    return Scaffold(
      body: SafeArea(child: Center(child: child)),
    );
  }
}

class _Gate extends StatelessWidget {
  const _Gate({
    required this.title,
    required this.body,
    this.detail,
    this.hint,
    required this.actionLabel,
    required this.onAction,
  });

  final String title;
  final String body;

  /// Secondary diagnostics — the failure's own words — kept quiet and short so
  /// the reader gets an actionable sentence first.
  final String? detail;

  /// The fix for the most common cause of [detail], when it has one.
  final String? hint;
  final String actionLabel;
  final VoidCallback onAction;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final ox = context.ox;
    final content = Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(Icons.cloud_off_outlined, size: 32, color: ox.textSubtle),
        const SizedBox(height: Spacing.md),
        Text(title, textAlign: TextAlign.center, style: text.titleMedium),
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
        if (hint != null) ...[
          const SizedBox(height: Spacing.md),
          Text(hint!, textAlign: TextAlign.center, style: text.bodySmall),
        ],
        const SizedBox(height: Spacing.md),
        const PanelTargetLabel(),
        const SizedBox(height: Spacing.xl),
        ActionButton(label: actionLabel, onPressed: onAction),
      ],
    ).animate().fadeIn(duration: Motion.slow).slideY(begin: Motion.liftRow);

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            child: Padding(
              padding: const EdgeInsets.all(Spacing.xl),
              child: content,
            ),
          ),
        ),
      ),
    );
  }
}

/// A fix, when the transport's own words name a cause the reader can act on.
///
/// The dev stack's certificate is issued for `localhost` only, so a phone that
/// reaches the panel as `10.0.2.2` (or any other address) always fails here —
/// the sentence that explains the run flag beats the raw BoringSSL line.
String? tlsHint(String? userMessage) {
  final detail = userMessage ?? '';
  final certificateProblem =
      detail.contains('CERTIFICATE_VERIFY_FAILED') ||
      detail.contains('HandshakeException');
  if (!certificateProblem) return null;
  if (AppConfig.allowAnyCert) {
    return 'The certificate could not be verified. Pin it with the '
        'PANEL_CERT_ASSET asset, or reach the panel at the address it was '
        'issued for.';
  }
  return 'The certificate could not be verified. For development, run the app '
      'with --dart-define=DEV_ALLOW_ANY_CERT=true, or pin the panel '
      'certificate with PANEL_CERT_ASSET.';
}

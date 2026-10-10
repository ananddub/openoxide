import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/panel_target_label.dart';
import 'package:mobile/src/core/ui/widgets/status_notices.dart';
import 'package:mobile/src/features/auth/widgets/sign_in_brand.dart';

/// The email, password and (when demanded) second-factor fields.
///
/// The panel answers a 2FA login attempt as `unauthenticated` with
/// "two-factor authentication code is required", so the code field appears after
/// the first rejection rather than before it — the web client has no 2FA screen
/// to copy, and showing a field most users never fill would be worse.
class SignInFields extends StatelessWidget {
  const SignInFields({
    super.key,
    required this.formKey,
    required this.email,
    required this.password,
    required this.code,
    required this.askedForCode,
    required this.showPassword,
    required this.busy,
    required this.error,
    required this.onTogglePassword,
    required this.onSubmit,
  });

  final GlobalKey<FormState> formKey;
  final TextEditingController email;
  final TextEditingController password;
  final TextEditingController code;
  final bool askedForCode;
  final bool showPassword;
  final bool busy;
  final AppError? error;
  final VoidCallback onTogglePassword;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return Form(
      key: formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          signInEnter(
            2,
            TextFormField(
              key: const ValueKey('email'),
              controller: email,
              autofocus: true,
              textInputAction: TextInputAction.next,
              keyboardType: TextInputType.emailAddress,
              autofillHints: const [AutofillHints.email],
              decoration: const InputDecoration(labelText: 'Email'),
              validator: (value) =>
                  (value ?? '').contains('@') ? null : 'Enter an email',
            ),
          ),
          const SizedBox(height: Spacing.md),
          signInEnter(3, _passwordField(context)),
          // The one state animation on this screen: the row grows rather than
          // popping in, so the button below it slides instead of jumping.
          AnimatedSize(
            duration: Motion.base,
            curve: Motion.move,
            alignment: Alignment.topCenter,
            child: !askedForCode
                ? const SizedBox(width: double.infinity)
                : Padding(
                    padding: const EdgeInsets.only(top: Spacing.md),
                    child: signInEnter(1, _codeField(context)),
                  ),
          ),
          if (error != null)
            Padding(
              padding: const EdgeInsets.only(top: Spacing.lg),
              child: ErrorNotice(error: error!)
                  .animate()
                  .fadeIn(duration: Motion.fast),
            ),
          const SizedBox(height: Spacing.xl),
          signInEnter(
            4,
            ActionButton(
              key: const ValueKey('submit'),
              label: 'Sign in',
              expanded: true,
              loading: busy,
              onPressed: busy ? null : onSubmit,
            ),
          ),
          const SizedBox(height: Spacing.lg),
          Center(
            child: const PanelTargetLabel()
                .animate(delay: Motion.delay(5))
                .fadeIn(duration: Motion.base),
          ),
        ],
      ),
    );
  }

  Widget _passwordField(BuildContext context) => TextFormField(
    key: const ValueKey('password'),
    controller: password,
    obscureText: !showPassword,
    textInputAction: TextInputAction.done,
    autofillHints: const [AutofillHints.password],
    onFieldSubmitted: (_) => onSubmit(),
    decoration: InputDecoration(
      labelText: 'Password',
      suffixIcon: IconButton(
        onPressed: onTogglePassword,
        icon: Icon(
          showPassword
              ? Icons.visibility_off_outlined
              : Icons.visibility_outlined,
        ),
        tooltip: showPassword ? 'Hide password' : 'Show password',
      ),
    ),
    validator: (value) => (value ?? '').isEmpty ? 'Enter your password' : null,
  );

  Widget _codeField(BuildContext context) => TextFormField(
    key: const ValueKey('code'),
    controller: code,
    textInputAction: TextInputAction.done,
    onFieldSubmitted: (_) => onSubmit(),
    maxLength: 16,
    keyboardType: TextInputType.text,
    autofillHints: const [AutofillHints.oneTimeCode],
    decoration: const InputDecoration(
      labelText: 'Authentication code',
      helperText: '6-digit code, or a recovery code',
      counterText: '',
    ),
    validator: (value) => (value ?? '').length >= 6 ? null : 'Enter a code',
  );
}

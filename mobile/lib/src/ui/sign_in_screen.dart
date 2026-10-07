import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:provider/provider.dart';

import '../grpc/app_error.dart';
import '../state/auth_controller.dart';
import 'common.dart';
import 'tokens.dart';

/// Email and password, with the second factor only when the panel asks for it.
///
/// The panel answers a 2FA login attempt as `unauthenticated` with
/// "two-factor authentication code is required", so the code field appears after
/// the first rejection rather than before it — the web client has no 2FA screen
/// to copy, and showing a field most users never fill would be worse.
class SignInScreen extends StatefulWidget {
  const SignInScreen({super.key});

  @override
  State<SignInScreen> createState() => _SignInScreenState();
}

class _SignInScreenState extends State<SignInScreen> {
  final _form = GlobalKey<FormState>();
  final _email = TextEditingController();
  final _password = TextEditingController();
  final _code = TextEditingController();
  final _scroll = ScrollController();
  bool _askedForCode = false;
  bool _showPassword = false;

  @override
  void dispose() {
    _email.dispose();
    _password.dispose();
    _code.dispose();
    _scroll.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!(_form.currentState?.validate() ?? false)) return;
    final controller = context.read<AuthController>();
    final code = _code.text.trim();
    final accepted = await controller.signIn(
      email: _email.text.trim(),
      password: _password.text,
      twoFactorCode: _askedForCode && code.isNotEmpty ? code : null,
    );
    if (!mounted || accepted) return;
    final error = controller.error;
    setState(() {
      _askedForCode = error?.demandsSecondFactor ?? false;
      if (!_askedForCode) _code.clear();
    });
    if (_askedForCode) _revealCodeField();
  }

  /// The new field pushes the button down, and on a small phone with the
  /// keyboard up that is exactly what the user needs to reach.
  void _revealCodeField() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted || !_scroll.hasClients) return;
      _scroll.animateTo(
        _scroll.position.maxScrollExtent,
        duration: Motion.base,
        curve: Motion.move,
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    final controller = context.watch<AuthController>();
    final error = controller.error;

    final brand = const _Brand().animate().fadeIn(
      duration: Motion.slow,
    ).slideY(begin: -Motion.liftBlock, curve: Motion.enter);

    final fields = Form(
      key: _form,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _enter(
            2,
            TextFormField(
              key: const ValueKey('email'),
              controller: _email,
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
          _enter(
            3,
            TextFormField(
              key: const ValueKey('password'),
              controller: _password,
              obscureText: !_showPassword,
              textInputAction: TextInputAction.done,
              autofillHints: const [AutofillHints.password],
              onFieldSubmitted: (_) => _submit(),
              decoration: InputDecoration(
                labelText: 'Password',
                suffixIcon: IconButton(
                  onPressed: () => setState(() => _showPassword = !_showPassword),
                  icon: Icon(
                    _showPassword
                        ? Icons.visibility_off_outlined
                        : Icons.visibility_outlined,
                  ),
                  tooltip: _showPassword ? 'Hide password' : 'Show password',
                ),
              ),
              validator: (value) =>
                  (value ?? '').isEmpty ? 'Enter your password' : null,
            ),
          ),
          // The one state animation on this screen: the row grows rather than
          // popping in, so the button below it slides instead of jumping.
          AnimatedSize(
            duration: Motion.base,
            curve: Motion.move,
            alignment: Alignment.topCenter,
            child: !_askedForCode
                ? const SizedBox(width: double.infinity)
                : Padding(
                    padding: const EdgeInsets.only(top: Spacing.md),
                    child: _enter(
                      1,
                      TextFormField(
                        key: const ValueKey('code'),
                        controller: _code,
                        textInputAction: TextInputAction.done,
                        onFieldSubmitted: (_) => _submit(),
                        maxLength: 16,
                        keyboardType: TextInputType.text,
                        autofillHints: const [AutofillHints.oneTimeCode],
                        decoration: const InputDecoration(
                          labelText: 'Authentication code',
                          helperText: '6-digit code, or a recovery code',
                          counterText: '',
                        ),
                        validator: (value) => (value ?? '').length >= 6
                            ? null
                            : 'Enter a code',
                      ),
                    ),
                  ),
          ),
          if (error != null)
            Padding(
              padding: const EdgeInsets.only(top: Spacing.lg),
              child: ErrorNotice(error: error)
                  .animate()
                  .fadeIn(duration: Motion.fast),
            ),
          const SizedBox(height: Spacing.xl),
          _enter(
            4,
            ActionButton(
              key: const ValueKey('submit'),
              label: 'Sign in',
              expanded: true,
              loading: controller.busy,
              onPressed: controller.busy ? null : _submit,
            ),
          ),
          const SizedBox(height: Spacing.lg),
          Center(
            child: const PanelTargetLabel().animate(
              delay: Motion.delay(5),
            ).fadeIn(duration: Motion.base),
          ),
        ],
      ),
    );

    final column = ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: Layout.measure),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          brand,
          const SizedBox(height: Spacing.xl),
          fields,
        ],
      ),
    );

    return Scaffold(
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, viewport) {
            // Tablet and landscape get the wordmark beside the form instead of
            // above it, which is the only way the extra width is worth having.
            if (viewport.maxWidth >= Layout.wide) {
              return Center(
                child: Padding(
                  padding: const EdgeInsets.all(Spacing.xxl),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Expanded(child: brand),
                      const SizedBox(width: Spacing.xxl),
                      Expanded(
                        child: SingleChildScrollView(
                          controller: _scroll,
                          child: ConstrainedBox(
                            constraints: const BoxConstraints(maxWidth: Layout.measure),
                            child: fields,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              );
            }
            return Center(
              child: SingleChildScrollView(
                controller: _scroll,
                padding: const EdgeInsets.fromLTRB(
                  Spacing.xl,
                  Spacing.xxl,
                  Spacing.xl,
                  Spacing.lg,
                ),
                child: column,
              ),
            );
          },
        ),
      ),
    );
  }

  /// Entrance for one item of the form. `step` is its position in the stagger,
  /// so the fields arrive top-down rather than all at once.
  Widget _enter(int step, Widget child) => child
      .animate(delay: Motion.delay(step))
      .fadeIn(duration: Motion.base)
      .slideY(begin: Motion.liftBlock, curve: Motion.enter);
}

class _Brand extends StatelessWidget {
  const _Brand();

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
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

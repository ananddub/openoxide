import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/layout.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/features/auth/widgets/sign_in_brand.dart';
import 'package:mobile/src/features/auth/widgets/sign_in_fields.dart';

/// Email and password, with the second factor only when the panel asks for it.
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

  void _submit() {
    if (!(_form.currentState?.validate() ?? false)) return;
    final code = _code.text.trim();
    context.read<AuthBloc>().add(
      AuthSignInRequested(
        email: _email.text.trim(),
        password: _password.text,
        twoFactorCode: _askedForCode && code.isNotEmpty ? code : null,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final brand = const SignInBrand()
        .animate()
        .fadeIn(duration: Motion.slow)
        .slideY(begin: -Motion.liftBlock, curve: Motion.enter);

    final fields = BlocSelector<AuthBloc, AuthState, (bool, AppError?)>(
      selector: (state) => (state.busy, state.error),
      builder: (context, values) {
        final (busy, error) = values;
        return SignInFields(
          formKey: _form,
          email: _email,
          password: _password,
          code: _code,
          askedForCode: _askedForCode,
          showPassword: _showPassword,
          busy: busy,
          error: error,
          onTogglePassword: () =>
              setState(() => _showPassword = !_showPassword),
          onSubmit: _submit,
        );
      },
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
        child: MultiBlocListener(
          listeners: [
            // The panel demands the second factor by rejecting the first
            // attempt, so the field's appearance is a reaction to that error.
            BlocListener<AuthBloc, AuthState>(
              listenWhen: (previous, current) =>
                  previous.error != current.error && current.error != null,
              listener: (context, state) {
                final error = state.error;
                if (error == null) return;
                setState(() {
                  _askedForCode = error.demandsSecondFactor;
                  if (!_askedForCode) _code.clear();
                });
                if (_askedForCode) _revealCodeField();
              },
            ),
            // A successful sign-in leaves nothing worth retyping.
            BlocListener<AuthBloc, AuthState>(
              listenWhen: (previous, current) =>
                  current.stage == AppStage.signedIn &&
                  previous.stage != AppStage.signedIn,
              listener: (context, state) =>
                  setState(() => _askedForCode = false),
            ),
          ],
          child: LayoutBuilder(
            builder: (context, viewport) {
              // Tablet and landscape get the wordmark beside the form instead of
              // above it, which is the only way the extra width is worth having.
              if (viewport.maxWidth >= Layout.wide) {
                return _wide(brand, fields);
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
      ),
    );
  }

  Widget _wide(Widget brand, Widget fields) => Center(
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
}

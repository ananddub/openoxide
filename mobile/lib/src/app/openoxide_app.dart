import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/core/theme/app_theme.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/features/auth/screens/boot_gate.dart';

class OpenoxideApp extends StatefulWidget {
  const OpenoxideApp({super.key, required this.deps});

  final Deps deps;

  @override
  State<OpenoxideApp> createState() => _OpenoxideAppState();
}

class _OpenoxideAppState extends State<OpenoxideApp> {
  late final AuthBloc _auth = AuthBloc(
    session: widget.deps.session,
    api: widget.deps.api,
  );
  StreamSubscription<AuthState>? _authSub;

  @override
  void initState() {
    super.initState();
    // The ask that decides which screen appears: public, so no credentials and
    // therefore no reason the launch of the app should wait on a session.
    _auth.add(const AuthBootRequested());
    // The table watch authenticates, so it only makes sense while signed in;
    // leaving it running after a sign-out would reconnect against a dead token
    // every few seconds for as long as the app is alive.
    _authSub = _auth.stream.listen(_followRealtime);
    _followRealtime(_auth.state);
  }

  void _followRealtime(AuthState state) {
    if (state.stage == AppStage.signedIn) {
      widget.deps.realtime.start();
    } else {
      widget.deps.realtime.stop();
    }
  }

  @override
  void dispose() {
    _authSub?.cancel();
    _auth.close();
    widget.deps.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MultiRepositoryProvider(
      providers: [
        RepositoryProvider<Deps>.value(value: widget.deps),
        RepositoryProvider<Session>.value(value: widget.deps.session),
      ],
      child: MultiBlocProvider(
        providers: [BlocProvider<AuthBloc>.value(value: _auth)],
        child: MaterialApp(
          title: 'Openoxide',
          debugShowCheckedModeBanner: false,
          theme: appTheme(Brightness.light),
          darkTheme: appTheme(Brightness.dark),
          home: const BootGate(),
        ),
      ),
    );
  }
}

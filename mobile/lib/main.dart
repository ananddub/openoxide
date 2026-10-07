import 'dart:async';

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'src/session/session.dart';
import 'src/state/auth_controller.dart';
import 'src/state/deps.dart';
import 'src/ui/boot_screen.dart';
import 'src/ui/theme.dart';
import 'src/ui/tokens.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  Motion.install();
  final deps = await Deps.live();
  runApp(
    OpenoxideApp(
      deps: deps,
      controller: AuthController(session: deps.session, api: deps.api),
    ),
  );
}

class OpenoxideApp extends StatefulWidget {
  const OpenoxideApp({super.key, required this.deps, required this.controller});

  final Deps deps;
  final AuthController controller;

  @override
  State<OpenoxideApp> createState() => _OpenoxideAppState();
}

class _OpenoxideAppState extends State<OpenoxideApp> {
  @override
  void initState() {
    super.initState();
    widget.controller.addListener(_followRealtime);
    // The ask that decides which screen appears: public, so no credentials and
    // therefore no reason the launch of the app should wait on a session.
    unawaited(widget.controller.boot());
  }

  /// The table watch authenticates, so it only makes sense while signed in;
  /// leaving it running after a sign-out would reconnect against a dead token
  /// every few seconds for as long as the app is alive.
  void _followRealtime() {
    if (!mounted) return;
    if (widget.controller.stage == AppStage.signedIn) {
      widget.deps.realtime.start();
    } else {
      widget.deps.realtime.stop();
    }
  }

  @override
  void dispose() {
    widget.controller.removeListener(_followRealtime);
    widget.deps.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        Provider<Deps>.value(value: widget.deps),
        ChangeNotifierProvider<AuthController>.value(value: widget.controller),
        ChangeNotifierProvider<Session>.value(value: widget.deps.session),
      ],
      child: MaterialApp(
        title: 'Openoxide',
        debugShowCheckedModeBanner: false,
        theme: appTheme(Brightness.light),
        darkTheme: appTheme(Brightness.dark),
        home: BootScreen(controller: widget.controller),
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/core/theme/app_theme.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';

/// Every width the app is expected to survive, from the smallest handset the
/// panel supports up to a tablet and a landscape phone — the two places a row
/// of chips or dropdowns is most likely to run out of room.
const List<Size> kLayoutSizes = [
  Size(320, 568),
  Size(393, 873),
  Size(800, 1280),
  Size(873, 393),
];

/// The provider set `OpenoxideApp` installs, so a screen under test reads its
/// state exactly the way it does at runtime.
Widget appScope({required Widget child, required AuthBloc auth, Deps? deps}) {
  final scopeDeps = deps;
  return MultiRepositoryProvider(
    providers: [
      if (scopeDeps != null) RepositoryProvider<Deps>.value(value: scopeDeps),
      RepositoryProvider<Session>.value(value: auth.session),
    ],
    child: MultiBlocProvider(
      providers: [BlocProvider<AuthBloc>.value(value: auth)],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: appTheme(Brightness.light),
        darkTheme: appTheme(Brightness.dark),
        home: child,
      ),
    ),
  );
}

/// Pump one screen at each of [kLayoutSizes] and fail on the first size where
/// the layout breaks.
///
/// A `RenderFlex` overflow is reported through `FlutterError.reportError`, which
/// `flutter_test` keeps as a pending exception — so without this check a screen
/// that spills its pixels still passes. Reading it here names the screen and
/// the size instead of letting the failure scroll past.
///
/// [textScalers] is applied through the test platform dispatcher, which the
/// binding's own `MediaQuery` picks up — so it reaches widgets the way the
/// device's "large font" setting does, rather than only those under a wrapper
/// this harness could place. It must be set outside a build: setting it while
/// the tree is building makes `_MediaQueryFromView` mark itself dirty mid-build.
///
/// Every size is visited at every scale, because a row that fits at 1.0x on a
/// wide screen is the same row that has to survive 1.3x on the narrowest one.
Future<void> pumpAtSizes(
  WidgetTester tester,
  Widget Function() buildApp, {
  List<Finder> finds = const [],
  List<Size> sizes = kLayoutSizes,
  List<double> textScalers = const [1.0],
}) async {
  for (final scaler in textScalers) {
    tester.platformDispatcher.textScaleFactorTestValue = scaler;
    addTearDown(tester.platformDispatcher.clearTextScaleFactorTestValue);
    for (final size in sizes) {
      // Awaited: both this and `pumpWidget` run inside `TestAsyncUtils.guard`,
      // and an unawaited call here makes the next pump fail as a nested guard.
      await tester.binding.setSurfaceSize(size);
      addTearDown(() => tester.binding.setSurfaceSize(null));

      await tester.pumpWidget(buildApp());
      await pumpEntrance(tester);

      expectLayoutIntact(tester, 'at $size with ${scaler}x text');
      for (final finder in finds) {
        expect(
          finder,
          findsOneWidget,
          reason: 'expected exactly one match at $size with ${scaler}x text',
        );
      }

      // Unmount before the next size, so a ticker or a scroll position from
      // this iteration cannot be mistaken for the next one's.
      await tester.pumpWidget(const SizedBox.shrink());
    }
  }
}

/// Advance through the entrance animation without ever settling.
///
/// `pumpAndSettle` is unusable app-wide because a screen may hold a looping
/// affordance that never settles, and it would also hide an entrance animation
/// stuck in a loop. The chains here are bounded by [Motion.entranceBudget], so
/// pumping past it is both enough and exact.
Future<void> pumpEntrance(
  WidgetTester tester, {
  Duration budget = Motion.entranceBudget,
  Duration step = const Duration(milliseconds: 100),
}) async {
  for (var elapsed = Duration.zero; elapsed < budget; elapsed += step) {
    await tester.pump(step);
  }
  await tester.pump();
}

/// Fail with every layout error the frames produced, if there were any.
void expectLayoutIntact(WidgetTester tester, String where) {
  final problems = <Object>[];
  for (
    Object? error = tester.takeException();
    error != null;
    error = tester.takeException()
  ) {
    problems.add(error);
  }
  if (problems.isEmpty) return;
  fail('layout broke $where:\n${problems.map((p) => p.toString()).join('\n')}');
}

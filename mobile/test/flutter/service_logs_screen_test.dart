import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/logs/screens/service_logs_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';

import '../fakes.dart';
import 'harness.dart';

/// The long docker-generated name is the point: the app bar row that carries it
/// is the one that used to run out of room on a narrow handset.
const String _target =
    'openoxide_web.1.zq2xk9m4vb7vls0of2fh4n8y1u3d6gjp5tq8wma0cxs';

AuthScope feed({
  List<DeploymentEvent> lines = const [],
  Object? error,
}) {
  final scope = bootedScope(
    initial: const StoredSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      userId: 7,
    ),
    organizations: [organization(3, 'Acme')],
  );
  scope.api
    ..logEvents = lines
    ..logError = error;
  return scope;
}

Widget _screen(AuthScope scope) => appScope(
  child: ServiceLogsScreen(title: 'web', target: _target, serverId: 5),
  auth: scope.bloc,
  deps: fakeDeps(scope.api, scope.session),
);

void main() {
  setUp(Motion.install);

  testWidgets('lays a service log out at every size', (tester) async {
    final scope = feed(
      lines: [
        logLine('listening on 0.0.0.0:8080\n'),
        logLine('[STDERR] error: connection refused to redis:6379\n'),
      ],
    );
    await pumpAtSizes(
      tester,
      () => _screen(scope),
      finds: [find.text('Service'), find.text('Container')],
      textScalers: const [1.0, 1.3],
    );
  });

  testWidgets('the target name is ellipsized, never clipped', (tester) async {
    final scope = feed();
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    final label = tester.widget<Text>(find.text(_target));
    expect(label.overflow, TextOverflow.ellipsis);
    expect(label.maxLines, 1);
  });

  testWidgets('switching to container mode reopens the stream', (tester) async {
    final scope = feed(lines: [logLine('container startup\n')]);
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    expect(
      tester
          .widget<ChoiceChip>(find.widgetWithText(ChoiceChip, 'Container'))
          .selected,
      isFalse,
    );

    await tester.tap(find.text('Container'));
    await pumpEntrance(tester);

    expect(
      tester
          .widget<ChoiceChip>(find.widgetWithText(ChoiceChip, 'Container'))
          .selected,
      isTrue,
    );
    expectLayoutIntact(tester, 'after switching to container logs');
  });

  testWidgets('a stream the panel could not open says so', (tester) async {
    final scope = feed(
      error: const PanelUnreachable('docker is not running on that server'),
    );
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    expect(find.text('Try again'), findsOneWidget);
    expect(find.textContaining('docker is not running'), findsOneWidget);
  });
}

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/remote_server.pb.dart';
import 'package:mobile/src/session/session.dart';
import 'package:mobile/src/state/auth_controller.dart';
import 'package:mobile/src/ui/health_screen.dart';
import 'package:mobile/src/ui/tokens.dart';

import '../fakes.dart';
import 'harness.dart';

/// A signed-in session: the health screen reads nothing but the server list, so
/// the organization is only there to get [AuthController.boot] out of the way.
Future<({AuthController controller, FakePanelApi api})> panelWith({
  List<RemoteServer> servers = const [],
  Map<int, MonitoringServerMetric> serverMetrics = const {},
  List<MonitoringContainerState> containerStates = const [],
  Set<int> unreachableServers = const {},
}) async {
  final scope = authScope(
    initial: const StoredSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      userId: 7,
    ),
    organizations: [organization(3, 'Acme')],
  )
    ..api.servers = servers
    ..api.serverMetrics = serverMetrics
    ..api.containerStates = containerStates
    ..api.unreachableServers = unreachableServers;
  await scope.controller.boot();
  return scope;
}

/// The colour the card actually paints for one status label.
Color _labelColor(WidgetTester tester, String label) {
  final finder = find.text(label);
  expect(finder, findsOneWidget, reason: 'no "$label" label on screen');
  return tester.widget<Text>(finder).style!.color!;
}

void main() {
  setUp(Motion.install);

  testWidgets('lays the server cards out at every size', (tester) async {
    final scope = await panelWith(
      servers: [
        remoteServer(1, 'web-01', ipAddress: '203.0.113.7'),
        remoteServer(
          2,
          'build-01-that-is-long-enough-to-push-a-narrow-card',
          ipAddress: '203.0.113.8',
        ),
      ],
      serverMetrics: {1: serverMetric(), 2: serverMetric(cpu: 98)},
      containerStates: [
        containerState('api'),
        containerState('traefik', state: 'exited'),
        containerState(
          'a-container-name-generated-by-compose-that-keeps-going',
        ),
      ],
    );

    await pumpAtSizes(
      tester,
      () => appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
      finds: [find.text('web-01')],
    );
  });

  testWidgets('reads the agent numbers the way the overview does', (
    tester,
  ) async {
    final scope = await panelWith(
      servers: [remoteServer(1, 'web-01')],
      serverMetrics: {1: serverMetric()},
    );
    await tester.pumpWidget(
      appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('42%'), findsOneWidget);
    // The agent sends memory as a percentage and the used amount in gigabytes;
    // dividing one by the other used to draw a full bar on every machine.
    expect(find.text('4.8 GB'), findsOneWidget);
    expect(find.text('16.0 GB total'), findsOneWidget);
    expect(find.text('50 GB'), findsOneWidget);
    expect(find.text('200 GB total'), findsOneWidget);
  });

  testWidgets('one dead agent does not blank the other server', (
    tester,
  ) async {
    final scope = await panelWith(
      servers: [
        remoteServer(1, 'web-01'),
        remoteServer(
          2,
          'quiet-01',
          serverStatus: RemoteServerStatus.REMOTE_SERVER_STATUS_INACTIVE,
        ),
      ],
      serverMetrics: {1: serverMetric()},
      unreachableServers: {2},
    );
    await tester.pumpWidget(
      appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.byType(Card), findsNWidgets(2));
    expect(find.text('Agent not answering'), findsOneWidget);
    expect(find.text('42%'), findsOneWidget);
  });

  testWidgets('a server that reports nothing says so instead of showing zeros', (
    tester,
  ) async {
    final scope = await panelWith(servers: [remoteServer(1, 'web-01')]);
    await tester.pumpWidget(
      appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('No metrics reported yet'), findsOneWidget);
    expect(find.textContaining('%'), findsNothing);
  });

  testWidgets('colors the status the panel actually sends', (tester) async {
    final scope = await panelWith(
      servers: [
        remoteServer(1, 'web-01'),
        remoteServer(
          2,
          'quiet-01',
          serverStatus: RemoteServerStatus.REMOTE_SERVER_STATUS_INACTIVE,
        ),
      ],
    );
    await tester.pumpWidget(
      appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
    );
    await pumpEntrance(tester);

    // `ACTIVE`/`INACTIVE` is the whole set for a server: matching the word
    // "online" left every card grey no matter what the agent did.
    final ox =
        Theme.of(find.text('Active').evaluate().single)
            .extension<OpenoxideColors>()!;
    expect(_labelColor(tester, 'Active'), ox.success);
    expect(_labelColor(tester, 'Inactive'), ox.danger);
  });

  testWidgets('an account with no servers says where to add one', (
    tester,
  ) async {
    final scope = await panelWith();
    await tester.pumpWidget(
      appScope(
        child: const HealthScreen(),
        auth: scope.controller,
        deps: fakeDeps(scope.api, scope.controller.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('No servers connected'), findsOneWidget);
    expect(find.byType(Card), findsNothing);
  });
}

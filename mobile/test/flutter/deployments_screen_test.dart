import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/deployments/screens/deployments_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';

import '../fakes.dart';
import 'harness.dart';

/// A signed-in session with a deploy queue. The screen asks for the running set
/// and the table's history as two calls, so a test fills each one separately.
AuthScope queue({
  List<Deployment> active = const [],
  List<Deployment> recent = const [],
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
    ..activeDeployments = active
    ..deployments = recent
    ..deploymentsError = error;
  return scope;
}

void main() {
  setUp(Motion.install);

  testWidgets('lays the queue out at every size', (tester) async {
    final scope = queue(
      active: [
        deployment(
          9,
          status: 'BUILDING',
          resourceType: 'a-service-name-long-enough-to-need-ellipsizing',
          userEmail: 'das@example.com',
        ),
      ],
      recent: [
        deployment(8, status: 'DEPLOYED', resourceType: 'website'),
        deployment(
          7,
          status: 'FAILED',
          resourceType: 'worker',
          error: 'the image build exited with code 137 while pushing layers',
        ),
      ],
    );

    await pumpAtSizes(
      tester,
      () => appScope(
        child: const DeploymentsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
      // Headings are upper-cased by the shared label, so this is the text on
      // screen rather than the word in the source.
      finds: [find.text('IN PROGRESS'), find.text('RECENT')],
      textScalers: const [1.0, 1.3],
    );
  });

  testWidgets('the running deploy sits above the history', (tester) async {
    final scope = queue(
      active: [deployment(9, status: 'BUILDING', resourceType: 'api')],
      recent: [deployment(8, status: 'DEPLOYED', resourceType: 'website')],
    );
    await tester.pumpWidget(
      appScope(
        child: const DeploymentsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.byType(Card), findsNWidgets(2));
    expect(
      tester.getTopLeft(find.text('api')).dy,
      lessThan(tester.getTopLeft(find.text('website')).dy),
    );
  });

  testWidgets('an age is read whatever unit the panel wrote it in', (
    tester,
  ) async {
    final scope = queue(
      recent: [
        deployment(8, resourceType: 'seconds-row', ageSeconds: 120),
        deployment(
          7,
          resourceType: 'millis-row',
          ageSeconds: 300,
          milliseconds: true,
        ),
      ],
    );
    await tester.pumpWidget(
      appScope(
        child: const DeploymentsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('2m'), findsOneWidget);
    expect(find.text('5m'), findsOneWidget);
  });

  testWidgets('an empty queue says what to wait for', (tester) async {
    final scope = queue();
    await tester.pumpWidget(
      appScope(
        child: const DeploymentsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('Nothing has been deployed here yet'), findsOneWidget);
    expect(find.byType(Card), findsNothing);
    // Both lists empty means neither heading belongs on screen.
    expect(find.text('IN PROGRESS'), findsNothing);
  });

  testWidgets('a queue that never came back offers a retry that works', (
    tester,
  ) async {
    final scope = queue(error: const PanelUnreachable('no answer'));
    await tester.pumpWidget(
      appScope(
        child: const DeploymentsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('no answer'), findsOneWidget);

    scope.api.deploymentsError = null;
    scope.api.deployments = [deployment(8, resourceType: 'website')];
    await tester.tap(find.text('Try again'));
    await pumpEntrance(tester);

    expect(find.text('website'), findsOneWidget);
    expect(find.text('no answer'), findsNothing);
  });
}

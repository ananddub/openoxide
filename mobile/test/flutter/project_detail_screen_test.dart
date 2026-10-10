import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/projects/screens/project_detail_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/overview.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';

import '../fakes.dart';
import 'harness.dart';

/// A signed-in session on one project. The detail screen asks for the project and
/// then for the organization's whole service and domain lists, so a test says
/// which rows belong to it and which belong to something else.
AuthScope openProject({
  required Project row,
  List<OverviewServiceItem> services = const [],
  List<OverviewDomainItem> domains = const [],
  Object? detailError,
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
    ..projectRow = row
    ..services = services
    ..domains = domains
    ..detailError = detailError;
  return scope;
}

void main() {
  setUp(Motion.install);

  testWidgets('lays the detail out at every size', (tester) async {
    final scope = openProject(
      row: project(
        11,
        'alpha',
        description:
            'A description long enough that a card with a narrow padding '
            'would have to decide between wrapping and overflowing.',
        env: 'A=1\nB=2\nC=3',
      ),
      services: [
        serviceItem(1, 'api', 11, status: 'restarting'),
        serviceItem(2, 'worker', 11, serverName: ''),
      ],
      domains: [
        domainItem(
          1,
          'api.example.com-a-domain-long-enough-to-be-a-problem',
          'alpha',
        ),
      ],
    );

    await pumpAtSizes(
      tester,
      () => appScope(
        child: const ProjectDetailScreen(projectId: 11),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
      finds: [find.text('alpha')],
    );
  });

  testWidgets('shows only this project, and counts its variables', (
    tester,
  ) async {
    final scope = openProject(
      row: project(11, 'alpha', env: 'A=1\n# a comment\n\nB=2\n'),
      services: [serviceItem(1, 'api', 11), serviceItem(2, 'not-mine', 12)],
      domains: [
        domainItem(1, 'alpha.example.com', 'alpha', path: '/v1'),
        domainItem(2, 'beta.example.com', 'beta'),
      ],
    );
    await tester.pumpWidget(
      appScope(
        child: const ProjectDetailScreen(projectId: 11),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('api'), findsOneWidget);
    expect(find.text('not-mine'), findsNothing);
    expect(find.text('alpha.example.com'), findsOneWidget);
    expect(find.text('beta.example.com'), findsNothing);
    expect(find.text('2 env variables  ·  11'), findsOneWidget);
  });

  testWidgets('a project with nothing running says so twice', (tester) async {
    final scope = openProject(row: project(11, 'alpha'));
    await tester.pumpWidget(
      appScope(
        child: const ProjectDetailScreen(projectId: 11),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('No services in this project yet'), findsOneWidget);
    expect(find.text('No domains point at this project'), findsOneWidget);
    expect(find.byType(ListTile), findsNothing);
  });

  testWidgets('a domain row hands over the whole address', (tester) async {
    final scope = openProject(
      row: project(11, 'alpha'),
      domains: [domainItem(1, 'alpha.example.com', 'alpha', path: '/v1')],
    );
    await tester.pumpWidget(
      appScope(
        child: const ProjectDetailScreen(projectId: 11),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    await tester.tap(find.text('alpha.example.com'));
    await tester.pump();
    expect(find.text('https://alpha.example.com/v1'), findsOneWidget);

    // Pumped in frames rather than one long jump: the snack bar closes itself
    // after three seconds and its exit needs the frames to finish, and leaving
    // that timer running would fail the test for a reason the screen caused.
    for (var frame = 0; frame < 45; frame++) {
      await tester.pump(const Duration(milliseconds: 100));
    }
    expect(find.text('https://alpha.example.com/v1'), findsNothing);
  });

  testWidgets('a detail load that never came back offers a retry that works', (
    tester,
  ) async {
    final scope = openProject(
      row: project(11, 'alpha'),
      detailError: const PanelUnreachable('no answer'),
    );
    await tester.pumpWidget(
      appScope(
        child: const ProjectDetailScreen(projectId: 11),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('no answer'), findsOneWidget);

    scope.api.detailError = null;
    scope.api.services = [serviceItem(1, 'api', 11)];
    await tester.tap(find.text('Try again'));
    await pumpEntrance(tester);

    expect(find.text('api'), findsOneWidget);
    expect(find.text('no answer'), findsNothing);
  });
}

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/projects/screens/projects_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart'

    show WatchTablesEvent;
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart';

import '../fakes.dart';
import 'harness.dart';

/// A session already inside the first organization, which is the state the shell
/// hands this screen: the boot resolves an organization before the signed-in
/// screens make their first list call.
AuthScope inOrganization({
  List<Project> projects = const [],
  Object? projectsError,
}) {
  final scope = bootedScope(
    initial: const StoredSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      userId: 7,
    ),
    organizations: [organization(3, 'Acme')],
    projects: projects,
  );
  scope.api.projectsError = projectsError;
  return scope;
}

/// Drives the screen the way the panel does: open the watch stream and hand back
/// the deps a test moves a table on.
({Deps deps, StreamController<WatchTablesEvent> ticks}) watchProjects(
  FakePanelApi api,
  Session session,
) {
  final ticks = StreamController<WatchTablesEvent>.broadcast();
  final deps = fakeDeps(api, session, ticks: ticks.stream);
  deps.realtime.start();
  // Closed in this order on purpose: stopping first means the watch is not
  // listening when the stream ends, so it schedules no reconnect timer that
  // would outlive the test.
  addTearDown(() {
    deps.realtime.stop();
    ticks.close();
  });
  return (deps: deps, ticks: ticks);
}

/// A tick is debounced for 400ms so a burst around one deploy costs one fetch; a
/// test has to outlive that window before the screen refetches.
Future<void> tickProjects(
  WidgetTester tester,
  StreamController<WatchTablesEvent> ticks,
) async {
  ticks.add(WatchTablesEvent(tables: [WatchedTable.WATCHED_TABLE_PROJECTS]));
  await tester.pump();
  await tester.pump(const Duration(milliseconds: 500));
  await tester.pump();
}

void main() {
  setUp(Motion.install);

  testWidgets('lays the list out at every size', (tester) async {
    final scope = inOrganization(
      projects: [
        project(
          11,
          'alpha',
          description:
              'A description long enough that a card with a narrow padding '
              'would have to decide between wrapping and overflowing.',
          env: 'A=1\nB=2',
        ),
      ],
    );
    final deps = fakeDeps(scope.api, scope.session);

    await pumpAtSizes(
      tester,
      () =>
          appScope(child: const ProjectsScreen(), auth: scope.bloc, deps: deps),
      finds: [find.text('alpha')],
    );
  });

  testWidgets('counts the variables, not the characters of the blob', (
    tester,
  ) async {
    final scope = inOrganization(
      projects: [project(11, 'alpha', env: 'A=1\n# a comment\n\nB=2\n')],
    );
    await tester.pumpWidget(
      appScope(
        child: const ProjectsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('2 env variables  ·  11'), findsOneWidget);
  });

  testWidgets('an organization with no projects says so', (tester) async {
    final scope = inOrganization();
    await tester.pumpWidget(
      appScope(
        child: const ProjectsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('No projects yet'), findsOneWidget);
    expect(find.byType(Card), findsNothing);
  });

  testWidgets('a first load that never came back offers a retry that works', (
    tester,
  ) async {
    final scope = inOrganization(
      projectsError: const PanelUnreachable('no answer'),
    );
    await tester.pumpWidget(
      appScope(
        child: const ProjectsScreen(),
        auth: scope.bloc,
        deps: fakeDeps(scope.api, scope.session),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('no answer'), findsOneWidget);

    scope.api.projectsError = null;
    scope.api.projects = [project(11, 'alpha')];
    await tester.tap(find.text('Try again'));
    await pumpEntrance(tester);

    expect(find.text('alpha'), findsOneWidget);
    expect(find.text('no answer'), findsNothing);
  });

  testWidgets('a tick refetches, and a failed refetch keeps the rows', (
    tester,
  ) async {
    final scope = inOrganization(projects: [project(11, 'alpha')]);
    final watch = watchProjects(scope.api, scope.session);
    await tester.pumpWidget(
      appScope(
        child: const ProjectsScreen(),
        auth: scope.bloc,
        deps: watch.deps,
      ),
    );
    await pumpEntrance(tester);

    scope.api.projects = [project(11, 'alpha'), project(12, 'beta')];
    await tickProjects(tester, watch.ticks);
    expect(find.text('beta'), findsOneWidget);

    // The panel moved the table again and then went away: the reader keeps what
    // is already on screen rather than losing it to a retry button.
    scope.api.projectsError = const PanelUnreachable('no answer');
    await tickProjects(tester, watch.ticks);
    expect(find.text('beta'), findsOneWidget);
    expect(find.text('no answer'), findsNothing);
  });
}

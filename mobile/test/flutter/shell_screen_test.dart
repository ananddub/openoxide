import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/features/shell/shell_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';

import '../fakes.dart';
import 'harness.dart';

/// A signed-in shell. All three tabs live in the widget tree at once, so a test
/// has to give every screen an answer: the project list, an empty deploy queue
/// and no servers.
AuthScope shell({List<Organization> organizations = const []}) {
  final scope = bootedScope(
    initial: const StoredSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      userId: 7,
    ),
    organizations: organizations.isEmpty
        ? [organization(3, 'Acme')]
        : organizations,
    projects: [project(11, 'alpha')],
  );
  return scope;
}

Widget _opened(AuthScope scope) => appScope(
  child: const ShellScreen(),
  auth: scope.bloc,
  deps: fakeDeps(scope.api, scope.session),
);

void main() {
  setUp(Motion.install);

  testWidgets('lays the shell out at every size', (tester) async {
    final scope = shell();
    await pumpAtSizes(
      tester,
      () => _opened(scope),
      finds: [find.text('alpha'), find.byType(FloatingActionButton)],
    );
  });

  testWidgets('the sign-out control is a real touch target', (tester) async {
    final scope = shell();
    await tester.pumpWidget(_opened(scope));
    await pumpEntrance(tester);

    // It used to be a `.small` FAB, which measures 40dp — under the minimum
    // Android asks for and under what every button in this app is held to.
    final size = tester.getSize(find.byType(FloatingActionButton));
    expect(size.width, greaterThanOrEqualTo(48));
    expect(size.height, greaterThanOrEqualTo(48));
  });

  testWidgets('switching tabs keeps each screen loaded', (tester) async {
    final scope = shell();
    await tester.pumpWidget(_opened(scope));
    await pumpEntrance(tester);

    expect(find.text('alpha'), findsOneWidget);
    expect(find.text('Projects'), findsNWidgets(2));

    await tester.tap(find.text('Deploys'));
    await pumpEntrance(tester);
    expect(find.text('Deployments'), findsOneWidget);
    expect(find.text('Nothing has been deployed here yet'), findsOneWidget);

    await tester.tap(find.text('Servers'));
    await pumpEntrance(tester);
    expect(find.text('No servers connected'), findsOneWidget);

    await tester.tap(find.text('Projects'));
    await pumpEntrance(tester);
    expect(find.text('alpha'), findsOneWidget);
  });

  testWidgets('the menu switches the organization every screen reads', (
    tester,
  ) async {
    final scope = shell(
      organizations: [organization(3, 'Acme'), organization(9, 'Beta')],
    );

    await tester.pumpWidget(_opened(scope));
    await pumpEntrance(tester);
    expect(scope.bloc.currentOrganization?.name, 'Acme');

    await tester.tap(find.byType(PopupMenuButton<int>));
    await pumpEntrance(tester);
    expect(find.text('Beta'), findsOneWidget);

    await tester.tap(find.text('Beta'));
    await pumpEntrance(tester);

    expect(scope.bloc.currentOrganization?.name, 'Beta');
    expect(scope.session.organizationId, 9);
    // The tab stays mounted when the organization changes, so nothing would
    // refetch unless the screen re-reads the organization it lists inside of.
    expect(scope.api.projectListRequests, [3, 9]);
  });

  testWidgets('an account with no organization can reload into one', (
    tester,
  ) async {
    final scope = bootedScope(
      initial: const StoredSession(
        accessToken: 'access',
        refreshToken: 'refresh',
        userId: 7,
      ),
      projects: [project(11, 'alpha')],
    );

    await tester.pumpWidget(_opened(scope));
    await pumpEntrance(tester);
    expect(scope.bloc.organizationId, isNull);

    expect(
      find.text('You are not a member of any organization on this panel yet.'),
      findsOneWidget,
    );
    expect(
      tester.getSize(find.byType(ActionButton)).height,
      greaterThanOrEqualTo(48),
    );

    scope.api.organizations = [organization(9, 'Beta')];
    await tester.tap(find.text('Reload'));
    await pumpEntrance(tester);

    expect(
      find.text('You are not a member of any organization on this panel yet.'),
      findsNothing,
    );
    expect(find.text('alpha'), findsOneWidget);
  });

  testWidgets('the floating button signs the session out', (tester) async {
    final scope = shell();
    await tester.pumpWidget(_opened(scope));
    await pumpEntrance(tester);

    await tester.tap(find.byType(FloatingActionButton));
    await pumpEntrance(tester);

    expect(scope.bloc.state.stage, AppStage.signedOut);
    expect(scope.session.isSignedIn, isFalse);
  });
}

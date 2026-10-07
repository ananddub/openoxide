import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';
import 'package:mobile/src/session/session.dart';
import 'package:mobile/src/state/auth_controller.dart';
import 'package:mobile/src/state/deployment_status.dart';
import 'package:mobile/src/ui/deployment_logs_screen.dart';
import 'package:mobile/src/ui/theme.dart';
import 'package:mobile/src/ui/tokens.dart';

import '../fakes.dart';
import 'harness.dart';

Future<({AuthController controller, FakePanelApi api})> deploy({
  Deployment? row,
  List<DeploymentEvent> lines = const [],
}) async {
  final scope = authScope(
    initial: const StoredSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      userId: 7,
    ),
    organizations: [organization(3, 'Acme')],
  )
    ..api.deploymentRow = row
    ..api.logEvents = lines;
  await scope.controller.boot();
  return scope;
}

Widget _screen(
  ({AuthController controller, FakePanelApi api}) scope,
) => appScope(
  // A title that is not any of the row's own strings, so a finder on the
  // resource type cannot also match the app bar.
  child: const DeploymentLogsScreen(deploymentId: 4, title: 'Deploy 4'),
  auth: scope.controller,
  deps: fakeDeps(scope.api, scope.controller.session),
);

void main() {
  setUp(Motion.install);

  testWidgets('lays a deploy log out at every size', (tester) async {
    final scope = await deploy(
      row: deployment(
        4,
        status: 'BUILDING',
        resourceType: 'a-resource-type-long-enough-to-need-ellipsizing',
      ),
      lines: [logLine('#7 3.241 fetching go.mod for a dependency graph that is\n', stage: 'build')],
    );
    await pumpAtSizes(
      tester,
      () => _screen(scope),
      finds: [find.text('BUILDING')],
    );
  });

  testWidgets('the bar carries the status, the resource and the failure', (
    tester,
  ) async {
    final scope = await deploy(
      row: deployment(
        4,
        status: 'FAILED',
        resourceType: 'website',
        error: 'the image build exited with code 137',
      ),
      lines: [logLine('killed\n')],
    );
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    expect(find.text('FAILED'), findsOneWidget);
    expect(find.text('website'), findsOneWidget);
    expect(find.textContaining('exited with code 137'), findsOneWidget);
  });

  testWidgets('the status colour is the one the queue uses', (tester) async {
    final scope = await deploy(
      row: deployment(4, status: 'FAILED', resourceType: 'website'),
    );
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    final colors = phaseColors(DeploymentPhase.failed, Brightness.light);
    expect(tester.widget<Text>(find.text('FAILED')).style?.color, colors.text);
  });

  testWidgets('a deploy that produced nothing says the stream ended', (
    tester,
  ) async {
    final scope = await deploy(row: deployment(4, status: 'DEPLOYED'));
    await tester.pumpWidget(_screen(scope));
    await pumpEntrance(tester);

    // The fake closes its stream, as the panel does once a deploy settles, so
    // this is the state a finished build really lands in.
    expect(find.text('No output for this deployment'), findsOneWidget);
  });
}

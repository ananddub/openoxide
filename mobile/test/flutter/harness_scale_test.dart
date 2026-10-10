import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/tokens/motion.dart';

import '../fakes.dart';
import 'harness.dart';

/// How tall a 20px line becomes with the harness's scaler applied.
///
/// This uses the same two calls `pumpAtSizes` makes — the test dispatcher value,
/// then a pump through `appScope` — so it measures the real path, not a
/// paraphrase of it.
Future<double> _heightOfScaledLine(WidgetTester tester, double scaler) async {
  final scope = authScope();
  tester.platformDispatcher.textScaleFactorTestValue = scaler;
  addTearDown(tester.platformDispatcher.clearTextScaleFactorTestValue);
  await tester.pumpWidget(
    appScope(
      child: const Center(child: Text('rhythm', style: TextStyle(fontSize: 20))),
      auth: scope.bloc,
    ),
  );
  return tester.getRect(find.text('rhythm')).height;
}

void main() {
  setUp(Motion.install);

  // A layout dimension that silently stops applying is worse than no dimension:
  // every 1.3x test would keep passing while only ever re-running the 1.0x
  // layout. This is the guard for that.
  testWidgets('the text-scale dimension reaches the widget tree', (
    WidgetTester tester,
  ) async {
    final unscaled = await _heightOfScaledLine(tester, 1.0);
    final scaled = await _heightOfScaledLine(tester, 1.3);

    expect(unscaled, greaterThan(0));
    // Not exactly 1.3x: line height rounds through font metrics. Anything well
    // above 1.0 is proof the scaler travelled from the dispatcher to the text.
    expect(
      scaled / unscaled,
      greaterThan(1.25),
      reason: 'the scaler left a 1.0x line at ${unscaled}px and a 1.3x one at ${scaled}px',
    );
  });
}

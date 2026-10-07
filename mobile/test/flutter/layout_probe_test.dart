import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'harness.dart';

/// Why this file exists: every `pumpAtSizes` check leans on `flutter_test`
/// turning a `RenderFlex` overflow into a pending exception. If that ever stops
/// happening — a framework change, a different binding — the whole responsive
/// suite would go silently green while the screens break. This pins the
/// mechanism the harness assumes.
void main() {
  testWidgets('an overflowing row is reported, not tolerated', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Align(
          alignment: Alignment.topLeft,
          child: SizedBox(
            width: 100,
            child: Row(
              children: [SizedBox(width: 80, height: 10), SizedBox(width: 80, height: 10)],
            ),
          ),
        ),
      ),
    );

    final error = tester.takeException();
    expect(error, isNotNull);
    expect('$error', contains('overflowed'));
  });

  testWidgets('the same row passes once it can share the width',
    (tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Align(
            alignment: Alignment.topLeft,
            child: SizedBox(
              width: 100,
              child: Row(
                children: [
                  Expanded(child: SizedBox(height: 10)),
                  Expanded(child: SizedBox(height: 10)),
                ],
              ),
            ),
          ),
        ),
      );

      expectLayoutIntact(tester, 'at 100 logical pixels');
    });
}

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/logs/log_buffer.dart';
import 'package:mobile/src/logs/log_model.dart';
import 'package:mobile/src/ui/log_view.dart';
import 'package:mobile/src/ui/theme.dart';
import 'package:mobile/src/ui/tokens.dart';

import 'harness.dart';

/// The pane the way the app renders it: inside the real theme, because it reads
/// its colours from the extension, and fed through the real buffer so the lines
/// arrive as parsed entries rather than as a fixture shaped by hand.
Widget _pane(List<String> lines, {int dropped = 0}) {
  final buffer = LogBuffer()..append('${lines.join('\n')}\n');
  return MaterialApp(
    debugShowCheckedModeBanner: false,
    theme: appTheme(Brightness.light),
    darkTheme: appTheme(Brightness.dark),
    home: Scaffold(
      body: LogView(entries: buffer.entries, droppedLines: dropped),
    ),
  );
}

void main() {
  setUp(Motion.install);

  testWidgets('lays a hostile log line out at every size', (tester) async {
    await pumpAtSizes(
      tester,
      () => _pane([
        'building image for service web with a message long enough that it has '
            'to wrap several times before the pane can show all of it, which it '
            'does by breaking the line wherever it runs out of room',
        '[STDERR] error: failed to bind 0.0.0.0:8080 — address already in use',
        '── progress ────────────────────────────────',
      ]),
      finds: [find.text('3 lines')],
      textScalers: const [1.0, 1.3],
    );
  });

  testWidgets('the stream filter narrows the pane and the count follows', (
    tester,
  ) async {
    await tester.pumpWidget(
      _pane([
        'pulling layer 1/7',
        '[STDERR] error: manifest unknown',
        'pulling layer 2/7',
      ]),
    );
    await pumpEntrance(tester);

    expect(find.textContaining('manifest unknown'), findsOneWidget);

    await tester.tap(find.byType(DropdownButton<StreamType>));
    await pumpEntrance(tester);
    await tester.tap(find.text('stderr'));
    await pumpEntrance(tester);

    expect(find.text('1 lines'), findsOneWidget);
    expect(find.textContaining('pulling layer'), findsNothing);
    expect(find.textContaining('manifest unknown'), findsOneWidget);
  });

  testWidgets('a drop is announced rather than hidden', (tester) async {
    await tester.pumpWidget(_pane(['first line'], dropped: 40));
    await pumpEntrance(tester);

    expect(find.textContaining('40 older lines dropped'), findsOneWidget);
  });

  testWidgets('an empty pane says what it is waiting for', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        theme: appTheme(Brightness.light),
        home: const Scaffold(
          body: LogView(entries: [], emptyHint: 'Waiting for output…'),
        ),
      ),
    );
    await pumpEntrance(tester);

    expect(find.text('Waiting for output…'), findsOneWidget);
    expect(find.text('0 lines'), findsOneWidget);
  });
}

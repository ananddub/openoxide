import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/logs/ansi_sgr.dart';

const esc = '\x1B';

void main() {
  group('parseAnsi', () {
    test('leaves plain text as one unstyled span', () {
      final spans = parseAnsi('nothing styled');
      expect(spans.single.text, 'nothing styled');
      expect(spans.single.styles, isEmpty);
      expect(spans.single.color, isNull);
    });

    test('applies a foreground colour until the reset', () {
      final spans = parseAnsi('$esc[31mfailed$esc[0m ok');
      expect(spans.length, 2);
      expect(spans.first.color, AnsiColor.red);
      expect(spans.first.text, 'failed');
      expect(spans.last.color, isNull);
      expect(spans.last.text.trim(), 'ok');
    });

    test('carries styles across codes and clears on reset', () {
      final spans = parseAnsi('$esc[1m$esc[4mbold underline$esc[0mdull');
      expect(spans.first.styles, {AnsiStyle.bold, AnsiStyle.underline});
      expect(spans.last.styles, isEmpty);
    });

    test('maps the bright palette', () {
      final spans = parseAnsi('$esc[90mdim grey$esc[0m');
      expect(spans.single.color, AnsiColor.brightBlack);
    });

    test('supports codes given in a single sequence', () {
      final spans = parseAnsi('$esc[1;33mwarn$esc[0m');
      expect(spans.single.styles, {AnsiStyle.bold});
      expect(spans.single.color, AnsiColor.yellow);
    });

    test('ignores 256-colour and true-colour sequences', () {
      final spans = parseAnsi('$esc[38;5;196mred?$esc[0m');
      // 38 and 5 are not in the supported subset, so the text stays uncoloured
      // rather than being guessed at.
      expect(spans.single.color, isNull);
      expect(spans.single.text, 'red?');
    });
  });

  group('stripAnsi', () {
    test('removes every csi sequence, not only colours', () {
      expect(stripAnsi('$esc[2K$esc[1;1Hcleared'), 'cleared');
    });

    test('hasAnsi only reports true for sgr', () {
      expect(hasAnsi('$esc[31mred'), isTrue);
      expect(hasAnsi('$esc[2K'), isFalse);
      expect(hasAnsi('plain'), isFalse);
    });
  });
}

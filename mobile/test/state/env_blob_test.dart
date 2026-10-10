import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/domain/env_blob.dart';

void main() {
  group('envVariableCount', () {
    test('counts assignment lines', () {
      expect(envVariableCount('A=1\nB=2'), 2);
      expect(envVariableCount('A=1\r\nB=2'), 2);
    });

    test('an empty blob has none', () {
      expect(envVariableCount(''), 0);
      expect(envVariableCount('\n\n'), 0);
    });

    test('a value may be empty, a key may not', () {
      expect(envVariableCount('A='), 1);
      expect(envVariableCount('=1'), 0);
    });

    test('comments, blank lines and stray text are not variables', () {
      expect(envVariableCount('# note\n; other\n\nA=1\nnot an assignment'), 1);
    });

    test('an export prefix still counts once', () {
      expect(envVariableCount('export A=1\nA=2'), 2);
    });

    test('a value containing = is still one variable', () {
      expect(envVariableCount('DATABASE_URL=postgres://a=b@c'), 1);
    });
  });
}

import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/logs/log_buffer.dart';

void main() {
  group('LogChunker', () {
    test('holds a partial line until its newline arrives', () {
      final chunker = LogChunker();
      expect(chunker.push('buildin'), isEmpty);
      expect(chunker.push('g started\nrest'), ['building started']);
      expect(chunker.partial, 'rest');
    });

    test('splits several lines from one chunk and keeps the remainder', () {
      final chunker = LogChunker();
      expect(chunker.push('a\nb\nc'), ['a', 'b']);
      expect(chunker.flush(), ['c']);
      expect(chunker.partial, isEmpty);
    });

    test('drops carriage returns and empty or keep-alive lines', () {
      final chunker = LogChunker();
      expect(chunker.push('one\r\ntwo\r\n'), ['one', 'two']);
      expect(chunker.push('\n\n'), isEmpty);
      expect(chunker.push(': keep-alive\n'), isEmpty);
      expect(chunker.push('event: log\n'), isEmpty);
    });

    test('flush is empty when the stream ended on a newline', () {
      final chunker = LogChunker()..push('done\n');
      expect(chunker.flush(), isEmpty);
    });
  });

  group('LogBuffer', () {
    test('numbers entries in arrival order and returns only new ones', () {
      final buffer = LogBuffer(capacity: 10);
      final first = buffer.append('hello\nworld\n');
      expect(first.map((entry) => entry.id), ['log-0', 'log-1']);
      final second = buffer.append('third\n');
      expect(second.map((entry) => entry.id), ['log-2']);
      expect(buffer.entries.length, 3);
    });

    test('carries the stage of the event that produced the line', () {
      final buffer = LogBuffer()..append('compiling\n', stage: 'BUILDING');
      expect(buffer.entries.single.stage, 'BUILDING');
    });

    test('an error payload becomes a line of its own', () {
      final buffer = LogBuffer()..appendError('ERROR: build failed');
      final entry = buffer.entries.single;
      // The leading level tag is the parser's to strip; the raw line is kept.
      expect(entry.clean, 'build failed');
      expect(entry.raw, 'ERROR: build failed');
      expect(entry.isError, isTrue);
    });

    test('drops the oldest lines past the capacity and counts them', () {
      final buffer = LogBuffer(capacity: 3);
      buffer.append('1\n2\n3\n4\n5\n');
      expect(buffer.entries.map((entry) => entry.clean), ['3', '4', '5']);
      expect(buffer.droppedLines, 2);
    });

    test('a final line without a newline still lands', () {
      final buffer = LogBuffer()..append('tail');
      expect(buffer.entries, isEmpty);
      buffer.finish();
      expect(buffer.entries.single.clean, 'tail');
    });

    test('clear resets lines, numbering and the drop count', () {
      final buffer = LogBuffer(capacity: 2)
        ..append('1\n2\n3\n4\n')
        ..clear();
      expect(buffer.isEmpty, isTrue);
      expect(buffer.droppedLines, 0);
      buffer.append('fresh\n');
      expect(buffer.entries.single.id, 'log-0');
    });
  });
}

import 'dart:collection';

import 'log_model.dart';
import 'log_parser.dart';

/// Turns a stream of wire chunks into whole lines.
///
/// The panel flushes whenever it has bytes, so a chunk can end mid-line, carry
/// several lines, or be a lone `\n` keep-alive. Everything after the last
/// newline is held back until the newline that completes it arrives.
class LogChunker {
  String _partial = '';

  /// The text still waiting for its newline. Only useful for a tail preview.
  String get partial => _partial;

  /// Feed one payload; get back the lines it completed, oldest first.
  List<String> push(String payload) {
    if (payload.isEmpty) return const [];
    _partial += payload.replaceAll('\r', '');
    final lines = <String>[];
    var cut = _partial.indexOf('\n');
    while (cut >= 0) {
      final line = _partial.substring(0, cut);
      _partial = _partial.substring(cut + 1);
      if (cleanLogLine(line).isNotEmpty) lines.add(line);
      cut = _partial.indexOf('\n');
    }
    return lines;
  }

  /// Take the trailing fragment as a line, for when the channel closes without
  /// a final newline.
  List<String> flush() {
    final line = _partial;
    _partial = '';
    return cleanLogLine(line).isEmpty ? const [] : [line];
  }

  void reset() => _partial = '';
}

/// The rendered log: parsed lines, newest kept when the cap is hit.
///
/// A long build can emit far more than a phone can hold, so the oldest lines are
/// dropped and the drop is recorded — a pane that silently scrolled away history
/// is worse than one that says how much it lost.
class LogBuffer {
  LogBuffer({this.capacity = 2000});

  final int capacity;
  final LogChunker _chunker = LogChunker();
  final Queue<ParsedLogEntry> _entries = Queue<ParsedLogEntry>();
  int _index = 0;
  int _dropped = 0;

  UnmodifiableListView<ParsedLogEntry> get entries =>
      UnmodifiableListView<ParsedLogEntry>(_entries);

  int get droppedLines => _dropped;

  bool get isEmpty => _entries.isEmpty;

  /// Append the lines one payload completed; returns only the new entries.
  List<ParsedLogEntry> append(String payload, {String? stage}) =>
      _addAll(_chunker.push(payload), stage: stage);

  /// An `error` field on an event is output in its own right, not metadata.
  List<ParsedLogEntry> appendError(String error, {String? stage}) =>
      _addAll([error], stage: stage);

  /// Call when the stream ends: a last line without a newline is still a line.
  List<ParsedLogEntry> finish() => _addAll(_chunker.flush());

  void clear() {
    _chunker.reset();
    _entries.clear();
    _index = 0;
    _dropped = 0;
  }

  List<ParsedLogEntry> _addAll(List<String> lines, {String? stage}) {
    if (lines.isEmpty) return const [];
    final added = [
      for (final line in lines) parseLogEntry(line, _index++, stage: stage),
    ];
    _entries.addAll(added);
    while (_entries.length > capacity) {
      _entries.removeFirst();
      _dropped++;
    }
    return added;
  }
}

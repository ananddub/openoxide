/// The ANSI subset the panel's own log viewer renders: SGR (`CSI … m`) codes for
/// reset, bold, dim, underline and the 8/16-colour palette. No 256-colour or
/// true-colour sequences — build output from the deploy pipeline never emits
/// them, and supporting them here would only make the mapping harder to test.
enum AnsiStyle { bold, dim, underline }

enum AnsiColor {
  black,
  red,
  green,
  yellow,
  blue,
  magenta,
  cyan,
  white,
  brightBlack,
  brightRed,
  brightGreen,
  brightYellow,
  brightBlue,
  brightMagenta,
  brightCyan,
  brightWhite,
}

class AnsiSpan {
  const AnsiSpan({required this.text, this.styles = const {}, this.color});

  final String text;
  final Set<AnsiStyle> styles;
  final AnsiColor? color;

  @override
  String toString() => 'AnsiSpan($text, $styles, $color)';
}

// The escape byte is introduced by design: log payloads carry real ANSI codes.
final _sgr = RegExp(r'\x1B\[([0-9;]*)m');
final _anyCsi = RegExp(r'\x1B\[[0-?]*[ -/]*[@-~]');

bool hasAnsi(String text) => _sgr.hasMatch(text);

/// Every CSI sequence, not just SGR, so cursor and erase codes disappear from
/// the line instead of showing as literal glyphs.
String stripAnsi(String text) => text.replaceAll(_anyCsi, '');

/// Splits [text] into styled runs, carrying the active attributes across
/// sequences the way a terminal would.
List<AnsiSpan> parseAnsi(String text) {
  if (text.isEmpty || !hasAnsi(text)) {
    return text.isEmpty ? const [] : [AnsiSpan(text: text)];
  }

  final spans = <AnsiSpan>[];
  final styles = <AnsiStyle>{};
  AnsiColor? color;
  var lastIndex = 0;

  void emit(String chunk) {
    if (chunk.isEmpty) return;
    spans.add(
      AnsiSpan(
        text: chunk,
        styles: Set<AnsiStyle>.unmodifiable(styles),
        color: color,
      ),
    );
  }

  for (final match in _sgr.allMatches(text)) {
    emit(text.substring(lastIndex, match.start));

    for (final codeText in match.group(1)!.split(';')) {
      final code = int.tryParse(codeText);
      if (code == null || code == 0) {
        styles.clear();
        color = null;
      } else if (code == 1) {
        styles.add(AnsiStyle.bold);
      } else if (code == 2) {
        styles.add(AnsiStyle.dim);
      } else if (code == 4) {
        styles.add(AnsiStyle.underline);
      } else if (code >= 30 && code <= 37) {
        color = AnsiColor.values[code - 30];
      } else if (code >= 90 && code <= 97) {
        color = AnsiColor.values[8 + (code - 90)];
      }
    }

    lastIndex = match.end;
  }

  emit(text.substring(lastIndex));
  return spans;
}

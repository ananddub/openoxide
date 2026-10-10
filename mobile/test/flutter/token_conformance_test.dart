import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

/// The scales the design system owns: colour, spacing, radii, type, motion and
/// the widths the layout chooses for itself.
///
/// This guard exists because a screen that writes `EdgeInsets.all(14)` passes
/// every other test in the suite. `mobile/` shipped with six different horizontal
/// paddings, four unrelated corner radii and ten hand-rolled buttons before the
/// scale was drawn, and nothing could tell — so the scale is only durable while
/// something fails the moment a widget reaches around it.
///
/// `tokens.dart` and `theme.dart` are exempt: they are where those values are
/// allowed to be written down. Everything else takes them from `Spacing`,
/// `Radii`, `Mark`, `Fonts`, `Motion`, `Layout` or `context.ox`.
void main() {
  test('no screen writes a value the token system already owns', () {
    final lib = Directory('lib');
    // Every rule below reads nothing if the base path is wrong, and an empty
    // scan looks exactly like a clean one.
    expect(lib.existsSync(), isTrue, reason: 'run from the package root');

    final sources = _sources(lib);
    expect(sources, hasLength(greaterThan(20)));

    final breaches = [
      for (final source in sources)
        for (final problem in _problemsIn(source)) //
          '${source.path}:${problem.line}: ${problem.why}',
    ];

    expect(
      breaches,
      isEmpty,
      reason: 'values bypassing the design system:\n${breaches.join('\n')}',
    );
  });

  test('the sanctioned teal is declared once', () {
    // Accent teal belongs to `OpenoxideColors.accent` and nothing else: a screen
    // that re-declares it is a fourth teal the app does not want. The log
    // palette in `lib/src/logs` is excluded — those are eleven phase hues
    // matching the web's, not the app's chrome.
    final ui = Directory('lib/src/core/theme')
        .listSync(recursive: true)
        .whereType<File>()
        .where((f) => f.path.endsWith('.dart'));
    final offenders = [
      for (final file in ui) //
        if (file.readAsStringSync().contains(_teal)) _basename(file.path),
    ];

    expect(offenders, [
      'openoxide_colors.dart',
    ], reason: '$_teal may only appear in the colour tokens');
  });

  test('the guard fails on exactly what it claims to own', () {
    // A file-walking check that never matches anything is decoration, so these
    // cases are the guard's own tests: the first list must all be caught, the
    // second must all stay silent.
    const caught = {
      'padding: const EdgeInsets.all(14)': 'spacing',
      'borderRadius: BorderRadius.circular(6)': 'corner radius',
      'duration: const Duration(milliseconds: 250)': 'duration',
      'style: TextStyle(fontSize: 13)': 'font size',
      'height: 1.6,': 'line height',
      'color: const Color(0xFF112233)': 'raw colour',
      '.slideY(begin: 0.25)': 'animation',
      'on: Colors.white.withValues(alpha: 0.5)': 'raw alpha',
      'withOpacity(0.5)': 'deprecated',
      'const BoxConstraints(maxWidth: 360)': 'layout width',
    };
    for (final entry in caught.entries) {
      expect(
        _problemsIn(_ui(entry.key)),
        isNotEmpty,
        reason: '${entry.key} should read as ${entry.value}',
      );
    }

    const sanctioned = [
      'padding: const EdgeInsets.only(top: Spacing.xxs)',
      'constraints: const BoxConstraints(maxWidth: _nameMaxWidth)',
      'borderRadius: BorderRadius.circular(Radii.dot)',
      'duration: Motion.pulse',
      'fontSize: Fonts.logSize',
      'color: context.ox.accent',
      'child.animate(delay: Motion.delay(5)).fadeIn(duration: Motion.base)',
      'constraints: const BoxConstraints(maxWidth: Layout.measure)',
    ];
    for (final sample in sanctioned) {
      expect(_problemsIn(_ui(sample)), isEmpty, reason: sample);
    }
  });
}

_Source _ui(String body) => _Source('lib/src/core/ui/probe_screen.dart', body);

const _teal = '0xFF0D9488';

/// Files that may name any value, because they are the definitions.
const _owners = {
  'spacing.dart',
  'radii.dart',
  'layout.dart',
  'mark.dart',
  'fonts.dart',
  'motion.dart',
  'openoxide_colors.dart',
  'app_theme.dart',
  'control_styles.dart',
  'log_colors.dart',
  'phase_colors.dart',
};

/// Sites where a raw alpha is the right expression: a status tint derived from
/// whichever status it is, which no token can pre-bake. Extending this map is
/// how a new legitimate case gets admitted; deleting this test is not.
const _derivedAlpha = {'container_chip.dart', 'status_notices.dart'};

/// Match any call's opening, then inspect its own argument list for a number —
/// however the call is wrapped across lines.
class _CallRule {
  _CallRule(this.head, this.what, {this.uiOnly = false});

  final RegExp head;
  final String what;
  final bool uiOnly;
}

final _calls = [
  _CallRule(RegExp(r'EdgeInsets\.\w+'), 'spacing'),
  _CallRule(RegExp(r'\bRadius\.circular'), 'corner radius'),
  _CallRule(RegExp(r'BorderRadius\.circular'), 'corner radius'),
  _CallRule(RegExp(r'BoxConstraints\('), 'layout width'),
  _CallRule(RegExp(r'Duration\('), 'duration', uiOnly: true),
  _CallRule(
    RegExp(
      r'\.(animate|fadeIn|fadeTo|fadeOut|slideY|slideX|scale|scaleX|scaleY|rotate|moveBy|blur|spin|shimmer|then)\(',
    ),
    'animation',
    uiOnly: true,
  ),
];

class _Problem {
  const _Problem(this.line, this.why);

  final int line;
  final String why;
}

class _Source {
  _Source(this.path, this.text);

  /// Slash-separated path from the package root, for readable failure output.
  final String path;
  final String text;

  bool get isOwner => _owners.contains(_basename(path));
  bool get isUi => path.contains('/src/core/ui/');
  bool get allowsDerivedAlpha => _derivedAlpha.contains(_basename(path));

  late final List<int> _lineStarts = () {
    final starts = <int>[0];
    for (var i = 0; i < text.length; i++) {
      if (text.codeUnitAt(i) == 10) starts.add(i + 1);
    }
    return starts;
  }();

  /// One-based line number containing [offset].
  int lineAt(int offset) {
    var low = 0;
    var high = _lineStarts.length - 1;
    while (low < high) {
      final mid = (low + high + 1) ~/ 2;
      if (_lineStarts[mid] <= offset) {
        low = mid;
      } else {
        high = mid - 1;
      }
    }
    return low + 1;
  }
}

String _basename(String path) {
  final slash = path.lastIndexOf('/');
  return slash < 0 ? path : path.substring(slash + 1);
}

List<_Source> _sources(Directory lib) => [
  for (final file in lib.listSync(recursive: true).whereType<File>())
    if (file.path.endsWith('.dart'))
      // Generated gRPC and proto stubs are not design code and are not ours to
      // style.
      if (!_normalise(file.path).contains('/src/gen/'))
        _Source(_normalise(file.path), file.readAsStringSync()),
];

String _normalise(String path) => path.replaceAll(r'\', '/');

List<_Problem> _problemsIn(_Source source) {
  if (source.isOwner) return const [];
  final problems = <_Problem>[
    ..._scan(
      source,
      RegExp(r'Color\(0x'),
      'raw colour — use context.ox or a theme role',
    ),
    ..._scan(
      source,
      RegExp(r'\bfontSize: *-?\d'),
      'font size — use a text style or Fonts',
    ),
    ..._scan(
      source,
      RegExp(r'\bheight: *-?\d*\.\d'),
      'line height — belongs in Fonts',
    ),
    ..._scan(
      source,
      RegExp(r'withOpacity\('),
      'withOpacity is deprecated and names a raw alpha',
    ),
    if (!source.allowsDerivedAlpha) //
      ..._scan(
        source,
        RegExp(r'withValues\(alpha:'),
        'raw alpha — take a colour token',
      ),
    for (final rule in _calls)
      if (!rule.uiOnly || source.isUi) ..._scanCalls(source, rule),
  ];
  return problems..sort((a, b) => a.line.compareTo(b.line));
}

List<_Problem> _scan(_Source source, RegExp pattern, String why) => [
  for (final match in pattern.allMatches(source.text)) //
    _Problem(source.lineAt(match.start), why),
];

/// Every call matching [rule.head] whose own argument list holds a number that
/// the token system did not supply.
///
/// A token call in the argument list is sanctioned whatever number it carries:
/// `Motion.delay(5)` names a position in a stagger, not a value the scale owns.
List<_Problem> _scanCalls(_Source source, _CallRule rule) {
  final found = <_Problem>[];
  for (final match in rule.head.allMatches(source.text)) {
    final open = source.text.indexOf('(', match.start);
    if (open < 0 || open > match.end) continue;
    final args = _arguments(source.text, open);
    if (args == null) continue;
    if (!args.replaceAll(_tokenCall, '').contains(RegExp(r'\d'))) continue;
    found.add(
      _Problem(
        source.lineAt(match.start),
        '${rule.what} is owned by the token system: ${args.trim()}',
      ),
    );
  }
  return found;
}

final _tokenCall = RegExp(
  r'\b(Spacing|Radii|Mark|Fonts|Motion|Layout|OpenoxideColors|Theme)\.[A-Za-z_][\w]*(\([^()]*\))?',
);

/// Text between the paren at [open] and its matching close, exclusive. Returns
/// null if the parens never balance, which means the file does not parse and
/// some other gate will say so.
String? _arguments(String text, int open) {
  var depth = 0;
  for (var i = open; i < text.length; i++) {
    final c = text[i];
    if (c == '(') {
      depth++;
    } else if (c == ')') {
      depth--;
      if (depth == 0) return text.substring(open + 1, i);
    }
  }
  return null;
}

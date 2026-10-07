import 'package:flutter/material.dart';

import '../logs/ansi_sgr.dart';
import '../logs/log_model.dart';
import '../logs/log_tone.dart';
import 'theme.dart';
import 'tokens.dart';

/// Live log pane: filters, tone-and-ANSI colouring, and tail pinning.
///
/// The pane follows the bottom of the stream unless the reader scrolls up, in
/// which case it holds position — the same 40 px follow zone the web viewer
/// uses, because anything larger looks like it lost the reader's place.
class LogView extends StatefulWidget {
  const LogView({
    super.key,
    required this.entries,
    this.droppedLines = 0,
    this.emptyHint = 'No output yet',
  });

  final List<ParsedLogEntry> entries;
  final int droppedLines;
  final String emptyHint;

  /// How close to the bottom still counts as "reading the live edge".
  static const double followZonePx = 40;

  @override
  State<LogView> createState() => _LogViewState();
}

class _LogViewState extends State<LogView> {
  final ScrollController _scroll = ScrollController();
  LogLevel _level = LogLevel.all;
  StreamType _stream = StreamType.all;
  bool _following = true;
  List<ParsedLogEntry> _visible = const [];

  @override
  void initState() {
    super.initState();
    _visible = _filter(widget.entries);
    _scroll.addListener(_onScroll);
  }

  @override
  void didUpdateWidget(LogView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!identical(oldWidget.entries, widget.entries)) {
      _visible = _filter(widget.entries);
      if (_following) _pinToBottom();
    }
  }

  @override
  void dispose() {
    _scroll
      ..removeListener(_onScroll)
      ..dispose();
    super.dispose();
  }

  void _onScroll() {
    if (!_scroll.hasClients) return;
    final position = _scroll.position;
    final atBottom =
        position.pixels >= position.maxScrollExtent - LogView.followZonePx;
    if (atBottom != _following) setState(() => _following = atBottom);
  }

  void _pinToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted || !_scroll.hasClients) return;
      _scroll.jumpTo(_scroll.position.maxScrollExtent);
    });
  }

  List<ParsedLogEntry> _filter(List<ParsedLogEntry> entries) {
    if (_level == LogLevel.all && _stream == StreamType.all) return entries;
    return [
      for (final entry in entries)
        if ((_level == LogLevel.all || entry.level == _level) &&
            (_stream == StreamType.all || entry.stream == _stream))
          entry,
    ];
  }

  void _setFilters(LogLevel level, StreamType stream) {
    setState(() {
      _level = level;
      _stream = stream;
      _visible = _filter(widget.entries);
    });
    if (_following) _pinToBottom();
  }

  @override
  Widget build(BuildContext context) {
    final brightness = Theme.of(context).brightness;
    return Column(
      children: [
        _FilterBar(
          level: _level,
          stream: _stream,
          count: _visible.length,
          onChanged: _setFilters,
        ),
        if (widget.droppedLines > 0)
          _Banner(
            text:
                '${widget.droppedLines} older lines dropped — open the deployment on the panel for the full log',
          ),
        Expanded(
          child: _visible.isEmpty
              ? Center(
                  child: Text(
                    widget.emptyHint,
                    style: TextStyle(color: context.ox.textSubtle),
                  ),
                )
              : ListView.builder(
                  controller: _scroll,
                  padding: const EdgeInsets.fromLTRB(
                    Spacing.sm,
                    Spacing.xs,
                    Spacing.sm,
                    Spacing.lg,
                  ),
                  itemCount: _visible.length,
                  itemBuilder: (context, index) => _LogLine(
                    entry: _visible[index],
                    brightness: brightness,
                  ),
                ),
        ),
      ],
    );
  }
}

class _LogLine extends StatelessWidget {
  const _LogLine({required this.entry, required this.brightness});

  final ParsedLogEntry entry;
  final Brightness brightness;

  @override
  Widget build(BuildContext context) {
    final tone = lineTone(entry.clean);
    final fallback = logColor(tone, brightness);
    final base = TextStyle(
      fontFamily: Fonts.mono,
      fontFamilyFallback: Fonts.monoFallback,
      fontSize: Fonts.logSize,
      height: Fonts.logLineHeight,
      color: fallback,
    );

    final children = <InlineSpan>[
      for (final span in parseAnsi(entry.clean))
        TextSpan(
          text: span.text,
          style: ansiTextStyle(
            span.styles,
            ansiColor(span.color, brightness),
            fallback,
            brightness,
          ),
        ),
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: Fonts.logLinePadding),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: Fonts.logMarkerColumn,
            child: entry.isError
                ? Text('•', style: base.copyWith(color: logColor(LogTone.error, brightness)))
                : null,
          ),
          Expanded(child: SelectableText.rich(TextSpan(children: children, style: base))),
        ],
      ),
    );
  }
}

class _FilterBar extends StatelessWidget {
  const _FilterBar({
    required this.level,
    required this.stream,
    required this.count,
    required this.onChanged,
  });

  final LogLevel level;
  final StreamType stream;
  final int count;
  final void Function(LogLevel, StreamType) onChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(
        Spacing.md,
        Spacing.sm,
        Spacing.md,
        Spacing.xs,
      ),
      child: Row(
        children: [
          _Dropdown<LogLevel>(
            value: level,
            label: 'level',
            options: LogLevel.values,
            nameOf: _levelName,
            onChanged: (next) => onChanged(next, stream),
          ),
          const SizedBox(width: Spacing.sm),
          _Dropdown<StreamType>(
            value: stream,
            label: 'stream',
            options: StreamType.values,
            nameOf: _streamName,
            onChanged: (next) => onChanged(level, next),
          ),
          // `Expanded`, not a `Spacer` beside a bare text: the two dropdowns and
          // the count have to share a 320dp row, and a text next to a Spacer is
          // measured with the whole row still available — which is how this bar
          // overflowed on the smallest handset.
          Expanded(
            child: Text(
              '$count lines',
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              textAlign: TextAlign.end,
              style: Theme.of(context).textTheme.labelSmall,
            ),
          ),
        ],
      ),
    );
  }
}

String _levelName(LogLevel value) => switch (value) {
  LogLevel.all => 'all',
  LogLevel.error => 'error',
  LogLevel.warn => 'warn',
  LogLevel.success => 'success',
  LogLevel.debug => 'debug',
  LogLevel.info => 'info',
};

String _streamName(StreamType value) => switch (value) {
  StreamType.all => 'both',
  StreamType.stdout => 'stdout',
  StreamType.stderr => 'stderr',
};

class _Dropdown<T> extends StatelessWidget {
  const _Dropdown({
    required this.value,
    required this.label,
    required this.options,
    required this.nameOf,
    required this.onChanged,
  });

  final T value;
  final String label;
  final List<T> options;
  final String Function(T) nameOf;
  final ValueChanged<T> onChanged;

  @override
  Widget build(BuildContext context) {
    return DropdownButton<T>(
      value: value,
      isDense: true,
      underline: const SizedBox.shrink(),
      borderRadius: Radii.menuR,
      hint: Text(label, style: Theme.of(context).textTheme.labelSmall),
      items: [
        for (final option in options)
          DropdownMenuItem(
            value: option,
            child: Text(nameOf(option), style: Theme.of(context).textTheme.bodySmall),
          ),
      ],
      onChanged: (next) {
        if (next != null) onChanged(next);
      },
    );
  }
}

class _Banner extends StatelessWidget {
  const _Banner({required this.text});

  final String text;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(
        horizontal: Spacing.md,
        vertical: Spacing.xs,
      ),
      child: Align(
        alignment: Alignment.centerLeft,
        child: Text(text, style: Theme.of(context).textTheme.labelSmall),
      ),
    );
  }
}

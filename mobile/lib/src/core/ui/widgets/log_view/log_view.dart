import 'package:flutter/material.dart';
import 'package:mobile/src/core/logs/log_model.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/log_view/log_filter_bar.dart';
import 'package:mobile/src/core/ui/widgets/log_view/log_line.dart';

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
    final brightness = context.brightness;
    return Column(
      children: [
        LogFilterBar(
          level: _level,
          stream: _stream,
          count: _visible.length,
          onChanged: _setFilters,
        ),
        if (widget.droppedLines > 0)
          DroppedLinesBanner(droppedLines: widget.droppedLines),
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
                  itemBuilder: (context, index) =>
                      LogLine(entry: _visible[index], brightness: brightness),
                ),
        ),
      ],
    );
  }
}

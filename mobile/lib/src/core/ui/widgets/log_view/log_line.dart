import 'package:flutter/material.dart';
import 'package:mobile/src/core/logs/ansi_sgr.dart';
import 'package:mobile/src/core/logs/log_model.dart';
import 'package:mobile/src/core/logs/log_tone.dart';
import 'package:mobile/src/core/theme/log_colors.dart';
import 'package:mobile/src/core/tokens/fonts.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// One log line: error marker, ANSI-styled monospaced text.
class LogLine extends StatelessWidget {
  const LogLine({super.key, required this.entry, required this.brightness});

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
                ? Text(
                    '•',
                    style: base.copyWith(
                      color: logColor(LogTone.error, brightness),
                    ),
                  )
                : null,
          ),
          Expanded(
            child: SelectableText.rich(
              TextSpan(children: children, style: base),
            ),
          ),
        ],
      ),
    );
  }
}

/// The note above the pane when older lines fell off the capped buffer.
class DroppedLinesBanner extends StatelessWidget {
  const DroppedLinesBanner({super.key, required this.droppedLines});

  final int droppedLines;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(
        horizontal: Spacing.md,
        vertical: Spacing.xs,
      ),
      child: Align(
        alignment: Alignment.centerLeft,
        child: Text(
          '$droppedLines older lines dropped — open the deployment on the panel '
          'for the full log',
          style: Theme.of(context).textTheme.labelSmall,
        ),
      ),
    );
  }
}

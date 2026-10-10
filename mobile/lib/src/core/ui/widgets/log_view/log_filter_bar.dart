import 'package:flutter/material.dart';
import 'package:mobile/src/core/logs/log_model.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// Level and stream dropdowns plus the visible-line count.
class LogFilterBar extends StatelessWidget {
  const LogFilterBar({
    super.key,
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
          FilterDropdown<LogLevel>(
            value: level,
            label: 'level',
            options: LogLevel.values,
            nameOf: levelName,
            onChanged: (next) => onChanged(next, stream),
          ),
          const SizedBox(width: Spacing.sm),
          FilterDropdown<StreamType>(
            value: stream,
            label: 'stream',
            options: StreamType.values,
            nameOf: streamName,
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

String levelName(LogLevel value) => switch (value) {
  LogLevel.all => 'all',
  LogLevel.error => 'error',
  LogLevel.warn => 'warn',
  LogLevel.success => 'success',
  LogLevel.debug => 'debug',
  LogLevel.info => 'info',
};

String streamName(StreamType value) => switch (value) {
  StreamType.all => 'both',
  StreamType.stdout => 'stdout',
  StreamType.stderr => 'stderr',
};

/// One dense dropdown without an underline, for the filter row.
class FilterDropdown<T> extends StatelessWidget {
  const FilterDropdown({
    super.key,
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
            child: Text(
              nameOf(option),
              style: Theme.of(context).textTheme.bodySmall,
            ),
          ),
      ],
      onChanged: (next) {
        if (next != null) onChanged(next);
      },
    );
  }
}

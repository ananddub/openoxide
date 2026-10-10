/// Font families, as declared in `pubspec.yaml`, and the log pane's type metrics.
abstract final class Fonts {
  static const String ui = 'Inter';
  static const String mono = 'JetBrains Mono';

  /// Kept for the case where the bundled mono face is somehow unavailable —
  /// log output must still be monospaced to stay aligned.
  static const List<String> monoFallback = ['monospace', 'RobotoMono'];

  /// Log panes run smaller and tighter than any UI text style: a build that
  /// emits 200 lines a screen has to be readable at that density, so the size
  /// belongs to the pane rather than to a number in its widget.
  static const double logSize = 11.5;
  static const double logLineHeight = 1.45;

  /// Vertical padding per log line. Half a pixel, because the pane's own
  /// [logLineHeight] already carries most of the leading — this is only enough
  /// to stop two adjacent lines from touching.
  static const double logLinePadding = 0.5;

  /// The column reserved for the error marker, held whether or not a line is an
  /// error so every line's text starts at the same x.
  static const double logMarkerColumn = 6;
}

/// Widths the layout chooses for itself, as opposed to the room inside them.
abstract final class Layout {
  /// The widest a single column of prose or a form should run. Past this a line
  /// is harder to track back to its start, and spare width is worth more beside
  /// the column than inside it.
  static const double measure = 420;

  /// Where a phone becomes a tablet — the app's only breakpoint, and the one the
  /// responsive harness brackets from both sides.
  static const double wide = 720;
}

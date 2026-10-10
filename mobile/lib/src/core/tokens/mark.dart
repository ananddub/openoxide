/// Marks a row or a control carries, as opposed to the room around it.
///
/// These sit outside the 4px grid because they are not spacing: a status dot and
/// a button's leading slot are drawn objects whose size is read against the text
/// beside them.
abstract final class Mark {
  /// The one status dot — a list tile's leading mark, a tile's trailing icon and
  /// a legend entry all take it, so a deploy looks the same wherever it appears.
  static const double dot = 9;

  /// A button's leading slot. The spinner that replaces the icon while the
  /// button is busy has to measure exactly this, or the button jumps as it
  /// enters its loading state.
  static const double glyph = 18;

  /// Stroke weight inside [glyph] — thin enough to read as motion, thick enough
  /// to survive a low-density screen.
  static const double stroke = 2;

  /// Thinnest progress bar that still shows its own fill.
  static const double bar = 5;
}

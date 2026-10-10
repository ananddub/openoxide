/// Layout scale, on a 4px grid.
///
/// These are `const` rather than theme values on purpose: they do not change
/// with brightness, and a `ThemeExtension` lookup cannot appear inside a
/// `const` constructor, which would cost every `const EdgeInsets` in the app.
abstract final class Spacing {
  /// Half the [xs] step — the hair of separation a stacked subtitle needs so its
  /// first line clears the descender above it. Below this a gap is invisible.
  static const double xxs = 2;
  static const double xs = 4;
  static const double sm = 8;
  static const double md = 12;
  static const double lg = 16;
  static const double xl = 24;
  static const double xxl = 32;

  /// Horizontal inset for every list and screen body.
  static const double gutter = 12;

  /// Bottom inset that keeps the last row clear of the floating button.
  static const double fabInset = 90;
}

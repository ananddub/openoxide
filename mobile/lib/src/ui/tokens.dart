import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';

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

/// Corner radii, matching the web panel's button/card/popover scale.
abstract final class Radii {
  static const double control = 8;
  static const double card = 12;
  static const double menu = 12;
  static const double dot = 4;

  static const BorderRadius controlR = BorderRadius.all(
    Radius.circular(control),
  );
  static const BorderRadius cardR = BorderRadius.all(Radius.circular(card));
  static const BorderRadius menuR = BorderRadius.all(Radius.circular(menu));

  /// The same corners as [Border] shapes — Material's `shape` slots take an
  /// [OutlinedBorder], not a [BorderRadius], so callers need both forms.
  static const OutlinedBorder controlShape = RoundedRectangleBorder(
    borderRadius: controlR,
  );
  static const OutlinedBorder cardShape = RoundedRectangleBorder(
    borderRadius: cardR,
  );
  static const OutlinedBorder menuShape = RoundedRectangleBorder(
    borderRadius: menuR,
  );
}

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

/// One rhythm for the whole app: entrances and state changes, no loops.
abstract final class Motion {
  static const Duration micro = Duration(milliseconds: 120);
  static const Duration fast = Duration(milliseconds: 200);
  static const Duration base = Duration(milliseconds: 300);
  static const Duration slow = Duration(milliseconds: 450);

  /// Gap between successive items of a staggered entrance.
  static const Duration stagger = Duration(milliseconds: 60);

  /// One breathing cycle of the in-flight status dot. Longer than any entrance
  /// on purpose: an urgent loop reads as a spinner, a slow one reads as alive.
  static const Duration pulse = Duration(milliseconds: 1200);

  /// How long a transient notice stays up. Not an animation — a reading time.
  static const Duration notice = Duration(seconds: 3);

  /// Entrance travel, as a fraction of the widget's own height: a list row
  /// barely moves, a screen's contents arrive from further down, and the brand
  /// comes from above so the page reads as opening downward.
  static const double liftRow = 0.06;
  static const double liftBlock = 0.12;

  /// Dimmest point of the [pulse] cycle — low enough to read as breathing, high
  /// enough that an in-flight row stays legible.
  static const double pulseOpacity = 0.35;

  static const Curve enter = Curves.easeOutCubic;
  static const Curve move = Curves.easeInOut;

  /// Longest any entrance chain can occupy. The test harness pumps this much
  /// instead of `pumpAndSettle`, which never returns while a repeating
  /// animation is on screen.
  static const Duration entranceBudget = Duration(milliseconds: 1600);

  /// Retargets flutter_animate's statics so every `.animate()` inherits this
  /// scale rather than its 1s/linear default. Called from `main()` and from
  /// test setup, since they are mutable globals.
  static void install() {
    Animate.defaultDuration = base;
    Animate.defaultCurve = enter;
  }

  /// `Motion.stagger * 2` — the delay of the third item in a list.
  static Duration delay(int step) => stagger * step;
}

/// The panel's colour tokens, ported from the web client's Geist system.
///
/// Colour is the one scale that genuinely differs between light and dark, so
/// unlike [Spacing] and [Radii] it resolves through the theme instead of a
/// constant.
@immutable
class OpenoxideColors extends ThemeExtension<OpenoxideColors> {
  const OpenoxideColors({
    required this.page,
    required this.card,
    required this.popover,
    required this.text,
    required this.textMuted,
    required this.textSubtle,
    required this.border,
    required this.inputBorder,
    required this.focusRing,
    required this.subtleFill,
    required this.accent,
    required this.onAccent,
    required this.success,
    required this.danger,
    required this.warning,
    required this.info,
    required this.neutral,
  });

  final Color page;
  final Color card;
  final Color popover;
  final Color text;
  final Color textMuted;
  final Color textSubtle;
  final Color border;
  final Color inputBorder;
  final Color focusRing;
  final Color subtleFill;

  /// The only teal in the app: primary actions, the selected nav indicator and
  /// the focused input border. Nothing else may use it.
  final Color accent;
  final Color onAccent;

  final Color success;
  final Color danger;
  final Color warning;
  final Color info;
  final Color neutral;

  /// Hairlines carry their alpha in the token itself, so a `BorderSide` of
  /// `border` composites against whatever surface it is drawn on exactly like
  /// the web's `rgba()`. That only holds while no elevation tint is blended
  /// underneath, which is why the theme sets elevation to zero everywhere.
  static const OpenoxideColors light = OpenoxideColors(
    page: Color(0xFFFFFFFF),
    card: Color(0xFFFFFFFF),
    popover: Color(0xFFFFFFFF),
    text: Color(0xFF171717),
    textMuted: Color(0xFF666666),
    textSubtle: Color(0xFF8F8F8F),
    border: Color(0x14000000),
    inputBorder: Color(0x14000000),
    focusRing: Color(0x36000000),
    subtleFill: Color(0x0D000000),
    accent: Color(0xFF0D9488),
    onAccent: Color(0xFFFFFFFF),
    success: Color(0xFF45A557),
    danger: Color(0xFFDA2F35),
    warning: Color(0xFFFFB224),
    info: Color(0xFF0072F5),
    neutral: Color(0xFF8F8F8F),
  );

  static const OpenoxideColors dark = OpenoxideColors(
    page: Color(0xFF000000),
    card: Color(0xFF0A0A0A),
    popover: Color(0xFF0A0A0A),
    text: Color(0xFFEDEDED),
    textMuted: Color(0xFFA1A1A1),
    textSubtle: Color(0xFF8F8F8F),
    border: Color(0x17FFFFFF),
    inputBorder: Color(0x24FFFFFF),
    focusRing: Color(0x3DFFFFFF),
    subtleFill: Color(0x0FFFFFFF),
    accent: Color(0xFF0D9488),
    onAccent: Color(0xFFFFFFFF),
    success: Color(0xFF45A557),
    danger: Color(0xFFE5484D),
    warning: Color(0xFFFFB224),
    info: Color(0xFF0072F5),
    neutral: Color(0xFF8F8F8F),
  );

  static OpenoxideColors of(Brightness brightness) =>
      brightness == Brightness.dark ? dark : light;

  @override
  OpenoxideColors copyWith({
    Color? page,
    Color? card,
    Color? popover,
    Color? text,
    Color? textMuted,
    Color? textSubtle,
    Color? border,
    Color? inputBorder,
    Color? focusRing,
    Color? subtleFill,
    Color? accent,
    Color? onAccent,
    Color? success,
    Color? danger,
    Color? warning,
    Color? info,
    Color? neutral,
  }) {
    return OpenoxideColors(
      page: page ?? this.page,
      card: card ?? this.card,
      popover: popover ?? this.popover,
      text: text ?? this.text,
      textMuted: textMuted ?? this.textMuted,
      textSubtle: textSubtle ?? this.textSubtle,
      border: border ?? this.border,
      inputBorder: inputBorder ?? this.inputBorder,
      focusRing: focusRing ?? this.focusRing,
      subtleFill: subtleFill ?? this.subtleFill,
      accent: accent ?? this.accent,
      onAccent: onAccent ?? this.onAccent,
      success: success ?? this.success,
      danger: danger ?? this.danger,
      warning: warning ?? this.warning,
      info: info ?? this.info,
      neutral: neutral ?? this.neutral,
    );
  }

  @override
  OpenoxideColors lerp(ThemeExtension<OpenoxideColors>? other, double t) {
    if (other is! OpenoxideColors) return this;
    return OpenoxideColors(
      page: Color.lerp(page, other.page, t)!,
      card: Color.lerp(card, other.card, t)!,
      popover: Color.lerp(popover, other.popover, t)!,
      text: Color.lerp(text, other.text, t)!,
      textMuted: Color.lerp(textMuted, other.textMuted, t)!,
      textSubtle: Color.lerp(textSubtle, other.textSubtle, t)!,
      border: Color.lerp(border, other.border, t)!,
      inputBorder: Color.lerp(inputBorder, other.inputBorder, t)!,
      focusRing: Color.lerp(focusRing, other.focusRing, t)!,
      subtleFill: Color.lerp(subtleFill, other.subtleFill, t)!,
      accent: Color.lerp(accent, other.accent, t)!,
      onAccent: Color.lerp(onAccent, other.onAccent, t)!,
      success: Color.lerp(success, other.success, t)!,
      danger: Color.lerp(danger, other.danger, t)!,
      warning: Color.lerp(warning, other.warning, t)!,
      info: Color.lerp(info, other.info, t)!,
      neutral: Color.lerp(neutral, other.neutral, t)!,
    );
  }
}

extension ThemeAccess on BuildContext {
  OpenoxideColors get ox => Theme.of(this).extension<OpenoxideColors>()!;
  Brightness get brightness => Theme.of(this).brightness;
}

import 'package:flutter/material.dart';

/// The panel's colour tokens, ported from the web client's Geist system.
///
/// Colour is the one scale that genuinely differs between light and dark, so
/// unlike the constant spacing tokens it resolves through the theme.
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

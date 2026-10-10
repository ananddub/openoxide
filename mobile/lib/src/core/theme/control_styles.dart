import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// Outlined fields with a teal focus ring — the second of the accent's three
/// sanctioned uses, the first being the primary button.
InputDecorationTheme oxInputDecoration(OpenoxideColors ox, TextTheme text) {
  OutlineInputBorder edge(Color color, [double width = 1]) =>
      OutlineInputBorder(
        borderRadius: Radii.controlR,
        borderSide: BorderSide(color: color, width: width),
      );
  return InputDecorationTheme(
    filled: false,
    isDense: false,
    contentPadding: const EdgeInsets.symmetric(
      horizontal: Spacing.md,
      vertical: Spacing.md,
    ),
    labelStyle: text.bodyMedium?.copyWith(color: ox.textMuted),
    // Muted, not accent: the 1.5px teal ring below is the focus signal, and a
    // floated label that also turned teal would spend the accent on every field
    // the reader has merely moved past.
    floatingLabelStyle: text.bodySmall?.copyWith(
      color: ox.textMuted,
      fontWeight: FontWeight.w600,
    ),
    hintStyle: text.bodyMedium?.copyWith(color: ox.textSubtle),
    helperStyle: text.bodySmall?.copyWith(color: ox.textMuted),
    errorStyle: text.bodySmall?.copyWith(color: ox.danger),
    counterStyle: text.bodySmall?.copyWith(color: ox.textMuted),
    prefixIconColor: ox.textMuted,
    suffixIconColor: ox.textMuted,
    iconColor: ox.textMuted,
    border: edge(ox.inputBorder),
    enabledBorder: edge(ox.inputBorder),
    focusedBorder: edge(ox.accent, 1.5),
    errorBorder: edge(ox.danger),
    focusedErrorBorder: edge(ox.danger, 1.5),
    disabledBorder: edge(ox.border),
  );
}

/// Heights are explicit because Material's default button is ~40dp tall, which
/// is under Android's 48dp minimum touch target. `medium` is the default.
const double kButtonHeightSmall = 40;
const double kButtonHeightMedium = 48;
const double kButtonHeightLarge = 52;

ButtonStyle _buttonGeometry(TextTheme text) => ButtonStyle(
  textStyle: WidgetStatePropertyAll<TextStyle?>(
    text.titleMedium?.copyWith(fontSize: 15, fontWeight: FontWeight.w600),
  ),
  shape: const WidgetStatePropertyAll<OutlinedBorder>(Radii.controlShape),
  padding: const WidgetStatePropertyAll<EdgeInsetsGeometry>(
    EdgeInsets.symmetric(horizontal: Spacing.xl),
  ),
  elevation: const WidgetStatePropertyAll<double>(0),
  shadowColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
  overlayColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
);

ButtonStyle oxFilledStyle(
  OpenoxideColors ox,
  TextTheme text, {
  required Color background,
  required Color foreground,
}) {
  return _buttonGeometry(text).copyWith(
    backgroundColor: WidgetStateProperty.resolveWith(
      (states) => states.contains(WidgetState.disabled)
          ? background.withValues(alpha: 0.42)
          : background,
    ),
    foregroundColor: WidgetStateProperty.resolveWith(
      (states) => states.contains(WidgetState.disabled)
          ? foreground.withValues(alpha: 0.6)
          : foreground,
    ),
    side: const WidgetStatePropertyAll<BorderSide?>(null),
  );
}

ButtonStyle oxOutlinedStyle(OpenoxideColors ox, TextTheme text) {
  return _buttonGeometry(text).copyWith(
    backgroundColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
    foregroundColor: WidgetStatePropertyAll<Color>(ox.text),
    side: WidgetStatePropertyAll<BorderSide>(
      BorderSide(color: ox.inputBorder, width: 1),
    ),
  );
}

ButtonStyle oxTextStyle(OpenoxideColors ox, TextTheme text) {
  return _buttonGeometry(text).copyWith(
    backgroundColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
    foregroundColor: WidgetStatePropertyAll<Color>(ox.text),
    padding: const WidgetStatePropertyAll<EdgeInsetsGeometry>(
      EdgeInsets.symmetric(horizontal: Spacing.md),
    ),
  );
}

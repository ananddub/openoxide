import 'package:flutter/material.dart';

import '../logs/ansi_sgr.dart';
import '../logs/log_tone.dart';
import '../state/deployment_status.dart';
import 'tokens.dart';

/// The app's look, in one place, so the log ramp below and the widget themes
/// cannot drift apart between light and dark.
///
/// The [ColorScheme] is written out rather than derived with `fromSeed`: a seed
/// tints every neutral role (`surface`, `surfaceContainer*`,
/// `onSurfaceVariant`) toward itself, which would put teal under all the text
/// and borders. The web panel's palette is flat Geist neutral with a single teal
/// accent, so the roles are set one by one to keep it that way.
ThemeData appTheme(Brightness brightness) {
  final ox = OpenoxideColors.of(brightness);
  final isDark = brightness == Brightness.dark;

  final scheme = ColorScheme(
    brightness: brightness,
    primary: ox.accent,
    onPrimary: ox.onAccent,
    primaryContainer: ox.accent,
    onPrimaryContainer: ox.onAccent,
    secondary: ox.text,
    onSecondary: ox.page,
    secondaryContainer: ox.subtleFill,
    onSecondaryContainer: ox.text,
    tertiary: ox.text,
    onTertiary: ox.page,
    tertiaryContainer: ox.subtleFill,
    onTertiaryContainer: ox.text,
    error: ox.danger,
    onError: ox.onAccent,
    errorContainer: ox.danger,
    onErrorContainer: ox.onAccent,
    surface: ox.page,
    onSurface: ox.text,
    surfaceContainerLowest: ox.page,
    surfaceContainerLow: ox.card,
    surfaceContainer: ox.card,
    surfaceContainerHigh: ox.card,
    surfaceContainerHighest: ox.card,
    onSurfaceVariant: ox.textMuted,
    outline: ox.border,
    outlineVariant: ox.border,
    inverseSurface: ox.text,
    onInverseSurface: ox.page,
    scrim: Colors.black,
    shadow: Colors.transparent,
  );

  final typography = Typography.material2021(
    platform: TargetPlatform.android,
    colorScheme: scheme,
  );
  final text = (isDark ? typography.white : typography.black)
      .apply(fontFamily: Fonts.ui, bodyColor: ox.text, displayColor: ox.text)
      .copyWith(
        headlineMedium: const TextStyle(
          fontSize: 24,
          height: 1.15,
          fontWeight: FontWeight.w700,
        ),
        titleMedium: const TextStyle(
          fontSize: 16,
          height: 1.25,
          fontWeight: FontWeight.w600,
        ),
        bodyMedium: const TextStyle(fontSize: 14, height: 1.4),
        bodySmall: TextStyle(fontSize: 12, height: 1.35, color: ox.textMuted),
        labelMedium: const TextStyle(fontSize: 12, height: 1.2),
        labelSmall: TextStyle(
          fontSize: 11,
          height: 1.2,
          letterSpacing: 0.3,
          color: ox.textMuted,
        ),
      );

  const transparent = WidgetStatePropertyAll<Color>(Colors.transparent);

  return ThemeData(
    useMaterial3: true,
    brightness: brightness,
    fontFamily: Fonts.ui,
    colorScheme: scheme,
    extensions: <ThemeExtension<dynamic>>[ox],
    scaffoldBackgroundColor: ox.page,
    // Elevation must stay at zero for the whole app: the hairline borders are
    // semi-transparent, and an M3 surface tint blended under them would shift
    // the neutral away from the web palette they are meant to match.
    splashFactory: InkRipple.splashFactory,
    textTheme: text,
    primaryTextTheme: text,
    iconTheme: IconThemeData(size: 24, color: ox.text),
    appBarTheme: AppBarTheme(
      backgroundColor: ox.page,
      foregroundColor: ox.text,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      scrolledUnderElevation: 0,
      centerTitle: false,
      titleTextStyle: text.titleMedium,
      iconTheme: IconThemeData(size: 24, color: ox.text),
    ),
    navigationBarTheme: NavigationBarThemeData(
      backgroundColor: ox.page,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      height: 68,
      indicatorColor: ox.subtleFill,
      indicatorShape: Radii.controlShape,
      labelTextStyle: WidgetStateProperty.resolveWith(
        (states) =>
            text.labelMedium!.copyWith(
              color: states.contains(WidgetState.selected)
                  ? ox.accent
                  : ox.textMuted,
              fontWeight: states.contains(WidgetState.selected)
                  ? FontWeight.w600
                  : FontWeight.w400,
            ),
      ),
      iconTheme: WidgetStateProperty.resolveWith(
        (states) => IconThemeData(
          size: 24,
          color: states.contains(WidgetState.selected) ? ox.accent : ox.textMuted,
        ),
      ),
    ),
    cardTheme: CardThemeData(
      color: ox.card,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      shadowColor: Colors.transparent,
      margin: const EdgeInsets.only(bottom: Spacing.sm),
      shape: RoundedRectangleBorder(
        borderRadius: Radii.cardR,
        side: BorderSide(color: ox.border, width: 1),
      ),
    ),
    dividerTheme: DividerThemeData(color: ox.border, thickness: 1, space: 1),
    inputDecorationTheme: _inputDecoration(ox, text),
    filledButtonTheme: FilledButtonThemeData(
      style: _filledStyle(ox, text, background: ox.accent, foreground: ox.onAccent),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: _filledStyle(ox, text, background: ox.accent, foreground: ox.onAccent),
    ),
    outlinedButtonTheme: OutlinedButtonThemeData(style: _outlinedStyle(ox, text)),
    textButtonTheme: TextButtonThemeData(style: _textStyle(ox, text)),
    iconButtonTheme: IconButtonThemeData(
      style: ButtonStyle(
        minimumSize: const WidgetStatePropertyAll<Size>(Size(48, 48)),
        shape: const WidgetStatePropertyAll<OutlinedBorder>(Radii.controlShape),
        iconColor: WidgetStatePropertyAll<Color>(ox.text),
        backgroundColor: transparent,
      ),
    ),
    floatingActionButtonTheme: FloatingActionButtonThemeData(
      backgroundColor: ox.subtleFill,
      foregroundColor: ox.text,
      elevation: 0,
      focusElevation: 0,
      hoverElevation: 0,
      extendedTextStyle: text.titleMedium,
      shape: Radii.controlShape,
    ),
    popupMenuTheme: PopupMenuThemeData(
      color: ox.popover,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      shape: Radii.menuShape,
      textStyle: text.bodyMedium,
    ),
    dialogTheme: DialogThemeData(
      backgroundColor: ox.popover,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: Radii.cardR,
        side: BorderSide(color: ox.border, width: 1),
      ),
    ),
    chipTheme: ChipThemeData(
      backgroundColor: ox.card,
      selectedColor: ox.accent,
      disabledColor: ox.subtleFill,
      side: BorderSide(color: ox.border, width: 1),
      shape: Radii.controlShape,
      padding: const EdgeInsets.symmetric(
        horizontal: Spacing.md,
        vertical: Spacing.sm,
      ),
      labelStyle: text.labelMedium?.copyWith(color: ox.text),
    ),
    dropdownMenuTheme: DropdownMenuThemeData(
      menuStyle: MenuStyle(
        backgroundColor: WidgetStatePropertyAll<Color>(ox.popover),
        surfaceTintColor: transparent,
        elevation: const WidgetStatePropertyAll<double>(0),
        shape: const WidgetStatePropertyAll<OutlinedBorder>(Radii.menuShape),
      ),
      textStyle: text.bodyMedium,
    ),
    progressIndicatorTheme: ProgressIndicatorThemeData(
      color: ox.accent,
      linearTrackColor: ox.subtleFill,
      circularTrackColor: Colors.transparent,
    ),
    snackBarTheme: SnackBarThemeData(
      backgroundColor: isDark ? ox.card : ox.text,
      contentTextStyle: text.bodyMedium?.copyWith(
        color: isDark ? ox.text : ox.page,
      ),
      actionTextColor: isDark ? ox.accent : ox.page,
      behavior: SnackBarBehavior.floating,
      insetPadding: const EdgeInsets.all(Spacing.md),
      shape: Radii.controlShape,
      elevation: 0,
    ),
    // Tiles live inside cards, which do not clip their own children, so the
    // splash takes the card's radius from here rather than each screen re-typing
    // an inset of its own.
    listTileTheme: ListTileThemeData(
      contentPadding: const EdgeInsets.symmetric(horizontal: Spacing.md),
      iconColor: ox.textMuted,
      textColor: ox.text,
      shape: RoundedRectangleBorder(borderRadius: Radii.cardR),
      titleTextStyle: text.bodyMedium,
      subtitleTextStyle: text.bodySmall,
    ),
  );
}

/// Outlined fields with a teal focus ring — the second of the accent's three
/// sanctioned uses, the first being the primary button.
InputDecorationTheme _inputDecoration(OpenoxideColors ox, TextTheme text) {
  OutlineInputBorder edge(Color color, [double width = 1]) => OutlineInputBorder(
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

ButtonStyle _filledStyle(
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

ButtonStyle _outlinedStyle(OpenoxideColors ox, TextTheme text) {
  return _buttonGeometry(text).copyWith(
    backgroundColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
    foregroundColor: WidgetStatePropertyAll<Color>(ox.text),
    side: WidgetStatePropertyAll<BorderSide>(
      BorderSide(color: ox.inputBorder, width: 1),
    ),
  );
}

ButtonStyle _textStyle(OpenoxideColors ox, TextTheme text) {
  return _buttonGeometry(text).copyWith(
    backgroundColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
    foregroundColor: WidgetStatePropertyAll<Color>(ox.text),
    padding: const WidgetStatePropertyAll<EdgeInsetsGeometry>(
      EdgeInsets.symmetric(horizontal: Spacing.md),
    ),
  );
}

/// Log text colour for a tone at the active brightness.
Color logColor(LogTone tone, Brightness brightness) => Color(
  brightness == Brightness.dark ? toneColors(tone).dark : toneColors(tone).light,
);

/// The 16 ANSI palette, dimmed half a step in dark mode the way a terminal with
/// a dark background would.
Color? ansiColor(AnsiColor? color, Brightness brightness) {
  if (color == null) return null;
  const base = <int>[
    0x000000, 0x7F1D1D, 0x14532D, 0x78350F, 0x1E3A8A, 0x6B21A8, 0x155E75, 0xD4D4D8,
    0x52525B, 0xEF4444, 0x22C55E, 0xEAB308, 0x3B82F6, 0xA855F7, 0x06B6D4, 0xFFFFFF,
  ];
  final index = color.index;
  // On a dark background the first eight are unreadably dark, so lift them to
  // their bright counterparts rather than inventing a new ramp.
  final effective = brightness == Brightness.dark && index < 8 && index != 0
      ? index + 8
      : index;
  return Color(0xFF000000 | base[effective]);
}

TextStyle ansiTextStyle(
  Set<AnsiStyle> styles,
  Color? color,
  Color fallback,
  Brightness brightness,
) {
  var result = TextStyle(color: color ?? fallback);
  if (styles.contains(AnsiStyle.bold)) {
    result = result.copyWith(fontWeight: FontWeight.w700);
  }
  if (styles.contains(AnsiStyle.dim)) {
    result = result.copyWith(
      color: (color ?? fallback).withValues(alpha: brightness == Brightness.dark ? 0.55 : 0.65),
    );
  }
  if (styles.contains(AnsiStyle.underline)) {
    result = result.copyWith(decoration: TextDecoration.underline);
  }
  return result;
}

/// Status pill colours for a deployment, following the web's status tone rules.
({Color dot, Color text, bool pulse}) phaseColors(
  DeploymentPhase phase,
  Brightness brightness,
) {
  final isDark = brightness == Brightness.dark;
  Color pick({required int light, required int dark}) =>
      Color(isDark ? dark : light);
  return switch (phase) {
    DeploymentPhase.working => (
      dot: pick(light: 0xFFF59F0A, dark: 0xFFFBBF24),
      text: pick(light: 0xFFB45309, dark: 0xFFFCD34D),
      pulse: true,
    ),
    DeploymentPhase.queued => (
      dot: pick(light: 0xFF64748B, dark: 0xFF94A3B8),
      text: pick(light: 0xFF475569, dark: 0xFFCBD5E1),
      pulse: true,
    ),
    DeploymentPhase.succeeded => (
      dot: pick(light: 0xFF10B981, dark: 0xFF34D399),
      text: pick(light: 0xFF047857, dark: 0xFF6EE7B7),
      pulse: false,
    ),
    DeploymentPhase.failed => (
      dot: pick(light: 0xFFDC2626, dark: 0xFFF87171),
      text: pick(light: 0xFFB91C1C, dark: 0xFFFCA5A5),
      pulse: false,
    ),
    DeploymentPhase.cancelled => (
      dot: pick(light: 0xFF71717A, dark: 0xFFA1A1AA),
      text: pick(light: 0xFF52525B, dark: 0xFFD4D4D8),
      pulse: false,
    ),
    DeploymentPhase.unknown => (
      dot: pick(light: 0xFFA1A1AA, dark: 0xFF525252),
      text: pick(light: 0xFF71717A, dark: 0xFFA1A1AA),
      pulse: false,
    ),
  };
}

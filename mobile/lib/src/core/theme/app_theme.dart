import 'package:flutter/material.dart';

import 'package:mobile/src/core/theme/control_styles.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/theme/widget_themes.dart';
import 'package:mobile/src/core/tokens/fonts.dart';
import 'package:mobile/src/core/tokens/radii.dart';

export 'package:mobile/src/core/theme/control_styles.dart'
    show kButtonHeightSmall, kButtonHeightMedium, kButtonHeightLarge;
export 'package:mobile/src/core/theme/log_colors.dart'
    show ansiColor, ansiTextStyle, logColor;
export 'package:mobile/src/core/theme/openoxide_colors.dart'
    show OpenoxideColors, ThemeAccess;
export 'package:mobile/src/core/theme/phase_colors.dart' show phaseColors;
export 'package:mobile/src/core/theme/widget_themes.dart';

/// The app's look, in one place, so the widget themes cannot drift apart
/// between light and dark.
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
    appBarTheme: oxAppBarTheme(ox, text),
    navigationBarTheme: oxNavigationBarTheme(ox, text),
    cardTheme: oxCardTheme(ox),
    dividerTheme: DividerThemeData(color: ox.border, thickness: 1, space: 1),
    inputDecorationTheme: oxInputDecoration(ox, text),
    filledButtonTheme: FilledButtonThemeData(
      style: oxFilledStyle(
        ox,
        text,
        background: ox.accent,
        foreground: ox.onAccent,
      ),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: oxFilledStyle(
        ox,
        text,
        background: ox.accent,
        foreground: ox.onAccent,
      ),
    ),
    outlinedButtonTheme: OutlinedButtonThemeData(
      style: oxOutlinedStyle(ox, text),
    ),
    textButtonTheme: TextButtonThemeData(style: oxTextStyle(ox, text)),
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
    popupMenuTheme: oxPopupMenuTheme(ox, text),
    dialogTheme: oxDialogTheme(ox),
    chipTheme: oxChipTheme(ox, text),
    dropdownMenuTheme: DropdownMenuThemeData(
      menuStyle: MenuStyle(
        backgroundColor: WidgetStatePropertyAll<Color>(ox.popover),
        surfaceTintColor: transparent,
        elevation: const WidgetStatePropertyAll<double>(0),
        shape: const WidgetStatePropertyAll<OutlinedBorder>(Radii.menuShape),
      ),
      textStyle: text.bodyMedium,
    ),
    progressIndicatorTheme: oxProgressTheme(ox),
    snackBarTheme: oxSnackBarTheme(ox, text, isDark),
    listTileTheme: oxListTileTheme(ox, text),
  );
}

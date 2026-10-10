import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// The per-widget themes a screen touches, built once by `appTheme` so the
/// light and dark palettes cannot drift apart between them.

/// Zero-elevation bar on the page background, titles in the app's own scale.
AppBarTheme oxAppBarTheme(OpenoxideColors ox, TextTheme text) => AppBarTheme(
  backgroundColor: ox.page,
  foregroundColor: ox.text,
  surfaceTintColor: Colors.transparent,
  elevation: 0,
  scrolledUnderElevation: 0,
  centerTitle: false,
  titleTextStyle: text.titleMedium,
  iconTheme: IconThemeData(size: 24, color: ox.text),
);

NavigationBarThemeData oxNavigationBarTheme(
  OpenoxideColors ox,
  TextTheme text,
) => NavigationBarThemeData(
  backgroundColor: ox.page,
  surfaceTintColor: Colors.transparent,
  elevation: 0,
  height: 68,
  indicatorColor: ox.subtleFill,
  indicatorShape: Radii.controlShape,
  labelTextStyle: WidgetStateProperty.resolveWith(
    (states) => text.labelMedium!.copyWith(
      color: states.contains(WidgetState.selected) ? ox.accent : ox.textMuted,
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
);

/// Flat cards: the hairline border is the whole elevation, exactly like the
/// web panel's `border` token.
CardThemeData oxCardTheme(OpenoxideColors ox) => CardThemeData(
  color: ox.card,
  surfaceTintColor: Colors.transparent,
  elevation: 0,
  shadowColor: Colors.transparent,
  margin: const EdgeInsets.only(bottom: Spacing.sm),
  shape: RoundedRectangleBorder(
    borderRadius: Radii.cardR,
    side: BorderSide(color: ox.border, width: 1),
  ),
);

DialogThemeData oxDialogTheme(OpenoxideColors ox) => DialogThemeData(
  backgroundColor: ox.popover,
  surfaceTintColor: Colors.transparent,
  elevation: 0,
  shape: RoundedRectangleBorder(
    borderRadius: Radii.cardR,
    side: BorderSide(color: ox.border, width: 1),
  ),
);

ChipThemeData oxChipTheme(OpenoxideColors ox, TextTheme text) => ChipThemeData(
  backgroundColor: ox.card,
  selectedColor: ox.accent,
  disabledColor: ox.subtleFill,
  side: BorderSide(color: ox.border, width: 1),
  shape: Radii.controlShape,
  padding: const EdgeInsets.symmetric(horizontal: Spacing.md, vertical: Spacing.sm),
  labelStyle: text.labelMedium?.copyWith(color: ox.text),
);

PopupMenuThemeData oxPopupMenuTheme(OpenoxideColors ox, TextTheme text) =>
    PopupMenuThemeData(
      color: ox.popover,
      surfaceTintColor: Colors.transparent,
      elevation: 0,
      shape: Radii.menuShape,
      textStyle: text.bodyMedium,
    );

SnackBarThemeData oxSnackBarTheme(
  OpenoxideColors ox,
  TextTheme text,
  bool isDark,
) => SnackBarThemeData(
  backgroundColor: isDark ? ox.card : ox.text,
  contentTextStyle: text.bodyMedium?.copyWith(
    color: isDark ? ox.text : ox.page,
  ),
  actionTextColor: isDark ? ox.accent : ox.page,
  behavior: SnackBarBehavior.floating,
  insetPadding: const EdgeInsets.all(Spacing.md),
  shape: Radii.controlShape,
  elevation: 0,
);

/// Tiles live inside cards, which do not clip their own children, so the
/// splash takes the card's radius from here rather than each screen re-typing
/// an inset of its own.
ListTileThemeData oxListTileTheme(OpenoxideColors ox, TextTheme text) =>
    ListTileThemeData(
      contentPadding: const EdgeInsets.symmetric(horizontal: Spacing.md),
      iconColor: ox.textMuted,
      textColor: ox.text,
      shape: RoundedRectangleBorder(borderRadius: Radii.cardR),
      titleTextStyle: text.bodyMedium,
      subtitleTextStyle: text.bodySmall,
    );

ProgressIndicatorThemeData oxProgressTheme(OpenoxideColors ox) =>
    ProgressIndicatorThemeData(
      color: ox.accent,
      linearTrackColor: ox.subtleFill,
      circularTrackColor: Colors.transparent,
    );

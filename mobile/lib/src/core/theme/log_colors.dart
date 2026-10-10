import 'package:flutter/material.dart';
import 'package:mobile/src/core/logs/ansi_sgr.dart';
import 'package:mobile/src/core/logs/log_tone.dart';

/// Log text colour for a tone at the active brightness.
Color logColor(LogTone tone, Brightness brightness) => Color(
  brightness == Brightness.dark
      ? toneColors(tone).dark
      : toneColors(tone).light,
);

/// The 16 ANSI palette, dimmed half a step in dark mode the way a terminal with
/// a dark background would.
Color? ansiColor(AnsiColor? color, Brightness brightness) {
  if (color == null) return null;
  const base = <int>[
    0x000000,
    0x7F1D1D,
    0x14532D,
    0x78350F,
    0x1E3A8A,
    0x6B21A8,
    0x155E75,
    0xD4D4D8,
    0x52525B,
    0xEF4444,
    0x22C55E,
    0xEAB308,
    0x3B82F6,
    0xA855F7,
    0x06B6D4,
    0xFFFFFF,
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
      color: (color ?? fallback).withValues(
        alpha: brightness == Brightness.dark ? 0.55 : 0.65,
      ),
    );
  }
  if (styles.contains(AnsiStyle.underline)) {
    result = result.copyWith(decoration: TextDecoration.underline);
  }
  return result;
}

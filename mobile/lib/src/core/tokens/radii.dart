import 'package:flutter/material.dart';

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

import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';

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

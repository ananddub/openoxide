import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

/// The one button in the app.
///
/// Before this, ten call sites each built their own button and exactly one of
/// them handled a disabled or busy state, which is why some actions stayed
/// tappable while their request was still in flight. Height is set explicitly
/// because Material's default button is ~40dp — under Android's 48dp minimum
/// touch target.
class ActionButton extends StatelessWidget {
  const ActionButton({
    super.key,
    required this.label,
    this.onPressed,
    this.icon,
    this.loading = false,
    this.expanded = false,
    this.size = ButtonSize.medium,
    this.tone = ButtonTone.accent,
  });

  final String label;
  final VoidCallback? onPressed;
  final IconData? icon;
  final bool loading;
  final bool expanded;
  final ButtonSize size;
  final ButtonTone tone;

  /// `small` is only for toolbars and dense rows, where the surrounding padding
  /// restores a 48dp hit area.
  double get _height => switch (size) {
    ButtonSize.small => 40,
    ButtonSize.medium => 48,
    ButtonSize.large => 52,
  };

  @override
  Widget build(BuildContext context) {
    final ox = context.ox;
    final enabled = onPressed != null && !loading;
    final child = loading
        ? const SizedBox.square(
            dimension: Mark.glyph,
            child: CircularProgressIndicator(strokeWidth: Mark.stroke),
          )
        : Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (icon != null) ...[
                Icon(icon, size: Mark.glyph),
                const SizedBox(width: Spacing.sm),
              ],
              Text(label),
            ],
          );

    Widget button = switch (tone) {
      ButtonTone.outline => OutlinedButton(
        onPressed: enabled ? onPressed : null,
        child: child,
      ),
      ButtonTone.text => TextButton(
        onPressed: enabled ? onPressed : null,
        child: child,
      ),
      ButtonTone.neutral => FilledButton(
        onPressed: enabled ? onPressed : null,
        style: FilledButton.styleFrom(
          backgroundColor: ox.text,
          foregroundColor: ox.page,
        ),
        child: child,
      ),
      ButtonTone.danger => FilledButton(
        onPressed: enabled ? onPressed : null,
        style: FilledButton.styleFrom(
          backgroundColor: ox.danger,
          foregroundColor: ox.onAccent,
        ),
        child: child,
      ),
      ButtonTone.accent => FilledButton(
        onPressed: enabled ? onPressed : null,
        child: child,
      ),
    };

    return SizedBox(
      height: _height,
      width: expanded ? double.infinity : null,
      child: button,
    );
  }
}

enum ButtonSize { small, medium, large }

enum ButtonTone { accent, neutral, outline, text, danger }

/// Entrance delay for the [index]th row of a list.
///
/// Capped: past a handful of rows the rest are off-screen anyway, and an
/// uncapped queue would leave the bottom of a long list blank while the reader
/// is already scrolling through it.
Duration rowDelay(int index) =>
    Motion.stagger * (index > _rowStaggerCap ? _rowStaggerCap : index);

const int _rowStaggerCap = 8;

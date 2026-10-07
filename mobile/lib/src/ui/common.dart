import 'package:flutter/material.dart';

import '../config/app_config.dart';
import '../grpc/app_error.dart';
import 'tokens.dart';

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

/// Pull-to-refresh, standard insets and the three non-content states, so every
/// list in the app places them identically.
///
/// Deliberately presentational: the caller keeps owning its `Query` and
/// disposing it, so this cannot disturb the realtime invalidation contract.
class SectionScaffold extends StatelessWidget {
  const SectionScaffold({
    super.key,
    this.children = const [],
    this.onRefresh,
    this.loading = false,
    this.error,
    this.onRetry,
    this.emptyTitle,
    this.emptySubtitle,
    this.padding,
  });

  final List<Widget> children;
  final Future<void> Function()? onRefresh;
  final bool loading;
  final AppError? error;
  final Future<void> Function()? onRetry;
  final String? emptyTitle;
  final String? emptySubtitle;
  final EdgeInsets? padding;

  static const EdgeInsets listPadding = EdgeInsets.fromLTRB(
    Spacing.gutter,
    Spacing.sm,
    Spacing.gutter,
    Spacing.fabInset,
  );

  @override
  Widget build(BuildContext context) {
    final hasContent = children.isNotEmpty;
    if (!hasContent) {
      if (loading) return const Center(child: Loading());
      if (error != null) {
        return _centered(
          ErrorNotice(
            error: error!,
            onRetry: onRetry == null ? null : () => onRetry!(),
          ),
        );
      }
      if (emptyTitle != null) {
        return _centered(
          EmptyNotice(title: emptyTitle!, subtitle: emptySubtitle),
        );
      }
    }
    final list = ListView(
      physics: const AlwaysScrollableScrollPhysics(),
      padding: padding ?? listPadding,
      children: children,
    );
    if (onRefresh == null) return list;
    return RefreshIndicator(onRefresh: onRefresh!, child: list);
  }

  /// Empty and failed states sit a little below the optical centre rather than
  /// at whatever offset each screen happened to choose, which used to be 80, 80
  /// and 60 in three different files.
  Widget _centered(Widget child) => ListView(
    physics: const AlwaysScrollableScrollPhysics(),
    padding: (padding ?? listPadding) + const EdgeInsets.only(top: Spacing.xxl),
    children: [child],
  );
}

/// A group heading inside a scrolling list: upper-cased, letter-spaced, and in
/// the colour that reads as a label rather than as content. The project detail
/// and the deploy queue both needed one and disagreed on colour and inset.
class SectionLabel extends StatelessWidget {
  const SectionLabel(this.title, {super.key, this.first = false});

  final String title;

  /// The first heading sits under the list's own top padding, so it does not
  /// want the extra gap the headings below it do.
  final bool first;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.fromLTRB(
        Spacing.xs,
        first ? Spacing.xs : Spacing.lg,
        Spacing.xs,
        Spacing.sm,
      ),
      child: Text(
        title.toUpperCase(),
        style: Theme.of(context).textTheme.labelSmall?.copyWith(
          letterSpacing: 1,
          color: context.ox.textMuted,
        ),
      ),
    );
  }
}

/// Shared pieces every screen needs, so "failed" and "empty" look the same
/// wherever they appear.
class Loading extends StatelessWidget {
  const Loading({super.key, this.label});

  final String? label;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const CircularProgressIndicator(strokeWidth: Mark.stroke),
          if (label != null) ...[
            const SizedBox(height: Spacing.md),
            Text(label!, style: Theme.of(context).textTheme.labelMedium),
          ],
        ],
      ),
    );
  }
}

class ErrorNotice extends StatelessWidget {
  const ErrorNotice({super.key, required this.error, this.onRetry});

  final AppError error;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          error.userMessage,
          textAlign: TextAlign.center,
          style: Theme.of(context).textTheme.bodyMedium,
        ),
        if (onRetry != null) ...[
          const SizedBox(height: Spacing.md),
          ActionButton(
            label: 'Try again',
            tone: ButtonTone.neutral,
            size: ButtonSize.small,
            onPressed: onRetry,
          ),
        ],
      ],
    );
  }
}

class EmptyNotice extends StatelessWidget {
  const EmptyNotice({super.key, required this.title, this.subtitle});

  final String title;
  final String? subtitle;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(Spacing.xxl),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              title,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            if (subtitle != null) ...[
              const SizedBox(height: Spacing.xs),
              Text(
                subtitle!,
                textAlign: TextAlign.center,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ],
          ],
        ),
      ),
    );
  }
}

/// The panel address in use. Shown wherever a connection failure appears,
/// because the most common cause is pointing the app at the wrong host.
class PanelTargetLabel extends StatelessWidget {
  const PanelTargetLabel({super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      'https://${AppConfig.panelHost}:${AppConfig.panelPort}',
      style: Theme.of(context).textTheme.labelSmall,
    );
  }
}

/// Turns an enum's wire name into a label a reader can scan:
/// `REMOTE_SERVER_STATUS_ACTIVE` → "Active", `OVERVIEW_SERVICE_TYPE_DB` → "Db".
/// Anything without the prefix is already a label, so it is returned untouched
/// rather than hidden.
String wireLabel(String value, String prefix) {
  if (!value.startsWith(prefix) || value.length == prefix.length) return value;
  final words = value
      .substring(prefix.length)
      .split('_')
      .where((word) => word.isNotEmpty)
      .map((word) => word.toLowerCase())
      .map((word) => word[0].toUpperCase() + word.substring(1));
  final label = words.join(' ');
  return label.isEmpty ? value : label;
}

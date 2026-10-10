import 'package:flutter/material.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/status_notices.dart';

/// Pull-to-refresh, standard insets and the three non-content states, so every
/// list in the app places them identically.
///
/// Deliberately presentational: the caller keeps owning its bloc and disposing
/// it, so this cannot disturb the realtime invalidation contract.
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

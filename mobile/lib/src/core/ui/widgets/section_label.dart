import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/spacing.dart';

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
        style: Theme.of(context).textTheme.labelSmall
            ?.copyWith(letterSpacing: 1, color: context.ox.textMuted),
      ),
    );
  }
}

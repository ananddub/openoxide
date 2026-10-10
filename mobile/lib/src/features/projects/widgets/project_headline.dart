import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/domain/env_blob.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/section_label.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';

/// The project's own card at the top of the detail screen.
class ProjectHeadline extends StatelessWidget {
  const ProjectHeadline({super.key, required this.project});

  final Project project;

  @override
  Widget build(BuildContext context) {
    final variables = envVariableCount(project.environmentVariables);
    final text = Theme.of(context).textTheme;
    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.all(Spacing.lg),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(project.name, style: text.titleMedium),
            if (project.description.isNotEmpty) ...[
              const SizedBox(height: Spacing.xs),
              Text(project.description, style: text.bodyMedium),
            ],
            const SizedBox(height: Spacing.sm),
            Text(
              '$variables env ${variables == 1 ? 'variable' : 'variables'}'
              '  ·  ${project.id}',
              style: text.labelSmall,
            ),
          ],
        ),
      ),
    ).animate().fadeIn(duration: Motion.slow).slideY(begin: Motion.liftRow);
  }
}

/// A titled group of rows inside the detail screen, with its own empty line.
class DetailSection extends StatelessWidget {
  const DetailSection({
    super.key,
    required this.title,
    required this.children,
    required this.empty,
  });

  final String title;
  final List<Widget> children;
  final String empty;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SectionLabel(title),
        if (children.isEmpty)
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: Spacing.xs,
              vertical: Spacing.sm,
            ),
            child: Text(empty, style: text.bodySmall),
          )
        else
          ...children,
      ],
    );
  }
}

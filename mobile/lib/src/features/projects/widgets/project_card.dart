import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/domain/env_blob.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/features/projects/screens/project_detail_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';

/// One project as a tappable card.
class ProjectCard extends StatelessWidget {
  const ProjectCard({
    super.key,
    required this.project,
    this.delay = Duration.zero,
  });

  final Project project;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final variables = envVariableCount(project.environmentVariables);
    final text = Theme.of(context).textTheme;
    // Margins, corners and the row's horizontal inset all come from the theme;
    // a card here used to repeat them with different numbers.
    return Card(
          child: ListTile(
            title: Text(project.name, style: text.titleMedium),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (project.description.isNotEmpty) ...[
                  const SizedBox(height: Spacing.xs),
                  Text(
                    project.description,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: text.bodySmall,
                  ),
                ],
                const SizedBox(height: Spacing.xs),
                Text(
                  '$variables env ${variables == 1 ? 'variable' : 'variables'}'
                  '  ·  ${project.id}',
                  style: text.labelSmall,
                ),
              ],
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) =>
                    ProjectDetailScreen(projectId: project.id.toInt()),
              ),
            ),
          ),
        )
        .animate(delay: delay)
        .fadeIn(duration: Motion.fast)
        .slideY(begin: Motion.liftRow);
  }
}

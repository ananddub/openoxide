import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/project.pb.dart';
import '../gen/openoxide/api/v1/realtime.pbenum.dart';
import '../state/auth_controller.dart';
import '../state/deps.dart';
import '../state/env_blob.dart';
import '../state/query.dart';
import 'common.dart';
import 'project_detail_screen.dart';
import 'tokens.dart';

/// Everything in one organization, one card each.
///
/// The panel answers the whole list at once — there is no paging on this RPC —
/// so the screen sorts locally instead of pretending a server filter exists.
class ProjectsScreen extends StatefulWidget {
  const ProjectsScreen({super.key});

  @override
  State<ProjectsScreen> createState() => _ProjectsScreenState();
}

class _ProjectsScreenState extends State<ProjectsScreen> {
  Query<List<Project>>? _query;
  int? _organizationId;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final deps = context.read<Deps>();
    // `watch`, not `read`: the dependency is what makes this method run again
    // when the organization changes. Reading it would leave every list on
    // screen showing the organization that was selected when the tab first
    // opened, because the shell keeps its tabs alive to preserve their state.
    final organizationId = context.watch<AuthController>().organizationId;
    if (organizationId == null) return;
    if (_query != null && _organizationId == organizationId) return;
    _query?.dispose();
    _organizationId = organizationId;
    _query = Query<List<Project>>(
      load: () async {
        final response = await deps.api.listProjects(organizationId);
        final projects = [...response.projects]
          ..sort((a, b) => a.name.toLowerCase().compareTo(b.name.toLowerCase()));
        return projects;
      },
      realtime: deps.realtime,
      listensOn: const [WatchedTable.WATCHED_TABLE_PROJECTS],
    )..start();
  }

  @override
  Widget build(BuildContext context) {
    final query = _query;
    if (query == null) {
      return const SectionScaffold(
        emptyTitle: 'No organization selected',
        emptySubtitle: 'Choose one from the organization menu at the top.',
      );
    }
    return ListenableBuilder(
      listenable: query,
      builder: (context, _) {
        final projects = query.data;
        final hasRows = projects != null && projects.isNotEmpty;
        return SectionScaffold(
          onRefresh: query.refresh,
          // A failed refresh over a list that already arrived is not worth
          // replacing the list with a retry button; only a first load that
          // never came back is.
          loading: !hasRows && query.loading && projects == null,
          error: projects == null ? query.error : null,
          onRetry: query.refresh,
          emptyTitle: 'No projects yet',
          emptySubtitle: 'Create one in the web dashboard to see it here.',
          children: [
            if (projects != null)
              for (var index = 0; index < projects.length; index++)
                _ProjectCard(project: projects[index], delay: rowDelay(index)),
          ],
        );
      },
    );
  }

  @override
  void dispose() {
    _query?.dispose();
    super.dispose();
  }
}

class _ProjectCard extends StatelessWidget {
  const _ProjectCard({required this.project, this.delay = Duration.zero});

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
            builder: (_) => ProjectDetailScreen(projectId: project.id.toInt()),
          ),
        ),
      ),
    ).animate(delay: delay).fadeIn(duration: Motion.fast).slideY(
      begin: Motion.liftRow,
    );
  }
}

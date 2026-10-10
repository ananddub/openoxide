import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/section_scaffold.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/features/projects/bloc/projects_bloc.dart';
import 'package:mobile/src/features/projects/widgets/project_card.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';

/// Everything in one organization, one card each.
///
/// The bloc is rebuilt when the organization changes, because the shell keeps
/// its tabs alive to preserve their state: a list that only read the id once
/// would keep showing the organization that was selected when the tab first
/// opened. The swap happens in build — the only place [context.select] may run —
/// and the previous bloc closes once the frame is done.
class ProjectsScreen extends StatefulWidget {
  const ProjectsScreen({super.key});

  @override
  State<ProjectsScreen> createState() => _ProjectsScreenState();
}

class _ProjectsScreenState extends State<ProjectsScreen> {
  ProjectsBloc? _bloc;
  int? _organizationId;

  @override
  Widget build(BuildContext context) {
    final deps = context.read<Deps>();
    final organizationId = context.select<AuthBloc, int?>(
      (bloc) => bloc.organizationId,
    );
    if (organizationId != null && organizationId != _organizationId) {
      final previous = _bloc;
      _organizationId = organizationId;
      _bloc = ProjectsBloc(
        api: deps.api,
        organizationId: organizationId,
        realtime: deps.realtime,
      )..start();
      WidgetsBinding.instance.addPostFrameCallback((_) => previous?.close());
    }

    final bloc = _bloc;
    if (bloc == null) {
      return const SectionScaffold(
        emptyTitle: 'No organization selected',
        emptySubtitle: 'Choose one from the organization menu at the top.',
      );
    }
    return BlocProvider<ProjectsBloc>.value(
      value: bloc,
      child: BlocBuilder<ProjectsBloc, QueryState<List<Project>>>(
        builder: (context, state) {
          final projects = state.data;
          final hasRows = projects != null && projects.isNotEmpty;
          return SectionScaffold(
            onRefresh: () async => bloc.refresh(),
            // A failed refresh over a list that already arrived is not worth
            // replacing the list with a retry button; only a first load that
            // never came back is.
            loading: !hasRows && state.loading && projects == null,
            error: projects == null ? state.error : null,
            onRetry: () async => bloc.refresh(),
            emptyTitle: 'No projects yet',
            emptySubtitle: 'Create one in the web dashboard to see it here.',
            children: [
              if (projects != null)
                for (var index = 0; index < projects.length; index++)
                  ProjectCard(project: projects[index], delay: rowDelay(index)),
            ],
          );
        },
      ),
    );
  }

  @override
  void dispose() {
    _bloc?.close();
    super.dispose();
  }
}

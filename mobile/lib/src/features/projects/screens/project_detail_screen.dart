import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/section_scaffold.dart';
import 'package:mobile/src/features/projects/bloc/project_detail_bloc.dart';
import 'package:mobile/src/features/projects/widgets/project_headline.dart';
import 'package:mobile/src/features/projects/widgets/service_tile.dart';

/// One project with the things the panel runs for it.
class ProjectDetailScreen extends StatefulWidget {
  const ProjectDetailScreen({super.key, required this.projectId});

  final int projectId;

  @override
  State<ProjectDetailScreen> createState() => _ProjectDetailScreenState();
}

class _ProjectDetailScreenState extends State<ProjectDetailScreen> {
  ProjectDetailBloc? _bloc;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_bloc != null) return;
    final deps = context.read<Deps>();
    _bloc = ProjectDetailBloc(
      api: deps.api,
      session: deps.session,
      projectId: widget.projectId,
      realtime: deps.realtime,
    )..start();
  }

  @override
  void dispose() {
    _bloc?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final bloc = _bloc;
    if (bloc == null) return const Scaffold(body: SizedBox.shrink());
    return BlocProvider<ProjectDetailBloc>.value(
      value: bloc,
      child: Scaffold(
        // A generic title: the headline card carries the name, and a title
        // that repeats it reads as two things to read.
        appBar: AppBar(title: const Text('Project')),
        body: SafeArea(
          child: BlocBuilder<ProjectDetailBloc, QueryState<ProjectBundle>>(
            builder: (context, state) {
              final bundle = state.data;
              return SectionScaffold(
                // No floating button on a pushed screen, so the list only needs
                // room to breathe at the bottom.
                padding: const EdgeInsets.fromLTRB(
                  Spacing.gutter,
                  Spacing.sm,
                  Spacing.gutter,
                  Spacing.xl,
                ),
                onRefresh: () async => bloc.refresh(),
                loading: bundle == null && state.loading,
                error: bundle == null ? state.error : null,
                onRetry: () async => bloc.refresh(),
                children: [
                  if (bundle != null) ...[
                    ProjectHeadline(project: bundle.project),
                    DetailSection(
                      title: 'Services',
                      empty: 'No services in this project yet',
                      children: [
                        for (
                          var index = 0;
                          index < bundle.services.length;
                          index++
                        )
                          ServiceTile(
                            service: bundle.services[index],
                            delay: rowDelay(index),
                          ),
                      ],
                    ),
                    DetailSection(
                      title: 'Domains',
                      empty: 'No domains point at this project',
                      children: [
                        for (
                          var index = 0;
                          index < bundle.domains.length;
                          index++
                        )
                          DomainTile(
                            domain: bundle.domains[index],
                            delay: rowDelay(index),
                          ),
                      ],
                    ),
                  ],
                ],
              );
            },
          ),
        ),
      ),
    );
  }
}

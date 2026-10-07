import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/overview.pb.dart';
import '../gen/openoxide/api/v1/project.pb.dart';
import '../gen/openoxide/api/v1/realtime.pbenum.dart';
import '../state/deps.dart';
import '../state/env_blob.dart';
import '../state/query.dart';
import 'common.dart';
import 'service_logs_screen.dart';
import 'tokens.dart';

/// One project with the things the panel runs for it.
///
/// Services and domains are listed per organization by the panel, so this screen
/// filters by project id locally. That is not laziness: the overview RPCs have no
/// project filter, and asking for them is the only implemented path.
class ProjectDetailScreen extends StatefulWidget {
  const ProjectDetailScreen({super.key, required this.projectId});

  final int projectId;

  @override
  State<ProjectDetailScreen> createState() => _ProjectDetailScreenState();
}

typedef _ProjectBundle =
    ({
        Project project,
        List<OverviewServiceItem> services,
        List<OverviewDomainItem> domains,
    });

class _ProjectDetailScreenState extends State<ProjectDetailScreen> {
  late final Query<_ProjectBundle> _query;

  @override
  void initState() {
    super.initState();
    final deps = context.read<Deps>();
    final projectId = widget.projectId;
    _query = Query<_ProjectBundle>(
      load: () async {
        final organizationId = deps.session.organizationId ?? 0;
        // All three are in flight together, and the wait listens to every one of
        // them: a panel that is down rejects all three, and a rejection nobody
        // listened to would surface as an unhandled error instead of the single
        // message on screen. `Future.wait` fails with the first, which is what
        // the reader sees.
        final project = deps.api.getProject(projectId);
        final services = deps.api.listOrganizationServices(organizationId);
        final domains = deps.api.listOrganizationDomains(organizationId);
        await Future.wait<Object?>([project, services, domains]);
        final bundle = (await project, await services, await domains);
        return (
          project: bundle.$1,
          services: bundle.$2.services
              .where((item) => item.projectId.toInt() == projectId)
              .toList(),
          // The overview rows carry the project name the panel filled in, which
          // is the only project link a domain row has.
          domains: bundle.$3.domains
              .where((item) => item.projectName == bundle.$1.name)
              .toList(),
        );
      },
      realtime: deps.realtime,
      listensOn: const [
        WatchedTable.WATCHED_TABLE_SERVICES,
        WatchedTable.WATCHED_TABLE_DOMAINS,
        WatchedTable.WATCHED_TABLE_DEPLOYMENTS,
      ],
    )..start();
  }

  @override
  void dispose() {
    _query.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(_query.data?.project.name ?? 'Project')),
      body: SafeArea(
        child: ListenableBuilder(
          listenable: _query,
          builder: (context, _) {
            final bundle = _query.data;
            return SectionScaffold(
              // No floating button on a pushed screen, so the list only needs
              // room to breathe at the bottom.
              padding: const EdgeInsets.fromLTRB(
                Spacing.gutter,
                Spacing.sm,
                Spacing.gutter,
                Spacing.xl,
              ),
              onRefresh: _query.refresh,
              loading: bundle == null && _query.loading,
              error: bundle == null ? _query.error : null,
              onRetry: _query.refresh,
              children: [
                if (bundle != null) ...[
                  _ProjectHeadline(project: bundle.project),
                  _Section(
                    title: 'Services',
                    empty: 'No services in this project yet',
                    children: [
                      for (var index = 0; index < bundle.services.length; index++)
                        _ServiceTile(
                          service: bundle.services[index],
                          delay: rowDelay(index),
                        ),
                    ],
                  ),
                  _Section(
                    title: 'Domains',
                    empty: 'No domains point at this project',
                    children: [
                      for (var index = 0; index < bundle.domains.length; index++)
                        _DomainTile(
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
    );
  }
}

class _ProjectHeadline extends StatelessWidget {
  const _ProjectHeadline({required this.project});

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

class _Section extends StatelessWidget {
  const _Section({
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

class _ServiceTile extends StatelessWidget {
  const _ServiceTile({required this.service, this.delay = Duration.zero});

  final OverviewServiceItem service;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final name = service.name.isEmpty ? 'Service ${service.id}' : service.name;
    return Card(
      child: ListTile(
        title: Text(name, style: Theme.of(context).textTheme.titleMedium),
        subtitle: Text(
          [
            if (service.serviceType.name.isNotEmpty)
              wireLabel(service.serviceType.name, 'OVERVIEW_SERVICE_TYPE_'),
            if (service.status.isNotEmpty) service.status,
            if (service.serverName.isNotEmpty) service.serverName,
            if (service.dbKind.isNotEmpty) service.dbKind,
          ].join('  ·  '),
          style: Theme.of(context).textTheme.bodySmall,
        ),
        trailing: const Icon(Icons.description_outlined),
        onTap: () => Navigator.of(context).push(
          MaterialPageRoute(
            builder: (_) => ServiceLogsScreen(
              title: name,
              target: name,
              serverId: service.serverId.toInt() == 0
                  ? null
                  : service.serverId.toInt(),
            ),
          ),
        ),
      ),
    ).animate(delay: delay).fadeIn(duration: Motion.fast).slideY(
      begin: Motion.liftRow,
    );
  }
}

class _DomainTile extends StatelessWidget {
  const _DomainTile({required this.domain, this.delay = Duration.zero});

  final OverviewDomainItem domain;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final url = '${domain.https ? 'https' : 'http'}://'
        '${domain.host}${domain.path.isEmpty ? '' : domain.path}';
    return Card(
      child: ListTile(
        title: Text(domain.host),
        subtitle: Text(
          [
            if (domain.domainType.isNotEmpty) domain.domainType,
            if (domain.serviceName.isNotEmpty) domain.serviceName,
            if (domain.port != 0) ':${domain.port}',
          ].join('  ·  '),
          style: Theme.of(context).textTheme.bodySmall,
        ),
        trailing: Icon(
          domain.https ? Icons.lock_outline : Icons.lock_open_outlined,
        ),
        onTap: () => ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(url), duration: Motion.notice),
        ),
      ),
    ).animate(delay: delay).fadeIn(duration: Motion.fast).slideY(
      begin: Motion.liftRow,
    );
  }
}


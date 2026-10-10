export 'package:mobile/src/core/data/query_bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/gen/openoxide/api/v1/overview.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart' show WatchedTable;

/// One project plus the rows the panel runs for it.
///
/// Services and domains are listed per organization by the panel, so the bloc
/// filters by project id locally. That is not laziness: the overview RPCs have
/// no project filter, and asking for them is the only implemented path.
class ProjectBundle extends Equatable {
  const ProjectBundle({
    required this.project,
    required this.services,
    required this.domains,
  });

  final Project project;
  final List<OverviewServiceItem> services;
  final List<OverviewDomainItem> domains;

  @override
  List<Object?> get props => [project, services, domains];
}

class ProjectDetailBloc extends QueryBloc<ProjectBundle> {
  ProjectDetailBloc({
    required PanelApi api,
    required Session session,
    required int projectId,
    super.realtime,
  }) : super(
         load: () async {
           final organizationId = session.organizationId ?? 0;
           // All three are in flight together, and the wait listens to every one
           // of them: a panel that is down rejects all three, and a rejection
           // nobody listened to would surface as an unhandled error instead of
           // the single message on screen.
           final project = api.getProject(projectId);
           final services = api.listOrganizationServices(organizationId);
           final domains = api.listOrganizationDomains(organizationId);
           await Future.wait<Object?>([project, services, domains]);
           final answered = (await project, await services, await domains);
           return ProjectBundle(
             project: answered.$1,
             services: answered.$2.services
                 .where((item) => item.projectId.toInt() == projectId)
                 .toList(),
             // The overview rows carry the project name the panel filled in,
             // which is the only project link a domain row has.
             domains: answered.$3.domains
                 .where((item) => item.projectName == answered.$1.name)
                 .toList(),
           );
         },
         listensOn: const [
           WatchedTable.WATCHED_TABLE_SERVICES,
           WatchedTable.WATCHED_TABLE_DOMAINS,
           WatchedTable.WATCHED_TABLE_DEPLOYMENTS,
         ],
       );
}

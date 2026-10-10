export 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart' show WatchedTable;

/// The sorted project list of one organization.
///
/// The panel answers the whole list at once — there is no paging on this RPC —
/// so the bloc sorts locally instead of pretending a server filter exists.
class ProjectsBloc extends QueryBloc<List<Project>> {
  ProjectsBloc({
    required PanelApi api,
    required int organizationId,
    super.realtime,
  }) : super(
         load: () async {
           final response = await api.listProjects(organizationId);
           final projects = [...response.projects]
             ..sort(
               (a, b) => a.name.toLowerCase().compareTo(b.name.toLowerCase()),
             );
           return projects;
         },
         listensOn: const [WatchedTable.WATCHED_TABLE_PROJECTS],
       );
}

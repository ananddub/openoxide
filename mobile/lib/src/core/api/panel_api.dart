import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart' show ClientChannel;
import 'package:grpc/service_api.dart' show CallOptions;
import 'package:mobile/src/core/api/auth_section.dart';
import 'package:mobile/src/core/api/log_streams_section.dart';
import 'package:mobile/src/core/grpc/auth_retry.dart';
import 'package:mobile/src/core/grpc/metadata.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/common.pb.dart' show Empty;
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/overview.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbgrpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/remote_server.pbgrpc.dart';

/// The gRPC clients a [PanelApi] talks through, named so the section mixins can
/// require them without reaching into private state.
abstract class PanelApiBase {
  GuardedCalls get calls;
  Session get session;
  AuthServiceClient get authClient;
  DeploymentServiceClient get deploymentClient;
  RealtimeServiceClient get realtimeClient;
}

/// The panel surface the app uses. Screens talk to this, never to a generated
/// client, so every call goes through the same auth metadata and refresh-and-
/// replay path and leaves failures as [AppError]s.
///
/// The call groups live in the section mixins beside this file, which keeps
/// each file small and every method still a real member — the test fake
/// overrides them exactly like before.
class PanelApi extends PanelApiBase with AuthSection, LogStreamsSection {
  PanelApi({
    required this.session,
    required this.calls,
    required ClientChannel channel,
  }) : _auth = AuthServiceClient(channel, options: _authOptions(session)),
       _organizations = OrganizationServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _projects = ProjectServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _overview = OverviewServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _deployments = DeploymentServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _monitoring = MonitoringServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _servers = RemoteServerServiceClient(
         channel,
         options: _authOptions(session),
       ),
       _realtime = RealtimeServiceClient(
         channel,
         options: _authOptions(session),
       );

  @override
  final Session session;
  @override
  final GuardedCalls calls;

  final AuthServiceClient _auth;
  final OrganizationServiceClient _organizations;
  final ProjectServiceClient _projects;
  final OverviewServiceClient _overview;
  final DeploymentServiceClient _deployments;
  final MonitoringServiceClient _monitoring;
  final RemoteServerServiceClient _servers;
  final RealtimeServiceClient _realtime;

  @override
  AuthServiceClient get authClient => _auth;
  @override
  DeploymentServiceClient get deploymentClient => _deployments;
  @override
  RealtimeServiceClient get realtimeClient => _realtime;

  static CallOptions _authOptions(Session session) =>
      CallOptions(providers: [authMetadataProvider(session)]);

  // ---- organizations ----

  Future<ListOrganizationsResponse> listOrganizations() => calls.unary(
    () => _organizations.listOrganizations(ListOrganizationsRequest()),
  );

  // ---- projects ----

  /// `organization_id` is a body field the controller validates as `gt: 0`; the
  /// header alone is not enough.
  Future<ListProjectsResponse> listProjects(int organizationId) => calls.unary(
    () => _projects.listProjects(
      ListProjectsRequest(organizationId: Int64(organizationId)),
    ),
  );

  Future<Project> getProject(int projectId) => calls.unary(
    () => _projects.getProject(GetProjectRequest(id: Int64(projectId))),
  );

  // ---- project detail ----

  /// Not `listServices`: the overview proto labels those `ListServices`-named
  /// RPCs legacy aliases with no server-side implementation.
  Future<OverviewListOrganizationServicesResponse> listOrganizationServices(
    int organizationId,
  ) => calls.unary(
    () => _overview.listOrganizationServices(
      OverviewListOrganizationServicesRequest(
        organizationId: Int64(organizationId),
      ),
    ),
  );

  Future<OverviewListOrganizationDomainsResponse> listOrganizationDomains(
    int organizationId,
  ) => calls.unary(
    () => _overview.listOrganizationDomains(
      OverviewListOrganizationDomainsRequest(
        organizationId: Int64(organizationId),
      ),
    ),
  );

  // ---- deployments ----

  Future<ListDeploymentsResponse> listDeployments({int limit = 50}) =>
      calls.unary(
        () => _deployments.listDeployments(
          ListDeploymentsRequest(limit: Int64(limit)),
        ),
      );

  Future<ListDeploymentsResponse> listActiveDeployments() =>
      calls.unary(() => _deployments.listActiveDeployments(Empty()));

  Future<Deployment> getDeployment(int deploymentId) => calls.unary(
    () => _deployments.getDeployment(
      GetDeploymentRequest(id: Int64(deploymentId)),
    ),
  );

  // ---- servers and health ----

  /// `ServerService` is declared in the proto but has no server-side
  /// implementation; `RemoteServerService` is what the web dashboard calls.
  Future<RemoteServerListResponse> listServers() =>
      calls.unary(() => _servers.listRemoteServers(RemoteServerListRequest()));

  /// `getOverview`/`getMetrics`/`getContainerState` answer with `Empty`: these
  /// are the implemented ones.
  Future<MonitoringGetServerMetricsResponse> getServerMetrics(int serverId) =>
      calls.unary(
        () => _monitoring.getServerMetrics(
          MonitoringGetServerMetricsRequest(serverId: Int64(serverId)),
        ),
      );

  Future<MonitoringGetContainerStatesResponse> getContainerStates(
    int serverId,
  ) => calls.unary(
    () => _monitoring.getContainerStates(
      MonitoringGetContainerStatesRequest(serverId: Int64(serverId)),
    ),
  );
}

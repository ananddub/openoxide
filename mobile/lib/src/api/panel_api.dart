import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart' show ClientChannel;
import 'package:grpc/service_api.dart' show CallOptions;

import '../gen/openoxide/api/v1/auth.pbgrpc.dart';
import '../gen/openoxide/api/v1/common.pb.dart';
import '../gen/openoxide/api/v1/deployment.pbgrpc.dart';
import '../gen/openoxide/api/v1/monitoring.pbgrpc.dart';
import '../gen/openoxide/api/v1/organization.pbgrpc.dart';
import '../gen/openoxide/api/v1/overview.pbgrpc.dart';
import '../gen/openoxide/api/v1/project.pbgrpc.dart';
import '../gen/openoxide/api/v1/realtime.pbgrpc.dart';
import '../gen/openoxide/api/v1/remote_server.pbgrpc.dart';
import '../session/session.dart';
import '../grpc/app_error.dart';
import '../grpc/auth_retry.dart';
import '../grpc/metadata.dart';

/// The panel surface the app uses. Screens talk to this, never to a generated
/// client, so every call goes through the same auth metadata and refresh-and-
/// replay path and leaves failures as [AppError]s.
class PanelApi {
  PanelApi({
    required this.session,
    required this._calls,
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

  final Session session;
  final GuardedCalls _calls;

  final AuthServiceClient _auth;
  final OrganizationServiceClient _organizations;
  final ProjectServiceClient _projects;
  final OverviewServiceClient _overview;
  final DeploymentServiceClient _deployments;
  final MonitoringServiceClient _monitoring;
  final RemoteServerServiceClient _servers;
  final RealtimeServiceClient _realtime;

  static CallOptions _authOptions(Session session) =>
      CallOptions(providers: [authMetadataProvider(session)]);

  // ---- auth ----

  /// Public: no credentials. The sign-in screen gates on it.
  Future<SetupStatus> getSetupStatus() => _calls.unary(
    () => _auth.getSetupStatus(GetSetupStatusRequest()),
    allowRetry: false,
  );

  /// Verifies credentials and hands back the session to persist. A login the
  /// panel answers without a token pair counts as a rejection, exactly as an
  /// empty refresh response does.
  Future<StoredSession> signIn({
    required String email,
    required String password,
    String? twoFactorCode,
    String? recoveryCode,
  }) async {
    final response = await _calls.unary(
      () => _auth.login(
        LoginRequest(
          email: email,
          password: password,
          twoFactorCode: twoFactorCode,
          recoveryCode: recoveryCode,
        ),
      ),
      allowRetry: false,
    );
    if (!response.hasTokens()) {
      throw const SessionRejected('the panel returned no tokens');
    }
    final tokens = response.tokens;
    final user = response.hasUser() ? response.user : null;
    return StoredSession(
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
      userId: user?.id.toInt(),
      email: _present(user?.email),
      firstName: _present(user?.firstName),
      lastName: _present(user?.lastName),
      avatar: _present(user?.avatar),
    );
  }

  static String? _present(String? value) =>
      value == null || value.isEmpty ? null : value;

  Future<User> whoAmI() => _calls.unary(() => _auth.whoAmI(WhoAmIRequest()));

  Future<void> logout() => _calls.unary(() => _auth.logout(LogoutRequest()));

  // ---- organizations ----

  Future<ListOrganizationsResponse> listOrganizations() => _calls.unary(
    () => _organizations.listOrganizations(ListOrganizationsRequest()),
  );

  // ---- projects ----

  /// `organization_id` is a body field the controller validates as `gt: 0`; the
  /// header alone is not enough.
  Future<ListProjectsResponse> listProjects(int organizationId) => _calls.unary(
    () => _projects.listProjects(
      ListProjectsRequest(organizationId: Int64(organizationId)),
    ),
  );

  Future<Project> getProject(int projectId) => _calls.unary(
    () => _projects.getProject(GetProjectRequest(id: Int64(projectId))),
  );

  // ---- project detail ----

  /// Not `listServices`: the overview proto labels those `ListServices`-named
  /// RPCs legacy aliases with no server-side implementation.
  Future<OverviewListOrganizationServicesResponse> listOrganizationServices(
    int organizationId,
  ) => _calls.unary(
    () => _overview.listOrganizationServices(
      OverviewListOrganizationServicesRequest(
        organizationId: Int64(organizationId),
      ),
    ),
  );

  Future<OverviewListOrganizationDomainsResponse> listOrganizationDomains(
    int organizationId,
  ) => _calls.unary(
    () => _overview.listOrganizationDomains(
      OverviewListOrganizationDomainsRequest(
        organizationId: Int64(organizationId),
      ),
    ),
  );

  // ---- deployments ----

  Future<ListDeploymentsResponse> listDeployments({int limit = 50}) =>
      _calls.unary(
        () => _deployments.listDeployments(
          ListDeploymentsRequest(limit: Int64(limit)),
        ),
      );

  Future<ListDeploymentsResponse> listActiveDeployments() =>
      _calls.unary(() => _deployments.listActiveDeployments(Empty()));

  Future<Deployment> getDeployment(int deploymentId) => _calls.unary(
    () => _deployments.getDeployment(
      GetDeploymentRequest(id: Int64(deploymentId)),
    ),
  );

  /// Ends when the panel closes the channel at a terminal deployment state;
  /// there is no end-of-log sentinel on the wire.
  Stream<DeploymentEvent> streamDeploymentLogs(int deploymentId) =>
      _calls.serverStream(
        () => _deployments.streamDeploymentLogs(
          GetDeploymentRequest(id: Int64(deploymentId)),
        ),
      );

  Stream<DeploymentEvent> streamServiceLogs({
    required String target,
    int? serverId,
    int tail = 500,
    bool follow = true,
  }) => _calls.serverStream(
    () => _deployments.streamServiceLogs(
      LogTargetRequest(
        target: target,
        serverId: serverId == null ? null : Int64(serverId),
        tail: Int64(tail),
        follow: follow,
      ),
    ),
  );

  Stream<DeploymentEvent> streamContainerLogs({
    required String target,
    int? serverId,
    int tail = 500,
    bool follow = true,
  }) => _calls.serverStream(
    () => _deployments.streamContainerLogs(
      LogTargetRequest(
        target: target,
        serverId: serverId == null ? null : Int64(serverId),
        tail: Int64(tail),
        follow: follow,
      ),
    ),
  );

  // ---- servers and health ----

  /// `ServerService` is declared in the proto but has no server-side
  /// implementation; `RemoteServerService` is what the web dashboard calls.
  Future<RemoteServerListResponse> listServers() =>
      _calls.unary(() => _servers.listRemoteServers(RemoteServerListRequest()));

  /// `getOverview`/`getMetrics`/`getContainerState` answer with `Empty`: these
  /// are the implemented ones.
  Future<MonitoringGetServerMetricsResponse> getServerMetrics(int serverId) =>
      _calls.unary(
        () => _monitoring.getServerMetrics(
          MonitoringGetServerMetricsRequest(serverId: Int64(serverId)),
        ),
      );

  Future<MonitoringGetContainerStatesResponse> getContainerStates(
    int serverId,
  ) => _calls.unary(
    () => _monitoring.getContainerStates(
      MonitoringGetContainerStatesRequest(serverId: Int64(serverId)),
    ),
  );

  // ---- realtime ----

  /// Ticks carry table names only; the first event is a full resync.
  Stream<WatchTablesEvent> watchTables() =>
      _calls.serverStream(() => _realtime.watchTables(Empty()));
}

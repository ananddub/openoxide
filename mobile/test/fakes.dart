import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart' show ClientChannel;
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/grpc/auth_coordinator.dart';
import 'package:mobile/src/core/grpc/auth_retry.dart';
import 'package:mobile/src/core/realtime/realtime_watch.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pb.dart' show SetupStatus;
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/overview.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/project.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart'

    show WatchTablesEvent;
import 'package:mobile/src/gen/openoxide/api/v1/remote_server.pb.dart';

export 'package:mobile/src/core/realtime/realtime_watch.dart';
export 'package:mobile/src/core/session/session.dart';
export 'package:mobile/src/features/auth/bloc/auth_bloc.dart';

/// One login the screen asked for, exactly as it was sent.
typedef SignInAttempt = ({
  String email,
  String password,
  String? twoFactorCode,
});

/// The panel, replaced by a function the test controls.
///
/// It subclasses the real [PanelApi] rather than inventing an interface because
/// the screens — and [AuthBloc] — are written against it, and only the calls a
/// test actually exercises are intercepted. Anything the fake has no answer for
/// throws instead of quietly reaching a socket, so a test that asks a question
/// the fake was not told about fails loudly rather than hanging.
class FakePanelApi extends PanelApi {
  FakePanelApi._(Session session, ClientChannel channel)
    : super(
        session: session,
        calls: _neverRefreshes(session),
        channel: channel,
      );

  factory FakePanelApi.attached(Session session) =>
      FakePanelApi._(session, _deadChannel);

  static final ClientChannel _deadChannel = ClientChannel('127.0.0.1', port: 1);

  final List<SignInAttempt> attempts = [];

  /// Answer for the next login. Tests that need a specific outcome replace this.
  Future<StoredSession> Function(SignInAttempt attempt) onSignIn = (_) async =>
      const StoredSession(
        accessToken: 'access',
        refreshToken: 'refresh',
        userId: 7,
        email: 'test@example.com',
      );

  SetupStatus setupStatus = SetupStatus(ownerPresent: true);

  /// Thrown instead of answering `GetSetupStatus`, to drive the app into the
  /// unreachable or failed-boot stages.
  Object? setupError;

  List<Organization> organizations = const [];

  List<Project> projects = const [];

  /// Every organization the screen asked to list projects inside of, in the
  /// order it asked — so a test can see an organization switch refetch instead
  /// of only seeing the rows the fake returns regardless of which one.
  List<int> projectListRequests = [];

  /// Thrown instead of answering `ListProjects`.
  Object? projectsError;

  /// Answer for `GetProject`. The project detail screen reads one project, then
  /// filters the organization's overview rows down to it.
  Project? projectRow;

  List<OverviewServiceItem> services = const [];

  List<OverviewDomainItem> domains = const [];

  /// Thrown instead of answering all three project-detail calls.
  Object? detailError;

  /// The deploy queue, which the screen asks for as two calls: [activeDeployments]
  /// is the running set, [deployments] the table's history under it.
  List<Deployment> activeDeployments = const [];

  List<Deployment> deployments = const [];

  /// Thrown instead of answering either deployment call.
  Object? deploymentsError;

  /// The lines every log stream hands out, then the channel closes — which is
  /// what the panel does once a deployment settles. [logError] replaces the
  /// whole stream, the way a target docker cannot find does.
  List<DeploymentEvent> logEvents = const [];

  Object? logError;

  /// Answer for `GetDeployment`, which the deployment log screen re-reads when
  /// its stream ends.
  Deployment? deploymentRow;

  /// Health screen: the server list first, then per-server metrics and container
  /// states. A server missing from [serverMetrics] reports no metrics; a server
  /// in [unreachableServers] answers nothing, which is how a dead agent looks.
  List<RemoteServer> servers = const [];

  Map<int, MonitoringServerMetric> serverMetrics = const {};

  List<MonitoringContainerState> containerStates = const [];

  Set<int> unreachableServers = const {};

  Object? logoutError;

  @override
  Future<SetupStatus> getSetupStatus() async {
    final error = setupError;
    if (error != null) throw error;
    return setupStatus;
  }

  @override
  Future<StoredSession> signIn({
    required String email,
    required String password,
    String? twoFactorCode,
    String? recoveryCode,
  }) async {
    final attempt = (
      email: email,
      password: password,
      twoFactorCode: twoFactorCode,
    );
    attempts.add(attempt);
    return onSignIn(attempt);
  }

  @override
  Future<ListOrganizationsResponse> listOrganizations() async =>
      ListOrganizationsResponse(organizations: organizations);

  @override
  Future<ListProjectsResponse> listProjects(int organizationId) async {
    projectListRequests.add(organizationId);
    final error = projectsError;
    if (error != null) throw error;
    return ListProjectsResponse(projects: projects);
  }

  @override
  Future<Project> getProject(int projectId) async {
    final error = detailError;
    if (error != null) throw error;
    final row = projectRow;
    if (row == null) {
      throw StateError('the fake panel has no project to return');
    }
    return row;
  }

  @override
  Future<OverviewListOrganizationServicesResponse> listOrganizationServices(
    int organizationId,
  ) async {
    final error = detailError;
    if (error != null) throw error;
    return OverviewListOrganizationServicesResponse(services: services);
  }

  @override
  Future<OverviewListOrganizationDomainsResponse> listOrganizationDomains(
    int organizationId,
  ) async {
    final error = detailError;
    if (error != null) {
      throw error;
    }
    return OverviewListOrganizationDomainsResponse(domains: domains);
  }

  @override
  Future<RemoteServerListResponse> listServers() async =>
      RemoteServerListResponse(servers: servers);

  @override
  Future<MonitoringGetServerMetricsResponse> getServerMetrics(
    int serverId,
  ) async {
    if (unreachableServers.contains(serverId)) {
      throw const PanelUnreachable('agent did not answer');
    }
    final metric = serverMetrics[serverId];
    return MonitoringGetServerMetricsResponse(
      metrics: metric == null ? const [] : [metric],
    );
  }

  @override
  Future<MonitoringGetContainerStatesResponse> getContainerStates(
    int serverId,
  ) async => MonitoringGetContainerStatesResponse(states: containerStates);

  @override
  Future<ListDeploymentsResponse> listDeployments({int limit = 50}) async {
    final error = deploymentsError;
    if (error != null) throw error;
    return ListDeploymentsResponse(deployments: deployments);
  }

  @override
  Future<ListDeploymentsResponse> listActiveDeployments() async {
    final error = deploymentsError;
    if (error != null) throw error;
    return ListDeploymentsResponse(deployments: activeDeployments);
  }

  @override
  Future<Deployment> getDeployment(int deploymentId) async {
    final row = deploymentRow;
    if (row == null) {
      throw StateError('the fake panel has no deployment to return');
    }
    return row;
  }

  @override
  Stream<DeploymentEvent> streamDeploymentLogs(int deploymentId) =>
      _logStream();

  @override
  Stream<DeploymentEvent> streamServiceLogs({
    required String target,
    int? serverId,
    int tail = 500,
    bool follow = true,
  }) => _logStream();

  @override
  Stream<DeploymentEvent> streamContainerLogs({
    required String target,
    int? serverId,
    int tail = 500,
    bool follow = true,
  }) => _logStream();

  Stream<DeploymentEvent> _logStream() {
    final error = logError;
    if (error != null) return Stream<DeploymentEvent>.error(error);
    return Stream<DeploymentEvent>.fromIterable(logEvents);
  }

  @override
  Future<void> logout() async {
    final error = logoutError;
    if (error != null) throw error;
  }
}

/// A channel no test ever connects through: [FakePanelApi] answers every call
/// itself, so this only satisfies the constructor's type.
///
/// The refresh coordinator it carries is a tripwire — reaching it means a test
/// took a path the fake does not model.
GuardedCalls _neverRefreshes(Session session) => GuardedCalls(
  coordinator: AuthCoordinator(
    session: session,
    isRejected: (_) => true,
    caller: (_) async =>
        throw StateError('the fake panel does not refresh tokens'),
  ),
);

Organization organization(int id, String name) =>
    Organization(id: Int64(id), name: name, slug: name.toLowerCase());

/// A project as the panel sends it — environment variables arrive as one
/// newline-separated `KEY=VALUE` blob, exactly like production.
Project project(
  int id,
  String name, {
  String description = '',
  String env = '',
}) => Project(
  id: Int64(id),
  name: name,
  description: description,
  environmentVariables: env,
);

/// A session in memory: no platform storage, so a test never touches the
/// keychain plugin and never leaks a token into a fixture.
Session memorySession({StoredSession? initial}) =>
    Session(vault: InMemorySessionVault(), initial: initial);

/// An overview service row. The project detail screen keeps only the rows whose
/// [projectId] matches the project on screen, so a test says which project each
/// one belongs to.
OverviewServiceItem serviceItem(
  int id,
  String name,
  int projectId, {
  String status = 'running',
  int serverId = 5,
  String serverName = 'build-01',
  OverviewServiceType serviceType =
      OverviewServiceType.OVERVIEW_SERVICE_TYPE_APP,
}) => OverviewServiceItem(
  id: Int64(id),
  name: name,
  projectId: Int64(projectId),
  serviceType: serviceType,
  status: status,
  serverId: Int64(serverId),
  serverName: serverName,
);

/// An overview domain row, which the detail screen links to a project by name —
/// the only project reference a domain row carries.
OverviewDomainItem domainItem(
  int id,
  String host,
  String projectName, {
  String path = '',
  int port = 443,
  bool https = true,
  String serviceName = 'web',
  String domainType = 'traefik',
}) => OverviewDomainItem(
  id: Int64(id),
  host: host,
  path: path,
  port: port,
  https: https,
  projectName: projectName,
  serviceName: serviceName,
  domainType: domainType,
);

RemoteServer remoteServer(
  int id,
  String name, {
  String ipAddress = '10.0.0.4',
  RemoteServerStatus serverStatus =
      RemoteServerStatus.REMOTE_SERVER_STATUS_ACTIVE,
}) => RemoteServer(
  id: Int64(id),
  name: name,
  ipAddress: ipAddress,
  serverStatus: serverStatus,
);

/// One metric sample. `mem_used` and `disk_used` are percentages while the
/// totals are gigabytes, which is how the agent actually reports.
MonitoringServerMetric serverMetric({
  double cpu = 42,
  double memUsed = 30,
  double memUsedGb = 4.8,
  double memTotal = 16,
  double diskUsed = 25,
  double totalDisk = 200,
}) => MonitoringServerMetric(
  cpu: cpu,
  memUsed: memUsed,
  memUsedGb: memUsedGb,
  memTotal: memTotal,
  diskUsed: diskUsed,
  totalDisk: totalDisk,
);

MonitoringContainerState containerState(
  String name, {
  String state = 'running',
}) =>
    MonitoringContainerState(containerId: 'c-$name', name: name, state: state);

/// A deployment row, [ageSeconds] old.
///
/// The timestamp is derived from the clock the test runs on because the queue
/// renders an age, not a date — a fixture pinned to an epoch would read
/// "59 years" the day someone runs the suite. [milliseconds] writes it the way
/// the older rows in the table are stored, which is what the screen's unit
/// inference exists for.
Deployment deployment(
  int id, {
  String status = 'DEPLOYED',
  String resourceType = '',
  String userEmail = '',
  String error = '',
  int ageSeconds = 30,
  bool milliseconds = false,
}) => Deployment(
  id: Int64(id),
  status: status,
  resourceType: resourceType,
  userEmail: userEmail,
  error: error,
  updatedAt: Int64(
    milliseconds
        ? DateTime.now().toUtc().millisecondsSinceEpoch - ageSeconds * 1000
        : DateTime.now().toUtc().millisecondsSinceEpoch ~/ 1000 - ageSeconds,
  ),
);

/// One wire event. Which stream a line belongs to is read out of the line itself
/// — the panel prefixes `[STDERR]` rather than setting a field — so a test writes
/// the prefix the way the panel does. [stage] labels the step, and an empty one
/// means the line carries no stage at all.
DeploymentEvent logLine(String payload, {String stage = ''}) =>
    DeploymentEvent(type: stage, payload: payload);

/// The scope a screen under test runs in: the bloc the UI reads, the fake
/// panel behind it, and the session both share.
final class AuthScope {
  AuthScope(this.bloc, this.api, this.session);

  final AuthBloc bloc;
  final FakePanelApi api;
  final Session session;
}

/// A bloc wired to the fake panel, not yet booted.
AuthScope authScope({
  StoredSession? initial,
  List<Organization> organizations = const [],
  List<Project> projects = const [],
}) {
  final session = memorySession(initial: initial);
  final api = FakePanelApi.attached(session)
    ..organizations = organizations
    ..projects = projects;
  return AuthScope(AuthBloc(session: session, api: api), api, session);
}

/// A scope whose boot has been requested but not yet run.
///
/// Inside a widget test pending futures only advance on `tester.pump`, so the
/// tests pump first and assert on the settled state afterwards; nothing here
/// awaits.
AuthScope bootedScope({
  StoredSession? initial,
  List<Organization> organizations = const [],
  List<Project> projects = const [],
}) {
  final scope = authScope(
    initial: initial,
    organizations: organizations,
    projects: projects,
  );
  scope.bloc.add(const AuthBootRequested());
  return scope;
}

/// A [Deps] for screens that read it. The channel is a socket address no test
/// ever connects through, because [FakePanelApi] answers every call itself.
///
/// `ticks` is what the watch stream yields, so a test can move a table exactly
/// the way the panel does and see the screen refetch. Leaving it out means a
/// stream that never emits and, since nothing calls [RealtimeWatch.start]
/// either, no reconnect timer that can outlive a test.
Deps fakeDeps(
  FakePanelApi api,
  Session session, {
  Stream<WatchTablesEvent>? ticks,
}) => Deps(
  session: session,
  api: api,
  realtime: RealtimeWatch(open: () => ticks ?? const Stream<Never>.empty()),
  channel: ClientChannel('127.0.0.1', port: 1),
);

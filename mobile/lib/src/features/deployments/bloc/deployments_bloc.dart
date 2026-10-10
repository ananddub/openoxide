export 'package:mobile/src/core/data/query_bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart' show WatchedTable;

/// The deploy queue in two parts: what is running now, then recent history.
///
/// Two calls, not one, because `ListActiveDeployments` answers from the running
/// set while `ListDeployments` answers from the table; a single list would
/// either hide a stuck build or show fifty finished rows above it.
class DeploymentQueue extends Equatable {
  const DeploymentQueue({required this.active, required this.recent});

  final List<Deployment> active;
  final List<Deployment> recent;

  @override
  List<Object?> get props => [active, recent];
}

class DeploymentsBloc extends QueryBloc<DeploymentQueue> {
  DeploymentsBloc({required PanelApi api, super.realtime})
    : super(
        load: () async {
          final history = await api.listDeployments(limit: 50);
          final active = await api.listActiveDeployments();
          return DeploymentQueue(
            active: active.deployments.toList(),
            recent: history.deployments.toList(),
          );
        },
        listensOn: const [WatchedTable.WATCHED_TABLE_DEPLOYMENTS],
      );
}

/// One deployment's status row, refetched while the log screen is open.
class DeploymentStatusBloc extends QueryBloc<Deployment> {
  DeploymentStatusBloc({
    required PanelApi api,
    required int deploymentId,
    super.realtime,
  }) : super(
         load: () => api.getDeployment(deploymentId),
         listensOn: const [],
       );
}

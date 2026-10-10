import 'package:fixnum/fixnum.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart'

    show DeploymentEvent, GetDeploymentRequest, LogTargetRequest;
import 'package:mobile/src/gen/openoxide/api/v1/common.pb.dart' show Empty;
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart' show WatchTablesEvent;
import 'package:mobile/src/core/api/panel_api.dart';

/// The server-push surface: log streams and the realtime table watch.
mixin LogStreamsSection on PanelApiBase {
  /// Ends when the panel closes the channel at a terminal deployment state;
  /// there is no end-of-log sentinel on the wire.
  Stream<DeploymentEvent> streamDeploymentLogs(int deploymentId) =>
      calls.serverStream(
        () => deploymentClient.streamDeploymentLogs(
          GetDeploymentRequest(id: Int64(deploymentId)),
        ),
      );

  Stream<DeploymentEvent> streamServiceLogs({
    required String target,
    int? serverId,
    int tail = 500,
    bool follow = true,
  }) => calls.serverStream(
    () => deploymentClient.streamServiceLogs(
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
  }) => calls.serverStream(
    () => deploymentClient.streamContainerLogs(
      LogTargetRequest(
        target: target,
        serverId: serverId == null ? null : Int64(serverId),
        tail: Int64(tail),
        follow: follow,
      ),
    ),
  );

  /// Ticks carry table names only; the first event is a full resync.
  Stream<WatchTablesEvent> watchTables() =>
      calls.serverStream(() => realtimeClient.watchTables(Empty()));
}

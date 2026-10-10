import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/log_view/log_view.dart';
import 'package:mobile/src/core/ui/widgets/status_notices.dart';
import 'package:mobile/src/features/deployments/bloc/deployments_bloc.dart';
import 'package:mobile/src/features/deployments/widgets/deployment_status_bar.dart';
import 'package:mobile/src/features/logs/bloc/log_feed_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart' hide Container;

/// One deployment's output, live.
///
/// The status row listens to the status bloc and the rest of the body listens
/// to the log feed: a feed that fails after the first build only repaints if
/// its banner is read inside the builder.
class DeploymentLogsScreen extends StatefulWidget {
  const DeploymentLogsScreen({
    super.key,
    required this.deploymentId,
    required this.title,
  });

  final int deploymentId;
  final String title;

  @override
  State<DeploymentLogsScreen> createState() => _DeploymentLogsScreenState();
}

class _DeploymentLogsScreenState extends State<DeploymentLogsScreen> {
  LogFeedBloc? _feed;
  DeploymentStatusBloc? _status;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_feed != null) return;
    final deps = context.read<Deps>();
    _feed = LogFeedBloc(
      open: () => deps.api.streamDeploymentLogs(widget.deploymentId),
    )..start();
    _status = DeploymentStatusBloc(
      api: deps.api,
      deploymentId: widget.deploymentId,
      realtime: deps.realtime,
    )..start();
  }

  @override
  void dispose() {
    _feed?.close();
    _status?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final feed = _feed;
    final status = _status;
    if (feed == null || status == null) {
      return const Scaffold(body: SizedBox.shrink());
    }
    return MultiBlocProvider(
      providers: [
        BlocProvider<LogFeedBloc>.value(value: feed),
        BlocProvider<DeploymentStatusBloc>.value(value: status),
      ],
      child: Scaffold(
        appBar: AppBar(
          title: Text(
            widget.title,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ),
        body: SafeArea(
          child: MultiBlocListener(
            listeners: [
              // The stream ending is the panel's way of saying the deployment
              // settled; re-read it for the final status.
              BlocListener<LogFeedBloc, LogFeedState>(
                listenWhen: (previous, current) =>
                    current.isDone && !previous.isDone,
                listener: (context, _) => status.refresh(),
              ),
            ],
            child: BlocBuilder<LogFeedBloc, LogFeedState>(
              builder: (context, feedState) => Column(
                children: [
                  BlocBuilder<DeploymentStatusBloc, QueryState<Deployment>>(
                    builder: (context, statusState) =>
                        DeploymentStatusBar(deployment: statusState.data),
                  ),
                  if (feedState.error != null)
                    Padding(
                      padding: const EdgeInsets.only(
                        left: Spacing.gutter,
                        top: Spacing.sm,
                        right: Spacing.gutter,
                      ),
                      child: ErrorNotice(
                        error: feedState.error!,
                        onRetry: feed.restart,
                      ),
                    ),
                  Expanded(
                    child: LogView(
                      entries: feedState.entries,
                      droppedLines: feedState.droppedLines,
                      emptyHint: feedState.isRunning
                          ? 'Waiting for output…'
                          : 'No output for this deployment',
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

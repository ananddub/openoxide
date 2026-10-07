import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/deployment.pb.dart' hide Container;
import '../state/deployment_status.dart';
import '../state/log_feed.dart';
import '../state/query.dart';
import '../state/deps.dart';
import 'common.dart';
import 'log_view.dart';
import 'theme.dart';
import 'tokens.dart';

/// One deployment's output, live.
///
/// The panel closes this stream by itself once the deployment settles, and there
/// is no "that is everything" message on the wire — so the end of the stream is
/// treated as a hint to re-read the deployment for its final status rather than
/// as a completion the app can trust on its own.
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
  late final LogFeed _feed;
  late final Query<Deployment> _status;

  @override
  void initState() {
    super.initState();
    final deps = context.read<Deps>();
    final id = widget.deploymentId;
    _status = Query<Deployment>(
      load: () => deps.api.getDeployment(id),
      realtime: deps.realtime,
      listensOn: const [],
    )..start();
    _feed = LogFeed(
      open: () => deps.api.streamDeploymentLogs(id),
      onEnded: _refetchStatus,
    )..start();
  }

  void _refetchStatus() {
    if (mounted) _status.refresh();
  }

  @override
  void dispose() {
    _feed.dispose();
    _status.dispose();
    super.dispose();
  }

  Future<void> _restart() async {
    await _feed.stop();
    _feed.buffer.clear();
    setState(() {});
    await _feed.start();
    _status.refresh();
  }

  @override
  Widget build(BuildContext context) {
    final brightness = Theme.of(context).brightness;
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title, maxLines: 1, overflow: TextOverflow.ellipsis),
      ),
      body: SafeArea(
        // The status bar listens to the query and the rest of the body listens
        // to the stream: a feed that fails after the first build only repaints
        // if its banner is read inside the builder.
        child: ListenableBuilder(
          listenable: _feed,
          builder: (context, _) => Column(
            children: [
              ListenableBuilder(
                listenable: _status,
                builder: (context, _) => _StatusBar(
                  deployment: _status.data,
                  brightness: brightness,
                ),
              ),
              if (_feed.error != null)
                Padding(
                  padding: const EdgeInsets.only(
                    left: Spacing.gutter,
                    top: Spacing.sm,
                    right: Spacing.gutter,
                  ),
                  child: ErrorNotice(error: _feed.error!, onRetry: _restart),
                ),
              Expanded(
                child: LogView(
                  entries: _feed.buffer.entries,
                  droppedLines: _feed.buffer.droppedLines,
                  emptyHint: _feed.isRunning
                      ? 'Waiting for output…'
                      : 'No output for this deployment',
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _StatusBar extends StatelessWidget {
  const _StatusBar({required this.deployment, required this.brightness});

  final Deployment? deployment;
  final Brightness brightness;

  @override
  Widget build(BuildContext context) {
    final text = Theme.of(context).textTheme;
    final item = deployment;
    final status = item?.status ?? '';
    final phase = deploymentPhase(status);
    final colors = phaseColors(phase, brightness);
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(
        Spacing.md,
        Spacing.sm,
        Spacing.md,
        Spacing.md,
      ),
      color: context.ox.card,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.circle, size: Mark.dot, color: colors.dot),
              const SizedBox(width: Spacing.sm),
              Expanded(
                child: Text(
                  status.isEmpty ? 'unknown' : status,
                  style: text.bodySmall?.copyWith(color: colors.text),
                ),
              ),
              if (item != null && item.resourceType.isNotEmpty)
                Flexible(
                  child: Text(
                    item.resourceType,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: text.labelSmall,
                  ),
                ),
            ],
          ),
          if (item != null && item.error.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(top: Spacing.xs),
              child: Text(
                item.error,
                style: text.labelSmall?.copyWith(color: colors.text),
              ),
            ),
        ],
      ),
    );
  }
}

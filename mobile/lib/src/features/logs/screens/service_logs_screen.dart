import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/log_view/log_view.dart';
import 'package:mobile/src/core/ui/widgets/status_notices.dart';
import 'package:mobile/src/features/logs/bloc/log_feed_bloc.dart';

/// Whether the pane follows a swarm service or one container's logs.
enum LogsMode { service, container }

/// Running output for one service or container.
///
/// The panel takes a docker target name for both RPCs, so the only difference
/// between the two modes is what the panel asks docker for — `docker service
/// logs` versus `docker logs`. A deploy that has not finished yet usually has
/// nothing in the second mode, which is why the switch is here instead of
/// guessed.
class ServiceLogsScreen extends StatefulWidget {
  const ServiceLogsScreen({
    super.key,
    required this.title,
    required this.target,
    this.serverId,
  });

  final String title;
  final String target;
  final int? serverId;

  @override
  State<ServiceLogsScreen> createState() => _ServiceLogsScreenState();
}

class _ServiceLogsScreenState extends State<ServiceLogsScreen> {
  LogsMode _mode = LogsMode.service;
  LogFeedBloc? _feed;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_feed != null) return;
    _feed = _openFeed()..start();
  }

  LogFeedBloc _openFeed() {
    final deps = context.read<Deps>();
    final service = _mode == LogsMode.service;
    return LogFeedBloc(
      open: () => service
          ? deps.api.streamServiceLogs(
              target: widget.target,
              serverId: widget.serverId,
            )
          : deps.api.streamContainerLogs(
              target: widget.target,
              serverId: widget.serverId,
            ),
    );
  }

  void _setMode(LogsMode mode) {
    if (mode == _mode) return;
    setState(() {
      _mode = mode;
      _feed?.close();
      _feed = _openFeed()..start();
    });
  }

  @override
  void dispose() {
    _feed?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final feed = _feed;
    if (feed == null) return const Scaffold(body: SizedBox.shrink());
    return BlocProvider<LogFeedBloc>.value(
      value: feed,
      child: Scaffold(
        appBar: AppBar(
          title: Text(
            widget.title,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ),
        body: SafeArea(
          // The header listens too: a stream the panel refused to open fails
          // after this screen's first build, so a banner read outside this
          // builder would stay invisible for the rest of the visit.
          child: BlocBuilder<LogFeedBloc, LogFeedState>(
            builder: (context, state) => Column(
              children: [
                _Header(
                  target: widget.target,
                  mode: _mode,
                  onModeChanged: _setMode,
                ),
                if (state.error != null)
                  Padding(
                    padding: const EdgeInsets.only(
                      left: Spacing.gutter,
                      top: Spacing.sm,
                      right: Spacing.gutter,
                    ),
                    child: ErrorNotice(
                      error: state.error!,
                      onRetry: () => setState(() {
                        _feed?.close();
                        _feed = _openFeed()..start();
                      }),
                    ),
                  ),
                Expanded(
                  child: LogView(
                    entries: state.entries,
                    droppedLines: state.droppedLines,
                    emptyHint: state.isRunning
                        ? 'Waiting for output from ${widget.target}…'
                        : 'No output',
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// The mode chips and the target they apply to.
///
/// A `Wrap` rather than a `Row`: the two chips are the widest controls in the
/// app and the target name is a generated string, so any row that makes them
/// share a line eventually spills off a narrow handset. Stacking costs a line,
/// never a pixel.
class _Header extends StatelessWidget {
  const _Header({
    required this.target,
    required this.mode,
    required this.onModeChanged,
  });

  final String target;
  final LogsMode mode;
  final void Function(LogsMode) onModeChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(
        Spacing.gutter,
        Spacing.sm,
        Spacing.gutter,
        Spacing.xs,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Wrap(
            spacing: Spacing.sm,
            runSpacing: Spacing.sm,
            children: [
              for (final mode in LogsMode.values)
                ChoiceChip(
                  label: Text(
                    mode == LogsMode.service ? 'Service' : 'Container',
                  ),
                  selected: mode == this.mode,
                  onSelected: (_) => onModeChanged(mode),
                ),
            ],
          ),
          const SizedBox(height: Spacing.xs),
          Text(
            target,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: Theme.of(context).textTheme.labelSmall,
          ),
        ],
      ),
    );
  }
}

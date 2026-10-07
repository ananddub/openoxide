import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../state/deps.dart';
import '../state/log_feed.dart';
import 'common.dart';
import 'log_view.dart';
import 'tokens.dart';

/// Running output for one service or container.
///
/// The panel takes a docker target name for both RPCs, so the only difference
/// between the two modes is what the panel asks docker for — `docker service logs`
/// versus `docker logs`. A deploy that has not finished yet usually has nothing
/// in the second mode, which is why the switch is here instead of guessed.
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

enum _Mode { service, container }

class _ServiceLogsScreenState extends State<ServiceLogsScreen> {
  _Mode _mode = _Mode.service;
  late LogFeed _feed;

  @override
  void initState() {
    super.initState();
    _feed = _openFeed()..start();
  }

  LogFeed _openFeed() {
    final deps = context.read<Deps>();
    final service = _mode == _Mode.service;
    return LogFeed(
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

  void _reopen() {
    _feed.dispose();
    _feed = _openFeed()..start();
  }

  void _setMode(_Mode mode) {
    if (mode == _mode) return;
    setState(() {
      _mode = mode;
      _reopen();
    });
  }

  @override
  void dispose() {
    _feed.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final feed = _feed;
    return Scaffold(
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
        child: ListenableBuilder(
          listenable: feed,
          builder: (context, _) => Column(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(
                  Spacing.gutter,
                  Spacing.sm,
                  Spacing.gutter,
                  Spacing.xs,
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // A `Wrap` rather than a `Row`: the two chips are the widest
                    // controls in the app and the target name is a generated
                    // string, so any row that makes them share a line eventually
                    // spills off a narrow handset. Stacking costs a line, never
                    // a pixel.
                    Wrap(
                      spacing: Spacing.sm,
                      runSpacing: Spacing.sm,
                      children: [
                        for (final mode in _Mode.values)
                          ChoiceChip(
                            label: Text(
                              mode == _Mode.service ? 'Service' : 'Container',
                            ),
                            selected: mode == _mode,
                            onSelected: (_) => _setMode(mode),
                          ),
                      ],
                    ),
                    const SizedBox(height: Spacing.xs),
                    Text(
                      widget.target,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: Theme.of(context).textTheme.labelSmall,
                    ),
                  ],
                ),
              ),
              if (feed.error != null)
                Padding(
                  padding: const EdgeInsets.only(
                    left: Spacing.gutter,
                    top: Spacing.sm,
                    right: Spacing.gutter,
                  ),
                  child: ErrorNotice(
                    error: feed.error!,
                    onRetry: () => setState(_reopen),
                  ),
                ),
              Expanded(
                child: LogView(
                  entries: feed.buffer.entries,
                  droppedLines: feed.buffer.droppedLines,
                  emptyHint: feed.isRunning
                      ? 'Waiting for output from ${widget.target}…'
                      : 'No output',
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

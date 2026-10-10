import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/section_label.dart';
import 'package:mobile/src/core/ui/widgets/section_scaffold.dart';
import 'package:mobile/src/features/deployments/bloc/deployments_bloc.dart';
import 'package:mobile/src/features/deployments/widgets/deployment_tile.dart';

/// The deploy queue: what is running now, then the recent history.
class DeploymentsScreen extends StatefulWidget {
  const DeploymentsScreen({super.key});

  @override
  State<DeploymentsScreen> createState() => _DeploymentsScreenState();
}

class _DeploymentsScreenState extends State<DeploymentsScreen> {
  DeploymentsBloc? _bloc;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_bloc != null) return;
    final deps = context.read<Deps>();
    _bloc = DeploymentsBloc(api: deps.api, realtime: deps.realtime)..start();
  }

  @override
  void dispose() {
    _bloc?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final bloc = _bloc;
    if (bloc == null) return const SizedBox.shrink();
    return BlocProvider<DeploymentsBloc>.value(
      value: bloc,
      child: BlocBuilder<DeploymentsBloc, QueryState<DeploymentQueue>>(
        builder: (context, state) {
          final rows = state.data;
          return SectionScaffold(
            onRefresh: () async => bloc.refresh(),
            loading: rows == null && state.loading,
            error: rows == null ? state.error : null,
            onRetry: () async => bloc.refresh(),
            emptyTitle: 'Nothing has been deployed here yet',
            emptySubtitle: 'Deploys appear as soon as the panel starts one.',
            children: rows == null ? const [] : _tiles(rows),
          );
        },
      ),
    );
  }

  /// Two groups, each entering top-down. The gap between them is the next
  /// heading's own top inset, so it is not a spacer that has to agree with one.
  static List<Widget> _tiles(DeploymentQueue rows) {
    final tiles = <Widget>[];
    if (rows.active.isNotEmpty) {
      tiles.add(const SectionLabel('In progress', first: true));
      for (var index = 0; index < rows.active.length; index++) {
        tiles.add(
          DeploymentTile(
            deployment: rows.active[index],
            emphasize: true,
            delay: rowDelay(index),
          ),
        );
      }
    }
    if (rows.recent.isNotEmpty) {
      tiles.add(SectionLabel('Recent', first: rows.active.isEmpty));
      for (var index = 0; index < rows.recent.length; index++) {
        tiles.add(
          DeploymentTile(
            deployment: rows.recent[index],
            emphasize: false,
            delay: rowDelay(index),
          ),
        );
      }
    }
    return tiles;
  }
}

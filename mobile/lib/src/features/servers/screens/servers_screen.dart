import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/core/ui/widgets/section_scaffold.dart';
import 'package:mobile/src/features/servers/bloc/servers_health_bloc.dart';
import 'package:mobile/src/features/servers/widgets/server_card.dart';

/// Server health, one card per server.
class ServersScreen extends StatefulWidget {
  const ServersScreen({super.key});

  @override
  State<ServersScreen> createState() => _ServersScreenState();
}

class _ServersScreenState extends State<ServersScreen> {
  ServersHealthBloc? _bloc;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_bloc != null) return;
    final deps = context.read<Deps>();
    _bloc = ServersHealthBloc(api: deps.api, realtime: deps.realtime)..start();
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
    return BlocProvider<ServersHealthBloc>.value(
      value: bloc,
      child: BlocBuilder<ServersHealthBloc, QueryState<List<ServerHealth>>>(
        builder: (context, state) {
          final servers = state.data;
          return SectionScaffold(
            onRefresh: () async => bloc.refresh(),
            loading: servers == null && state.loading,
            error: servers == null ? state.error : null,
            onRetry: () async => bloc.refresh(),
            emptyTitle: 'No servers connected',
            emptySubtitle:
                'Add a server in the web dashboard to monitor it here.',
            children: [
              if (servers != null)
                for (var index = 0; index < servers.length; index++)
                  ServerCard(health: servers[index], delay: rowDelay(index)),
            ],
          );
        },
      ),
    );
  }
}

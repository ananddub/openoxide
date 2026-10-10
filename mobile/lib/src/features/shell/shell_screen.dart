import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/features/deployments/screens/deployments_screen.dart';
import 'package:mobile/src/features/projects/screens/projects_screen.dart';
import 'package:mobile/src/features/servers/screens/servers_screen.dart';
import 'package:mobile/src/features/shell/widgets/organization_menu.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';

/// The signed-in container: three tabs, one organization, one place to leave.
///
/// The tabs live in an [IndexedStack] rather than being rebuilt on each switch,
/// because a screen with a live log stream or a loaded list should keep what it
/// has when the reader moves away and back.
class ShellScreen extends StatefulWidget {
  const ShellScreen({super.key});

  @override
  State<ShellScreen> createState() => _ShellScreenState();
}

class _ShellScreenState extends State<ShellScreen> {
  int _tab = 0;

  static const _titles = ['Projects', 'Deployments', 'Servers'];

  @override
  Widget build(BuildContext context) {
    final organizations = context.select<AuthBloc, List<Organization>>(
      (bloc) => bloc.state.organizations,
    );
    final selected = context.select<AuthBloc, Organization?>(
      (bloc) => bloc.currentOrganization,
    );
    final hasOrganization = context.select<AuthBloc, bool>(
      (bloc) => bloc.hasOrganization,
    );

    return Scaffold(
      appBar: AppBar(
        title: Text(_titles[_tab]),
        actions: [
          OrganizationMenu(
            organizations: organizations,
            selected: selected,
            onSelect: (id) =>
                context.read<AuthBloc>().add(AuthOrganizationSelected(id)),
          ),
        ],
      ),
      body: SafeArea(
        child: !hasOrganization
            ? const NoOrganization()
            : IndexedStack(
                index: _tab,
                children: [
                  ProjectsScreen(key: const ValueKey('projects')),
                  DeploymentsScreen(key: const ValueKey('deployments')),
                  ServersScreen(key: const ValueKey('health')),
                ],
              ),
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: _tab,
        onDestinationSelected: (index) => setState(() => _tab = index),
        destinations: [
          NavigationDestination(
            icon: const Icon(Icons.folder_outlined),
            selectedIcon: const Icon(Icons.folder),
            label: 'Projects',
          ),
          NavigationDestination(
            icon: const Icon(Icons.rocket_launch_outlined),
            selectedIcon: const Icon(Icons.rocket_launch),
            label: 'Deploys',
          ),
          NavigationDestination(
            icon: const Icon(Icons.monitor_heart_outlined),
            selectedIcon: const Icon(Icons.monitor_heart),
            label: 'Servers',
          ),
        ],
      ),
      // Default size, not `.small`: the small variant measures 40dp, under the
      // same minimum touch target every other control in the app is held to.
      floatingActionButton: FloatingActionButton(
        heroTag: 'sign-out',
        tooltip: 'Sign out',
        onPressed: () =>
            context.read<AuthBloc>().add(const AuthSignOutRequested()),
        child: const Icon(Icons.logout),
      ),
    );
  }
}

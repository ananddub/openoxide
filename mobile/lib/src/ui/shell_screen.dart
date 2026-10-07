import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../gen/openoxide/api/v1/organization.pb.dart';
import '../state/auth_controller.dart';
import 'common.dart';
import 'deployments_screen.dart';
import 'health_screen.dart';
import 'projects_screen.dart';
import 'tokens.dart';

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
    final controller = context.watch<AuthController>();
    final organization = controller.currentOrganization;

    return Scaffold(
      appBar: AppBar(
        title: Text(_titles[_tab]),
        actions: [
          _OrganizationMenu(
            organizations: controller.organizations,
            selected: organization,
            onSelect: controller.selectOrganization,
          ),
        ],
      ),
      body: SafeArea(
        child: controller.organizationId == null
            ? const _NoOrganization()
            : IndexedStack(
                index: _tab,
                children: [
                  ProjectsScreen(key: const ValueKey('projects')),
                  DeploymentsScreen(key: const ValueKey('deployments')),
                  HealthScreen(key: const ValueKey('health')),
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
        onPressed: () => context.read<AuthController>().signOut(),
        child: const Icon(Icons.logout),
      ),
    );
  }
}

class _NoOrganization extends StatelessWidget {
  const _NoOrganization();

  @override
  Widget build(BuildContext context) {
    final controller = context.read<AuthController>();
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(Spacing.xl),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              'You are not a member of any organization on this panel yet.',
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            const SizedBox(height: Spacing.lg),
            ActionButton(
              label: 'Reload',
              tone: ButtonTone.outline,
              onPressed: controller.loadOrganizations,
            ),
          ],
        ),
      ),
    );
  }
}

class _OrganizationMenu extends StatelessWidget {
  const _OrganizationMenu({
    required this.organizations,
    required this.selected,
    required this.onSelect,
  });

  final List<Organization> organizations;
  final Organization? selected;
  final Future<void> Function(int) onSelect;

  String _label(Organization organization) {
    if (organization.name.isNotEmpty) return organization.name;
    if (organization.slug.isNotEmpty) return organization.slug;
    return 'Organization ${organization.id}';
  }

  @override
  Widget build(BuildContext context) {
    if (organizations.isEmpty) return const SizedBox.shrink();
    return PopupMenuButton<int>(
      tooltip: 'Organization',
      icon: const Icon(Icons.apartment_outlined),
      itemBuilder: (context) => [
        for (final organization in organizations)
          PopupMenuItem<int>(
            value: organization.id.toInt(),
            child: Row(
              children: [
                Icon(
                  selected?.id == organization.id
                      ? Icons.check
                      : Icons.circle_outlined,
                  size: 16,
                ),
                const SizedBox(width: Spacing.md),
                Expanded(child: Text(_label(organization))),
              ],
            ),
          ),
      ],
      onSelected: onSelect,
    );
  }
}

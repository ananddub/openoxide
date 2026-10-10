import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/core/ui/widgets/action_button.dart';
import 'package:mobile/src/features/auth/bloc/auth_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';

/// The organization switcher in the app bar.
class OrganizationMenu extends StatelessWidget {
  const OrganizationMenu({
    super.key,
    required this.organizations,
    required this.selected,
    required this.onSelect,
  });

  final List<Organization> organizations;
  final Organization? selected;
  final void Function(int) onSelect;

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

/// Shown when the signed-in member belongs to no organization on this panel.
class NoOrganization extends StatelessWidget {
  const NoOrganization({super.key});

  @override
  Widget build(BuildContext context) {
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
              onPressed: () => context.read<AuthBloc>().add(
                const AuthOrganizationsReloaded(),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

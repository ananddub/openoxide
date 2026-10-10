import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/ui/widgets/wire_label.dart';
import 'package:mobile/src/features/logs/screens/service_logs_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/overview.pb.dart';

/// One service of the project, opening its live logs.
class ServiceTile extends StatelessWidget {
  const ServiceTile({
    super.key,
    required this.service,
    this.delay = Duration.zero,
  });

  final OverviewServiceItem service;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final name = service.name.isEmpty ? 'Service ${service.id}' : service.name;
    return Card(
          child: ListTile(
            title: Text(name, style: Theme.of(context).textTheme.titleMedium),
            subtitle: Text(
              [
                if (service.serviceType.name.isNotEmpty)
                  wireLabel(service.serviceType.name, 'OVERVIEW_SERVICE_TYPE_'),
                if (service.status.isNotEmpty) service.status,
                if (service.serverName.isNotEmpty) service.serverName,
                if (service.dbKind.isNotEmpty) service.dbKind,
              ].join('  ·  '),
              style: Theme.of(context).textTheme.bodySmall,
            ),
            trailing: const Icon(Icons.description_outlined),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => ServiceLogsScreen(
                  title: name,
                  target: name,
                  serverId: service.serverId.toInt() == 0
                      ? null
                      : service.serverId.toInt(),
                ),
              ),
            ),
          ),
        )
        .animate(delay: delay)
        .fadeIn(duration: Motion.fast)
        .slideY(begin: Motion.liftRow);
  }
}

/// One domain row, showing its address in a snackbar on tap.
class DomainTile extends StatelessWidget {
  const DomainTile({
    super.key,
    required this.domain,
    this.delay = Duration.zero,
  });

  final OverviewDomainItem domain;
  final Duration delay;

  @override
  Widget build(BuildContext context) {
    final url =
        '${domain.https ? 'https' : 'http'}://'
        '${domain.host}${domain.path.isEmpty ? '' : domain.path}';
    return Card(
          child: ListTile(
            title: Text(domain.host),
            subtitle: Text(
              [
                if (domain.domainType.isNotEmpty) domain.domainType,
                if (domain.serviceName.isNotEmpty) domain.serviceName,
                if (domain.port != 0) ':${domain.port}',
              ].join('  ·  '),
              style: Theme.of(context).textTheme.bodySmall,
            ),
            trailing: Icon(
              domain.https ? Icons.lock_outline : Icons.lock_open_outlined,
            ),
            onTap: () => ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(url), duration: Motion.notice),
            ),
          ),
        )
        .animate(delay: delay)
        .fadeIn(duration: Motion.fast)
        .slideY(begin: Motion.liftRow);
  }
}

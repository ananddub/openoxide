import 'package:flutter/material.dart';
import 'package:mobile/src/core/config/app_config.dart';

/// The panel address in use. Shown wherever a connection failure appears,
/// because the most common cause is pointing the app at the wrong host.
class PanelTargetLabel extends StatelessWidget {
  const PanelTargetLabel({super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      'https://${AppConfig.panelHost}:${AppConfig.panelPort}',
      style: Theme.of(context).textTheme.labelSmall,
    );
  }
}

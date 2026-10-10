import 'package:flutter/material.dart';
import 'package:mobile/src/core/theme/openoxide_colors.dart';
import 'package:mobile/src/core/tokens/mark.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/core/tokens/radii.dart';
import 'package:mobile/src/core/tokens/spacing.dart';
import 'package:mobile/src/gen/openoxide/api/v1/monitoring.pb.dart';

/// CPU, memory and disk bars for one server.
class MetricGauges extends StatelessWidget {
  const MetricGauges({super.key, required this.metric});

  final MonitoringServerMetric metric;

  @override
  Widget build(BuildContext context) {
    // The agent sends `mem_used` and `disk_used` as percentages and the totals
    // in GB — the same reading the web's overview tab takes. Dividing a percent
    // by a GB total fills the bar at any size of machine.
    final memPct = metric.memUsed.clamp(0.0, 100.0);
    final diskPct = metric.diskUsed.clamp(0.0, 100.0);
    final diskGb = metric.totalDisk * diskPct / 100;
    return Column(
      children: [
        GaugeRow(
          label: 'CPU',
          value: '${metric.cpu.clamp(0.0, 100.0).toStringAsFixed(0)}%',
          fraction: metric.cpu / 100,
        ),
        GaugeRow(
          label: 'Memory',
          value: '${metric.memUsedGb.toStringAsFixed(1)} GB',
          fraction: memPct / 100,
          detail: '${metric.memTotal.toStringAsFixed(1)} GB total',
        ),
        GaugeRow(
          label: 'Disk',
          value: '${diskGb.toStringAsFixed(0)} GB',
          fraction: diskPct / 100,
          detail: '${metric.totalDisk.toStringAsFixed(0)} GB total',
        ),
      ],
    );
  }
}

/// One labelled progress bar, animating to its new value on refresh.
class GaugeRow extends StatelessWidget {
  const GaugeRow({
    super.key,
    required this.label,
    required this.value,
    required this.fraction,
    this.detail,
  });

  final String label;
  final String value;
  final double fraction;
  final String? detail;

  @override
  Widget build(BuildContext context) {
    final clamped = fraction.clamp(0.0, 1.0);
    final text = Theme.of(context).textTheme;
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: Spacing.xs),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              SizedBox(width: 60, child: Text(label, style: text.labelMedium)),
              Text(value, style: text.labelMedium),
              if (detail != null) ...[
                const SizedBox(width: Spacing.xs),
                Expanded(
                  child: Text(
                    detail!,
                    style: text.labelSmall,
                    textAlign: TextAlign.end,
                  ),
                ),
              ],
            ],
          ),
          const SizedBox(height: Spacing.xs),
          // A metric refresh moves the bar; sliding to the new value keeps the
          // number and the fill readable as one change.
          TweenAnimationBuilder<double>(
            tween: Tween(begin: 0, end: clamped),
            duration: Motion.base,
            curve: Motion.move,
            builder: (context, value, _) => ClipRRect(
              borderRadius: BorderRadius.circular(Radii.dot),
              child: LinearProgressIndicator(
                value: value,
                minHeight: Mark.bar,
                backgroundColor: context.ox.subtleFill,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:mobile/src/core/domain/deployment_status.dart';

/// Status pill colours for a deployment, following the web's status tone rules.
({Color dot, Color text, bool pulse}) phaseColors(
  DeploymentPhase phase,
  Brightness brightness,
) {
  final isDark = brightness == Brightness.dark;
  Color pick({required int light, required int dark}) =>
      Color(isDark ? dark : light);
  return switch (phase) {
    DeploymentPhase.working => (
      dot: pick(light: 0xFFF59F0A, dark: 0xFFFBBF24),
      text: pick(light: 0xFFB45309, dark: 0xFFFCD34D),
      pulse: true,
    ),
    DeploymentPhase.queued => (
      dot: pick(light: 0xFF64748B, dark: 0xFF94A3B8),
      text: pick(light: 0xFF475569, dark: 0xFFCBD5E1),
      pulse: true,
    ),
    DeploymentPhase.succeeded => (
      dot: pick(light: 0xFF10B981, dark: 0xFF34D399),
      text: pick(light: 0xFF047857, dark: 0xFF6EE7B7),
      pulse: false,
    ),
    DeploymentPhase.failed => (
      dot: pick(light: 0xFFDC2626, dark: 0xFFF87171),
      text: pick(light: 0xFFB91C1C, dark: 0xFFFCA5A5),
      pulse: false,
    ),
    DeploymentPhase.cancelled => (
      dot: pick(light: 0xFF71717A, dark: 0xFFA1A1AA),
      text: pick(light: 0xFF52525B, dark: 0xFFD4D4D8),
      pulse: false,
    ),
    DeploymentPhase.unknown => (
      dot: pick(light: 0xFFA1A1AA, dark: 0xFF525252),
      text: pick(light: 0xFF71717A, dark: 0xFFA1A1AA),
      pulse: false,
    ),
  };
}

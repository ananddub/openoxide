/// Deployment status rules, ported from the panel's own two lists.
///
/// `web/openoxide_react/src/api/live/deployments.ts` and
/// `components/projects/detail/deployments/build-status.ts` disagree slightly —
/// one adds `STOPPED`/`CANCELED`, the other adds `CRASHED`. A deployment counted
/// as finished while the panel still works on it stops the log stream early, so
/// this app takes the union of the two for "final" and keeps the keyword list for
/// "active" exactly as the panel does.
library;

enum DeploymentPhase { queued, working, succeeded, failed, cancelled, unknown }

const Set<String> _finalStates = {
  'DONE',
  'DEPLOYED',
  'SUCCESS',
  'FAILED',
  'ERROR',
  'CANCELLED',
  'CANCELED',
  'STOPPED',
  'STOPPEDBYUSER',
  'CRASHED',
};

const List<String> _activeKeywords = [
  'RUNNING',
  'BUILDING',
  'PREPARING',
  'QUEUE',
  'STARTING',
  'DEPLOYING',
  'PENDING',
  'GIT',
  'DOCKER',
];

String _normalized(String status) => status.trim().toUpperCase();

/// True once the panel will not move this deployment again — the point at which
/// it also closes a log stream.
bool isFinalDeploymentStatus(String status) =>
    _finalStates.contains(_normalized(status));

/// True while a deployment is still building or deploying.
bool isActiveDeploymentStatus(String status) {
  final value = _normalized(status);
  if (value.isEmpty || _finalStates.contains(value)) return false;
  return _activeKeywords.any(value.contains);
}

bool isSucceededDeploymentStatus(String status) {
  final value = _normalized(status);
  return value.contains('DONE') ||
      value.contains('HEALTHY') ||
      value.contains('SUCCESS') ||
      value.contains('DEPLOYED');
}

bool isFailedDeploymentStatus(String status) {
  final value = _normalized(status);
  return value.contains('ERROR') ||
      value.contains('FAIL') ||
      value.contains('CRASH');
}

DeploymentPhase deploymentPhase(String status) {
  final value = _normalized(status);
  if (value.isEmpty) return DeploymentPhase.unknown;
  if (value == 'CANCELLING') return DeploymentPhase.cancelled;
  if (value == 'CANCELLED' || value == 'STOPPEDBYUSER' || value == 'STOPPED') {
    return DeploymentPhase.cancelled;
  }
  if (isFailedDeploymentStatus(value)) return DeploymentPhase.failed;
  if (isSucceededDeploymentStatus(value)) return DeploymentPhase.succeeded;
  if (isActiveDeploymentStatus(value)) {
    return value.contains('QUEUE') || value.contains('PENDING')
        ? DeploymentPhase.queued
        : DeploymentPhase.working;
  }
  return isFinalDeploymentStatus(value)
      ? DeploymentPhase.succeeded
      : DeploymentPhase.unknown;
}

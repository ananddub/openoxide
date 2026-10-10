import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/domain/deployment_status.dart';

void main() {
  group('isFinalDeploymentStatus', () {
    test('covers both of the panel lists', () {
      for (final status in [
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
      ]) {
        expect(isFinalDeploymentStatus(status), isTrue, reason: status);
      }
    });

    test('a deployment still working is not final', () {
      for (final status in ['BUILDING', 'DEPLOYING', 'QUEUED', 'PENDING']) {
        expect(isFinalDeploymentStatus(status), isFalse, reason: status);
      }
    });

    test('tolerates the casing and padding the panel varies', () {
      expect(isFinalDeploymentStatus(' done '), isTrue);
      expect(isFinalDeploymentStatus('StoppedByUser'), isTrue);
    });
  });

  group('isActiveDeploymentStatus', () {
    test('recognises the pipeline verbs', () {
      for (final status in [
        'RUNNING',
        'BUILDING',
        'PREPARING',
        'QUEUED',
        'STARTING',
        'DEPLOYING',
        'PENDING',
        'GIT_CLONE',
        'DOCKER_BUILD',
      ]) {
        expect(isActiveDeploymentStatus(status), isTrue, reason: status);
      }
    });

    test('a finished or blank status is never active', () {
      expect(isActiveDeploymentStatus('DONE'), isFalse);
      expect(isActiveDeploymentStatus('CRASHED'), isFalse);
      expect(isActiveDeploymentStatus(''), isFalse);
    });
  });

  group('deploymentPhase', () {
    test('splits queued from working', () {
      expect(deploymentPhase('QUEUED'), DeploymentPhase.queued);
      expect(deploymentPhase('PENDING'), DeploymentPhase.queued);
      expect(deploymentPhase('BUILDING'), DeploymentPhase.working);
      expect(deploymentPhase('DEPLOYING'), DeploymentPhase.working);
    });

    test('cancellations outrank their own wording', () {
      expect(deploymentPhase('CANCELLING'), DeploymentPhase.cancelled);
      expect(deploymentPhase('CANCELLED'), DeploymentPhase.cancelled);
      expect(deploymentPhase('STOPPEDBYUSER'), DeploymentPhase.cancelled);
    });

    test('failure beats success for a status that claims both', () {
      expect(deploymentPhase('FAILED'), DeploymentPhase.failed);
      expect(deploymentPhase('CRASHED'), DeploymentPhase.failed);
      expect(deploymentPhase('DONE'), DeploymentPhase.succeeded);
      expect(deploymentPhase('HEALTHY'), DeploymentPhase.succeeded);
    });

    test('nothing is invented for a status the app has not seen', () {
      expect(deploymentPhase(''), DeploymentPhase.unknown);
      expect(deploymentPhase('TELEPORTING'), DeploymentPhase.unknown);
    });
  });
}

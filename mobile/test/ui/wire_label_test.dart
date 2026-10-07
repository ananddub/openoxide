import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/ui/common.dart';

void main() {
  group('wireLabel', () {
    test('drops the enum prefix and reads as a label', () {
      expect(
        wireLabel('REMOTE_SERVER_STATUS_ACTIVE', 'REMOTE_SERVER_STATUS_'),
        'Active',
      );
      expect(
        wireLabel('OVERVIEW_SERVICE_TYPE_COMPOSE', 'OVERVIEW_SERVICE_TYPE_'),
        'Compose',
      );
    });

    test('joins a multi-word value', () {
      expect(
        wireLabel(
          'OVERVIEW_SERVICE_TYPE_MAIN_SERVER',
          'OVERVIEW_SERVICE_TYPE_',
        ),
        'Main Server',
      );
    });

    test('keeps a value the prefix does not explain', () {
      expect(wireLabel('docker', 'REMOTE_SERVER_STATUS_'), 'docker');
      expect(
        wireLabel('REMOTE_SERVER_STATUS_', 'REMOTE_SERVER_STATUS_'),
        'REMOTE_SERVER_STATUS_',
      );
    });
  });
}

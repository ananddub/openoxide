import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/service_api.dart' show MetadataProvider;
import 'package:mobile/src/grpc/metadata.dart';
import 'package:mobile/src/session/session.dart';

Session _session({int? organizationId}) => Session(
  vault: InMemorySessionVault(),
  initial: StoredSession(
    accessToken: 'access-token',
    refreshToken: 'refresh-token',
    organizationId: organizationId,
  ),
);

Map<String, String> _send(MetadataProvider provider) {
  final metadata = <String, String>{'user-agent': 'test'};
  provider(metadata, 'https://panel.test');
  return metadata;
}

void main() {
  group('authMetadataProvider', () {
    test('sends the bearer token and the organization together', () {
      final metadata = _send(authMetadataProvider(_session(organizationId: 7)));
      expect(metadata['authorization'], 'Bearer access-token');
      expect(metadata['x-organization-id'], '7');
      expect(metadata['user-agent'], 'test');
    });

    test('omits the organization header rather than sending it empty', () {
      final metadata = _send(authMetadataProvider(_session()));
      expect(metadata.containsKey('x-organization-id'), isFalse);
      expect(metadata['authorization'], 'Bearer access-token');
    });

    test('is read per call, so a refreshed token reaches the next RPC', () async {
      final session = _session(organizationId: 7);
      final provider = authMetadataProvider(session);
      expect(_send(provider)['authorization'], 'Bearer access-token');

      await session.update(
        (current) => current.copyWith(accessToken: 'rotated'),
      );
      expect(_send(provider)['authorization'], 'Bearer rotated');
    });

    test('sends nothing when there is no session at all', () {
      final session = Session(vault: InMemorySessionVault());
      expect(_send(authMetadataProvider(session)).containsKey('authorization'),
          isFalse);
    });
  });

  group('refreshMetadataProvider', () {
    test('sends no bearer token: the refresh token is in the body', () {
      final metadata = _send(refreshMetadataProvider(_session(organizationId: 7)));
      expect(metadata.containsKey('authorization'), isFalse);
      expect(metadata['x-organization-id'], '7');
    });
  });
}

import 'package:grpc/service_api.dart' show MetadataProvider;

import '../session/session.dart';

/// Attaches the bearer token and the active organization to every RPC — the same
/// pair of headers the web transport sends. Both are omitted rather than sent
/// empty when there is no token or no organization selected yet, and these
/// providers run per RPC, so a refreshed token is picked up by the next call.
MetadataProvider authMetadataProvider(Session session) =>
    (Map<String, String> metadata, String uri) {
      final token = session.accessToken;
      if (token.isNotEmpty) {
        metadata['authorization'] = 'Bearer $token';
      }
      final organizationId = session.organizationId;
      if (organizationId != null) {
        metadata['x-organization-id'] = '$organizationId';
      }
    };

/// Metadata for the token-refresh call itself, which authenticates with the
/// refresh token in the request body rather than an access token.
MetadataProvider refreshMetadataProvider(Session session) =>
    (Map<String, String> metadata, String uri) {
      final organizationId = session.organizationId;
      if (organizationId != null) {
        metadata['x-organization-id'] = '$organizationId';
      }
    };

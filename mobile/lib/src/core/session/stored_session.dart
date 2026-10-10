/// Everything the app needs to talk as the signed-in user.
class StoredSession {
  const StoredSession({
    required this.accessToken,
    required this.refreshToken,
    this.organizationId,
    this.userId,
    this.email,
    this.firstName,
    this.lastName,
    this.avatar,
  });

  final String accessToken;
  final String refreshToken;
  final int? organizationId;
  final int? userId;
  final String? email;
  final String? firstName;
  final String? lastName;
  final String? avatar;

  StoredSession copyWith({
    String? accessToken,
    String? refreshToken,
    int? organizationId,
    int? userId,
    String? email,
    String? firstName,
    String? lastName,
    String? avatar,
  }) => StoredSession(
    accessToken: accessToken ?? this.accessToken,
    refreshToken: refreshToken ?? this.refreshToken,
    organizationId: organizationId ?? this.organizationId,
    userId: userId ?? this.userId,
    email: email ?? this.email,
    firstName: firstName ?? this.firstName,
    lastName: lastName ?? this.lastName,
    avatar: avatar ?? this.avatar,
  );

  /// Setting an organization needs its own method: `copyWith` cannot tell
  /// "leave it alone" from "clear it", and the switcher does both.
  StoredSession withOrganization(int? organizationId) => StoredSession(
    accessToken: accessToken,
    refreshToken: refreshToken,
    organizationId: organizationId,
    userId: userId,
    email: email,
    firstName: firstName,
    lastName: lastName,
    avatar: avatar,
  );

  Map<String, String> toMap() => {
    'accessToken': accessToken,
    'refreshToken': refreshToken,
    if (organizationId != null) 'organizationId': '$organizationId',
    if (userId != null) 'userId': '$userId',
    'email': ?email,
    'firstName': ?firstName,
    'lastName': ?lastName,
    'avatar': ?avatar,
  };

  static StoredSession? fromMap(Map<String, String>? map) {
    final access = map?[keyAccessToken] ?? '';
    final refresh = map?[keyRefreshToken] ?? '';
    if (access.isEmpty || refresh.isEmpty) return null;
    return StoredSession(
      accessToken: access,
      refreshToken: refresh,
      organizationId: int.tryParse(map?[keyOrganizationId] ?? ''),
      userId: int.tryParse(map?[keyUserId] ?? ''),
      email: map?[keyEmail],
      firstName: map?[keyFirstName],
      lastName: map?[keyLastName],
      avatar: map?[keyAvatar],
    );
  }

  static const keyAccessToken = 'accessToken';
  static const keyRefreshToken = 'refreshToken';
  static const keyOrganizationId = 'organizationId';
  static const keyUserId = 'userId';
  static const keyEmail = 'email';
  static const keyFirstName = 'firstName';
  static const keyLastName = 'lastName';
  static const keyAvatar = 'avatar';
}

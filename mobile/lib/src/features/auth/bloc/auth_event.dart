/// What the UI may ask of the sign-in state. Every change goes through one of
/// these events; nothing mutates the session from a screen directly.
sealed class AuthEvent {
  const AuthEvent();
}

/// One round trip to the public `GetSetupStatus` decides everything: whether
/// the panel is there at all, whether it is usable yet, and whether a stored
/// session is worth keeping.
class AuthBootRequested extends AuthEvent {
  const AuthBootRequested();
}

class AuthSignInRequested extends AuthEvent {
  const AuthSignInRequested({
    required this.email,
    required this.password,
    this.twoFactorCode,
    this.recoveryCode,
  });

  final String email;
  final String password;
  final String? twoFactorCode;
  final String? recoveryCode;
}

/// Refetch the organization list — also the "Reload" action on a screen whose
/// member has no organization yet.
class AuthOrganizationsReloaded extends AuthEvent {
  const AuthOrganizationsReloaded();
}

class AuthOrganizationSelected extends AuthEvent {
  const AuthOrganizationSelected(this.organizationId);

  final int organizationId;
}

class AuthSignOutRequested extends AuthEvent {
  const AuthSignOutRequested();
}

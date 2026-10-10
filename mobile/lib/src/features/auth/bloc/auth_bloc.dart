export 'package:mobile/src/features/auth/bloc/app_stage.dart';
export 'package:mobile/src/features/auth/bloc/auth_event.dart';
export 'package:mobile/src/features/auth/bloc/auth_state.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/api/panel_api.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/session/session.dart';
import 'package:mobile/src/features/auth/bloc/app_stage.dart';
import 'package:mobile/src/features/auth/bloc/auth_event.dart';
import 'package:mobile/src/features/auth/bloc/auth_state.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';

/// Owns the sign-in state and the selected organization, as a bloc.
class AuthBloc extends Bloc<AuthEvent, AuthState> {
  AuthBloc({required this.session, required this.api})
    : super(AuthState(organizationId: session.organizationId)) {
    on<AuthBootRequested>(_onBoot);
    on<AuthSignInRequested>(_onSignIn);
    on<AuthOrganizationsReloaded>(_onOrganizationsReloaded);
    on<AuthOrganizationSelected>(_onOrganizationSelected);
    on<AuthSignOutRequested>(_onSignOut);
  }

  final Session session;
  final PanelApi api;

  bool get hasOrganization => (session.organizationId ?? 0) > 0;
  int? get organizationId => hasOrganization ? session.organizationId : null;

  Organization? get currentOrganization {
    final id = organizationId;
    if (id == null) return null;
    for (final organization in state.organizations) {
      if (organization.id.toInt() == id) return organization;
    }
    return null;
  }

  /// The state carried between the steps of one boot or sign-in, stamped with
  /// whatever the session says now.
  AuthState _carried() => AuthState(
    stage: state.stage,
    error: state.error,
    organizations: state.organizations,
    busy: state.busy,
    organizationId: session.organizationId,
  );

  Future<void> _onBoot(AuthBootRequested event, Emitter<AuthState> emit) async {
    emit(AuthState(organizationId: session.organizationId));
    try {
      final status = await api.getSetupStatus();
      if (!status.ownerPresent) {
        emit(const AuthState(stage: AppStage.needsSetup));
      } else if (!session.isSignedIn) {
        emit(const AuthState(stage: AppStage.signedOut));
      } else {
        emit(
          AuthState(
            stage: AppStage.signedIn,
            organizationId: session.organizationId,
          ),
        );
      }
      // A stored session may predate the organization switcher, and every screen
      // lists resources inside an organization, so resolve one before the shell
      // makes its first list call.
      if (state.stage == AppStage.signedIn) {
        await _loadOrganizations(emit);
      }
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.unreachable, emit: emit);
    }
  }

  Future<void> _onSignIn(
    AuthSignInRequested event,
    Emitter<AuthState> emit,
  ) async {
    if (state.busy) return;
    emit(state.copyWith(busy: true, error: null, organizationId: null));
    try {
      final stored = await api.signIn(
        email: event.email,
        password: event.password,
        twoFactorCode: event.twoFactorCode,
        recoveryCode: event.recoveryCode,
      );
      await session.save(stored);
      emit(_carried().copyWith(stage: AppStage.signedIn, busy: false));
      await _loadOrganizations(emit);
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.signedOut, emit: emit);
      emit(state.copyWith(busy: false));
    }
  }

  Future<void> _onOrganizationsReloaded(
    AuthOrganizationsReloaded event,
    Emitter<AuthState> emit,
  ) async {
    try {
      await _loadOrganizations(emit);
    } on Object catch (cause) {
      await _report(cause, fallback: AppStage.signedOut, emit: emit);
    }
  }

  /// Loads the organizations and falls back to the first one when the stored
  /// selection is no longer valid.
  Future<void> _loadOrganizations(Emitter<AuthState> emit) async {
    final response = await api.listOrganizations();
    final organizations = List<Organization>.unmodifiable(
      response.organizations,
    );
    emit(state.copyWith(organizations: organizations));
    final selected = organizationId;
    final stillValid =
        selected != null && organizations.any((o) => o.id.toInt() == selected);
    if (!stillValid && organizations.isNotEmpty) {
      await _selectOrganization(organizations.first.id.toInt(), emit);
    }
  }

  Future<void> _onOrganizationSelected(
    AuthOrganizationSelected event,
    Emitter<AuthState> emit,
  ) => _selectOrganization(event.organizationId, emit);

  Future<void> _selectOrganization(
    int organizationId,
    Emitter<AuthState> emit,
  ) async {
    final current = session.current;
    if (current == null) return;
    await session.update((stored) => stored.withOrganization(organizationId));
    emit(state.copyWith(organizationId: organizationId));
  }

  Future<void> _onSignOut(
    AuthSignOutRequested event,
    Emitter<AuthState> emit,
  ) async {
    // Best effort: the server-side session is nice to drop, but a phone that
    // cannot reach the panel still has to be able to forget its tokens.
    try {
      await api.logout();
    } on Object {
      // Ignored on purpose.
    }
    await session.signOut();
    emit(const AuthState(stage: AppStage.signedOut));
  }

  /// Records a failure and lands on the stage that still shows something
  /// useful. A rejected session is the one case that changes the tokens
  /// themselves — keeping them would fail the same way on the next launch.
  Future<void> _report(
    Object cause, {
    required AppStage fallback,
    required Emitter<AuthState> emit,
  }) async {
    final appError = toAppError(cause);
    if (appError is SessionRejected && session.isSignedIn) {
      await session.signOut();
      emit(const AuthState(stage: AppStage.signedOut));
    } else {
      emit(
        state.copyWith(
          error: appError,
          stage: session.isSignedIn ? AppStage.signedIn : fallback,
        ),
      );
    }
  }
}

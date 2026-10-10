import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/features/auth/bloc/app_stage.dart';
import 'package:mobile/src/gen/openoxide/api/v1/organization.pb.dart';

/// The sign-in state, the selected organization and the member's organization
/// list, together.
///
/// The organization id is part of the state — not read from the session — so a
/// switch is a real state change a rebuild can see. Both it and the list are
/// app-wide because both change what every RPC means: the bearer token and
/// `x-organization-id` are read synchronously by the metadata provider on each
/// call, so a screen never passes them — it sends an [AuthEvent] to change them
/// and the next call picks the new values up.
class AuthState extends Equatable {
  const AuthState({
    this.stage = AppStage.probing,
    this.error,
    this.organizations = const [],
    this.busy = false,
    this.organizationId,
  });

  final AppStage stage;
  final AppError? error;
  final List<Organization> organizations;
  final bool busy;

  /// The organization every list call names. Null means no member ship yet, and
  /// every screen shows its "choose one" empty state.
  final int? organizationId;

  AuthState copyWith({
    AppStage? stage,
    AppError? error,
    List<Organization>? organizations,
    bool? busy,
    int? organizationId,
  }) => AuthState(
    stage: stage ?? this.stage,
    error: error ?? this.error,
    organizations: organizations ?? this.organizations,
    busy: busy ?? this.busy,
    organizationId: organizationId ?? this.organizationId,
  );

  @override
  List<Object?> get props => [
    stage,
    error,
    organizations,
    busy,
    organizationId,
  ];
}

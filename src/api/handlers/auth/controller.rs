use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AuthResponse, AuthService, AuthServiceExt, ConfirmEmailVerificationRequest,
    CreatePersonalAccessTokenRequest, CreatedPersonalAccessToken, DisableTwoFactorRequest,
    EmailVerificationStatus, Empty, EnableTwoFactorRequest, GetEmailVerificationStatusRequest,
    GetSetupStatusRequest, GetTwoFactorStatusRequest, ListPersonalAccessTokensRequest,
    ListPersonalAccessTokensResponse, ListSessionsRequest, ListSessionsResponse, LoginRequest,
    LogoutRequest, RecoveryCodes, RefreshRequest, RegenerateRecoveryCodesRequest,
    RequestEmailVerificationRequest, RequestPasswordResetRequest, ResetPasswordRequest,
    RevokePersonalAccessTokenRequest, RevokeSessionRequest, SetupStatus, SetupTwoFactorRequest,
    SignupRequest, TwoFactorSetup, TwoFactorStatus, UpdateUserRequest, User, WhoAmIRequest,
};

use super::helper::*;
use super::init::AuthController;

// ── ConnectRPC service implementation ──────────────────────────────

#[auto_grpc::service(connect = true, auth = true)]
impl AuthService for AuthController {
    // ── Public RPCs ──────────────────────────────────────────────────

    async fn signup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SignupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AuthResponse> + Send + use<'a>> {
        let result = self
            .service
            .signup(request.to_owned_message())
            .await
            .map_err(map_auth_error)?;
        Response::ok(auth_response_proto(result.user, result.tokens))
    }

    async fn login<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LoginRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AuthResponse> + Send + use<'a>> {
        let result = self
            .service
            .login(request.to_owned_message())
            .await
            .map_err(map_auth_error)?;
        Response::ok(auth_response_proto(result.user, result.tokens))
    }

    async fn refresh<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, RefreshRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AuthResponse> + Send + use<'a>> {
        let result = self.service.refresh(request.refresh_token).await.map_err(map_auth_error)?;
        Response::ok(auth_response_proto(result.user, result.tokens))
    }

    async fn get_setup_status<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, GetSetupStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SetupStatus> + Send + use<'a>> {
        let present = self.service.is_owner_present().await.map_err(map_auth_error)?;
        Response::ok(SetupStatus {
            owner_present: present,
            ..Default::default()
        })
    }

    async fn request_password_reset<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, RequestPasswordResetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.request_password_reset(request.email).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    async fn reset_password<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResetPasswordRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.reset_password(request.token, request.password.to_owned()).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    // ── Authenticated RPCs ──────────────────────────────────────────

    async fn who_am_i<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, WhoAmIRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<User> + Send + use<'a>> {
        let subject = self.service.get_whoami(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(subject_to_user(subject))
    }

    async fn logout<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, LogoutRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.logout_all(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    async fn update_user<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateUserRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<User> + Send + use<'a>> {
        let subject = self
            .service
            .update_user(auth.user_id(), request.to_owned_message())
            .await
            .map_err(map_auth_error)?;
        Response::ok(subject_to_user(subject))
    }

    // ── Email verification ──────────────────────────────────────────

    async fn get_email_verification_status<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, GetEmailVerificationStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EmailVerificationStatus> + Send + use<'a>> {
        let status = self.service.email_verification_status(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(status)
    }

    async fn request_email_verification<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, RequestEmailVerificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.request_email_verification(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    async fn confirm_email_verification<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ConfirmEmailVerificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.verify_email(request.token).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    // ── Two-Factor Auth ─────────────────────────────────────────────

    async fn get_two_factor_status<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, GetTwoFactorStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TwoFactorStatus> + Send + use<'a>> {
        let status = self.service.two_factor_status(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(status)
    }

    async fn setup_two_factor<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetupTwoFactorRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TwoFactorSetup> + Send + use<'a>> {
        let result = self.service.setup_two_factor(auth.user_id(), request.password.to_owned()).await.map_err(map_auth_error)?;
        Response::ok(result)
    }

    async fn enable_two_factor<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, EnableTwoFactorRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.enable_two_factor(auth.user_id(), request.code).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    async fn disable_two_factor<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DisableTwoFactorRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.disable_two_factor(auth.user_id(), request.password.to_owned(), request.code).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    async fn regenerate_recovery_codes<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RegenerateRecoveryCodesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RecoveryCodes> + Send + use<'a>> {
        let result = self.service.regenerate_recovery_codes(auth.user_id(), request.password.to_owned(), request.code).await.map_err(map_auth_error)?;
        Response::ok(result)
    }

    // ── Sessions ────────────────────────────────────────────────────

    async fn list_sessions<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListSessionsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListSessionsResponse> + Send + use<'a>> {
        let sessions = self.service.list_sessions(auth.user_id(), &auth.claims.jti).await.map_err(map_auth_error)?;
        Response::ok(ListSessionsResponse { sessions, ..Default::default() })
    }

    async fn revoke_session<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RevokeSessionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.revoke_session(auth.user_id(), request.session_id).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }

    // ── Personal Access Tokens ──────────────────────────────────────

    async fn list_personal_access_tokens<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListPersonalAccessTokensRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListPersonalAccessTokensResponse> + Send + use<'a>> {
        let tokens = self.service.list_personal_access_tokens(auth.user_id()).await.map_err(map_auth_error)?;
        Response::ok(ListPersonalAccessTokensResponse { tokens, ..Default::default() })
    }

    async fn create_personal_access_token<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreatePersonalAccessTokenRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreatedPersonalAccessToken> + Send + use<'a>> {
        let result = self.service.create_personal_access_token(
            auth.user_id(), request.name.to_owned(), request.password.to_owned(), request.expires_at,
        ).await.map_err(map_auth_error)?;
        Response::ok(result)
    }

    async fn revoke_personal_access_token<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RevokePersonalAccessTokenRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.revoke_personal_access_token(auth.user_id(), request.id).await.map_err(map_auth_error)?;
        Response::ok(Empty::default())
    }
}

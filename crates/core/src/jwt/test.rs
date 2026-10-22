#[cfg(test)]
mod tests {
    use crate::jwt::claim::JwtSubject;
    use crate::jwt::config::JwtConfig;
    use crate::jwt::error::TokenError;
    use crate::jwt::service::JwtService;

    fn subject() -> JwtSubject {
        JwtSubject {
            user_id: 123,
            email: Some("user@example.com".into()),
            first_name: Some("Test".into()),
            last_name: Some("User".into()),
            avatar: "avatar.png".into(),
            role: Some("OWNER".into()),
            group_id: 1,
        }
    }

    #[test]
    fn full_flow() {
        let service = JwtService::new(std::sync::Arc::new(JwtConfig::default()));

        let subject = subject();
        let pair = service.generate_token_pair(&subject).unwrap();

        let access_claims = service.validate_access_token(&pair.access_token).unwrap();
        assert_eq!(access_claims.sub, "123");
        assert_eq!(access_claims.user, subject);

        let refresh_claims = service.validate_refresh_token(&pair.refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, "123");
        assert_eq!(refresh_claims.user, subject);

        let wrong = service.validate_refresh_token(&pair.access_token);
        assert!(wrong.is_err());

        let new_access = service.refresh_access_token(&pair.refresh_token).unwrap();
        let new_claims = service.validate_access_token(&new_access).unwrap();
        assert_eq!(new_claims.sub, "123");
        assert_eq!(new_claims.user, subject);
    }

    #[test]
    fn expired_access_token_is_rejected_by_default() {
        let service = JwtService::new(std::sync::Arc::new(JwtConfig {
            access_expiry_mins: -120,
            ..JwtConfig::default()
        }));
        let pair = service.generate_token_pair(&subject()).unwrap();

        assert!(matches!(
            service.validate_access_token(&pair.access_token),
            Err(TokenError::Expired)
        ));
    }

    #[test]
    fn debug_skip_time_check_accepts_expired_access_token() {
        let service = JwtService::new(std::sync::Arc::new(JwtConfig {
            access_expiry_mins: -120,
            debug_skip_time_check: true,
            ..JwtConfig::default()
        }));
        let pair = service.generate_token_pair(&subject()).unwrap();

        let claims = service.validate_access_token(&pair.access_token).unwrap();
        assert_eq!(claims.sub, "123");
    }
}

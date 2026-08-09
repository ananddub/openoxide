# Rustploy Authentication & Session Security Architecture

The **Auth Service** handles user registration, Argon2id password hashing, JWT token issuance, session tracking, TOTP 2FA multi-factor authentication, and active session revoking.

---

## 1. Architecture Overview

```
                        ┌───────────────────────────────┐
                        │        AuthController         │
                        │      (/api/auth/...)          │
                        └───────────────┬───────────────┘
                                        │
                                        ▼
                        ┌───────────────────────────────┐
                        │          AuthService          │
                        └───────────────┬───────────────┘
                                        │
        ┌───────────────────────────────┼───────────────────────────────┐
        ▼                               ▼                               ▼
┌───────────────────────┐   ┌───────────────────────┐   ┌───────────────────────┐
│ Argon2id Password     │   │ JWT Token Engine      │   │ TOTP 2FA Engine       │
│ Hasher (OWASP Spec)   │   │ (Access & Session ID) │   │ (Secret / QR Code)    │
└───────────────────────┘   └───────────┬───────────┘   └───────────────────────┘
                                        │
                                        ▼
                            ┌───────────────────────┐
                            │ Session Blacklist &   │
                            │ Active Session Repo   │
                            └───────────────────────┘
```

---

## 2. Detailed Internal Working Mechanism

Authentication executes through a 5-phase security pipeline:

```
[1. User Login]     ──► Email & Plaintext Password Received
                             │
[2. Argon2id Verify]──► Password Hash Checked via Argon2id (OWASP Guidelines)
                             │
[3. TOTP 2FA Check] ──► If 2FA enabled: Verify 6-digit TOTP token / Backup Code
                             │
[4. JWT & Session]  ──► Unique Session ID (`sid`) + Signed JWT Access Token Issued
                             │
[5. Session Store]  ──► Active Session Logged in DB ──► Returned to Client (Bearer Header)
```

### Phase 1: Login Request & Identity Verification (`user.rs`)
1. Client sends `email` and `password` payload to `/api/auth/login`.
2. Resolves user record from `users` table.

### Phase 2: Argon2id Password Hash Verification (`security.rs`)
1. Verifies plaintext password against Argon2id hash (`$argon2id$v=19$m=19456,t=2,p=1$...`).
2. Adheres to OWASP security guidelines to prevent side-channel timing attacks.

### Phase 3: Multi-Factor Authentication (`two_factor/`)
If `two_factor_enable = 1`:
1. Requires 6-digit TOTP token or 8-character recovery backup code.
2. Computes HMAC-SHA1 TOTP verification using `two_factor` secret key.

### Phase 4: JWT Token & Session ID Generation (`jwt/`)
1. Generates a unique UUID Session ID (`sid`).
2. Signs JWT Access Token containing:
   - `user_id`: User database ID.
   - `email`: User email address.
   - `is_owner`: Super admin flag.
   - `session_id`: Unique `sid`.
   - `exp`: Token expiration timestamp (default 24 hours).

### Phase 5: Session Registration & Blacklist Control (`sessions.rs`)
1. Logs active session in `user_sessions` table (`user_id`, `session_id`, `user_agent`, `ip_address`, `expires_at`).
2. On logout or revoking: Session ID is blacklisted or deleted, immediately invalidating any stolen JWT tokens containing that `session_id`.

---

## 3. Database Schema & Models

### 3.1 `user_sessions` Table
- `id` (INTEGER PRIMARY KEY)
- `user_id` (INTEGER REFERENCES users(id) ON DELETE CASCADE)
- `session_id` (TEXT NOT NULL UNIQUE)
- `user_agent` (TEXT NULLABLE)
- `ip_address` (TEXT NULLABLE)
- `expires_at` (INTEGER NOT NULL)
- `created_at` (INTEGER DEFAULT (unixepoch())).

### 3.2 `two_factor` Table
- `id` (INTEGER PRIMARY KEY)
- `user_id` (INTEGER UNIQUE REFERENCES users(id) ON DELETE CASCADE)
- `secret` (TEXT NOT NULL)
- `recovery_codes` (TEXT NOT NULL): Encrypted JSON string array.

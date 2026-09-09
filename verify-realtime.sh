#!/usr/bin/env bash
# verify-realtime.sh — SSH key delete realtime fix: check → rebuild → restart → verify.
#
# Workflow (justfile ke mutabiq — SPA panel binary me embed hota hai):
#   bash verify-realtime.sh            # checks + `just restart` (rebuild + recreate)
#   bash verify-realtime.sh --check    # sirf type-checks, rebuild nahi
#
# NOTE: Pehla docker build slow ho sakta hai (musl release build). Uske baad
# deps layer cached rehta hai — sirf src/ + web/ layers rebuild hote hain.

set -uo pipefail
cd "$(dirname "$0")"

DEPLOY=1
[ "${1:-}" = "--check" ] && DEPLOY=0

pass=0; fail=0
ok()   { echo "  ✅ $1"; pass=$((pass+1)); }
bad()  { echo "  ❌ $1"; fail=$((fail+1)); }
step() { echo; echo "== $1 =="; }

step "1/4 Tools"
command -v docker >/dev/null 2>&1 && ok "docker: $(docker --version | head -1)" || bad "docker nahi mila"
command -v just   >/dev/null 2>&1 && ok "just: $(just --version)"              || { echo "  ⚠️  just nahi hai — direct docker compose fallback use hoga"; }
command -v cargo  >/dev/null 2>&1 && ok "cargo: $(cargo --version)"            || echo "  ⚠️  cargo nahi hai — quick check skip (docker build hi compile karega)"

step "2/4 Backend quick check (docker build se pehle errors pakdo)"
if command -v cargo >/dev/null 2>&1; then
  if cargo check -p openoxide 2>&1 | tail -15; then
    ok "cargo check passed (watcher.rs state-machine fix compiles)"
  else
    bad "cargo check FAILED — pehle yeh fix karo, docker build waste na ho"
  fi
else
  echo "  ⏭️  skip (cargo nahi hai)"
fi

step "3/4 Frontend type check"
if [ -d web/openoxide_react/node_modules ]; then
  cd web/openoxide_react
  if bunx tsc --noEmit 2>&1 | tail -15 || npx tsc --noEmit 2>&1 | tail -15; then
    ok "tsc passed (ssh_keys.ts invalidate + use-table-watch logging)"
  else
    bad "tsc FAILED — output upar dekho"
  fi
  cd ../..
else
  echo "  ⏭️  skip (node_modules nahi — docker build me bun install khud karega)"
fi

step "4/4 Rebuild + restart (docker compose dev stack)"
if [ "$DEPLOY" = "1" ]; then
  docker network create openoxide-network 2>/dev/null || true
  if command -v just >/dev/null 2>&1; then
    just restart
  else
    docker compose -f docker-compose.dev.yml up -d --build --force-recreate
  fi
  ok "Stack rebuild + recreate hua"
  echo
  echo "  Logs dekhne ke liye (doosre terminal me):"
  echo "    just logs              # ya: docker logs -f openoxide"
else
  echo "  ⏭️  --check mode: rebuild nahi kiya. Deploy ke liye: bash verify-realtime.sh"
fi

cat <<'EOF'

=====================================================================
RUNTIME TEST — https://localhost:4000 kholo, phir:
=====================================================================
1. Startup verify (docker logs openoxide | head -50):
     "Database connection established in WAL mode with realtime CDC attached."

2. Browser F12 Console me page load pe:
     [table-watch] resync → full invalidation      ← stream khula ✅
   Agar dikhe:
     [table-watch] stream dropped, reconnecting …  ← stream fail, error padho ❌

3. SSH key DELETE karo — 3 cheezein ek saath check karo:
   a) UI me row INSTANT gayab          (mutation-side invalidate — naya fix)
   b) Console: [table-watch] change → ['ssh_keys']   (CDC event browser tak aaya)
   c) docker logs openoxide -f me:
        sqlite realtime change committed tables=["ssh_keys"]

4. Doosra browser tab (same login) me ssh-keys page kholo → wahan delete
   sirf stream se aayega (150ms coalesced tick ke andar).

PASS  = a) + b) + c) teeno
FAIL-A: server log (c) hi nahi  → image purana hai (rebuild skip hua?) ya CDC attach nahi
FAIL-B: (c) hai par (b) nahi   → stream transport issue — console warning ka error paste karo
FAIL-C: (b) hai par doosre tab ka UI stale → mapping bug — report karo
=====================================================================
EOF

echo
echo "RESULT: $pass passed, $fail failed"
exit $fail
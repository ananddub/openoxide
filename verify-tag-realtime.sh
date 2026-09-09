#!/usr/bin/env bash
# verify-tag-realtime.sh — TAG update realtime fix: check → rebuild → restart → verify.
#
# Problem: tag CREATE ka CDC event aata hai, lekin tag UPDATE ka nahi.
# Yeh script same pipeline pe UPDATE ko verify karta hai.
#
# Workflow (justfile ke mutabiq — SPA panel binary me embed hota hai):
#   bash verify-tag-realtime.sh            # checks + `just restart` (rebuild + recreate)
#   bash verify-tag-realtime.sh --check    # sirf type-checks, rebuild nahi
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
    ok "cargo check passed"
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
    ok "tsc passed"
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
  echo "  ⏭️  --check mode: rebuild nahi kiya. Deploy ke liye: bash verify-tag-realtime.sh"
fi

cat <<'EOF'

=====================================================================
RUNTIME TEST — https://localhost:4000 kholo, phir (2 browser tabs):
=====================================================================
1. Startup verify (docker logs openoxide | head -50):
     "Database connection established in WAL mode with realtime CDC attached."

2. Browser F12 Console me page load pe:
     [table-watch] resync → full invalidation      ← stream khula ✅
   Agar dikhe:
     [table-watch] stream dropped, reconnecting …  ← stream fail, error padho ❌

3. TAB-1: Tags page kholo → kisi tag ka naam/color EDIT karo → Save
   (dusre TAB-2 me bhi tags page khula ho)

4. TEEN cheezein ek saath check karo:
   a) TAB-2 me tag INSTANT update hua?           (table-watch invalidate)
   b) TAB-1 Console: [table-watch] change → ['tags','projects']
   c) docker logs openoxide -f me (SAVE ke turant baad):
        sqlite realtime change committed tables=["tags"]
      ...aur uske pehle:
        cdc event forwarded to bus table=tags rowid=<tag_id> organization_id=Some(1)

5. EXTRA (org-scope check): docker logs me UPDATE ke waqt dekho —
   agar "organization_id=None" dikhe to org lookup fail ho raha hai
   (tags row ka organization_id col check karo: sqlite3 db "SELECT id, organization_id FROM tags;")

PASS  = a) + b) + c) teeno
FAIL-A: server log (c) hi nahi (create pe dikhta hai par update pe nahi)
        → UPDATE SQL hooks tak pahunch hi nahi raha; sqlite3_update_hook op=23
          filter check karo (realtime.c realtime_update_hook)
FAIL-B: (c) hai par (b) nahi   → stream transport issue — console warning ka error paste karo
FAIL-C: (b) hai par TAB-2 UI stale → mapping bug — report karo
FAIL-D: mutation ke baad TAB-1 me bhi tag purana dikh raha hai
        → update khud fail hua hai; network tab me UpdateTag response dekho
=====================================================================
EOF

echo
echo "RESULT: $pass passed, $fail failed"
exit $fail
#!/usr/bin/env bash
#
# Signs a development JWT for the orchard sync endpoint without requiring a
# real account. The tenant is simply `user_identifier`, so a throwaway UUID is
# enough to exercise the offline-first store.
#
# Usage:
#   scripts/mint-dev-token.sh [--user-id <uuid>] [--exp <unix-ts>]
#
# The signing key is taken from $JWT_SIGNING_KEY, falling back to the value in
# `server/.env` (the server loads that file via dotenv at boot). Set the
# printed token as the app's `LUNAR_DEV_TOKEN` when running `tauri dev`, or
# pass it as the `Authorization: Bearer <token>` header for curl tests.
set -euo pipefail

cd "$(dirname "$0")/.."

key="${JWT_SIGNING_KEY:-}"
if [[ -z "$key" && -f server/.env ]]; then
  key="$(grep -E '^JWT_SIGNING_KEY=' server/.env | head -n1 | cut -d= -f2- | tr -d '"' || true)"
fi
if [[ -z "$key" ]]; then
  echo "JWT_SIGNING_KEY is not set (and server/.env does not define it)" >&2
  exit 1
fi

user_id="$(uuidgen | tr '[:upper:]' '[:lower:]')"
exp="$(( $(date +%s) + 7 * 86400 ))"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --user-id) user_id="${2:?--user-id requires a value}"; shift 2 ;;
    --exp) exp="${2:?--exp requires a value}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 1 ;;
  esac
done

b64url() {
  openssl base64 -A | tr '+/' '-_' | tr -d '='
}

now="$(date +%s)"
header="$(printf '{"alg":"HS256","typ":"JWT"}' | b64url)"
payload="$(printf '{"sub":"%s","iat":%s,"exp":%s,"email":"dev@test.test","user_identifier":"%s"}' \
  "$user_id" "$now" "$exp" "$user_id" | b64url)"
signature="$(printf '%s.%s' "$header" "$payload" | openssl dgst -sha256 -hmac "$key" -binary | b64url)"

echo "$header.$payload.$signature"
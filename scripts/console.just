[working-directory('console')]
watch-frontend:
    npm run dev

[working-directory('console')]
watch-frontend-mobile:
    NUXT_APP_TARGET=mobile npm run dev

[working-directory('console')]
watch-tauri:
    npm run tauri dev

[working-directory('console')]
watch-mobile:
    ./src-tauri/gen/android/gradlew --stop
    NUXT_APP_TARGET=mobile npm run tauri android dev

[working-directory('console')]
watch-mobile-device HOST:
    TAURI_DEV_HOST={{ HOST }} npm run tauri android dev

[working-directory('console')]
watch-ios:
    NUXT_APP_TARGET=mobile npm run tauri ios dev

[working-directory('console')]
watch-ios-device HOST:
    NUXT_APP_TARGET=mobile TAURI_DEV_HOST={{ HOST }} npm run tauri ios dev

[working-directory('console')]
watch-console:
    NUXT_APP_TARGET=desktop npm run dev & npm run tauri dev

[working-directory('console')]
build-console:
    NUXT_APP_TARGET=desktop npm run tauri build

[working-directory('console')]
build-mobile:
    ./src-tauri/gen/android/gradlew --stop
    NUXT_APP_TARGET=mobile npm run tauri android build

[working-directory('console')]
install-frontend-dependencies:
    npm install

[working-directory('console')]
test-console:
    echo "TBD"

[working-directory('console/src-tauri')]
test-tauri:
    echo "TBD"

# Mint a development JWT for the orchard sync endpoint — no account required
# (the tenant is just the JWT's user_identifier). Requires JWT_SIGNING_KEY to
# match the running server (env or server/.env). Optional passthrough args:
# `just dev-token -- --user-id <uuid> --exp <unix-ts>`
dev-token ARGS="":
    {{ justfile_directory() }}/scripts/mint-dev-token.sh {{ ARGS }}

# Act as a second device against the local sync endpoint to exercise the
# inbound mirror path without running a second Tauri instance. Set
# LUNAR_DEV_TOKEN first (see `just dev-token`).
[working-directory('console/src-tauri')]
dev-device-b ID="" TITLE="from device B":
    #!/usr/bin/env bash
    set -euo pipefail
    : "${LUNAR_DEV_TOKEN:?run \`just dev-token\` first and export LUNAR_DEV_TOKEN}"
    if [ -z "{{ ID }}" ]; then
        cargo run --example dev_device_b -- --title "{{ TITLE }}"
    else
        cargo run --example dev_device_b -- --id "{{ ID }}" --title "{{ TITLE }}"
    fi

# One-shot device-B injector: mints a token, sets LUNAR_DEV_TOKEN, and pushes a
# todo from a fresh in-memory second device. Start the server first.
dev-sync ID="" TITLE="from device B":
    #!/usr/bin/env bash
    set -euo pipefail
    LUNAR_DEV_TOKEN="$("{{ justfile_directory() }}"/scripts/mint-dev-token.sh)"
    export LUNAR_DEV_TOKEN
    just dev-device-b "{{ ID }}" "{{ TITLE }}"

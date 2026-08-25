[working-directory: 'lunar']
watch-lunar:
	cargo watch -qcx check

# Start postgres and mysql containers (docker/docker-compose.yml)
[working-directory: 'lunar']
db-up:
	docker compose -f docker/docker-compose.yml up -d

[working-directory: 'lunar']
@migrate-add target:
	@sea-orm-cli migrate generate "{{target}}"

[working-directory: 'lunar']
@migrate-init:
	@sea-orm-cli migrate init

# Run pending migrations against postgres (docker)
[working-directory: 'lunar']
@migrate-up-postgres:
	sea-orm-cli migrate up --database-url {{POSTGRES_URL}}

[working-directory: 'server']
@migrate-up-server:
	sea-orm-cli migrate up --database-url {{DATABASE_URL}}

[working-directory: 'lunar']
@migrate-up:
	just migrate-up-postgres

# Pull schema and generate entities for PostgreSQL using docker defaults (feature: postgres)
[working-directory: 'lunar']
@db-pull-postgres:
	sea-orm-cli generate entity \
		--database-url {{POSTGRES_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		--model-extra-attributes 'lunar_macros::ts_rs_export_sea_orm_entity_name' \
		--enum-extra-derives 'ts_rs::TS' \
		--enum-extra-attributes 'ts(export)' \
		--enum-extra-attributes 'serde(rename_all="snake_case")' \
		-o src/entities \
		--seaography
	node ../scripts/fix-ts-exports.mjs src/entities
	
# Generate entities for all backends.
# Starts docker, runs migrations, then generates entities.
#   just db-pull
[working-directory: 'lunar']
db-pull:
	just db-up
	@echo "Waiting for PostgreSQL to be ready..."
	just migrate-up
	just db-pull-postgres
	# just db-pull-server

[working-directory: 'lunar']
test-lunar:
	cargo test

db-pull-server:
	just migrate-up-server
	just generate-server-entities

# --- WASM build & PGlite migration export ---

# Export migration SQL to generated/pglite/migrations, then build the WASM bundle.
#   just build-wasm          # dev profile (matches pkg/ checked into the repo)
#   just build-wasm release  # release profile
[working-directory: 'lunar']
build-wasm profile='dev':
	cargo run --manifest-path migration/Cargo.toml --bin migration-export -- generated/pglite/migrations
	wasm-pack build --target web --{{profile}}

# Export PGlite migration SQL only (no WASM build).
# This is what console/scripts/sync-lunar-schema.mjs runs if the generated
# files are missing.
[working-directory: 'lunar']
schema-export:
	cargo run --manifest-path migration/Cargo.toml --bin migration-export -- generated/pglite/migrations

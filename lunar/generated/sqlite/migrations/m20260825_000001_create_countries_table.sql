-- ============================================
-- m20260825_000001_create_countries_table
-- ============================================

CREATE TABLE IF NOT EXISTS "country" ( "identifier" char(26) PRIMARY KEY, "currency_code" varchar(10) NOT NULL, "currency" varchar(100) NOT NULL, "country" varchar(100) NOT NULL, "flag" text NULL );


-- Initial meNexus schema
-- Migrated from db/init/01_schema.sql

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Events table for activity/federation
CREATE TABLE IF NOT EXISTS events (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  event_type    TEXT NOT NULL,
  module_kind   TEXT,
  module_slug   TEXT,

  agent         TEXT NOT NULL,
  agent_signature TEXT,                  -- Optional signature for federated auth

  target        JSONB,

  previous      UUID REFERENCES events(id) ON DELETE SET NULL,

  content       TEXT,
  artifacts     TEXT[],                  -- Option<Vec<ArtifactUri>>
  metadata      JSONB,                   -- Option<HashMap<String, String>>
  links         TEXT[],                  -- Option<Vec<Url>> (stored as TEXT)
  data          BYTEA,                   -- Option<Vec<u8>>
  expiration    TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_events_created_at   ON events (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_events_event_type   ON events (event_type);
CREATE INDEX IF NOT EXISTS idx_events_module       ON events (module_kind, module_slug, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_events_agent        ON events (agent);
CREATE INDEX IF NOT EXISTS idx_events_previous     ON events (previous);
CREATE INDEX IF NOT EXISTS idx_events_target_gin   ON events USING GIN (target);
CREATE INDEX IF NOT EXISTS idx_events_metadata_gin ON events USING GIN (metadata);

-- User profiles (Automerge CRDT docs)
CREATE TABLE IF NOT EXISTS profiles (
    public_key      TEXT PRIMARY KEY,
    doc_bytes       BYTEA NOT NULL,   -- Automerge serialized doc
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Auth challenges for login flow
CREATE TABLE IF NOT EXISTS challenges (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  agent         TEXT NOT NULL,
  nonce         TEXT NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at    TIMESTAMPTZ NOT NULL
);

-- User sessions
CREATE TABLE IF NOT EXISTS sessions (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  agent         TEXT NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at    TIMESTAMPTZ NOT NULL,
  revoked       BOOL NOT NULL DEFAULT FALSE
);

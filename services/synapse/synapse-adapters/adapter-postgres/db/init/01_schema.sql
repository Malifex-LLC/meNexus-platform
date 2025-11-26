CREATE EXTENSION IF NOT EXISTS pgcrypto;

DROP TABLE IF EXISTS events CASCADE;

CREATE TABLE events (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  event_type    TEXT NOT NULL,
  module_kind   TEXT,
  module_slug   TEXT,

  agent         TEXT NOT NULL,

  target        JSONB,

  previous      UUID REFERENCES events(id) ON DELETE SET NULL,

  content       TEXT,
  artifacts     TEXT[],                  -- Option<Vec<ArtifactUri>>
  metadata      JSONB,                   -- Option<HashMap<String, String>>
  links         TEXT[],                  -- Option<Vec<Url>> (stored as TEXT)
  data          BYTEA,                   -- Option<Vec<u8>>
  expiration    TIMESTAMPTZ
);

CREATE INDEX idx_events_created_at   ON events (created_at DESC);
CREATE INDEX idx_events_event_type   ON events (event_type);
CREATE INDEX idx_events_module       ON events (module_kind, module_slug, created_at DESC);
CREATE INDEX idx_events_agent        ON events (agent);
CREATE INDEX idx_events_previous     ON events (previous);
CREATE INDEX idx_events_target_gin   ON events USING GIN (target);
CREATE INDEX idx_events_metadata_gin ON events USING GIN (metadata);

CREATE TABLE profiles (
    public_key      text PRIMARY KEY,
    doc_bytes       bytea NOT NULL,   -- Automerge serialized doc
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE challenges (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  agent         TEXT NOT NULL,
  nonce         TEXT NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at    TIMESTAMPTZ NOT NULL
);

CREATE TABLE sessions (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  agent         TEXT NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at    TIMESTAMPTZ NOT NULL,
  revoked       bool NOT NULL
);

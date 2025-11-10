-- UUID helper
CREATE EXTENSION IF NOT EXISTS pgcrypto;

DROP TABLE IF EXISTS events CASCADE;

CREATE TABLE events (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  event_type    TEXT NOT NULL,
  module_kind   TEXT,
  module_slug   TEXT,

  -- store the public key as text; constrain format/length later if desired
  agent         TEXT NOT NULL,

  -- JSONB for your enum + “custom” variants
  target        JSONB,

  previous      UUID REFERENCES events(id) ON DELETE SET NULL,

  content       TEXT,
  artifacts     TEXT[],                  -- Option<Vec<ArtifactUri>>
  metadata      JSONB,                   -- Option<HashMap<String, String>>
  links         TEXT[],                  -- Option<Vec<Url>> (stored as TEXT)
  data          BYTEA,                   -- Option<Vec<u8>>
  expiration    TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_events_created_at   ON events (created_at DESC);
CREATE INDEX idx_events_event_type   ON events (event_type);
CREATE INDEX idx_events_module       ON events (module_kind, module_slug, created_at DESC);
CREATE INDEX idx_events_agent        ON events (agent);
CREATE INDEX idx_events_previous     ON events (previous);
CREATE INDEX idx_events_target_gin   ON events USING GIN (target);
CREATE INDEX idx_events_metadata_gin ON events USING GIN (metadata);

-- If you *really* want a length check on the key, make sure it matches your encoding.
-- For example, hex-encoded 33-byte compressed keys are length 66, uncompressed 65 bytes -> 130 hex chars:
-- ALTER TABLE events
--   ADD CONSTRAINT chk_agent_len_hex CHECK (char_length(agent) IN (66, 130));


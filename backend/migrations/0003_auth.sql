CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO users (id, email, password_hash, name)
VALUES (
    '11111111-1111-1111-1111-111111111111'::uuid,
    'legacy@slopify.local',
    'legacy-user',
    'Legacy User'
)
ON CONFLICT (email) DO NOTHING;

ALTER TABLE threads
    ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE CASCADE;

UPDATE threads
SET user_id = '11111111-1111-1111-1111-111111111111'::uuid
WHERE user_id IS NULL;

ALTER TABLE threads
    ALTER COLUMN user_id SET NOT NULL;

CREATE INDEX IF NOT EXISTS threads_user_id_updated_at_idx
    ON threads (user_id, updated_at DESC, created_at DESC);

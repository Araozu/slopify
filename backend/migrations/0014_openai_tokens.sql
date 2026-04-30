CREATE TABLE IF NOT EXISTS openai_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    auth_type TEXT NOT NULL CHECK (auth_type IN ('api_key', 'oauth_refresh_token')),
    token TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, name)
);

CREATE INDEX IF NOT EXISTS openai_tokens_user_id_created_at_idx
    ON openai_tokens (user_id, created_at DESC);

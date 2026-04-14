-- Store system prompt content per message so forked threads keep an audit trail
-- of which system prompt was active when each message was sent/received.
ALTER TABLE messages ADD COLUMN IF NOT EXISTS system_prompt TEXT;

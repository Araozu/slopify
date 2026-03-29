-- Add model to threads
ALTER TABLE threads ADD COLUMN IF NOT EXISTS model TEXT;

-- Move conversations to threads if needed (though they seem to be separate entities for now)
-- For now, let's just make sure messages can be linked to threads.
-- The existing messages table already has conversation_id.
-- If we want to use threads as conversations, we should probably rename or link them.

-- Let's check if we should rename conversations to threads or vice versa.
-- Since 'threads' table exists and is used by the frontend, let's make messages point to threads.

ALTER TABLE messages DROP CONSTRAINT IF EXISTS messages_conversation_id_fkey;
ALTER TABLE messages RENAME COLUMN conversation_id TO thread_id;
ALTER TABLE messages ADD CONSTRAINT messages_thread_id_fkey FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE;

-- Add content column to messages if it doesn't exist (it currently uses 'parts' JSONB)
-- The frontend expects 'content' string.
ALTER TABLE messages ADD COLUMN IF NOT EXISTS content TEXT;

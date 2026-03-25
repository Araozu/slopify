CREATE TYPE "public"."chat_generation_status" AS ENUM('queued', 'running', 'complete', 'failed');--> statement-breakpoint
CREATE TYPE "public"."chat_message_role" AS ENUM('system', 'user', 'assistant', 'tool');--> statement-breakpoint
CREATE TYPE "public"."chat_message_status" AS ENUM('pending', 'streaming', 'complete', 'failed');--> statement-breakpoint
CREATE TYPE "public"."chat_thread_status" AS ENUM('draft', 'active', 'archived');--> statement-breakpoint
CREATE TABLE "app_user" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"email" varchar(320),
	"display_name" text NOT NULL,
	"avatar_url" text,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "chat_generation" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"thread_id" uuid NOT NULL,
	"started_by_message_id" uuid,
	"provider" varchar(120) NOT NULL,
	"model" varchar(120) NOT NULL,
	"status" "chat_generation_status" DEFAULT 'queued' NOT NULL,
	"temperature" real DEFAULT 0.7 NOT NULL,
	"prompt_tokens" integer DEFAULT 0 NOT NULL,
	"completion_tokens" integer DEFAULT 0 NOT NULL,
	"total_tokens" integer DEFAULT 0 NOT NULL,
	"failure_reason" text,
	"metadata" jsonb DEFAULT '{}'::jsonb NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"completed_at" timestamp with time zone
);
--> statement-breakpoint
CREATE TABLE "chat_message" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"thread_id" uuid NOT NULL,
	"sequence" integer NOT NULL,
	"role" "chat_message_role" NOT NULL,
	"status" "chat_message_status" DEFAULT 'complete' NOT NULL,
	"content" text NOT NULL,
	"content_parts" jsonb DEFAULT '[]'::jsonb NOT NULL,
	"metadata" jsonb DEFAULT '{}'::jsonb NOT NULL,
	"model" varchar(120),
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "chat_thread" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"owner_id" uuid,
	"title" text NOT NULL,
	"summary" text,
	"status" "chat_thread_status" DEFAULT 'active' NOT NULL,
	"pinned" boolean DEFAULT false NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL,
	"last_message_at" timestamp with time zone DEFAULT now() NOT NULL,
	"archived_at" timestamp with time zone
);
--> statement-breakpoint
ALTER TABLE "chat_generation" ADD CONSTRAINT "chat_generation_thread_id_chat_thread_id_fk" FOREIGN KEY ("thread_id") REFERENCES "public"."chat_thread"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "chat_generation" ADD CONSTRAINT "chat_generation_started_by_message_id_chat_message_id_fk" FOREIGN KEY ("started_by_message_id") REFERENCES "public"."chat_message"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "chat_message" ADD CONSTRAINT "chat_message_thread_id_chat_thread_id_fk" FOREIGN KEY ("thread_id") REFERENCES "public"."chat_thread"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "chat_thread" ADD CONSTRAINT "chat_thread_owner_id_app_user_id_fk" FOREIGN KEY ("owner_id") REFERENCES "public"."app_user"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
CREATE UNIQUE INDEX "app_user_email_unique" ON "app_user" USING btree ("email");--> statement-breakpoint
CREATE INDEX "chat_generation_thread_created_idx" ON "chat_generation" USING btree ("thread_id","created_at");--> statement-breakpoint
CREATE INDEX "chat_generation_status_idx" ON "chat_generation" USING btree ("status");--> statement-breakpoint
CREATE INDEX "chat_message_thread_created_idx" ON "chat_message" USING btree ("thread_id","created_at");--> statement-breakpoint
CREATE UNIQUE INDEX "chat_message_thread_sequence_unique" ON "chat_message" USING btree ("thread_id","sequence");--> statement-breakpoint
CREATE INDEX "chat_thread_status_idx" ON "chat_thread" USING btree ("status");--> statement-breakpoint
CREATE INDEX "chat_thread_owner_last_message_idx" ON "chat_thread" USING btree ("owner_id","last_message_at");
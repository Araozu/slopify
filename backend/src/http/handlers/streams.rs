use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_core::Stream;
use serde::Serialize;

use crate::chat::contracts::{
	ChatMessageStatus, ChatRole, ClientChatEvent, ClientChatMessage, ClientChatMessageMetadata,
	ClientChatPart, ProviderDescriptor,
};

#[derive(Serialize)]
struct HelloEnvelope {
	payload: ClientChatEvent
}

pub async fn hello_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
	let stream = stream! {
		for index in 0..4 {
			yield Ok(build_event(index));

			if index < 3 {
				tokio::time::sleep(Duration::from_millis(600)).await;
			}
		}
	};

	Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("ping"))
}

fn build_event(index: usize) -> Event {
	let message = hello_world_message();

	let payload = match index {
		0 => ClientChatEvent::MessageStarted {
			message: ClientChatMessage {
				status: ChatMessageStatus::Streaming,
				parts: vec![],
				..message.clone()
			}
		},
		1 => ClientChatEvent::MessageDelta {
			message_id: message.id.clone(),
			delta: "Hello".to_string()
		},
		2 => ClientChatEvent::MessageDelta {
			message_id: message.id.clone(),
			delta: " world from SSE".to_string()
		},
		_ => ClientChatEvent::MessageCompleted { message }
	};

	let event_name = match index {
		0 => "message_started",
		1 | 2 => "message_delta",
		_ => "message_completed"
	};

	let data = serde_json::to_string(&HelloEnvelope { payload }).expect("stream payload is valid");

	Event::default().event(event_name).data(data)
}

fn hello_world_message() -> ClientChatMessage {
	ClientChatMessage {
		id: "msg_hello_world".to_string(),
		conversation_id: "conv_demo".to_string(),
		role: ChatRole::Assistant,
		status: ChatMessageStatus::Completed,
		parts: vec![ClientChatPart::Text {
			text: "Hello world from the Rust backend SSE endpoint.".to_string()
		}],
		provider: ProviderDescriptor {
			provider: "demo".to_string(),
			model: "hello-stream-v1".to_string(),
			endpoint: Some("local".to_string())
		},
		created_at: "2026-03-25T00:00:00Z".to_string(),
		metadata: ClientChatMessageMetadata {
			finish_reason: Some("stop".to_string()),
			vendor_metadata: serde_json::json!({
				"demo": true,
				"source": "hello_stream"
			})
		}
	}
}

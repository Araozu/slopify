use async_stream::stream;
use futures_core::Stream;
use serde_json::json;
use std::time::Duration;

use crate::chat::contracts::{
    ChatMessageStatus, ChatRole, ClientChatEvent, ClientChatMessage, ClientChatMessageMetadata,
    ClientChatPart, ProviderDescriptor,
};

pub fn hello_world_stream() -> impl Stream<Item = ClientChatEvent> {
    stream! {
        for index in 0..4 {
            yield build_event(index);

            if index < 3 {
                tokio::time::sleep(Duration::from_millis(600)).await;
            }
        }
    }
}

fn build_event(index: usize) -> ClientChatEvent {
    let message = hello_world_message();

    match index {
        0 => ClientChatEvent::MessageStarted {
            message: ClientChatMessage {
                status: ChatMessageStatus::Streaming,
                parts: vec![],
                ..message.clone()
            },
        },
        1 => ClientChatEvent::MessageDelta {
            message_id: message.id.clone(),
            delta: "Hello".to_string(),
        },
        2 => ClientChatEvent::MessageDelta {
            message_id: message.id.clone(),
            delta: " world from SSE".to_string(),
        },
        _ => ClientChatEvent::MessageCompleted { message },
    }
}

fn hello_world_message() -> ClientChatMessage {
    ClientChatMessage {
        id: "msg_hello_world".to_string(),
        conversation_id: "conv_demo".to_string(),
        role: ChatRole::Assistant,
        status: ChatMessageStatus::Completed,
        parts: vec![ClientChatPart::Text {
            text: "Hello world from the Rust backend SSE endpoint.".to_string(),
        }],
        provider: ProviderDescriptor {
            provider: "demo".to_string(),
            model: "hello-stream-v1".to_string(),
            endpoint: Some("local".to_string()),
        },
        created_at: "2026-03-25T00:00:00Z".to_string(),
        metadata: ClientChatMessageMetadata {
            finish_reason: Some("stop".to_string()),
            vendor_metadata: json!({
                "demo": true,
                "source": "hello_stream"
            }),
        },
    }
}

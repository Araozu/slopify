use std::{convert::Infallible, time::Duration};

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_core::Stream;
use serde::Serialize;

use crate::chat::{contracts::ClientChatEvent, streams as chat_streams};

#[derive(Serialize)]
struct HelloEnvelope {
    payload: ClientChatEvent,
}

pub async fn hello_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = futures_util::StreamExt::map(chat_streams::hello_world_stream(), |event| {
        Ok(build_sse_event(event))
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("ping"),
    )
}

fn build_sse_event(payload: ClientChatEvent) -> Event {
    let event_name = match &payload {
        ClientChatEvent::MessageStarted { .. } => "message_started",
        ClientChatEvent::MessageDelta { .. } => "message_delta",
        ClientChatEvent::MessageCompleted { .. } => "message_completed",
        ClientChatEvent::MessageFailed { .. } => "message_failed",
    };

    let data = serde_json::to_string(&HelloEnvelope { payload }).expect("stream payload is valid");

    Event::default().event(event_name).data(data)
}

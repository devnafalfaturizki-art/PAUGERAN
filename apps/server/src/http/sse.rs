//! Server-Sent Events (SSE) utilities for streaming agent responses.
//!
//! [CB §4.7] — HTTP Server Layer
//!
//! Provides ergonomic wrappers around `axum::response::sse` for the
//! chat and reasoning streaming endpoints used by the frontend.

use axum::{
    response::{sse::Event as SseEvent, Sse},
};
use futures::stream::{Stream, StreamExt};

/// Build an SSE response from a stream of already-shaped events.
pub fn sse_response<S>(stream: S) -> Sse<S>
where
    S: Stream<Item = Result<SseEvent, axum::Error>> + Send + 'static,
{
    Sse::new(stream)
}

/// Adapt a stream of string chunks into SSE `message` events.
pub fn text_event_stream<S>(stream: S) -> impl Stream<Item = Result<SseEvent, axum::Error>> + Send + 'static
where
    S: Stream<Item = String> + Send + 'static,
{
    stream.map(|text| Ok(SseEvent::default().event("message").data(text)))
}

/// Convenience constructor for building an SSE event from local types.
pub fn event_from(name: impl Into<String>, data: impl Into<String>) -> SseEvent {
    SseEvent::default().event(name.into()).data(data.into())
}

/// Stream of typed events used internally by the reasoning engine.
#[derive(Debug, Clone)]
pub struct AgentEvent {
    pub event: String,
    pub data: String,
}

impl AgentEvent {
    pub fn new(event: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            event: event.into(),
            data: data.into(),
        }
    }

    pub fn into_sse(self) -> SseEvent {
        SseEvent::default().event(self.event).data(self.data)
    }
}
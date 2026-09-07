//! Server-Sent Events (SSE) utilities for streaming agent responses.
//! 
//! [CB §4.7] — HTTP Server Layer

use axum::{
    body::Bytes,
    response::{IntoResponse, Response, Sse},
    Error,
};
use futures::{future::Ready, stream::Stream};
use std::future::Future;
use std::task::{Context, Poll};

/// Create an SSE response from a stream of events.
/// 
/// [CB §3.4] — Streaming text renderer
pub fn sse_response<S, E>(stream: S) -> Sse<S>
where
    S: Stream<Item = Result<axum::response::sse::Event, E>> + Send + 'static,
    E: std::error::Error,
{
    Sse::new(stream)
}

/// Wrap a stream of strings as SSE text events.
pub fn text_event_stream<S, E>(stream: S) -> impl Stream<Item = Result<axum::response::sse::Event, E>>
where
    S: Stream<Item = String> + Send + 'static,
    E: std::error::Error,
{
    stream.map(|text| {
        Ok(axum::response::sse::Event::default().event("message").data(text))
    })
}

#[derive(Debug)]
pub struct Event {
    pub event: String,
    pub data: String,
}

impl axum::response::sse::Event {
    pub fn from_event(event: Event) -> Self {
        Self::default()
            .event(event.event)
            .data(event.data)
    }
}

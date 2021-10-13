use std::{error::Error, task::Poll};

use futures::Sink;
use winput::Vk;

pub struct Paster;

impl Sink<String> for Paster {
    type Error = Box<dyn Error>;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // TODO: focus vrchat window here
        Poll::Ready(Ok(()))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
        self.paste_text(item);
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Paster {
    pub fn new() -> Self {
        Self
    }

    fn paste_text(&self, text: String) {
        winput::send_str(text.as_str());
        winput::send(Vk::Enter);
    }
}

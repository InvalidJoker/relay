// insipired by https://github.com/ekzhang/bore/blob/main/src/shared.rs

use crate::constants::{MAX_FRAME_LENGTH, NETWORK_TIMEOUT};
use anyhow::Context;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::timeout;
use tokio_util::codec::{Framed, FramedParts, LengthDelimitedCodec};
use tracing::trace;

/// A wrapper around a framed stream that uses length-delimited frames and postcard serialization.
pub struct StreamWorker<U>(Framed<U, LengthDelimitedCodec>);

impl<U: AsyncRead + AsyncWrite + Unpin> StreamWorker<U> {
    pub fn new(stream: U) -> Self {
        let codec = LengthDelimitedCodec::builder()
            .max_frame_length(MAX_FRAME_LENGTH)
            .new_codec();
        Self(Framed::new(stream, codec))
    }

    pub async fn recv<T: DeserializeOwned>(&mut self) -> anyhow::Result<Option<T>> {
        trace!("waiting to receive postcard message");
        if let Some(next_message) = self.0.next().await {
            let byte_message = next_message.context("frame error, invalid byte length")?;
            let serialized_obj =
                postcard::from_bytes(&byte_message).context("unable to parse message")?;
            Ok(Some(serialized_obj))
        } else {
            Ok(None)
        }
    }

    pub async fn recv_timeout<T: DeserializeOwned>(&mut self) -> anyhow::Result<Option<T>> {
        timeout(NETWORK_TIMEOUT, self.recv())
            .await
            .context("timed out waiting for initial message")?
    }

    pub async fn send<T: Serialize>(&mut self, msg: T) -> anyhow::Result<()> {
        trace!("sending postcard message");
        let bytes = postcard::to_allocvec(&msg)?;
        self.0.send(Bytes::from(bytes)).await?;
        Ok(())
    }

    pub fn into_parts(self) -> FramedParts<U, LengthDelimitedCodec> {
        self.0.into_parts()
    }
}

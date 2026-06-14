// insipired by https://github.com/ekzhang/bore/blob/main/src/shared.rs

use anyhow::{Context, Result};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::timeout;
use tokio_util::codec::{Framed, FramedParts, LengthDelimitedCodec};

use crate::constants::NETWORK_TIMEOUT;

/// A wrapper around `postcard` for serialization.
pub struct StreamWorker<U>(Framed<U, LengthDelimitedCodec>);

impl<U: AsyncRead + AsyncWrite + Unpin> StreamWorker<U> {
    pub fn new(stream: U) -> Self {
        Self(Framed::new(stream, LengthDelimitedCodec::new()))
    }

    /// Read the next null-delimited JSON instruction, with a default timeout.
    ///
    /// This is useful for parsing the initial message of a stream for handshake or
    /// other protocol purposes, where we do not want to wait indefinitely.
    pub async fn recv_timeout<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        timeout(NETWORK_TIMEOUT, self.recv())
            .await
            .context("timed out waiting for initial message")?
    }

    pub async fn send<T: Serialize>(&mut self, msg: T) -> Result<()> {
        let bytes = postcard::to_stdvec(&msg)?;
        self.0.send(Bytes::from(bytes)).await?;
        Ok(())
    }

    pub async fn recv<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        match self.0.next().await {
            Some(frame) => {
                let bytes = frame?;
                Ok(Some(postcard::from_bytes(&bytes)?))
            }
            None => Ok(None),
        }
    }

    pub fn into_parts(self) -> FramedParts<U, LengthDelimitedCodec> {
        self.0.into_parts()
    }
}

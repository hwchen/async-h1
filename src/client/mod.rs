//! Process HTTP connections on the client.

use futures_io::{AsyncRead, AsyncWrite};
use futures_util::io::copy;
use http_types::{Request, Response};

mod decode;
mod encode;

pub use decode::decode;
pub use encode::Encoder;

/// Opens an HTTP/1.1 connection to a remote host.
pub async fn connect<RW>(mut stream: RW, req: Request) -> http_types::Result<Response>
where
    RW: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
{
    let mut req = Encoder::encode(req).await?;
    log::trace!("> {:?}", &req);

    copy(&mut req, &mut stream).await?;

    let res = decode(stream).await?;
    log::trace!("< {:?}", &res);

    Ok(res)
}

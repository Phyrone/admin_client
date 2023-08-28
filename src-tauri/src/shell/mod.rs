use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use russh::client::Handle;

struct SshHandler {}

impl SshHandler {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl russh::client::Handler for SshHandler {
    type Error = anyhow::Error;


}

async fn a()  -> Result<(),Box<dyn Error>>{
    let handler = SshHandler::new();
    let config = russh::client::Config::default();
    let client =
        russh::client::connect(Arc::new(config), ("", 22),handler).await?;
    let mut session = client.channel_open_session().await?;
    session.request_shell(true).await?;



    Ok(())


}
struct SshServerConnection {
    handler:SshHandler,
    client:Handle<SshHandler>
}

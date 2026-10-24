use std::path::PathBuf;
use iroh::{Endpoint, endpoint::presets, protocol::Router};
use iroh_blobs::{store::mem::MemStore, ticket::BlobTicket, BlobsProtocol};

pub struct Node {
    router: Router,
    store: MemStore,
}

impl Node {
    pub async fn start() -> anyhow::Result<Self> {
        let endpoint = Endpoint::bind(presets::N0).await?;
        let store = MemStore::new();
        let blobs = BlobsProtocol::new(&store, None);
        let router = Router::builder(endpoint)
            .accept(iroh_blobs::ALPN, blobs)
            .spawn();
        Ok(Self { router, store })
    }

    pub fn id(&self) -> String {
        self.router.endpoint().id().to_string()
    }

    pub async fn share(&self, path: PathBuf) -> anyhow::Result<String> {
        let path = std::fs::canonicalize(path)?; // iroh-blobs wants absolute paths
        let tag = self.store.blobs().add_path(path).await?;
        let ep = self.router.endpoint();
        ep.online().await;
        Ok(BlobTicket::new(ep.addr(), tag.hash, tag.format).to_string())
    }

pub async fn receive(&self, ticket: &str, dest: PathBuf) -> anyhow::Result<()> {
    let ticket: BlobTicket = ticket.parse()?;
    // absolute() rather than canonicalize(): dest doesn't exist yet
    let dest = std::path::absolute(dest)?;

    // Fetch the blob from the sender into our local store
    let downloader = self.store.downloader(self.router.endpoint());
    downloader
        .download(ticket.hash(), Some(ticket.addr().id))
        .await?;

    // Now it's local, so we can write it out
    self.store.blobs().export(ticket.hash(), dest).await?;
    Ok(())
}}

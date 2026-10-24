#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("usage: send <file>");
    let node = shuttle_core::Node::start().await?;
    println!("{}", node.share(path.into()).await?);
    tokio::signal::ctrl_c().await?;
    Ok(())
}

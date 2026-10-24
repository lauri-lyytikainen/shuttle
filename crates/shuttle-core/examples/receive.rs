#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let ticket = args.next().expect("usage: receive <ticket> <out>");
    let out = args.next().expect("usage: receive <ticket> <out>");
    let node = shuttle_core::Node::start().await?;
    node.receive(&ticket, out.into()).await?;
    println!("done");
    Ok(())
}

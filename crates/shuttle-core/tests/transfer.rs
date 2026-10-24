#[tokio::test]
async fn transfer_between_two_nodes() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src = dir.path().join("in.txt");
    std::fs::write(&src, b"hello shuttle")?;

    let a = shuttle_core::Node::start().await?;
    let b = shuttle_core::Node::start().await?;

    let ticket = a.share(src).await?;
    let out = dir.path().join("out.txt");
    b.receive(&ticket, out.clone()).await?;

    assert_eq!(std::fs::read(out)?, b"hello shuttle");
    Ok(())
}

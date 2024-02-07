#![allow(unused)] //for beginning
                  //
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let base_url = "http://localhost:8080";

    let hc = httpc_test::new_client(base_url)?;
    hc.do_get("/hello2/Vincent").await?.print().await?;
    Ok(())
}

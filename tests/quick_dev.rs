#![allow(unused)] //for beginning
                  //
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let base_url = "http://localhost:8080";

    let hc = httpc_test::new_client(base_url)?;

    hc.do_get("/hello2/Vincent").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
        "username" :"toto",
        "password" : "123456"
        }),
    );

    req_login.await?.print().await?;
    let req_create_ticket = hc.do_post("/api/tickets", json!({"title" : "my title"}));

    req_create_ticket.await?.print().await?;

    //hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
}

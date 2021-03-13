#[cfg(test)]
use super::*;

#[tokio::test]
async fn test_new_with_pass() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    assert_eq!(op_cli.session.len(), 44);
}

#[tokio::test]
async fn test_account() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let account = op_cli.get().account().run().await;
    println!("{:?}", &account);
    assert!(account.is_ok())
}

#[tokio::test]
async fn test_account_flags() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let account = op_cli
        .get()
        .account()
        .add_flag(&["--include-trash"])
        .run()
        .await;
    println!("{:?}", &account);
    assert!(account.is_ok())
}

#[tokio::test]
async fn test_item_lite() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let account = op_cli.get().item_lite("facebook").run().await;
    let otps = op_cli.get().totp("facebook").run().await;
    println!("{:?},otps:{:?}", &account, &otps);
    assert!(account.is_ok())
}

#[tokio::test]
async fn test_get_item() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let account = op_cli.get().item("facebook").run().await;
    println!("{:?}", &account);
    assert!(account.is_ok())
}

#[tokio::test]
async fn test_create_document() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let account = op_cli.create().document("newnew_json.json").run().await;
    println!("{:?}", &account);
    assert!(account.is_ok())
}

#[tokio::test]
async fn test_get_document() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let doc = op_cli.get().document("new_doc.txt").run().await;
    println!("{:?}", &doc);
    assert!(doc.is_ok())
}

#[tokio::test]
async fn test_get_totp() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let doc = op_cli.get().totp("facebook").run().await;
    println!("{:?}", &doc);
    assert!(doc.is_ok())
}

#[tokio::test]
async fn test_list_documents() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let doc = op_cli.list().documents().run().await;
    println!("{:?}", &doc);
    assert!(doc.is_ok())
}

#[tokio::test]
async fn test_list_items() {
    dotenv::dotenv().unwrap();
    let pass = dotenv::var("OP_PASS").unwrap();
    let op_cli = OpCLI::new_with_pass("my".to_string(), pass, false)
        .await
        .unwrap();
    let doc = op_cli.list().items().run().await;
    println!("{:?}", &doc);
    assert!(doc.is_ok())
}
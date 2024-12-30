use spread_sheet::{Authenticator, SheetsClient};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // サービスアカウントの認証情報を読み込む
    let auth = Authenticator::from_service_account_file("google-credential.json")?;
    
    // シートクライアントを初期化
    let client = SheetsClient::new(auth);
    
    // API疎通テストを実行
    println!("APIへの疎通テストを開始します...");
    match client.ping_api().await {
        Ok(response) => println!("テスト成功: {}", response),
        Err(e) => println!("テスト失敗: {}", e),
    }
    
    Ok(())
}

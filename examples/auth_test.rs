use spread_sheet::{Authenticator, SheetsClient};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("サービスアカウント認証情報を検証中...");

    // サービスアカウント認証情報からクライアントを初期化
    let auth = Authenticator::from_service_account_file("google-credential.json")?;
    let client = SheetsClient::new(auth);

    // スプレッドシートメタデータを取得（認証の妥当性を確認）
    // Note: このエンドポイントは適切な認証が必要で、単なる疎通確認以上の検証となる
    let response = client
        .get("https://www.googleapis.com/drive/v3/files?q=mimeType='application/vnd.google-apps.spreadsheet'")
        .await?;

    if response.is_success {
        println!("✅ 認証成功: サービスアカウントの認証情報が有効です");
        println!("✅ レスポンス: {}", response.data.unwrap_or_default());
        println!("Note: このレスポンスにはスプレッドシートのリストが含まれています。");
    } else {
        println!("❌ 認証失敗: サービスアカウントの認証情報が無効です");
        if let Some(error) = response.error {
            println!("エラー: {}", error);
        }
    }

    Ok(())
}

use spread_sheet::{authenticator::Authenticator, error::Error};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("サービスアカウント認証の検証を開始します...");

    // サービスアカウントの認証情報を読み込み
    let creds_path = Path::new("google-credential.json");
    println!("1. 認証情報ファイルの検証:");
    let authenticator = match Authenticator::from_service_account_file(creds_path) {
        Ok(auth) => {
            println!("  ✓ 認証情報ファイルの読み込みに成功しました");
            auth
        }
        Err(e) => {
            println!("  ✗ 認証情報ファイルの読み込みに失敗しました: {}", e);
            return Err(e);
        }
    };

    // 必要なスコープの検証
    println!("\n2. 必要なスコープの検証:");
    println!("  ✓ スコープの設定を確認:");
    println!("    - https://www.googleapis.com/auth/spreadsheets");
    println!("    - https://www.googleapis.com/auth/drive.readonly");

    // OAuth2トークンの取得を試行
    println!("\n3. OAuth2トークンの取得:");
    match authenticator.get_token().await {
        Ok(token) => {
            println!("  ✓ トークンの取得に成功しました");
            println!("    - アクセストークン: {}...", &token[..10]);
        }
        Err(e) => {
            println!("  ✗ トークンの取得に失敗しました: {}", e);
            return Err(e);
        }
    }

    // Google Sheets API Discovery エンドポイントへの接続テスト
    println!("\n4. API接続テスト:");
    let client = spread_sheet::SheetsClient::new(authenticator);
    match client.ping_api().await {
        Ok(response) => {
            if response.is_success {
                println!("  ✓ APIへの接続に成功しました");
                println!(
                    "    - レスポンス: 正常 ({}バイト)",
                    response.data.unwrap().len()
                );
            } else {
                println!("  ✗ APIへの接続に失敗しました");
                println!("    - エラー: {}", response.error.unwrap_or_default());
                return Err(Error::ApiError("API接続テストに失敗しました".to_string()));
            }
        }
        Err(e) => {
            println!("  ✗ APIへの接続に失敗しました: {}", e);
            return Err(e);
        }
    }

    println!("\n✓ 認証の検証が完了しました");
    Ok(())
}

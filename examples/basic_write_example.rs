use spread_sheet::{Authenticator, SheetsClient, SpreadsheetReader, SpreadsheetWriter};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Google Sheets APIの認証情報を環境変数またはファイルから取得
    let cred = env::var("GOOGLE_CRED").unwrap_or_else(|_| {
        // 環境変数が設定されていない場合、ローカルファイルから読み込む
        std::fs::read_to_string("google-credential.json")
            .expect("Failed to read local google-credential.json")
    });
    let client = SheetsClient::new(Authenticator::new(Some(cred)));
    let writer = SpreadsheetWriter::new(client.clone());
    let reader = SpreadsheetReader::new(client);

    // テスト用スプレッドシートの情報
    // ファイル名: spread_sheet_rs
    // シート名: シート1, Query Result Dec 13 2024 (1)
    let sheet_id = "1OU4eEeDargcZTPaW7O5FNYE_vyrUQRysGCVYzAiOChQ";

    // シート1のA1セルに書き込み
    println!("シート1のA1セルに書き込みを実行中...");
    match writer
        .write_cell(sheet_id, "シート1!A1", "Hello from writer!")
        .await
    {
        Ok(response) => {
            if response.is_success {
                println!("書き込み成功: {:?}", response.data);
                // 書き込んだデータを読み込んで確認
                println!("\n書き込んだデータを確認中...");
                match reader.read_range(sheet_id, "シート1!A1").await {
                    Ok(read_response) => {
                        if read_response.is_success {
                            println!("読み込み成功: {:?}", read_response.data);
                        } else if let Some(error) = read_response.error {
                            println!("読み込み時にエラーが発生しました: {}", error);
                        }
                    }
                    Err(e) => println!("読み込み時にエラーが発生しました: {}", e),
                }
            } else if let Some(error) = response.error {
                println!("書き込み時にエラーが発生しました: {}", error);
            }
        }
        Err(e) => println!("書き込み時にエラーが発生しました: {}", e),
    }

    Ok(())
}

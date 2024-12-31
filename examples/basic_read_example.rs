use spread_sheet::{Authenticator, SheetsClient, SpreadsheetReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 認証情報の設定
    // サービスアカウント認証情報を使用
    let auth = Authenticator::from_service_account_file("google-credential.json")
        .expect("Failed to create authenticator from service account file");
    let client = SheetsClient::new(auth);
    let reader = SpreadsheetReader::new(client);

    // スプレッドシートIDとシート名を指定
    let spreadsheet_id = "1OU4eEeDargcZTPaW7O5FNYE_vyrUQRysGCVYzAiOChQ";
    let sheet_name = "read_sheet"; // シート名を"read_sheet"に変更

    println!("シート全体の読み込みを試行中...");
    match reader.read_entire_sheet(spreadsheet_id, sheet_name).await {
        Ok(response) => {
            if response.is_success {
                println!("データの読み込みに成功しました:");
                if let Some(data) = response.data {
                    // データを表形式で表示
                    println!("----------------------------------------");
                    for (row_idx, row) in data.iter().enumerate() {
                        print!("行 {:2}: ", row_idx + 1);
                        for cell in row {
                            print!("{:<15}", cell); // 15文字幅で左寄せ
                        }
                        println!(); // 改行
                    }
                    println!("----------------------------------------");
                } else {
                    println!("データが空です");
                }
            } else if let Some(error) = response.error {
                println!("エラーが発生しました: {}", error);
            }
        }
        Err(e) => println!("エラーが発生しました: {}", e),
    }

    Ok(())
}

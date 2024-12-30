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
    let sheet_name = "シート1";

    println!("シート全体の読み込みを試行中...");
    match reader.read_entire_sheet(spreadsheet_id, sheet_name).await {
        Ok(response) => {
            if response.is_success {
                println!("データの読み込みに成功しました:");
                if let Some(data) = response.data {
                    println!("{}", data);
                }
            } else if let Some(error) = response.error {
                println!("エラーが発生しました: {}", error);
            }
        }
        Err(e) => println!("エラーが発生しました: {}", e),
    }

    println!("\n特定範囲の読み込みを試行中...");
    let range = "シート1!A1:B10"; // 例: A1からB10までの範囲
    match reader.read_range(spreadsheet_id, range).await {
        Ok(response) => {
            if response.is_success {
                println!("データの読み込みに成功しました:");
                if let Some(data) = response.data {
                    println!("{}", data);
                }
            } else if let Some(error) = response.error {
                println!("エラーが発生しました: {}", error);
            }
        }
        Err(e) => println!("エラーが発生しました: {}", e),
    }

    Ok(())
}

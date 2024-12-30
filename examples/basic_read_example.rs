use spread_sheet_rs::{Authenticator, Error, Response, SheetsClient, SpreadsheetReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 認証情報の設定
    // Note: 実際の使用時はGOOGLE_SA_SHEET_CREDなどの環境変数から認証情報を取得します
    let auth = Authenticator::new(Some("your_api_key".to_string()));
    let client = SheetsClient::new(auth);
    let reader = SpreadsheetReader::new(client);

    // スプレッドシートIDとシート名を指定
    let spreadsheet_id = "your_spreadsheet_id";
    let sheet_name = "Sheet1";

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
    let range = "Sheet1!A1:B10"; // 例: A1からB10までの範囲
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

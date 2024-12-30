use spread_sheet::{Authenticator, SheetsClient, SpreadsheetReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 認証情報の設定
    // 環境変数GOOGLE_CREDまたはローカルファイルから認証情報を取得
    let cred = std::env::var("GOOGLE_CRED").unwrap_or_else(|_| {
        // 環境変数が設定されていない場合、ローカルファイルから読み込む
        std::fs::read_to_string("google-credential.json")
            .expect("Failed to read local google-credential.json")
    });
    let auth = Authenticator::new(Some(cred));
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

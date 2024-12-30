use spread_sheet::{Authenticator, SheetsClient, SpreadsheetWriter};
use std::env;

fn main() {
    // Google Sheets APIの認証情報を環境変数から取得
    let cred = env::var("GOOGLE_SA_SHEET_CRED").expect("GOOGLE_SA_SHEET_CRED must be set");
    let client = SheetsClient::new(Authenticator::new(Some(cred)));
    let writer = SpreadsheetWriter::new(client);

    // テスト用スプレッドシートのID
    let sheet_id = "1OU4eEeDargcZTPaW7O5FNYE_vyrUQRysGCVYzAiOChQ";
    match writer.write_cell(sheet_id, "A1", "Hello from writer!") {
        Ok(response) => println!("書き込み成功: {:?}", response.data),
        Err(e) => eprintln!("エラーが発生しました: {:?}", e),
    }
}

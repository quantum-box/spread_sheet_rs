use spread_sheet::{Authenticator, SheetsClient, SpreadsheetWriter};
use std::env;

fn main() {
    // Google Sheets APIの認証情報を環境変数から取得
    let cred = env::var("GOOGLE_SA_SHEET_CRED").expect("GOOGLE_SA_SHEET_CRED must be set");
    let client = SheetsClient::new(Authenticator::new(Some(cred)));
    let writer = SpreadsheetWriter::new(client);

    // テスト用スプレッドシートの情報
    // ファイル名: spread_sheet_rs
    // シート名: シート1, Query Result Dec 13 2024 (1)
    let sheet_id = "1OU4eEeDargcZTPaW7O5FNYE_vyrUQRysGCVYzAiOChQ";

    // シート1のA1セルに書き込み
    match writer.write_cell(sheet_id, "シート1!A1", "Hello from writer!") {
        Ok(response) => println!("書き込み成功: {:?}", response.data),
        Err(e) => eprintln!("エラーが発生しました: {:?}", e),
    }
}

use spread_sheet::{Authenticator, SheetsClient, SpreadsheetWriter};

fn main() {
    // 認証情報のセットアップは本来ここに必要
    let client = SheetsClient::new(Authenticator::new(None)); // 簡易化のためAPI keyなしで生成
    let writer = SpreadsheetWriter::new(client);

    let sheet_id = "YOUR_SHEET_ID";
    match writer.write_cell(sheet_id, "A1", "Hello from writer!") {
        Ok(response) => println!("OK: {:?}", response.data),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}

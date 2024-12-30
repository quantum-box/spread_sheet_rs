use spread_sheet::{SheetsClient, SpreadsheetWriter};

fn main() {
    // 認証情報のセットアップは本来ここに必要
    let client = SheetsClient::default(); // 簡易化のためデフォルト生成としている
    let writer = SpreadsheetWriter::new(client);

    let sheet_id = "YOUR_SHEET_ID";
    match writer.write_cell(sheet_id, "A1", "Hello from writer!") {
        Ok(response) => println!("OK: {:?}", response.data),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}

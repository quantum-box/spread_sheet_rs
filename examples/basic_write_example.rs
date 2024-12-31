use chrono::Local;
use spread_sheet::{
    spreadsheet_writer::ValueInputOption, Authenticator, SheetsClient, SpreadsheetReader,
    SpreadsheetWriter,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Google Sheets APIの認証情報を環境変数から取得
    let cred = env::var("GOOGLE_SA_SHEET_CRED").expect("GOOGLE_SA_SHEET_CREDが設定されていません");
    let client = SheetsClient::new(Authenticator::new(Some(cred)));
    let writer = SpreadsheetWriter::new(client.clone());
    let reader = SpreadsheetReader::new(client);

    // テスト用スプレッドシートの情報
    let sheet_id = "1OU4eEeDargcZTPaW7O5FNYE_vyrUQRysGCVYzAiOChQ";
    let sheet_name = "write_sheet";

    // 現在のデータを読み込んで最終行を確認
    println!("現在のシートデータを読み込み中...");
    let read_response = reader.read_entire_sheet(sheet_id, sheet_name).await?;

    let next_row = if let Some(data) = read_response.data {
        // 既存のデータがある場合は最終行の次の行に書き込む
        data.len() + 1
    } else {
        // データがない場合は1行目から書き込む
        1
    };

    // 現在のタイムスタンプと値を含む新しい行を作成
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let test_value = format!("テスト書き込み ({})", timestamp);
    let range = format!("{}!A{}", sheet_name, next_row);

    // 新しい行にデータを書き込み
    println!("{}行目に書き込みを実行中...", next_row);
    let write_response = writer
        .write_range(
            sheet_id,
            &range,
            vec![vec![timestamp, test_value]],
            Some(ValueInputOption::UserEntered),
        )
        .await?;

    if write_response.is_success {
        println!("書き込み成功: {:?}", write_response.data);

        // 書き込んだデータを読み込んで確認
        println!("\n書き込んだデータを確認中...");
        let verify_response = reader.read_range(sheet_id, &range).await?;

        if verify_response.is_success {
            if let Some(data) = verify_response.data {
                println!("読み込み成功:");
                println!("タイムスタンプ: {}", data[0][0]);
                println!("書き込み値: {}", data[0][1]);
            } else {
                println!("警告: データが読み取れませんでした");
            }
        } else if let Some(error) = verify_response.error {
            println!("読み込み時にエラーが発生しました: {}", error);
        }
    } else if let Some(error) = write_response.error {
        println!("書き込み時にエラーが発生しました: {}", error);
    }

    Ok(())
}

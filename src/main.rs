use std::net::TcpListener;

fn main() {
    // 入力ストリームをリッスンし、ストリームを受け付けた時にメッセージを出力する

    // ポートに接続。失敗したらプログラムを停止する
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 各TcpStreamを順番に処理する
    for stream in listener.incoming() {
        // 接続の試行に失敗したら終了。ex) OS側の接続数上限を超えた場合など
        let stream = stream.unwrap();

        // 接続が確立しました
        println!("Connection established!");
    }
}
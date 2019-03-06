use std::io::prelude::*; // ストリームから読み書きさせてくれるトレイト
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // 入力ストリームをリッスンし、ストリームを受け付けた時にメッセージを出力する

    // ポートに接続。失敗したらプログラムを停止する
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 各TcpStreamを順番に処理する
    for stream in listener.incoming() {
        // 接続の試行に失敗したら終了。ex) OS側の接続数上限を超えた場合など
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// TcpStreamから読み取り、データを出力する
//   mut stream: TcpStream 内部の書き込み領域をずらすため(?)可変である必要がある
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 読み取ったデータを保持するバッファー

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // バイトが全て接続に書き込まれるまでプログラムを待機させる
}
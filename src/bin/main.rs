extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*; // ストリームから読み書きさせてくれるトレイト
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    // 入力ストリームをリッスンし、ストリームを受け付けた時にメッセージを出力する

    // ポートに接続。失敗したらプログラムを停止する
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // 設定可能なスレッド数で新しいスレッドプールを作成

    // 各TcpStreamを順番に処理する
    for stream in listener.incoming() {
        // 接続の試行に失敗したら終了。ex) OS側の接続数上限を超えた場合など
        let stream = stream.unwrap();

        // 各ストリームに対してプール内のスレッドにで処理する
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// TcpStreamから読み取り、データを出力する
//   mut stream: TcpStream 内部の書き込み領域をずらすため(?)可変である必要がある
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 読み取ったデータを保持するバッファー
    stream.read(&mut buffer).unwrap();

    // リクエストとマッチさせ、/ へのリクエストを他のリクエストとは異なる形で扱う
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // 5秒間スリープすることで遅いリクエストをシミュレーションする
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // ファイルの中身を読み込む

    let response = format!("{}{}", status_line, contents); // レスポンスの内容に追記する

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // バイトが全て接続に書き込まれるまでプログラムを待機させる
}
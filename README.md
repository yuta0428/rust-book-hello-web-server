# Final Project: Building a Multithreaded Web Server
日本訳: https://y-yu.github.io/trpl-2nd-pdf/book.pdf

実装ごとにコミット分けてあります。

[20.1.1 TCP 接続をリッスンする](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/90f117189d6e37b8305bc25c58a89c642530f251)

[20.1.2 リクエストを読み取る](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/eefa48eb2af595b882e8e8e6497409a92da46cc7)

[20.1.4 レスポンスを記述する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/23d78301559d3bc170b91774f9889b240315a643)

[20.1.5 本物のHTMLを返す](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/940e626e109fbc3bc6bb11a61eb3c5a74fd21521)

[20.1.6 リクエストにバリデーションをかけ、選択的にレスポンスを返す](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/a888f1f1b7ac205a0290d306fd151f01ade29bc6)

[20.1.7 リファクタリングの触り](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/6bcf0741c72a5cf0c158032ae8ed8eecb1315eaa)

[20.2.1 現在のサーバの実装で遅いリクエストをシミュレーションする](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/d892a5cdbcbb6d3352ac93b8d4ff927b6183917b)

[20.2.2.1 各リクエストに対してスレッドを立ち上げられる場合のコードの構造](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/98f4e37f03abc67f00a18991e41c80f35def5f16)

[20.2.2.2 有限数のスレッド用に似たインターフェイスを作成する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/1d61b179e93a7f7e3be5d2d2e5f94c06cc644a80)

[20.2.2.3 コンパイラ駆動開発でThreadPool 構造体を構築する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/0461a66da35521d1b26cd0adf57cec521bf839b6)

[20.2.2.4 newでスレッド数を検査する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/5a896d81c1e9436003c7d83ae4ea05d542ec1a32)

[20.2.2.5 スレッドを格納するスペースを生成する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/92e839a5a89237b5b1cd709b69a163577446350a)

[20.2.2.6 ThreadPoolからスレッドにコードを送信する責任を負うWorker構造体](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/194ae9ae7b10b63f5540efdc67ba8d829eda55d0)

[20.2.2.7 チャンネル経由でスレッドにリクエストを送信する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/5ca3e835911b03332db3c86f2a5d5fe333feb1bf)

[20.2.2.8 executeメソッドを実装する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/dabdea747b9b56ec13fa3a490a653ce52011be9b)

[20.3.1 ThreadPoolにDropトレイトを実装する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/02d1d4e1c1c65bbf070a5e7b9ff1c7a993f3fc5c)

[20.3.2 スレッドに仕事をリッスンするのを止めるよう通知する](https://github.com/yuta0428/rust-book-multithreaded-web-server/commit/2824af9a3a5a3b05e49483373507f09fe8fc0d72)


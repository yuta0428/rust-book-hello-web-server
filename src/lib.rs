use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job), // 実行すべきJobか
    Terminate,   // ループから抜けて停止させるか
}

// スレッドプール構造体
pub struct ThreadPool {
    workers: Vec<Worker>, // スレッドを直接保持するのではなく、Workerインスタンスを保持する
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()  // Box<T>からクロージャをムーブし、クロージャを呼び出す
    }
}

type Job = Box<FnBox + Send + 'static>;


impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    ///
    /// sizeがプールのスレッド数です。
    ///
    /// # パニック
    ///
    /// sizeが0なら、`new`関数はパニックします。
    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // チャンネルを生成
        let (sender, receiver) = mpsc::channel();

        // 全ワーカー間で単独のreceiverを共有し、チャンネルキューから仕事を取りだす（receiverを可変化する）
        //  複数のスレッドで所有権を共有しつつ、スレッドに値を可変化させる -> スレッド安全なスマートポインタにさせる
        //    Arc: 複数の参照間で値の所有権を同時に共有できるようにする型
        //    Mutex: 同時に一つのスレッドのなかでしか値は変更できないということを保証できる型
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); // sizeキャパシティで初期化

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // チャンネルの受信側をワーカーに渡す
        }

        ThreadPool {
            workers,
            sender, // チャンネルの送信側をThreadPoolインスタンスに保持する
        }
    }

    // thread::spawnに変わるインターフェース
    pub fn execute<F>(&self, f: F)
        where
            // Send: トレイト境界。あるスレッドから別のスレッドにクロージャを移動するため
            // 'static: ライフタイム境界。スレッドの実行にどれくらいかかるかわからないため
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    // スレッドプールがスコープを抜けた時にスレッドをjoinさせる
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // 全ワーカーを閉じます
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            // ワーカー{}を終了します
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// idとJoinHandle<()>を保持するWorker構造体
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // 実行中のworkerがあるか確認するためOption<> None:なし
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            // Rustではunlockの代わりにライフタイムを使用している。
            // ↓書き方だと1loopのスコープが終わるまでlockされるため並行にならない。
            //  while let Ok(job) = receiver.lock().unwrap().recv() {
            loop {
                // ワーカーのスレッドでメッセージを受け取る
                let message = receiver.lock().unwrap().recv().unwrap(); // こっちはここでunlockされる

                match message {
                    Message::NewJob(job) => {
                        // ワーカー{}は仕事を得ました; 実行します
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        // ワーカー{}は停止するよう指示された
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });
        // idとスレッドを保持するWorkerインスタンスを返す
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

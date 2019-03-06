use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// スレッドプール構造体
pub struct ThreadPool {
    workers: Vec<Worker>, // スレッドを直接保持するのではなく、Workerインスタンスを保持する
    sender: mpsc::Sender<Job>,
}

struct Job;

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

    }
}

// idとJoinHandle<()>を保持するWorker構造体
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        // idとスレッドを保持するWorkerインスタンスを返す
        Worker {
            id,
            thread,
        }
    }
}
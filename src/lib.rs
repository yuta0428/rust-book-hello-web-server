// スレッドプール構造体
pub struct ThreadPool;

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

        ThreadPool
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
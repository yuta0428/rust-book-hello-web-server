// スレッドプール構造体
pub struct ThreadPool;

impl ThreadPool {
    // size: スレッド数
    pub fn new(size: usize) -> ThreadPool {
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
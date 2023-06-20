#[macro_export]
macro_rules! async_main {
    ($($tt:tt)*) => {
        use tokio::runtime::Builder;
        fn main() {
            let mut runtime = Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async move { $($tt)* });
        }
    };
}

#[macro_export]
macro_rules! spawn {
    ($($tt:tt)*) => {
        tokio::task::spawn($($tt)*)
    };
}

#[macro_export]
macro_rules! spawn_async {
    ($($tt:tt)*) => {
        tokio::spawn(async move { $($tt)* });
    };
}

#[macro_export]
macro_rules! spawn_delayed {
    ($delay:expr, $($tt:tt)*) => {
        use tokio;
        tokio::spawn(async move {
            sleep($delay).await;
            $($tt)*;
        });
    };
}

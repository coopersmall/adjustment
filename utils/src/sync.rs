pub mod spawn {

    /// Spawns a task with the specified execution semantics.
    ///
    /// # Arguments
    ///
    /// * `async move $($tt:tt)*` - An `async` closure with `move` semantics.
    /// * `move $($tt:tt)*` - A closure with `move` semantics.
    /// * `async $($tt:tt)*` - An `async` closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::spawn;
    ///
    /// #[tokio::test]
    /// async fn test() {
    ///    spawn!(async move {
    ///    // Asynchronous code with move semantics
    ///    // ...
    ///    });
    ///    
    ///    spawn!(move {
    ///    // Synchronous code with move semantics
    ///    // ...
    ///    });
    ///
    ///    spawn!(async {
    ///    // Asynchronous code
    ///    // ...
    ///    });
    /// }
    /// ```
    ///
    ///
    #[macro_export]
    macro_rules! spawn {
      (async move $($tt:tt)*) => {
          tokio::spawn(async move { $($tt)* });
      };
      (move $($tt:tt)*) => {
          tokio::spawn(async { $($tt)* });
      };
      (async $($tt:tt)*) => {
          tokio::spawn(async { $($tt)* });
      };
    }

    /// Spawns a thread task with a delay before execution.
    ///
    /// # Arguments
    ///
    /// * `$delay:expr` - The delay duration before executing the task.
    /// * `$($tt:tt)*` - The code block representing the task to be executed.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::spawn_delayed;
    /// use tokio::time::Duration;
    ///
    /// #[tokio::test]
    /// async fn test() {
    ///    spawn_delayed!(Duration::from_secs(2), {
    ///    // Code block to be executed after a 2-second delay
    ///    // ...
    ///    });
    ///    // ...
    /// }
    /// ```
    ///
    ///
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
}

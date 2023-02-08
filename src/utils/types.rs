pub type BufferReference<'a> = &'a [u8];

pub type FutureResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type ThreadResult<T> = Result<std::thread::JoinHandle<T>, Box<dyn std::error::Error + 'static>>;

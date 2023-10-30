use std::sync::*;
use std::thread::JoinHandle;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Thread<T: Send + 'static>(Option<JoinHandle<T>>);

impl<T: Send + 'static> Default for Thread<T> {
    fn default() -> Self { Self { 0: None } }
}

impl<T: Send + 'static> Thread<T> {
    pub fn reset(&mut self) { self.0 = None; }

    pub fn spawn<F: FnOnce() -> T + Send + 'static>(&mut self, f: F) {
        self.0 = Some(std::thread::spawn(f));
    }

    pub fn join_if_finished(&mut self) -> Option<T> {
        if self.is_finished() { Some(self.join()) } else { None }
    }

    pub fn join(&mut self) -> T {
        let value = self.0.take().unwrap().join().unwrap();
        self.0 = None;
        value
    }

    pub fn is_finished(&self) -> bool {
        if let Some(join_handle) = &self.0 {
            if join_handle.is_finished() { return true; }
        }

        false
    }

    pub fn is_busy(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ThreadWorker<T: Sync + Send + 'static> {
    ptr: Arc<Mutex<Vec<T>>>,
    thread: Thread<()>,
}

impl<T: Sync + Send + 'static> Default for ThreadWorker<T> {
    fn default() -> Self { Self { ptr: Arc::new(Mutex::new(vec![])), thread: Thread::default() } }
}

impl<T: Sync + Send + 'static> ThreadWorker<T> {
    pub fn start<F: FnOnce(Arc<Mutex<Vec<T>>>) + Send + 'static>(&mut self, f: F) {
        let arc = self.ptr.clone();
        self.thread.spawn(|| { f(arc) });
    }
    
    pub fn reset(&mut self) {
        self.ptr = Arc::new(Mutex::new(vec![]));
        self.thread.reset();
    }

    pub fn get_data_if_finished(&mut self) -> Option<MutexGuard<Vec<T>>> {
        if self.thread.is_finished() {
            Some(self.ptr.lock().unwrap())
        } else {
            None
        }
    }

    pub fn is_busy(&self) -> bool {
        self.thread.is_busy()
    }
}
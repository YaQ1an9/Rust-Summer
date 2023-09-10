use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Waker, RawWaker, Context, Poll, Wake};
use std::task::RawWakerVTable;
use std::time::Duration;
use async_channel;
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use std::sync::{Condvar, Mutex, Arc};
use scoped_tls::scoped_thread_local;
use pin_utils::pin_mut;

scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);

struct Signal{
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Waiting,
    Notified,
}

struct Task {
    future: RefCell<LocalBoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        println!("wake");
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

impl Signal {
    fn new() -> Self {
        Signal {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("multiple wait");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }

    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Empty => *state = State::Notified,
            State::Notified => {}
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}
struct Demo;

impl Future for Demo {
    type Output = ();
    
    fn poll(self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>
            ) -> std::task::Poll<Self::Output> {
            println!("Hello, world!");
            std::task::Poll::Ready(())  
            }
}
fn dummy_waker() -> Waker {
    static Data: () = ();
    unsafe { Waker::from_raw(RawWaker::new(&Data, &VTABLE)) }
}

const VTABLE: RawWakerVTable = 
    RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);

unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(_p, &VTABLE)
}

unsafe fn vtable_wake(_p: *const ()) {

}
unsafe fn vtable_wake_by_ref(_p: *const ()) {

}
unsafe fn vtable_drop(_p: *const ()) {

}

async fn demo() {
    let (tx, rx) = async_channel::bounded(1);
    let (tx_2, rx_2) = async_channel::bounded(1);
    spawn(demo2(tx, rx_2));
    println!("Hello, world!");
    let _ = rx.recv().await;
    let _ = tx_2.send(()).await;
}
async fn demo2(tx: async_channel::Sender<()>, rx_2: async_channel::Receiver<()>) {
    println!("Hello, world2!");
    let _ = tx.send(()).await;
    let _ = rx_2.recv().await;
}

fn main() {
    let future = demo();
    block_on(future);
}

fn block_on<F: Future> (future: F) -> F::Output {
    let fut = future;
    pin_utils::pin_mut!(fut);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());

    let mut cx = Context::from_waker(&waker);
    let runnable = Mutex::new(VecDeque::with_capacity(1024));

    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || {
            loop {
                if let  Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = runnable.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
} 
fn spawn(fut: impl Future<Output = ()> + 'static) {
    let task = Arc::new(Task {
        future: RefCell::new(fut.boxed_local()),
        signal: SIGNAL.with(|signal| signal.clone()),
    });
    RUNNABLE.with(|runnable| runnable.try_lock().unwrap().push_back(task));
}

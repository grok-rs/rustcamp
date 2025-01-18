use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread;

// 1. OnlySync: Sync but not Send
struct OnlySync {
    _marker: PhantomData<*const ()>, // Raw pointer to prevent Send
}

unsafe impl Sync for OnlySync {} // Manually implementing Sync

// 2. OnlySend: Send but not Sync
struct OnlySend {
    _marker: PhantomData<*const ()>, // Raw pointer to prevent Sync
}

unsafe impl Send for OnlySend {} // Manually implementing Send

// 3. SyncAndSend: Sync and Send
struct SyncAndSend;

unsafe impl Sync for SyncAndSend {}
unsafe impl Send for SyncAndSend {}

// 4. NotSyncNotSend: Neither Sync nor Send
#[derive(Debug)]
struct NotSyncNotSend {
    data: UnsafeCell<i32>,
    _not_send_or_sync: PhantomData<*const ()>, // Marker to prevent Send and Sync
}

fn main() {
    // Test OnlySync
    let only_sync = Arc::new(OnlySync {
        _marker: PhantomData,
    });
    let thread1 = thread::spawn({
        let only_sync_ref = Arc::clone(&only_sync);
        move || {
            // This works because OnlySync is Sync
            println!("OnlySync is accessible in this thread.");
        }
    });
    thread1.join().unwrap();

    // Test OnlySend
    let only_send = OnlySend {
        _marker: PhantomData,
    };
    let thread2 = thread::spawn(move || {
        // This works because OnlySend is Send
        println!("OnlySend is accessible in this thread.");
    });
    thread2.join().unwrap();

    // Test SyncAndSend
    let sync_and_send = Arc::new(SyncAndSend);
    let thread3 = thread::spawn({
        let sync_and_send_ref = Arc::clone(&sync_and_send);
        move || {
            // This works because SyncAndSend is both Sync and Send
            println!("SyncAndSend is accessible in this thread.");
        }
    });
    thread3.join().unwrap();

    // Test NotSyncNotSend
    let not_sync_not_send = NotSyncNotSend {
        data: UnsafeCell::new(42),
        _not_send_or_sync: PhantomData,
    };

    // Uncommenting this block will fail to compile because NotSyncNotSend is neither Send nor Sync
    //     let thread4 = thread::spawn(move || {
    //         println!(
    //             "NotSyncNotSend is accessible in this thread., {:?}",
    //             not_sync_not_send
    //         );
    //     });
    //     thread4.join().unwrap();
}

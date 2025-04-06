//! Stress tests.
#![allow(clippy::tests_outside_test_module, clippy::expect_used, reason = "Tests")]

use std::sync::{
	Arc,
	atomic::{AtomicUsize, Ordering},
};

use key_lock::KeyLock;

#[tokio::test(flavor = "multi_thread")]
async fn stress() {
	let lock = Arc::new(KeyLock::new());
	let check_a = Arc::new(AtomicUsize::new(0));
	let check_b = Arc::new(AtomicUsize::new(0));
	let n = 10000;

	let mut handles = Vec::new();
	for _ in 0 .. n {
		let lock_a = lock.clone();
		let check_a = check_a.clone();
		let handle = tokio::spawn(async move {
			let _guard = lock_a.lock("a").await;
			check_a.fetch_add(1, Ordering::Relaxed);
		});
		handles.push(handle);

		let lock_b = lock.clone();
		let check_b = check_b.clone();
		let handle = tokio::spawn(async move {
			let _guard = lock_b.lock("b").await;
			check_b.fetch_add(1, Ordering::Relaxed);
		});
		handles.push(handle);
	}
	futures::future::try_join_all(handles).await.expect("execution");

	assert_eq!(check_a.load(Ordering::Relaxed), n);
}

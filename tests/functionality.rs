//! Functionality tests.

use key_lock::KeyLock;

#[test]
#[ntest::timeout(1000)]
fn basic() {
	let lock = KeyLock::new();
	let _a = lock.blocking_lock("a");
	let _b = lock.blocking_lock("b");
	assert!(lock.blocking_try_lock("a").is_err());
	assert!(lock.blocking_try_lock("c").is_ok());
}

#[tokio::test]
#[ntest::timeout(1000)]
async fn basic_async() {
	let lock = KeyLock::new();
	let _a = lock.lock("a").await;
	let _b = lock.lock("b").await;
	assert!(lock.try_lock("a").await.is_err());
	assert!(lock.try_lock("c").await.is_ok());
}

#[test]
#[ntest::timeout(1000)]
fn clean_up() {
	let lock = KeyLock::new();
	let _a = lock.blocking_lock("a");
	lock.blocking_clean();
	assert!(lock.blocking_try_lock("a").is_err());
}

#[tokio::test]
#[ntest::timeout(1000)]
async fn clean_up_async() {
	let lock = KeyLock::new();
	let _a = lock.lock("a").await;
	lock.clean().await;
	assert!(lock.try_lock("a").await.is_err());
}

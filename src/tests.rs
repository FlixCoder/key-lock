use super::*;

#[test]
#[ntest::timeout(1000)]
fn clean_up() {
	let lock = KeyLock::new();
	lock.blocking_lock("a");
	assert_eq!(lock.locks.blocking_lock().len(), 1);
	lock.blocking_clean();
	assert_eq!(lock.locks.blocking_lock().len(), 0);
}

#[tokio::test]
#[ntest::timeout(1000)]
async fn clean_up_async() {
	let lock = KeyLock::new();
	lock.lock("a").await;
	assert_eq!(lock.locks.lock().await.len(), 1);
	lock.clean().await;
	assert_eq!(lock.locks.lock().await.len(), 0);
}

//! Utility functions regarding JWT key refresh functionality
//!
//! Provides functionality relating to JWT key refresh locks &
//! notifications of when a refresh completes.
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use log::{debug, trace};
use tokio::sync::Notify;

/// Attempts to atomically acquire the refresh lock for updating JWT keys
///
/// This method uses an atomic compare-and-exchange operation to safely acquire
/// the lock in a concurrent environment. It only succeeds if no other thread
/// currently holds the lock (i.e., the atomic flag is false).
///
/// # Implementation Details
/// The lock acquisition uses an atomic compare-exchange operation with the following properties:
/// - The operation succeeds only if the current value is `false` (no lock held)
/// - Uses `Ordering::Acquire` for successful exchanges, ensuring all subsequent memory
///   operations by this thread cannot be reordered before this lock acquisition
/// - Uses `Ordering::Relaxed` for failed exchanges since we don't care about
///   memory ordering guarantees when we fail to acquire the lock
///
/// # Thread Safety
/// This method is thread-safe and can be called concurrently from multiple threads.
/// Only one thread will successfully acquire the lock. The atomic operation ensures
/// there are no race conditions in determining which thread gets the lock.
///
/// # Returns
/// - [`true`] if the lock is acquired successfully,
/// - [`false`] if the lock is already held by another thread
pub(super) fn jwk_refresh_lock_try_acquire(refresh_lock: &Arc<AtomicBool>) -> bool {
    // Attempt to acquire a lock
    let lock_acquired = refresh_lock.compare_exchange(
        false,
        true,
        std::sync::atomic::Ordering::Acquire,
        std::sync::atomic::Ordering::Relaxed,
    );

    if !lock_acquired.is_err() {
        #[cfg(not(tarpaulin_include))]
        debug!("Successfully acquired JWT key refresh lock");

        // Lock successfully acquired
        true
    } else {
        #[cfg(not(tarpaulin_include))]
        trace!("Failed to acquire JWT key refresh lock (already held by another thread)");

        // Lock already in use
        false
    }
}

/// Releases the JWT key refresh lock and notifies any waiting threads
///
/// This method is called after a JWT key refresh operation completes (either
/// successfully or with an error). It performs two key actions:
/// 1. Releases the atomic lock that prevents concurrent refresh operations
/// 2. Notifies all threads waiting for the refresh operation to complete
///
/// # Implementation Details
/// - Uses atomic operations with [`std::sync::atomic::Ordering::Release`] to
///   ensure memory visibility
/// - Calls [`tokio::sync::Notify::notify_waiters()`] on the notification channel
///   to wake up any threads that are blocked in [`Self::wait_for_ongoing_refresh()`]
///
/// # Thread Safety
/// This method is thread-safe and uses proper memory ordering to ensure that
/// all memory operations performed during the refresh are visible to threads
/// that subsequently acquire the lock or are notified.
pub(super) fn jwk_refresh_lock_release_and_notify(
    refresh_lock: &Arc<AtomicBool>,
    refresh_notifier: &Arc<Notify>,
) {
    // Release the lock
    #[cfg(not(tarpaulin_include))]
    debug!("Releasing JWT key refresh lock");

    refresh_lock.store(false, std::sync::atomic::Ordering::Release);

    // Notify waiters
    #[cfg(not(tarpaulin_include))]
    trace!("Notifying waiters about JWT key refresh completion");

    refresh_notifier.notify_waiters();

    #[cfg(not(tarpaulin_include))]
    debug!("JWT key refresh lock released and waiters notified");
}

#[cfg(test)]
mod jwk_refresh_lock_try_acquire_tests {
    use super::jwk_refresh_lock_try_acquire;
    use crate::EsiClient;

    /// Checks that lock is properly acquired when not already in use
    ///
    /// Attempts to acquire a lock to refresh JWT keys on a new
    /// EsiClient which should return as successful (true) indicating
    /// that no other threads are currently attempting a key refresh.
    ///
    /// # Test Setup
    /// - Setup a basic EsiClient
    /// - Attempt to acquire a lock for JWT key refresh
    ///
    /// # Assertions
    /// - Assert that result is true when acquiring lock
    #[test]
    fn test_jwk_refresh_lock_try_acquire_success() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Attempt to acquire lock
        let lock_acquired = jwk_refresh_lock_try_acquire(&esi_client.jwt_key_refresh_lock);

        // Assert
        assert_eq!(lock_acquired, true)
    }

    /// Checks that lock is not acquired when already in use
    ///
    /// Acquires a lock initially and then attempts to acquire a lock
    /// again despite it already being in use which should return as
    /// unsuccessful (false).
    ///
    /// # Test Setup
    /// - Setup basic EsiClient
    /// - Acquire a lock initially
    /// - Attempt to acquire lock again
    ///
    /// # Assertions
    /// - Asserts that result is false when attempting to acquire lock
    ///   a second time indicating it is already in use.
    #[test]
    fn test_jwk_refresh_lock_try_acquire_unsuccessful() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Acquire a lock initially
        let lock = jwk_refresh_lock_try_acquire(&esi_client.jwt_key_refresh_lock);

        if !lock {
            panic!("Failed to acquire initial lock")
        }

        // Acquire lock a second time
        // Should return false indicating lock is already in use
        let lock_acquired = jwk_refresh_lock_try_acquire(&esi_client.jwt_key_refresh_lock);

        // Assert
        assert_eq!(lock_acquired, false)
    }
}

#[cfg(test)]
mod jwk_lock_release_and_notify_tests {
    use std::time::Duration;

    use crate::EsiClient;

    use super::jwk_refresh_lock_release_and_notify;

    /// Verifies that lock is successfully released & waiters are notified
    ///
    /// Acquires a lock and sets up a notification listener which listens
    /// for the notification of when the lock is released. If the notification
    /// is never received than the listener will timeout. Checks to ensure that
    /// notification was properly received as well as the lock was released
    /// without issues.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Acquire a JWT key refresh lock
    /// - Setup a notification listener
    ///
    /// # Assertions
    /// - Assert that lock has been properly acquired
    /// - Assert that lock release notification was received
    /// - Assert that lock has been properly released
    #[tokio::test]
    async fn test_jwk_refresh_lock_release_and_notify_success() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Acquire a lock
        let lock = !esi_client
            .jwt_key_refresh_lock
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err();

        // Assert that lock is in place
        assert_eq!(lock, true);

        // Create the notification future BEFORE triggering release
        let notification = esi_client.jwt_key_refresh_notifier.notified();
        let timeout = tokio::time::sleep(Duration::from_millis(50));

        // Release and notify
        jwk_refresh_lock_release_and_notify(
            &esi_client.jwt_key_refresh_lock,
            &esi_client.jwt_key_refresh_notifier,
        );

        let notified = tokio::select! {
            _ = notification => {
                // Notification received successfully
                true
            }
            _ = timeout => {
                // Timed out waiting for notification
                false
            }
        };

        // Assert that notification was received
        assert_eq!(notified, true);

        // Assert that lock has been released and can be acquired again
        let lock = !esi_client
            .jwt_key_refresh_lock
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err();

        assert_eq!(lock, true)
    }
}

use alloc::rc::Rc;
use core::{borrow::Borrow, fmt, ops::Bound};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ref<T>(pub Rc<T>);

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> fmt::Debug for Ref<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Wrapper<T: ?Sized>(pub T);

impl<T: ?Sized> Wrapper<T> {
    pub fn wrap(value: &T) -> &Self {
        // safe because Wrapper<T> is #[repr(transparent)]
        unsafe { &*(value as *const T as *const Self) }
    }

    pub fn wrap_bound(bound: Bound<&T>) -> Bound<&Self> {
        match bound {
            Bound::Included(t) => Bound::Included(Self::wrap(t)),
            Bound::Excluded(t) => Bound::Excluded(Self::wrap(t)),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}

// SAFETY: The Rc inside Ref is never exposed by the public API. Each value is
// shared between exactly two internal maps via Rc::clone, and the reference
// count is only modified in methods taking &mut self or ownership. This makes
// it safe to send Ref values across threads and share them between threads,
// provided the inner type T is itself Send/Sync.
unsafe impl<T: Send> Send for Ref<T> {}
unsafe impl<T: Sync> Sync for Ref<T> {}

impl<K, Q> Borrow<Wrapper<Q>> for Ref<K>
where
    K: Borrow<Q>,
    Q: ?Sized,
{
    fn borrow(&self) -> &Wrapper<Q> {
        // Rc<K>: Borrow<K>
        let k: &K = self.0.borrow();
        // K: Borrow<Q>
        let q: &Q = k.borrow();

        Wrapper::wrap(q)
    }
}

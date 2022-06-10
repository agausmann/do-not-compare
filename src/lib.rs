#![no_std]

use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct DoNotCompare<T>(pub T);

impl<T> DoNotCompare<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> PartialEq for DoNotCompare<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for DoNotCompare<T> {}

impl<T> Hash for DoNotCompare<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialOrd for DoNotCompare<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> Ord for DoNotCompare<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T> From<T> for DoNotCompare<T> {
    fn from(inner: T) -> DoNotCompare<T> {
        DoNotCompare(inner)
    }
}

// NOTE - Orphan rules do not allow this impl - use into_inner instead
// impl<T> From<DoNotCompare<T>> for T {
//     fn from(wrapper: DoNotCompare<T>) -> T {
//         wrapper.0
//     }
// }

impl<T> AsRef<T> for DoNotCompare<T> {
    fn as_ref(&self) -> &T {
        self.inner()
    }
}

impl<T> AsMut<T> for DoNotCompare<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<T> Deref for DoNotCompare<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T> DerefMut for DoNotCompare<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

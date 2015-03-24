//! Container abstraction.

use core::slice::SliceExt;

/// An abstraction of a container.
///
/// This does not use the standard library directly, as it provides a somewhat
/// insufficient interface.
pub trait Container<N> {
    /// Optional access
    fn get(&self, idx: usize) -> Option<&N>;

    /// Access an element immutably.
    fn index(&self, idx: usize) -> &N;

    /// Access an element mutably.
    fn index_mut(&mut self, idx: usize) -> &mut N;

    /// Execute a function on each element immutably.
    fn for_each<F: FnMut(&N)>(&self, f: F);

    /// Execute a function on each element mutably.
    fn for_each_mut<F: FnMut(&mut N)>(&mut self, f: F);
}


impl<N> Container<N> for Vec<N> {
    fn get(&self, idx: usize) -> Option<&N> { self[..].get(idx) }
    fn index(&self, idx: usize) -> &N { &self[idx] }
    fn index_mut(&mut self, idx: usize) -> &mut N { &mut self[idx] }
    fn for_each<F: FnMut(&N)>(&self, mut f: F) { for x in self { f(x) } }
    fn for_each_mut<F: FnMut(&mut N)>(&mut self, mut f: F) { for x in self { f(x) } }
}


impl<N> Container<N> for [N; 2] {
    fn get(&self, idx: usize) -> Option<&N> {
        if idx < 2 { Some(&self[idx]) }
        else { None }
    }

    fn index(&self, idx: usize) -> &N { &self[idx] }

    fn index_mut(&mut self, idx: usize) -> &mut N { &mut self[idx] }

    fn for_each<F: FnMut(&N)>(&self, mut f: F) {
        f(&self[0]);
        f(&self[1]);
    }

    fn for_each_mut<F: FnMut(&mut N)>(&mut self, mut f: F) {
        f(&mut self[0]);
        f(&mut self[1]);
    }
}

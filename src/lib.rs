/// An "entry" object corresponding to the top element of the stack.
///
/// Existence of this object guarantees that the stack is not empty.
///
/// In a vector, this is an object representing the last element.
pub struct LIFOEntry<'a, C: ?Sized>(&'a mut C);

impl<'a, C: ?Sized + Stack> LIFOEntry<'a, C> {
    /// Creates a new "entry" object from the mutable reference to the container.
    ///
    /// ## Safety
    ///
    /// The stack must not be empty.
    pub unsafe fn new(stack: &'a mut C) -> Self {
        // SAFETY: The stack is not empty, so the call is safe.
        Self(stack)
    }

    /// Pops the LIFO element from the stack.
    pub fn pop(self) -> C::Item {
        let LIFOEntry(stack) = self;
        // SAFETY: The stack is not empty by the virtue of
        // existence of the LIFOEntry object, so the call is safe.
        unsafe { stack.s_pop_unchecked() }
    }
}

impl<'a, C: ?Sized + Stack> std::ops::Deref for LIFOEntry<'a, C> {
    type Target = C::Item;

    fn deref(&self) -> &Self::Target {
        let LIFOEntry(stack) = self;
        // SAFETY: The stack is not empty, so the call is safe.
        unsafe { stack.lifo_ref_unchecked() }
    }
}

impl<'a, C: ?Sized + Stack> std::ops::DerefMut for LIFOEntry<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let LIFOEntry(stack) = self;
        // SAFETY: The stack is not empty, so the call is safe.
        unsafe { stack.lifo_mut_unchecked() }
    }
}

/// Implementors of this method can be used as a [stack] in DSA terms.
///
/// [stack]: https://www.geeksforgeeks.org/stack-data-structure/
pub trait Stack {
    /// The type of the items stored in the stack.
    type Item;

    /// Returns `true` if the stack is empty.
    fn s_is_empty(&self) -> bool;

    /// Pushes an item to the stack.
    ///
    /// For vector, use [`Vec::push`] instead.
    ///
    /// ## Also see
    ///
    /// * [`Stack::s_push_checked`]
    // s_ prefix prevents name collision with Vec::push
    fn s_push(&mut self, item: Self::Item);

    /// Pushes an item to the stack.
    ///
    /// ## Notes
    ///
    /// For vector, use [`Vec::push`] instead. It is meant primarily for the `heapless::Vec`.
    ///
    /// ## Also see
    ///
    /// * [`Stack::s_push`]
    fn s_push_checked(&mut self, item: Self::Item) -> Option<()> {
        self.s_push(item);
        Some(())
    }

    // we don't create chain push because Extend::extend_one API will be better

    /// Pushes an item to the stack and returns an the "entry" object
    /// corresponding to the pushed element.
    fn lifo_push(&mut self, item: Self::Item) -> LIFOEntry<Self>;

    /// Pops an item from the stack.
    ///
    /// ## Notes
    ///
    /// For vector, use [`Vec::pop`] instead.
    ///
    /// ## Also see
    ///
    /// * [`Stack::s_pop_unchecked`].
    fn s_pop(&mut self) -> Option<Self::Item>;

    /// Pops an item from the stack without checking if the stack is empty.
    ///
    /// ## Safety
    ///
    /// The stack must not be empty.
    ///
    /// ## Also see
    ///
    /// * [`Stack::s_pop`]
    #[inline]
    unsafe fn s_pop_unchecked(&mut self) -> Self::Item {
        self.s_pop().unwrap_unchecked()
    }

    /// Returns an "entry" object corresponding to the top element of the stack.
    ///
    /// ## Notes
    ///
    /// This is useful when the entry might need to be used to be dereferenced to
    /// `&T` and/or `&mut T` and/or be promoted to `T` by popping the item from the stack.
    ///
    /// ## Also see
    ///
    /// * [`Stack::lifo_unchecked`]
    #[inline]
    fn lifo(&mut self) -> Option<LIFOEntry<Self>> {
        if self.s_is_empty() {
            None
        } else {
            Some(unsafe { LIFOEntry::new(self) })
        }
    }

    /// Returns an "entry" object corresponding to the top element of the stack
    /// without checking if the stack is empty.
    ///
    /// ## Safety
    ///
    /// The stack must not be empty.
    ///
    /// ## Also see
    ///
    /// * [`Stack::lifo`]
    #[inline]
    unsafe fn lifo_unchecked(&mut self) -> LIFOEntry<Self> {
        self.lifo().unwrap_unchecked()
    }

    /// Returns a shared reference to the top element of the stack.
    ///
    /// ## Also see
    ///
    /// * [`Stack::lifo_ref_unchecked`]
    /// * [`Stack::lifo`]
    /// * [`Stack::s_pop`]
    /// * [`Stack::lifo_mut`]
    fn lifo_ref(&self) -> Option<&Self::Item>;

    /// Returns a shared reference to the top element of the stack without checking if the stack is empty.
    ///
    /// ## Safety
    ///
    /// The stack must not be empty.
    ///
    /// ## Also see
    ///
    /// * [`Stack::lifo_ref`]
    #[inline]
    unsafe fn lifo_ref_unchecked(&self) -> &Self::Item {
        self.lifo_ref().unwrap_unchecked()
    }

    /// Returns a mutable reference to the top element of the stack.
    ///
    /// ## Also see
    ///
    /// * [`Stack::lifo_ref`]
    /// * [`Stack::lifo_mut_unchecked`]
    /// * [`Stack::lifo`]
    /// * [`Stack::s_pop`]
    fn lifo_mut(&mut self) -> Option<&mut Self::Item>;

    /// Returns a mutable reference to the top element of the stack without checking if the stack is empty.
    ///
    /// ## Safety
    ///
    /// The stack must not be empty.
    #[inline]
    unsafe fn lifo_mut_unchecked(&mut self) -> &mut Self::Item {
        self.lifo_mut().unwrap_unchecked()
    }
}

impl<T> Stack for Vec<T> {
    type Item = T;

    #[inline]
    fn s_is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn s_push(&mut self, item: Self::Item) {
        self.push(item);
    }

    #[inline]
    fn lifo_push(&mut self, item: Self::Item) -> LIFOEntry<Self> {
        self.push(item);
        // We just pushed to the vector, so the vector is not empty.
        unsafe { self.lifo_unchecked() }
    }

    #[inline]
    fn s_pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    #[inline]
    fn lifo_ref(&self) -> Option<&Self::Item> {
        self.last()
    }

    #[inline]
    fn lifo_mut(&mut self) -> Option<&mut Self::Item> {
        self.last_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_or_insert() {
        let mut stack = vec![1, 2, 3];
        let mut entry = stack.lifo_push(4);
        assert_eq!(*entry, 4);
        *entry = 5;
        assert_eq!(*entry, 5);
        drop(entry);
        assert_eq!(stack, vec![1, 2, 3, 5]);
        let entry = stack.lifo().unwrap();
        assert_eq!(entry.pop(), 5);
        assert_eq!(stack, vec![1, 2, 3]);
    }
}

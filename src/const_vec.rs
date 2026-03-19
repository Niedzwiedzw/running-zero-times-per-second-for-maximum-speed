use std::mem::MaybeUninit;

const MAX_SIZE: usize = 2usize.pow(13);

pub(crate) struct ConstVec<T> {
    memory: [MaybeUninit<T>; MAX_SIZE],
    len: usize,
}

impl<T> ConstVec<T> {
    pub const fn new() -> Self {
        Self {
            memory: [const { MaybeUninit::uninit() }; MAX_SIZE],
            len: 0,
        }
    }

    pub const fn push(self, value: T) -> Self {
        let mut memory = self.memory;
        memory[self.len] = MaybeUninit::new(value);
        Self {
            memory,
            len: self.len + 1,
        }
    }

    pub const fn extend(self, slice: &'static [T]) -> Self
    where
        T: Copy,
    {
        let mut this = self;
        let mut extend = slice;
        while let [next, tail @ ..] = extend {
            this = this.push(*next);
            extend = tail;
        }
        this
    }

    pub const fn as_ref<'a>(&'a self) -> &'a [T] {
        unsafe {
            // SAFETY: Only the first `self.len`-th elements are accessed, all initialized via push().
            //         MaybeUninit<T> has the same size and alignment as T.
            //         The returned reference lifetime is tied to &self.
            &*(self.memory.split_at(self.len).0 as *const [MaybeUninit<T>] as *const [T])
        }
    }
}

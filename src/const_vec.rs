use std::mem::MaybeUninit;

const MAX_SIZE: usize = 2usize.pow(13);

pub(crate) struct ConstVec<T> {
    memory: [MaybeUninit<T>; MAX_SIZE],
    len: usize,
}

impl<T> ConstVec<T> {
    pub const fn new() -> Self {
        Self {
            memory: unsafe { MaybeUninit::uninit().assume_init() },
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

    pub const fn as_ref(&self) -> &[T] {
        unsafe { &*(self.memory.split_at(self.len).0 as *const [MaybeUninit<T>] as *const [T]) }
    }
}

impl ConstVec<u8> {
    pub const fn as_str(&'static self) -> &'static str {
        unsafe { std::str::from_utf8_unchecked(self.as_ref()) }
    }
    pub const fn push_str(self, string: &'static str) -> Self {
        let mut this = self;
        let mut extend = string.as_bytes();
        while let [next, tail @ ..] = extend {
            this = this.push(*next);
            extend = tail;
        }
        this
    }
}

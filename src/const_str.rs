use crate::const_vec::ConstVec;

pub struct ConstStr {
    inner: ConstVec<u8>,
}

impl ConstStr {
    #[expect(
        clippy::new_without_default,
        reason = "it is quite large, allocating it should be more explicit"
    )]
    pub const fn new() -> Self {
        Self {
            inner: ConstVec::new(),
        }
    }
    pub const fn as_str(&'static self) -> &'static str {
        unsafe {
            // SAFETY: Bytes are added only via push_str() which takes &str - guaranteed valid UTF-8.
            std::str::from_utf8_unchecked(self.inner.as_ref())
        }
    }
    pub const fn push_str(self, string: &'static str) -> Self {
        Self {
            inner: self.inner.extend(string.as_bytes()),
        }
    }
}

#import "rustikon.typ": rustikon

#show: rustikon

// Slide 1: Title
#align(center + horizon)[
  #text(size: 24pt, weight: "bold")[
    Running 0 times per second for maximum speed
  ]

  #v(2cm)

  #text(size: 18pt)[Wojciech Brożek]


  #pagebreak()
  // Slide 2: introduction
  #link("https://github.com/Niedzwiedzw")[github.com/Niedzwiedzw] \
  #link(
    "mailto:wojciech.brozek@niedzwiedz.it",
  )[wojciech.brozek\@niedzwiedz.it] \
  #link("https://exein.io")[exein.io]

  ```rust
  trait Exein: Rust + Security {}
  ```
  // Slide 3: Problem
  #pagebreak()
  60FPS

  #pagebreak()
  60FPS (\~16ms per frame)

  // Slide 4: Solution
  #pagebreak()
  *Solution: doing less at runtime!*

  #pagebreak()
  *Solution: doing less at runtime!* \
  _(some nightly features)_

  #pagebreak()
  *Solution: doing less at runtime!* \
  _(some nightly features)_ \
  _(some unsafe code)_

  // Slide 5: Part 1 - constvec
  #pagebreak()
  ```rust
  /// we need a way to push/pop inside `const {}` blocks
  struct ConstVec<T> {
      // ..
  }
  ```
  #pagebreak()
  ```rust
  const MAX_SIZE: usize = 2usize.pow(13);

  /// we need a way to push/pop inside `const {}` blocks
  pub struct ConstVec<T> {
      memory: [MaybeUninit<T>; MAX_SIZE],
      len: usize,
  }
  ```

  #pagebreak()
  ```rust
  impl<T> ConstVec<T> {
      pub const fn new() -> Self {
          Self {
              memory: [
                  const { MaybeUninit::uninit() };
                  MAX_SIZE
              ],
              len: 0,
          }
      }
  }
  ```

  #pagebreak()
  ```rust
  impl<T> ConstVec<T> {
      pub const fn push(self, value: T) -> Self {
          let mut memory = self.memory;
          memory[self.len] = MaybeUninit::new(value);
          Self { memory, len: self.len + 1 }
      }
  }
  ```
  #pagebreak()
  ```rust
  impl<T> ConstVec<T> {
      pub const fn as_ref(&self) -> &[T] {
          unsafe {
              &*(
                  self.memory.split_at(self.len).0
                  as *const [MaybeUninit<T>]
                  as *const [T]
              )
          }
      }
  }
  ```

  #pagebreak()
  ```rust
  impl ConstVec<u8> {
      pub const fn push_str(self, s: &'static str) -> Self {
          let mut this = self;
          let mut extend = s.as_bytes();
          while let [next, tail @ ..] = extend {
              this = this.push(*next);
              extend = tail;
          }
          this
      }
  }
  ```

  #pagebreak()
  ```rust
  impl ConstVec<u8> {
      pub const fn as_str(&'static self) -> &'static str {
          unsafe {
              std::str::from_utf8_unchecked(self.as_ref())
          }
      }
  }

  // We can build strings at compile time
  ```
  #pagebreak()
  HTML builder... in Rust's type system?

  #pagebreak()
  ```rust
  // ./src/lib.rs
  // + #![feature(adt_const_params)]
  // + #![feature(unsized_const_params)]
  ```

  #pagebreak()
  ```rust
    pub trait IsAttribute {
        const ATTRIBUTE_BYTES: ConstVec<u8>;
        const ATTRIBUTE: &'static str;
    // }

    pub trait IsChild {
        const CHILD_BYTES: ConstVec<u8>;
        const CHILD: &'static str;
    }
  ```

  #pagebreak()
  ```rust
    pub trait IsChildren {
        const CHILDREN_BYTES: ConstVec<u8>;
        const CHILDREN: &'static str;
    }
    pub trait IsAttributes {
        const ATTRIBUTES_BYTES: ConstVec<u8>;
        const ATTRIBUTES: &'static str;
    }
  ```
  #pagebreak()
  ```rust
  /// For example: "background: black"
  pub struct Attribute<
      const K: &'static str,
      const V: &'static str,
  >;
  ```

  #pagebreak()
  ```rust
    impl<const K: &'static str, const V: &'static str>
        IsAttribute for Attribute<K, V> {
        const ATTRIBUTE_BYTES: ConstVec<u8> = const {
            ConstVec::new()
                .push_str(K).push_str(r#"=""#)
                .push_str(V).push_str(r#"""#)
        };
        // ...
    }
  ```
  #pagebreak()
  ```rust
    impl<const K: &'static str, const V: &'static str>
        IsAttribute for Attribute<K, V> {
        // ...
        const ATTRIBUTES: &'static str = const {
            Self::ATTRIBUTES_BYTES.as_str()
        };
    }
  ```

  // push attribute
  #pagebreak()
  ```rust
    pub struct PushAttribute<
        T: IsAttributes,
        Attribute: IsAttribute,
    >(PhantomData<(T, Attribute)>);

  ```
  #pagebreak()
  ```rust
    impl<T: IsAttributes, A: IsAttribute>
        IsAttributes for PushAttribute<T, A> {
        const ATTRIBUTES_BYTES: ConstVec<u8> = const {
            ConstVec::new()
                .push_str(T::ATTRIBUTES)
                .push_str(" ")
                .push_str(A::ATTRIBUTE)
        };
    }
  ```

  #pagebreak()
    ```rust
    impl<const TAG: &'static str, A, C> DomTree<TAG, A, C>
    where
        A: IsAttributes + 'static,
        C: IsChildren + 'static,
    {
        pub const fn child<Child: IsChild + 'static>(
            self,
            _child: Finished<Child>,
        ) -> DomTree<T, Attributes, PushChild<C, Child>> {
            DomTree::new()
        }

    ```
]

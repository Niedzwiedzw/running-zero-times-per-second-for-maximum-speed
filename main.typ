#import "rustikon.typ": rustikon

#show: rustikon

#set align(center + horizon);

#title[
  Running 0 times per second for maximum speed
]

#v(2cm)

#text(size: 18pt)[Wojciech Brożek]

#let slide(body, size: 13pt) = {
  set text(size: size)
  [
    #pagebreak()
    #body
  ]
}


#slide()[
  #link("https://github.com/Niedzwiedzw")[github.com/Niedzwiedzw] \
  #link(
    "mailto:wojciech.brozek@niedzwiedz.it",
  )[wojciech.brozek\@niedzwiedz.it] \

  ```rust
  /// `https://exein.io`
  trait Exein: Rust + Security {}
  ```
]

#slide()[
  60FPS
]

#slide()[
  60FPS (\~16ms per frame)
]

// Slide 4: Solution
#slide()[
  *Solution: doing less at runtime!*
]

#slide()[
  *Solution: doing less at runtime!* \
  _(some nightly features)_
]

#slide()[
  *Solution: doing less at runtime!* \
  _(some nightly features)_ \
  _(some unsafe code)_
]

#slide()[
  *Joining _&'static str_-ings at runtime.*
]
#slide()[
  *Joining _&'static str_-ings at runtime.* \
  `concat!("a", "b")`: only accepts literals (not very flexible)
]

// Slide 5: Part 1 - constvec
#slide()[
  ```rust
  /// build a &'static str inside `const {}`
  ///
  /// implementation:
  /// `github.com/Niedzwiedzw/`
  struct ConstStr {
      // ..
  }
  ```
]

#slide(size: 11pt)[
  ```rust
  impl ConstStr {
      /// Initialize in `const {}`
      pub const fn new() -> Self {}
      /// push strings in `const {}`
      pub const fn push_str(self, string: &'static str) -> Self {}
      /// safely cast to `&'static str` in `const {}`
      pub const fn as_str(&'static self) -> &'static str {}
  }

  ```
]

#slide()[
  HTML builder... in Rust's type system!
]

#slide(size: 10pt)[
  ```rust
  // The dream - this Rust code...
  el("main")
    .attribute("id", "rustikon")
    .child(
      el("form")
        .child(
          el("input")
            .attribute("value", "hello Rustikon 2026!")
        )
    )
  ```
  ```html
  <!-- ...produces this html - at compile time -->
  <main id="rustikon">
    <form>
      <input value="hello Rustikon 2026"></input>
    </form>
  </main>
  ```
]


#slide()[
  ```rust
  // ./src/lib.rs
  // + #![feature(adt_const_params)]
  // + #![feature(unsized_const_params)]
  ```
]

#slide()[
  ```rust
    // Notice: no methods...
    pub trait IsAttribute {
        const ATTRIBUTE: ConstStr;
    }


    // ...just slots to store &'static str-s
    pub trait IsChild {
        const CHILD: ConstStr;
    }

  ```
]

#slide()[
  ```rust
    pub trait IsChildren {
        const CHILDREN: ConstStr;
    }
    pub trait IsAttributes {
        const ATTRIBUTES: ConstStr;
    }
  ```
]

#slide()[
  ```rust
  /// For example: `style="background: black"`
  pub struct Attribute<
      const K: &'static str,
      const V: &'static str,
  >;
  ```
]

#slide(size: 11pt)[
  ```rust
    impl<
      const K: &'static str,
      const V: &'static str,
    > IsAttribute for Attribute<K, V> {
        const ATTRIBUTE: ConstStr = ConstStr::new()
            .push_str(K)
            .push_str(r#"=""#)
            .push_str(V)
            .push_str(r#"""#);
    }
  ```
]

#slide(size: 11pt)[
  ```rust
    pub struct Empty;

    impl IsChildren for Empty {
        const CHILDREN: ConstStr = ConstStr::new();
    }

    impl IsAttributes for Empty {
        const ATTRIBUTES: ConstStr = ConstStr::new();
    }
  ```
]

#slide(size: 11pt)[
  ```rust
    pub struct PushAttribute<
        T: IsAttributes,
        A: IsAttribute,
    >(PhantomData<(T, A)>);
  
    impl<
      T: IsAttributes,
      A: IsAttribute,
    > IsAttributes for PushAttribute<T, A> {
        const ATTRIBUTES: ConstStr = ConstStr::new()
            .push_str(T::ATTRIBUTES.as_str())
            .push_str(" ")
            .push_str(A::ATTRIBUTE.as_str());
    }
  ```
]
#slide(size: 11pt)[
  ```rust
  pub struct PushChild<T, C>(PhantomData<T>, PhantomData<C>);

  impl<T: IsChildren, C: IsChild> IsChildren for PushChild<T, C> {
      const CHILDREN: ConstStr = ConstStr::new()
          .push_str(T::CHILDREN.as_str())
          .push_str(C::CHILD.as_str());
  }
  ```
]

#slide()[
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

#slide()[
  *LIVE DEMO!*
]

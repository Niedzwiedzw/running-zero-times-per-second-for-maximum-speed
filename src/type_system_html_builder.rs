#![allow(private_bounds)]
#![allow(private_interfaces)]
#![allow(clippy::new_without_default)]

use std::marker::PhantomData;

use crate::const_vec::ConstVec;

pub trait IsAttribute {
    const ATTRIBUTE_BYTES: ConstVec<u8>;
    const ATTRIBUTE: &'static str;
}

pub trait IsChild {
    const CHILD_BYTES: ConstVec<u8>;
    const CHILD: &'static str;
}

pub trait IsChildren {
    const CHILDREN_BYTES: ConstVec<u8>;
    const CHILDREN: &'static str;
}
pub trait IsAttributes {
    const ATTRIBUTES_BYTES: ConstVec<u8>;
    const ATTRIBUTES: &'static str;
}

pub struct Empty;

impl IsChildren for Empty {
    const CHILDREN_BYTES: ConstVec<u8> = ConstVec::new();
    const CHILDREN: &'static str = "";
}

impl IsAttributes for Empty {
    const ATTRIBUTES_BYTES: ConstVec<u8> = ConstVec::new();
    const ATTRIBUTES: &'static str = "";
}

pub struct DomTree<const TAG: &'static str, Attributes, Children>(
    PhantomData<(Children, Attributes)>,
)
where
    Attributes: IsAttributes,
    Children: IsChildren;

impl<const TAG: &'static str, Attributes, Children> DomTree<TAG, Attributes, Children>
where
    Attributes: IsAttributes + 'static,
    Children: IsChildren + 'static,
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const TEXT: &'static str> TextElement<TEXT> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self
    }
}

impl<const T: &'static str, Attributes, Children> IsChild for DomTree<T, Attributes, Children>
where
    Attributes: IsAttributes + 'static,
    Children: IsChildren + 'static,
{
    const CHILD_BYTES: ConstVec<u8> = const {
        ConstVec::new()
            .push_str("<")
            .push_str(T)
            .push_str(Attributes::ATTRIBUTES)
            .push_str(">")
            .push_str(Children::CHILDREN)
            .push_str("</")
            .push_str(T)
            .push_str(">")
    };
    const CHILD: &'static str = const { Self::CHILD_BYTES.as_str() };
}

pub struct TextElement<const V: &'static str>;

impl<const T: &'static str> IsChild for TextElement<T> {
    const CHILD_BYTES: ConstVec<u8> = ConstVec::new();
    const CHILD: &'static str = T;
}

pub struct PushChild<T, C>(PhantomData<T>, PhantomData<C>);

impl<T: IsChildren, C: IsChild> IsChildren for PushChild<T, C> {
    const CHILDREN_BYTES: ConstVec<u8> =
        const { ConstVec::new().push_str(T::CHILDREN).push_str(C::CHILD) };
    const CHILDREN: &'static str = const { Self::CHILDREN_BYTES.as_str() };
}

pub struct Attribute<const K: &'static str, const V: &'static str>;

impl<const K: &'static str, const V: &'static str> IsAttribute for Attribute<K, V> {
    const ATTRIBUTE_BYTES: ConstVec<u8> = const {
        ConstVec::new()
            .push_str(K)
            .push_str(r#"=""#)
            .push_str(V)
            .push_str(r#"""#)
    };
    const ATTRIBUTE: &'static str = const { Self::ATTRIBUTE_BYTES.as_str() };
}

pub struct PushAttribute<T: IsAttributes, Attribute: IsAttribute>(PhantomData<(T, Attribute)>);

impl<T: IsAttributes, A: IsAttribute> IsAttributes for PushAttribute<T, A> {
    const ATTRIBUTES_BYTES: ConstVec<u8> = const {
        ConstVec::new()
            .push_str(T::ATTRIBUTES)
            .push_str(" ")
            .push_str(A::ATTRIBUTE)
    };
    const ATTRIBUTES: &'static str = const { Self::ATTRIBUTES_BYTES.as_str() };
}

pub type EmptyElement<const TAG_NAME: &'static str> = DomTree<TAG_NAME, Empty, Empty>;

pub const fn el<const TAG_NAME: &'static str>() -> EmptyElement<TAG_NAME> {
    EmptyElement::new()
}

pub struct Finished<C: IsChild>(PhantomData<C>);

impl<C: IsChild> Finished<C> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const TAG: &'static str, Attributes, Children> DomTree<TAG, Attributes, Children>
where
    Attributes: IsAttributes + 'static,
    Children: IsChildren + 'static,
{
    pub const fn child<Child: IsChild + 'static>(
        self,
        _child: Finished<Child>,
    ) -> DomTree<TAG, Attributes, PushChild<Children, Child>> {
        DomTree::new()
    }

    pub const fn text<const V: &'static str>(
        self,
    ) -> DomTree<TAG, Attributes, PushChild<Children, TextElement<V>>> {
        DomTree::new()
    }

    pub const fn attribute<const K: &'static str, const V: &'static str>(
        self,
    ) -> DomTree<TAG, PushAttribute<Attributes, Attribute<K, V>>, Children> {
        DomTree::new()
    }

    pub const fn finish(self) -> Finished<Self> {
        Finished::new()
    }

    pub const fn to_html(self) -> &'static str {
        Self::CHILD
    }
}

#![allow(private_bounds)]
#![allow(private_interfaces)]
#![allow(clippy::new_without_default)]

use {crate::const_str::ConstStr, std::marker::PhantomData};

pub trait IsAttribute {
    const ATTRIBUTE: ConstStr;
}

pub trait IsChild {
    const CHILD: ConstStr;
}

pub trait IsChildren {
    const CHILDREN: ConstStr;
}
pub trait IsAttributes {
    const ATTRIBUTES: ConstStr;
}

pub struct Empty;

impl IsChildren for Empty {
    const CHILDREN: ConstStr = ConstStr::new();
}

impl IsAttributes for Empty {
    const ATTRIBUTES: ConstStr = ConstStr::new();
}

pub struct DomTree<const TAG: &'static str, Attributes, Children>(PhantomData<(Children, Attributes)>)
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
    const CHILD: ConstStr = ConstStr::new()
        .push_str("<")
        .push_str(T)
        .push_str(Attributes::ATTRIBUTES.as_str())
        .push_str(">")
        .push_str(Children::CHILDREN.as_str())
        .push_str("</")
        .push_str(T)
        .push_str(">");
}

pub struct TextElement<const V: &'static str>;

impl<const T: &'static str> IsChild for TextElement<T> {
    const CHILD: ConstStr = ConstStr::from_str(T);
}

pub struct PushChild<T, C>(PhantomData<T>, PhantomData<C>);

impl<T: IsChildren, C: IsChild> IsChildren for PushChild<T, C> {
    const CHILDREN: ConstStr = ConstStr::new()
        .push_str(T::CHILDREN.as_str())
        .push_str(C::CHILD.as_str());
}

pub struct Attribute<const K: &'static str, const V: &'static str>;

impl<const K: &'static str, const V: &'static str> IsAttribute for Attribute<K, V> {
    const ATTRIBUTE: ConstStr = ConstStr::new()
        .push_str(K)
        .push_str(r#"=""#)
        .push_str(V)
        .push_str(r#"""#);
}

pub struct PushAttribute<T: IsAttributes, Attribute: IsAttribute>(PhantomData<(T, Attribute)>);

impl<T: IsAttributes, A: IsAttribute> IsAttributes for PushAttribute<T, A> {
    const ATTRIBUTES: ConstStr = ConstStr::new()
        .push_str(T::ATTRIBUTES.as_str())
        .push_str(" ")
        .push_str(A::ATTRIBUTE.as_str());
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
    pub const fn child<Child: IsChild + 'static>(self, _child: Finished<Child>) -> DomTree<TAG, Attributes, PushChild<Children, Child>> {
        DomTree::new()
    }

    pub const fn text<const V: &'static str>(self) -> DomTree<TAG, Attributes, PushChild<Children, TextElement<V>>> {
        DomTree::new()
    }

    pub const fn attribute<const K: &'static str, const V: &'static str>(self) -> DomTree<TAG, PushAttribute<Attributes, Attribute<K, V>>, Children> {
        DomTree::new()
    }

    pub const fn finish(self) -> Finished<Self> {
        Finished::new()
    }

    pub const fn to_html(self) -> &'static str {
        Self::CHILD.as_str()
    }
}

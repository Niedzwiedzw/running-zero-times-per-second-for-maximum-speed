#![expect(incomplete_features, reason = "Hello from Rustikon 2026!")]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]

use crate::type_system_html_builder::{DomTree, Empty};

pub mod const_vec;
pub mod type_system_html_builder;

type Element<const TAG: &'static str> = DomTree<TAG, Empty, Empty>;

fn el<const TAG: &'static str>() -> Element<TAG> {
    Element::new()
}

fn main() {
    let webpage = el::<"main">()
        .attribute::<"style", "width: 100dvw; height: 100dvh; background: black; color: white">()
        .child(
            el::<"nav">()
                .attribute::<"style", "display: grid; gap: 2rem; justify-items:center; padding-top: 2rem">()
                .child(
                    el::<"img">()
                        .attribute::<"src", "logo.svg">()
                        .attribute::<"style", "width: 20rem;">()
                        .finish(),
                )
                .child(
                    el::<"img">()
                        .attribute::<"src", "rustikon-logo.png">()
                        .attribute::<"style", "width: 20rem; padding-left: 1rem">()
                        .finish(),
                )
                .finish(),
        )
        .child(
            el::<"div">()
                .attribute::<"class", "rustikon-2026">()
                .attribute::<"style", "width: 100%; display: grid; alignt-items: center; justify-items: center;">()
                .child(
                    el::<"form">()
                        .attribute::<"style", "display: grid; grid-direction: row; gap: 1rem; width: 18rem">()
                        .child(
                            el::<"input">()
                                .attribute::<"style", "border: 3px solid lightgray; border-radius: 3px; background: darkgray; width: 18rem">()
                                .attribute::<"id", "first-name">()
                                .attribute::<"value", "Wojciech">()
                                .finish(),
                        )
                        .child(
                            el::<"input">()
                                .attribute::<"style", "border: 3px solid lightgray; border-radius: 3px; background: darkgray; width: 18rem">()
                                .attribute::<"id", "last-name">()
                                .attribute::<"value", "Brozek">()
                                .finish(),
                        )
                        .child(
                            el::<"input">()
                                .attribute::<"style", "border: 3px solid lightgray; border-radius: 3px; background: darkgray; width: 18rem">()
                                .attribute::<"id", "github">()
                                .attribute::<"value", "github.com/Niedzwiedzw">()
                                .finish(),
                        )
                        .child(
                            el::<"input">()
                                .attribute::<"style", "border: 3px solid lightgray; border-radius: 3px; background: darkgray; width: 18rem">()
                                .attribute::<"id", "company">()
                                .attribute::<"value", "Exein">()
                                .finish(),
                        )
                        .child(
                            el::<"button">()
                                .attribute::<"type", "submit">()
                                .text::<"Thank you">()
                                .finish(),
                        )
                        .finish(),
                )
                .finish(),
        );
    let html: &'static str = webpage.to_html();
    println!("{html}")
}

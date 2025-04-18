//! Showcase for `doc-image-embed`.
//!
// Important: note the blank line of documentation on each side of the image lookup table.
// The "image lookup table" can be placed anywhere.
#![cfg_attr(doc,
    doc = doc_image_embed::embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"),
    doc = doc_image_embed::embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"),
    doc = doc_image_embed::embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"),
    doc = doc_image_embed::embed_image!("corro", "images/corro.svg")
)]
//!
//! This crate contains no functionality, it is merely a demonstration of how to use
//! [embed-doc-image](https://crates.io/crates/embed-doc-image) to embed images local to the
//! repository that work across both [docs.rs](https://docs.rs) and
//! local documentation. The motivation for this crate is
//! [rustdoc's inability to include local images](https://github.com/rust-lang/rust/issues/32104)
//! in a way that consistently works across local copies of the repository and `docs.rs`.
//!
//! See [the documentation](https://docs.rs/embed-doc-image) for more information.
//! In addition, you are encouraged to browse the source code for this showcase crate to see a
//! fleshed out example of how the solution works.
//!
//! In addition to serving as a showcase, this crate is used to verify that the solution indeed
//! works across both local installations and `docs.rs`.
//! This is necessary because a proc macro crate cannot use its own macros in its own documentation.
//!
//! `embed-doc-image` should work across the usual web-supported file types
//! (jpg, png, svg, gif, bmp). If you find that it does not work with your files, please
//! file an issue.
//!
//! The below Ferris images are courtesy of [rustacean.net](https://rustacean.net).
//!
//! ![Original Ferris][ferris]
//!
//! ![Ferris making gesture][ferris-gesture]
//!
//! ![Corro][corro]
//!
//! ![Dancing Ferris][dancing-ferris]
//!
#[cfg(doc)]
use doc_image_embed::embed_image;

/// Test that images render in function docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = embed_image!("corro", "images/corro.svg"))]
pub fn function_docs_work() {}

/// Test that images render in module docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = doc_image_embed::embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = doc_image_embed::embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = doc_image_embed::embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = doc_image_embed::embed_image!("corro", "images/corro.svg"))]
pub mod module_docs_work {}

/// Test that images render in macro docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = embed_image!("corro", "images/corro.svg"))]
#[macro_export]
macro_rules! macro_docs_work {
    () => {};
}

/// Test that images render in struct docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = embed_image!("corro", "images/corro.svg"))]
pub struct StructDocsWork {}

/// Test that images render in trait docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = embed_image!("corro", "images/corro.svg"))]
pub trait TraitDocsWork {}

/// Test that images render in type docs.
///
/// ![Original Ferris][ferris] ![Ferris makes gesture][ferris-gesture]
///
/// Some more docs.
///
/// ![Corro][corro] ![Dancing Ferris][dancing-ferris]
#[cfg_attr(doc, doc = embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("ferris-gesture", "images/rustacean-flat-gesture-tiny.png"))]
#[cfg_attr(doc, doc = embed_image!("dancing-ferris", "images/dancing-ferris-tiny.gif"))]
#[cfg_attr(doc, doc = embed_image!("corro", "images/corro.svg"))]
pub type TypeAliasDocsWork = f64;

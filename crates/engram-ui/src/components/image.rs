//! Thin wrapper around [`gpui::img`] for showing bitmap / vector images.
//!
//! GPUI's `img()` element already handles loading from URLs, file paths,
//! and pre-loaded [`Arc<Image>`](gpui::Image) sources. This wrapper exists
//! to:
//!
//! - give callers a consistent `Image::new` constructor alongside the other
//!   engram components,
//! - bake engram's sizing and rounding tokens in through the type system
//!   ([`Radius`]), and
//! - give us a place to hang future defaults (loading spinner, error
//!   fallback) without changing the downstream call sites.
//!
//! For a circular profile picture, prefer
//! [`Avatar::image`](crate::components::Avatar::image) — it sizes and clips
//! the image to match the rest of the avatar family.

use engram_theme::Radius;
use gpui::{
    App, ImageSource, IntoElement, ObjectFit, Pixels, RenderOnce, Styled, StyledImage, Window,
    div, img, prelude::*,
};

/// A styled image element.
///
/// Simple builder around [`gpui::img`]. All fields default to "unset" —
/// if neither `width` nor `height` is given, the image lays out at its
/// intrinsic size.
#[derive(IntoElement)]
pub struct Image {
    source: ImageSource,
    width: Option<Pixels>,
    height: Option<Pixels>,
    radius: Option<Radius>,
    object_fit: ObjectFit,
    grayscale: bool,
}

impl Image {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
            radius: None,
            // `Cover` is the sane default for avatars and thumbnails; it
            // fills the frame and crops overflow rather than letterboxing.
            object_fit: ObjectFit::Cover,
            grayscale: false,
        }
    }

    /// Set both width and height to the same value (useful for square
    /// thumbnails).
    pub fn size(mut self, size: Pixels) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    /// Round the image's corners using an engram [`Radius`] token.
    pub fn rounded(mut self, radius: Radius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Fully round the image (convenience for circular avatars).
    pub fn rounded_full(self) -> Self {
        self.rounded(Radius::Full)
    }

    /// Override the default [`ObjectFit::Cover`] behavior.
    pub fn object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.object_fit = object_fit;
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl RenderOnce for Image {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Wrap `img` in a div so we can apply engram's Radius token without
        // caring whether the Img element's own rounding clips correctly on
        // every backend (SVG vs raster follow different paths).
        let mut container = div().overflow_hidden();
        if let Some(w) = self.width {
            container = container.w(w);
        }
        if let Some(h) = self.height {
            container = container.h(h);
        }
        if let Some(r) = self.radius {
            container = container.rounded(r.pixels());
        }

        let mut image = img(self.source).object_fit(self.object_fit).size_full();
        if self.grayscale {
            image = image.grayscale(true);
        }

        container.child(image)
    }
}

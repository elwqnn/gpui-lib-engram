//! Spacing, typography, and radius tokens.
//!
//! Deliberately simple compared to Zed's `DynamicSpacing` (which has a
//! ui-density dimension driven by a proc macro). We can add density later
//! if it turns out to matter; for now every token is a fixed `Pixels`.

use gpui::{Pixels, Rems, px, rems};

/// Semantic spacing step used for gaps, padding, and margins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spacing {
    /// 0px
    None,
    /// 2px
    XXSmall,
    /// 4px
    XSmall,
    /// 6px
    Small,
    /// 8px
    Medium,
    /// 12px
    Large,
    /// 16px
    XLarge,
    /// 20px
    XXLarge,
    /// 24px
    XXXLarge,
}

impl Spacing {
    pub const fn pixels(self) -> Pixels {
        match self {
            Self::None => px(0.0),
            Self::XXSmall => px(2.0),
            Self::XSmall => px(4.0),
            Self::Small => px(6.0),
            Self::Medium => px(8.0),
            Self::Large => px(12.0),
            Self::XLarge => px(16.0),
            Self::XXLarge => px(20.0),
            Self::XXXLarge => px(24.0),
        }
    }
}

/// Semantic text size for UI text.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextSize {
    /// 10px
    XSmall,
    /// 12px
    Small,
    /// 14px (default)
    #[default]
    Default,
    /// 16px
    Large,
    /// 18px
    XLarge,
}

impl TextSize {
    pub fn rems(self) -> Rems {
        match self {
            Self::XSmall => rems(10.0 / 16.0),
            Self::Small => rems(12.0 / 16.0),
            Self::Default => rems(14.0 / 16.0),
            Self::Large => rems(1.0),
            Self::XLarge => rems(18.0 / 16.0),
        }
    }

    pub const fn pixels(self) -> Pixels {
        match self {
            Self::XSmall => px(10.0),
            Self::Small => px(12.0),
            Self::Default => px(14.0),
            Self::Large => px(16.0),
            Self::XLarge => px(18.0),
        }
    }
}

/// Semantic border radius.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Radius {
    None,
    Small,
    #[default]
    Medium,
    Large,
    /// Fully rounded (pill).
    Full,
}

impl Radius {
    pub const fn pixels(self) -> Pixels {
        match self {
            Self::None => px(0.0),
            Self::Small => px(2.0),
            Self::Medium => px(4.0),
            Self::Large => px(8.0),
            Self::Full => px(9999.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_is_monotonically_increasing() {
        let steps = [
            Spacing::None,
            Spacing::XXSmall,
            Spacing::XSmall,
            Spacing::Small,
            Spacing::Medium,
            Spacing::Large,
            Spacing::XLarge,
            Spacing::XXLarge,
            Spacing::XXXLarge,
        ];
        for window in steps.windows(2) {
            assert!(
                window[0].pixels() < window[1].pixels(),
                "{:?} ({:?}) should be less than {:?} ({:?})",
                window[0],
                window[0].pixels(),
                window[1],
                window[1].pixels(),
            );
        }
    }

    #[test]
    fn text_size_pixels_match_doc_comments() {
        assert_eq!(TextSize::XSmall.pixels(), px(10.0));
        assert_eq!(TextSize::Small.pixels(), px(12.0));
        assert_eq!(TextSize::Default.pixels(), px(14.0));
        assert_eq!(TextSize::Large.pixels(), px(16.0));
        assert_eq!(TextSize::XLarge.pixels(), px(18.0));
    }

    #[test]
    fn text_size_default_is_default_variant() {
        assert_eq!(TextSize::default(), TextSize::Default);
    }

    #[test]
    fn radius_full_is_pill_sized() {
        // The "Full" radius should swallow any reasonable element height,
        // i.e. be larger than every other named radius combined.
        let other_max = [
            Radius::None,
            Radius::Small,
            Radius::Medium,
            Radius::Large,
        ]
        .iter()
        .map(|r| r.pixels())
        .max()
        .unwrap();
        assert!(Radius::Full.pixels() > other_max);
    }

    #[test]
    fn radius_default_is_medium() {
        assert_eq!(Radius::default(), Radius::Medium);
    }
}

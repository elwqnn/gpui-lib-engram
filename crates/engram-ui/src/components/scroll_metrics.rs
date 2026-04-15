//! Scrollbar thumb geometry — shared by the list-like components that
//! overlay an engram-styled scrollbar ([`VirtualList`], [`VariableList`]).
//!
//! The generic `Scrollbar` component handles the axis-aware overflow case;
//! the list components need the same thumb math for a custom overlay that
//! sits alongside the list primitive (`gpui::uniform_list` or `gpui::list`).
//! Extracting [`ThumbMetrics`] here keeps both components' decoration path
//! and drag-handler path driving from a single source of truth — drift
//! between the two would mean a dragged thumb that doesn't track the
//! cursor.

use gpui::{Pixels, px};

pub(crate) const SCROLLBAR_THICKNESS: Pixels = px(10.0);
pub(crate) const SCROLLBAR_MIN_THUMB_RATIO: f32 = 0.08;

/// Thumb sizing + bidirectional mapping between thumb-top and content
/// scroll offset. Returns `None` when the viewport or content is empty,
/// or when the content fully fits inside the viewport (no scrollbar
/// needed).
#[derive(Clone, Copy)]
pub(crate) struct ThumbMetrics {
    pub thumb_h: f32,
    pub travel: f32,
    pub max_scroll: f32,
}

impl ThumbMetrics {
    pub fn compute(viewport: Pixels, content: Pixels) -> Option<Self> {
        let viewport = viewport.as_f32();
        let content = content.as_f32();
        let max_scroll = content - viewport;
        if viewport <= 0.0 || max_scroll <= 0.0 {
            return None;
        }
        let ratio = (viewport / content).clamp(SCROLLBAR_MIN_THUMB_RATIO, 1.0);
        let thumb_h = viewport * ratio;
        let travel = (viewport - thumb_h).max(0.0);
        Some(Self {
            thumb_h,
            travel,
            max_scroll,
        })
    }

    pub fn thumb_top_for_scroll(&self, scroll: f32) -> f32 {
        let r = (scroll / self.max_scroll).clamp(0.0, 1.0);
        self.travel * r
    }

    pub fn scroll_for_thumb_top(&self, thumb_top: Pixels) -> Pixels {
        if self.travel <= 0.0 {
            return px(0.0);
        }
        px((thumb_top.as_f32() / self.travel).clamp(0.0, 1.0) * self.max_scroll)
    }
}

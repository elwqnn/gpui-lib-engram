//! Icon primitive backed by embedded SVG assets.
//!
//! [`IconName`] is a curated set of ~140 common UI icons, each variant
//! mapping to an `icons/<snake_case>.svg` asset shipped inside this crate.
//! The SVGs are sourced from [Lucide](https://lucide.dev) (ISC License — see
//! `assets/icons/LICENSES`) at their canonical 24×24 stroke form. The actual
//! asset resolution is performed by gpui's `svg()` element, which goes
//! through the [`AssetSource`](gpui::AssetSource) registered on the
//! [`Application`]. The showcase wires up [`crate::assets::Assets`] for that
//! purpose.
//!
//! Need an icon that isn't in the catalogue? Either drop the SVG into the
//! consuming app's own asset source and use [`Icon::from_path`], or open a
//! PR adding the variant.

use std::sync::Arc;

use engram_theme::{ActiveTheme, Color};
use gpui::{App, IntoElement, Pixels, RenderOnce, SharedString, Window, prelude::*, px, svg};
use strum::IntoStaticStr;

/// Size of an [`Icon`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconSize {
    /// 10px
    Indicator,
    /// 12px
    XSmall,
    /// 14px
    Small,
    /// 16px (default)
    #[default]
    Medium,
    /// 20px
    Large,
    /// 48px
    XLarge,
}

impl IconSize {
    pub const fn pixels(self) -> Pixels {
        match self {
            Self::Indicator => px(10.0),
            Self::XSmall => px(12.0),
            Self::Small => px(14.0),
            Self::Medium => px(16.0),
            Self::Large => px(20.0),
            Self::XLarge => px(48.0),
        }
    }
}

/// Catalogue of every icon shipped with engram-ui. Each variant resolves to
/// `icons/<snake_case>.svg` via [`IconName::path`].
///
/// The set is curated from [Lucide](https://lucide.dev) — the names follow
/// Lucide's vocabulary except for a few engram-side renames where the
/// component layer already speaks differently (e.g. [`Self::Close`] →
/// Lucide `x`, [`Self::Dash`] → Lucide `minus`, [`Self::MagnifyingGlass`] →
/// Lucide `search`, [`Self::Warning`] → Lucide `triangle-alert`,
/// [`Self::XCircle`] → Lucide `circle-x`). [`Self::StarFilled`] is a
/// derivative of Lucide's `star` with `fill="currentColor"`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum IconName {
    ArrowDown,
    ArrowDownLeft,
    ArrowDownRight,
    ArrowLeft,
    ArrowRight,
    ArrowRightLeft,
    ArrowUp,
    ArrowUpLeft,
    ArrowUpRight,
    AtSign,
    Bell,
    BellOff,
    BellRing,
    Bookmark,
    Bug,
    Calendar,
    Camera,
    Chat,
    Check,
    CheckCircle,
    CheckDouble,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronsLeftRight,
    ChevronsUpDown,
    Circle,
    CircleAlert,
    CircleHelp,
    Clipboard,
    Clock,
    Close,
    Cloud,
    Code,
    Copy,
    Cpu,
    Dash,
    Database,
    Diamond,
    Download,
    Ellipsis,
    EllipsisVertical,
    Eraser,
    ExternalLink,
    Eye,
    EyeOff,
    FastForward,
    File,
    FileCode,
    FileDiff,
    FileImage,
    FileLock,
    FilePlus,
    FileText,
    Filter,
    Flag,
    Flame,
    Folder,
    FolderOpen,
    FolderPlus,
    FolderSearch,
    GitBranch,
    GitCommit,
    GitMerge,
    GitPullRequest,
    Globe,
    Hash,
    Headphones,
    Heart,
    Hexagon,
    History,
    Home,
    Image,
    Info,
    Keyboard,
    Languages,
    Layers,
    Layout,
    Link,
    List,
    ListFilter,
    ListOrdered,
    ListTodo,
    ListTree,
    Lock,
    MagnifyingGlass,
    Mail,
    Maximize,
    Menu,
    Mic,
    MicOff,
    Minimize,
    Moon,
    Paperclip,
    Pause,
    Pencil,
    Phone,
    Pin,
    PinOff,
    Play,
    Plus,
    Power,
    Quote,
    Refresh,
    RotateCcw,
    RotateCw,
    Save,
    Scissors,
    Send,
    Server,
    Settings,
    Share,
    Sidebar,
    SkipBack,
    SkipForward,
    Sliders,
    Sparkles,
    Square,
    Star,
    StarFilled,
    Stop,
    Sun,
    Table,
    Terminal,
    ThumbsDown,
    ThumbsUp,
    Timer,
    Trash,
    Triangle,
    Unlock,
    Upload,
    User,
    UserCheck,
    UserGroup,
    UserPlus,
    UserRound,
    Volume,
    VolumeOff,
    Warning,
    XCircle,
    Zap,
}

impl IconName {
    /// Asset path to this icon's SVG, e.g. `IconName::ArrowDown` →
    /// `icons/arrow_down.svg`. Resolved by gpui's `svg().path(...)`.
    pub fn path(self) -> Arc<str> {
        let stem: &'static str = self.into();
        format!("icons/{stem}.svg").into()
    }
}

/// An SVG icon resolved from the active [`AssetSource`](gpui::AssetSource).
#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    size: IconSize,
    color: Color,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            path: SharedString::from(name.path()),
            size: IconSize::default(),
            color: Color::default(),
        }
    }

    /// Construct an icon from a raw asset path. Useful for icons not in
    /// [`IconName`] (e.g. extension-supplied SVGs).
    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            size: IconSize::default(),
            color: Color::default(),
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Icons resolve against the `icon*` tokens, not the text tokens, so
        // `Color::Default` lands on `theme.colors.icon`.
        let colors = cx.theme().colors();
        let hsla = match self.color {
            Color::Default => colors.icon,
            Color::Muted => colors.icon_muted,
            Color::Disabled => colors.icon_disabled,
            Color::Accent => colors.icon_accent,
            other => other.hsla(colors),
        };

        svg()
            .size(self.size.pixels())
            .flex_none()
            .path(self.path)
            .text_color(hsla)
    }
}

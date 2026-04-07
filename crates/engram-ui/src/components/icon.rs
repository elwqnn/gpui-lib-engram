//! Icon primitive backed by embedded SVG assets.
//!
//! [`IconName`] mirrors Zed's icon set verbatim — every variant maps to a
//! `icons/<snake_case>.svg` asset shipped inside this crate. The actual asset
//! resolution is performed by gpui's `svg()` element, which goes through the
//! [`AssetSource`](gpui::AssetSource) registered on the [`Application`]. The
//! showcase wires up [`crate::assets::Assets`] for that purpose.

use std::sync::Arc;

use engram_theme::{ActiveTheme, Color};
use gpui::{App, IntoElement, Pixels, RenderOnce, SharedString, Window, prelude::*, px, svg};
use strum::IntoStaticStr;

/// Size of an [`Icon`]. The numbers match Zed.
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
/// The set is the same as Zed's `IconName` enum so call sites can refer to
/// any of them — and so we can swap in newer Zed icons by re-syncing the
/// SVG assets without touching call sites.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum IconName {
    AcpRegistry,
    AiAnthropic,
    AiBedrock,
    AiClaude,
    AiDeepSeek,
    AiEdit,
    AiGemini,
    AiGoogle,
    AiLmStudio,
    AiMistral,
    AiOllama,
    AiOpenAi,
    AiOpenAiCompat,
    AiOpenCode,
    AiOpenRouter,
    AiVercel,
    AiVZero,
    AiXAi,
    AiZed,
    Archive,
    ArrowCircle,
    ArrowDown,
    ArrowDown10,
    ArrowDownRight,
    ArrowLeft,
    ArrowRight,
    ArrowRightLeft,
    ArrowUp,
    ArrowUpRight,
    AtSign,
    Attach,
    AudioOff,
    AudioOn,
    Backspace,
    Bell,
    BellDot,
    BellOff,
    BellRing,
    Binary,
    Blocks,
    BoltFilled,
    BoltOutlined,
    Book,
    BookCopy,
    Box,
    BoxOpen,
    CaseSensitive,
    Chat,
    Check,
    CheckDouble,
    ChevronDown,
    ChevronDownUp,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronUpDown,
    Circle,
    CircleHelp,
    Close,
    CloudDownload,
    Code,
    Command,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    CountdownTimer,
    Crosshair,
    CursorIBeam,
    Dash,
    DatabaseZap,
    Debug,
    DebugBreakpoint,
    DebugContinue,
    DebugDetach,
    DebugDisabledBreakpoint,
    DebugDisabledLogBreakpoint,
    DebugIgnoreBreakpoints,
    DebugLogBreakpoint,
    DebugPause,
    DebugStepInto,
    DebugStepOut,
    DebugStepOver,
    Diff,
    DiffSplit,
    DiffSplitAuto,
    DiffUnified,
    Disconnected,
    Download,
    EditorAtom,
    EditorCursor,
    EditorEmacs,
    EditorJetBrains,
    EditorSublime,
    EditorVsCode,
    Ellipsis,
    Envelope,
    Eraser,
    Escape,
    Exit,
    ExpandDown,
    ExpandUp,
    ExpandVertical,
    Eye,
    EyeOff,
    FastForward,
    FastForwardOff,
    File,
    FileCode,
    FileDiff,
    FileDoc,
    FileGeneric,
    FileGit,
    FileLock,
    FileMarkdown,
    FileRust,
    FileTextFilled,
    FileTextOutlined,
    FileToml,
    FileTree,
    Filter,
    Flame,
    Folder,
    FolderOpen,
    FolderPlus,
    FolderSearch,
    Font,
    FontSize,
    FontWeight,
    ForwardArrow,
    ForwardArrowUp,
    GenericClose,
    GenericMaximize,
    GenericMinimize,
    GenericRestore,
    GitBranch,
    GitBranchAlt,
    GitBranchPlus,
    GitCommit,
    GitGraph,
    GitMergeConflict,
    GitWorktree,
    Github,
    Hash,
    HistoryRerun,
    Image,
    Inception,
    Indicator,
    Info,
    Json,
    Keyboard,
    Library,
    LineHeight,
    Link,
    Linux,
    ListCollapse,
    ListFilter,
    ListTodo,
    ListTree,
    ListX,
    LoadCircle,
    LocationEdit,
    LockOutlined,
    MagnifyingGlass,
    Maximize,
    MaximizeAlt,
    Menu,
    MenuAltTemp,
    Mic,
    MicMute,
    Minimize,
    NewThread,
    Notepad,
    OpenFolder,
    Option,
    PageDown,
    PageUp,
    Paperclip,
    Pencil,
    PencilUnavailable,
    Person,
    Pin,
    PlayFilled,
    PlayOutlined,
    Plus,
    Power,
    Public,
    PullRequest,
    QueueMessage,
    Quote,
    Reader,
    RefreshTitle,
    Regex,
    ReplNeutral,
    Replace,
    ReplaceAll,
    ReplaceNext,
    ReplyArrowRight,
    Rerun,
    Return,
    RotateCcw,
    RotateCw,
    Scissors,
    Screen,
    SelectAll,
    Send,
    Server,
    Settings,
    Shift,
    SignalHigh,
    SignalLow,
    SignalMedium,
    Slash,
    Sliders,
    Space,
    Sparkle,
    Split,
    SplitAlt,
    SquareDot,
    SquareMinus,
    SquarePlus,
    Star,
    StarFilled,
    Stop,
    Tab,
    Terminal,
    TerminalAlt,
    TextSnippet,
    ThinkingMode,
    ThinkingModeOff,
    Thread,
    ThreadFromSummary,
    ThreadImport,
    ThreadsSidebarLeftClosed,
    ThreadsSidebarLeftOpen,
    ThreadsSidebarRightClosed,
    ThreadsSidebarRightOpen,
    ThumbsDown,
    ThumbsUp,
    TodoComplete,
    TodoPending,
    TodoProgress,
    ToolCopy,
    ToolDeleteFile,
    ToolDiagnostics,
    ToolFolder,
    ToolHammer,
    ToolNotification,
    ToolPencil,
    ToolSearch,
    ToolTerminal,
    ToolThink,
    ToolWeb,
    Trash,
    Triangle,
    TriangleRight,
    Undo,
    Unpin,
    UserCheck,
    UserGroup,
    UserRoundPen,
    Warning,
    WholeWord,
    XCircle,
    XCircleFilled,
    ZedAgent,
    ZedAgentTwo,
    ZedAssistant,
    ZedPredict,
    ZedPredictDisabled,
    ZedPredictDown,
    ZedPredictError,
    ZedPredictUp,
    ZedSrcCustom,
    ZedSrcExtension,
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

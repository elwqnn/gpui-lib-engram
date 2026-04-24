//! Watch a directory of JSON theme files and reload them in-place when
//! they change on disk.
//!
//! The primary entry point is [`watch_themes_dir`]. It takes an absolute
//! path and, from that moment on, any `.json` file dropped into (or saved
//! over) that directory is parsed via [`Theme::from_json_bytes`] and merged
//! into the [`ThemeRegistry`]. If the file holds the currently-active
//! theme, the active theme is re-applied so the next frame reflects the
//! new colors.
//!
//! The actual filesystem notifications are delivered by `notify` on a
//! background thread. They are forwarded over an async channel to a GPUI
//! foreground task that `.await`s them and runs the update on the main
//! thread, so reloads are always observed with a live `App`.

use std::path::{Path, PathBuf};

use anyhow::Result;
use async_channel::{Receiver, unbounded};
use gpui::{App, Task};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::registry::{ThemeRegistry, activate_theme};
use crate::{ActiveTheme, Theme};

/// Handle returned by [`watch_themes_dir`]. Keep it alive for as long as
/// hot reload should run - dropping it tears down the watcher and the
/// reload task.
#[must_use = "dropping ThemeWatcher immediately stops hot reload; bind it for as long as you want reloads to run"]
pub struct ThemeWatcher {
    _watcher: RecommendedWatcher,
    _reload_task: Task<()>,
}

/// Start watching `dir` for JSON theme changes. Any pre-existing `.json`
/// files are loaded immediately; new files are picked up as they appear.
///
/// The returned [`ThemeWatcher`] must be held alive for as long as hot
/// reload should run - dropping it tears down the underlying `notify`
/// watcher and the GPUI reload task.
///
/// # Errors
///
/// Returns an error if `dir` does not exist, is not a directory, or if
/// the watcher cannot be installed (e.g. on platforms without an inotify
/// backend or equivalent).
#[must_use = "the returned ThemeWatcher must be kept alive; dropping it immediately stops hot reload"]
pub fn watch_themes_dir(dir: impl AsRef<Path>, cx: &mut App) -> Result<ThemeWatcher> {
    let dir = dir.as_ref().to_path_buf();
    if !dir.is_dir() {
        anyhow::bail!("theme directory does not exist: {}", dir.display());
    }

    // Load everything already on disk once, so the registry reflects the
    // contents of the directory even before the first filesystem event.
    load_all_themes(&dir, cx);

    // notify fires events from a background thread - forward them onto an
    // async channel that the foreground reload task awaits.
    let (tx, rx) = unbounded::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send_blocking(res);
    })?;
    watcher.watch(&dir, RecursiveMode::NonRecursive)?;

    let reload_task = spawn_reload_task(dir, rx, cx);

    Ok(ThemeWatcher {
        _watcher: watcher,
        _reload_task: reload_task,
    })
}

fn spawn_reload_task(
    dir: PathBuf,
    rx: Receiver<notify::Result<Event>>,
    cx: &mut App,
) -> Task<()> {
    cx.spawn(async move |cx| {
        // Block on the next event; once one arrives, coalesce any siblings
        // that are already queued before doing a single reload pass. This
        // avoids thrashing the registry when an editor emits a burst of
        // Create/Modify events for the same save.
        while let Ok(first) = rx.recv().await {
            let mut touched = matches!(first, Ok(ref ev) if is_relevant(ev));
            while let Ok(event) = rx.try_recv() {
                if let Ok(ev) = event
                    && is_relevant(&ev)
                {
                    touched = true;
                }
            }
            if touched {
                let dir = dir.clone();
                #[allow(clippy::let_unit_value, unused_must_use)]
                let _ = cx.update(|cx| load_all_themes(&dir, cx));
            }
        }
    })
}

fn is_relevant(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    ) && event
        .paths
        .iter()
        .any(|p| p.extension().map(|e| e == "json").unwrap_or(false))
}

fn load_all_themes(dir: &Path, cx: &mut App) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e != "json").unwrap_or(true) {
            continue;
        }
        match load_single(&path) {
            Ok(theme) => {
                let name = theme.name.clone();
                ThemeRegistry::global_mut(cx).insert(theme);
                // If the reloaded theme is the one currently active, re-activate
                // it so the next frame paints with the new colors.
                if cx.theme().name == name {
                    let _ = activate_theme(&name, cx);
                }
            }
            Err(err) => {
                eprintln!("engram-theme: failed to reload `{}`: {err}", path.display());
            }
        }
    }
}

fn load_single(path: &Path) -> Result<Theme> {
    let bytes = std::fs::read(path)?;
    Theme::from_json_bytes(&bytes)
}

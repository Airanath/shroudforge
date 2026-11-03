use std::sync::OnceLock;
use tao::window::CursorIcon;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::System::LibraryLoader::GetProcAddress;

/// Wine keeps cursor handles per process: when the WebView2 browser process
/// calls `SetCursor` for the page, Wine forwards the handle to the window
/// owner (this process), rejects it ("icon handle … from other process") and
/// keeps the last cursor this process set, usually a resize or move arrow
/// from the undecorated window edges. Under Wine the page therefore reports
/// its CSS cursor and the host sets the same shape itself. On Windows the
/// browser process sets the cursor directly, so nothing is mirrored there.
pub(crate) fn mirrors_page_cursor() -> bool {
    running_under_wine()
}

/// True when the process runs under Wine/Proton (`ntdll!wine_get_version`).
pub(crate) fn running_under_wine() -> bool {
    static WINE: OnceLock<bool> = OnceLock::new();
    *WINE.get_or_init(|| unsafe {
        let ntdll = GetModuleHandleA(c"ntdll.dll".as_ptr().cast());
        !ntdll.is_null() && GetProcAddress(ntdll, c"wine_get_version".as_ptr().cast()).is_some()
    })
}

/// Maps a CSS cursor keyword reported by the page to the host cursor.
pub(crate) fn page_cursor_icon(name: &str) -> Option<CursorIcon> {
    Some(match name {
        "default" | "auto" => CursorIcon::Default,
        "pointer" => CursorIcon::Hand,
        "text" => CursorIcon::Text,
        "vertical-text" => CursorIcon::VerticalText,
        "not-allowed" => CursorIcon::NotAllowed,
        "no-drop" => CursorIcon::NoDrop,
        "wait" => CursorIcon::Wait,
        "progress" => CursorIcon::Progress,
        "help" => CursorIcon::Help,
        "crosshair" => CursorIcon::Crosshair,
        "move" => CursorIcon::Move,
        "grab" => CursorIcon::Grab,
        "grabbing" => CursorIcon::Grabbing,
        "all-scroll" => CursorIcon::AllScroll,
        "ew-resize" | "e-resize" | "w-resize" => CursorIcon::EwResize,
        "ns-resize" | "n-resize" | "s-resize" => CursorIcon::NsResize,
        "nesw-resize" | "ne-resize" | "sw-resize" => CursorIcon::NeswResize,
        "nwse-resize" | "nw-resize" | "se-resize" => CursorIcon::NwseResize,
        "col-resize" => CursorIcon::ColResize,
        "row-resize" => CursorIcon::RowResize,
        "copy" => CursorIcon::Copy,
        "alias" => CursorIcon::Alias,
        "cell" => CursorIcon::Cell,
        "context-menu" => CursorIcon::ContextMenu,
        "zoom-in" => CursorIcon::ZoomIn,
        "zoom-out" => CursorIcon::ZoomOut,
        _ => return None,
    })
}

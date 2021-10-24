use alloc::borrow::Cow;
use core::str::FromStr;

// Based on
// https://www.w3.org/TR/uievents-code/
//
// Also somewhat based on MDN, but it turns out to be wrong in many ways:
// https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values
//
// Chromium's sources:
// https://github.com/chromium/chromium/blob/5af3e41ce69e2e18b899589b46540e4360527733/ui/events/keycodes/dom/dom_code_data.inc
//
// Firefox's sources:
// https://github.com/mozilla/gecko-dev/blob/25002b534963ad95ff0c1a3dd0f906ba023ddc8e/widget/NativeKeyToDOMCodeName.h
//
// Safari's sources:
// Windows:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/win/WindowsKeyNames.cpp
// macOS:
// https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/mac/PlatformEventFactoryMac.mm
// Linux GTK:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/gtk/PlatformKeyboardEventGtk.cpp
// WPE:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/libwpe/PlatformKeyboardEventLibWPE.cpp

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum KeyCode {
    // Writing System Keys
    Backquote,
    Backslash,
    Backspace,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,

    // Functional Keys
    AltLeft,
    AltRight,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    MetaLeft, // Known as OSLeft in all Firefox versions and Chrome <52, as well as all versions of Safari GTK and WPE.
    MetaRight, // Known as OSRight in all Firefox versions and Chrome <52, as well as all versions of Safari GTK and WPE.
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,

    // Functional Keys found on Japanese and Korean keyboards
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,

    // Control Pad Section
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,

    // Arrow Pad Section
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    // Numpad Section
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,  // No browser seems to use these?
    NumpadClear,      // No browser seems to use these?
    NumpadClearEntry, // No browser seems to use these?
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,           // No browser seems to use these?
    NumpadMemoryAdd,      // No browser seems to use these?
    NumpadMemoryClear,    // No browser seems to use these?
    NumpadMemoryRecall,   // No browser seems to use these?
    NumpadMemoryStore,    // No browser seems to use these?
    NumpadMemorySubtract, // No browser seems to use these?
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar, // No browser seems to use these?
    NumpadSubtract,

    // Function Section
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Fn,
    FnLock, // No browser seems to use this?
    PrintScreen,
    ScrollLock,
    Pause,

    // Media Keys
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,

    // Legacy, Non-Standard and Special Keys
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,

    // Gamepad Keys
    Gamepad0,
    Gamepad1,
    Gamepad2,
    Gamepad3,
    Gamepad4,
    Gamepad5,
    Gamepad6,
    Gamepad7,
    Gamepad8,
    Gamepad9,
    Gamepad10,
    Gamepad11,
    Gamepad12,
    Gamepad13,
    Gamepad14,
    Gamepad15,
    Gamepad16,
    Gamepad17,
    Gamepad18,
    Gamepad19,

    // Chrome only Keys
    BrightnessDown,
    BrightnessUp,
    DisplayToggleIntExt,
    KeyboardLayoutSelect,
    LaunchAssistant,
    LaunchControlPanel,
    LaunchScreenSaver,
    MailForward,
    MailReply,
    MailSend,
    MediaFastForward,
    MediaPause,
    MediaPlay,
    MediaRecord,
    MediaRewind,
    PrivacyScreenToggle,
    SelectTask,
    ShowAllWindows,
    ZoomToggle,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum KeyCodeClass {
    WritingSystem,
    Functional,
    ControlPad,
    ArrowPad,
    Numpad,
    Function,
    Media,
    Legacy,
    Gamepad,
    NonStandard,
}

impl KeyCode {
    /// Resolve the KeyCode according to the standard US layout.
    pub fn as_str(self) -> &'static str {
        use self::KeyCode::*;
        match self {
            Backquote => "`",
            Backslash => r"\",
            Backspace => "⌫",
            BracketLeft => "[",
            BracketRight => "]",
            Comma => ",",
            Digit0 => "0",
            Digit1 => "1",
            Digit2 => "2",
            Digit3 => "3",
            Digit4 => "4",
            Digit5 => "5",
            Digit6 => "6",
            Digit7 => "7",
            Digit8 => "8",
            Digit9 => "9",
            Equal => "=",
            IntlBackslash => r"International Backslash",
            IntlRo => "ろ",
            IntlYen => "¥",
            KeyA => "A",
            KeyB => "B",
            KeyC => "C",
            KeyD => "D",
            KeyE => "E",
            KeyF => "F",
            KeyG => "G",
            KeyH => "H",
            KeyI => "I",
            KeyJ => "J",
            KeyK => "K",
            KeyL => "L",
            KeyM => "M",
            KeyN => "N",
            KeyO => "O",
            KeyP => "P",
            KeyQ => "Q",
            KeyR => "R",
            KeyS => "S",
            KeyT => "T",
            KeyU => "U",
            KeyV => "V",
            KeyW => "W",
            KeyX => "X",
            KeyY => "Y",
            KeyZ => "Z",
            Minus => "-",
            Period => ".",
            Quote => "'",
            Semicolon => ";",
            Slash => "/",
            AltLeft => "Alt Left",
            AltRight => "Alt Right",
            CapsLock => "⇪",
            ContextMenu => "Context Menu",
            ControlLeft => "Control Left",
            ControlRight => "Control Right",
            Enter => "↵",
            MetaLeft => "⌘ Left",
            MetaRight => "⌘ Right",
            ShiftLeft => "⇧ Left",
            ShiftRight => "⇧ Right",
            Space => "Space",
            Tab => "⇥",
            Convert => "変換",
            KanaMode => "カタカナ/ひらがな/ローマ字",
            Lang1 => "한/영 かな",
            Lang2 => "한자 英数",
            Lang3 => "カタカナ",
            Lang4 => "ひらがな",
            Lang5 => "半角/全角/漢字",
            NonConvert => "無変換",
            Delete => "Delete",
            End => "End",
            Help => "Help",
            Home => "Home",
            Insert => "Insert",
            PageDown => "Page Down",
            PageUp => "Page Up",
            ArrowDown => "↓",
            ArrowLeft => "←",
            ArrowRight => "→",
            ArrowUp => "↑",
            NumLock => "Num Lock",
            Numpad0 => "Numpad 0",
            Numpad1 => "Numpad 1",
            Numpad2 => "Numpad 2",
            Numpad3 => "Numpad 3",
            Numpad4 => "Numpad 4",
            Numpad5 => "Numpad 5",
            Numpad6 => "Numpad 6",
            Numpad7 => "Numpad 7",
            Numpad8 => "Numpad 8",
            Numpad9 => "Numpad 9",
            NumpadAdd => "Numpad +",
            NumpadBackspace => "Numpad ⌫",
            NumpadClear => "Numpad C",
            NumpadClearEntry => "Numpad CE",
            NumpadComma => "Numpad ,",
            NumpadDecimal => "Numpad .",
            NumpadDivide => "Numpad /",
            NumpadEnter => "Numpad ↵",
            NumpadEqual => "Numpad =",
            NumpadHash => "Numpad #",
            NumpadMemoryAdd => "Numpad M+",
            NumpadMemoryClear => "Numpad MC",
            NumpadMemoryRecall => "Numpad MR",
            NumpadMemoryStore => "Numpad MS",
            NumpadMemorySubtract => "Numpad M-",
            NumpadMultiply => "Numpad *",
            NumpadParenLeft => "Numpad (",
            NumpadParenRight => "Numpad )",
            NumpadStar => "Numpad * (Star)",
            NumpadSubtract => "Numpad -",
            Escape => "Escape",
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            F13 => "F13",
            F14 => "F14",
            F15 => "F15",
            F16 => "F16",
            F17 => "F17",
            F18 => "F18",
            F19 => "F19",
            F20 => "F20",
            F21 => "F21",
            F22 => "F22",
            F23 => "F23",
            F24 => "F24",
            Fn => "Fn",
            FnLock => "FnLock",
            PrintScreen => "Print Screen",
            ScrollLock => "Scroll Lock",
            Pause => "Pause Break",
            BrowserBack => "Browser ⏮",
            BrowserFavorites => "Browser Favorites",
            BrowserForward => "Browser ⏭",
            BrowserHome => "Browser 🏠",
            BrowserRefresh => "Browser Refresh",
            BrowserSearch => "Browser Search",
            BrowserStop => "Browser Stop",
            Eject => "⏏",
            LaunchApp1 => "Launch App 1",
            LaunchApp2 => "Launch App 2",
            LaunchMail => "Launch Mail",
            MediaPlayPause => "⏯",
            MediaSelect => "Media Select",
            MediaStop => "◼",
            MediaTrackNext => "⏭",
            MediaTrackPrevious => "⏮",
            Power => "Power",
            Sleep => "Sleep",
            AudioVolumeDown => "🔉",
            AudioVolumeMute => "🔇",
            AudioVolumeUp => "🔊",
            WakeUp => "Wake Up",
            Again => "Again",
            Copy => "Copy",
            Cut => "Cut",
            Find => "Find",
            Open => "Open",
            Paste => "Paste",
            Props => "Props",
            Select => "Select",
            Undo => "Undo",
            Gamepad0 => "Gamepad 0",
            Gamepad1 => "Gamepad 1",
            Gamepad2 => "Gamepad 2",
            Gamepad3 => "Gamepad 3",
            Gamepad4 => "Gamepad 4",
            Gamepad5 => "Gamepad 5",
            Gamepad6 => "Gamepad 6",
            Gamepad7 => "Gamepad 7",
            Gamepad8 => "Gamepad 8",
            Gamepad9 => "Gamepad 9",
            Gamepad10 => "Gamepad 10",
            Gamepad11 => "Gamepad 11",
            Gamepad12 => "Gamepad 12",
            Gamepad13 => "Gamepad 13",
            Gamepad14 => "Gamepad 14",
            Gamepad15 => "Gamepad 15",
            Gamepad16 => "Gamepad 16",
            Gamepad17 => "Gamepad 17",
            Gamepad18 => "Gamepad 18",
            Gamepad19 => "Gamepad 19",
            BrightnessDown => "Brightness Down",
            BrightnessUp => "Brightness Up",
            DisplayToggleIntExt => "Display Toggle Intern / Extern",
            KeyboardLayoutSelect => "Keyboard Layout Select",
            LaunchAssistant => "Launch Assistant",
            LaunchControlPanel => "Launch Control Panel",
            LaunchScreenSaver => "Launch Screen Saver",
            MailForward => "Mail Forward",
            MailReply => "Mail Reply",
            MailSend => "Mail Send",
            MediaFastForward => "⏩",
            MediaPause => "⏸",
            MediaPlay => "▶",
            MediaRecord => "⏺",
            MediaRewind => "⏪",
            PrivacyScreenToggle => "Privacy Screen Toggle",
            SelectTask => "Select Task",
            ShowAllWindows => "Show All Windows",
            ZoomToggle => "Zoom Toggle",
        }
    }

    pub fn classify(self) -> KeyCodeClass {
        use self::KeyCode::*;
        match self {
            // Writing System Keys
            Backquote | Backslash | Backspace | BracketLeft | BracketRight | Comma | Digit0
            | Digit1 | Digit2 | Digit3 | Digit4 | Digit5 | Digit6 | Digit7 | Digit8 | Digit9
            | Equal | IntlBackslash | IntlRo | IntlYen | KeyA | KeyB | KeyC | KeyD | KeyE
            | KeyF | KeyG | KeyH | KeyI | KeyJ | KeyK | KeyL | KeyM | KeyN | KeyO | KeyP | KeyQ
            | KeyR | KeyS | KeyT | KeyU | KeyV | KeyW | KeyX | KeyY | KeyZ | Minus | Period
            | Quote | Semicolon | Slash => KeyCodeClass::WritingSystem,

            // Functional Keys
            AltLeft | AltRight | CapsLock | ContextMenu | ControlLeft | ControlRight | Enter
            | MetaLeft | MetaRight | ShiftLeft | ShiftRight | Space | Tab | Convert | KanaMode
            | Lang1 | Lang2 | Lang3 | Lang4 | Lang5 | NonConvert => KeyCodeClass::Functional,

            // Control Pad Section
            Delete | End | Help | Home | Insert | PageDown | PageUp => KeyCodeClass::ControlPad,

            // Arrow Pad Section
            ArrowDown | ArrowLeft | ArrowRight | ArrowUp => KeyCodeClass::ArrowPad,

            // Numpad Section
            NumLock | Numpad0 | Numpad1 | Numpad2 | Numpad3 | Numpad4 | Numpad5 | Numpad6
            | Numpad7 | Numpad8 | Numpad9 | NumpadAdd | NumpadBackspace | NumpadClear
            | NumpadClearEntry | NumpadComma | NumpadDecimal | NumpadDivide | NumpadEnter
            | NumpadEqual | NumpadHash | NumpadMemoryAdd | NumpadMemoryClear
            | NumpadMemoryRecall | NumpadMemoryStore | NumpadMemorySubtract | NumpadMultiply
            | NumpadParenLeft | NumpadParenRight | NumpadStar | NumpadSubtract => {
                KeyCodeClass::Numpad
            }

            // Function Section
            Escape | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 | F12 | F13 | F14
            | F15 | F16 | F17 | F18 | F19 | F20 | F21 | F22 | F23 | F24 | Fn | FnLock
            | PrintScreen | ScrollLock | Pause => KeyCodeClass::Function,

            // Media Keys
            BrowserBack | BrowserFavorites | BrowserForward | BrowserHome | BrowserRefresh
            | BrowserSearch | BrowserStop | Eject | LaunchApp1 | LaunchApp2 | LaunchMail
            | MediaPlayPause | MediaSelect | MediaStop | MediaTrackNext | MediaTrackPrevious
            | Power | Sleep | AudioVolumeDown | AudioVolumeMute | AudioVolumeUp | WakeUp => {
                KeyCodeClass::Media
            }

            // Legacy, Non-Standard and Special Keys
            Again | Copy | Cut | Find | Open | Paste | Props | Select | Undo => {
                KeyCodeClass::Legacy
            }

            // Gamepad Keys
            Gamepad0 | Gamepad1 | Gamepad2 | Gamepad3 | Gamepad4 | Gamepad5 | Gamepad6
            | Gamepad7 | Gamepad8 | Gamepad9 | Gamepad10 | Gamepad11 | Gamepad12 | Gamepad13
            | Gamepad14 | Gamepad15 | Gamepad16 | Gamepad17 | Gamepad18 | Gamepad19 => {
                KeyCodeClass::Gamepad
            }

            // Browser specific Keys
            BrightnessDown | BrightnessUp | DisplayToggleIntExt | KeyboardLayoutSelect
            | LaunchAssistant | LaunchControlPanel | LaunchScreenSaver | MailForward
            | MailReply | MailSend | MediaFastForward | MediaPause | MediaPlay | MediaRecord
            | MediaRewind | PrivacyScreenToggle | SelectTask | ShowAllWindows | ZoomToggle => {
                KeyCodeClass::NonStandard
            }
        }
    }

    pub fn resolve(self) -> Cow<'static, str> {
        let class = self.classify();
        if class == KeyCodeClass::WritingSystem {
            if let Some(resolved) = crate::platform::try_resolve(self) {
                let uppercase = if resolved != "ß" {
                    resolved.to_uppercase()
                } else {
                    resolved
                };
                return uppercase.into();
            }
        }
        self.as_str().into()
    }
}

impl FromStr for KeyCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::KeyCode::*;
        Ok(match s {
            // Writing System Keys
            "Backquote" => Backquote,
            "Backslash" => Backslash,
            "Backspace" => Backspace,
            "BracketLeft" => BracketLeft,
            "BracketRight" => BracketRight,
            "Comma" => Comma,
            "Digit0" | "0" => Digit0,
            "Digit1" | "1" => Digit1,
            "Digit2" | "2" => Digit2,
            "Digit3" | "3" => Digit3,
            "Digit4" | "4" => Digit4,
            "Digit5" | "5" => Digit5,
            "Digit6" | "6" => Digit6,
            "Digit7" | "7" => Digit7,
            "Digit8" | "8" => Digit8,
            "Digit9" | "9" => Digit9,
            "Equal" => Equal,
            "IntlBackslash" => IntlBackslash,
            "IntlRo" => IntlRo,
            "IntlYen" => IntlYen,
            "KeyA" | "A" => KeyA,
            "KeyB" | "B" => KeyB,
            "KeyC" | "C" => KeyC,
            "KeyD" | "D" => KeyD,
            "KeyE" | "E" => KeyE,
            "KeyF" | "F" => KeyF,
            "KeyG" | "G" => KeyG,
            "KeyH" | "H" => KeyH,
            "KeyI" | "I" => KeyI,
            "KeyJ" | "J" => KeyJ,
            "KeyK" | "K" => KeyK,
            "KeyL" | "L" => KeyL,
            "KeyM" | "M" => KeyM,
            "KeyN" | "N" => KeyN,
            "KeyO" | "O" => KeyO,
            "KeyP" | "P" => KeyP,
            "KeyQ" | "Q" => KeyQ,
            "KeyR" | "R" => KeyR,
            "KeyS" | "S" => KeyS,
            "KeyT" | "T" => KeyT,
            "KeyU" | "U" => KeyU,
            "KeyV" | "V" => KeyV,
            "KeyW" | "W" => KeyW,
            "KeyX" | "X" => KeyX,
            "KeyY" | "Y" => KeyY,
            "KeyZ" | "Z" => KeyZ,
            "Minus" => Minus,
            "Period" => Period,
            "Quote" => Quote,
            "Semicolon" => Semicolon,
            "Slash" => Slash,

            // Functional Keys
            "AltLeft" => AltLeft,
            "AltRight" => AltRight,
            "CapsLock" => CapsLock,
            "ContextMenu" => ContextMenu,
            "ControlLeft" => ControlLeft,
            "ControlRight" => ControlRight,
            "Enter" => Enter,
            // OS is used instead of Meta in all Firefox versions, Chrome <52
            // and all Safari GTK and WPE versions.
            // Firefox Tracking Issue: https://bugzilla.mozilla.org/show_bug.cgi?id=1595863
            "MetaLeft" | "OSLeft" => MetaLeft,
            "MetaRight" | "OSRight" => MetaRight,
            "ShiftLeft" => ShiftLeft,
            "ShiftRight" => ShiftRight,
            "Space" => Space,
            "Tab" => Tab,

            // Functional Keys found on Japanese and Korean keyboards
            "Convert" => Convert,
            "KanaMode" => KanaMode,
            "Lang1" => Lang1, // MDN claims Chrome uses `HangulMode` but it's not true
            "Lang2" => Lang2, // MDN claims Chrome uses `Hanja` but it's not true
            "Lang3" => Lang3,
            "Lang4" => Lang4,
            "Lang5" => Lang5,
            "NonConvert" => NonConvert,

            // Control Pad Section
            "Delete" => Delete,
            "End" => End,
            "Help" => Help,
            "Home" => Home,
            "Insert" => Insert,
            "PageDown" => PageDown,
            "PageUp" => PageUp,

            // Arrow Pad Section
            "ArrowDown" => ArrowDown,
            "ArrowLeft" => ArrowLeft,
            "ArrowRight" => ArrowRight,
            "ArrowUp" => ArrowUp,

            // Numpad Section
            "NumLock" => NumLock,
            "Numpad0" => Numpad0,
            "Numpad1" => Numpad1,
            "Numpad2" => Numpad2,
            "Numpad3" => Numpad3,
            "Numpad4" => Numpad4,
            "Numpad5" => Numpad5,
            "Numpad6" => Numpad6,
            "Numpad7" => Numpad7,
            "Numpad8" => Numpad8,
            "Numpad9" => Numpad9,
            "NumpadAdd" => NumpadAdd,
            "NumpadBackspace" => NumpadBackspace, // No browser seems to use these?
            "NumpadClear" => NumpadClear,         // No browser seems to use these?
            "NumpadClearEntry" => NumpadClearEntry, // No browser seems to use these?
            "NumpadComma" => NumpadComma,
            "NumpadDecimal" => NumpadDecimal,
            "NumpadDivide" => NumpadDivide,
            "NumpadEnter" => NumpadEnter,
            "NumpadEqual" => NumpadEqual,
            "NumpadHash" => NumpadHash, // No browser seems to use these?
            "NumpadMemoryAdd" => NumpadMemoryAdd, // No browser seems to use these?
            "NumpadMemoryClear" => NumpadMemoryClear, // No browser seems to use these?
            "NumpadMemoryRecall" => NumpadMemoryRecall, // No browser seems to use these?
            "NumpadMemoryStore" => NumpadMemoryStore, // No browser seems to use these?
            "NumpadMemorySubtract" => NumpadMemorySubtract, // No browser seems to use these?
            "NumpadMultiply" => NumpadMultiply,
            "NumpadParenLeft" => NumpadParenLeft,
            "NumpadParenRight" => NumpadParenRight,
            "NumpadStar" => NumpadStar, // No browser seems to use these?
            "NumpadSubtract" => NumpadSubtract,

            // Function Section
            "Escape" => Escape,
            "F1" => F1,
            "F2" => F2,
            "F3" => F3,
            "F4" => F4,
            "F5" => F5,
            "F6" => F6,
            "F7" => F7,
            "F8" => F8,
            "F9" => F9,
            "F10" => F10,
            "F11" => F11,
            "F12" => F12,
            "F13" => F13,
            "F14" => F14,
            "F15" => F15,
            "F16" => F16,
            "F17" => F17,
            "F18" => F18,
            "F19" => F19,
            "F20" => F20,
            "F21" => F21,
            "F22" => F22,
            "F23" => F23,
            "F24" => F24,
            "Fn" => Fn,
            "FnLock" => FnLock, // No browser seems to use these?
            "PrintScreen" => PrintScreen,
            "ScrollLock" => ScrollLock,
            "Pause" => Pause,

            // Media Keys
            "BrowserBack" => BrowserBack,
            "BrowserFavorites" => BrowserFavorites,
            "BrowserForward" => BrowserForward,
            "BrowserHome" => BrowserHome,
            "BrowserRefresh" => BrowserRefresh,
            "BrowserSearch" => BrowserSearch,
            "BrowserStop" => BrowserStop, // MDN claims it is `Cancel` in Chrome, but it never even was.
            "Eject" => Eject,
            "LaunchApp1" => LaunchApp1,
            "LaunchApp2" => LaunchApp2,
            "LaunchMail" => LaunchMail,
            "MediaPlayPause" => MediaPlayPause,
            // According to MDN some versions of Firefox use `LaunchMediaPlayer`
            // here, but that's wrong. However Safari GTK and WPE use this value.
            "MediaSelect" | "LaunchMediaPlayer" => MediaSelect,
            "MediaStop" => MediaStop,
            "MediaTrackNext" => MediaTrackNext,
            "MediaTrackPrevious" => MediaTrackPrevious,
            "Power" => Power,
            "Sleep" => Sleep,
            // Wrong Volume* names in Firefox are tracked here: https://bugzilla.mozilla.org/show_bug.cgi?id=1272579
            "AudioVolumeDown" | "VolumeDown" => AudioVolumeDown, // VolumeDown in old browsers, sometimes even new ones
            "AudioVolumeMute" | "VolumeMute" => AudioVolumeMute, // VolumeMute in old browsers, sometimes even new ones
            "AudioVolumeUp" | "VolumeUp" => AudioVolumeUp, // VolumeUp in old browsers, sometimes even new ones
            "WakeUp" => WakeUp,

            // Legacy, Non-Standard and Special Keys
            "Again" => Again,
            "Copy" => Copy,
            "Cut" => Cut,
            "Find" => Find,
            "Open" => Open,
            "Paste" => Paste,
            "Props" => Props,
            "Select" => Select,
            "Undo" => Undo,

            // Gamepad Keys
            "Gamepad0" => Gamepad0,
            "Gamepad1" => Gamepad1,
            "Gamepad2" => Gamepad2,
            "Gamepad3" => Gamepad3,
            "Gamepad4" => Gamepad4,
            "Gamepad5" => Gamepad5,
            "Gamepad6" => Gamepad6,
            "Gamepad7" => Gamepad7,
            "Gamepad8" => Gamepad8,
            "Gamepad9" => Gamepad9,
            "Gamepad10" => Gamepad10,
            "Gamepad11" => Gamepad11,
            "Gamepad12" => Gamepad12,
            "Gamepad13" => Gamepad13,
            "Gamepad14" => Gamepad14,
            "Gamepad15" => Gamepad15,
            "Gamepad16" => Gamepad16,
            "Gamepad17" => Gamepad17,
            "Gamepad18" => Gamepad18,
            "Gamepad19" => Gamepad19,

            // Browser specific Keys
            "BrightnessDown" => BrightnessDown,
            "BrightnessUp" => BrightnessUp,
            "DisplayToggleIntExt" => DisplayToggleIntExt,
            "KeyboardLayoutSelect" => KeyboardLayoutSelect,
            "LaunchAssistant" => LaunchAssistant,
            "LaunchControlPanel" => LaunchControlPanel,
            "LaunchScreenSaver" => LaunchScreenSaver,
            "MailForward" => MailForward,
            "MailReply" => MailReply,
            "MailSend" => MailSend,
            "MediaFastForward" => MediaFastForward,
            "MediaPause" => MediaPause,
            "MediaPlay" => MediaPlay,
            "MediaRecord" => MediaRecord,
            "MediaRewind" => MediaRewind,
            "PrivacyScreenToggle" => PrivacyScreenToggle,
            "SelectTask" => SelectTask,
            "ShowAllWindows" => ShowAllWindows,
            "ZoomToggle" => ZoomToggle,
            _ => return Err(()),
        })
    }
}

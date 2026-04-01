use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol};
use objc2::{AnyThread, DefinedClass, MainThreadMarker, define_class, msg_send, sel};
use objc2_app_kit::{NSApplication, NSEventModifierFlags, NSMenuItem};
use objc2_foundation::ns_string;

struct SettingsMenuHandlerIvars {
    config_path: Option<PathBuf>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `SettingsMenuHandler` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[name = "AlacrittySettingsMenuHandler"]
    #[ivars = SettingsMenuHandlerIvars]
    struct SettingsMenuHandler;

    impl SettingsMenuHandler {
        #[unsafe(method(openSettings:))]
        fn open_settings(&self, _sender: *mut AnyObject) {
            if let Some(path) = &self.ivars().config_path {
                let _ = std::process::Command::new("open")
                    .args(["-t", &path.to_string_lossy()])
                    .spawn();
            }
        }
    }

    unsafe impl NSObjectProtocol for SettingsMenuHandler {}
);

impl SettingsMenuHandler {
    fn new(config_path: Option<PathBuf>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(SettingsMenuHandlerIvars { config_path });
        unsafe { msg_send![super(this), init] }
    }
}

/// Add a "Settings..." item to the macOS application menu.
///
/// The item is inserted between "About Alacritty" and the existing separator, so the
/// app menu reads: About | --- | Settings... | --- | Services | ...
/// Activating it (either via click or the Cmd+, key equivalent) opens the config
/// file in the system's default text editor.
pub fn install_settings_menu_item(config_path: Option<PathBuf>) {
    let mtm = match MainThreadMarker::new() {
        Some(mtm) => mtm,
        None => return,
    };

    let app = NSApplication::sharedApplication(mtm);
    let main_menu = match app.mainMenu() {
        Some(menu) => menu,
        None => return,
    };

    // The first item in the menu bar is the application menu.
    let app_menu_item = match main_menu.itemAtIndex(0) {
        Some(item) => item,
        None => return,
    };
    let app_menu = match app_menu_item.submenu() {
        Some(menu) => menu,
        None => return,
    };

    let handler = SettingsMenuHandler::new(config_path);

    let settings_item = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            mtm.alloc(),
            ns_string!("Settings..."),
            Some(sel!(openSettings:)),
            ns_string!(","),
        )
    };
    // NSMenuItem.target is unsafe_unretained, so we also store the handler in
    // representedObject (which IS retained) to keep it alive for the menu's lifetime.
    unsafe { settings_item.setRepresentedObject(Some(&*handler)) };
    unsafe { settings_item.setTarget(Some(&*handler)) };
    settings_item.setKeyEquivalentModifierMask(NSEventModifierFlags::Command);

    // Winit's default app menu is: [About(0), sep(1), Services(2), ...].
    // Insert a separator at index 1, then "Settings..." at index 2, so the layout becomes:
    // About | --- | Settings... | --- (existing) | Services | ...
    let separator = NSMenuItem::separatorItem(mtm);
    app_menu.insertItem_atIndex(&separator, 1);
    app_menu.insertItem_atIndex(&settings_item, 2);
}

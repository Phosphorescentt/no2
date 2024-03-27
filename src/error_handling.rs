use std::panic;

use crate::terminal;
use color_eyre::eyre;

pub fn install_hooks() -> color_eyre::Result<()> {
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        terminal::restore_terminal().unwrap();
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    let _ = eyre::set_hook(Box::new(move |error| {
        terminal::restore_terminal().unwrap();
        eyre_hook(error)
    }));

    Ok(())
}

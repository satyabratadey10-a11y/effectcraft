//! Native Android entry point for EffectCraft.
//!
//! This is deliberately a thin platform shell. The engine, renderer and egui frontend remain
//! shared with the desktop app; Android-specific behaviour is limited to the activity lifecycle,
//! fullscreen phone sizing and conservative preview defaults.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_os = "android")]
use android_activity::AndroidApp;
#[cfg(target_os = "android")]
use effectcraft_ui_egui::EffectcraftApp;
#[cfg(target_os = "android")]
use eframe::{NativeOptions, egui};

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub fn android_main(android_app: AndroidApp) {
    effectcraft_engine::logging::install();
    effectcraft_engine::logging::install_panic_hook();

    let options = NativeOptions {
        android_app: Some(android_app),
        viewport: egui::ViewportBuilder::default().with_title("EffectCraft").with_fullscreen(true).with_decorations(false),
        ..Default::default()
    };

    let result = eframe::run_native(
        "EffectCraft",
        options,
        Box::new(|cc| {
            let mut session = effectcraft_host::session();
            session.check_footage_on_open = true;
            session.autosave.background = true;
            session.load_settings();
            session.prefs.appearance.ui_scale = session.prefs.appearance.ui_scale.clamp(110, 140);
            session.prefs.previews.adaptive_resolution_limit = "1/4".into();
            session.prefs.previews.fast_previews = true;
            session.prefs.previews.cache_frames_when_idle = false;

            let mut app = EffectcraftApp::new(session);
            app.fps = 30.0;
            app.playback.res = Some(2);
            app.ui.start_screen = true;
            app.ui.status = "Touch-optimized Android mode".into();

            if let Some(render_state) = &cc.wgpu_render_state {
                log::info!("Android graphics adapter: {:?}", render_state.adapter.get_info());
            }

            Ok(Box::new(app))
        }),
    );

    if let Err(error) = result {
        log::error!("EffectCraft Android stopped: {error}");
    }
}

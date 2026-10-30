# EffectCraft on Android

EffectCraft's Android port is a native Rust application. It does **not** use the web/PWA
frontend: the Android activity hosts the same engine, renderer and egui frontend as the desktop
application through `winit` and `eframe`.

The Android shell is intentionally small:

- fullscreen, edge-to-edge phone layout;
- shared `.ecproj` engine and renderer;
- no desktop menu bar, window persistence, TCP control server or desktop-only audio device setup;
- conservative phone defaults, with rendering and file operations still handled by the shared
  background engine.

## Build a release APK

Install the Android SDK and NDK, then install the Rust targets and `cargo-apk`:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-apk --locked
```

Set `ANDROID_HOME` (or `ANDROID_SDK_ROOT`) and `ANDROID_NDK_HOME`, then run:

```sh
cargo apk build -p effectcraft-android --release
```

The APK is written below `target/release/apk/`. For a modern 64-bit phone, use:

```sh
cargo apk build -p effectcraft-android --release --target aarch64-linux-android
```

## Build from GitHub Actions

The `Android APK` workflow in `.github/workflows/android.yml` builds both phone ABIs:

- `effectcraft-android-arm64-v8a` for current 64-bit phones;
- `effectcraft-android-armeabi-v7a` for older 32-bit phones.

Run it from the repository's **Actions** tab with **Run workflow**. Download the artifact for
your phone, enable installation from the browser/files app if Android asks, and open the APK.
The workflow runs `cargo apk check`, builds optimized release APKs, and publishes SHA-256 files
beside them.

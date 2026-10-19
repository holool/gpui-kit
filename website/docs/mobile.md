---
title: Mobile
description: Native iOS and Android applications using Holool GPUI Fast, with Arabic and RTL preserved.
order: -10
maturity: [experimental, platform-dependent]
---

# Mobile

Holool maintains native platform adapters in [gpui_ios](https://github.com/holool/gpui_ios)
and [gpui_android](https://github.com/holool/gpui_android). Enable Kit's `mobile`
feature to expose the adapter for the current target through `gpui_kit::mobile`.
The feature selects GPUI Fast throughout Kit; it does not add a second GPUI
runtime. Mobile support remains experimental: consult the backend validation
records for the devices and behaviors actually exercised.

## Dependencies and native hosts

The core, compatibility wrappers, text system, renderer, and platform adapter
must resolve the same Holool Fast revision. Kit pins these dependencies together.
Do not substitute the historical `gpui-pre-mobile` / `gpui-pre 0.3.4` example:
it uses a different runtime and lacks Holool's current bidi geometry.

```toml
[lib]
crate-type = ["staticlib", "cdylib"]

[dependencies]
gpui-kit = { git = "https://github.com/holool/gpui-kit", features = ["mobile"] }
```

Pin a tested Kit commit in production. The complete
[gpui-mobile-lab](https://github.com/holool/gpui-mobile-lab) contains pinned
manifests, a shared Rust SQLite engine, native GPUI hosts, and a Flutter
comparison application using the same engine.

A native host owns application startup and frame scheduling. iOS uses a UIKit
view backed by `CAMetalLayer`; Android uses an Activity and SurfaceView. The
backend retains the `ApplicationHandle` from `Application::run_embedded` while
native callbacks can reach GPUI. Calling ordinary `Application::run` and then
returning to a native event loop drops the application too early.

For a crate serving both mobile targets, emit only the Rust archive on iOS
(`cargo rustc --lib --target aarch64-apple-ios-sim --crate-type staticlib`). Swift
supplies the host callbacks at the final application link. Android emits a
`cdylib` instead. The lab's build scripts select these outputs automatically.

## Shared Kit content

Use the normal Kit initialization and window helper. The builder returns
application content; `open_window` mounts exactly one Base Root and its overlays.
Do not construct another `component::Root` in your content.

```rust
use gpui_kit::*;

fn configure(application: Application) -> Application {
    application.with_assets(gpui_kit::assets::Assets)
}

fn launch(cx: &mut App) {
    gpui_kit::init(cx);
    gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
        cx.new(|_| OrdersView::new())
    }).expect("open orders");
}

#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn orders_launch() {
    gpui_kit::mobile::run_with(configure(gpui_kit::mobile::application()), launch);
}

#[cfg(target_os = "android")]
gpui_kit::mobile::android::android_main!(launch, configure);
```

`OrdersView` above is your application view. The iOS host subclasses the
backend's `GPUIAppDelegate` and calls `orders_launch` from `start`. Android
subclasses `GpuiActivity`, sets the Rust shared-library name, and lets the host
initialize the platform before the launch callback. See each backend's minimal
example for the corresponding Swift/Kotlin and build files.

`configure` above installs Kit's bundled SVG assets. It can also configure an
HTTP client or application-owned assets. `run_with` retains that configured
application; `run` uses a default application when no configuration is needed.
Android's entry macro initializes the host before invoking `configure`.
`mobile::application` requires that initialization first on Android.

## Arabic and RTL

Keep text in logical UTF-8 order. The platform converts native UTF-16 ranges;
neither a native host nor an application should reverse the string or mirror
physical input coordinates. Apply inherited `.rtl()` and logical spacing at the
Rust view boundary, keeping LTR fields such as email in explicit `.ltr()` islands.

The iOS adapter shares Fast's `AppleTextSystem` and Metal renderer. Android
shares `CosmicTextSystem` and the WGPU renderer. This preserves `fast_bidi`, visual
caret navigation, disjoint selection ranges, grapheme-safe wrapping and cached
scroll tiles. The donor's older renderer and shapers are not replacements.

Bundle required Arabic fonts and register them with `cx.text_system().add_fonts`
before opening the first window. Use the same fonts and fixture text when
comparing platforms; system fallback differs across devices.

Keep transient IME composition in the focused Input/Textarea. Do not asynchronously
replace its composing text with a normalized search or saved value. Normalize
search keys in the domain layer while preserving authored text for display.

## Input, layout, and acceptance

Kit already supplies touch selection handles, edit menus, tap-operated HoverCard,
mobile tooltip suppression and scroll-bounce behavior. Native input adapters
synchronize composition and UTF-16 selection with the existing InputHandler.
GPUI owns selection presentation in the first integration; this is not a promise
of complete UIKit/Android system-selection parity.

Give each scroll region one owner, release flex minimum sizes with `min_h_0` /
`min_w_0`, and adapt dense desktop screens to the phone's actual viewport. Let the
host deliver safe-area and keyboard insets once. Do not subtract the keyboard
height again in both native layout and Rust content.

Build checks cover iOS device/simulator and Android arm64. Runtime acceptance
must additionally cover Arabic marks and ligatures, Arabic/Latin/emoji mixtures,
composition, visual caret movement, both-side deletion, copy/paste, touch
selection, rotation, keyboard transitions, background recovery and Android
surface recreation. Verify semantic labels and actions with VoiceOver/TalkBack.
Simulator/emulator results establish functionality, not physical-device frame
time, battery behavior, or complete assistive-technology acceptance.

## Attribution

The mobile work builds on Zed GPUI and learns from
[Zedra's mobile fork](https://github.com/tanlethanh/zed/tree/feat/gpui-mobile)
and [gpui-mobile](https://github.com/itsbalamurali/gpui-mobile). The Holool adapters
are maintained against Holool Fast so the Arabic/RTL and rendering extensions
remain intact. See their NOTICE files for source attribution.

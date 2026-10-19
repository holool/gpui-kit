---
title: Mobile
description: 使用 Holool GPUI Fast 构建原生 iOS 与 Android 应用，保留阿拉伯语和 RTL 支持。
order: -10
maturity: [experimental, platform-dependent]
---

# 移动端

Holool 在 [gpui_ios](https://github.com/holool/gpui_ios) 和
[gpui_android](https://github.com/holool/gpui_android) 中维护原生平台适配器。
启用 Kit 的 `mobile` feature 后，通过 `gpui_kit::mobile` 使用当前平台的适配器。
此 feature 为 Kit 的各层统一选择 GPUI Fast，不会引入第二套 GPUI runtime。
移动端仍处于实验阶段；实际验证的平台与行为以各后端的验证记录为准。

## 依赖与原生宿主

Core、兼容包装、TextSystem、Renderer 和平台适配器必须使用相同的 Holool Fast
revision，Kit 会统一固定这些依赖。不要使用历史上的 `gpui-pre-mobile` /
`gpui-pre 0.3.4` 示例替换当前依赖：它使用不同的 runtime，也没有 Holool 当前的
bidi geometry。

```toml
[lib]
crate-type = ["staticlib", "cdylib"]

[dependencies]
gpui-kit = { git = "https://github.com/holool/gpui-kit", features = ["mobile"] }
```

生产应用应固定经过验证的 Kit commit。
[gpui-mobile-lab](https://github.com/holool/gpui-mobile-lab) 提供固定版本的 manifest、
共享 Rust SQLite engine、原生 GPUI 宿主，以及使用同一 engine 的 Flutter 对照应用。

原生宿主负责启动和帧调度。iOS 使用带有 `CAMetalLayer` 的 UIKit View，Android 使用
Activity 和 SurfaceView。只要原生回调仍可访问 GPUI，后端就保留
`Application::run_embedded` 返回的 `ApplicationHandle`。普通 `Application::run`
返回原生事件循环后会过早释放应用，不适合此生命周期。

同一 crate 面向两个移动平台时，iOS 构建只生成 Rust 静态库：
`cargo rustc --lib --target aarch64-apple-ios-sim --crate-type staticlib`。
Swift 在最终应用链接时提供宿主 callback；Android 则生成 `cdylib`。
lab 的构建脚本会自动选择对应的输出类型。

## 共享 Kit 内容

使用标准 Kit 初始化和窗口辅助函数。builder 返回应用内容；`open_window` 自动挂载
一个 Base Root 及其 overlay。不要在内容中再次构造 `component::Root`。

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

这里的 `OrdersView` 是应用自己的 View。iOS 宿主继承后端的 `GPUIAppDelegate`，
在 `start` 中调用 `orders_launch`。Android 宿主继承 `GpuiActivity` 并设置 Rust
动态库名称，由宿主先初始化平台，再执行 launch callback。完整的 Swift/Kotlin
和构建配置见各后端的最小示例。

上面的 `configure` 为应用配置 Kit 内置的 SVG 资源，也可用于配置 HTTP client 或应用自己的资源。
`run_with` 保留已配置的 application；无需配置时可使用 `run` 创建默认 application。
Android entry macro 先初始化宿主，再调用 `configure`；在 Android 上使用
`mobile::application` 前必须完成宿主初始化。

## 阿拉伯语与 RTL

文本始终按逻辑 UTF-8 顺序保存，平台负责转换原生 UTF-16 range。宿主和应用都不应
反转字符串或镜像物理触摸坐标。在 Rust View 边界使用可继承的 `.rtl()` 和逻辑间距，
电子邮件等 LTR 字段放入显式的 `.ltr()` 区域。

iOS 适配器复用 Fast 的 `AppleTextSystem` 与 Metal renderer；Android 复用
`CosmicTextSystem` 与 WGPU renderer，因此保留 `fast_bidi`、visual caret navigation、
不连续选择区域、grapheme-safe wrapping 和缓存 scroll tiles。旧 donor 的 renderer
和 shaper 不能直接替换这些实现。

在打开第一个窗口前，将必需的阿拉伯语字体打包并通过 `cx.text_system().add_fonts`
注册。跨平台比较应使用相同字体和 fixture text；不同设备的系统字体回退并不一致。

IME composition 的临时状态由焦点 Input/Textarea 管理。不要异步以标准化搜索文本或
保存值覆盖正在 composing 的内容。领域层可标准化搜索 key，但显示文本应保留原样。

## 输入、布局与验收

Kit 已提供触摸选择 handles、edit menu、通过 tap 操作的 HoverCard、移动端 tooltip
抑制及 scroll bounce。原生输入适配器通过现有 InputHandler 同步 composition 和
UTF-16 selection。初次集成由 GPUI 呈现选择 UI，不代表完全复现 UIKit/Android
系统文本选择行为。

每个滚动区域只有一个 owner，使用 `min_h_0` / `min_w_0` 释放 flex 最小尺寸，并根据
手机的实际 viewport 调整桌面密集布局。safe area 和 keyboard insets 只处理一次，
不要同时在原生 layout 和 Rust 内容中重复减去键盘高度。

构建检查覆盖 iOS device/simulator 和 Android arm64。运行时还须验证阿拉伯语 marks、
ligature、Arabic/Latin/emoji 混排、composition、visual caret、双向删除、复制粘贴、
触摸选择、旋转、键盘切换、后台恢复及 Android surface recreation。通过
VoiceOver/TalkBack 验证语义标签和操作。模拟器结果只能证明功能，不能代替真机帧耗时、
电池表现或完整的辅助技术验收。

## 致谢

此工作基于 Zed GPUI，并参考
[Zedra mobile fork](https://github.com/tanlethanh/zed/tree/feat/gpui-mobile) 和
[gpui-mobile](https://github.com/itsbalamurali/gpui-mobile)。Holool 适配器持续跟随
Holool Fast，以保留阿拉伯语、RTL 和渲染扩展。源代码归属见各后端的 NOTICE 文件。

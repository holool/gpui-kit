---
title: "阿拉伯语与 RTL（Holool 分支）"
description: "Holool 分支的继承式 RTL 布局、原生阿拉伯语编辑和 GPUI Fast 集成。"
---

# 阿拉伯语与 RTL（Holool 分支）

Holool 分支默认使用 [Holool GPUI Fast](https://github.com/holool/gpui-fast)。兼容 crate
让关闭 Kit 默认功能的调用方也使用同一个 GPUI 引擎。请使用 Git 源和仓库中的锁文件；
这些修改尚未发布到 crates.io。

```rust
use gpui_kit::{Directional as _, LayoutDirection, ParentElement as _, Styled as _, div, px};
let content = div()
    .direction(LayoutDirection::Rtl)
    .ps(px(12.))
    .child("مرحبا بالعالم")
    .child(div().ltr().child("LTR source code"));
```

方向通过样式元素、保留视图和延迟绘制继承。Taffy 负责 RTL flex/grid 布局，源顺序、
焦点顺序与无障碍顺序保持逻辑顺序。逻辑边距使用 `ps`/`pe`、`ms`/`me`、`start`/`end`；
`pl`/`pr`、`left`/`right` 仍表示物理方向。代码编辑器或路径可显式使用 `.ltr()`。
方向边界默认按行内起点对齐，显式物理文本对齐优先。

输入框与多行输入继承方向和对齐。普通文本行使用双向光标几何处理点击、方向键、
Shift+方向键与不连续选区。空占位文本单独绘制，不影响空文档光标或滚动。
IME 候选矩形规范化反向横坐标。开关的选中滑块位于行内终点；面包屑与侧栏箭头跟随方向；
弹出子菜单优先使用阅读方向并保留屏幕边缘回退；树形控件按最近绘制方向处理展开键。

运行 `cargo run -p arabic_rtl` 可检查阿拉伯语组合附标、英数混排、空占位符、多行编辑、
开关、嵌套 LTR 区域和实时方向切换。

## 移植决策

| gpui-arabic 中的修改 | 本分支处理 |
| --- | --- |
| 布局方向继承 | 使用 Fast 样式上下文和 Taffy 0.14，包含保留缓存失效。 |
| 文本和光标几何 | 适配原生排版，不移植 CE 的 Parley 引擎。 |
| 最终内容宽度协调 | Kit 已使用最终 prepaint 边界排版输入文本。 |
| 空占位文本与光标 | 移植到 Input、textarea、editor 的共享引擎。 |
| 标签、焦点与无障碍 | Kit 的 Base/Component 已处理。 |
| 开关拖动语义 | Kit 使用受控点击/键盘开关，不增加无关拖动 API。 |
| X11 触摸联系人 | 移植到 Fast Linux 后端，附联系人与取消测试。 |
| 优先硬件 GPU | Fast 已实现，无需重复。 |

本修改支持显式 LTR/RTL 布局，不推断 CSS `direction:auto`。混排边界目前使用单个主光标位置，
不新增原生光标亲和性 API。行内 token 片段、自定义物理边距与图标仍需单独审查。
原生排版与设备 QA 限制见 Fast 的 `docs/holool-rtl.md`。

完整 Base 测试中有 17 项失败，在未修改的上游 Kit 和 Fast 组合中同样可复现；请参阅[验证记录](https://github.com/holool/gpui-kit/blob/codex/arabic-rtl/docs/rtl-validation.md)。

普通文本行的方向键和 Shift+方向键支持跨越硬换行和软换行，沿用 Kit 现有的软换行光标亲和性。
